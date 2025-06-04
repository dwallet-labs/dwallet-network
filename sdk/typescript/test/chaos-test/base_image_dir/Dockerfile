# ==========================
# Build stage
# ==========================
FROM rust:1.85-bullseye AS builder

# Arguments
ARG PROFILE=release
ARG GIT_REVISION=unknown
ARG WORKDIR=/opt

# Environment
ENV CARGO_HOME=/usr/local/cargo
ENV GIT_REVISION=$GIT_REVISION

# Install build dependencies
RUN apt-get update && apt-get install -y cmake clang pkg-config libssl-dev
RUN cargo install cargo-chef
