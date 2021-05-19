// Take a look at the license at the top of the repository in the LICENSE file.

use crate::NetTimeProvider;

use glib::prelude::*;
use glib::translate::*;

impl NetTimeProvider {
    #[doc(alias = "gst_net_time_provider_new")]
    pub fn new<P: IsA<gst::Clock>>(clock: &P, address: Option<&str>, port: i32) -> NetTimeProvider {
        assert_initialized_main_thread!();
        let address = address.to_glib_none();

        let (major, minor, _, _) = gst::version();
        if (major, minor) > (1, 12) {
            unsafe {
                from_glib_full(ffi::gst_net_time_provider_new(
                    clock.as_ref().to_glib_none().0,
                    address.0,
                    port,
                ))
            }
        } else {
            // Workaround for bad floating reference handling in 1.12. This issue was fixed for 1.13
            unsafe {
                from_glib_none(ffi::gst_net_time_provider_new(
                    clock.as_ref().to_glib_none().0,
                    address.0,
                    port,
                ))
            }
        }
    }
}
