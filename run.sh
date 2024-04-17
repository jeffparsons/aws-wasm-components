#!/bin/bash

set -ex

wasmtime \
    -S http \
    --env AWS_REGION \
    --env AWS_ACCESS_KEY_ID \
    --env AWS_SECRET_ACCESS_KEY \
    --env AWS_SESSION_TOKEN \
    --env AWS_SESSION_EXPIRATION \
    ./composed.wasm
