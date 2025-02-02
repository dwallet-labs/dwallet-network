FROM rust:1.81-bullseye  AS chef
WORKDIR ika
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y cmake clang libpq5 ca-certificates libpq-dev postgresql

# Build application
FROM chef AS builder 

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY ika-execution ika-execution
COPY narwhal narwhal
COPY external-crates external-crates
RUN cargo build --release \
    --bin ika-oracle \
    --bin ikans-indexer
RUN mkdir /ika/bin/
RUN find /ika/target/release/ -maxdepth 1 -type f -executable -print 
RUN find /ika/target/release/ -maxdepth 1 -type f -executable -print | xargs cp -t /ika/bin/

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR ika
COPY --from=builder /ika/bin /tmp/ika-bin
RUN find /tmp/ika-bin -maxdepth 1 -type f -executable -print | xargs cp -t /usr/local/bin/
RUN rm -rf /tmp/ika-bin
RUN apt update && apt install -y libpq5 libpq-dev postgresql

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

