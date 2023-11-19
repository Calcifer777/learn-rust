use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{startup::run, configuration::get_configuration};
// use zero2prod::telemetry::{get_subscriber, init_subscriber};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // let subscriber = 
    //     get_subscriber("zero2prod".into(), "info".into());
    // init_subscriber(subscriber);
    
    let cfg = get_configuration("./configuration.yaml")
        .expect("Failed to read configuration.")
        ;
    let listener = TcpListener::bind(
        format!("{}:{}", cfg.app.host, cfg.app.port)
    )?;
    let conn = PgPool::connect_lazy(
        &cfg.db.connection_string().expose_secret()
    ).expect("DB connection failed");
    let server = run(listener, conn);
    server.await
}
