// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt};

use glib::translate::{from_glib, from_glib_full, IntoGlib, ToGlibPtr};

use crate::StructureRef;

mini_object_wrapper!(Context, ContextRef, ffi::GstContext, || {
    ffi::gst_context_get_type()
});

impl Context {
    #[doc(alias = "gst_context_new")]
    pub fn new(context_type: &str, persistent: bool) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_context_new(
                context_type.to_glib_none().0,
                persistent.into_glib(),
            ))
        }
    }
}

impl ContextRef {
    #[doc(alias = "get_context_type")]
    #[doc(alias = "gst_context_get_context_type")]
    pub fn context_type(&self) -> &str {
        unsafe {
            let raw = ffi::gst_context_get_context_type(self.as_mut_ptr());
            CStr::from_ptr(raw).to_str().unwrap()
        }
    }

    #[doc(alias = "gst_context_has_context_type")]
    pub fn has_context_type(&self, context_type: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_context_has_context_type(
                self.as_mut_ptr(),
                context_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_context_is_persistent")]
    pub fn is_persistent(&self) -> bool {
        unsafe { from_glib(ffi::gst_context_is_persistent(self.as_mut_ptr())) }
    }

    #[doc(alias = "get_structure")]
    #[doc(alias = "gst_context_get_structure")]
    pub fn structure(&self) -> &StructureRef {
        unsafe { StructureRef::from_glib_borrow(ffi::gst_context_get_structure(self.as_mut_ptr())) }
    }

    #[doc(alias = "get_mut_structure")]
    pub fn structure_mut(&mut self) -> &mut StructureRef {
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
            .field("type", &self.context_type())
            .field("structure", &self.structure())
            .finish()
    }
}
