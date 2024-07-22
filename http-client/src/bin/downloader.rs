use std::{env, error::Error, fs::File, io::Write};

use futures_util::StreamExt;
use reqwest::{Client, Proxy};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let repo_owner = env::var("REPOSITORY").expect("Don't find correct repo owner");
    let repo_name = env::var("REPOSITORY_NAME").expect("Don't find correct repo");

    let release_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        repo_owner, repo_name
    );
    // create a client
    let proxy_url = env::var("PROXY_URL").unwrap_or("".into());

    let client = if proxy_url.is_empty() {
        reqwest::Client::new()
    } else {
        println!("use proxy to download file");
        Client::builder().proxy(Proxy::all(proxy_url)?).build()?
    };

    let release_info: Value = client
        .get(&release_url)
        .header("User-Agent", "Rust Github Release Downloader")
        .send()
        .await?
        .json()
        .await?;

    let assets = release_info["assets"].as_array().ok_or("No assets found")?;
    let asset = assets
        .into_iter()
        .find(|x| match x["browser_download_url"].as_str() {
            Some(url) => url.contains("tar.gz") && url.contains("linux") && url.contains("amd64v3"),
            None => false,
        })
        .ok_or("No valid asset in the url")?;
    let url = asset["browser_download_url"]
        .as_str()
        .ok_or("No download URL found")?;
    let file_name = asset["name"].as_str().ok_or("No asset name found")?;

    println!("asset url is {} and asset name is {}", url, file_name);

    println!("file name is {}", file_name);

    let resp = client.get(url).send().await?;

    if resp.status().is_success() {
        let mut file = File::create(file_name)?;

        let mut stream = resp.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
        }
        println!("Download completed");
    } else {
        println!("Request failed with status: {:?}", resp.status());
    }
    Ok(())
}
