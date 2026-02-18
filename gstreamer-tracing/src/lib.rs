#![doc = include_str!("../README.md")]
#![deny(unreachable_pub)]
// This crate uses unsafe code for introspecting GObject fields (ref_count, type,
// name, state, etc.) in the log handler, since the safe bindings don't expose
// these details through `LoggedObject`.
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
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        // Keep the tracer alive for the rest of the process lifetime so parent
        // propagation hooks remain active.
        let tracer = gst::glib::Object::new::<tracer::TracingTracer>();
        std::mem::forget(tracer);
    });
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
