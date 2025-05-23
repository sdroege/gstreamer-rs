// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, WebRTCICEComponent, WebRTCICEConnectionState, WebRTCICEGatheringState};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstWebRTCICETransport")]
    pub struct WebRTCICETransport(Object<ffi::GstWebRTCICETransport, ffi::GstWebRTCICETransportClass>);

    match fn {
        type_ => || ffi::gst_webrtc_ice_transport_get_type(),
    }
}

impl WebRTCICETransport {
    #[doc(alias = "gst_webrtc_ice_transport_connection_state_change")]
    pub fn connection_state_change(&self, new_state: WebRTCICEConnectionState) {
        unsafe {
            ffi::gst_webrtc_ice_transport_connection_state_change(
                self.to_glib_none().0,
                new_state.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_transport_gathering_state_change")]
    pub fn gathering_state_change(&self, new_state: WebRTCICEGatheringState) {
        unsafe {
            ffi::gst_webrtc_ice_transport_gathering_state_change(
                self.to_glib_none().0,
                new_state.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_transport_new_candidate")]
    pub fn new_candidate(&self, stream_id: u32, component: WebRTCICEComponent, attr: &str) {
        unsafe {
            ffi::gst_webrtc_ice_transport_new_candidate(
                self.to_glib_none().0,
                stream_id,
                component.into_glib(),
                attr.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_transport_selected_pair_change")]
    pub fn selected_pair_change(&self) {
        unsafe {
            ffi::gst_webrtc_ice_transport_selected_pair_change(self.to_glib_none().0);
        }
    }

    pub fn component(&self) -> WebRTCICEComponent {
        ObjectExt::property(self, "component")
    }

    #[doc(alias = "gathering-state")]
    pub fn gathering_state(&self) -> WebRTCICEGatheringState {
        ObjectExt::property(self, "gathering-state")
    }

    pub fn state(&self) -> WebRTCICEConnectionState {
        ObjectExt::property(self, "state")
    }

    #[doc(alias = "on-new-candidate")]
    pub fn connect_on_new_candidate<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_new_candidate_trampoline<
            F: Fn(&WebRTCICETransport, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICETransport,
            object: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"on-new-candidate".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    on_new_candidate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-selected-candidate-pair-change")]
    pub fn connect_on_selected_candidate_pair_change<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_selected_candidate_pair_change_trampoline<
            F: Fn(&WebRTCICETransport) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICETransport,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"on-selected-candidate-pair-change".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    on_selected_candidate_pair_change_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "gathering-state")]
    pub fn connect_gathering_state_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_gathering_state_trampoline<
            F: Fn(&WebRTCICETransport) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICETransport,
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
                c"notify::gathering-state".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_gathering_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "state")]
    pub fn connect_state_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<
            F: Fn(&WebRTCICETransport) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICETransport,
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
                c"notify::state".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for WebRTCICETransport {}
unsafe impl Sync for WebRTCICETransport {}
