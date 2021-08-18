use gstreamer::ffi::{
    gst_debug_add_log_function, gst_debug_category_get_name, gst_debug_message_get,
    gst_debug_remove_log_function_by_data, GstDebugCategory, GstDebugLevel, GstDebugMessage,
};
pub(crate) use gstreamer::ffi::{
    GST_LEVEL_COUNT, GST_LEVEL_DEBUG, GST_LEVEL_ERROR, GST_LEVEL_FIXME, GST_LEVEL_INFO,
    GST_LEVEL_LOG, GST_LEVEL_MEMDUMP, GST_LEVEL_TRACE, GST_LEVEL_WARNING,
};
use gstreamer::glib::translate::FromGlibPtrBorrow;
use libc::{c_char, c_int, c_void};
use std::{convert::TryFrom, ffi::CStr};

pub(crate) type DebugLevel = GstDebugLevel;

type LogFn = fn(DebugCategory, DebugLevel, &CStr, &CStr, u32, LoggedObject<'_>, DebugMessage);

pub(crate) enum LoggedObject<'a> {
    Valid(&'a gstreamer::glib::Object),
    ZeroRef(*mut gstreamer::glib::gobject_ffi::GObject),
    Null,
}

pub(crate) struct DebugCategory(*mut GstDebugCategory);
impl DebugCategory {
    pub(crate) fn name(&self) -> &CStr {
        unsafe {
            // SAFETY: Users of the GStreamer `gst_debug_log` API are required to pass in a
            // valid `GstDebugCategory`. `gst_debug_category_get_name` shall return a valid
            // null terminated string.
            CStr::from_ptr(
                gst_debug_category_get_name(self.0)
                    .as_ref()
                    .expect("`category` has no name?"),
            )
        }
    }
}

pub(crate) struct DebugMessage(*mut GstDebugMessage);
impl DebugMessage {
    pub(crate) fn message(&self) -> Option<&CStr> {
        unsafe {
            // SAFETY: This function has no soundness invariants. It can return a null pointer,
            // which we handle by `as_ref`. A valid pointer to a null-terminated C string must
            // be returned.
            gst_debug_message_get(self.0)
                .as_ref()
                .map(|v| CStr::from_ptr(v))
        }
    }
}

unsafe extern "C" fn log_callback(
    category: *mut gstreamer::ffi::GstDebugCategory,
    level: gstreamer::ffi::GstDebugLevel,
    file: *const c_char,
    module: *const c_char,
    line: c_int,
    gobject: *mut gstreamer::glib::gobject_ffi::GObject,
    message: *mut gstreamer::ffi::GstDebugMessage,
    actual_cb: *mut c_void,
) {
    // SAFETY: unwinding back into C land is UB. We abort if there was a panic inside.
    std::panic::catch_unwind(move || {
        // Take extra care to make sure nothing sketchy is going on.
        if category.is_null() || message.is_null() {
            return;
        }
        let actual_cb = unsafe {
            // SAFETY: We pass in a `LogFn` as the callback in `debug_add_log_function` which
            // is the only way this code can be reached. This below extracts the originally
            // passed in value.
            *(&actual_cb as *const *mut c_void).cast::<LogFn>()
        };
        let file = unsafe {
            // SAFETY: Users of the GStreamer `gst_debug_log` API are required to pass in a
            // null terminated string.
            CStr::from_ptr(file.as_ref().expect("`file` string is nullptr"))
        };
        let module = unsafe {
            // SAFETY: Users of the GStreamer `gst_debug_log` API are required to pass in a
            // null terminated string.
            CStr::from_ptr(module.as_ref().expect("`function` string is nullptr"))
        };
        let line = u32::try_from(line).expect("`line` is not a valid u32");
        let storage;
        let gobject = if gobject.is_null() {
            LoggedObject::Null
        } else if unsafe {
            // SAFETY: we have verified that the object pointer is not null above.
            (*gobject).ref_count == 0
        } {
            LoggedObject::ZeroRef(gobject)
        } else {
            unsafe {
                // SAFETY: we have verified that the pointer is non-null and has references (is
                // valid)
                storage = gstreamer::glib::Object::from_glib_borrow(gobject);
            }
            LoggedObject::Valid(&*storage)
        };
        actual_cb(
            DebugCategory(category),
            level,
            file,
            module,
            line,
            gobject,
            DebugMessage(message),
        );
    })
    .unwrap_or_else(|_e| std::process::abort());
}

pub(crate) fn debug_add_log_function(callback: LogFn) {
    unsafe {
        // SAFETY: this function has no soundness invariants.
        gst_debug_add_log_function(Some(log_callback), callback as *mut _, None);
    }
}

pub(crate) fn debug_remove_log_function(callback: LogFn) {
    unsafe {
        // SAFETY: this function has no soundness invariants.
        gst_debug_remove_log_function_by_data(callback as *mut _);
    }
}
