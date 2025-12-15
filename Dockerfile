# Multi-stage Dockerfile for Axiom Assistant
# Stage 1: Build dependencies and compile Rust backend

FROM rust:1.91-slim as builder

# Install system dependencies required for building
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libglib2.0-dev \
    libsoup-3.0-dev \
    libjavascriptcoregtk-4.1-dev \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src
COPY ui ./ui

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime environment
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libgtk-3-0 \
    libwebkit2gtk-4.1-0 \
    libayatana-appindicator3-1 \
    libglib2.0-0 \
    libsoup-3.0-0 \
    libjavascriptcoregtk-4.1-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 axiom && \
    mkdir -p /app/models && \
    chown -R axiom:axiom /app

WORKDIR /app

# Copy built binary from builder
COPY --from=builder /app/target/release/axiom-assistant /app/axiom-assistant

# Copy models directory structure
COPY --chown=axiom:axiom models /app/models

# Set user
USER axiom

# Environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Expose port for potential web interface
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep -f axiom-assistant || exit 1

# Run the application
CMD ["/app/axiom-assistant"]
