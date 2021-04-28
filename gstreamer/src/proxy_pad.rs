// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Buffer;
use crate::BufferList;
use crate::FlowError;
use crate::FlowSuccess;
use crate::Object;
use crate::Pad;
use crate::ProxyPad;
use std::ptr;

use glib::prelude::*;
use glib::translate::*;

pub trait ProxyPadExtManual: 'static {
    fn chain_default<P: IsA<Object>>(
        &self,
        parent: Option<&P>,
        buffer: Buffer,
    ) -> Result<FlowSuccess, FlowError>;

    fn chain_list_default<P: IsA<Object>>(
        &self,
        parent: Option<&P>,
        list: BufferList,
    ) -> Result<FlowSuccess, FlowError>;

    fn getrange_default<P: IsA<Object>>(
        &self,
        parent: Option<&P>,
        offset: u64,
        size: u32,
    ) -> Result<Buffer, FlowError>;

    fn iterate_internal_links_default<P: IsA<Object>>(
        &self,
        parent: Option<&P>,
    ) -> Option<crate::Iterator<Pad>>;
}

impl<O: IsA<ProxyPad>> ProxyPadExtManual for O {
    fn chain_default<P: IsA<Object>>(
        &self,
        parent: Option<&P>,
        buffer: Buffer,
    ) -> Result<FlowSuccess, FlowError> {
        skip_assert_initialized!();
        unsafe {
            FlowSuccess::try_from_glib(ffi::gst_proxy_pad_chain_default(
                self.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                buffer.into_ptr(),
            ))
        }
    }

    fn chain_list_default<P: IsA<Object>>(
        &self,
        parent: Option<&P>,
        list: BufferList,
    ) -> Result<FlowSuccess, FlowError> {
        skip_assert_initialized!();
        unsafe {
            FlowSuccess::try_from_glib(ffi::gst_proxy_pad_chain_list_default(
                self.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                list.into_ptr(),
            ))
        }
    }

    fn getrange_default<P: IsA<Object>>(
        &self,
        parent: Option<&P>,
        offset: u64,
        size: u32,
    ) -> Result<Buffer, FlowError> {
        skip_assert_initialized!();
        unsafe {
            let mut buffer = ptr::null_mut();
            FlowSuccess::try_from_glib(ffi::gst_proxy_pad_getrange_default(
                self.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ))
            .map(|_| from_glib_full(buffer))
        }
    }

    fn iterate_internal_links_default<P: IsA<Object>>(
        &self,
        parent: Option<&P>,
    ) -> Option<crate::Iterator<Pad>> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_proxy_pad_iterate_internal_links_default(
                self.as_ptr() as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}
