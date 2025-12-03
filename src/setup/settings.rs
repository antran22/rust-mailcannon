use snafu::Snafu;
use snafu::prelude::*;

use url::ParseError;
use url::Url;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    #[serde(rename = "postgres")]
    pub database: DatabaseSettings,
    #[serde(default = "default_app_port")]
    pub application_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    #[serde(rename = "user")]
    pub username: String,
    pub password: String,
    #[serde(default = "default_db_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(rename = "db")]
    pub database_name: String,
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

#[derive(Debug, Snafu)]
#[snafu(display("failed parsing config"))]
pub struct GetConfigError {
    source: config::ConfigError,
}

#[derive(Debug, Snafu)]
pub enum DbCredentialError {
    #[snafu(display("failed parsing database host"))]
    HostField { source: ParseError },

    #[snafu(whatever, display("{message}"))]
    OtherField { message: String },
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Result<String, DbCredentialError> {
        self.connection_url().map(|url| url.to_string())
    }

    pub fn connection_string_with_db(&self) -> Result<String, DbCredentialError> {
        self.connection_url_with_db().map(|url| url.to_string())
    }

    fn connection_url(&self) -> Result<Url, DbCredentialError> {
        let mut url = Url::parse("postgres://example.com").expect("that should not have failed");

        url.set_host(Some(&self.host)).context(HostFieldSnafu)?;

        url.set_username(&self.username)
            .map_err(|()| DbCredentialError::OtherField {
                message: "failed setting username".to_string(),
            })?;
        url.set_password(Some(&self.password))
            .map_err(|()| DbCredentialError::OtherField {
                message: "failed setting password".to_string(),
            })?;
        url.set_port(Some(self.port))
            .map_err(|()| DbCredentialError::OtherField {
                message: "failed setting port".to_string(),
            })?;
        url.set_path("postgres"); // connect to the default maintenance database
        Ok(url)
    }

    fn connection_url_with_db(&self) -> Result<Url, DbCredentialError> {
        let mut url = self.connection_url()?;
        url.set_path(&self.database_name);

        Ok(url)
    }
}

pub fn get_configuration() -> Result<Settings, GetConfigError> {
    let env_source = config::Environment::default().separator("_");

    config::Config::builder()
        .add_source(env_source)
        .build()
        .context(GetConfigSnafu {})?
        .try_deserialize::<Settings>()
        .context(GetConfigSnafu {})
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_connection_string() {
        let db_settings = DatabaseSettings {
            username: "user".to_string(),
            password: "password".to_string(),
            port: 5432,
            host: "postgres.com".to_string(),
            database_name: "random_db".to_string(),
        };

        let connection_string = db_settings
            .connection_string()
            .expect("must build connection string successfully");

        assert_eq!(
            connection_string,
            "postgres://user:password@postgres.com:5432/postgres"
        );

        let connection_string = db_settings
            .connection_string_with_db()
            .expect("must build connection string successfully");

        assert_eq!(
            connection_string,
            "postgres://user:password@postgres.com:5432/random_db"
        );
    }

    #[test]
    fn test_connection_string_urlencode() {
        let db_settings = DatabaseSettings {
            username: "user".to_string(),
            password: "123#@:456".to_string(),
            port: 5432,
            host: "postgres.com".to_string(),
            database_name: "random_db".to_string(),
        };

        let connection_string = db_settings
            .connection_string()
            .expect("must build connection string successfully");

        assert_eq!(
            connection_string,
            "postgres://user:123%23%40%3A456@postgres.com:5432/postgres"
        );

        let connection_string = db_settings
            .connection_string_with_db()
            .expect("must build connection string successfully");

        assert_eq!(
            connection_string,
            "postgres://user:123%23%40%3A456@postgres.com:5432/random_db"
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_host() {
        let db_settings = DatabaseSettings {
            username: "user".to_string(),
            password: "123#@:456".to_string(),
            port: 5432,
            host: "".to_string(),
            database_name: "random_db".to_string(),
        };

        let value = db_settings.connection_string().unwrap();

        dbg!(value);
    }
}
