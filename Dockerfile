FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies (optional for caching)
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# Copy the rest of the project
COPY . .

# Build the Rust project
# RUN cargo build --release --target x86_64-unknown-linux-gnu
RUN cargo build --release

# Use a minimal image for the final binary
FROM ubuntu:24.04

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libpq-dev ca-certificates curl && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/local/bin

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/skumb .

# Expose the port for the webserver and websocket
EXPOSE 9123

# Excluded for now - ECS has it's own and doesn't use this
# HEALTHCHECK CMD curl -f http://localhost:9123/health || exit 1

# Run the binary
CMD ["./skumb"]

