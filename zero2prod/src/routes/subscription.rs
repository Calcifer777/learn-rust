use actix_web::{ post, web, HttpResponse, };
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::{self, Instrument};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

#[post("/subscription")]
async fn subscribe(user: web::Json<User>, _db_pool: web::Data<PgPool>) -> HttpResponse {
    let req_id = Uuid::new_v4();
    let req_span = tracing::info_span!(
        "req_id: {} - Adding a new subscriber", 
        %req_id,
        sub_name=%user.name,
        sub_email=%user.email,
    );
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
    .instrument(req_span)
    .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "ok"})),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to execute query: {:?}", e),
            }))
        },
    }
    
}
