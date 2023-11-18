use std::net::TcpListener;

use actix_web::{
    dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder, middleware,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

#[post("/subscription")]
async fn subscribe(user: web::Json<User>) -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

pub fn run(listener: TcpListener) -> Server {
    HttpServer::new(|| 
        App::new()
        .wrap(middleware::Logger::default())
        .service(health_check)
        .service(subscribe)
    )
    .listen(listener)
    .expect("Failed to bind address")
    .run()
}
