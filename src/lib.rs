use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{Error, StatusCode},
    HttpRequest, HttpResponse, Responder,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "validation error on field: {}", field)]
    ValidationError { field: String },

    #[display(fmt = "an internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&JsonResponse {
            ok: false,
            data: self.to_string(),
        })
        .unwrap();

        HttpResponseBuilder::new(self.status_code())
            .content_type("application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(body)
    }

    /* match an user error to an HTTP error code */
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse {
    pub ok: bool,
    pub data: String,
}

impl Responder for JsonResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(body)))
    }
}
