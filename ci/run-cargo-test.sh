#! /bin/bash

set -ex

rustc --version
cargo --version

for crate in gstreamer* gstreamer-gl/{egl,wayland,x11}; do
    if [ -e "$crate/Cargo.toml" ]; then
        if [ -n "$ALL_FEATURES" ]; then
            FEATURES="--all-features"
        else
            FEATURES=""
        fi

        echo "Building and testing $crate with $FEATURES"

        cargo build --locked --color=always --manifest-path "$crate/Cargo.toml" $FEATURES
        G_DEBUG=fatal_warnings cargo test --color=always --manifest-path "$crate/Cargo.toml" $FEATURES
    fi
done

if [ -n "$EXAMPLES_TUTORIALS" ]; then
    # Keep in sync with examples/Cargo.toml
    # List all features except windows/win32
    EXAMPLES_FEATURES="--features=rtsp-server,rtsp-server-record,pango-cairo,overlay-composition,gl,gst-gl-x11,gst-gl-wayland,gst-gl-egl,allocators,gst-play,gst-player,ges,image,cairo-rs,gst-video/v1_18"

    cargo build --locked --color=always --manifest-path examples/Cargo.toml --bins --examples "$EXAMPLES_FEATURES"
    cargo build --locked --color=always --manifest-path tutorials/Cargo.toml --bins --examples --all-features
fi
