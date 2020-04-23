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

use gst_sys;

impl ProxyPad {
    pub fn chain_default<O: IsA<ProxyPad>, P: IsA<Object>>(
        pad: &O,
        parent: Option<&P>,
        buffer: Buffer,
    ) -> Result<FlowSuccess, FlowError> {
        skip_assert_initialized!();
        let ret: FlowReturn = unsafe {
            from_glib(gst_sys::gst_proxy_pad_chain_default(
                pad.as_ptr() as *mut gst_sys::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                buffer.into_ptr(),
            ))
        };
        ret.into_result()
    }

    pub fn chain_list_default<O: IsA<ProxyPad>, P: IsA<Object>>(
        pad: &O,
        parent: Option<&P>,
        list: BufferList,
    ) -> Result<FlowSuccess, FlowError> {
        skip_assert_initialized!();
        let ret: FlowReturn = unsafe {
            from_glib(gst_sys::gst_proxy_pad_chain_list_default(
                pad.as_ptr() as *mut gst_sys::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                list.into_ptr(),
            ))
        };
        ret.into_result()
    }

    pub fn getrange_default<O: IsA<ProxyPad>, P: IsA<Object>>(
        pad: &O,
        parent: Option<&P>,
        offset: u64,
        size: u32,
    ) -> Result<Buffer, FlowError> {
        skip_assert_initialized!();
        unsafe {
            let mut buffer = ptr::null_mut();
            let ret: FlowReturn = from_glib(gst_sys::gst_proxy_pad_getrange_default(
                pad.as_ptr() as *mut gst_sys::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ));
            ret.into_result_value(|| from_glib_full(buffer))
        }
    }

    pub fn iterate_internal_links_default<O: IsA<ProxyPad>, P: IsA<Object>>(
        pad: &O,
        parent: Option<&P>,
    ) -> Option<::Iterator<Pad>> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gst_sys::gst_proxy_pad_iterate_internal_links_default(
                pad.as_ptr() as *mut gst_sys::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}
