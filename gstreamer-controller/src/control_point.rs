use glib::translate::*;
use gst_controller_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ControlPoint(Boxed<gst_controller_sys::GstControlPoint>);

    match fn {
        copy => |ptr| gst_controller_sys::gst_control_point_copy(mut_override(ptr)),
        free => |ptr| gst_controller_sys::gst_control_point_free(ptr),
        get_type => || gst_controller_sys::gst_control_point_get_type(),
    }
}

impl ControlPoint {
    pub fn timestamp(&self) -> gst::ClockTime {
        unsafe {
            let ptr = self.to_glib_none().0;
            from_glib((*ptr).timestamp)
        }
    }

    pub fn value(&self) -> f64 {
        unsafe {
            let ptr = self.to_glib_none().0;
            (*ptr).value
        }
    }
}

unsafe impl Send for ControlPoint {}
unsafe impl Sync for ControlPoint {}
