FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

RUN cargo build --release
RUN mv ./target/release/rust-oauth2-openid-keycloak ./app

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]