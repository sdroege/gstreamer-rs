// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::mut_override;
use glib_sys;

#[must_use = "if unused the Mutex will immediately unlock"]
pub struct MutexGuard<'a>(&'a glib_sys::GMutex);

impl<'a> MutexGuard<'a> {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn lock(mutex: &'a glib_sys::GMutex) -> Self {
        skip_assert_initialized!();
        unsafe {
            glib_sys::g_mutex_lock(mut_override(mutex));
        }
        MutexGuard(mutex)
    }
}

impl<'a> Drop for MutexGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            glib_sys::g_mutex_unlock(mut_override(self.0));
        }
    }
}
