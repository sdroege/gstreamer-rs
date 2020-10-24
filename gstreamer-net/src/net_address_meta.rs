use std::fmt;

use gio;
use glib;
use glib::translate::*;
use gst;
use gst::prelude::*;
use gst_net_sys;

#[repr(transparent)]
pub struct NetAddressMeta(gst_net_sys::GstNetAddressMeta);

unsafe impl Send for NetAddressMeta {}
unsafe impl Sync for NetAddressMeta {}

impl NetAddressMeta {
    pub fn add<'a, A: IsA<gio::SocketAddress>>(
        buffer: &'a mut gst::BufferRef,
        addr: &A,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_net_sys::gst_buffer_add_net_address_meta(
                buffer.as_mut_ptr(),
                addr.as_ref().to_glib_none().0,
            );
            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_addr(&self) -> gio::SocketAddress {
        unsafe { from_glib_none(self.0.addr) }
    }

    pub fn set_addr<T: IsA<gio::SocketAddress>>(&mut self, addr: &T) {
        #![allow(clippy::cast_ptr_alignment)]
        unsafe {
            gobject_sys::g_object_unref(self.0.addr as *mut _);
            self.0.addr = addr.as_ref().to_glib_full();
        }
    }
}

unsafe impl MetaAPI for NetAddressMeta {
    type GstType = gst_net_sys::GstNetAddressMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_net_sys::gst_net_address_meta_api_get_type()) }
    }
}

impl fmt::Debug for NetAddressMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NetAddressMeta")
            .field("addr", &self.get_addr())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gio::InetAddressExt;
    use gio::InetSocketAddressExt;

    #[test]
    fn test_add_get_meta() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::new();
        let port = 5000;
        let inet_addr = gio::InetAddress::from_string("127.0.0.1").unwrap();

        let expected_addr = &gio::InetSocketAddress::new(&inet_addr, port);

        let expected_inet_addr = expected_addr.get_address().unwrap();

        {
            let meta = NetAddressMeta::add(
                buffer.get_mut().unwrap(),
                &gio::InetSocketAddress::new(&inet_addr, port),
            );

            let actual_addr = meta
                .get_addr()
                .downcast::<gio::InetSocketAddress>()
                .unwrap();

            assert_eq!(actual_addr.get_port(), expected_addr.get_port());

            let actual_inet_addr = actual_addr.get_address().unwrap();

            assert!(actual_inet_addr.equal(&expected_inet_addr));
        }

        {
            let meta = buffer.get_meta::<NetAddressMeta>().unwrap();
            let actual_addr = meta
                .get_addr()
                .downcast::<gio::InetSocketAddress>()
                .unwrap();

            assert_eq!(actual_addr.get_port(), expected_addr.get_port());

            let actual_inet_addr = actual_addr.get_address().unwrap();

            assert!(actual_inet_addr.equal(&expected_inet_addr));
        }
    }
}
