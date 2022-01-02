# rust-jwt-provider

JWT provider written in Rust.

## Tech stacks

* Language: Rust 
* Framework: [actix-web](https://github.com/actix/actix-web)
* Database: postgres
* ORM: [Prisma](https://www.prisma.io/)
* Payload validation: [celebrate](https://www.npmjs.com/package/celebrate)
* JWT token: [jsonwebtoken](https://www.npmjs.com/package/jsonwebtoken)
* Test: jest, [supertest](https://www.npmjs.com/package/supertest)

## Features
* issue token endpoint
* introspect endpoint
* refresh token endpoint
* multi-tenant support 

### Getting started
To run app on docker:
1. Copy `.env.sample` and rename to `.env`.
2. Run `docker-compose up`
3. Run `cargo run`
4. App should be accessible at http://localhost:5555

### Testing

```
cargo test
```
