#! /bin/bash

set -e

DEFAULT_BRANCH="$GST_UPSTREAM_BRANCH"

pip3 install meson==1.7.2 --break-system-packages

# gstreamer-rs already has a 'gstreamer' directory so don't clone there
pushd .
cd ..
git clone https://gitlab.freedesktop.org/gstreamer/gstreamer.git \
    --depth 1 \
    --branch "$DEFAULT_BRANCH"

cd gstreamer

# plugins required by tests
PLUGINS=(
    -Dgst-plugins-base:ogg=enabled
    -Dgst-plugins-base:vorbis=enabled
    -Dgst-plugins-base:theora=enabled
    -Dgst-plugins-good:matroska=enabled
    -Dgst-plugins-good:vpx=enabled
    -Dgst-plugins-bad:opus=enabled
    -Dgst-plugins-ugly:x264=enabled
)

meson setup build \
    -Dprefix=/usr/local \
    -Dgpl=enabled \
    -Dugly=enabled \
    -Dexamples=disabled \
    -Dgtk_doc=disabled \
    -Dintrospection=disabled \
    -Dlibav=disabled \
    -Dpython=disabled \
    -Dvaapi=disabled \
    "${PLUGINS[@]}" "$@"
meson compile -C build
meson install -C build
ldconfig

cd ..
rm -rf gstreamer/

# Check what plugins we installed
gst-inspect-1.0

popd
