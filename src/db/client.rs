use crate::db::schema::clients;
use crate::db::schema::clients::dsl::{client_name, clients as client_table};
use chrono::NaiveDateTime;
use diesel::result::Error as DieselError;
use diesel::*;
use diesel::{Identifiable, Insertable, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Identifiable, Queryable, PartialEq, Serialize, Deserialize)]
#[table_name = "clients"]
pub struct Client {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub client_name: String,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "clients"]
pub struct NewClient {
    pub client_name: String,
    pub tenant_id: Uuid,
}

pub fn create_client(new_client: &NewClient, conn: &PgConnection) -> Result<Client, DieselError> {
    diesel::insert_into(client_table)
        .values(new_client)
        .get_result::<Client>(conn)
        .map_err(Into::into)
}

pub fn get_client_by_name(cn: &str, conn: &PgConnection) -> Result<Client, DieselError> {
    client_table
        .filter(client_name.eq(cn))
        .get_result::<Client>(conn)
        .map_err(Into::into)
}
