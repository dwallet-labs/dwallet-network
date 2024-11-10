# Build stage.
FROM rust:1.78-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR /lc

# Install dependencies for building
RUN apt-get update && apt-get install -y cmake clang ca-certificates

# Cache dependencies
# Copy only Cargo.toml and Cargo.lock for dependency installation and caching
COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY sui-execution sui-execution
COPY narwhal narwhal
COPY external-crates external-crates

# Final build of the application binaries
RUN cargo build --profile ${PROFILE} \
    --bin light-client-service \
    --bin light-client

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR /opt/lc

# Copy binaries to /opt/lc/bin and add them to PATH.
COPY --from=builder /lc/target/release/light-client-service /opt/lc/bin/light-client-service
COPY --from=builder /lc/target/release/light-client /opt/lc/bin/light-client

# Copy genesis files.
COPY --from=builder /lc/crates/sui-light-client/example_config/genesis_devnet.blob /opt/lc/config/genesis_devnet.blob
COPY --from=builder /lc/crates/sui-light-client/example_config/genesis_mainnet.blob /opt/lc/config/genesis_mainnet.blob
COPY --from=builder /lc/crates/sui-light-client/example_config/genesis_testnet.blob /opt/lc/config/genesis_testnet.blob

# Add binaries to system PATH for convenience.
RUN ln -s /opt/lc/bin/light-client-service /usr/local/bin/light-client-service \
    && ln -s /opt/lc/bin/light-client /usr/local/bin/light-client

# Set build metadata labels
ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
