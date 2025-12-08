use std::net::TcpListener;

use mailcannon::{get_configuration, get_database, make_server, telemetry};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber =
        telemetry::make_tracing_subscriber("mailcannon".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    tracing::info!("starting the app");

    let settings = get_configuration().expect("failed to read configuration");

    let db = get_database(&settings)
        .await
        .expect("failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", settings.application_port);

    let listener = TcpListener::bind(&address)?;

    tracing::info!("listening at address {}", &address);

    make_server(listener, db)?.await
}
