set -e

pip3 install meson==0.59.1

# gstreamer-rs already has a 'gstreamer' directory so don't clone there
pushd .
cd ..
git clone --depth 1 https://gitlab.freedesktop.org/gstreamer/gstreamer.git --branch main
cd gstreamer

meson build -D prefix=/usr/local -D devtools=disabled -D examples=disabled -D gtk_doc=disabled -D introspection=disabled -D libav=disabled -D libnice=disabled -D python=disabled -D ugly=disabled -D vaapi=disabled
ninja -C build
ninja -C build install

popd