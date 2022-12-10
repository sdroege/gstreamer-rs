// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DebugLevel;

use libc::c_char;
use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt;
use std::ptr;

use once_cell::sync::Lazy;

use glib::ffi::gpointer;
use glib::prelude::*;
use glib::translate::*;

#[derive(PartialEq, Eq)]
#[doc(alias = "GstDebugMessage")]
pub struct DebugMessage(ptr::NonNull<ffi::GstDebugMessage>);

impl fmt::Debug for DebugMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("DebugMessage").field(&self.get()).finish()
    }
}

impl DebugMessage {
    #[doc(alias = "gst_debug_message_get")]
    pub fn get(&self) -> Option<Cow<str>> {
        unsafe {
            let message = ffi::gst_debug_message_get(self.0.as_ptr());

            if message.is_null() {
                None
            } else {
                Some(CStr::from_ptr(message).to_string_lossy())
            }
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_debug_message_get_id")]
    pub fn id(&self) -> Option<&str> {
        unsafe {
            let message = ffi::gst_debug_message_get_id(self.0.as_ptr());

            if message.is_null() {
                None
            } else {
                Some(CStr::from_ptr(message).to_str().unwrap())
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[doc(alias = "GstDebugCategory")]
pub struct DebugCategory(Option<ptr::NonNull<ffi::GstDebugCategory>>);

impl DebugCategory {
    #[doc(alias = "gst_debug_category_new")]
    #[doc(alias = "GST_DEBUG_CATEGORY")]
    #[doc(alias = "GST_DEBUG_CATEGORY_INIT")]
    pub fn new(
        name: &str,
        color: crate::DebugColorFlags,
        description: Option<&str>,
    ) -> DebugCategory {
        skip_assert_initialized!();
        extern "C" {
            fn _gst_debug_category_new(
                name: *const c_char,
                color: ffi::GstDebugColorFlags,
                description: *const c_char,
            ) -> *mut ffi::GstDebugCategory;
        }

        // Gets the category if it exists already
        unsafe {
            let ptr = _gst_debug_category_new(
                name.to_glib_none().0,
                color.into_glib(),
                description.to_glib_none().0,
            );
            // Can be NULL if the debug system is compiled out
            DebugCategory(ptr::NonNull::new(ptr))
        }
    }

    #[doc(alias = "gst_debug_get_category")]
    pub fn get(name: &str) -> Option<DebugCategory> {
        skip_assert_initialized!();
        unsafe {
            extern "C" {
                fn _gst_debug_get_category(name: *const c_char) -> *mut ffi::GstDebugCategory;
            }

            let cat = _gst_debug_get_category(name.to_glib_none().0);

            if cat.is_null() {
                None
            } else {
                Some(DebugCategory(Some(ptr::NonNull::new_unchecked(cat))))
            }
        }
    }

    #[doc(alias = "get_threshold")]
    #[doc(alias = "gst_debug_category_get_threshold")]
    pub fn threshold(self) -> crate::DebugLevel {
        match self.0 {
            Some(cat) => unsafe { from_glib(cat.as_ref().threshold) },
            None => crate::DebugLevel::None,
        }
    }

    #[doc(alias = "gst_debug_category_set_threshold")]
    pub fn set_threshold(self, threshold: crate::DebugLevel) {
        if let Some(cat) = self.0 {
            unsafe { ffi::gst_debug_category_set_threshold(cat.as_ptr(), threshold.into_glib()) }
        }
    }

    #[doc(alias = "gst_debug_category_reset_threshold")]
    pub fn reset_threshold(self) {
        if let Some(cat) = self.0 {
            unsafe { ffi::gst_debug_category_reset_threshold(cat.as_ptr()) }
        }
    }

    #[doc(alias = "get_color")]
    #[doc(alias = "gst_debug_category_get_color")]
    pub fn color(self) -> crate::DebugColorFlags {
        match self.0 {
            Some(cat) => unsafe { from_glib(cat.as_ref().color) },
            None => crate::DebugColorFlags::empty(),
        }
    }

    #[doc(alias = "get_name")]
    #[doc(alias = "gst_debug_category_get_name")]
    pub fn name<'a>(self) -> &'a str {
        match self.0 {
            Some(cat) => unsafe { CStr::from_ptr(cat.as_ref().name).to_str().unwrap() },
            None => "",
        }
    }

    #[doc(alias = "get_description")]
    #[doc(alias = "gst_debug_category_get_description")]
    pub fn description<'a>(self) -> Option<&'a str> {
        let cat = self.0?;

        unsafe {
            let ptr = cat.as_ref().description;

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[inline]
    #[doc(alias = "gst_debug_log")]
    #[doc(alias = "gst_debug_log_literal")]
    pub fn log<O: IsA<glib::Object>>(
        self,
        obj: Option<&O>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        module: &glib::GStr,
        line: u32,
        args: fmt::Arguments,
    ) {
        let cat = match self.0 {
            Some(cat) => cat,
            None => return,
        };

        unsafe {
            if level.into_glib() as i32 > cat.as_ref().threshold {
                return;
            }
        }

        self.log_unfiltered(obj, level, file, module, line, args)
    }

    // rustdoc-stripper-ignore-next
    /// Logs without checking the log level.
    #[inline]
    #[doc(alias = "gst_debug_log")]
    #[doc(alias = "gst_debug_log_literal")]
    pub fn log_unfiltered<O: IsA<glib::Object>>(
        self,
        obj: Option<&O>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        module: &glib::GStr,
        line: u32,
        args: fmt::Arguments,
    ) {
        let cat = match self.0 {
            Some(cat) => cat,
            None => return,
        };

        let obj_ptr = match obj {
            Some(obj) => obj.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
            None => ptr::null_mut(),
        };

        let mut w = glib::GStringBuilder::default();

        // Can't really happen but better safe than sorry
        if fmt::write(&mut w, args).is_err() {
            return;
        }

        #[cfg(feature = "v1_20")]
        unsafe {
            ffi::gst_debug_log_literal(
                cat.as_ptr(),
                level.into_glib(),
                file.as_ptr(),
                module.as_ptr(),
                line as i32,
                obj_ptr,
                w.into_string().to_glib_none().0,
            );
        }
        #[cfg(not(feature = "v1_20"))]
        unsafe {
            ffi::gst_debug_log(
                cat.as_ptr(),
                level.into_glib(),
                file.as_ptr(),
                module.as_ptr(),
                line as i32,
                obj_ptr,
                b"%s\0".as_ptr() as *const _,
                ToGlibPtr::<*const i8>::to_glib_none(&w.into_string()).0,
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[inline]
    #[doc(alias = "gst_debug_log_id")]
    #[doc(alias = "gst_debug_log_id_literal")]
    pub fn log_id(
        self,
        id: Option<&str>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        module: &glib::GStr,
        line: u32,
        args: fmt::Arguments,
    ) {
        let cat = match self.0 {
            Some(cat) => cat,
            None => return,
        };

        unsafe {
            if level.into_glib() as i32 > cat.as_ref().threshold {
                return;
            }
        }

        let mut w = glib::GStringBuilder::default();

        // Can't really happen but better safe than sorry
        if fmt::write(&mut w, args).is_err() {
            return;
        }

        self.log_id_unfiltered(id, level, file, module, line, args);
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    // rustdoc-stripper-ignore-next
    /// Logs without checking the log level.
    #[inline]
    #[doc(alias = "gst_debug_log_id")]
    #[doc(alias = "gst_debug_log_id_literal")]
    pub fn log_id_unfiltered(
        self,
        id: Option<&str>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        module: &glib::GStr,
        line: u32,
        args: fmt::Arguments,
    ) {
        let cat = match self.0 {
            Some(cat) => cat,
            None => return,
        };

        let mut w = glib::GStringBuilder::default();

        // Can't really happen but better safe than sorry
        if fmt::write(&mut w, args).is_err() {
            return;
        }

        unsafe {
            ffi::gst_debug_log_id_literal(
                cat.as_ptr(),
                level.into_glib(),
                file.as_ptr(),
                module.as_ptr(),
                line as i32,
                id.to_glib_none().0,
                w.into_string().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_all_categories")]
    #[doc(alias = "gst_debug_get_all_categories")]
    pub fn all_categories() -> glib::SList<DebugCategory> {
        unsafe { glib::SList::from_glib_container_static(ffi::gst_debug_get_all_categories()) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_debug_log_get_line")]
    pub fn get_line(
        &self,
        level: crate::DebugLevel,
        file: &str,
        module: &str,
        line: u32,
        object: Option<&LoggedObject>,
        message: &DebugMessage,
    ) -> Option<glib::GString> {
        let cat = self.0?;

        unsafe {
            from_glib_full(ffi::gst_debug_log_get_line(
                cat.as_ptr(),
                level.into_glib(),
                file.to_glib_none().0,
                module.to_glib_none().0,
                line as i32,
                object.map(|o| o.as_ptr()).unwrap_or(ptr::null_mut()),
                message.0.as_ptr(),
            ))
        }
    }
}

unsafe impl Sync for DebugCategory {}
unsafe impl Send for DebugCategory {}

impl fmt::Debug for DebugCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("DebugCategory").field(&self.name()).finish()
    }
}

impl GlibPtrDefault for DebugCategory {
    type GlibType = *mut ffi::GstDebugCategory;
}

impl FromGlibPtrNone<*mut ffi::GstDebugCategory> for DebugCategory {
    unsafe fn from_glib_none(ptr: *mut ffi::GstDebugCategory) -> Self {
        assert!(!ptr.is_null());
        DebugCategory(Some(ptr::NonNull::new_unchecked(ptr)))
    }
}

impl FromGlibPtrFull<*mut ffi::GstDebugCategory> for DebugCategory {
    unsafe fn from_glib_full(ptr: *mut ffi::GstDebugCategory) -> Self {
        assert!(!ptr.is_null());
        DebugCategory(Some(ptr::NonNull::new_unchecked(ptr)))
    }
}

pub static CAT_RUST: Lazy<DebugCategory> = Lazy::new(|| {
    DebugCategory::new(
        "GST_RUST",
        crate::DebugColorFlags::UNDERLINE,
        Some("GStreamer's Rust binding core"),
    )
});

macro_rules! declare_debug_category_from_name(
    ($cat:ident, $cat_name:expr) => (
        pub static $cat: Lazy<DebugCategory> = Lazy::new(|| DebugCategory::get($cat_name)
            .expect(&format!("Unable to find `DebugCategory` with name {}", $cat_name)));
    );
);

declare_debug_category_from_name!(CAT_DEFAULT, "default");
declare_debug_category_from_name!(CAT_GST_INIT, "GST_INIT");
declare_debug_category_from_name!(CAT_MEMORY, "GST_MEMORY");
declare_debug_category_from_name!(CAT_PARENTAGE, "GST_PARENTAGE");
declare_debug_category_from_name!(CAT_STATES, "GST_STATES");
declare_debug_category_from_name!(CAT_SCHEDULING, "GST_SCHEDULING");
declare_debug_category_from_name!(CAT_BUFFER, "GST_BUFFER");
declare_debug_category_from_name!(CAT_BUFFER_LIST, "GST_BUFFER_LIST");
declare_debug_category_from_name!(CAT_BUS, "GST_BUS");
declare_debug_category_from_name!(CAT_CAPS, "GST_CAPS");
declare_debug_category_from_name!(CAT_CLOCK, "GST_CLOCK");
declare_debug_category_from_name!(CAT_ELEMENT_PADS, "GST_ELEMENT_PADS");
declare_debug_category_from_name!(CAT_PADS, "GST_PADS");
declare_debug_category_from_name!(CAT_PERFORMANCE, "GST_PERFORMANCE");
declare_debug_category_from_name!(CAT_PIPELINE, "GST_PIPELINE");
declare_debug_category_from_name!(CAT_PLUGIN_LOADING, "GST_PLUGIN_LOADING");
declare_debug_category_from_name!(CAT_PLUGIN_INFO, "GST_PLUGIN_INFO");
declare_debug_category_from_name!(CAT_PROPERTIES, "GST_PROPERTIES");
declare_debug_category_from_name!(CAT_NEGOTIATION, "GST_NEGOTIATION");
declare_debug_category_from_name!(CAT_REFCOUNTING, "GST_REFCOUNTING");
declare_debug_category_from_name!(CAT_ERROR_SYSTEM, "GST_ERROR_SYSTEM");
declare_debug_category_from_name!(CAT_EVENT, "GST_EVENT");
declare_debug_category_from_name!(CAT_MESSAGE, "GST_MESSAGE");
declare_debug_category_from_name!(CAT_PARAMS, "GST_PARAMS");
declare_debug_category_from_name!(CAT_CALL_TRACE, "GST_CALL_TRACE");
declare_debug_category_from_name!(CAT_SIGNAL, "GST_SIGNAL");
declare_debug_category_from_name!(CAT_PROBE, "GST_PROBE");
declare_debug_category_from_name!(CAT_REGISTRY, "GST_REGISTRY");
declare_debug_category_from_name!(CAT_QOS, "GST_QOS");
declare_debug_category_from_name!(CAT_META, "GST_META");
declare_debug_category_from_name!(CAT_LOCKING, "GST_LOCKING");
declare_debug_category_from_name!(CAT_CONTEXT, "GST_CONTEXT");

#[macro_export]
macro_rules! error(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Error, obj: $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Error, imp: $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Error, id: $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Error, $($args)*)
    }};
);

#[macro_export]
macro_rules! warning(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Warning, obj: $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Warning, imp: $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Warning, id: $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Warning, $($args)*)
    }};
);

#[macro_export]
macro_rules! fixme(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Fixme, obj: $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Fixme, imp: $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Fixme, id: $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Fixme, $($args)*)
    }};
);

#[macro_export]
macro_rules! info(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Info, obj: $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Info, imp: $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Info, id: $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Info, $($args)*)
    }};
);

#[macro_export]
macro_rules! debug(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Debug, obj: $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Debug, imp: $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Debug, id: $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Debug, $($args)*)
    }};
);

