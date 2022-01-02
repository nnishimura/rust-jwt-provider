use crate::db::client::{create_client, Client, NewClient};
use diesel::{result::Error as DieselError, PgConnection};

pub async fn create_new_client(
    input: &NewClient,
    conn: &PgConnection,
) -> Result<Client, DieselError> {
    create_client(input, conn)
}
