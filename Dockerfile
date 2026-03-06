FROM rust:1.80-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy source code
COPY . .

# Build the release binary
RUN cargo build --release --bin doc_transformer

# Final stage - use distroless or debian slim for a tiny image
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL is needed dynamically unless statically linked)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/doc_transformer /usr/local/bin/doc_transformer

WORKDIR /data

# Set the entrypoint so it acts exactly like the CLI
ENTRYPOINT ["doc_transformer"]
