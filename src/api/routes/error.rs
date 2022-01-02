use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Display)]
pub enum AppError {
    EnvironmentVariableError(String),
    DBError(String),
    UnhandledError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIErrorResponse {
    pub code: String,
    pub message: String,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        error!("Error occurred during request processing: {:?}", body);

        match self {
            AppError::UnhandledError => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(APIErrorResponse {
                    code: "UNHANDLED_ERROR".to_string(),
                    message: body,
                }),
            AppError::EnvironmentVariableError(_) => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(APIErrorResponse {
                    code: "ENVIRONMENT_VARIABLE_ERROR".to_string(),
                    message: body,
                }),
            AppError::DBError(_) => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(APIErrorResponse {
                    code: "DB_ERROR".to_string(),
                    message: body,
                }),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::UnhandledError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EnvironmentVariableError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
