// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_base_sys;
use std::mem;

pub fn type_find_helper_for_data<P: IsA<gst::Object>, R: AsRef<[u8]>>(
    obj: Option<&P>,
    data: R,
) -> (Option<gst::Caps>, gst::TypeFindProbability) {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::uninitialized();
        let data = data.as_ref();
        let (ptr, len) = (data.as_ptr(), data.len());
        let ret = from_glib_full(gst_base_sys::gst_type_find_helper_for_data(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(ptr),
            len,
            &mut prob,
        ));
        (ret, from_glib(prob))
    }
}
