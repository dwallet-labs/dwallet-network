# ==========================
# ARM64 runtime - uses pre-built ARM64 binary
# ==========================
FROM --platform=linux/arm64 debian:bullseye-slim

ARG WORKDIR=/opt
ARG BUILD_DATE
ARG GIT_REVISION

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates curl jq && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR ${WORKDIR}/ika

# Copy the pre-built Linux binary
COPY target/release/ika-node-linux /opt/ika/bin/ika-node
COPY target/release/ika-node-linux /usr/local/bin/ika-node

# Make executable
RUN chmod +x /opt/ika/bin/ika-node /usr/local/bin/ika-node

# Add build metadata
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION

# Default command
CMD ["ika-node"]
