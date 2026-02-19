FROM rust:1.88-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Install trunk and WASM target
RUN cargo install trunk --locked
RUN rustup target add wasm32-unknown-unknown

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY crates/dashboard/Cargo.toml crates/dashboard/Cargo.toml
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN mkdir -p crates/dashboard/src && echo '#[allow(dead_code)] fn main() {}' > crates/dashboard/src/lib.rs
RUN mkdir -p dist
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src crates/dashboard/src dist

# Copy full source
COPY src ./src
COPY crates/dashboard ./crates/dashboard

# Build dashboard WASM first
RUN cd crates/dashboard && trunk build --release

# Build backend (embeds dist/ via rust-embed)
RUN cargo build --release

# Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/reeverb /usr/local/bin/reeverb

EXPOSE 3000

CMD ["reeverb"]