#[macro_export]
macro_rules! log(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Log, obj: $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Log, imp: $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Log, id: $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Log, $($args)*)
    }};
);

#[macro_export]
macro_rules! trace(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Trace, obj: $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Trace, imp: $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Trace, id: $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Trace, $($args)*)
    }};
);

#[macro_export]
macro_rules! memdump(
    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Memdump, obj: $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Memdump, imp: $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Memdump, id: $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat.clone(), level: $crate::DebugLevel::Memdump, $($args)*)
    }};
);

#[macro_export]
macro_rules! log_with_level(
    ($cat:expr, level: $level:expr, obj: $obj:expr, $($args:tt)*) => { {
        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if $level <= $cat.threshold() {
            use $crate::glib::Cast;

            #[allow(unused_unsafe)]
            let obj = unsafe { $obj.unsafe_cast_ref::<$crate::glib::Object>() };
            $crate::DebugCategory::log_unfiltered(
                $cat.clone(),
                Some(obj),
                $level,
                unsafe { $crate::glib::GStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                unsafe { $crate::glib::GStr::from_bytes_with_nul_unchecked(concat!(module_path!(), "\0").as_bytes()) },
                line!(),
                format_args!($($args)*),
            )
        }
    }};
    ($cat:expr, level: $level:expr, imp: $imp:expr, $($args:tt)*) => { {
        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if $level <= $cat.threshold() {
            use $crate::glib::Cast;

            let obj = $imp.obj();
            #[allow(unused_unsafe)]
            let obj = unsafe { obj.unsafe_cast_ref::<$crate::glib::Object>() };
            $crate::DebugCategory::log_unfiltered(
                $cat.clone(),
                Some(obj),
                $level,
                unsafe { $crate::glib::GStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                unsafe { $crate::glib::GStr::from_bytes_with_nul_unchecked(concat!(module_path!(), "\0").as_bytes()) },
                line!(),
                format_args!($($args)*),
            )
        }
    }};
    ($cat:expr, level: $level:expr, id: $id:expr, $($args:tt)*) => { {
        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if $level <= $cat.threshold() {
            $crate::DebugCategory::log_id_unfiltered(
                $cat.clone(),
                Some($id),
                $level,
                unsafe { $crate::glib::GStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                unsafe { $crate::glib::GStr::from_bytes_with_nul_unchecked(concat!(module_path!(), "\0").as_bytes()) },
                line!(),
                format_args!($($args)*),
            )
        }
    }};
    ($cat:expr, level: $level:expr, $($args:tt)*) => { {
        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if $level <= $cat.threshold() {
            $crate::DebugCategory::log_unfiltered(
                $cat.clone(),
                None as Option<&$crate::glib::Object>,
                $level,
                unsafe { $crate::glib::GStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                unsafe { $crate::glib::GStr::from_bytes_with_nul_unchecked(concat!(module_path!(), "\0").as_bytes()) },
                line!(),
                format_args!($($args)*),
            )
        }
    }};
);

unsafe extern "C" fn log_handler<T>(
    category: *mut ffi::GstDebugCategory,
    level: ffi::GstDebugLevel,
    file: *const c_char,
    function: *const c_char,
    line: i32,
    object: *mut glib::gobject_ffi::GObject,
    message: *mut ffi::GstDebugMessage,
    user_data: gpointer,
) where
    T: Fn(DebugCategory, DebugLevel, &str, &str, u32, Option<&LoggedObject>, &DebugMessage)
        + Send
        + Sync
        + 'static,
{
    if category.is_null() {
        return;
    }
    let category = DebugCategory(Some(ptr::NonNull::new_unchecked(category)));
    let level = from_glib(level);
    let file = CStr::from_ptr(file).to_string_lossy();
    let function = CStr::from_ptr(function).to_string_lossy();
    let line = line as u32;
    let object = ptr::NonNull::new(object).map(LoggedObject);
    let message = DebugMessage(ptr::NonNull::new_unchecked(message));
    let handler = &*(user_data as *mut T);
    (handler)(
        category,
        level,
        &file,
        &function,
        line,
        object.as_ref(),
        &message,
    );
}

unsafe extern "C" fn log_handler_data_free<T>(data: gpointer) {
    let data = Box::from_raw(data as *mut T);
    drop(data);
}

#[derive(Debug)]
pub struct DebugLogFunction(ptr::NonNull<std::os::raw::c_void>);

// The contained pointer is never dereferenced and has no thread affinity.
// It may be convenient to send it or share it between threads to allow cleaning
// up log functions from other threads than the one that created it.
unsafe impl Send for DebugLogFunction {}
unsafe impl Sync for DebugLogFunction {}

#[derive(Debug)]
#[doc(alias = "GObject")]
pub struct LoggedObject(ptr::NonNull<glib::gobject_ffi::GObject>);

impl LoggedObject {
    pub fn as_ptr(&self) -> *mut glib::gobject_ffi::GObject {
        self.0.as_ptr()
    }
}

impl fmt::Display for LoggedObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let ptr = self.0.as_ptr();
            let g_type_instance = &mut (*ptr).g_type_instance;
            if glib::gobject_ffi::g_type_check_instance_is_fundamentally_a(
                g_type_instance,
                glib::gobject_ffi::g_object_get_type(),
            ) != glib::ffi::GFALSE
            {
                let type_ = (*g_type_instance.g_class).g_type;

                if glib::gobject_ffi::g_type_is_a(type_, ffi::gst_pad_get_type())
                    != glib::ffi::GFALSE
                {
                    let name_ptr = (*(ptr as *mut ffi::GstObject)).name;
                    let name = if name_ptr.is_null() {
                        "<null>"
                    } else {
                        CStr::from_ptr(name_ptr)
                            .to_str()
                            .unwrap_or("<invalid name>")
                    };

                    let parent_ptr = (*(ptr as *mut ffi::GstObject)).parent;
                    let parent_name = if parent_ptr.is_null() {
                        "<null>"
                    } else {
                        let name_ptr = (*(parent_ptr as *mut ffi::GstObject)).name;
                        if name_ptr.is_null() {
                            "<null>"
                        } else {
                            CStr::from_ptr(name_ptr)
                                .to_str()
                                .unwrap_or("<invalid name>")
                        }
                    };

                    write!(f, "{}:{}", parent_name, name)
                } else if glib::gobject_ffi::g_type_is_a(type_, ffi::gst_object_get_type())
                    != glib::ffi::GFALSE
                {
                    let name_ptr = (*(ptr as *mut ffi::GstObject)).name;
                    let name = if name_ptr.is_null() {
                        "<null>"
                    } else {
                        CStr::from_ptr(name_ptr)
                            .to_str()
                            .unwrap_or("<invalid name>")
                    };
                    write!(f, "{}", name)
                } else {
                    let type_name = CStr::from_ptr(glib::gobject_ffi::g_type_name(type_));
                    write!(
                        f,
                        "{}:{:?}",
                        type_name.to_str().unwrap_or("<invalid type>"),
                        ptr
                    )
                }
            } else {
                write!(f, "{:?}", ptr)
            }
        }
    }
}

#[doc(alias = "gst_debug_add_log_function")]
pub fn debug_add_log_function<T>(function: T) -> DebugLogFunction
where
    T: Fn(DebugCategory, DebugLevel, &str, &str, u32, Option<&LoggedObject>, &DebugMessage)
        + Send
        + Sync
        + 'static,
{
    skip_assert_initialized!();
    unsafe {
        let user_data = Box::new(function);
        let user_data_ptr = Box::into_raw(user_data) as gpointer;
        ffi::gst_debug_add_log_function(
            Some(log_handler::<T>),
            user_data_ptr,
            Some(log_handler_data_free::<T>),
        );
        DebugLogFunction(ptr::NonNull::new_unchecked(user_data_ptr))
    }
}

pub fn debug_remove_default_log_function() {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_remove_log_function(None);
    }
}

