use actix_web::{ post, web, HttpResponse, };
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

#[post("/subscription")]
async fn subscribe(user: web::Json<User>) -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}
