FROM rust:1 as base

##################################################################################
FROM base as dev
WORKDIR /backends/rust-axum-diesel

# RUN cargo install cargo-watch

EXPOSE 3000

CMD cd /backends/rust-axum-diesel/; \
    cargo run;

##################################################################################
FROM base as deps-and-code
WORKDIR /backends/rust-axum-diesel

COPY backends/rust-axum-diesel/Cargo.toml backends/rust-axum-diesel/Cargo.lock ./
RUN mkdir backends/ ; \
    echo "fn main() {}" > ./backends/main.rs
RUN cargo build --release --locked
RUN rm -rf ./backends
COPY backends/rust-axum-diesel/ ./

##################################################################################
FROM deps-and-code as cargo-test
WORKDIR /backends/rust-axum-diesel
CMD ["cargo", "test"]

##################################################################################
FROM deps-and-code as prod
WORKDIR /backends/rust-axum-diesel

RUN cargo install --offline --path .

# CMD cargo run --release --offline
# CMD tail -f /dev/null;
CMD ["rust-axum-diesel"]

##################################################################################
# FROM debian:bullseye-slim as prod

# RUN apt-get update && apt-get install -y build-essential curl htop libc6 && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/rust-axum-diesel /usr/local/bin/rust-axum-diesel

# # CMD ["rust-axum-diesel"]
# CMD tail -f /dev/null;
