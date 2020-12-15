// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;
use std::fmt;

use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr};

use crate::StructureRef;

gst_define_mini_object_wrapper!(Context, ContextRef, ffi::GstContext, || {
    ffi::gst_context_get_type()
});

impl Context {
    pub fn new(context_type: &str, persistent: bool) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_context_new(
                context_type.to_glib_none().0,
                persistent.to_glib(),
            ))
        }
    }
}

impl ContextRef {
    pub fn get_context_type(&self) -> &str {
        unsafe {
            let raw = ffi::gst_context_get_context_type(self.as_mut_ptr());
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    pub fn has_context_type(&self, context_type: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_context_has_context_type(
                self.as_mut_ptr(),
                context_type.to_glib_none().0,
            ))
        }
    }

    pub fn is_persistent(&self) -> bool {
        unsafe { from_glib(ffi::gst_context_is_persistent(self.as_mut_ptr())) }
    }

    pub fn get_structure(&self) -> &StructureRef {
        unsafe { StructureRef::from_glib_borrow(ffi::gst_context_get_structure(self.as_mut_ptr())) }
    }

    pub fn get_mut_structure(&mut self) -> &mut StructureRef {
        unsafe {
            StructureRef::from_glib_borrow_mut(ffi::gst_context_writable_structure(
                self.as_mut_ptr(),
            ))
        }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ContextRef::fmt(self, f)
    }
}

impl fmt::Debug for ContextRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Context")
            .field("type", &self.get_context_type())
            .field("structure", &self.get_structure())
            .finish()
    }
}