#[doc(alias = "gst_debug_remove_log_function_by_data")]
pub fn debug_remove_log_function(log_fn: DebugLogFunction) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_remove_log_function_by_data(log_fn.0.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};

    #[test]
    #[doc(alias = "get_existing")]
    fn existing() {
        crate::init().unwrap();

        let perf_cat = DebugCategory::get("GST_PERFORMANCE")
            .expect("Unable to find `DebugCategory` with name \"GST_PERFORMANCE\"");
        assert_eq!(perf_cat.name(), CAT_PERFORMANCE.name());
    }

    #[test]
    fn all() {
        crate::init().unwrap();

        assert!(DebugCategory::all_categories().any(|c| c.name() == "GST_PERFORMANCE"));
    }

    #[test]
    fn new_and_log() {
        crate::init().unwrap();

        let cat = DebugCategory::new(
            "test-cat",
            crate::DebugColorFlags::empty(),
            Some("some debug category"),
        );

        error!(cat, "meh");
        warning!(cat, "meh");
        fixme!(cat, "meh");
        info!(cat, "meh");
        debug!(cat, "meh");
        log!(cat, "meh");
        trace!(cat, "meh");
        memdump!(cat, "meh");

        let obj = crate::Bin::new(Some("meh"));

        error!(cat, obj: &obj, "meh");
        warning!(cat, obj: &obj, "meh");
        fixme!(cat, obj: &obj, "meh");
        info!(cat, obj: &obj, "meh");
        debug!(cat, obj: &obj, "meh");
        log!(cat, obj: &obj, "meh");
        trace!(cat, obj: &obj, "meh");
        memdump!(cat, obj: &obj, "meh");

        error!(cat, obj: obj, "meh");
        warning!(cat, obj: obj, "meh");
        fixme!(cat, obj: obj, "meh");
        info!(cat, obj: obj, "meh");
        debug!(cat, obj: obj, "meh");
        log!(cat, obj: obj, "meh");
        trace!(cat, obj: obj, "meh");
        memdump!(cat, obj: obj, "meh");
    }

    #[test]
    fn log_handler() {
        crate::init().unwrap();

        let cat = DebugCategory::new(
            "test-cat-log",
            crate::DebugColorFlags::empty(),
            Some("some debug category"),
        );
        cat.set_threshold(DebugLevel::Info);
        let obj = crate::Bin::new(Some("meh"));

        let (sender, receiver) = mpsc::channel();

        let sender = Arc::new(Mutex::new(sender));

        let handler = move |category: DebugCategory,
                            level: DebugLevel,
                            _file: &str,
                            _function: &str,
                            _line: u32,
                            _object: Option<&LoggedObject>,
                            message: &DebugMessage| {
            let cat = DebugCategory::get("test-cat-log").unwrap();

            if category != cat {
                // This test can run in parallel with other tests, including new_and_log above.
                // We cannot be certain we only see our own messages.
                return;
            }

            assert_eq!(level, DebugLevel::Info);
            assert_eq!(&message.get().unwrap(), "meh");
            let _ = sender.lock().unwrap().send(());
        };

        debug_remove_default_log_function();
        let log_fn = debug_add_log_function(handler);
        info!(cat, obj: &obj, "meh");

        receiver.recv().unwrap();

        debug_remove_log_function(log_fn);

        info!(cat, obj: &obj, "meh2");
        assert!(receiver.recv().is_err());
    }

    #[test]
    fn no_argument_evaluation() {
        crate::init().unwrap();

        let cat = DebugCategory::new(
            "no_argument_evaluation",
            crate::DebugColorFlags::empty(),
            Some("No Argument Evaluation debug category"),
        );

        let mut arg_evaluated = false;
        trace!(cat, "{}", {
            arg_evaluated = true;
            "trace log"
        });

        assert!(!arg_evaluated);
    }

    #[cfg(feature = "v1_22")]
    #[test]
    fn id_logging() {
        crate::init().unwrap();

        let cat = DebugCategory::new(
            "log_with_id_test_category",
            crate::DebugColorFlags::empty(),
            Some("Blablabla"),
        );

        trace!(cat, id: "123", "test");
        trace!(cat, id: &String::from("123"), "test");
    }
}
