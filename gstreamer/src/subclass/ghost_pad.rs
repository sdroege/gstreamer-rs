// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;

use super::prelude::*;
use glib::subclass::prelude::*;

use GhostPadClass;

pub trait GhostPadImpl: PadImpl + Send + Sync + 'static {}

unsafe impl<T: ObjectSubclass + GhostPadImpl> IsSubclassable<T> for GhostPadClass {
    fn override_vfuncs(&mut self) {
        <::PadClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let _klass = &mut *(self as *const Self as *mut ffi::GstGhostPadClass);
            // Nothing to do here
        }
    }
}
