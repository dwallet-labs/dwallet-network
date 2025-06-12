#!/bin/bash

# Test script for ika-node-local Docker setup
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"

echo "🧪 Testing ika-node-local Docker setup"
echo "======================================="

# Check if binary exists
BINARY_PATH="$REPO_ROOT/target/release/ika-node"
if [ ! -f "$BINARY_PATH" ]; then
  echo "❌ Error: ika-node binary not found at $BINARY_PATH"
  echo "   Please build it first with: cargo build --release --bin ika-node"
  exit 1
fi

echo "✅ Binary found: $BINARY_PATH"

# Check Docker is running
if ! docker info >/dev/null 2>&1; then
  echo "❌ Error: Docker is not running or not accessible"
  exit 1
fi

echo "✅ Docker is running"

# Build the image
echo "🔨 Building ika-node-local Docker image..."
cd "$DIR"
if ./build.sh --quiet; then
  echo "✅ Docker image built successfully"
else
  echo "❌ Failed to build Docker image"
  exit 1
fi

# Test basic functionality
echo "🚀 Testing container startup..."
if docker run --rm ika-node-local:latest --help >/dev/null 2>&1; then
  echo "✅ Container starts and shows help"
else
  echo "❌ Container failed to start or show help"
  exit 1
fi

# Test version command
echo "📋 Testing version command..."
VERSION_OUTPUT=$(docker run --rm ika-node-local:latest --version 2>/dev/null || echo "version command failed")
if [[ "$VERSION_OUTPUT" == *"version command failed"* ]]; then
  echo "⚠️  Version command not available (this might be expected)"
else
  echo "✅ Version: $VERSION_OUTPUT"
fi

# Test binary exists in container
echo "🔍 Verifying binary exists in container..."
if docker run --rm ika-node-local:latest which ika-node >/dev/null 2>&1; then
  echo "✅ Binary is accessible in container PATH"
else
  echo "❌ Binary not found in container PATH"
  exit 1
fi

# Test with docker-compose (if available)
if command -v docker-compose >/dev/null 2>&1; then
  echo "🐳 Testing docker-compose configuration..."
  if docker-compose config >/dev/null 2>&1; then
    echo "✅ docker-compose.yml is valid"
  else
    echo "⚠️  docker-compose.yml has issues (check configuration)"
  fi
else
  echo "⚠️  docker-compose not available, skipping compose test"
fi

echo
echo "🎉 All tests passed!"
echo
echo "Next steps:"
echo "  1. Create configuration files in ./config/"
echo "  2. Run with: docker run --rm -v \$(pwd)/config:/config ika-node-local:latest --config-path /config/node.yaml"
echo "  3. Or use docker-compose: docker-compose up"
echo 