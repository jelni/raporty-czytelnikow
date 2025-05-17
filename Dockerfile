FROM rust:slim AS bot-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=target \
    cargo build --release \
    && cp target/release/raporty-czytelnikow raporty-czytelnikow

FROM debian:stable-slim
COPY --from=bot-builder /app/raporty-czytelnikow /app/
ENTRYPOINT ["/app/raporty-czytelnikow"]
