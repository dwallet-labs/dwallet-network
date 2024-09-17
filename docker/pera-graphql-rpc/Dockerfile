# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.80.1-bullseye AS builder
ARG PROFILE=release
ENV PROFILE=$PROFILE
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/pera"
RUN apt-get update && apt-get install -y cmake clang libpq-dev

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY pera-execution pera-execution
COPY narwhal narwhal
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} --bin pera-graphql-rpc

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/pera"
# Both bench and release profiles copy from release dir
RUN mkdir -p /opt/pera/bin
COPY --from=builder /pera/target/release/pera-graphql-rpc /opt/pera/bin
RUN apt-get update && apt-get install -y libpq5 ca-certificates libpq-dev postgresql

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
