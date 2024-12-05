FROM ubuntu:latest AS chef
WORKDIR ika
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
COPY ika-execution ika-execution
COPY narwhal narwhal
COPY external-crates external-crates
RUN cargo build --release --bin ika --bin ika-rosetta

# Production Image
FROM ubuntu:latest AS runtime
WORKDIR ika
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /ika/target/release/ika /usr/local/bin
COPY --from=builder /ika/target/release/ika-rosetta /usr/local/bin
COPY --from=builder /ika/bin/rosetta-cli /usr/local/bin
COPY --from=builder /ika/crates/ika-config/data/fullnode-template.yaml /ika/devnet/fullnode.yaml
RUN /usr/local/bin/ika genesis

ARG BUILD_DATE
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
