use std::net::TcpListener;

use mailcannon::Settings;
use sqlx::{Connection, PgConnection};

pub fn get_configuration() -> Settings {
    mailcannon::get_configuration().expect("failed to get configuration")
}

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = mailcannon::make_server(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub async fn get_database() -> PgConnection {
    let config = get_configuration();
    let addr = config.database.connection_string();

    PgConnection::connect(&addr)
        .await
        .expect("Failed to connect to Postgres.")
}
