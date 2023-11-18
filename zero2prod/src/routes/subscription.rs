use actix_web::{ post, web, HttpResponse, };
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;


#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

#[post("/subscription")]
async fn subscribe(user: web::Json<User>, _db_pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions
        (id, name, email, subscribed_at) VALUES
        ($1, $2, $3, $4)
        returning id
        "#,
        sqlx::types::Uuid::new_v4(),
        user.name,
        user.email,
        sqlx::types::chrono::Utc::now(),
    )
    .fetch_one(_db_pool.get_ref())
    .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "ok"})),
        Err(e) => {
            eprintln!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to execute query: {:?}", e),
            }))
        },
    }
    
}
