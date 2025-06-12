#!/bin/bash

# Build Linux binary using Docker, then create local container
set -e

echo "🔧 Building Linux binary using Docker..."

# Use the existing ika-node Dockerfile to build just the binary
docker build -f ../ika-node/Dockerfile ../.. \
  --target builder \
  --build-arg GITHUB_TOKEN="${GITHUB_TOKEN}" \
  --tag ika-node-builder:temp

# Extract the Linux binary from the builder image
echo "📦 Extracting Linux binary..."
docker create --name temp-container ika-node-builder:temp
docker cp temp-container:/opt/ika/target/release/ika-node ../../../target/release/ika-node-linux
docker rm temp-container
docker rmi ika-node-builder:temp

echo "✅ Linux binary created: target/release/ika-node-linux"
file ../../../target/release/ika-node-linux

echo ""
echo "🐳 Now building the local Docker image..."
DOCKER_TAG="${DOCKER_TAG:-ika-node-local:latest}" ./build.sh

echo ""
echo "🎉 Done! You can now run:"
echo "  docker run --rm ika-node-local:latest ika-node --help" 