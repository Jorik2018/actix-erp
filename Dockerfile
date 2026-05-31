# Etapa 1: build
FROM rust:1.89-bookworm AS builder

WORKDIR /app

# Copiar manifests primero para aprovechar cache
COPY Cargo.toml Cargo.lock ./

# Crear un main temporal para cachear dependencias
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copiar el resto del proyecto
COPY . .

# Compilar binario real
RUN cargo build --release

# Etapa 2: runtime
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/actix-erp /usr/local/bin/actix-erp

EXPOSE 8080

CMD ["actix-erp"]