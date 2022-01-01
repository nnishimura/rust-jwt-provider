use crate::db::access_token::{create_access_token, NewAccessToken};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::service::error::ServiceError;

pub async fn issue_token(token: &str) -> Result<String, ServiceError> {
    // create_access_token(NewAccessToken {

    // });
    Ok("token".to_string())
}

pub struct ServiceIntrospectResponse {
    active: bool,
}
