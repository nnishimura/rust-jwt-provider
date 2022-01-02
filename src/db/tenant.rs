use crate::db::schema::tenants;
use crate::db::schema::tenants::dsl::{id, tenant_name, tenants as tenant_table};
use chrono::NaiveDateTime;
use diesel::result::Error as DieselError;
use diesel::*;
use diesel::{Identifiable, Insertable, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Identifiable, Queryable, PartialEq, Serialize, Deserialize)]
#[table_name = "tenants"]
pub struct Tenant {
    pub id: Uuid,
    pub tenant_name: String,
    pub issuer: String,
    pub access_token_ttl: i32,
    pub refresh_token_ttl: i32,
    pub created_datetime: NaiveDateTime,
    pub modified_datetime: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "tenants"]
pub struct NewTenant {
    pub tenant_name: String,
    pub issuer: String,
    pub access_token_ttl: i32,
    pub refresh_token_ttl: i32,
}

pub fn create_tenant(new_tenant: &NewTenant, conn: &PgConnection) -> Result<Tenant, DieselError> {
    diesel::insert_into(tenant_table)
        .values(new_tenant)
        .get_result::<Tenant>(conn)
        .map_err(Into::into)
}

pub fn get_tenant_by_id(tenant_id: &Uuid, conn: &PgConnection) -> Result<Tenant, DieselError> {
    tenant_table
        .filter(id.eq(tenant_id))
        .get_result::<Tenant>(conn)
        .map_err(Into::into)
}

pub fn get_tenant_by_name(cn: &str, conn: &PgConnection) -> Result<Tenant, DieselError> {
    tenant_table
        .filter(tenant_name.eq(cn))
        .get_result::<Tenant>(conn)
        .map_err(Into::into)
}
