use actix_web::{test, App};
use sqlx::PgPool;
use zero2prod::{routes::subscription::subscribe, configuration::get_configuration};


#[actix_web::test]
async fn test_subscribe_ok() {
    let app = test::init_service(
        App::new().service(subscribe)
    ).await;
    let req = test::TestRequest::post()
        .uri("/subscription")
        .set_json(&serde_json::json!({"name": "marco", "email": "test@gmail.com"}))
        .to_request()
        ;
    let rsp = test::call_service(&app, req).await;
    assert!(rsp.status().is_success());
    
    // Check db status
    let cfg = get_configuration("./tests/resources/configuration.yaml")
        .expect("Failed to read configuration.");
    let conn_str = cfg.database.connection_string();
    let pool = PgPool::connect(&conn_str)
        .await
        .expect("DB connection failed");
    let sub = sqlx::query!("SELECT * FROM subscriptions")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch saved subscription")
        ;
    
}