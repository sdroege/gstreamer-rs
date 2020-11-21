// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub const MAJOR_VERSION: i32 = 1;

#[cfg(all(feature = "v1_14", not(feature = "v1_16")))]
pub const MINOR_VERSION: i32 = 14;
#[cfg(all(feature = "v1_16", not(feature = "v1_18")))]
pub const MINOR_VERSION: i32 = 16;
#[cfg(all(feature = "v1_18", not(feature = "v1_20")))]
pub const MINOR_VERSION: i32 = 18;

#[macro_export]
macro_rules! gst_plugin_define(
    ($name:ident, $description:expr, $plugin_init:ident,
     $version:expr, $license:expr, $source:expr,
     $package:expr, $origin:expr, $release_datetime:expr) => {
        pub mod plugin_desc {
            use $crate::glib::translate::{from_glib_borrow, ToGlib, from_glib};

            #[repr(transparent)]
            pub struct GstPluginDesc($crate::gst_sys::GstPluginDesc);
            unsafe impl Send for GstPluginDesc {}
            unsafe impl Sync for GstPluginDesc {}

            static GST_PLUGIN_DESC: GstPluginDesc = GstPluginDesc($crate::gst_sys::GstPluginDesc {
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
                _gst_reserved: [0 as $crate::glib_sys::gpointer; 4],
            });

            pub fn plugin_register_static() -> Result<(), $crate::glib::BoolError> {
                unsafe {
                    $crate::glib::glib_result_from_gboolean!(
                        $crate::gst_sys::gst_plugin_register_static(
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
                pub unsafe extern "C" fn [<gst_plugin_ $name _get_desc>] () -> *const $crate::gst_sys::GstPluginDesc {
                    &GST_PLUGIN_DESC.0
                }
            }

            #[allow(clippy::missing_safety_doc)]
            unsafe extern "C" fn plugin_init_trampoline(plugin: *mut $crate::gst_sys::GstPlugin) -> $crate::glib_sys::gboolean {
                use std::panic::{self, AssertUnwindSafe};

                let panic_result = panic::catch_unwind(AssertUnwindSafe(|| super::$plugin_init(&from_glib_borrow(plugin))));
                match panic_result {
                    Ok(register_result) => match register_result {
                        Ok(_) => $crate::glib_sys::GTRUE,
                        Err(err) => {
                            let cat = $crate::DebugCategory::get("GST_PLUGIN_LOADING").unwrap();
                            $crate::gst_error!(cat, "Failed to register plugin: {}", err);
                            $crate::glib_sys::GFALSE
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

                        $crate::glib_sys::GFALSE
                    }
                }
            }
        }
        pub use self::plugin_desc::plugin_register_static;
    };
);
