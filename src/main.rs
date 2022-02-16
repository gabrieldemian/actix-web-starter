pub mod lib;
pub mod routes;
mod tests;
use actix_web::{middleware, App, HttpServer};
use env_logger::Env;
use lib::*;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("🚀 listening at {} 🤠", port);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::say_hello::route)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
