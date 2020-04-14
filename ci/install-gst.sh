curl -L https://people.freedesktop.org/~slomo/gstreamer-1.16.1.tar.xz | tar xJC /usr/local
sed -i "s;prefix=/root/gstreamer;prefix=/usr/local/gstreamer;g" /usr/local/gstreamer/lib/x86_64-linux-gnu/pkgconfig/*.pc
