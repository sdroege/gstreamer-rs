use gstreamer::{glib, subclass::prelude::*};
use tracing::error;

use crate::tracer::{TracingTracer, TracingTracerImpl};

#[derive(Default)]
pub struct FmtTracer {}

#[glib::object_subclass]
impl ObjectSubclass for FmtTracer {
    const NAME: &'static str = "FmtTracer";
    type Type = super::FmtTracer;
    type ParentType = TracingTracer;
    type Interfaces = ();
}

impl ObjectImpl for FmtTracer {
    fn constructed(&self) {
        if let Err(e) = tracing_subscriber::fmt::try_init() {
            error!("Failed to initialize tracing subscriber: {e:?}");
        }

        self.parent_constructed();
    }
}

impl GstObjectImpl for FmtTracer {}
impl TracerImpl for FmtTracer {}
impl TracingTracerImpl for FmtTracer {}
