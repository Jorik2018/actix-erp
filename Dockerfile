# Etapa 1: build
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .

RUN cargo build --release

# Etapa 2: runtime
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/actix-erp /usr/local/bin/actix-erp

EXPOSE 8080

CMD ["actix-erp"]