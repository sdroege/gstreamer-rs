#!/bin/bash

source ./ci/env.sh

set -e

RICE_PROTO_VERSION=v0.3.0

git clone --depth=1 https://github.com/ystreet/librice -b $RICE_PROTO_VERSION
cd librice
cargo cinstall -p rice-proto --prefix /usr/local --libdir lib/x86_64-linux-gnu
cd ..
rm -rf librice
