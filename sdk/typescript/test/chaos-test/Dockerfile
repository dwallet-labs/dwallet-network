# ==========================
# Build stage
# ==========================
FROM ika-base:latest AS builder

# Arguments
ARG PROFILE=release
ARG GIT_REVISION=unknown
ARG GITHUB_TOKEN
ARG WORKDIR=/opt

# Environment
ENV CARGO_HOME=/usr/local/cargo
ENV GIT_REVISION=$GIT_REVISION

# Setup GitHub token for private repos
RUN git config --global url."https://${GITHUB_TOKEN}:x-oauth-basic@github.com/".insteadOf "https://github.com/"
# Set working directory
WORKDIR ${WORKDIR}/ika

RUN cargo chef prepare --recipe-path recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo build --profile ${PROFILE} --locked --bin ika-node

# ==========================
# Runtime stage
# ==========================
FROM debian:bullseye-slim AS runtime

ARG WORKDIR=/opt
ARG BUILD_DATE
ARG GIT_REVISION
ARG PROFILE=release
ARG TARGETARCH

# Install runtime dependencies and jemalloc.

# Use jemalloc as memory allocator.
ENV LD_PRELOAD=""
RUN if [ "$TARGETARCH" = "amd64" ]; then \
      LD_PRELOAD="/usr/lib/x86_64-linux-gnu/libjemalloc.so"; \
    fi

# Set working directory.
WORKDIR ${WORKDIR}/ika

# Copy the built binary from the builder stage.
COPY --from=builder /opt/ika/target/${PROFILE}/ika-node /opt/ika/bin/ika-node
COPY --from=builder /opt/ika/target/${PROFILE}/ika-node /usr/local/bin/ika-node

# Add build metadata.
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

