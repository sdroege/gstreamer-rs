#! /bin/sh

set -eux

BRANCH=4.8.1

git clone https://gitlab.gnome.org/GNOME/gtk.git --branch $BRANCH --depth=1
cd gtk

meson build -D prefix=/usr/local -Dbuild-tests=false -Dwayland-protocols:tests=false
ninja -C build
ninja -C build install
cd ..
rm -rf gtk/
