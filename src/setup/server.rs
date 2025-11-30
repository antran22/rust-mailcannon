use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;

use crate::routes;

pub fn make_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::healthcheck::check))
            .route(
                "/subscriptions",
                web::post().to(routes::subscriptions::create),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
