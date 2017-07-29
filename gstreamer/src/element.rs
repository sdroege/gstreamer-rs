// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Element;

use glib;
use glib::IsA;
use glib::translate::{ToGlibPtr, from_glib};
use QueryRef;
use miniobject::MiniObject;

use ffi;

impl Element {
    pub fn link_many<E: IsA<Element>>(elements: &[&E]) -> Result<(), glib::BoolError> {
        for (e1, e2) in elements.iter().zip(elements.iter().skip(1)) {
            unsafe {
                let ret: bool = from_glib(ffi::gst_element_link(
                    e1.to_glib_none().0,
                    e2.to_glib_none().0,
                ));
                if !ret {
                    return Err(glib::BoolError("Failed to link elements"));
                }
            }
        }

        return Ok(());
    }

    pub fn unlink_many<E: IsA<Element>>(elements: &[&E]) {
        for (e1, e2) in elements.iter().zip(elements.iter().skip(1)) {
            unsafe {
                ffi::gst_element_unlink(e1.to_glib_none().0, e2.to_glib_none().0);
            }
        }
    }
}

pub trait ElementExtManual {
    fn query(&self, query: &mut QueryRef) -> bool;
}

impl<O: IsA<Element>> ElementExtManual for O {
    fn query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_query(self.to_glib_none().0, query.as_mut_ptr()))
        }
    }
}
