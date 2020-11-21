use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ControlPoint(Boxed<ffi::GstControlPoint>);

    match fn {
        copy => |ptr| ffi::gst_control_point_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_control_point_free(ptr),
        get_type => || ffi::gst_control_point_get_type(),
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
