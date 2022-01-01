#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;

mod api;
mod db;
mod service;

use crate::api::routes::client::create_client;
use crate::api::routes::jwt::{introspect, issue};
use crate::db::create_pool;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub struct AppState {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                pool: create_pool(),
            })
            .service(healthcheck)
            .service(introspect)
            .service(issue)
            .service(create_client)
    })
    .bind("127.0.0.1:5555")?
    .run()
    .await
}
