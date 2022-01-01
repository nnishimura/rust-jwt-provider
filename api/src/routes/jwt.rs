use crate::routes::error::APIErrorResponse;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{ready, Ready};
use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};
use service::introspect::token_introspect;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntrospectRequest {
    token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IntrospectErrorResponse {
    active: bool,
}

/*
Token Introspection Response definition https://tools.ietf.org/html/rfc7662#section-2.2
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtIntrospectResponse {
    pub active: bool,
    // pub scope: Option<String>,
    // pub client_id: Option<String>,
    // pub username: Option<String>,
    // pub token_type: Option<String>,
    // pub exp: Option<usize>,
    // pub iat: Option<usize>,
    // pub nbf: Option<usize>,
    // pub sub: Option<String>,
    // pub aud: Option<String>,
    // pub iss: Option<String>,
    // pub jti: Option<String>,
}

#[post("/jwt/introspect")]
pub async fn introspect(input: web::Json<IntrospectRequest>) -> impl Responder {
    web::Json(JwtIntrospectResponse { active: true })
}

// impl From<ServiceIntrospectResponse> for TokenIntrospectResponse {
//     fn from(res: ServiceIntrospectResponse) -> TokenIntrospectResponse {
//         TokenIntrospectResponse {
//             active: true,
//             scope: res.scope,
//             client_id: Some(res.client_id),
//             username: Some(res.user_name),
//             token_type: None,
//             exp: res.exp,
//             iat: res.iat,
//             nbf: None,
//             sub: res.sub,
//             aud: res.aud,
//             iss: Some(res.iss),
//             jti: res.jti,
//             contract_id: res.contract_id,
//             device_id: res.device_id,
//             application_id: res.application_id,
//             subscription_status: res.subscription_status,
//         }
//     }
// }
