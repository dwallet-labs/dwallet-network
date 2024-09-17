FROM rust:1.80.1-bullseye  AS chef
WORKDIR pera
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang

FROM chef AS builder 

# Build application
COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY pera-execution pera-execution
COPY narwhal narwhal
COPY external-crates external-crates
RUN cargo build --release \
    --bin pera-source-validation-service

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR pera
RUN apt update && apt install -y git
COPY --from=builder /pera/target/release/pera-source-validation-service /usr/local/bin
COPY crates/pera-source-validation-service/config.toml /var/pera/

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
