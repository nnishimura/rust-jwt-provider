use crate::db::tenant::create_tenant as create_new_tenant;
use crate::db::tenant::{NewTenant, Tenant};
use crate::{api::routes::error::AppError, AppState};
use actix_web::web;
use chrono::NaiveDateTime;
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTenantRequest {
    pub tenant_name: String,
    pub issuer: String,
    pub access_token_ttl: i32,
    pub refresh_token_ttl: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantResponse {
    pub id: Uuid,
    pub tenant_name: String,
    pub issuer: String,
    pub access_token_ttl: i32,
    pub refresh_token_ttl: i32,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

impl From<Tenant> for TenantResponse {
    fn from(c: Tenant) -> TenantResponse {
        TenantResponse {
            id: c.id,
            tenant_name: c.tenant_name,
            issuer: c.issuer,
            access_token_ttl: c.access_token_ttl,
            refresh_token_ttl: c.refresh_token_ttl,
            created_datetime: c.created_datetime,
            modified_datetime: c.modified_datetime,
        }
    }
}

impl From<web::Json<CreateTenantRequest>> for NewTenant {
    fn from(c: web::Json<CreateTenantRequest>) -> NewTenant {
        let new_tenant = c.clone();
        NewTenant {
            tenant_name: new_tenant.tenant_name,
            issuer: new_tenant.issuer,
            access_token_ttl: new_tenant.access_token_ttl,
            refresh_token_ttl: new_tenant.refresh_token_ttl,
        }
    }
}

#[post("/internal/tenants")]
pub async fn create_tenant(
    data: web::Data<AppState>,
    input: web::Json<CreateTenantRequest>,
) -> Result<web::Json<TenantResponse>, AppError> {
    info!("incoming request for POST /tenants: {:?}", input);
    let pool = &data.pool;
    let conn = pool.get().map_err(|e| AppError::DBError(e.to_string()))?;
    let new_tenant = NewTenant::from(input);
    let token =
        create_new_tenant(&new_tenant, &conn).map_err(|e| AppError::DBError(e.to_string()))?;
    let response = TenantResponse::from(token);
    Ok(web::Json(response))
}
