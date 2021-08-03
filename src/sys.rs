use libc::{c_char, c_int, c_uint, c_void};

pub(crate) type GstDebugMessage = *mut c_void;
pub(crate) type GstDebugLevel = c_int;
pub(crate) type GstLogFunction = Option<
    unsafe extern "C" fn(
        *mut GstDebugCategory,
        GstDebugLevel,
        *const c_char,
        *const c_char,
        c_int,
        *mut GObject,
        *mut GstDebugMessage,
        *mut c_void,
    ),
>;

pub(crate) const GST_LEVEL_ERROR: GstDebugLevel = 1;
pub(crate) const GST_LEVEL_WARNING: GstDebugLevel = 2;
pub(crate) const GST_LEVEL_FIXME: GstDebugLevel = 3;
pub(crate) const GST_LEVEL_INFO: GstDebugLevel = 4;
pub(crate) const GST_LEVEL_DEBUG: GstDebugLevel = 5;
pub(crate) const GST_LEVEL_LOG: GstDebugLevel = 6;
pub(crate) const GST_LEVEL_TRACE: GstDebugLevel = 7;
pub(crate) const GST_LEVEL_MEMDUMP: GstDebugLevel = 9;
pub(crate) const GST_LEVEL_COUNT: GstDebugLevel = 10;

#[repr(C)]
pub(crate) struct GstDebugCategory {
    pub threshold: c_int,
    pub color: c_uint,
    pub name: *const c_char,
    pub description: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GTypeInstance {
    pub g_class: *mut c_void,
}

#[repr(C)]
pub struct GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: c_uint,
    pub qdata: *mut c_void,
}

// gstreamer
#[link(name = "gstreamer-1.0")]
extern "C" {
    pub(crate) fn gst_debug_category_get_name(category: *mut GstDebugCategory) -> *const c_char;
    pub(crate) fn gst_debug_message_get(message: *mut GstDebugMessage) -> *const c_char;
    pub(crate) fn gst_debug_add_log_function(
        func: GstLogFunction,
        user_data: *mut c_void,
        notify: Option<unsafe extern "C" fn(*mut c_void)>,
    );
    pub(crate) fn gst_debug_remove_log_function_by_data(data: *mut c_void) -> c_uint;
}

// gobject
#[link(name = "gobject-2.0")]
#[link(name = "glib-2.0")]
extern "C" {
    pub(crate) fn g_type_name_from_instance(instance: *mut GTypeInstance) -> *const c_char;
}

pub(crate) mod gobject {
    use std::ffi::CStr;

    pub(crate) struct Object(*mut super::GObject);

    impl Object {
        pub(super) fn new(gobject: *mut super::GObject) -> Option<Object> {
            if gobject.is_null() {
                None
            } else {
                Some(super::gobject::Object(gobject))
            }
        }

        pub(crate) fn address(&self) -> usize {
            self.0 as usize
        }

        pub(crate) fn type_name(&self) -> &str {
            unsafe {
                // SAFETY: The `g_type_name_from_instance` function is defined to return a
                // null-terminated string. `self.0` must be a valid pointer to the object at the
                // construction time of the `Object` (as ensured by the `new`).
                let name = super::g_type_name_from_instance(std::ptr::addr_of_mut!(
                    (*self.0).g_type_instance
                ));
                CStr::from_ptr(name)
                    .to_str()
                    .expect("type names must be utf-8")
            }
        }
    }
}

pub(crate) mod gst {
    use libc::{c_char, c_int, c_void};
    use std::{convert::TryFrom, ffi::CStr};

    pub(crate) type DebugLevel = super::GstDebugLevel;
    type LogFn = fn(
        DebugCategory,
        DebugLevel,
        &CStr,
        &CStr,
        u32,
        Option<super::gobject::Object>,
        DebugMessage,
    );

    pub(crate) struct DebugCategory(*mut super::GstDebugCategory);
    impl DebugCategory {
        pub(crate) fn name(&self) -> &CStr {
            unsafe {
                // SAFETY: Users of the GStreamer `gst_debug_log` API are required to pass in a
                // valid `GstDebugCategory`. `gst_debug_category_get_name` shall return a valid
                // null terminated string.
                CStr::from_ptr(
                    super::gst_debug_category_get_name(self.0)
                        .as_ref()
                        .expect("`category` has no name?"),
                )
            }
        }
    }

    pub(crate) struct DebugMessage(*mut super::GstDebugMessage);
    impl DebugMessage {
        pub(crate) fn message(&self) -> Option<&CStr> {
            unsafe {
                // SAFETY: This function has no soundness invariants. It can return a null pointer,
                // which we handle by `as_ref`. A valid pointer to a null-terminated C string must
                // be returned.
                super::gst_debug_message_get(self.0)
                    .as_ref()
                    .map(|v| CStr::from_ptr(v))
            }
        }
    }

    unsafe extern "C" fn log_callback(
        category: *mut super::GstDebugCategory,
        level: super::GstDebugLevel,
        file: *const c_char,
        module: *const c_char,
        line: c_int,
        gobject: *mut super::GObject,
        message: *mut super::GstDebugMessage,
        actual_cb: *mut c_void,
    ) {
        // SAFETY: unwinding back into C land is UB. We abort if there was a panic inside.
        std::panic::catch_unwind(move || {
            let actual_cb = unsafe {
                // SAFETY: We pass in a `LogFn` as the callback in `debug_add_log_function` which
                // is the only way this code can be reached. This below extracts the originally
                // passed in value.
                *((&actual_cb) as *const *mut c_void as *const LogFn)
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
            let gobject = super::gobject::Object::new(gobject);
            actual_cb(
                DebugCategory(category),
                level,
                file,
                module,
                line,
                gobject,
                DebugMessage(message),
            )
        })
        .unwrap_or_else(|_e| std::process::abort());
    }

    pub(crate) fn debug_add_log_function(callback: LogFn) {
        unsafe {
            // SAFETY: this function has no soundness invariants.
            super::gst_debug_add_log_function(Some(log_callback), callback as *mut _, None);
        }
    }

    pub(crate) fn debug_remove_log_function(callback: LogFn) {
        unsafe {
            // SAFETY: this function has no soundness invariants.
            super::gst_debug_remove_log_function_by_data(callback as *mut _);
        }
    }
}
