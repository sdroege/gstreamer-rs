// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GESDiscovererManager")]
    pub struct DiscovererManager(Object<ffi::GESDiscovererManager, ffi::GESDiscovererManagerClass>);

    match fn {
        type_ => || ffi::ges_discoverer_manager_get_type(),
    }
}

impl DiscovererManager {
    #[doc(alias = "ges_discoverer_manager_get_timeout")]
    #[doc(alias = "get_timeout")]
    pub fn timeout(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_discoverer_manager_get_timeout(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_discoverer_manager_get_use_cache")]
    #[doc(alias = "get_use_cache")]
    #[doc(alias = "use-cache")]
    pub fn uses_cache(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_discoverer_manager_get_use_cache(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_discoverer_manager_set_timeout")]
    #[doc(alias = "timeout")]
    pub fn set_timeout(&self, timeout: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::ges_discoverer_manager_set_timeout(
                self.to_glib_none().0,
                timeout.into().into_glib(),
            );
        }
    }

    #[doc(alias = "ges_discoverer_manager_set_use_cache")]
    #[doc(alias = "use-cache")]
    pub fn set_use_cache(&self, use_cache: bool) {
        unsafe {
            ffi::ges_discoverer_manager_set_use_cache(self.to_glib_none().0, use_cache.into_glib());
        }
    }

    #[cfg(not(feature = "v1_24"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "v1_24"))))]
    #[doc(alias = "use-cache")]
    pub fn uses_cache(&self) -> bool {
        ObjectExt::property(self, "use-cache")
    }

    #[cfg(not(feature = "v1_24"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "v1_24"))))]
    #[doc(alias = "use-cache")]
    pub fn set_use_cache(&self, use_cache: bool) {
        ObjectExt::set_property(self, "use-cache", use_cache)
    }

    #[doc(alias = "ges_discoverer_manager_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> DiscovererManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::ges_discoverer_manager_get_default()) }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "discovered")]
    pub fn connect_discovered<
        F: Fn(&Self, &gst_pbutils::DiscovererInfo, Option<&glib::Error>) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn discovered_trampoline<
            F: Fn(&DiscovererManager, &gst_pbutils::DiscovererInfo, Option<&glib::Error>) + 'static,
        >(
            this: *mut ffi::GESDiscovererManager,
            info: *mut gst_pbutils::ffi::GstDiscovererInfo,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(info),
                Option::<glib::Error>::from_glib_borrow(error)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"discovered\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    discovered_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "load-serialized-info")]
    pub fn connect_load_serialized_info<
        F: Fn(&Self, &str) -> Option<gst_pbutils::DiscovererInfo> + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn load_serialized_info_trampoline<
            F: Fn(&DiscovererManager, &str) -> Option<gst_pbutils::DiscovererInfo> + 'static,
        >(
            this: *mut ffi::GESDiscovererManager,
            uri: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> *mut gst_pbutils::ffi::GstDiscovererInfo {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(uri),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"load-serialized-info\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    load_serialized_info_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "source-setup")]
    pub fn connect_source_setup<F: Fn(&Self, &gst::Element) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn source_setup_trampoline<
            F: Fn(&DiscovererManager, &gst::Element) + 'static,
        >(
            this: *mut ffi::GESDiscovererManager,
            source: *mut gst::ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(source))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"source-setup\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    source_setup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "timeout")]
    pub fn connect_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<F: Fn(&DiscovererManager) + 'static>(
            this: *mut ffi::GESDiscovererManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-cache")]
    pub fn connect_use_cache_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_cache_trampoline<F: Fn(&DiscovererManager) + 'static>(
            this: *mut ffi::GESDiscovererManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-cache\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_cache_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
