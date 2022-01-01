use std::env::VarError;

use jsonwebtoken::errors::{Error, ErrorKind};

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceError {
    UnHandledError,
    TokenInvalid,
    TokenExpired,
    TokenValidationFailed,
    ClientInvalid,
}

impl From<Error> for ServiceError {
    fn from(e: Error) -> ServiceError {
        match e.kind() {
            ErrorKind::InvalidSignature
            | ErrorKind::InvalidToken
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::InvalidAlgorithmName => ServiceError::TokenInvalid,
            ErrorKind::ExpiredSignature => ServiceError::TokenExpired,
            _ => ServiceError::TokenValidationFailed,
        }
    }
}
