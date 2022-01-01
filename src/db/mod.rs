use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub mod access_token;
pub mod client;
pub mod refresh_token;
pub mod schema;

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(4) // actix-web creates 4 threads on start, adjust this value not to exceed postgres max connection numbers
        .build(manager)
        .expect("Failed to create pool")
}
