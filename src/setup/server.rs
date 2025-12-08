use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes;

pub fn make_server(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(routes::healthcheck::check))
            .route(
                "/subscriptions",
                web::post().to(routes::subscriptions::create),
            )
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
