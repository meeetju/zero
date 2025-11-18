# Builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1.91.1 AS chef

WORKDIR /app

RUN apt update && apt install lld clang -y

FROM chef AS planner
# Copy all files from our working environment to our Docker image
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Copy only the recipe file from the planner stage
COPY --from=planner /app/recipe.json recipe.json
# Build our project and dependencies, reusing cached dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero --bin migrate

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binaries from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero zero
COPY --from=builder /app/target/release/migrate migrate
# We need the configuration file and migrations at runtime
COPY configuration configuration
COPY --from=builder /app/migrations migrations

ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./zero"]