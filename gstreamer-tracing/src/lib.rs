#![doc = include_str!("../README.md")]
#![deny(unreachable_pub)]
// This crate interacts directly with the C API of glib, gobject and gstreamer libraries. As a
// result implementation of this crate uses unsafe code quite liberally.
//
// Originally the motivation to introduce the unsafe code was to reduce the build times and avoid
// the fairly heavy dependencies introduced by the `gstreamer` crate. However, since then we
// introduced `gstreamer` back in order to implement some `GstElement`s to generate spans from
// wrapped elements.
//
// We continue to use the unsafe code for hooking into the log subsystem however, for it avoids a
// non-free layer of abstraction (allocations for the closure, for instance).
//
// Additionally, it makes it easier to cross-check for soundness issues in the upstream library,
// such as https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/-/issues/352
#![deny(unsafe_op_in_unsafe_fn)]

macro_rules! skip_assert_initialized {
    () => {};
}

pub use gst;
pub use log::attach_span;
use tracing_core::field::Value;

#[macro_use]
mod macros;
mod callsite;
mod log;
pub mod tracer;

/// The top-level target component of the events and spans dispatched to `tracing` by this library.
///
/// You can use this to filter events with e.g. `RUST_LOG` environment variable when using the fmt
/// subscriber. The value of this constant is not guaranteed to be stable.
///
/// See [`tracing::Metadata`][tracing_core::Metadata] for further information.
pub const TARGET: &str = "gstreamer";

trait UnsizeValue {
    fn unsize_value(&self) -> Option<&dyn Value>;
}

impl<V: Value> UnsizeValue for Option<V> {
    fn unsize_value(&self) -> Option<&dyn Value> {
        match self {
            Some(v) => Some(v as &dyn Value),
            None => None,
        }
    }
}

struct PadFlags(u32);
impl std::fmt::Display for PadFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use gst::ffi as gffi;
        f.write_str("{")?;
        let mut sep = false;
        let flags = [
            (gffi::GST_PAD_FLAG_ACCEPT_INTERSECT, "ACCEPT_INTERSECT"),
            (gffi::GST_PAD_FLAG_ACCEPT_TEMPLATE, "ACCEPT_TEMPLATE"),
            (gffi::GST_PAD_FLAG_BLOCKED, "BLOCKED"),
            (gffi::GST_PAD_FLAG_BLOCKING, "BLOCKING"),
            (gffi::GST_PAD_FLAG_EOS, "EOS"),
            (gffi::GST_PAD_FLAG_FIXED_CAPS, "FIXED_CAPS"),
            (gffi::GST_PAD_FLAG_FLUSHING, "FLUSHING"),
            (gffi::GST_PAD_FLAG_NEED_PARENT, "NEED_PARENT"),
            (gffi::GST_PAD_FLAG_NEED_RECONFIGURE, "NEED_RECONFIGURE"),
            (gffi::GST_PAD_FLAG_PENDING_EVENTS, "PENDING_EVENTS"),
            (gffi::GST_PAD_FLAG_PROXY_ALLOCATION, "PROXY_ALLOCATION"),
            (gffi::GST_PAD_FLAG_PROXY_CAPS, "PROXY_CAPS"),
            (gffi::GST_PAD_FLAG_PROXY_SCHEDULING, "PROXY_SCHEDULING"),
        ];
        for (flag, name) in flags {
            if self.0 & flag != 0 {
                if sep {
                    f.write_str(", ")?;
                }
                f.write_str(name)?;
                sep = true;
            }
        }
        f.write_str("}")?;
        Ok(())
    }
}

fn state_desc(state: gst::ffi::GstState) -> &'static str {
    skip_assert_initialized!();
    use gst::ffi as gffi;
    match state {
        gffi::GST_STATE_NULL => "null",
        gffi::GST_STATE_PAUSED => "paused",
        gffi::GST_STATE_PLAYING => "playing",
        gffi::GST_STATE_READY => "ready",
        gffi::GST_STATE_VOID_PENDING => "void-pending",
        _ => "unknown",
    }
}

/// Enable the integration between GStreamer logging system and the `tracing` library.
///
/// Once enabled the default [`tracing::Subscriber`][tracing_core::subscriber::Subscriber] will
/// receive an event for each of the GStreamer debug log messages.
///
/// The events produced this way will specify the “current” span as the event's parent. Doing
/// nothing else, there won't be any span to act as the parent. Consider also using the
/// integrations for producing spans.
///
/// This function can be executed at any time and will process events that occur after its
/// execution.
///
/// Calling this function multiple times may cause duplicate events to be produced.
pub fn integrate_events() {
    log::debug_add_log_function();
}

/// Enable the integration between GStreamer tracing system and the `tracing` library.
///
/// Once enabled the default [`tracing::Subscriber`][tracing_core::subscriber::Subscriber] will
/// have spans entered for certain GStreamer events such as data being pushed to a pad.
///
/// This will provide some meaningful context to the events produced by integrating those.
///
/// This function may only be called after `gst::init`.
pub fn integrate_spans() {
    gst::glib::Object::new::<tracer::TracingTracer>();
}

/// Disable the integration between GStreamer logging system and the `tracing` library.
///
/// As an implementation detail, this will _not_ release certain resources that have been allocated
/// during the period of event integration. Some of the resources are required to live for
/// `'static` and therefore cannot be soundly released by any other way except by terminating the
/// program.
pub fn disable_events() {
    log::debug_remove_log_function();
}

#[cfg(test)]
mod tests;
