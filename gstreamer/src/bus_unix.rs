// Take a look at the license at the top of the repository in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(unix)] {
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

pub trait UnixBusExtManual: 'static {
    #[doc(alias = "get_pollfd")]
    fn pollfd(&self) -> unix::io::RawFd;
}

impl UnixBusExtManual for Bus {
    fn pollfd(&self) -> unix::io::RawFd {
        #[cfg(unix)]
        unsafe {
            let mut pollfd = mem::MaybeUninit::zeroed();
            ffi::gst_bus_get_pollfd(self.to_glib_none().0, pollfd.as_mut_ptr());
            let pollfd = pollfd.assume_init();
            pollfd.fd
        }

        #[cfg(all(not(unix), feature = "dox"))]
        unix::io::RawFd {}
    }
}
