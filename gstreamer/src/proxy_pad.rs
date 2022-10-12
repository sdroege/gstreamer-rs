// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Buffer;
use crate::BufferList;
use crate::FlowError;
use crate::FlowSuccess;
use crate::Pad;
use crate::ProxyPad;
use std::ptr;

use glib::prelude::*;
use glib::translate::*;

impl ProxyPad {
    #[doc(alias = "gst_proxy_pad_chain_default")]
    pub fn chain_default<O: IsA<ProxyPad>>(
        pad: &O,
        parent: Option<&impl IsA<crate::Object>>,
        buffer: Buffer,
    ) -> Result<FlowSuccess, FlowError> {
        skip_assert_initialized!();
        unsafe {
            try_from_glib(ffi::gst_proxy_pad_chain_default(
                pad.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                buffer.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_proxy_pad_chain_list_default")]
    pub fn chain_list_default<O: IsA<ProxyPad>>(
        pad: &O,
        parent: Option<&impl IsA<crate::Object>>,
        list: BufferList,
    ) -> Result<FlowSuccess, FlowError> {
        skip_assert_initialized!();
        unsafe {
            try_from_glib(ffi::gst_proxy_pad_chain_list_default(
                pad.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                list.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_proxy_pad_getrange_default")]
    pub fn getrange_default<O: IsA<ProxyPad>>(
        pad: &O,
        parent: Option<&impl IsA<crate::Object>>,
        offset: u64,
        size: u32,
    ) -> Result<Buffer, FlowError> {
        skip_assert_initialized!();
        unsafe {
            let mut buffer = ptr::null_mut();
            FlowSuccess::try_from_glib(ffi::gst_proxy_pad_getrange_default(
                pad.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ))
            .map(|_| from_glib_full(buffer))
        }
    }

    #[doc(alias = "gst_proxy_pad_iterate_internal_links_default")]
    pub fn iterate_internal_links_default<O: IsA<ProxyPad>>(
        pad: &O,
        parent: Option<&impl IsA<crate::Object>>,
    ) -> Option<crate::Iterator<Pad>> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_proxy_pad_iterate_internal_links_default(
                pad.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}
