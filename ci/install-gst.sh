set -e

pip3 install meson==0.63.3

# gstreamer-rs already has a 'gstreamer' directory so don't clone there
pushd .
cd ..
git clone --depth 1 https://gitlab.freedesktop.org/gstreamer/gstreamer.git --branch main
cd gstreamer

# plugins required by tests
PLUGINS="-D gst-plugins-base:ogg=enabled -D gst-plugins-base:vorbis=enabled -D gst-plugins-base:theora=enabled -D gst-plugins-good:matroska=enabled -D gst-plugins-good:vpx=enabled -D gst-plugins-bad:opus=enabled"

meson build -D prefix=/usr/local -D examples=disabled -D gtk_doc=disabled -D introspection=disabled -D libav=disabled -D python=disabled -D ugly=disabled -D vaapi=disabled $PLUGINS
ninja -C build
ninja -C build install

cd ..
rm -rf gstreamer/

popd
