# Production-ready Dockerfile for Axiom Assistant
# Multi-stage build for optimized image size

# Stage 1: Build environment
FROM rust:1.75-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy dependency manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src
COPY models ./models

# Build release binary with optimizations
RUN cargo build --release --bin axiom-assistant

# Stage 2: Runtime environment
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 axiom && \
    mkdir -p /app /app/models /app/logs && \
    chown -R axiom:axiom /app

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/axiom-assistant /app/

# Copy configuration files
COPY README.md ./

# Switch to non-root user
USER axiom

# Set environment variables
ENV RUST_LOG=info \
    AXIOM_MAX_TOKENS=2048 \
    AXIOM_TEMPERATURE=0.7 \
    AXIOM_MAX_QUERY_LENGTH=10000

# Expose port for future web interface (currently CLI only)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep axiom-assistant || exit 1

# Run the application
ENTRYPOINT ["/app/axiom-assistant"]
