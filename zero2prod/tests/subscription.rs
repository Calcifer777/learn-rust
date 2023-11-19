use actix_web::{test, App, web};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use uuid::Uuid;
use zero2prod::{routes::subscription::subscribe, configuration::get_configuration};


async fn init_db() -> web::Data<PgPool> {
    let mut cfg = get_configuration("./tests/resources/configuration.yaml")
        .unwrap();
    // Create temp db
    let db_pool = PgPool::connect(
        &cfg.db.connection_string().expose_secret()
    )
        .await
        .unwrap();
    let temp_db = Uuid::new_v4().to_string();
    sqlx::query(
        format!(r#"CREATE DATABASE "{}";"#, temp_db).as_str()
    )
        .execute(&db_pool)
        .await
        .expect("Failed to create database.");
    // Setup tmp db
    cfg.db.name = temp_db;
    let db_pool = PgPool::connect(
        &cfg.db.connection_string().expose_secret()
        )
        .await
        .unwrap();
    sqlx::migrate!("./../migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");
    web::Data::new(db_pool)
}

#[actix_web::test]
async fn test_subscribe_ok() {
    let db_pool = init_db().await;
    let app = test::init_service(
        App::new()
        .service(subscribe)
        .app_data(db_pool.clone())
    ).await;
    let req = test::TestRequest::post()
        .uri("/subscription")
        .set_json(&serde_json::json!({"name": "marco", "email": "t@gmail.com"}))
        .to_request();
    let rsp = test::call_service(&app, req).await;
    assert!(rsp.status().is_success());
    
    // Check db status
    let result = sqlx::query!("SELECT * FROM subscriptions")
        .fetch_one(db_pool.get_ref())
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(result.name, "marco");
    assert_eq!(result.email, "t@gmail.com");

}