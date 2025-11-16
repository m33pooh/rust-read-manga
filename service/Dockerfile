# builder
FROM rust:latest as builder
WORKDIR /usr/src/rust-read-manga

# Copy manifest and source
COPY ./rust-read-manga/Cargo.toml ./
COPY ./rust-read-manga/Cargo.lock ./
COPY ./rust-read-manga/src ./src

# Install FFmpeg dependencies
RUN apt-get update && apt-get install -y \
    ffmpeg \
    libavformat-dev \
    libavcodec-dev \
    libavutil-dev \
    libswscale-dev \
    libswresample-dev \
    libavdevice-dev \
    libavfilter-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Build in release with all features
RUN cargo build --release --all-features

# Runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/rust-read-manga/target/release/rust-read-manga /usr/local/bin/rust-read-manga

ENV RUST_LOG=info
EXPOSE 8080
CMD ["/usr/local/bin/rust-read-manga"]
