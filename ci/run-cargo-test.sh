#! /bin/bash

set -ex

rustc --version
cargo --version

cpus=$(nproc || sysctl -n hw.ncpu)
CARGO_FLAGS="--color=always -j${FDO_CI_CONCURRENT:-$cpus}"

if [ "$RUST_VERSION" = "1.71.1" ]; then
    CARGO_NEXTEST_FLAGS="--profile=ci"
else
    CARGO_NEXTEST_FLAGS="--profile=ci --no-tests=pass"
fi

parent="${CI_PROJECT_DIR:-$(pwd)}"

for crate in gstreamer* gstreamer-gl/{egl,wayland,x11}; do
    if [ -e "$crate/Cargo.toml" ]; then
        if [ -n "$ALL_FEATURES" ]; then
            FEATURES="--all-features"
        else
            FEATURES=""
        fi

        echo "Building and testing $crate with $FEATURES"

        cargo build $CARGO_FLAGS --locked --manifest-path "$crate/Cargo.toml" $FEATURES
        RUST_BACKTRACE=1 G_DEBUG=fatal_warnings cargo nextest run $CARGO_NEXTEST_FLAGS $CARGO_FLAGS --locked --manifest-path "$crate/Cargo.toml" $FEATURES

        new_report_dir="$parent/junit_reports/$crate"
        mkdir -p "$new_report_dir"
        mv "$parent/target/nextest/ci/junit.xml" "$new_report_dir/junit.xml"
    fi
done

if [ -n "$EXAMPLES_TUTORIALS" ]; then
    # Keep in sync with examples/Cargo.toml
    # List all features except windows/win32
    EXAMPLES_FEATURES="--features=rtsp-server,rtsp-server-record,pango-cairo,overlay-composition,gl,gst-gl-x11,gst-gl-egl,allocators,gst-play,gst-player,ges,image,cairo-rs,gst-video/v1_18"

    cargo build $CARGO_FLAGS --locked --manifest-path examples/Cargo.toml --bins --examples "$EXAMPLES_FEATURES"
    cargo build $CARGO_FLAGS --locked --manifest-path tutorials/Cargo.toml --bins --examples --all-features
fi
