# Use the official Rust image as a base
FROM rust:1.82 as builder

# Create a new empty shell project
RUN mkdir /rust-app
WORKDIR /rust-app

# Copy our manifest
ADD . ./

ENV SQLX_OFFLINE=true
# Build for Release
RUN cargo clean && \
    cargo build -vv --release

# Stage 2: Create a lightweight image for running the application
FROM debian:bookworm-slim

# Install OpenSSL for our application to work
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/* &&\
    apt-get install libc6

# Copy the build artifact from the builder stage
COPY --from=builder /rust-app/target/release/inventory-service /usr/local/bin/inventory-service

# Set the startup command to run your binary
CMD ["inventory-service"]

# Running application within docker on specific port
EXPOSE 3000