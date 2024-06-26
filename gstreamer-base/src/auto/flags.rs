// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::{bitflags::bitflags, translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstBaseParseFrameFlags")]
    pub struct BaseParseFrameFlags: u32 {
        #[doc(alias = "GST_BASE_PARSE_FRAME_FLAG_NEW_FRAME")]
        const NEW_FRAME = ffi::GST_BASE_PARSE_FRAME_FLAG_NEW_FRAME as _;
        #[doc(alias = "GST_BASE_PARSE_FRAME_FLAG_NO_FRAME")]
        const NO_FRAME = ffi::GST_BASE_PARSE_FRAME_FLAG_NO_FRAME as _;
        #[doc(alias = "GST_BASE_PARSE_FRAME_FLAG_CLIP")]
        const CLIP = ffi::GST_BASE_PARSE_FRAME_FLAG_CLIP as _;
        #[doc(alias = "GST_BASE_PARSE_FRAME_FLAG_DROP")]
        const DROP = ffi::GST_BASE_PARSE_FRAME_FLAG_DROP as _;
        #[doc(alias = "GST_BASE_PARSE_FRAME_FLAG_QUEUE")]
        const QUEUE = ffi::GST_BASE_PARSE_FRAME_FLAG_QUEUE as _;
    }
}

#[doc(hidden)]
impl IntoGlib for BaseParseFrameFlags {
    type GlibType = ffi::GstBaseParseFrameFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstBaseParseFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstBaseParseFrameFlags> for BaseParseFrameFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstBaseParseFrameFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}
