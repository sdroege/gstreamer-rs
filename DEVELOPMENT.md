# How to update the bindings

  * Take the updated .gir files (e.g. from your gstreamer main repo checkout)
    and put them in the gst-gir-files directory
  * In the gst-gir-files directory, run ./fix.sh
  * Commit the changes to gst-gir-files and create a commit in gstreamer-rs to
    update the reference
  * If there is a new GStreamer version: Manually update `gst*/Cargo.toml`
  * Run generator.py
  * Investigate the diff, fix any mess-ups, look at commented functions and
    implement them manually
  * `cargo build`
  * `for f in (ls |grep gstreamer); cd $f; cargo build --features v1_18; cd ..; end`
     (or with the new version you just added)
  * Commit your changes to gstreamer-rs
