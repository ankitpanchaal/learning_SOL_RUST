# Use the official Rust image as a base
FROM rust:1.82.0 AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release
RUN rm src/main.rs

# Copy the actual source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:bookworm-slim

# Install SSL certificates and other runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1001 appuser

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/rust /app/rust

# Change ownership to the app user
RUN chown -R appuser:appuser /app

# Switch to the app user
USER appuser

# Expose the port
EXPOSE 3000

# Set environment variables
ENV PORT=3000
ENV RCP_URL=https://api.devnet.solana.com

# Run the application
CMD ["./rust"]