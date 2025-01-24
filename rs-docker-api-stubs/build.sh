#!/bin/bash

set -ex

DOCKER_SWAGGER_URL="https://docs.docker.com/engine/api"
DOCKER_API_VERSION="v1.43"
DOCKER_SPEC_FILE="${DOCKER_API_VERSION}.yaml"
DOCKER_FULL_URL="${DOCKER_SWAGGER_URL}/${DOCKER_SPEC_FILE}"
RUSTGEN="https://github.com/vv9k/swagger-rustgen.git"
BUILD_DIR=build
BASE_DIR=$PWD

mkdir $BUILD_DIR || true

cd $BUILD_DIR
echo $PWD

curl -LO $DOCKER_FULL_URL

git clone $RUSTGEN || true
cd swagger-rustgen
cargo build --release
cd $BASE_DIR

cat base/models.rs > lib/src/models.rs

$BUILD_DIR/swagger-rustgen/target/release/swagger-gen generate models $BUILD_DIR/$DOCKER_SPEC_FILE >> lib/src/models.rs

cd lib

cargo fmt

# Fix for https://github.com/vv9k/docker-api-rs/pull/29
sed -r -i 's/(PortMap = HashMap<String, )(Vec<PortBinding>)/\1Option<\2>/g' src/models.rs

cargo fmt
