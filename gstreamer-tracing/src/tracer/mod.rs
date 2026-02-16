use gst::{Object, Tracer, glib, subclass::prelude::*};

mod imp;

glib::wrapper! {
    pub struct TracingTracer(ObjectSubclass<imp::TracingTracer>)
       @extends Tracer, Object;
}

glib::wrapper! {
    #[doc(hidden)]
    pub struct SpanPropagationTracer(ObjectSubclass<imp::SpanPropagationTracer>)
       @extends Tracer, Object;
}

/// Trait for implementing custom tracers that extend `TracingTracer`.
///
/// Implement this trait to create tracers that build upon the GStreamer-tracing
/// integration, such as tracers that use specific tracing subscribers.
pub trait TracingTracerImpl: TracerImpl {}

unsafe impl<T: TracingTracerImpl> IsSubclassable<T> for TracingTracer {
    fn class_init(class: &mut glib::Class<Self>) {
        skip_assert_initialized!();
        Self::parent_class_init::<T>(class);
    }
}
