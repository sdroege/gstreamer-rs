// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstNetClientClock")]
    pub struct NetClientClock(Object<ffi::GstNetClientClock, ffi::GstNetClientClockClass>) @extends gst::Clock, gst::Object;

    match fn {
        type_ => || ffi::gst_net_client_clock_get_type(),
    }
}

impl NetClientClock {
    #[doc(alias = "gst_net_client_clock_new")]
    pub fn new(
        name: Option<&str>,
        remote_address: &str,
        remote_port: i32,
        base_time: impl Into<Option<gst::ClockTime>>,
    ) -> NetClientClock {
        assert_initialized_main_thread!();
        unsafe {
            gst::Clock::from_glib_full(ffi::gst_net_client_clock_new(
                name.to_glib_none().0,
                remote_address.to_glib_none().0,
                remote_port,
                base_time.into().into_glib(),
            ))
            .unsafe_cast()
        }
    }

    pub fn address(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "address")
    }

    pub fn set_address(&self, address: Option<&str>) {
        ObjectExt::set_property(self, "address", address)
    }

    #[doc(alias = "base-time")]
    pub fn base_time(&self) -> u64 {
        ObjectExt::property(self, "base-time")
    }

    pub fn bus(&self) -> Option<gst::Bus> {
        ObjectExt::property(self, "bus")
    }

    pub fn set_bus<P: IsA<gst::Bus>>(&self, bus: Option<&P>) {
        ObjectExt::set_property(self, "bus", bus)
    }

    #[doc(alias = "internal-clock")]
    pub fn internal_clock(&self) -> Option<gst::Clock> {
        ObjectExt::property(self, "internal-clock")
    }

    #[doc(alias = "minimum-update-interval")]
    pub fn minimum_update_interval(&self) -> u64 {
        ObjectExt::property(self, "minimum-update-interval")
    }

    #[doc(alias = "minimum-update-interval")]
    pub fn set_minimum_update_interval(&self, minimum_update_interval: u64) {
        ObjectExt::set_property(self, "minimum-update-interval", minimum_update_interval)
    }

    pub fn port(&self) -> i32 {
        ObjectExt::property(self, "port")
    }

    pub fn set_port(&self, port: i32) {
        ObjectExt::set_property(self, "port", port)
    }

    #[doc(alias = "qos-dscp")]
    pub fn qos_dscp(&self) -> i32 {
        ObjectExt::property(self, "qos-dscp")
    }

    #[doc(alias = "qos-dscp")]
    pub fn set_qos_dscp(&self, qos_dscp: i32) {
        ObjectExt::set_property(self, "qos-dscp", qos_dscp)
    }

    #[doc(alias = "round-trip-limit")]
    pub fn round_trip_limit(&self) -> u64 {
        ObjectExt::property(self, "round-trip-limit")
    }

    #[doc(alias = "round-trip-limit")]
    pub fn set_round_trip_limit(&self, round_trip_limit: u64) {
        ObjectExt::set_property(self, "round-trip-limit", round_trip_limit)
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_net_client_clock_deinit")]
    pub fn deinit() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_net_client_clock_deinit();
        }
    }

    #[doc(alias = "address")]
    pub fn connect_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_address_trampoline<
            F: Fn(&NetClientClock) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetClientClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::address".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "bus")]
    pub fn connect_bus_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bus_trampoline<
            F: Fn(&NetClientClock) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetClientClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::bus".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_bus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "internal-clock")]
    pub fn connect_internal_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_internal_clock_trampoline<
            F: Fn(&NetClientClock) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetClientClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::internal-clock".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_internal_clock_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "minimum-update-interval")]
    pub fn connect_minimum_update_interval_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_update_interval_trampoline<
            F: Fn(&NetClientClock) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetClientClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::minimum-update-interval".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_minimum_update_interval_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "port")]
    pub fn connect_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_port_trampoline<
            F: Fn(&NetClientClock) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetClientClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::port".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_port_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "qos-dscp")]
    pub fn connect_qos_dscp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_dscp_trampoline<
            F: Fn(&NetClientClock) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetClientClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::qos-dscp".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_qos_dscp_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "round-trip-limit")]
    pub fn connect_round_trip_limit_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_round_trip_limit_trampoline<
            F: Fn(&NetClientClock) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetClientClock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::round-trip-limit".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_round_trip_limit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for NetClientClock {}
unsafe impl Sync for NetClientClock {}
