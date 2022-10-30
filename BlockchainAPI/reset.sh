#!/bin/bash

set -e

cd neardev
rm dev-account dev-account.env
cd ..

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p ./out
cp target/wasm32-unknown-unknown/release/*.wasm ./out/main.wasm
near dev-deploy 