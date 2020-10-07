FROM rustlang/rust:nightly-stretch

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

EXPOSE 8000

CMD "./target/release/web_server"
