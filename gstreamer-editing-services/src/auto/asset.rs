// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Extractable, MetaContainer};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GESAsset")]
    pub struct Asset(Object<ffi::GESAsset, ffi::GESAssetClass>) @implements MetaContainer;

    match fn {
        type_ => || ffi::ges_asset_get_type(),
    }
}

impl Asset {
    pub const NONE: Option<&'static Asset> = None;

    #[doc(alias = "ges_asset_needs_reload")]
    pub fn needs_reload(extractable_type: glib::types::Type, id: Option<&str>) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::ges_asset_needs_reload(
                extractable_type.into_glib(),
                id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_asset_request")]
    pub fn request(
        extractable_type: glib::types::Type,
        id: Option<&str>,
    ) -> Result<Option<Asset>, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_asset_request(
                extractable_type.into_glib(),
                id.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_asset_request_async")]
    pub fn request_async<P: FnOnce(Result<Asset, glib::Error>) + 'static>(
        extractable_type: glib::types::Type,
        id: Option<&str>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        assert_initialized_main_thread!();

        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn request_async_trampoline<
            P: FnOnce(Result<Asset, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_asset_request_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = request_async_trampoline::<P>;
        unsafe {
            ffi::ges_asset_request_async(
                extractable_type.into_glib(),
                id.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn request_future(
        extractable_type: glib::types::Type,
        id: Option<&str>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Asset, glib::Error>> + 'static>> {
        skip_assert_initialized!();
        let id = id.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::request_async(
                extractable_type,
                id.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }
}

unsafe impl Send for Asset {}
unsafe impl Sync for Asset {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Asset>> Sealed for T {}
}

pub trait AssetExt: IsA<Asset> + sealed::Sealed + 'static {
    #[doc(alias = "ges_asset_extract")]
    fn extract(&self) -> Result<Extractable, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_asset_extract(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_asset_get_error")]
    #[doc(alias = "get_error")]
    fn error(&self) -> Option<glib::Error> {
        unsafe { from_glib_none(ffi::ges_asset_get_error(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "ges_asset_get_extractable_type")]
    #[doc(alias = "get_extractable_type")]
    #[doc(alias = "extractable-type")]
    fn extractable_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::ges_asset_get_extractable_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_asset_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::ges_asset_get_id(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "ges_asset_get_proxy")]
    #[doc(alias = "get_proxy")]
    #[must_use]
    fn proxy(&self) -> Option<Asset> {
        unsafe { from_glib_none(ffi::ges_asset_get_proxy(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "ges_asset_get_proxy_target")]
    #[doc(alias = "get_proxy_target")]
    #[doc(alias = "proxy-target")]
    #[must_use]
    fn proxy_target(&self) -> Option<Asset> {
        unsafe {
            from_glib_none(ffi::ges_asset_get_proxy_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_asset_list_proxies")]
    fn list_proxies(&self) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_asset_list_proxies(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_asset_set_proxy")]
    #[doc(alias = "proxy")]
    fn set_proxy(&self, proxy: Option<&impl IsA<Asset>>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_asset_set_proxy(
                    self.as_ref().to_glib_none().0,
                    proxy.map(|p| p.as_ref()).to_glib_none().0
                ),
                "Failed to set proxy"
            )
        }
    }

    #[doc(alias = "ges_asset_unproxy")]
    fn unproxy(&self, proxy: &impl IsA<Asset>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_asset_unproxy(
                    self.as_ref().to_glib_none().0,
                    proxy.as_ref().to_glib_none().0
                ),
                "Failed to unproxy asset"
            )
        }
    }

    #[doc(alias = "proxy")]
    fn connect_proxy_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_trampoline<
            P: IsA<Asset>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GESAsset,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Asset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proxy\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_proxy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "proxy-target")]
    fn connect_proxy_target_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_target_trampoline<
            P: IsA<Asset>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GESAsset,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Asset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::proxy-target\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_proxy_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Asset>> AssetExt for O {}
