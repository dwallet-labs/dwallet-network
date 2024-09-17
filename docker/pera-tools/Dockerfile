# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.80.1-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/pera"
RUN apt-get update && apt-get install -y cmake clang libpq5 libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY pera-execution pera-execution
COPY narwhal narwhal
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin pera-node \
    --bin stress \
    --bin pera-bridge \
    --bin bridge-indexer \
    --bin pera-bridge-cli \
    --bin pera-analytics-indexer \
    --bin pera-proxy \
    --bin pera \
    --bin pera-faucet \
    --bin pera-cluster-test \
    --bin pera-tool

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/pera"

# pera-tool needs libpq at runtime
RUN apt-get update && apt-get install -y libpq5 libpq-dev ca-certificates

COPY --from=builder /pera/target/release/pera-node /usr/local/bin
COPY --from=builder /pera/target/release/stress /usr/local/bin
COPY --from=builder /pera/target/release/pera-bridge /usr/local/bin
COPY --from=builder /pera/target/release/bridge-indexer /usr/local/bin
COPY --from=builder /pera/target/release/pera-bridge-cli /usr/local/bin
COPY --from=builder /pera/target/release/pera-analytics-indexer /usr/local/bin
COPY --from=builder /pera/target/release/pera-proxy /usr/local/bin
COPY --from=builder /pera/target/release/pera /usr/local/bin
COPY --from=builder /pera/target/release/pera-faucet /usr/local/bin
COPY --from=builder /pera/target/release/pera-cluster-test /usr/local/bin
COPY --from=builder /pera/target/release/pera-tool /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
