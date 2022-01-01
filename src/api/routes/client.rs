use crate::db::client::{Client, NewClient};
use crate::service::client::create_new_client;
use crate::service::introspect::token_introspect;
use crate::{api::routes::error::ApplicationError, AppState};
use actix_web::{web, Responder};
use chrono::NaiveDateTime;
use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Jwks {
    kty: String,
    e: String,
    r#use: String,
    kid: String,
    alg: String,
    n: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClientRequest {
    client_name: String,
    audience: Vec<String>,
    jwks: Jwks,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponse {
    pub id: Uuid,
    pub client_name: String,
    pub audience: Value,
    pub jwks: Option<Value>,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

impl From<Client> for ClientResponse {
    fn from(c: Client) -> ClientResponse {
        ClientResponse {
            id: c.id,
            client_name: c.client_name,
            audience: c.audience,
            jwks: c.jwks,
            created_datetime: c.created_datetime,
            modified_datetime: c.modified_datetime,
        }
    }
}

impl From<web::Json<CreateClientRequest>> for NewClient {
    fn from(c: web::Json<CreateClientRequest>) -> NewClient {
        let new_client = c.clone();
        NewClient {
            client_name: new_client.client_name,
            audience: serde_json::to_value(new_client.audience).unwrap(),
            jwks: Some(serde_json::to_value(new_client.jwks).unwrap()),
        }
    }
}

#[post("/internal/clients")]
pub async fn create_client(
    data: web::Data<AppState>,
    input: web::Json<CreateClientRequest>,
) -> Result<web::Json<ClientResponse>, ApplicationError> {
    let pool = &data.pool;
    let conn = pool
        .get()
        .map_err(|e| ApplicationError::DBError(e.to_string()))?;
    let new_client = NewClient::from(input);
    let token = create_new_client(&new_client, &conn)
        .await
        .map_err(|e| ApplicationError::DBError(e.to_string()))?;
    let response = ClientResponse::from(token);
    Ok(web::Json(response))
}
