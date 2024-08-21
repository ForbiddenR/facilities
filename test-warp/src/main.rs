use serde::Deserialize;
use warp::{http::StatusCode, Filter, Rejection, Reply};

#[derive(Deserialize)]
struct PostData {
    message: String,
}

// #[derive(Serialize)]
// struct ResponseData {
//     status: String,
//     message: String,
// }

#[tokio::main]
async fn main() {
    // Define the route for POST requests
    let post_route = warp::post()
        .and(warp::path("submit"))
        .and(warp::body::json())
        .and_then(handle_post);

    // Start the server
    println!("Server starting on http://localhost:150000");
    warp::serve(post_route).run(([0, 0, 0, 0], 15000)).await;
}

async fn handle_post(post_data: PostData) -> Result<impl Reply, Rejection> {
    println!("Received message: {}", post_data.message);

    // let response = ResponseData {
    //     status: "success".to_string(),
    //     message: format!("Received: {}", post_data.message),
    // };

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
