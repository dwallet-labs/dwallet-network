# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.78-bullseye AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/lc"
RUN apt-get update && apt-get install -y cmake clang

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY sui-execution sui-execution
COPY narwhal narwhal
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin light-client-service \
    --bin light-client

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/lc"

COPY --from=builder /lc/target/release/light-client-service /opt/lc/bin/light-client-service
COPY --from=builder /lc/target/release/light-client-service /usr/local/bin

COPY --from=builder /lc/target/release/light-client /opt/lc/bin/light-client
COPY --from=builder /lc/target/release/light-client /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
