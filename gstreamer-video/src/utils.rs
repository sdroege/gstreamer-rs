// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub trait HasStreamLock {
    fn get_stream_lock(&self) -> *mut glib_sys::GRecMutex;
    fn get_element_as_ptr(&self) -> *const gst_sys::GstElement;
}
