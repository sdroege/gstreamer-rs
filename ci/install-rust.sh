source ./ci/env.sh

set -e
export CARGO_HOME='/usr/local/cargo'

RUSTUP_VERSION=1.21.1
RUST_VERSION=$1
RUST_ARCH="x86_64-unknown-linux-gnu"

if [ "$RUST_VERSION" = "stable" ]; then
    RUST_VERSION="1.44.1"
fi

RUSTUP_URL=https://static.rust-lang.org/rustup/archive/$RUSTUP_VERSION/$RUST_ARCH/rustup-init
wget $RUSTUP_URL

chmod +x rustup-init;
./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION;
rm rustup-init;
chmod -R a+w $RUSTUP_HOME $CARGO_HOME

rustup --version
cargo --version
rustc --version

if [ "$RUST_VERSION" = "1.44.1" ]; then
  rustup component add clippy-preview --toolchain $RUST_VERSION
  rustup component add rustfmt --toolchain $RUST_VERSION
  cargo install --force cargo-deny
  cargo install --force cargo-outdated
fi
