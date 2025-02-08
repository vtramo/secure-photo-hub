FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release

COPY ./.sqlx ./.sqlx
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./queries/postgres ./queries/postgres

ENV DATABASE_URL=''
ENV SQLX_OFFLINE=true
RUN cargo build --release
RUN mv ./target/release/secure-photo-hub ./app

FROM debian:stable-slim AS runtime
WORKDIR /app
ENV DATABASE_URL=''
ENV CONFIG_LOCATION=/config/application-properties.yaml
ENV SECRETS_LOCATION=/config/application-secrets.yaml
COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]