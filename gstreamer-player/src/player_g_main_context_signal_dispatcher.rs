// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::translate::*;
use gst_player_sys;
use PlayerGMainContextSignalDispatcher;

impl PlayerGMainContextSignalDispatcher {
    pub fn new(
        application_context: Option<&glib::MainContext>,
    ) -> PlayerGMainContextSignalDispatcher {
        assert_initialized_main_thread!();
        let application_context = application_context.to_glib_none();
        unsafe {
            from_glib_full(
                gst_player_sys::gst_player_g_main_context_signal_dispatcher_new(
                    application_context.0,
                ) as *mut gst_player_sys::GstPlayerGMainContextSignalDispatcher,
            )
        }
    }
}
