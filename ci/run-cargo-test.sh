#! /bin/bash

set -ex

rustc --version
cargo --version

# First build and test all the crates with their relevant features
# Keep features in sync with the list below below
get_features() {
    crate=$1
    case "$crate" in
        gstreamer-audio|gstreamer-editing-services|gstreamer-gl|gstreamer-pbutils|gstreamer-rtp|gstreamer-rtsp|gstreamer-video|gstreamer)
            echo "--features=serde,v1_22"
            ;;
        gstreamer-validate)
            echo ""
            ;;
        *)
            echo "--features=v1_22"
            ;;
    esac
}

for crate in gstreamer* gstreamer-gl/{egl,wayland,x11}; do
    if [ -e "$crate/Cargo.toml" ]; then
        if [ -n "$ALL_FEATURES" ]; then
            FEATURES=$(get_features "$crate")
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
    EXAMPLES_FEATURES="--features=gtksink,gtkvideooverlay,gtkvideooverlay-x11,gtkvideooverlay-quartz,rtsp-server,rtsp-server-record,pango-cairo,overlay-composition,gl,gst-gl-x11,gst-gl-wayland,gst-gl-egl,allocators,gst-play,gst-player,ges,image,cairo-rs,gst-video/v1_18"

    cargo build --locked --color=always --manifest-path examples/Cargo.toml --bins --examples "$EXAMPLES_FEATURES"
    cargo build --locked --color=always --manifest-path tutorials/Cargo.toml --bins --examples --all-features
fi
