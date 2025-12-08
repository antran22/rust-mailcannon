use secrecy::ExposeSecret;
use snafu::Snafu;
use snafu::prelude::*;
use sqlx::Error;
use sqlx::PgPool;

use crate::setup::DbCredentialError;

use super::Settings;

#[derive(Debug, Snafu)]
pub enum DatabaseConnectionError {
    #[snafu(display("connection string error"))]
    ConnectionStringError { source: DbCredentialError },

    #[snafu(display("db connection error"))]
    ConnectionError { source: Error },
}

pub async fn get_database(settings: &Settings) -> Result<PgPool, DatabaseConnectionError> {
    let addr = settings
        .database
        .connection_string_with_db()
        .context(ConnectionStringSnafu {})?;

    PgPool::connect(addr.expose_secret())
        .await
        .context(ConnectionSnafu {})
}
