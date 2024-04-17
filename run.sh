#!/bin/bash

set -ex

wasmtime -S http ./composed.wasm
