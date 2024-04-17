#!/bin/bash

set -ex

pushd ec2
cargo component build
popd

pushd example-cli
cargo component build
popd

wasm-tools compose \
    example-cli/target/wasm32-wasi/debug/example-cli.wasm \
    -d ec2/target/wasm32-wasi/debug/ec2.wasm \
    > composed.wasm
