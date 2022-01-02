use crate::api::routes::error::AppError;
use crate::service::introspect::token_introspect;
use crate::service::issue::issue_token;
use crate::AppState;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntrospectRequest {
    token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRequest {
    pub refresh_token: Option<String>,
    pub client_name: String, // mywebapp.com
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueResponse {
    token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntrospectErrorResponse {
    active: bool,
}

/*
Token Introspection Response https://tools.ietf.org/html/rfc7662#section-2.2
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtIntrospectResponse {
    pub active: bool,
    pub scope: Option<String>,
    pub client_id: Option<String>,
    pub username: Option<String>,
    pub token_type: Option<String>,
    pub exp: Option<i64>,
    pub iat: Option<i64>,
    pub nbf: Option<i64>,
    pub sub: Option<String>,
    pub aud: Option<String>,
    pub iss: Option<String>,
    pub jti: Option<String>,
}

#[post("/jwt/issue")]
pub async fn issue(
    data: web::Data<AppState>,
    input: web::Json<IssueRequest>,
) -> Result<web::Json<IssueResponse>, AppError> {
    let pool = &data.pool;
    let conn = pool.get().map_err(|e| AppError::DBError(e.to_string()))?;

    let token = issue_token(&input.0, &conn).await.unwrap();
    Ok(web::Json(IssueResponse { token }))
}

#[post("/jwt/introspect")]
pub async fn introspect(
    data: web::Data<AppState>,
    input: web::Json<IntrospectRequest>,
) -> Result<web::Json<JwtIntrospectResponse>, AppError> {
    let pool = &data.pool;
    let conn = pool.get().map_err(|e| AppError::DBError(e.to_string()))?;

    let res = token_introspect(&input.token, &conn)?;
    Ok(web::Json(res))
}
