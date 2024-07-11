# We use the latest Rust stable release as base image
# Builder stage
FROM rust:1.79-slim AS builder

# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Copy all files from our working environment to our Docker image
COPY . .
# Let's build our binary!
# We'll use the release profile to make it faaaast
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime Stage
FROM rust:1.79-slim AS runtime

WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
# Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/restaurant restaurant
# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./restaurant"]
