# Dockerfile for the hello_world project
#
# Build with
#     sudo docker build -t rust_hello_world .
#
# Debug build instructions with one of
# [1] `RUN sleep infinity` and `sudo nsenter -a -t $PID_OF_SLEEP sh`
#     https://github.com/moby/buildkit/issues/1053#issuecomment-592050352
# [2] sudo DOCKER_BUILDKIT=0 docker build -t rust_hello_world .
#
# Run with
#     sudo docker run -p 3000:3000 rust_hello_world

# Use a Rust base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
# RUN mkdir src && \
#     touch src/lib.rs && \
#     echo "fn main() {}" > src/main.rs && \
#     cargo build --release && \
#     rm -f target/release/*hello_world*
RUN cargo build --release || true

# Copy the source code into the container
COPY LICENSE .
COPY assets/ assets/
COPY doc/ doc/
COPY README.org .
COPY templates/ templates/
COPY src/ src/
COPY tests/ tests/

# Build the application
RUN cargo build --release

# Start a new stage and use a smaller base image
FROM debian:bookworm-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/hello_world /app/
COPY --from=builder /app/target/release/libhello_world_lib.rlib /app/

# Copy the static assets
COPY assets/ /app/assets/
COPY templates/ /app/templates/

# Expose the port your application runs on
EXPOSE 3000

# Command to run the application
CMD ["./hello_world"]
