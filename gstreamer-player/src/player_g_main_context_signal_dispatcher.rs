// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use PlayerGMainContextSignalDispatcher;
use ffi;
use glib;
use glib::translate::*;

impl PlayerGMainContextSignalDispatcher {
    pub fn new<'a, P: Into<Option<&'a glib::MainContext>>>(
        application_context: P,
    ) -> PlayerGMainContextSignalDispatcher {
        assert_initialized_main_thread!();
        let application_context = application_context.into();
        let application_context = application_context.to_glib_none();
        unsafe {
            from_glib_full(
                ffi::gst_player_g_main_context_signal_dispatcher_new(application_context.0)
                    as *mut ffi::GstPlayerGMainContextSignalDispatcher,
            )
        }
    }
}
