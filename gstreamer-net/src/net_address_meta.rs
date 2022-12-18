// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::translate::*;
use gst::prelude::*;

#[repr(transparent)]
#[doc(alias = "GstNetAddressMeta")]
pub struct NetAddressMeta(ffi::GstNetAddressMeta);

unsafe impl Send for NetAddressMeta {}
unsafe impl Sync for NetAddressMeta {}

impl NetAddressMeta {
    #[doc(alias = "gst_buffer_add_net_address_meta")]
    pub fn add<'a, A: IsA<gio::SocketAddress>>(
        buffer: &'a mut gst::BufferRef,
        addr: &A,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_net_address_meta(
                buffer.as_mut_ptr(),
                addr.as_ref().to_glib_none().0,
            );
            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_addr")]
    #[inline]
    pub fn addr(&self) -> gio::SocketAddress {
        unsafe { from_glib_none(self.0.addr) }
    }

    #[inline]
    pub fn set_addr(&mut self, addr: impl IsA<gio::SocketAddress>) {
        #![allow(clippy::cast_ptr_alignment)]
        unsafe {
            glib::gobject_ffi::g_object_unref(self.0.addr as *mut _);
            self.0.addr = addr.upcast().into_glib_ptr();
        }
    }
}

unsafe impl MetaAPI for NetAddressMeta {
    type GstType = ffi::GstNetAddressMeta;

    #[doc(alias = "gst_net_address_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_net_address_meta_api_get_type()) }
    }
}

impl fmt::Debug for NetAddressMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NetAddressMeta")
            .field("addr", &self.addr())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use gio::prelude::*;

    use super::*;

    #[test]
    fn test_add_get_meta() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::new();
        let port = 5000;
        let inet_addr = gio::InetAddress::from_string("127.0.0.1").unwrap();

        let expected_addr = &gio::InetSocketAddress::new(&inet_addr, port);

        let expected_inet_addr = expected_addr.address();

        {
            let meta = NetAddressMeta::add(
                buffer.get_mut().unwrap(),
                &gio::InetSocketAddress::new(&inet_addr, port),
            );

            let actual_addr = meta.addr().downcast::<gio::InetSocketAddress>().unwrap();

            assert_eq!(actual_addr.port(), expected_addr.port());

            let actual_inet_addr = actual_addr.address();

            assert!(actual_inet_addr.equal(&expected_inet_addr));
        }

        {
            let meta = buffer.meta::<NetAddressMeta>().unwrap();
            let actual_addr = meta.addr().downcast::<gio::InetSocketAddress>().unwrap();

            assert_eq!(actual_addr.port(), expected_addr.port());

            let actual_inet_addr = actual_addr.address();

            assert!(actual_inet_addr.equal(&expected_inet_addr));
        }
    }
}
