# Inventory Service - Rust

![Current Build](https://github.com/tdrozdowski/inventory-servce-rs/actions/workflows/rust.yml/badge.svg)
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