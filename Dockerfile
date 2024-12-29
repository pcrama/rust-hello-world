# Start with a base image that has Rust installed
FROM rust:1.82 as builder

# Set the working directory for the build
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal Docker image for the final artifact
FROM debian:bookworm-slim

# Set the working directory for the final container
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/hello_world .

# Expose the port on which your app runs
EXPOSE 8080

# Tell server which port to bind to (cf `EXPOSE`)
ENV RUST_HELLO_WORLD_BIND_TO=0.0.0.0:8080

# Run the application binary
CMD ["./hello_world"]
