# Recipe API

API to expose recipe stored in Postgres

## Requirements

[Install RUST](https://www.rust-lang.org/en-US/install.html):

```sh
curl https://sh.rustup.rs -sSf | sh
rustup component add clippy
rustup component add rustfmt
```

## Commands

```sh
cargo build # compile
cargo run --bin web_server # run the server
cargo test # run the tests
cargo fmt # format the code
cargo clippy # run the linter
```

## Run locally

-   run postgreSQL locally
-   create a file name `.env` with the following content

```bash
DATABASE_URL=postgres://localhost/diesel_demo
```

-   run `cargo run --bin web_server` to start the server and go to [localhost:8000](http://localhost:8000/)
