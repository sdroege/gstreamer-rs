#! /bin/bash

set -ex

rustc --version
cargo --version
cargo clippy --version

cpus=$(nproc || sysctl -n hw.ncpu)
CARGO_FLAGS="--color=always -j${FDO_CI_CONCURRENT:-$cpus}"

# Keep features in sync with run-cargo-test.sh
get_features() {
    crate=$1
    case "$crate" in
        gstreamer-audio|gstreamer-editing-services|gstreamer-gl|gstreamer-pbutils|gstreamer-rtp|gstreamer-rtsp|gstreamer-video|gstreamer)
            echo "--features=serde,v1_26"
            ;;
        gstreamer-mse)
            echo ""
            ;;
        *)
            echo "--features=v1_26"
            ;;
    esac
}

for crate in gstreamer* gstreamer-gl/{egl,wayland,x11}; do
    if [ -e "$crate/Cargo.toml" ]; then
        FEATURES=$(get_features "$crate")

        echo "Running clippy on $crate with $FEATURES"

        cargo clippy $CARGO_FLAGS --locked --manifest-path "$crate/Cargo.toml" $FEATURES --all-targets -- $CLIPPY_LINTS
    fi
done

# Keep in sync with examples/Cargo.toml
# List all features except windows/win32
EXAMPLES_FEATURES="--features=rtsp-server,rtsp-server-record,pango-cairo,overlay-composition,gl,gst-gl-x11,gst-gl-egl,allocators,gst-play,gst-player,ges,image,cairo-rs,gst-video/v1_18"

# And also run over all the examples/tutorials
cargo clippy $CARGO_FLAGS --locked --manifest-path examples/Cargo.toml --all-targets "$EXAMPLES_FEATURES" -- $CLIPPY_LINTS
cargo clippy $CARGO_FLAGS --locked --manifest-path tutorials/Cargo.toml --all-targets --all-features -- $CLIPPY_LINTS
