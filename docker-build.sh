#!/bin/bash
set -euo pipefail

cargo build --release --all-features --target x86_64-unknown-linux-musl

rm -rf .tmp
mkdir -p .tmp
cp target/x86_64-unknown-linux-musl/release/guntamatic .tmp
cp Dockerfile .tmp
pushd .tmp
docker build -t swimmes/guntamatic -f Dockerfile .
popd
rm -rf .tmp