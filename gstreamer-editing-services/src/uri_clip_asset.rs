// Take a look at the license at the top of the repository in the LICENSE file.
use crate::{ffi, UriClipAsset};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, pin::Pin};

impl UriClipAsset {
    #[doc(alias = "ges_uri_clip_asset_new")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<P: FnOnce(Result<UriClipAsset, glib::Error>) + 'static>(
        uri: &str,
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
        unsafe extern "C" fn new_trampoline<
            P: FnOnce(Result<UriClipAsset, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = {
                #[cfg(feature = "v1_16")]
                {
                    ffi::ges_uri_clip_asset_finish(res, &mut error)
                }
                #[cfg(not(feature = "v1_16"))]
                {
                    ffi::ges_asset_request_finish(res, &mut error) as *mut ffi::GESUriClipAsset
                }
            };
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
        let callback = new_trampoline::<P>;
        unsafe {
            ffi::ges_uri_clip_asset_new(
                uri.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_future(
        uri: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UriClipAsset, glib::Error>> + 'static>>
    {
        skip_assert_initialized!();
        let uri = String::from(uri);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
            Self::new(&uri, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }
}
