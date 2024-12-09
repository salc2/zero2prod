use std::net::TcpListener;

use actix_web::{dev::Server, web, App,  HttpServer };
use crate::routes::*;


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| 
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(handle_subscription))
    )
        .listen(listener)?
        .run();
    Ok(server)
}
