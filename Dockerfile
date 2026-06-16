FROM rust:1.87-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/truelayer_pokedex /usr/local/bin/truelayer_pokedex

EXPOSE 5000

CMD ["truelayer_pokedex"]
