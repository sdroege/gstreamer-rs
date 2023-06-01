#! /bin/sh

set -eux

BRANCH=4.10.3

git clone https://gitlab.gnome.org/GNOME/gtk.git --branch $BRANCH --depth=1
cd gtk

meson setup build \
    -D prefix=/usr/local \
    -Dbuild-tests=false  \
    -Dwayland-protocols:tests=false
meson compile -C build
meson install -C build
ldconfig
cd ..
rm -rf gtk/
