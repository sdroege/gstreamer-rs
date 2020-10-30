# How to update the bindings

  * Take the updated .gir files (e.g. from your gst-build checkout) and put
    them in the gir-files directory
  * In the gir-files directory, run ./fix.sh
  * In the GstVideo-1.0.gir file, the `GST_VIDEO_BUFFER_FLAG_ONEFIELD` and
    `GST_VIDEO_FRAME_FLAG_ONEFIELD` flags are twice. This is a gir bug. Delete
    the second one.
  * If there is a new GStreamer version: Manually update `gst*/Cargo.toml`
  * Run generator.py
  * Investigate the diff, fix any mess-ups, look at commented functions and
    implement them manually
  * `cargo build`
  * `for f in (ls |grep gstreamer); cd $f; cargo build --features v1_18; cd ..; end`
     (or with the new version you just added)
