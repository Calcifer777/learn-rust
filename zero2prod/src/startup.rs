use std::net::TcpListener;

use actix_web::{
    dev::Server, App, HttpServer, web,
};
use tracing_actix_web::TracingLogger;
use sqlx::PgPool;

use super::routes::health_check::health_check;
use super::routes::subscription::subscribe;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Server {
    let db_pool_arc = web::Data::new(db_pool);
    HttpServer::new(move || 
        App::new()
        .wrap(TracingLogger::default())
        .app_data(db_pool_arc.clone())
        .service(health_check)
        .service(subscribe)
    )
    .listen(listener)
    .expect("Failed to bind address")
    .run()
}
