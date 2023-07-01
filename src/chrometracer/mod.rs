use gstreamer::glib;

use crate::tracer::TracingTracer;

mod imp;

glib::wrapper! {
    pub struct ChromeTracer(ObjectSubclass<imp::ChromeTracer>)
       @extends TracingTracer, gstreamer::Tracer, gstreamer::Object;
}
