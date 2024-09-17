FROM ubuntu:latest AS chef
WORKDIR pera
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config curl cmake clang ca-certificates
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Build application
FROM chef AS builder

RUN curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s

COPY Cargo.toml Cargo.lock ./
COPY consensus consensus
COPY crates crates
COPY pera-execution pera-execution
COPY narwhal narwhal
COPY external-crates external-crates
RUN cargo build --release --bin pera --bin pera-rosetta

# Production Image
FROM ubuntu:latest AS runtime
WORKDIR pera
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /pera/target/release/pera /usr/local/bin
COPY --from=builder /pera/target/release/pera-rosetta /usr/local/bin
COPY --from=builder /pera/bin/rosetta-cli /usr/local/bin
COPY --from=builder /pera/crates/pera-config/data/fullnode-template.yaml /pera/devnet/fullnode.yaml
RUN /usr/local/bin/pera genesis

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
