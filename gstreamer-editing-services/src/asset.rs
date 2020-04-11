// Copyright (C) 2020 Guillaume Gomez <guillaume1.gomez@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gio;
use glib;
use glib::prelude::*;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::pin::Pin;
use std::ptr;
use Asset;

impl Asset {
    pub fn request_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Asset, glib::Error>) + Send + 'static,
    >(
        extractable_type: glib::types::Type,
        id: &str,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        assert_initialized_main_thread!();
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn request_async_trampoline<
            Q: FnOnce(Result<Asset, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ges_sys::ges_asset_request_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = request_async_trampoline::<Q>;
        unsafe {
            ges_sys::ges_asset_request_async(
                extractable_type.to_glib(),
                id.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn request_async_future(
        extractable_type: glib::types::Type,
        id: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Asset, glib::Error>> + 'static>> {
        skip_assert_initialized!();
        let id = String::from(id);
        Box_::pin(gio::GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            Self::request_async(extractable_type, &id, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}
