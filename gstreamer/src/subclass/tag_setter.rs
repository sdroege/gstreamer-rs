// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;

use glib::subclass::prelude::*;

use TagSetter;

pub trait TagSetterImpl: super::element::ElementImpl + Send + Sync + 'static {}

unsafe impl<T: ObjectSubclass + TagSetterImpl> IsImplementable<T> for TagSetter {
    unsafe extern "C" fn interface_init(
        _iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
    }
}
