use std::net::TcpListener;

use zero2prod::{startup::run, configuration::get_configuration};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let cfg = get_configuration("./configuration.yaml")
        .expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", cfg.port))?;
    let server = run(listener);
    server.await
}
