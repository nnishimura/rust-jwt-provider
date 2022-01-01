use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::service::error::ServiceError;

pub async fn token_introspect(
    token: &str,
    allowed_client_ids: &str,
    issuer: &str,
) -> Result<ServiceIntrospectResponse, ServiceError> {
    Ok(ServiceIntrospectResponse { active: true })
}

pub struct ServiceIntrospectResponse {
    active: bool,
}
