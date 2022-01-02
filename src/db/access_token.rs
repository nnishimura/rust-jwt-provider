use crate::db::schema::access_tokens;
use crate::db::schema::access_tokens::dsl::access_tokens as access_tokens_table;
use chrono::NaiveDateTime;
use diesel::result::Error as DieselError;
use diesel::*;
use diesel::{Identifiable, Insertable, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Identifiable, Queryable, PartialEq, Serialize, Deserialize)]
#[table_name = "access_tokens"]
pub struct AccessToken {
    pub id: Uuid,
    pub client_id: Uuid,
    pub active: bool,
    pub attributes: Option<Value>,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "access_tokens"]
pub struct NewAccessToken {
    pub client_id: Uuid,
    pub attributes: Option<Value>,
    pub active: bool,
}

pub fn create_access_token(
    new_access_token: &NewAccessToken,
    conn: &PgConnection,
) -> Result<AccessToken, DieselError> {
    diesel::insert_into(access_tokens_table)
        .values(new_access_token)
        .get_result::<AccessToken>(conn)
        .map_err(Into::into)
}
