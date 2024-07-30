use std::time::SystemTime;

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::models::MyResponse;

use super::models::MyRequest;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/test")]
async fn tests(body: web::Json<MyRequest>) -> impl Responder {
    let protocol = body.protocol.as_ref().unwrap();
    println!(
        "sn is {} and protocol is {:?}",
        body.equipment_sn, protocol
    );
    HttpResponse::Ok().json(MyResponse::new(0, ""))
}
