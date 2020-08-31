#!/bin/sh

PORT=3000
if [ ! -z $1 ]; then
	PORT=$1
fi
IMAGE=secondstate/ssvm-nodejs-starter:v1
if [ ! -z $2 ]; then
	IMAGE=$2
fi

docker run --rm -it \
	-p ${PORT}:3000 \
	-v $(pwd)/.cargo/registry:/usr/local/cargo/registry \
	-v $(pwd)/.target:/root/target \
	-v $(pwd):/app \
	-w /app \
	-e RUST_LOG=debug \
	-e CARGO_TARGET_DIR=/root/target \
	-e NODE_DEBUG=app \
	$IMAGE
