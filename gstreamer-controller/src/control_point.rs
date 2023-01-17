// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstControlPoint")]
    pub struct ControlPoint(Boxed<ffi::GstControlPoint>);

    match fn {
        copy => |ptr| ffi::gst_control_point_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_control_point_free(ptr),
        type_ => || ffi::gst_control_point_get_type(),
    }
}

impl ControlPoint {
    pub fn timestamp(&self) -> gst::ClockTime {
        unsafe { try_from_glib((*self.as_ptr()).timestamp).expect("undefined timestamp") }
    }

    pub fn value(&self) -> f64 {
        unsafe { (*self.as_ptr()).value }
    }
}

unsafe impl Send for ControlPoint {}
unsafe impl Sync for ControlPoint {}
