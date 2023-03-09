#! /bin/bash

set -e

pip3 install meson==1.0.1

# gstreamer-rs already has a 'gstreamer' directory so don't clone there
pushd .
cd ..
git clone https://gitlab.freedesktop.org/gstreamer/gstreamer.git \
    --depth 1 \
    --branch main

cd gstreamer

# plugins required by tests
PLUGINS="-D gst-plugins-base:ogg=enabled \
         -D gst-plugins-base:vorbis=enabled \
         -D gst-plugins-base:theora=enabled \
         -D gst-plugins-good:matroska=enabled \
         -D gst-plugins-good:vpx=enabled \
         -D gst-plugins-bad:opus=enabled \
         -D gst-plugins-ugly:x264=enabled"

meson setup build \
    -D prefix=/usr/local \
    -D gpl=enabled \
    -D ugly=enabled \
    -D examples=disabled \
    -D gtk_doc=disabled \
    -D introspection=disabled \
    -D libav=disabled \
    -D python=disabled \
    -D vaapi=disabled \
    $PLUGINS
meson compile -C build
meson install -C build
ldconfig

cd ..
rm -rf gstreamer/

# Check what plugins we installed
gst-inspect-1.0

popd
