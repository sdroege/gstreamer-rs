use Element;

use glib;
use glib::IsA;
use glib::translate::{ToGlibPtr, from_glib};

use ffi;

impl Element {
    pub fn link_many<E: IsA<Element>>(elements: &[&E]) -> Result<(), glib::BoolError> {
        for (e1, e2) in elements.iter().zip(elements.iter().skip(1)) {
            unsafe {
                let ret: bool = from_glib(ffi::gst_element_link(e1.to_glib_none().0, e2.to_glib_none().0));
                if !ret {
                    return Err(glib::BoolError("Failed to link elements"));
                }
            }
        }

        return Ok(());
    }

    pub fn unlink_many<E: IsA<Element>>(elements: &[&E]) {
        for (e1, e2) in elements.iter().zip(elements.iter().skip(1)) {
            unsafe {
                ffi::gst_element_unlink(e1.to_glib_none().0, e2.to_glib_none().0);
            }
        }
    }
}
