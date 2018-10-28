// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use Buffer;
use BufferList;
use FlowError;
use FlowReturn;
use Object;
use Pad;
use ProxyPad;

use glib::translate::{from_glib, from_glib_full, ToGlibPtr};
use glib::IsA;

use ffi;

impl ProxyPad {
    pub fn chain_default<'a, P: IsA<ProxyPad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(
        pad: &P,
        parent: R,
        buffer: Buffer,
    ) -> FlowReturn {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib(ffi::gst_proxy_pad_chain_default(
                pad.to_glib_none().0 as *mut ffi::GstPad,
                parent.0,
                buffer.into_ptr(),
            ))
        }
    }

    pub fn chain_list_default<'a, P: IsA<ProxyPad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(
        pad: &P,
        parent: R,
        list: BufferList,
    ) -> FlowReturn {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib(ffi::gst_proxy_pad_chain_list_default(
                pad.to_glib_none().0 as *mut ffi::GstPad,
                parent.0,
                list.into_ptr(),
            ))
        }
    }

    pub fn getrange_default<P: IsA<ProxyPad>, Q: IsA<Object>>(
        pad: &P,
        parent: &Q,
        offset: u64,
        size: u32,
    ) -> Result<Buffer, FlowError> {
        skip_assert_initialized!();
        unsafe {
            let mut buffer = ptr::null_mut();
            let ret: FlowReturn = from_glib(ffi::gst_proxy_pad_getrange_default(
                pad.to_glib_none().0 as *mut ffi::GstPad,
                parent.to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ));
            ret.into_result_value(|| from_glib_full(buffer))
        }
    }

    pub fn iterate_internal_links_default<
        'a,
        P: IsA<ProxyPad>,
        Q: IsA<Object> + 'a,
        R: Into<Option<&'a Q>>,
    >(
        pad: &P,
        parent: R,
    ) -> Option<::Iterator<Pad>> {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_proxy_pad_iterate_internal_links_default(
                pad.to_glib_none().0 as *mut ffi::GstPad,
                parent.0,
            ))
        }
    }
}
