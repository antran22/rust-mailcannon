use std::net::TcpListener;

use mailcannon::{get_configuration, make_server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("failed to read configuration");

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(&address)?;

    println!("listening at address {}", &address);

    make_server(listener)?.await
}
