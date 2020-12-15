// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::ToGlibPtr;
use glib::IsA;

pub trait GObjectExtManualGst: 'static {
    fn set_property_from_str(&self, name: &str, value: &str);
}

impl<O: IsA<glib::Object>> GObjectExtManualGst for O {
    fn set_property_from_str(&self, name: &str, value: &str) {
        unsafe {
            ffi::gst_util_set_object_arg(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}
