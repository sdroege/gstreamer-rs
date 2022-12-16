set -e

pip3 install meson==0.64.1

# gstreamer-rs already has a 'gstreamer' directory so don't clone there
pushd .
cd ..
git clone --depth 1 https://gitlab.freedesktop.org/gstreamer/gstreamer.git --branch main
cd gstreamer

# plugins required by tests
PLUGINS="-D gst-plugins-base:ogg=enabled -D gst-plugins-base:vorbis=enabled -D gst-plugins-base:theora=enabled -D gst-plugins-good:matroska=enabled -D gst-plugins-good:vpx=enabled -D gst-plugins-bad:opus=enabled -D gst-plugins-ugly:x264=enabled"

meson build -D prefix=/usr/local -D gpl=enabled -D ugly=enabled  -D examples=disabled -D gtk_doc=disabled -D introspection=disabled -D libav=disabled -D python=disabled -D vaapi=disabled $PLUGINS
ninja -C build
ninja -C build install

cd ..
rm -rf gstreamer/

# Check what plugins we installed
gst-inspect-1.0

popd
