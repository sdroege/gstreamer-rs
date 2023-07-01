use gstreamer::glib;

use crate::tracer::TracingTracer;

mod imp;

glib::wrapper! {
    pub struct FmtTracer(ObjectSubclass<imp::FmtTracer>)
       @extends TracingTracer, gstreamer::Tracer, gstreamer::Object;
}
