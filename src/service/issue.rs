use crate::{
    api::routes::{error::AppError, jwt::IssueRequest},
    db::{
        access_token::{create_access_token, NewAccessToken},
        client::get_client_by_name,
    },
};
use diesel::PgConnection;
use jsonwebtoken::{Algorithm, Header};
use serde::{Deserialize, Serialize};
use std::env;

pub async fn issue_token(input: &IssueRequest, conn: &PgConnection) -> Result<String, AppError> {
    // let issuer = env::var("AUTH_ISSUE_ISSUER must be set");
    // let client = get_client_by_name(&input.client_name, conn)?;

    // let token = encode(&Header::new(Algorithm::RS256), &my_claims, &client.jwks?)?;

    // create_access_token(NewAccessToken {

    // });
    Ok("token".to_string())
}
