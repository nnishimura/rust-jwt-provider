use crate::db::schema::refresh_tokens;
use crate::db::schema::refresh_tokens::dsl::refresh_tokens as refresh_tokens_table;
use chrono::NaiveDateTime;
use diesel::result::Error as DieselError;
use diesel::*;
use diesel::{Identifiable, Insertable, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Identifiable, Queryable, PartialEq, Serialize, Deserialize)]
#[table_name = "refresh_tokens"]
pub struct RefreshToken {
    pub id: Uuid,
    pub client_id: Uuid,
    pub active: bool,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken {
    pub client_id: Uuid,
    pub active: bool,
}

pub fn create_refresh_token(
    new_access_token: &NewRefreshToken,
    conn: &PgConnection,
) -> Result<RefreshToken, DieselError> {
    diesel::insert_into(refresh_tokens_table)
        .values(new_access_token)
        .get_result::<RefreshToken>(conn)
        .map_err(Into::into)
}
