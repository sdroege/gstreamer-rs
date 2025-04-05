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
    if [ "$RUST_VERSION" = "1.71.1" ]; then
        cargo install --locked cargo-outdated
    else
        # Don't use --locked because time-0.3.30 does not build with 1.80 or newer
        cargo install cargo-outdated
    fi
  cargo install --locked typos-cli --version "1.19.0"

  # Coverage tools
  rustup component add llvm-tools-preview
  if [ "$RUST_VERSION" = "1.71.1" ]; then
      cargo install --locked grcov
  else
      # Don't use --locked because time-0.3.30 does not build with 1.80 or newer
      cargo install grcov
  fi
fi

# Multiple dependencies of cargo-nextest require 1.74/1.75 nowadays
if [ "$RUST_VERSION" = "1.71.1" ]; then
  cargo install --locked cargo-nextest@0.9.67
else
  cargo install --locked cargo-nextest
fi

if [ "$RUST_VERSION" = "1.71.1" ]; then
    cargo install --locked cargo-c --version 0.9.26+cargo-0.74
else
    cargo install --locked cargo-c --version 0.10.12+cargo-0.87
fi

if [ "$RUST_VERSION" = "nightly" ]; then
  rustup component add rustfmt --toolchain nightly

  # Documentation tools
  cargo install --locked rustdoc-stripper
fi
