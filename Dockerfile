####################################
# STEP 1 build executable binary
####################################
FROM rust:slim as builder

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

##############################################
# STEP 2 build a small image using alpine:3.14
##############################################
FROM alpine:3.14

# Install runtime dependencies
RUN apk --no-cache add ca-certificates

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rust-shorty-challenge/target/release/rust-shorty-challenge /usr/local/bin/rust-shorty-challenge

COPY --from=builder /usr/src/rust-shorty-challenge/.env.example /usr/local/bin/.env

# Run the entrypoint
CMD ["rust-shorty-challenge"]