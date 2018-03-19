// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
cfg_if! {
    if #[cfg(unix)] {
        use ffi;
        use glib_ffi;
        use glib::translate::ToGlibPtr;

        use std::mem;
        use std::os::unix;
    } else if #[cfg(feature = "dox")] {
        // Declare a fake RawFd for doc generation on windows
        pub mod unix {
            pub mod io {
                pub struct RawFd{}
            }
        }
    }
}

use super::Bus;

pub trait UnixBusExtManual {
    fn get_pollfd(&self) -> unix::io::RawFd;
}

impl UnixBusExtManual for Bus {
    /// This is supported on **Unix** only.
    fn get_pollfd(&self) -> unix::io::RawFd {
        #[cfg(unix)]
        unsafe {
            let mut pollfd: glib_ffi::GPollFD = mem::zeroed();
            ffi::gst_bus_get_pollfd(self.to_glib_none().0, &mut pollfd);

            pollfd.fd
        }

        #[cfg(all(not(unix), feature = "dox"))]
        unix::io::RawFd {}
    }
}
