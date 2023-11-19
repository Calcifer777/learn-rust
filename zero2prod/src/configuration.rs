use secrecy::{Secret, ExposeSecret};


#[derive(serde::Deserialize)]
pub struct Settings {
    pub db: DatabaseSettings,
    pub app: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, 
            self.password.expose_secret(), 
            self.host, 
            self.port,
            self.name,
        ))
    }
}

pub fn get_configuration(fname: &str) -> Result<Settings, config::ConfigError> {
    config::Config::builder()
    .add_source(
        config::File::new(fname, config::FileFormat::Yaml)
    )
    .build()?
    .try_deserialize()
}