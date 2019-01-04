// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_ffi;
use gst;
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
            unsafe { from_glib_full(ffi::gst_player_new(video_renderer, signal_dispatcher)) }
        } else {
            // Workaround for bad floating reference handling in 1.12. This issue was fixed for 1.13 in
            // https://cgit.freedesktop.org/gstreamer/gst-plugins-bad/commit/gst-libs/gst/player/gstplayer.c?id=634cd87c76f58b5e1383715bafd5614db825c7d1
            unsafe { from_glib_none(ffi::gst_player_new(video_renderer, signal_dispatcher)) }
        }
    }

    pub fn get_config(&self) -> ::PlayerConfig {
        unsafe { from_glib_full(ffi::gst_player_get_config(self.to_glib_none().0)) }
    }

    pub fn set_config(&self, config: ::PlayerConfig) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ffi::gst_player_set_config(self.to_glib_none().0, config.into_ptr()),
                "Failed to set config",
            )
        }
    }

    pub fn connect_duration_changed<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, gst::ClockTime) + Send + 'static>> =
                Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "duration-changed",
                transmute(duration_changed_trampoline as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    pub fn connect_position_updated<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, gst::ClockTime) + Send + 'static>> =
                Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "position-updated",
                transmute(position_updated_trampoline as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    pub fn connect_seek_done<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Player, gst::ClockTime) + Send + 'static>> =
                Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "seek-done",
                transmute(seek_done_trampoline as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn duration_changed_trampoline(
    this: *mut ffi::GstPlayer,
    object: u64,
    f: glib_ffi::gpointer,
) {
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let f: &&(Fn(&Player, gst::ClockTime) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), gst::ClockTime(Some(object)))
}

unsafe extern "C" fn position_updated_trampoline(
    this: *mut ffi::GstPlayer,
    object: u64,
    f: glib_ffi::gpointer,
) {
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let f: &&(Fn(&Player, gst::ClockTime) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this), gst::ClockTime(Some(object)))
}

unsafe extern "C" fn seek_done_trampoline(
    this: *mut ffi::GstPlayer,
    object: u64,
    f: glib_ffi::gpointer,
) {
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let f: &&(Fn(&Player, gst::ClockTime) + Send + 'static) = transmute(f);
    f(&from_glib_borrow(this), gst::ClockTime(Some(object)))
}
