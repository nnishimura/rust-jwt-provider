use crate::{
    api::routes::{error::AppError, jwt::IssueRequest},
    db::{
        access_token::{create_access_token, mark_as_inactive_by_client, NewAccessToken},
        client::get_client_by_name,
        tenant::get_tenant_by_id,
    },
};
use chrono::Utc;
use diesel::PgConnection;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// @see https://datatracker.ietf.org/doc/html/rfc7519#section-4.1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaim {
    pub exp: Option<i64>,
    pub iat: Option<i64>,
    pub nbf: Option<i64>,
    pub sub: Option<String>,
    pub aud: Option<String>,
    pub iss: Option<String>,
    pub jti: Option<String>,
    pub client_id: Option<String>,
}

pub async fn issue_token(input: &IssueRequest, conn: &PgConnection) -> Result<String, AppError> {
    let client = get_client_by_name(&input.client_name, conn)?;
    let tenant = get_tenant_by_id(&client.tenant_id, conn)?;
    let token = create_access_token(
        &NewAccessToken {
            client_id: client.id,
            active: true,
            attributes: None,
        },
        conn,
    )?;

    mark_as_inactive_by_client(&client.id, conn)?;

    let jwt_claim = JwtClaim {
        exp: Some(Utc::now().timestamp() + i64::from(tenant.access_token_ttl)),
        iat: Some(Utc::now().timestamp()),
        nbf: None,
        sub: None,
        aud: Some(client.client_name),
        iss: Some(tenant.issuer),
        jti: Some(token.id.to_string()),
        client_id: Some(client.id.to_string()),
    };

    let token = encode(
        &Header::default(),
        &jwt_claim,
        &EncodingKey::from_secret(tenant.id.to_string().as_bytes()),
    )?;

    Ok(token)
}
