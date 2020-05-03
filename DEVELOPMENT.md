# How to update the bindings

  * Make sure gstreamer-rs-sys is up to date
  * Take the updated .gir files from gstreamer-rs-sys and copy them over
  * If there is a new GStreamer version: Manually update `gst*/Cargo.toml`
  * Run generator.py
  * Investigate the diff, fix any mess-ups, look at commented functions and
    implement them manually
  * `cargo build`
  * `for f in (ls |grep gstreamer); cd $f; cargo build --features v1_18; cd ..; end`
     (or with the new version you just added)
