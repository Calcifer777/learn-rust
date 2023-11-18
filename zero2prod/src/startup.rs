use std::net::TcpListener;

use actix_web::{
    dev::Server, App, HttpServer, middleware,
};

use super::routes::health_check::health_check;
use super::routes::subscription::subscribe;

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
