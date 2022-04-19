use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // wrap with smart pointer
    let connection = web::Data::new(connection);
    // capture `connection' from surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // get pointer copy and attach to application state
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
