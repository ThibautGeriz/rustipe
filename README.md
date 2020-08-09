# Recipe API

API to expose recipe stored in Postgres

## Requirements

[Install RUST](https://www.rust-lang.org/en-US/install.html):

```bash
curl https://sh.rustup.rs -sSf | sh
rustup default nightly # use nightly for rocket
rustup component add clippy # install linter
rustup component add rustfmt # install formatter
cargo install diesel_cli # install ORM CLI
echo "DATABASE_URL=postgres://localhost/rustipe
JWT_SECRET=secret" > .env # setup local conf
```

You will also need to [install postgresSQL](https://www.postgresql.org/download/) and init 2 database

```bash
diesel setup --database-url postgres://localhost/rustipe
```

## Commands

```bash
cargo build # compile
cargo run --bin web_server # run the server
cargo test -- --test-threads=1 # run the tests
cargo fmt # format the code
cargo clippy # run the linter
```

## Run locally

### Server

-   run `cargo run --bin web_server` to start the server and go to [localhost:8000](http://localhost:8000/)

### Tests

-   run `cargo test -- --test-threads=1`
