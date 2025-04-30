#!/bin/bash

# Fast fail on errors or unset variables
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"
DOCKERFILE="$DIR/Dockerfile"
GIT_REVISION="$(git describe --always --abbrev=12 --dirty --exclude '*')"
BUILD_DATE="$(date -u +'%Y-%m-%d')"

# Handle optional debug profile
if [ "$1" = "--debug-symbols" ]; then
  PROFILE="bench"
  echo "Building with full debug info enabled ... WARNING: binary size might significantly increase"
  shift
else
  PROFILE="release"
fi

echo
echo "Building ika-node docker image"
echo "Dockerfile:      $DOCKERFILE"
echo "Docker context:  $REPO_ROOT"
echo "Build date:      $BUILD_DATE"
echo "Git revision:    $GIT_REVISION"
echo "Docker tag:      $DOCKER_TAG"
echo "Build profile:   $PROFILE"
echo

docker build . \
  --build-arg GIT_REVISION="$GIT_REVISION" \
  --build-arg BUILD_DATE="$BUILD_DATE" \
  --build-arg PROFILE="$PROFILE" \
  --build-arg GITHUB_TOKEN="$GITHUB_TOKEN" \
  --build-arg WITH_NETWORK_DKG="$WITH_NETWORK_DKG" \
  --tag "$DOCKER_TAG" \
  "$@"
