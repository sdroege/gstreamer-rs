// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
cfg_if::cfg_if! {
    if #[cfg(windows)] {
        use glib::translate::ToGlibPtr;

        use std::mem;
        use std::os::windows;
    } else if #[cfg(all(not(doctest), doc))] {
        // Declare a fake RawHandle for doc generation on unix
        pub mod windows {
            pub mod io {
                pub struct RawHandle{}
            }
        }
    }
}

use super::Bus;

pub trait WindowsBusExtManual: 'static {
    fn get_pollfd(&self) -> windows::io::RawHandle;
}

impl WindowsBusExtManual for Bus {
    fn get_pollfd(&self) -> windows::io::RawHandle {
        #[cfg(windows)]
        unsafe {
            let mut pollfd = mem::MaybeUninit::zeroed();
            ffi::gst_bus_get_pollfd(self.to_glib_none().0, pollfd.as_mut_ptr());
            let pollfd = pollfd.assume_init();
            pollfd.fd as *mut _
        }

        #[cfg(all(not(windows), all(not(doctest), doc)))]
        windows::io::RawHandle {}
    }
}
