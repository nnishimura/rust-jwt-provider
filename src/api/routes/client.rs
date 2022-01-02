use crate::db::client::{Client, NewClient};
use crate::service::client::create_new_client;
use crate::{api::routes::error::AppError, AppState};
use actix_web::web;
use chrono::NaiveDateTime;
use log::info;
use serde::{Deserialize, Serialize};
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
    tenant_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponse {
    pub id: Uuid,
    pub client_name: String,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

impl From<Client> for ClientResponse {
    fn from(c: Client) -> ClientResponse {
        ClientResponse {
            id: c.id,
            client_name: c.client_name,
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
            tenant_id: new_client.tenant_id,
        }
    }
}

#[post("/internal/clients")]
pub async fn create_client(
    data: web::Data<AppState>,
    input: web::Json<CreateClientRequest>,
) -> Result<web::Json<ClientResponse>, AppError> {
    info!("incoming request for POST /clients: {:?}", input);
    let pool = &data.pool;
    let conn = pool.get().map_err(|e| AppError::DBError(e.to_string()))?;
    let new_client = NewClient::from(input);
    let token = create_new_client(&new_client, &conn)
        .await
        .map_err(|e| AppError::DBError(e.to_string()))?;
    let response = ClientResponse::from(token);
    Ok(web::Json(response))
}
