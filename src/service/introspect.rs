use crate::db::tenant::get_tenant_by_issuer;
use diesel::PgConnection;
use jsonwebtoken::{dangerous_insecure_decode, decode, DecodingKey, Validation};

use crate::api::routes::{error::AppError, jwt::JwtIntrospectResponse};

use crate::service::issue::JwtClaim;

pub fn token_introspect(
    token: &str,
    conn: &PgConnection,
) -> Result<JwtIntrospectResponse, AppError> {
    // get token issuer
    let token_message = dangerous_insecure_decode::<JwtClaim>(token)?;
    let issuer = token_message.claims.iss;
    if issuer.is_none() {
        return Err(AppError::InvalidJwt("empty issuer in claim".to_string()));
    }
    let tenant = get_tenant_by_issuer(&issuer.unwrap(), conn)?;
    let token_data = decode::<JwtClaim>(
        token,
        &DecodingKey::from_secret(tenant.id.to_string().as_bytes()),
        &Validation::default(),
    );

    if let Ok(data) = token_data {
        let claims = data.claims;
        Ok(JwtIntrospectResponse {
            active: true,
            scope: None,
            client_id: claims.client_id,
            username: None,
            token_type: None,
            exp: claims.exp,
            iat: claims.iat,
            nbf: claims.nbf,
            sub: claims.sub,
            aud: claims.aud,
            iss: claims.iss,
            jti: claims.jti,
        })
    } else {
        Ok(JwtIntrospectResponse {
            active: false,
            scope: None,
            client_id: None,
            username: None,
            token_type: None,
            exp: None,
            iat: None,
            nbf: None,
            sub: None,
            aud: None,
            iss: None,
            jti: None,
        })
    }
}
