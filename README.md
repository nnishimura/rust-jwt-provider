# rust-jwt-provider

JWT provider application written in Rust.

## Tech stacks

* Language: Rust 
* Framework: [actix-web](https://github.com/actix/actix-web)
* Database: postgres
* ORM/DB migration: [diesel](https://diesel.rs/guides/getting-started.html)
* JWT token: [jsonwebtoken](https://github.com/Keats/jsonwebtoken)

## Features
* issue token endpoint
* introspect endpoint
* multi-tenant support
* TODO: refresh token endpoint

## Getting started

### Prerequisites
* Rust 1.42 or later
* Cargo 
* Docker
* [Diesel CLI](https://diesel.rs/guides/getting-started.html)

### Running on local
1. Copy `.env.sample` and rename to `.env`.
2. Run `docker-compose up`
3. Run `diesel migration run`
4. Run `cargo run`
4. App should be accessible at http://localhost:5555

### Testing

```
cargo test
```
