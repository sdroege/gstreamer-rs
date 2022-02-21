// Take a look at the license at the top of the repository in the LICENSE file.

pub const MAJOR_VERSION: i32 = 1;

#[cfg(not(feature = "v1_10"))]
pub const MINOR_VERSION: i32 = 8;
#[cfg(all(feature = "v1_10", not(feature = "v1_12")))]
pub const MINOR_VERSION: i32 = 10;
#[cfg(feature = "v1_12")]
pub const MINOR_VERSION: i32 = 12;

#[macro_export]
macro_rules! plugin_define(
    ($name:ident, $description:expr, $plugin_init:ident,
     $version:expr, $license:expr, $source:expr,
     $package:expr, $origin:expr $(, $release_datetime:expr)?) => {
        pub mod plugin_desc {
            #[repr(transparent)]
            pub struct GstPluginDesc($crate::ffi::GstPluginDesc);
            unsafe impl Send for GstPluginDesc {}
            unsafe impl Sync for GstPluginDesc {}

            #[no_mangle]
            #[allow(non_upper_case_globals)]
            pub static gst_plugin_desc: GstPluginDesc = GstPluginDesc($crate::ffi::GstPluginDesc {
                major_version: $crate::subclass::MAJOR_VERSION,
                minor_version: $crate::subclass::MINOR_VERSION,
                name: concat!(stringify!($name), "\0") as *const str as *const _,
                description: concat!($description, "\0") as *const str as *const _,
                plugin_init: Some(plugin_init_trampoline),
                version: concat!($version, "\0") as *const str as *const _,
                license: concat!($license, "\0") as *const str as *const _,
                source: concat!($source, "\0") as *const str as *const _,
                package: concat!($package, "\0") as *const str as *const _,
                origin: concat!($origin, "\0") as *const str as *const _,
                release_datetime: {
                    // NB: if this looks a lot like `Option`, it is not a coincidence. Alas,
                    // Option::or is not `const` and neither is `unwrap_or` so we have to roll our
                    // own oli-obk-ified enum instead.
                    enum OptionalPtr<T>{
                        Null,
                        Some(*const T),
                    }
                    impl<T: Sized> OptionalPtr<T> {
                        const fn with(self, value: *const T) -> Self {
                            Self::Some(value)
                        }
                        const fn ptr(self) -> *const T {
                            match self {
                                Self::Null => std::ptr::null(),
                                Self::Some(ptr) => ptr
                            }
                        }
                    }
                    OptionalPtr::Null
                      $(.with(concat!($release_datetime, "\0").as_ptr() as _))?
                        .ptr()
                },
                _gst_reserved: [0 as $crate::glib::ffi::gpointer; 4],
            });

            pub fn plugin_register_static() -> Result<(), $crate::glib::BoolError> {
                unsafe {
                    $crate::glib::result_from_gboolean!(
                        $crate::ffi::gst_plugin_register_static(
                            $crate::subclass::MAJOR_VERSION,
                            $crate::subclass::MINOR_VERSION,
                            concat!(stringify!($name), "\0") as *const str as *const _,
                            concat!($description, "\0") as *const str as _,
                            Some(plugin_init_trampoline),
                            concat!($version, "\0") as *const str as *const _,
                            concat!($license, "\0") as *const str as *const _,
                            concat!($source, "\0") as *const str as *const _,
                            concat!($package, "\0") as *const str as *const _,
                            concat!($origin, "\0") as *const str as *const _,
                        ),
                        "Failed to register the plugin"
                    )
                }
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe extern "C" fn plugin_init_trampoline(plugin: *mut $crate::ffi::GstPlugin) -> $crate::glib::ffi::gboolean {
                use std::panic::{self, AssertUnwindSafe};

                let panic_result = panic::catch_unwind(AssertUnwindSafe(|| super::$plugin_init(&$crate::glib::translate::from_glib_borrow(plugin))));
                match panic_result {
                    Ok(register_result) => match register_result {
                        Ok(_) => $crate::glib::ffi::GTRUE,
                        Err(err) => {
                            let cat = $crate::DebugCategory::get("GST_PLUGIN_LOADING").unwrap();
                            $crate::error!(cat, "Failed to register plugin: {}", err);
                            $crate::glib::ffi::GFALSE
                        }
                    }
                    Err(err) => {
                        let cat = $crate::DebugCategory::get("GST_PLUGIN_LOADING").unwrap();
                        if let Some(cause) = err.downcast_ref::<&str>() {
                            $crate::error!(cat, "Failed to initialize plugin due to panic: {}", cause);
                        } else if let Some(cause) = err.downcast_ref::<String>() {
                            $crate::error!(cat, "Failed to initialize plugin due to panic: {}", cause);
                        } else {
                            $crate::error!(cat, "Failed to initialize plugin due to panic");
                        }

                        $crate::glib::ffi::GFALSE
                    }
                }
            }
        }
        pub use self::plugin_desc::plugin_register_static;
    };
);
