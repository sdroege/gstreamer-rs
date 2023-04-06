#! /bin/bash

set -ex

rustc --version
cargo --version

get_features() {
    module=${1%%/sys}
    case "$module" in
        gstreamer-validate)
            echo ""
            ;;
        *)
            echo "--features=v1_24"
            ;;
    esac
}

# First build and test all the crates with their relevant features
# Keep features in sync with below
for crate in gstreamer*/sys gstreamer-gl/*/sys; do
    if [ -e "$crate/Cargo.toml" ]; then
        echo "Building $crate with $(get_features "$crate")"
        cargo build --locked --color=always --manifest-path "$crate/Cargo.toml" $(get_features "$crate")
    fi
done

# Run tests for crates we can currently run.
# Other tests are broken currently.
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
    echo "Testing $crate with $(get_features $crate)"
    cargo test --locked --color=always --manifest-path $crate/Cargo.toml "$(get_features $crate)"
done
