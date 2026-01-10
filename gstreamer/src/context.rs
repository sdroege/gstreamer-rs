// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt};

use glib::translate::{IntoGlib, ToGlibPtr, from_glib, from_glib_full};

use crate::{StructureRef, ffi};

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

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_context_get_task_pool")]
    pub fn task_pool(&self) -> Option<crate::TaskPool> {
        assert_eq!(self.context_type(), TASK_POOL_CONTEXT_TYPE);

        unsafe {
            use std::ptr;

            let mut pool = ptr::null_mut();
            if from_glib(ffi::gst_context_get_task_pool(self.as_mut_ptr(), &mut pool)) {
                Some(from_glib_full(pool))
            } else {
                None
            }
        }
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_context_set_task_pool")]
    pub fn set_task_pool<'a, T: glib::prelude::IsA<crate::TaskPool>>(
        &self,
        pool: impl Into<Option<&'a T>>,
    ) {
        unsafe {
            ffi::gst_context_set_task_pool(
                self.as_mut_ptr(),
                pool.into().map(|d| d.as_ref()).to_glib_none().0,
            );
        }
    }
}

#[cfg(feature = "v1_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
#[doc(alias = "GST_TASK_POOL_CONTEXT_TYPE")]
pub static TASK_POOL_CONTEXT_TYPE: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_TASK_POOL_CONTEXT_TYPE) };

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
