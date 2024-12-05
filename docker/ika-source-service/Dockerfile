FROM rust:1.81-bullseye  AS chef
WORKDIR ika
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang

FROM chef AS builder 

# Build application
COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY ika-execution ika-execution
COPY narwhal narwhal
COPY external-crates external-crates
RUN cargo build --release \
    --bin ika-source-validation-service

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR ika
RUN apt update && apt install -y git
COPY --from=builder /ika/target/release/ika-source-validation-service /usr/local/bin
COPY crates/ika-source-validation-service/config.toml /var/ika/

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
