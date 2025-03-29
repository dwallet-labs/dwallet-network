#!/bin/sh
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: BSD-3-Clause-Clear

# fast fail.
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"
REPO_ROOT="$(git rev-parse --show-toplevel)"
DOCKERFILE="$DIR/Dockerfile"
GIT_REVISION="$(git describe --always --abbrev=12 --dirty --exclude '*')"
BUILD_DATE="$(date -u +'%Y-%m-%d')"

# Load environment variables from .env
if [ -f .env ]; then
  export "$(grep -v '^#' .env | xargs)"
else
  echo ".env file not found!"
  exit 1
fi

# Make sure GITHUB_TOKEN is set
if [ -z "$GITHUB_TOKEN" ]; then
  echo "GITHUB_TOKEN is not set. Check your .env file."
  exit 1
fi


# option to build using debug symbols
if [ "$1" = "--debug-symbols" ]; then
	PROFILE="bench"
	echo "Building with full debug info enabled ... WARNING: binary size might significantly increase"
	shift
else
	PROFILE="release"
fi

echo
echo "Building ika-node docker image"
echo "Dockerfile: \t$DOCKERFILE"
echo "docker context: $REPO_ROOT"
echo "build date: \t$BUILD_DATE"
echo "git revision: \t$GIT_REVISION"
echo

docker build -f "$DOCKERFILE" "$REPO_ROOT" \
	--build-arg GIT_REVISION="$GIT_REVISION" \
	--build-arg BUILD_DATE="$BUILD_DATE" \
	--build-arg PROFILE="$PROFILE" \
	--build-arg GITHUB_TOKEN="$GITHUB_TOKEN" \
	"$@"
