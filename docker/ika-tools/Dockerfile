# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.81-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/ika"
RUN apt-get update && apt-get install -y cmake clang libpq5 libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY ika-execution ika-execution
COPY narwhal narwhal
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin ika-node \
    --bin stress \
    --bin ika-bridge \
    --bin bridge-indexer \
    --bin deepbook-indexer \
    --bin ika-bridge-cli \
    --bin ika-analytics-indexer \
    --bin ika-proxy \
    --bin ika \
    --bin ika-faucet \
    --bin ika-cluster-test \
    --bin ika-tool

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/ika"

# ika-tool needs libpq at runtime
RUN apt-get update && apt-get install -y libpq5 libpq-dev ca-certificates

COPY --from=builder /ika/target/release/ika-node /usr/local/bin
COPY --from=builder /ika/target/release/stress /usr/local/bin
COPY --from=builder /ika/target/release/ika-bridge /usr/local/bin
COPY --from=builder /ika/target/release/bridge-indexer /usr/local/bin
COPY --from=builder /ika/target/release/deepbook-indexer /usr/local/bin
COPY --from=builder /ika/target/release/ika-bridge-cli /usr/local/bin
COPY --from=builder /ika/target/release/ika-analytics-indexer /usr/local/bin
COPY --from=builder /ika/target/release/ika-proxy /usr/local/bin
COPY --from=builder /ika/target/release/ika /usr/local/bin
COPY --from=builder /ika/target/release/ika-faucet /usr/local/bin
COPY --from=builder /ika/target/release/ika-cluster-test /usr/local/bin
COPY --from=builder /ika/target/release/ika-tool /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
