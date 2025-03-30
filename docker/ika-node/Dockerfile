# ==========================
# Build stage
# ==========================
FROM rust:1.85-bullseye AS builder

# Arguments
ARG PROFILE=release
ARG GIT_REVISION=unknown
ARG GITHUB_TOKEN
ARG WORKDIR=/opt

# Environment
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTFLAGS="-C target-cpu=native"
ENV GIT_REVISION=$GIT_REVISION

# Setup GitHub token for private repos
RUN git config --global url."https://${GITHUB_TOKEN}:x-oauth-basic@github.com/".insteadOf "https://github.com/"

# Install build dependencies
RUN apt-get update && \
    apt-get install -y cmake clang pkg-config libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR ${WORKDIR}/ika

# Copy Cargo manifests and crate folders for dependency resolution
COPY Cargo.toml Cargo.lock ./
COPY crates crates
COPY external-crates external-crates
COPY class-groups-keys-mock-files class-groups-keys-mock-files

# Build dependencies first for better layer caching
# --locked ensures you never accidentally change dependencies during a build.
RUN cargo fetch --locked
RUN cargo build --profile ${PROFILE} --locked --bin ika-node

# Final build
RUN cargo build --profile $PROFILE --bin ika-node

# ==========================
# Runtime stage
# ==========================
FROM debian:bullseye-slim AS runtime

ARG WORKDIR=/opt
ARG BUILD_DATE
ARG GIT_REVISION
ARG PROFILE=release

# Install runtime deps and jemalloc
RUN apt-get update && \
    apt-get install -y libjemalloc-dev ca-certificates curl jq && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Use jemalloc as memory allocator.
ENV LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so
ENV PATH="/opt/ika/bin:$PATH"

# Working directory
WORKDIR ${WORKDIR}/ika

# Create non-root user.
RUN useradd -m -u 1001 ika

# Copy built binary from builder stage
COPY --from=builder ${WORKDIR}/ika/target/${PROFILE}/ika-node /opt/ika/bin/ika-node
COPY --from=builder ${WORKDIR}/ika/target/${PROFILE}/ika-node /usr/local/bin/ika-node

# Metadata
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Switch to non-root user.
USER ika

# Default command.
ENTRYPOINT ["ika-node"]
