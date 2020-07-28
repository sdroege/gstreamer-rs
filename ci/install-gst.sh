set -e

pip3 install meson==0.55.0

git clone --depth 1 https://gitlab.freedesktop.org/gstreamer/gst-build.git --branch master
cd gst-build

meson build -D prefix=/usr/local -D devtools=disabled -D examples=disabled -D gtk_doc=disabled -D introspection=disabled -D libav=disabled -D libnice=disabled -D python=disabled -D ugly=disabled -D vaapi=disabled
ninja -C build
ninja -C build install
