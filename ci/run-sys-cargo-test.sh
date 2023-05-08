#! /bin/bash

set -ex

rustc --version
cargo --version

for crate in gstreamer*/sys gstreamer-gl/*/sys; do
    if [ -e "$crate/Cargo.toml" ]; then
        echo "Building $crate with --all-features"
        cargo build --locked --color=always --manifest-path "$crate/Cargo.toml" --all-features
    fi
done

for crate in gstreamer/sys \
             gstreamer-app/sys \
             gstreamer-audio/sys \
             gstreamer-base/sys \
             gstreamer-check/sys \
             gstreamer-controller/sys \
             gstreamer-gl/sys \
             gstreamer-gl/egl/sys \
             gstreamer-gl/wayland/sys \
             gstreamer-gl/x11/sys \
             gstreamer-mpegts/sys \
             gstreamer-net/sys \
             gstreamer-pbutils/sys \
             gstreamer-player/sys \
             gstreamer-rtsp-server/sys \
             gstreamer-rtsp/sys \
             gstreamer-sdp/sys \
             gstreamer-tag/sys \
             gstreamer-video/sys \
             gstreamer-webrtc/sys; do
    echo "Testing $crate with --all-features)"
    cargo test --locked --color=always --manifest-path $crate/Cargo.toml --all-features
done
