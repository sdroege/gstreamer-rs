// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, pin::Pin};

use glib::{prelude::*, translate::*};

use crate::{Asset, ffi};

impl Asset {
    // rustdoc-stripper-ignore-next
    /// Request an asset for a specific extractable type using generics.
    ///
    /// # Example
    /// ```ignore
    /// let asset = ges::Asset::request::<ges::TestClip>(None)?;
    /// ```
    #[doc(alias = "ges_asset_request")]
    pub fn request<T>(id: Option<&str>) -> Result<Asset, glib::Error>
    where
        T: StaticType + IsA<crate::Extractable>,
    {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_asset_request(
                T::static_type().into_glib(),
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

    // rustdoc-stripper-ignore-next
    /// Request an asset asynchronously for a specific extractable type using generics.
    ///
    /// # Example
    /// ```ignore
    /// ges::Asset::request_async::<ges::UriClip, _>(
    ///     Some("file:///path/to/file.mp4"),
    ///     None,
    ///     |result| {
    ///         match result {
    ///             Ok(asset) => println!("Asset loaded: {:?}", asset),
    ///             Err(err) => eprintln!("Failed to load asset: {}", err),
    ///         }
    ///     },
    /// );
    /// ```
    #[doc(alias = "ges_asset_request_async")]
    pub fn request_async<T, P>(
        id: Option<&str>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) where
        T: StaticType + IsA<crate::Extractable>,
        P: FnOnce(Result<Asset, glib::Error>) + 'static,
    {
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
            unsafe {
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
        }
        let callback = request_async_trampoline::<P>;
        unsafe {
            ffi::ges_asset_request_async(
                T::static_type().into_glib(),
                id.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    // rustdoc-stripper-ignore-next
    /// Request an asset as a future for a specific extractable type.
    ///
    /// # Example
    /// ```ignore
    /// let asset = ges::Asset::request_future::<ges::UriClip>(Some("file:///path/to/file.mp4")).await?;
    /// ```
    pub fn request_future<T>(
        id: Option<&str>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Asset, glib::Error>> + 'static>>
    where
        T: StaticType + IsA<crate::Extractable> + 'static,
    {
        skip_assert_initialized!();
        let id = id.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::request_async::<T, _>(
                id.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    // rustdoc-stripper-ignore-next
    /// Check if an asset needs to be reloaded for a specific extractable type.
    ///
    /// # Example
    /// ```ignore
    /// if ges::Asset::needs_reload::<ges::UriClip>(Some("file:///path/to/file.mp4")) {
    ///     println!("Asset needs reload");
    /// }
    /// ```
    #[doc(alias = "ges_asset_needs_reload")]
    pub fn needs_reload<T>(id: Option<&str>) -> bool
    where
        T: StaticType + IsA<crate::Extractable>,
    {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::ges_asset_needs_reload(
                T::static_type().into_glib(),
                id.to_glib_none().0,
            ))
        }
    }
}
