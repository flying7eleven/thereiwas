# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:latest AS cargo-build

# RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY . /usr/src/app

#RUN cargo build --release --target x86_64-unknown-linux-musl

RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
#FROM alpine:latest

FROM debian:stable-slim

EXPOSE 3000/tcp

WORKDIR /usr/local/bin

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "thereiwas"

RUN apt update && apt install -y libpq5 curl

RUN mkdir /var/log/thereiwas && \
    touch /var/log/thereiwas/server.log && \
    touch /var/log/thereiwas/unprocessable_entities.log && \
    chown thereiwas:thereiwas -R /var/log/thereiwas && \
    mkdir /usr/local/thereiwas && \
    chown thereiwas:thereiwas -R /usr/local/thereiwas

USER thereiwas:thereiwas

# COPY --from=cargo-build --chown=thereiwas:thereiwas /usr/src/app/target/x86_64-unknown-linux-musl/release/thereiwas /usr/local/bin/thereiwas

COPY --from=cargo-build --chown=thereiwas:thereiwas /usr/src/app/target/release/thereiwas /usr/local/bin/thereiwas

ENTRYPOINT ["/usr/local/bin/thereiwas"]
