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
RUN apt-get update && apt-get install -y libjemalloc-dev ca-certificates curl jq

# Use jemalloc as memory allocator.
ENV LD_PRELOAD=""
RUN if [ "$TARGETARCH" = "amd64" ]; then \
      LD_PRELOAD="/usr/lib/x86_64-linux-gnu/libjemalloc.so"; \
    fi

# Set working directory.
WORKDIR ${WORKDIR}/ika

# Copy the built binary from the builder stage.
COPY ./ika-node /opt/ika/bin/ika-node
COPY ./ika-node /usr/local/bin/ika-node

# Add build metadata.
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

