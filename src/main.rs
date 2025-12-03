use std::net::TcpListener;

use mailcannon::{get_configuration, get_database, make_server};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = get_configuration().expect("failed to read configuration");

    let db = get_database(&settings)
        .await
        .expect("failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", settings.application_port);

    let listener = TcpListener::bind(&address)?;

    println!("listening at address {}", &address);

    make_server(listener, db)?.await
}
