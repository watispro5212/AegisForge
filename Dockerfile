# Build Stage
FROM rust:1.77-slim AS builder

WORKDIR /usr/src/aegisforge

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy src/main.rs to build dependencies and cache them
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove the dummy source and copy the real code
RUN rm -rf src
COPY src ./src

# Touch main.rs to force recompilation of the actual project
RUN touch src/main.rs
RUN cargo build --release

# Runtime Stage (Minimal Debian)
FROM debian:bookworm-slim

# Install CA certificates (required for Rustls to verify Discord/Neon DB certs)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/aegisforge/target/release/aegisforge /app/aegisforge

# Make the binary executable
RUN chmod +x /app/aegisforge

# Run the bot
CMD ["./aegisforge"]
