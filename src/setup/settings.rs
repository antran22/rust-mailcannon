#[derive(serde::Deserialize)]
pub struct Settings {
    #[serde(rename = "postgres")]
    pub database: DatabaseSettings,
    #[serde(default = "default_app_port")]
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    #[serde(rename = "user")]
    pub username: String,
    pub password: String,
    #[serde(default = "default_db_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(rename = "db")]
    pub database: String,
}

fn default_host() -> String {
    "127.0.0.1".to_owned()
}

fn default_app_port() -> u16 {
    8080
}

fn default_db_port() -> u16 {
    5432
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let env_source = config::Environment::default().separator("_");

    let settings = config::Config::builder().add_source(env_source).build()?;

    settings.try_deserialize::<Settings>()
}
