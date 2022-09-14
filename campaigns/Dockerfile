FROM rust:1.63.0-bullseye as builder

WORKDIR /app

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo install --path .

FROM debian:11.5-slim as webserver
COPY --from=builder /usr/local/cargo/bin/campaigns /usr/local/bin/campaigns-api
EXPOSE 8080
CMD ["campaigns-api"]
