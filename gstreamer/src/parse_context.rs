// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ParseContext(Boxed<ffi::GstParseContext>);

    match fn {
        copy => |ptr| {
            cfg_if::cfg_if! {
                if #[cfg(feature = "v1_12_1")] {
                    ffi::gst_parse_context_copy(ptr)
                } else {
                    glib::gobject_ffi::g_boxed_copy(ffi::gst_parse_context_get_type(), ptr as *mut _) as *mut ffi::GstParseContext
                }
            }
        },
        free => |ptr| ffi::gst_parse_context_free(ptr),
        type_ => || ffi::gst_parse_context_get_type(),
    }
}

unsafe impl Send for ParseContext {}
unsafe impl Sync for ParseContext {}

impl ParseContext {
    pub fn new() -> Self {
        unsafe { from_glib_full(ffi::gst_parse_context_new()) }
    }

    #[doc(alias = "get_missing_elements")]
    pub fn missing_elements(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_parse_context_get_missing_elements(
                mut_override(self.to_glib_none().0),
            ))
        }
    }
}

impl Default for ParseContext {
    fn default() -> Self {
        Self::new()
    }
}
