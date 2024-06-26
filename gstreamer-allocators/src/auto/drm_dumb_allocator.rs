// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstDRMDumbAllocator")]
    pub struct DRMDumbAllocator(Object<ffi::GstDRMDumbAllocator, ffi::GstDRMDumbAllocatorClass>) @extends gst::Allocator;

    match fn {
        type_ => || ffi::gst_drm_dumb_allocator_get_type(),
    }
}

impl DRMDumbAllocator {
    #[doc(alias = "gst_drm_dumb_allocator_new_with_device_path")]
    #[doc(alias = "new_with_device_path")]
    pub fn with_device_path(
        drm_device_path: impl AsRef<std::path::Path>,
    ) -> Result<DRMDumbAllocator, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<gst::Allocator>::from_glib_full(
                ffi::gst_drm_dumb_allocator_new_with_device_path(
                    drm_device_path.as_ref().to_glib_none().0,
                ),
            )
            .map(|o| o.unsafe_cast())
            .ok_or_else(|| glib::bool_error!("Failed to create allocator"))
        }
    }

    #[doc(alias = "gst_drm_dumb_allocator_has_prime_export")]
    pub fn has_prime_export(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_drm_dumb_allocator_has_prime_export(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "drm-device-path")]
    pub fn drm_device_path(&self) -> Option<std::path::PathBuf> {
        ObjectExt::property(self, "drm-device-path")
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "drm-fd")]
    pub fn drm_fd(&self) -> i32 {
        ObjectExt::property(self, "drm-fd")
    }
}

unsafe impl Send for DRMDumbAllocator {}
unsafe impl Sync for DRMDumbAllocator {}
