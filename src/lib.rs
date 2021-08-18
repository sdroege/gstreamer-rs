#![doc = include_str!("../README.mkd")]
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

use gstreamer::prelude::{ObjectExt, ObjectType};
use std::ffi::CStr;
use log::{DebugCategory, DebugLevel, DebugMessage, LoggedObject};
use tracing_core::{field::Value, Callsite, Event, Kind, Level};

mod callsite;
mod log;
mod tracer;

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
            Some(ref v) => Some(v as &dyn Value),
            None => None,
        }
    }
}

fn log_callback(
    category: DebugCategory,
    level: DebugLevel,
    file: &CStr,
    module: &CStr,
    line: u32,
    object: log::LoggedObject<'_>,
    message: DebugMessage,
) {
    let level = match level {
        log::GST_LEVEL_ERROR => Level::ERROR,
        log::GST_LEVEL_WARNING | log::GST_LEVEL_FIXME => Level::WARN,
        log::GST_LEVEL_INFO => Level::INFO,
        log::GST_LEVEL_DEBUG | log::GST_LEVEL_LOG => Level::DEBUG,
        log::GST_LEVEL_TRACE | log::GST_LEVEL_MEMDUMP | log::GST_LEVEL_COUNT => Level::TRACE,
        _ => return,
    };
    let file = file.to_string_lossy();
    let module = module.to_string_lossy();
    let category_name = category.name().to_string_lossy();
    let callsite = callsite::DynamicCallsites::get().callsite_for(
        level,
        "",
        &category_name,
        Some(&file),
        Some(&module),
        Some(line as u32),
        Kind::EVENT,
        &["message", "gobject.address", "gobject.type"],
    );
    let interest = callsite.interest();
    if interest.is_never() {
        return;
    }
    let meta = callsite.metadata();
    tracing_core::dispatcher::get_default(move |dispatcher| {
        if !dispatcher.enabled(meta) {
            return;
        }
        let fields = meta.fields();
        let mut field_iter = fields.iter();
        let message_value = message.message().map(CStr::to_string_lossy);
        let message_value = message_value.as_deref();
        let gobject_addr_value = match object {
            LoggedObject::Valid(o) => Some(o.as_ptr() as usize),
            LoggedObject::ZeroRef(p) => Some(p as usize),
            LoggedObject::Null => None,
        };
        let gobject_type_value = match object {
            LoggedObject::Valid(o) => Some(o.type_().name()),
            _ => None,
        };
        let values = [
            (
                &field_iter.next().expect("message field"),
                message_value.unsize_value(),
            ),
            (
                &field_iter.next().expect("object address field"),
                gobject_addr_value.unsize_value(),
            ),
            (
                &field_iter.next().expect("object type field"),
                gobject_type_value.unsize_value(),
            ),
        ];
        let valueset = fields.value_set(&values);
        let event = Event::new(meta, &valueset);
        dispatcher.event(&event);
    });
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
    log::debug_add_log_function(log_callback);
}

/// Enable the integration between GStreamer tracing system and the `tracing` library.
///
/// Once enabled the default [`tracing::Subscriber`][tracing_core::subscriber::Subscriber] will
/// have spans entered for certain GStreamer events such as data being pushed to a pad.
///
/// This will provide some meaningful context to the events produced by integrating those.
///
/// This function may only be called after `gstreamer::init`.
pub fn integrate_spans() {
    gstreamer::glib::Object::new::<tracer::TracingTracer>(&[]).expect("create the tracer");
}

/// Disable the integration between GStreamer logging system and the `tracing` library.
///
/// As an implementation detail, this will _not_ release certain resources that have been allocated
/// during the period of event integration. Some of the resources are required to live for
/// `'static` and therefore cannot be soundly released by any other way except by terminating the
/// program.
pub fn disintegrate_events() {
    log::debug_remove_log_function(log_callback);
}

/// Register the `gstreamer::Object`s exposed by this library with GStreamer.
///
/// This is only necessary to call if you would like to reference the types exposed by this
/// library by their name through various factories.
pub fn register(p: Option<&gstreamer::Plugin>) -> Result<(), gstreamer::glib::BoolError> {
    gstreamer::Tracer::register(
        p,
        "rusttracing",
        <tracer::TracingTracer as gstreamer::glib::StaticType>::static_type(),
    )?;
    Ok(())
}

mod gst_plugin {
    fn plugin_init(plugin: &gstreamer::Plugin) -> Result<(), gstreamer::glib::BoolError> {
        crate::register(Some(plugin))
    }

    gstreamer::plugin_define!(
        tracing_gstreamer,
        env!("CARGO_PKG_DESCRIPTION"),
        plugin_init,
        env!("CARGO_PKG_VERSION"),
        "unknown", // https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/864
        "https://github.com/standard-ai/tracing-gstreamer",
        "tracing_gstreamer",
        "https://github.com/standard-ai/tracing-gstreamer"
    );
}

#[cfg(test)]
mod tests;
#[cfg(test)]
fn main() {
    tests::run()
}
