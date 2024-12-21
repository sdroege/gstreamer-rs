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

for crate in gstreamer*/sys gstreamer-gl/*/sys; do
    if [ -e "$crate/Cargo.toml" ]; then
        echo "Building $crate with --all-features"
        cargo build $CARGO_FLAGS --locked --manifest-path "$crate/Cargo.toml" --all-features
    fi
done

for crate in gstreamer/sys \
             gstreamer-allocators/sys \
             gstreamer-analytics/sys \
             gstreamer-app/sys \
             gstreamer-audio/sys \
             gstreamer-base/sys \
             gstreamer-check/sys \
             gstreamer-controller/sys \
             gstreamer-editing-services/sys \
             gstreamer-gl/sys \
             gstreamer-gl/egl/sys \
             gstreamer-gl/wayland/sys \
             gstreamer-gl/x11/sys \
             gstreamer-mpegts/sys \
             gstreamer-net/sys \
             gstreamer-pbutils/sys \
             gstreamer-play/sys \
             gstreamer-player/sys \
             gstreamer-rtp/sys \
             gstreamer-rtsp-server/sys \
             gstreamer-rtsp/sys \
             gstreamer-sdp/sys \
             gstreamer-tag/sys \
             gstreamer-validate/sys \
             gstreamer-video/sys \
             gstreamer-webrtc/sys; do
    echo "Testing $crate with --all-features)"
    RUST_BACKTRACE=1 cargo nextest run $CARGO_NEXTEST_FLAGS $CARGO_FLAGS --locked --manifest-path $crate/Cargo.toml --all-features

    new_report_dir="$parent/junit_reports/$crate"
    mkdir -p "$new_report_dir"
    mv "$parent/target/nextest/ci/junit.xml" "$new_report_dir/junit.xml"
done
