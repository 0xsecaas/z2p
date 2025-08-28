########## Builder + cache priming ##########
FROM rust:1.88.0 AS chef 
RUN cargo install cargo-chef --locked
WORKDIR /app
COPY Cargo.toml Cargo.lock ./ 
RUN cargo chef prepare --recipe-path recipe.json 

FROM rust:1.88.0 AS cacher 
WORKDIR /app 
RUN cargo install cargo-chef --locked
COPY --from=chef /app/recipe.json recipe.json 
RUN cargo chef cook --release --recipe-path recipe.json


########## Build z2p ##########
FROM rust:1.88.0 AS builder 
WORKDIR /app
ENV SQLX_OFFLINE=true

# Bring back dep build cache
COPY --from=cacher /app/target target
COPY . .
# Optional: speed up incremental linking
# ENV RUSTFLAGS="-C link-arg=-Wl,-O1"
# Keep cache mount only for dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release --locked

# ^ BuildKit mounts avoid re-downloading registry and re-compiling unchanged units


########## Minimal runtime ##########
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install system libs your binary needs (e.g., ca-certificates, tzdata)
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod /app/zero2prod
COPY --from=builder /app/configuration.yaml /app/configuration.yaml
RUN chmod +x /app/zero2prod
USER 1000
ENTRYPOINT ["/app/zero2prod"]
