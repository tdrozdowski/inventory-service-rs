# Inventory Service - Rust

![Current Build](https://github.com/tdrozdowski/inventory-service-rs/actions/workflows/main.yml/badge.svg)
[![codecov](https://codecov.io/github/tdrozdowski/inventory-service-rs/graph/badge.svg?token=GG5U2E7ZON)](https://codecov.io/github/tdrozdowski/inventory-service-rs)

A sample RESTful API written with Rust and Axum. Leverages the SQLX project for data access to Postgres.

Useful for learning all the above as well as trying out new things over time.

## Getting Started

As this is a Rust project, its recommended that you have `rustup` installed. You can install it with the following:

```bash
brew install rustup-init
rustup-init
```

That should install the latest stable version of Rust. If you want to use nightly, you can install it with the
following:

```bash
rustup toolchain install nightly --allow-downgrade
```

This project uses SQLX to interact with a Postgres database. You will need to install `sqlx-cli` to run the migrations.

```bash
cargo install sqlx-cli
```

To run the migrations, you will need to have a Postgres database running.

There are two files you need (which are part of the .gitignore) to run the database.
First is the `.env` file which should look like the following:

```
DATABASE_URL=postgres://postgres:password@localhost/inventory
```

Next is the `database.env` file which should look like the following:

```
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_DB=inventory
``` 

These are not stored in the repository for security reasons. You can create these files in the root of the project.

After you have created the above two files, you can now start the database with the following:

```bash
docker compose up -d db
```

After that, you can run the migrations with the following:

```bash
sqlx database setup
```

Now you can run the application with the following:

```bash
export JWT_SECRET=secret
export RUST_LOG=debug
cargo run
```

Test out the API endpoints using the Jetbrains REST Client or Postman.

Jetbrains REST Client example:

First generate a token for your session and the default user/password:

    ```http
    POST http://localhost:8080/auth/login
    Content-Type: application/json
    
    {
        "client_id": "foo",
        "client_secret": "bar"
    }
    ```

Then use the token to access the protected endpoints:

    ```http
    GET http://localhost:8080/inventory
    Authorization Bearer <token here>
    ```

There are a set of files in the http directory that you can use to test the API.

## Running Tests

You will need to export at least the `JWT_SECRET` environment variable to run the tests. You can do this with the
following:

```bash
export JWT_SECRET=secret
cargo test
```

That should set you up to execute the entire test suite - assuming the postgres database is running. (see above)

## OpenAPI

This API uses OpenAPI to document the endpoints. You can view the documentation by starting the server locally and then
opening the following:

https://localhost:3000/redoc

This will show you the endpoints and the expected request/response bodies.
