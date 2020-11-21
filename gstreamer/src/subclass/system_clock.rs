// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::SystemClock;

pub trait SystemClockImpl: ClockImpl {}

unsafe impl<T: SystemClockImpl> IsSubclassable<T> for SystemClock {
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <crate::Clock as IsSubclassable<T>>::override_vfuncs(klass);
        let _klass = klass.as_mut();
        // Nothing to do here
    }
}
