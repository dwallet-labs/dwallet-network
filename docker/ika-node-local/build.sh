#!/bin/bash -x

# Fast fail on errors or unset variables
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"
DOCKERFILE="$DIR/Dockerfile"
GIT_REVISION="$(git describe --always --abbrev=12 --dirty --exclude '*')"
BUILD_DATE="$(date -u +'%Y-%m-%d')"

# Default Docker tag if not provided
DEFAULT_DOCKER_TAG="ika-node-local:latest"

# Load environment variables from .env if not already set
if [ -f .env ]; then
  echo "Loading variables from .env"
  while IFS='=' read -r key value; do
    # Skip comments and empty lines
    if [ -z "$key" ] || echo "$key" | grep -q '^#'; then
      continue
    fi

    # Only export if not already set in environment
    if [ -z "${!key}" ]; then
      export "$key=$value"
    fi
  done < .env
else
  echo "Warning: .env file not found, using defaults"
fi

# Use provided DOCKER_TAG or default
DOCKER_TAG="${DOCKER_TAG:-$DEFAULT_DOCKER_TAG}"

# Check if Linux binary exists
BINARY_PATH="$REPO_ROOT/target/release/ika-node-linux"
if [ ! -f "$BINARY_PATH" ]; then
  echo "❌ Error: Linux ika-node binary not found at $BINARY_PATH"
  echo "   Please run: ./build-linux-binary.sh"
  exit 1
fi

echo "✅ Found Linux binary: $BINARY_PATH"

echo
echo "Building ika-node-local docker image (with pre-built Linux binary)"
echo "Dockerfile:      $DOCKERFILE"
echo "Docker context:  $REPO_ROOT"
echo "Build date:      $BUILD_DATE"
echo "Git revision:    $GIT_REVISION"
echo "Docker tag:      $DOCKER_TAG"
echo

# Build the Docker image
# Temporarily use our custom .dockerignore
ORIGINAL_DOCKERIGNORE="$REPO_ROOT/.dockerignore"
BACKUP_DOCKERIGNORE="$REPO_ROOT/.dockerignore.backup"

# Backup original .dockerignore if it exists
if [ -f "$ORIGINAL_DOCKERIGNORE" ]; then
  cp "$ORIGINAL_DOCKERIGNORE" "$BACKUP_DOCKERIGNORE"
fi

# Use our custom .dockerignore
cp "$DIR/.dockerignore" "$ORIGINAL_DOCKERIGNORE"

# Build the image
docker build -f "$DOCKERFILE" "$REPO_ROOT" \
  --build-arg GIT_REVISION="$GIT_REVISION" \
  --build-arg BUILD_DATE="$BUILD_DATE" \
  --tag "$DOCKER_TAG" \
  "$@"

# Restore original .dockerignore
if [ -f "$BACKUP_DOCKERIGNORE" ]; then
  mv "$BACKUP_DOCKERIGNORE" "$ORIGINAL_DOCKERIGNORE"
else
  rm -f "$ORIGINAL_DOCKERIGNORE"
fi

echo
echo "✅ Successfully built ika-node-local Docker image: $DOCKER_TAG"
echo
echo "To run the container:"
echo "  docker run --rm $DOCKER_TAG --help"
echo
echo "To run with custom configuration:"
echo "  docker run --rm -v /path/to/config:/config $DOCKER_TAG --config-path /config/node.yaml"
