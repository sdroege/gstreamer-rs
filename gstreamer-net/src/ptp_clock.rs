// Take a look at the license at the top of the repository in the LICENSE file.

use std::num::NonZeroU64;

use glib::translate::*;

use crate::PtpClock;

impl PtpClock {
    // rustdoc-stripper-ignore-next
    /// Initialize GStreamer PTP clock support
    ///
    /// This is automatically called once the first PTP clock instance is created.
    #[doc(alias = "gst_ptp_init")]
    pub fn init(clock_id: Option<u64>, interfaces: &[&str]) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let res: bool = from_glib(ffi::gst_ptp_init(
                clock_id.unwrap_or(u64::MAX),
                interfaces.to_glib_none().0,
            ));

            if res {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to initialize PTP subsystem"))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Deinitialize GStreamer PTP clock support
    ///
    /// Any PTP clocks that are still running will not receive any updates anymore.
    #[doc(alias = "gst_ptp_deinit")]
    pub fn deinit() {
        unsafe {
            ffi::gst_ptp_deinit();
        }
    }

    // rustdoc-stripper-ignore-next
    /// Check if the GStreamer PTP clock support is initialized
    #[doc(alias = "gst_ptp_is_initialized")]
    pub fn is_initialized() -> bool {
        unsafe { from_glib(ffi::gst_ptp_is_initialized()) }
    }

    // rustdoc-stripper-ignore-next
    /// Check if GStreamer PTP clocks are supported
    #[doc(alias = "gst_ptp_is_supported")]
    pub fn is_supported() -> bool {
        unsafe { from_glib(ffi::gst_ptp_is_supported()) }
    }

    // rustdoc-stripper-ignore-next
    /// Add a PTP clock statistics callback
    #[doc(alias = "gst_ptp_statistics_callback_add")]
    pub fn add_statistics_callback<
        F: Fn(u8, &gst::StructureRef) -> glib::Continue + 'static + Send + Sync,
    >(
        func: F,
    ) -> PtpStatisticsCallback {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: Fn(u8, &gst::StructureRef) -> glib::Continue + 'static + Send + Sync,
            >(
                domain: u8,
                stats: *const gst::ffi::GstStructure,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let callback = &*(user_data as *const F);
                callback(domain, gst::StructureRef::from_glib_borrow(stats)).into_glib()
            }

            unsafe extern "C" fn destroy<
                F: Fn(u8, &gst::StructureRef) -> glib::Continue + 'static + Send + Sync,
            >(
                user_data: glib::ffi::gpointer,
            ) {
                let _ = Box::from_raw(user_data as *mut F);
            }

            let user_data = Box::new(func);
            let id = ffi::gst_ptp_statistics_callback_add(
                Some(trampoline::<F>),
                Box::into_raw(user_data) as glib::ffi::gpointer,
                Some(destroy::<F>),
            );
            debug_assert_ne!(id, 0);

            PtpStatisticsCallback(NonZeroU64::new_unchecked(id as _))
        }
    }
}

#[derive(Debug)]
pub struct PtpStatisticsCallback(NonZeroU64);

impl PtpStatisticsCallback {
    #[doc(alias = "gst_ptp_statistics_callback_remove")]
    pub fn remove(self) {
        unsafe {
            ffi::gst_ptp_statistics_callback_remove(self.0.get() as _);
        }
    }
}
