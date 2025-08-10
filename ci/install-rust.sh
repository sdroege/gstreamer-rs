#! /bin/bash

source ./ci/env.sh

set -e
export CARGO_HOME='/usr/local/cargo'

RUSTUP_VERSION=1.28.1
RUST_VERSION=$1
RUST_IMAGE_FULL=$2
RUST_ARCH="x86_64-unknown-linux-gnu"

RUSTUP_URL=https://static.rust-lang.org/rustup/archive/$RUSTUP_VERSION/$RUST_ARCH/rustup-init
wget $RUSTUP_URL

chmod +x rustup-init;
./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION;
rm rustup-init;
chmod -R a+w $RUSTUP_HOME $CARGO_HOME

rustup --version
cargo --version
rustc --version

if [ "$RUST_IMAGE_FULL" = "1" ]; then
  rustup component add clippy-preview
  rustup component add rustfmt

  cargo install --locked cargo-deny
  cargo install --locked cargo-outdated

  # Coverage tools
  rustup component add llvm-tools-preview
  cargo install --locked grcov
fi

if [ "$RUST_VERSION" = "1.83.0" ]; then
    cargo install --locked cargo-nextest@0.9.94
else
    cargo install --locked cargo-nextest
fi

if [ "$RUST_VERSION" = "1.83.0" ]; then
    cargo install --locked cargo-c --version 0.10.11+cargo-0.86.0
else
    cargo install --locked cargo-c --version 0.10.15+cargo-0.90
fi

if [ "$RUST_VERSION" = "nightly" ]; then
  rustup component add rustfmt --toolchain nightly

  # Documentation tools
  cargo install --locked rustdoc-stripper
fi

cargo install --locked bindgen-cli
bindgen --version
