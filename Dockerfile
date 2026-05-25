# ─────────────────────────────────────────────────────────────────────────────
# RustForge Trader — Production Dockerfile
#
# Multi-stage build:
#   Stage 1 (builder): Full Rust toolchain — compiles the binary.
#   Stage 2 (runtime): Minimal Debian image — runs the binary.
#
# Why multi-stage?
#   The Rust compiler + all crate sources are ~2 GB.
#   The compiled binary is ~15 MB.
#   Shipping the compiler to production is wasteful and a security risk
#   (an attacker who breaks in can compile and run arbitrary code).
#   The final image contains ONLY the binary and its runtime dependencies.
# ─────────────────────────────────────────────────────────────────────────────

# ── Stage 1: Builder ──────────────────────────────────────────────────────────
# Use the official Rust image pinned to a specific version for reproducibility.
FROM rust:1.82-bookworm AS builder

WORKDIR /app

# ── Dependency caching trick ───────────────────────────────────────────────────
# Docker layer cache works line-by-line. If we copy ALL source files and then
# run `cargo build`, ANY change to any .rs file invalidates the cache and
# rebuilds ALL dependencies from scratch (~5 min).
#
# The trick: copy ONLY Cargo.toml and Cargo.lock first, create a dummy main.rs,
# build dependencies, THEN copy the real source. Now dependency compilation is
# cached as long as Cargo.toml doesn't change.
COPY Cargo.toml Cargo.lock ./

# Create a stub lib and main so cargo can resolve and compile dependencies.
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "// stub" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the real source code.
COPY src ./src
COPY migrations ./migrations

# Touch main.rs to ensure it's recompiled (Cargo tracks mtimes).
RUN touch src/main.rs src/lib.rs

# Build the actual binary with full optimizations.
# `--locked` ensures Cargo.lock is respected exactly (no silent upgrades).
RUN cargo build --release --locked

# ── Stage 2: Runtime ──────────────────────────────────────────────────────────
# `debian:bookworm-slim` is ~80 MB vs ~1.2 GB for the Rust builder.
# We need `libssl` because reqwest uses native-tls.
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies.
# `--no-install-recommends` keeps the layer lean.
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application.
# Running as root in a container is a security antipattern.
RUN useradd --uid 1000 --no-create-home --shell /bin/false rustforge
USER rustforge

WORKDIR /app

# Copy only the binary from the builder stage.
COPY --from=builder /app/target/release/rustforge-trader ./rustforge-trader

# Copy migrations (SQLx runs these at startup).
COPY --from=builder /app/migrations ./migrations

# Expose the HTTP port (the actual port is set via SERVER__PORT env var).
EXPOSE 8080

# Liveness probe endpoint for container orchestrators.
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# The binary reads all config from environment variables.
ENTRYPOINT ["./rustforge-trader"]