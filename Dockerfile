FROM rustlang/rust:nightly as builder

RUN USER=root cargo new --bin web_server
WORKDIR ./web_server
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY src ./src
COPY diesel.toml ./diesel.toml
COPY rust-toolchain ./rust-toolchain



RUN rm ./target/release/deps/recipes_backend*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /web_server/target/release/web_server ${APP}/web_server

# https://docs.docker.com/compose/startup-order/
ADD https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh ${APP}/wait-for-it.sh
RUN chmod +x ${APP}/wait-for-it.sh

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./web_server"]
