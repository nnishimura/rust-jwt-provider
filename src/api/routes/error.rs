use actix_web::{
    dev::HttpResponseBuilder, error, get, http::header, http::StatusCode, App, HttpResponse,
};
use derive_more::{Display, Error};
use diesel::result::Error as DieselError;
use futures::future::{ready, Ready};
use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Display)]
pub enum ApplicationError {
    EnvironmentVariableError(String),
    DBError(String),
    UnhandledError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIErrorResponse {
    pub code: String,
    pub message: String,
}

impl error::ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        error!("Error occurred during request processing: {:?}", body);

        match self {
            ApplicationError::UnhandledError => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(APIErrorResponse {
                    code: "UNHANDLED_ERROR".to_string(),
                    message: body,
                }),
            ApplicationError::EnvironmentVariableError(_) => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(APIErrorResponse {
                    code: "ENVIRONMENT_VARIABLE_ERROR".to_string(),
                    message: body,
                }),
            ApplicationError::DBError(_) => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(APIErrorResponse {
                    code: "DB_ERROR".to_string(),
                    message: body,
                }),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApplicationError::UnhandledError => StatusCode::INTERNAL_SERVER_ERROR,
            ApplicationError::EnvironmentVariableError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApplicationError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
