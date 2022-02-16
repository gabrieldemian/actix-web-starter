use crate::{JsonResponse, UserError};
use actix_web::web::{self, Path, ServiceConfig};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SayHello {
    pub name: String,
}

pub async fn say_hello(info: Path<SayHello>) -> Result<JsonResponse, UserError> {
    let name = info.name.to_string();

    if name.chars().count() <= 1 {
        return Err(UserError::ValidationError {
            field: name + ", please write a valid name",
        });
    }

    Ok(JsonResponse {
        ok: true,
        data: format!("hello, {}", name),
    })
}

pub fn route(cfg: &mut ServiceConfig) {
    cfg.route("/{name}", web::get().to(say_hello));
}
