use std::net::TcpListener;

use mailcannon::make_server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    make_server(listener)?.await
}
