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

USER thereiwas:thereiwas

# COPY --from=cargo-build --chown=thereiwas:thereiwas /usr/src/app/target/x86_64-unknown-linux-musl/release/thereiwas /usr/local/bin/thereiwas

COPY --from=cargo-build --chown=thereiwas:thereiwas /usr/src/app/target/release/thereiwas /usr/local/bin/thereiwas

ENTRYPOINT ["/usr/local/bin/thereiwas"]