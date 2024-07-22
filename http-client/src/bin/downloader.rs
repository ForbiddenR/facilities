use std::{env, error::Error, fs::File, io::Write};

use futures_util::StreamExt;
use reqwest::{Client, Proxy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create a client
    let proxy_url = env::var("PROXY_URL").unwrap_or("".into());

    let client = if proxy_url.is_empty() {
        reqwest::Client::new()
    } else {
        Client::builder().proxy(Proxy::all(proxy_url)?).build()?
    };

    let url = "https://test.com/mimic.tar.gz";

    let file_name = url
        .split("/")
        .last()
        .expect("cannot get filename from the path");

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
