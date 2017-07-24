use glib;
use glib::IsA;
use glib::translate::ToGlibPtr;

use ffi;

pub trait GObjectExtManualGst {
    fn set_property_from_str(&self, name: &str, value: &str);
}

impl<O: IsA<glib::Object>> GObjectExtManualGst for O {
    fn set_property_from_str(&self, name: &str, value: &str) {
        unsafe {
            ffi::gst_util_set_object_arg(
                self.to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}
