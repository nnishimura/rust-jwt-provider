#[macro_use]
extern crate actix_web;
extern crate dotenv;
extern crate serde_json;

use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::future::{ready, Ready};
use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};
mod routes;
use crate::routes::jwt::introspect;

#[get("/healthcheck")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().service(hello).service(introspect))
        .bind("127.0.0.1:5555")?
        .run()
        .await
}
