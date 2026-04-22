#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]

use std::sync::Once;

pub use glib;
#[cfg(feature = "v1_20")]
use glib::translate::ToGlibPtr;
pub use gst;
pub use gstreamer_mpegts_sys as ffi;

static MPEGTS_INIT: Once = Once::new();

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
        crate::MPEGTS_INIT.call_once(|| unsafe { crate::ffi::gst_mpegts_initialize() });
    };
}

pub fn init() {
    assert_initialized_main_thread!();
}

#[allow(unused_imports)]
mod auto;
#[allow(unused_imports)]
pub use crate::auto::*;

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(alias = "gst_event_new_mpegts_section")]
pub fn event_new_mpegts_section(section: &Section) -> gst::Event {
    assert_initialized_main_thread!();

    unsafe {
        let event = ffi::gst_event_new_mpegts_section(section.to_glib_none().0);
        glib::translate::from_glib_full(event)
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(alias = "gst_message_new_mpegts_section")]
pub fn message_new_mpegts_section(parent: &gst::Object, section: &Section) -> gst::Message {
    assert_initialized_main_thread!();

    unsafe {
        let message =
            ffi::gst_message_new_mpegts_section(parent.to_glib_none().0, section.to_glib_none().0);
        glib::translate::from_glib_full(message)
    }
}

// rustdoc-stripper-ignore-next
/// Parses a #Section from a #Event.
///
/// # Arguments
///
/// * `event` - The #Event to parse
///
/// # Returns
///
/// A `Section` if the event contains MPEG-TS section data, else `None`
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(alias = "gst_event_parse_mpegts_section")]
pub fn event_parse_mpegts_section(event: &gst::Event) -> Option<Section> {
    assert_initialized_main_thread!();

    unsafe {
        let section = ffi::gst_event_parse_mpegts_section(event.to_glib_none().0);
        glib::translate::from_glib_full(section)
    }
}

// rustdoc-stripper-ignore-next
/// Parses a #Section from a #Message.
///
/// Returns a `Section` if the message contains MPEG-TS section data, else
/// `None`
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(alias = "gst_message_parse_mpegts_section")]
pub fn message_parse_mpegts_section(message: &gst::Message) -> Option<Section> {
    assert_initialized_main_thread!();

    unsafe {
        let section = ffi::gst_message_parse_mpegts_section(message.to_glib_none().0);
        glib::translate::from_glib_full(section)
    }
}

#[cfg(feature = "v1_20")]
impl Section {
    // rustdoc-stripper-ignore-next
    /// Creates a new #Section from the provided data.
    ///
    /// # Arguments
    ///
    /// * `pid` - The PID to which this section belongs
    /// * `data` - A slice containing the section data (must start with table_id)
    ///
    /// # Returns
    ///
    /// A new `Section` if the data was valid, else `None`
    #[doc(alias = "gst_mpegts_section_new")]
    pub fn new(pid: u16, data: &[u8]) -> Option<Section> {
        assert_initialized_main_thread!();
        unsafe {
            let len = data.len();
            let ptr = data.to_glib_full();
            let section = ffi::gst_mpegts_section_new(pid, ptr, len);
            glib::translate::from_glib_full(section)
        }
    }
}
