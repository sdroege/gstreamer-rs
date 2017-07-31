// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use Pad;
use ProxyPad;
use Object;
use FlowReturn;
use Buffer;

use glib::IsA;
use glib::translate::{from_glib, from_glib_full, ToGlibPtr};

use ffi;

impl ProxyPad {
    pub fn chain_default<'a, P: IsA<Pad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(
        pad: &P,
        parent: R,
        buffer: Buffer,
    ) -> FlowReturn {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib(ffi::gst_proxy_pad_chain_default(
                pad.to_glib_none().0,
                parent.0,
                buffer.into_ptr(),
            ))
        }
    }

    pub fn getrange_default<P: IsA<Pad>, Q: IsA<Object>>(
        pad: &P,
        parent: &Q,
        offset: u64,
        size: u32,
    ) -> Result<Buffer, FlowReturn> {
        skip_assert_initialized!();
        unsafe {
            let mut buffer = ptr::null_mut();
            let ret = from_glib(ffi::gst_proxy_pad_getrange_default(
                pad.to_glib_none().0,
                parent.to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ));
            if ret == FlowReturn::Ok {
                Ok(from_glib_full(buffer))
            } else {
                Err(ret)
            }
        }
    }
}
