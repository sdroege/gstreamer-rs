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
use FlowSuccess;
use Object;
use Pad;
use ProxyPad;

use glib::object::IsA;
use glib::translate::{from_glib, from_glib_full, ToGlibPtr};

use ffi;

impl ProxyPad {
    pub fn chain_default<'a, P: IsA<ProxyPad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(
        pad: &P,
        parent: R,
        buffer: Buffer,
    ) -> Result<FlowSuccess, FlowError> {
        skip_assert_initialized!();
        let parent = parent.into();
        let ret: FlowReturn = unsafe {
            from_glib(ffi::gst_proxy_pad_chain_default(
                pad.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                buffer.into_ptr(),
            ))
        };
        ret.into_result()
    }

    pub fn chain_list_default<'a, P: IsA<ProxyPad>, Q: IsA<Object> + 'a, R: Into<Option<&'a Q>>>(
        pad: &P,
        parent: R,
        list: BufferList,
    ) -> Result<FlowSuccess, FlowError> {
        skip_assert_initialized!();
        let parent = parent.into();
        let ret: FlowReturn = unsafe {
            from_glib(ffi::gst_proxy_pad_chain_list_default(
                pad.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                list.into_ptr(),
            ))
        };
        ret.into_result()
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
                pad.as_ptr() as *mut ffi::GstPad,
                parent.as_ref().to_glib_none().0,
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
        unsafe {
            from_glib_full(ffi::gst_proxy_pad_iterate_internal_links_default(
                pad.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}
