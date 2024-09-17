FROM rust:1.80.1-bullseye  AS chef
WORKDIR pera
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang libpq5 ca-certificates libpq-dev postgresql

# Build application
FROM chef AS builder 

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY pera-execution pera-execution
COPY narwhal narwhal
COPY external-crates external-crates
RUN cargo build --release \
    --bin pera-oracle \
    --bin perans-indexer
RUN mkdir /pera/bin/
RUN find /pera/target/release/ -maxdepth 1 -type f -executable -print 
RUN find /pera/target/release/ -maxdepth 1 -type f -executable -print | xargs cp -t /pera/bin/

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR pera
COPY --from=builder /pera/bin/* /usr/local/bin
RUN apt update && apt install -y libpq5 libpq-dev postgresql

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

