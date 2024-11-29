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
RUN mv ./target/release/secure-photo-hub ./app

FROM debian:stable-slim AS runtime
WORKDIR /app
ENV CONFIG_LOCATION=/config/application-properties.yaml
ENV VAULT_SECRETS_LOCATION=/config/vault-secrets.yaml
COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]