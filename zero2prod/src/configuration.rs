
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, 
            self.password, 
            self.host, 
            self.port,
            self.name,
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, 
            self.password, 
            self.host, 
            self.port
        )
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