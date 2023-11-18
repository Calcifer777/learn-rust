use actix_web::{test, App};
use zero2prod::subscribe;


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
}