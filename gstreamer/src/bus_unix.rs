// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
cfg_if::cfg_if! {
    if #[cfg(unix)] {
        use glib::translate::ToGlibPtr;

        use std::mem;
        use std::os::unix;
    } else if #[cfg(all(not(doctest), doc))] {
        // Declare a fake RawFd for doc generation on windows
        pub mod unix {
            pub mod io {
                pub struct RawFd{}
            }
        }
    }
}

use super::Bus;

pub trait UnixBusExtManual: 'static {
    fn get_pollfd(&self) -> unix::io::RawFd;
}

impl UnixBusExtManual for Bus {
    fn get_pollfd(&self) -> unix::io::RawFd {
        #[cfg(unix)]
        unsafe {
            let mut pollfd = mem::MaybeUninit::zeroed();
            ffi::gst_bus_get_pollfd(self.to_glib_none().0, pollfd.as_mut_ptr());
            let pollfd = pollfd.assume_init();
            pollfd.fd
        }

        #[cfg(all(not(unix), all(not(doctest), doc)))]
        unix::io::RawFd {}
    }
}
