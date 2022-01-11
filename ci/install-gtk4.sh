set -eux

BRANCH=gtk-4-4

git clone https://gitlab.gnome.org/GNOME/gtk.git --branch $BRANCH --depth=1
cd gtk
meson build -D prefix=/usr/local
ninja -C build
ninja -C build install
cd ..
