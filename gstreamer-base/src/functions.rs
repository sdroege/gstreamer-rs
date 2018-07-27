// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::object::IsA;
use glib::translate::*;
use gst;
use std::mem;

pub fn type_find_helper_for_data<
    'a,
    P: IsA<gst::Object> + 'a,
    Q: Into<Option<&'a P>>,
    R: AsRef<[u8]>,
>(
    obj: Q,
    data: R,
) -> (Option<gst::Caps>, gst::TypeFindProbability) {
    assert_initialized_main_thread!();
    let obj = obj.into();
    let obj = obj.to_glib_none();
    unsafe {
        let mut prob = mem::uninitialized();
        let data = data.as_ref();
        let (ptr, len) = (data.as_ptr(), data.len());
        let ret = from_glib_full(ffi::gst_type_find_helper_for_data(
            obj.0,
            mut_override(ptr),
            len,
            &mut prob,
        ));
        (ret, from_glib(prob))
    }
}
