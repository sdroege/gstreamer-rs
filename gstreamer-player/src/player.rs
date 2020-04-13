// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::object::ObjectType;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gst;
use gst_player_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Player;
use PlayerSignalDispatcher;
use PlayerVideoRenderer;

impl Player {
    pub fn new(
        video_renderer: Option<&PlayerVideoRenderer>,
        signal_dispatcher: Option<&PlayerSignalDispatcher>,
    ) -> Player {
        assert_initialized_main_thread!();
        let video_renderer = video_renderer.to_glib_full();
        let signal_dispatcher = signal_dispatcher.to_glib_full();

        let (major, minor, _, _) = gst::version();
        if (major, minor) > (1, 12) {
            unsafe {
                from_glib_full(gst_player_sys::gst_player_new(
                    video_renderer,
                    signal_dispatcher,
                ))
            }
        } else {
            // Workaround for bad floating reference handling in 1.12. This issue was fixed for 1.13 in
            // https://cgit.freedesktop.org/gstreamer/gst-plugins-bad/commit/gst-libs/gst/player/gstplayer.c?id=634cd87c76f58b5e1383715bafd5614db825c7d1
            unsafe {
                from_glib_none(gst_player_sys::gst_player_new(
                    video_renderer,
                    signal_dispatcher,
                ))
            }
        }
    }

    pub fn get_config(&self) -> ::PlayerConfig {
        unsafe { from_glib_full(gst_player_sys::gst_player_get_config(self.to_glib_none().0)) }
    }

    pub fn set_config(&self, config: ::PlayerConfig) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_player_sys::gst_player_set_config(self.to_glib_none().0, config.into_ptr()),
                "Failed to set config",
            )
        }
    }

    pub fn connect_duration_changed<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"duration-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    duration_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_position_updated<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"position-updated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    position_updated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_seek_done<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"seek-done\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    seek_done_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn duration_changed_trampoline<
    F: Fn(&Player, gst::ClockTime) + Send + 'static,
>(
    this: *mut gst_player_sys::GstPlayer,
    object: u64,
    f: glib_sys::gpointer,
) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this), gst::ClockTime(Some(object)))
}

unsafe extern "C" fn position_updated_trampoline<
    F: Fn(&Player, gst::ClockTime) + Send + 'static,
>(
    this: *mut gst_player_sys::GstPlayer,
    object: u64,
    f: glib_sys::gpointer,
) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this), gst::ClockTime(Some(object)))
}

unsafe extern "C" fn seek_done_trampoline<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
    this: *mut gst_player_sys::GstPlayer,
    object: u64,
    f: glib_sys::gpointer,
) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this), gst::ClockTime(Some(object)))
}
