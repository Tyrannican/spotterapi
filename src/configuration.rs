use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub application_port: u32,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    host: String,
    port: u32,
    username: String,
    password: String,
    database_name: String,
}

impl DatabaseSettings {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<AppSettings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<AppSettings>()
}
