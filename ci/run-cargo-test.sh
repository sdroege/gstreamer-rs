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
    cargo build --locked --color=always --manifest-path examples/Cargo.toml --bins --examples --all-features
    cargo build --locked --color=always --manifest-path tutorials/Cargo.toml --bins --examples --all-features
fi
