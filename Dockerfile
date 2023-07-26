FROM rust:1.71.0 AS builder

COPY . .

RUN rustup default nightly
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

RUN mkdir -p /app

COPY --from=builder /target/release/rocket_lfs /app/rocket_lfs
COPY /Rocket.toml /app/Rocket.toml

RUN apt-get update
RUN apt-get install -y default-libmysqlclient-dev