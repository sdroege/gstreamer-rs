// Take a look at the license at the top of the repository in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        use glib::translate::ToGlibPtr;

        use std::mem;
        use std::os::unix;
    } else if #[cfg(docsrs)] {
        // Declare a fake RawFd for doc generation on windows
        pub mod unix {
            pub mod io {
                pub struct RawFd{}
            }
        }
    }
}

use super::Bus;
use glib::prelude::*;

pub trait UnixBusExtManual: IsA<Bus> + 'static {
    #[doc(alias = "get_pollfd")]
    #[doc(alias = "gst_bus_get_pollfd")]
    fn pollfd(&self) -> unix::io::RawFd;
}

impl<T: IsA<Bus>> UnixBusExtManual for T {
    fn pollfd(&self) -> unix::io::RawFd {
        #[cfg(unix)]
        unsafe {
            let mut pollfd = mem::MaybeUninit::uninit();
            crate::ffi::gst_bus_get_pollfd(self.as_ref().to_glib_none().0, pollfd.as_mut_ptr());
            let pollfd = pollfd.assume_init();
            pollfd.fd
        }

        #[cfg(all(not(unix), docsrs))]
        unix::io::RawFd {}
    }
}
