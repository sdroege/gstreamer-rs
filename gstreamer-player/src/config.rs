// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst;
use gst_player_sys;
use gst_sys;

use std::mem;
use std::ops;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerConfig(gst::Structure);

impl ops::Deref for PlayerConfig {
    type Target = gst::StructureRef;

    fn deref(&self) -> &gst::StructureRef {
        self.0.deref()
    }
}

impl ops::DerefMut for PlayerConfig {
    fn deref_mut(&mut self) -> &mut gst::StructureRef {
        self.0.deref_mut()
    }
}

impl AsRef<gst::StructureRef> for PlayerConfig {
    fn as_ref(&self) -> &gst::StructureRef {
        self.0.as_ref()
    }
}

impl AsMut<gst::StructureRef> for PlayerConfig {
    fn as_mut(&mut self) -> &mut gst::StructureRef {
        self.0.as_mut()
    }
}

impl PlayerConfig {
    pub fn get_position_update_interval(&self) -> u32 {
        assert_initialized_main_thread!();
        unsafe {
            gst_player_sys::gst_player_config_get_position_update_interval(self.0.to_glib_none().0)
        }
    }

    pub fn get_seek_accurate(&self) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(gst_player_sys::gst_player_config_get_seek_accurate(
                self.0.to_glib_none().0,
            ))
        }
    }

    pub fn get_user_agent(&self) -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_player_sys::gst_player_config_get_user_agent(
                self.0.to_glib_none().0,
            ))
        }
    }

    pub fn set_position_update_interval(&mut self, interval: u32) {
        assert_initialized_main_thread!();
        unsafe {
            gst_player_sys::gst_player_config_set_position_update_interval(
                self.0.to_glib_none_mut().0,
                interval,
            );
        }
    }

    pub fn set_seek_accurate(&mut self, accurate: bool) {
        assert_initialized_main_thread!();
        // FIXME: Work-around for
        // http://cgit.freedesktop.org/gstreamer/gst-plugins-bad/commit/?id=cc58bd6ae071dec4ea7b4be626034accd0372755
        self.set("accurate-seek", &accurate);
    }

    pub fn set_user_agent(&mut self, agent: &str) {
        assert_initialized_main_thread!();
        unsafe {
            gst_player_sys::gst_player_config_set_user_agent(
                self.0.to_glib_none_mut().0,
                agent.to_glib_none().0,
            );
        }
    }

    pub unsafe fn into_ptr(self) -> *mut gst_sys::GstStructure {
        let mut s = mem::ManuallyDrop::new(self);
        s.0.to_glib_none_mut().0
    }
}

impl FromGlibPtrFull<*mut gst_sys::GstStructure> for PlayerConfig {
    unsafe fn from_glib_full(ptr: *mut gst_sys::GstStructure) -> Self {
        PlayerConfig(from_glib_full(ptr))
    }
}
