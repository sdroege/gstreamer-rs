// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PlayerGMainContextSignalDispatcher;
use glib::translate::*;

impl PlayerGMainContextSignalDispatcher {
    #[doc(alias = "gst_player_g_main_context_signal_dispatcher_new")]
    pub fn new(
        application_context: Option<&glib::MainContext>,
    ) -> PlayerGMainContextSignalDispatcher {
        assert_initialized_main_thread!();
        let application_context = application_context.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_player_g_main_context_signal_dispatcher_new(
                application_context.0,
            )
                as *mut ffi::GstPlayerGMainContextSignalDispatcher)
        }
    }
}
