export RUSTUP_HOME='/usr/local/rustup'
export CARGO_HOME='/usr/local/cargo'
export PATH=$PATH:/usr/local/cargo/bin

export PKG_CONFIG_PATH=/usr/local/gstreamer/lib/x86_64-linux-gnu/pkgconfig
export GST_PLUGIN_SYSTEM_PATH=/usr/local/gstreamer/lib/x86_64-linux-gnu/gstreamer-1.0
export GST_PLUGIN_SCANNER=/usr/local/gstreamer/libexec/gstreamer-1.0/gst-plugin-scanner
export PATH=$PATH:/usr/local/gstreamer/bin
export LD_LIBRARY_PATH=/usr/local/gstreamer/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH
