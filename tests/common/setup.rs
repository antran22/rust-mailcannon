use std::net::TcpListener;

use mailcannon::{Settings, telemetry};
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "test".to_string();
    let filter_level = "info".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            telemetry::make_tracing_subscriber(subscriber_name, filter_level, std::io::stdout);

        telemetry::init_subscriber(subscriber);
    } else {
        let subscriber =
            telemetry::make_tracing_subscriber(subscriber_name, filter_level, std::io::sink);

        telemetry::init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let config = get_configuration();
    let db = configure_database(config).await;

    let server = mailcannon::make_server(listener, db.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db,
    }
}

fn get_configuration() -> Settings {
    let mut config = mailcannon::get_configuration().expect("failed to get configuration");
    config.database.database_name = Uuid::new_v4().to_string();
    config
}

async fn configure_database(config: Settings) -> PgPool {
    let db_config = &config.database;
    dbg!(db_config);
    let addr = db_config
        .connection_string()
        .expect("must get connection string");

    dbg!(&addr);

    let mut db = PgConnection::connect(&addr)
        .await
        .expect("failed to connect to Postgres.");

    db.execute(format!(r#"CREATE DATABASE "{}";"#, db_config.database_name).as_str())
        .await
        .expect("failed to create database");

    let addr = db_config
        .connection_string_with_db()
        .expect("must get connection string");

    let connection_pool = PgPool::connect(&addr)
        .await
        .expect("failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate the database");

    connection_pool
}
