// Take a look at the license at the top of the repository in the LICENSE file.

pub const MAJOR_VERSION: i32 = 1;

cfg_if::cfg_if! {
    if #[cfg(feature = "v1_20")] {
        pub const MINOR_VERSION: i32 = 20;
    } else if #[cfg(feature = "v1_18")] {
        pub const MINOR_VERSION: i32 = 18;
    } else if #[cfg(feature = "v1_16")] {
        pub const MINOR_VERSION: i32 = 16;
    } else if #[cfg(feature = "v1_14")] {
        pub const MINOR_VERSION: i32 = 14;
    }
}

#[macro_export]
macro_rules! plugin_define(
    ($name:ident, $description:expr, $plugin_init:ident,
     $version:expr, $license:expr, $source:expr,
     $package:expr, $origin:expr, $release_datetime:expr) => {
        pub mod plugin_desc {
            use $crate::glib::translate::{from_glib_borrow, IntoGlib, from_glib};

            #[repr(transparent)]
            pub struct GstPluginDesc($crate::ffi::GstPluginDesc);
            unsafe impl Send for GstPluginDesc {}
            unsafe impl Sync for GstPluginDesc {}

            static GST_PLUGIN_DESC: GstPluginDesc = GstPluginDesc($crate::ffi::GstPluginDesc {
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
                release_datetime: concat!($release_datetime, "\0") as *const str as *const _,
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

            $crate::paste::item! {
                #[no_mangle]
                #[allow(clippy::missing_safety_doc)]
                pub unsafe extern "C" fn [<gst_plugin_ $name _register>] () {
                    let _ = plugin_register_static();
                }

                #[no_mangle]
                #[allow(clippy::missing_safety_doc)]
                pub unsafe extern "C" fn [<gst_plugin_ $name _get_desc>] () -> *const $crate::ffi::GstPluginDesc {
                    &GST_PLUGIN_DESC.0
                }
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe extern "C" fn plugin_init_trampoline(plugin: *mut $crate::ffi::GstPlugin) -> $crate::glib::ffi::gboolean {
                use std::panic::{self, AssertUnwindSafe};

                let panic_result = panic::catch_unwind(AssertUnwindSafe(|| super::$plugin_init(&from_glib_borrow(plugin))));
                match panic_result {
                    Ok(register_result) => match register_result {
                        Ok(_) => $crate::glib::ffi::GTRUE,
                        Err(err) => {
                            let cat = $crate::DebugCategory::get("GST_PLUGIN_LOADING").unwrap();
                            $crate::gst_error!(cat, "Failed to register plugin: {}", err);
                            $crate::glib::ffi::GFALSE
                        }
                    }
                    Err(err) => {
                        let cat = $crate::DebugCategory::get("GST_PLUGIN_LOADING").unwrap();
                        if let Some(cause) = err.downcast_ref::<&str>() {
                            $crate::gst_error!(cat, "Failed to initialize plugin due to panic: {}", cause);
                        } else if let Some(cause) = err.downcast_ref::<String>() {
                            $crate::gst_error!(cat, "Failed to initialize plugin due to panic: {}", cause);
                        } else {
                            $crate::gst_error!(cat, "Failed to initialize plugin due to panic");
                        }

                        $crate::glib::ffi::GFALSE
                    }
                }
            }
        }
        pub use self::plugin_desc::plugin_register_static;
    };
);
