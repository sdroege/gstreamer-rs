// This file was generated by gir (https://github.com/gtk-rs/gir @ 47eb915)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPAddress;
use RTSPAddressFlags;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RTSPAddressPool(Object<ffi::GstRTSPAddressPool, ffi::GstRTSPAddressPoolClass>);

    match fn {
        get_type => || ffi::gst_rtsp_address_pool_get_type(),
    }
}

impl RTSPAddressPool {
    pub fn new() -> RTSPAddressPool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_address_pool_new())
        }
    }
}

impl Default for RTSPAddressPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPAddressPool {}
unsafe impl Sync for RTSPAddressPool {}

pub trait RTSPAddressPoolExt {
    fn acquire_address(&self, flags: RTSPAddressFlags, n_ports: i32) -> Option<RTSPAddress>;

    fn add_range(&self, min_address: &str, max_address: &str, min_port: u16, max_port: u16, ttl: u8) -> Result<(), glib::error::BoolError>;

    fn clear(&self);

    fn dump(&self);

    fn has_unicast_addresses(&self) -> bool;
}

impl<O: IsA<RTSPAddressPool>> RTSPAddressPoolExt for O {
    fn acquire_address(&self, flags: RTSPAddressFlags, n_ports: i32) -> Option<RTSPAddress> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_address_pool_acquire_address(self.to_glib_none().0, flags.to_glib(), n_ports))
        }
    }

    fn add_range(&self, min_address: &str, max_address: &str, min_port: u16, max_port: u16, ttl: u8) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_rtsp_address_pool_add_range(self.to_glib_none().0, min_address.to_glib_none().0, max_address.to_glib_none().0, min_port, max_port, ttl), "Failed to add address range")
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gst_rtsp_address_pool_clear(self.to_glib_none().0);
        }
    }

    fn dump(&self) {
        unsafe {
            ffi::gst_rtsp_address_pool_dump(self.to_glib_none().0);
        }
    }

    fn has_unicast_addresses(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_address_pool_has_unicast_addresses(self.to_glib_none().0))
        }
    }
}
