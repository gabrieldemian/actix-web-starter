use actix_web::{
    middleware,
    web::{self, Path},
    App, HttpServer,
};
use actix_web_starter::*;
use env_logger::Env;
use std::env;
mod tests;

async fn say_hello(info: Path<info::SayHello>) -> Result<JsonResponse, UserError> {
    let name = info.name.to_string();

    if name.chars().count() <= 1 as usize {
        return Err(UserError::ValidationError {
            field: name + ", please write a valid name",
        });
    }

    Ok(JsonResponse {
        ok: true,
        data: format!("hello, {}", name),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/{name}", web::get().to(say_hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
