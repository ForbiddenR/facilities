use actix_web::{App, HttpServer};
use handlers::{hello, tests};

mod handlers;

mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(tests))
        .bind("0.0.0.0:12000")?
        .run()
        .await
}
