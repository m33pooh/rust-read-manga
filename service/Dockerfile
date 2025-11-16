# builder
FROM rust:latest as builder
WORKDIR /usr/src/simple_api

# Copy manifest and source
COPY ./simple_api/Cargo.toml ./
COPY ./simple_api/Cargo.lock ./
COPY ./simple_api/src ./src

# Build in release
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/simple_api/target/release/simple_api /usr/local/bin/simple_api

ENV RUST_LOG=info
EXPOSE 8080
CMD ["/usr/local/bin/simple_api"]
