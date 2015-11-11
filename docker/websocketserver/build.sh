#! /usr/bin/env bash
echo ">> Building image"
docker build --tag websocket-builder --file Dockerfile.build .

echo ">> Creating build container"
BACK_ONE=${PWD%/*}
ROOT_DIR=${BACK_ONE%/*}
CONTAINER_ID=$(docker create --volume $ROOT_DIR:/code websocket-builder cargo build --release)

echo ">> Extracting out binary"
docker cp $CONTAINER_ID:/code/target/releases/simple_server .


