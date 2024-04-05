#! /bin/bash

source ./ci/env.sh

set -e
export CARGO_HOME='/usr/local/cargo'

RUSTUP_VERSION=1.26.0
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

  cargo install --locked --force cargo-deny
  cargo install --locked --force cargo-outdated
  cargo install --locked --force typos-cli --version "1.19.0"

  # Coverage tools
  rustup component add llvm-tools-preview
  cargo install --locked --force grcov
fi

if [ "$RUST_VERSION" = "nightly" ]; then
    # FIXME: Don't build cargo-c with --locked for now because otherwise a
    # version of ahash is used that doesn't build on nightly anymore
    cargo install cargo-c --version 0.9.22+cargo-0.72
else
    cargo install --locked cargo-c --version 0.9.22+cargo-0.72
fi

if [ "$RUST_VERSION" = "nightly" ]; then
  rustup component add rustfmt --toolchain nightly

  # Documentation tools
  cargo install --locked --force rustdoc-stripper
fi
