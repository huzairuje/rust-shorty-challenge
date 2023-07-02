####################################
# STEP 1 build executable binary
####################################
FROM rust:1.69 as builder

# Install required tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/rust-shorty-challenge/

# Copy the source code to the container
COPY . .

# Build the binary inside the container
RUN cargo rustc --release -- -C target-cpu=native

RUN chmod +x /usr/src/rust-shorty-challenge/target/release/rust-shorty-challenge

#########################################################
# STEP 2 build a small image using debian:buster-slim
#########################################################
FROM debian:buster-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the .env file from .env.example to use as env conf
COPY --from=builder /usr/src/rust-shorty-challenge/.env /usr/local/bin/

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rust-shorty-challenge/target/release/rust-shorty-challenge /usr/local/bin/

# Set the working directory
WORKDIR /usr/src/rust-shorty-challenge/

# Run the entrypoint
CMD ["rust-shorty-challenge"]