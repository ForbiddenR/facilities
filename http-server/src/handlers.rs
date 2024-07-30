use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::models::MyResponse;

use super::models::MyRequest;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/test")]
async fn tests(req: HttpRequest, body: web::Json<MyRequest>) -> impl Responder {
    for (k, v) in req.headers() {
        println!("key is {} and value is {}", k, v.to_str().unwrap());
    }
    let protocol = match &body.protocol {
        Some(protocol) => protocol,
        None => return HttpResponse::Ok().json(MyResponse::new(1, "no protocol message")),
    };
    println!(
        "sn is {} and protocol is {:?}",
        body.equipment_sn, protocol
    );
    HttpResponse::Ok().json(MyResponse::new(0, ""))
}
