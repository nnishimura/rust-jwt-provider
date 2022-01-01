use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{ready, Ready};
use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum APIErrorResponse {
    ServiceError(String),
    UnhandledError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl Responder for APIErrorResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        error!("Error occurred during request processing: {:?}", body);

        match self {
            APIErrorResponse::UnhandledError => ready(Ok(HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(ErrorResponse {
                    code: "UNHANDLED_ERROR".to_string(),
                    message: body,
                }))),
            APIErrorResponse::ServiceError(_) => todo!(),
        }
    }
}
