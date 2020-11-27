// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::TestClock;
use glib::translate::*;
use std::ptr;

impl TestClock {
    pub fn has_id(&self, id: &gst::ClockId) -> bool {
        unsafe {
            from_glib(ffi::gst_test_clock_has_id(
                self.to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    pub fn peek_next_pending_id(&self) -> Option<gst::ClockId> {
        unsafe {
            let mut id = ptr::null_mut();
            let ret: bool = from_glib(ffi::gst_test_clock_peek_next_pending_id(
                self.to_glib_none().0,
                &mut id,
            ));
            if ret {
                from_glib_full(id)
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn process_id(&self, pending_id: &gst::ClockId) -> bool {
        unsafe {
            from_glib(ffi::gst_test_clock_process_id(
                self.to_glib_none().0,
                pending_id.to_glib_none().0,
            ))
        }
    }

    pub fn process_id_list(&self, pending_list: &[&gst::ClockId]) -> u32 {
        unsafe {
            ffi::gst_test_clock_process_id_list(
                self.to_glib_none().0,
                pending_list.to_glib_none().0,
            )
        }
    }

    pub fn process_next_clock_id(&self) -> Option<gst::ClockId> {
        unsafe {
            from_glib_full(ffi::gst_test_clock_process_next_clock_id(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn wait_for_multiple_pending_ids(&self, count: u32) -> Vec<gst::ClockId> {
        unsafe {
            let mut pending_list = ptr::null_mut();
            ffi::gst_test_clock_wait_for_multiple_pending_ids(
                self.to_glib_none().0,
                count,
                &mut pending_list,
            );
            FromGlibPtrContainer::from_glib_full(pending_list)
        }
    }

    pub fn wait_for_next_pending_id(&self) -> gst::ClockId {
        unsafe {
            let mut id = ptr::null_mut();
            ffi::gst_test_clock_wait_for_next_pending_id(self.to_glib_none().0, &mut id);
            from_glib_full(id)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn timed_wait_for_multiple_pending_ids(
        &self,
        count: u32,
        timeout_ms: u32,
    ) -> (bool, Vec<gst::ClockId>) {
        unsafe {
            let mut pending_list = ptr::null_mut();
            let res = ffi::gst_test_clock_timed_wait_for_multiple_pending_ids(
                self.to_glib_none().0,
                count,
                timeout_ms,
                &mut pending_list,
            );
            (
                from_glib(res),
                FromGlibPtrContainer::from_glib_full(pending_list),
            )
        }
    }
}
