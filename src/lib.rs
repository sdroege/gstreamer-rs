#![doc = include_str!("../README.mkd")]
// This crate interacts directly with the C API of glib, gobject and gstreamer libraries. As a
// result implementation of this crate uses unsafe code quite liberally.
//
// There are a couple motivations for doing so:
//
// * This crate exposes tracing in its public API, and therefore wants to be versioned alongside
// `tracing`. If the `glib`, `gobject` and `gstreamer` crates were used, it'd be difficult for the
// consumers of this crate to match the version of this specific crate that depends on the same
// versions of `glib`, `gobject` and `gstreamer`;
// * Duplicating the crates seems very suboptimal as build times of `glib`, `gobject` and
// `gstreamer` are quite significant due to the dependencies that they pull in.
#![deny(unsafe_op_in_unsafe_fn)]

use once_cell::sync::OnceCell;
use std::{ffi::CStr, sync::PoisonError};
use sys::{
    gobject::Object,
    gst::{DebugCategory, DebugLevel, DebugMessage},
};
use tracing_core::{
    field::{FieldSet, Value},
    identify_callsite,
    metadata::Kind,
    Event, Level, Metadata,
};

mod callsite;
mod sys;

/// The name of the events dispatched to `tracing` by this library.
///
/// The value of this constant is not guaranteed to be stable.
///
/// See [`tracing::Metadata`][tracing_core::Metadata] for further information.
pub const NAME: &str = "gstreamer";

/// A map of metadata allocations we've made throughout the lifetime of the process.
///
/// [`tracing`] requires the metadata to have a lifetime of `'static` for them to be usable. This
/// is required for a number of reasons, one of which is performance of filtering the messages.
///
/// In order to facilitate this, we maintain a static map which allows us to allocate the necessary
/// data on the heap (and with the required `'static` lifetime we effectively leak)
struct MetadataAllocations {
    data: std::sync::RwLock<Map>,
}

type Map = std::collections::BTreeMap<Key, &'static Metadata<'static>>;
type Key = (
    Level,
    u32,
    Option<&'static str>,
    Option<&'static str>,
    &'static str,
);

fn leak_str(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

impl MetadataAllocations {
    fn get() -> &'static Self {
        static MAP: OnceCell<MetadataAllocations> = OnceCell::new();
        MAP.get_or_init(|| MetadataAllocations {
            data: std::sync::RwLock::new(Map::new()),
        })
    }

    fn metadata_for(
        &self,
        level: Level,
        category: &str,
        file: Option<&str>,
        module: Option<&str>,
        line: u32,
    ) -> &'static Metadata<'static> {
        let key = (level, line, module, file, category);
        if let Some(metadata) = self
            .data
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&key)
        {
            return metadata;
        }
        let mut lock = self.data.write().unwrap_or_else(PoisonError::into_inner);
        let key = (
            level,
            line,
            module.map(leak_str),
            file.map(leak_str),
            leak_str(category),
        );
        lock.entry(key).or_insert_with_key(|k| {
            let callsite = callsite::GstCallsite::make_static();
            let fields = FieldSet::new(
                &["message", "gobject_address", "gobject_type"],
                identify_callsite!(callsite),
            );
            let metadata = Box::leak(Box::new(Metadata::new(
                "gstreamer",
                k.4,
                k.0,
                k.3,
                Some(k.1),
                k.2,
                fields,
                Kind::EVENT,
            )));
            callsite.set_metadata(metadata);
            metadata
        })
    }
}

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
    object: Option<Object>,
    message: DebugMessage,
) {
    let level = match level {
        sys::GST_LEVEL_ERROR => Level::ERROR,
        sys::GST_LEVEL_WARNING | sys::GST_LEVEL_FIXME => Level::WARN,
        sys::GST_LEVEL_INFO => Level::INFO,
        sys::GST_LEVEL_DEBUG | sys::GST_LEVEL_LOG => Level::DEBUG,
        sys::GST_LEVEL_TRACE | sys::GST_LEVEL_MEMDUMP | sys::GST_LEVEL_COUNT => Level::TRACE,
        _ => return,
    };
    let file = file.to_string_lossy();
    let module = module.to_string_lossy();
    let category_name = category.name().to_string_lossy();
    let meta = MetadataAllocations::get().metadata_for(
        level,
        &category_name,
        Some(&file),
        Some(&module),
        line as u32,
    );
    let fields = meta.fields();
    let mut field_iter = fields.iter();
    let message_value = message.message().map(CStr::to_string_lossy);
    let message_value = message_value.as_deref();
    let gobject_addr_value = object.as_ref().map(Object::address);
    let gobject_type_value = object.as_ref().map(Object::type_name);
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
    Event::dispatch(meta, &valueset);
}

/// Enable the integration between GStreamer and the `tracing` library.
///
/// Once enabled the default [`tracing::Subscriber`][tracing_core::subscriber::Subscriber] will
/// receive an event for each of the GStreamer debug log messages.
///
/// This function can be executed at any time and will process events that occur after its
/// execution.
///
/// Calling this function multiple times may cause duplicate events to be produced.
pub fn integrate() {
    sys::gst::debug_add_log_function(log_callback);
}

/// Disable the integration between GStreamer and the `tracing` library.
///
/// As an implementation detail, this will _not_ release certain resources that have been allocated
/// during the period when this integration was enabled. Some of the resources are required to live
/// for `'static` and therefore cannot be soundly released by any other way except by terminating
/// the program.
pub fn disintegrate() {
    sys::gst::debug_remove_log_function(log_callback);
}

#[cfg(test)]
mod tests;
#[cfg(test)]
fn main() {
    tests::run()
}
