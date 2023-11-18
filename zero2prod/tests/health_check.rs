use actix_web::{test, App};

use zero2prod::routes::health_check::health_check;


#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new().service(health_check)
    ).await;
    let req = test::TestRequest::get()
    .uri("/health_check")
    .to_request();
    let rsp = test::call_service(&app, req).await;
    assert!(rsp.status().is_success());
}