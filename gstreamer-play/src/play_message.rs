use crate::{Play, PlayMediaInfo, PlayMessageType, PlayState};

#[derive(Debug)]
#[non_exhaustive]
#[doc(alias = "GstPlayMessage")]
pub enum PlayMessage<'a> {
    #[doc(alias = "GST_PLAY_MESSAGE_URI_LOADED")]
    UriLoaded(&'a UriLoaded),
    #[doc(alias = "GST_PLAY_MESSAGE_POSITION_UPDATED")]
    PositionUpdated(&'a PositionUpdated),
    #[doc(alias = "GST_PLAY_MESSAGE_DURATION_CHANGED")]
    DurationChanged(&'a DurationChanged),
    #[doc(alias = "GST_PLAY_MESSAGE_STATE_CHANGED")]
    StateChanged(&'a StateChanged),
    #[doc(alias = "GST_PLAY_MESSAGE_BUFFERING")]
    Buffering(&'a Buffering),
    #[doc(alias = "GST_PLAY_MESSAGE_END_OF_STREAM")]
    EndOfStream(&'a EndOfStream),
    #[doc(alias = "GST_PLAY_MESSAGE_ERROR")]
    Error(&'a Error),
    #[doc(alias = "GST_PLAY_MESSAGE_WARNING")]
    Warning(&'a Warning),
    #[doc(alias = "GST_PLAY_MESSAGE_VIDEO_DIMENSIONS_CHANGED")]
    VideoDimensionsChanged(&'a VideoDimensionsChanged),
    #[doc(alias = "GST_PLAY_MESSAGE_MEDIA_INFO_UPDATED")]
    MediaInfoUpdated(&'a MediaInfoUpdated),
    #[doc(alias = "GST_PLAY_MESSAGE_VOLUME_CHANGED")]
    VolumeChanged(&'a VolumeChanged),
    #[doc(alias = "GST_PLAY_MESSAGE_MUTE_CHANGED")]
    MuteChanged(&'a MuteChanged),
    #[doc(alias = "GST_PLAY_MESSAGE_SEEK_DONE")]
    SeekDone(&'a SeekDone),
    Other(&'a Other),
}

macro_rules! declare_concrete_message(
    ($name:ident) => {
        #[repr(transparent)]
        pub struct $name<T = gst::MessageRef>(T);

        impl $name {
            #[inline]
            pub fn message(&self) -> &gst::MessageRef {
                unsafe { &*(self as *const Self as *const gst::MessageRef) }
            }

            #[inline]
            unsafe fn view(message: &gst::MessageRef) -> PlayMessage<'_> {
                let message = &*(message as *const gst::MessageRef as *const Self);
                PlayMessage::$name(message)
            }
        }

        impl std::ops::Deref for $name {
            type Target = gst::MessageRef;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe {
                    &*(self as *const Self as *const Self::Target)
                }
            }
        }

        impl ToOwned for $name {
            type Owned = $name<gst::Message>;

            #[inline]
            fn to_owned(&self) -> Self::Owned {
                $name::<gst::Message>(self.copy())
            }
        }

        impl std::ops::Deref for $name<gst::Message> {
            type Target = $name;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.0.as_ptr() as *const Self::Target) }
            }
        }

        impl std::borrow::Borrow<$name> for $name<gst::Message> {
            #[inline]
            fn borrow(&self) -> &$name {
                &*self
            }
        }

        impl From<$name<gst::Message>> for gst::Message {
            #[inline]
            fn from(concrete: $name<gst::Message>) -> Self {
                skip_assert_initialized!();
                concrete.0
            }
        }
    }
);

declare_concrete_message!(UriLoaded);
impl UriLoaded {
    pub fn uri(&self) -> &glib::GStr {
        self.message().structure().unwrap().get("uri").unwrap()
    }
}
impl std::fmt::Debug for UriLoaded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UriLoaded")
            .field("structure", &self.message().structure())
            .field("uri", &self.uri())
            .finish()
    }
}

declare_concrete_message!(PositionUpdated);
impl PositionUpdated {
    pub fn position(&self) -> Option<gst::ClockTime> {
        self.message().structure().unwrap().get("position").unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for PositionUpdated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PositionUpdated")
            .field("structure", &self.message().structure())
            .field("position", &self.position())
            .finish()
    }
}

declare_concrete_message!(DurationChanged);
impl DurationChanged {
    pub fn duration(&self) -> Option<gst::ClockTime> {
        self.message().structure().unwrap().get("duration").unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for DurationChanged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DurationChanged")
            .field("structure", &self.message().structure())
            .field("duration", &self.duration())
            .finish()
    }
}

declare_concrete_message!(StateChanged);
impl StateChanged {
    pub fn state(&self) -> PlayState {
        self.message()
            .structure()
            .unwrap()
            .get("play-state")
            .unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for StateChanged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateChanged")
            .field("structure", &self.message().structure())
            .field("state", &self.state())
            .finish()
    }
}

declare_concrete_message!(Buffering);
impl Buffering {
    pub fn percent(&self) -> u32 {
        self.message()
            .structure()
            .unwrap()
            // Typo in the library
            .get("bufferring-percent")
            .unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for Buffering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffering")
            .field("structure", &self.message().structure())
            .field("percent", &self.percent())
            .finish()
    }
}

declare_concrete_message!(EndOfStream);
impl std::fmt::Debug for EndOfStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EndOfStream")
            .field("structure", &self.message().structure())
            .finish()
    }
}

declare_concrete_message!(Error);
impl Error {
    pub fn error(&self) -> &glib::Error {
        self.message().structure().unwrap().get("error").unwrap()
    }

    pub fn details(&self) -> Option<&gst::StructureRef> {
        self.message()
            .structure()
            .unwrap()
            .get_optional("error-details")
            .unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_stream_id")]
    pub fn stream_id(&self) -> Option<glib::GString> {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_stream_id(mut_override(
                self.message().as_ptr(),
            )))
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_parse_error_missing_plugin")]
    pub fn missing_plugin(&self) -> Option<Vec<(glib::GString, glib::GString)>> {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            let mut descriptions = std::ptr::null_mut();
            let mut installer_details = std::ptr::null_mut();
            let ret = from_glib(ffi::gst_play_message_parse_error_missing_plugin(
                mut_override(self.message().as_ptr()),
                &mut descriptions,
                &mut installer_details,
            ));
            if ret {
                let mut ret = Vec::new();
                for idx in 0.. {
                    let description = *descriptions.add(idx);
                    let installer_detail = *installer_details.add(idx);

                    if description.is_null() {
                        assert!(installer_detail.is_null());
                        break;
                    }

                    ret.push((
                        glib::GString::from_glib_full(description),
                        glib::GString::from_glib_full(installer_detail),
                    ));
                }
                glib::ffi::g_free(descriptions as glib::ffi::gpointer);
                glib::ffi::g_free(installer_details as glib::ffi::gpointer);

                Some(ret)
            } else {
                None
            }
        }
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Error")
            .field("structure", &self.message().structure())
            .field("error", &self.error())
            .field("details", &self.details())
            .finish()
    }
}

declare_concrete_message!(Warning);
impl Warning {
    pub fn error(&self) -> &glib::Error {
        self.message().structure().unwrap().get("warning").unwrap()
    }

    pub fn details(&self) -> Option<&gst::StructureRef> {
        self.message()
            .structure()
            .unwrap()
            .get_optional("warning-details")
            .unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_stream_id")]
    pub fn stream_id(&self) -> Option<glib::GString> {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_stream_id(mut_override(
                self.message().as_ptr(),
            )))
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_parse_warning_missing_plugin")]
    pub fn missing_plugin(&self) -> Option<Vec<(glib::GString, glib::GString)>> {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            let mut descriptions = std::ptr::null_mut();
            let mut installer_details = std::ptr::null_mut();
            let ret = from_glib(ffi::gst_play_message_parse_warning_missing_plugin(
                mut_override(self.message().as_ptr()),
                &mut descriptions,
                &mut installer_details,
            ));
            if ret {
                let mut ret = Vec::new();
                for idx in 0.. {
                    let description = *descriptions.add(idx);
                    let installer_detail = *installer_details.add(idx);

                    if description.is_null() {
                        assert!(installer_detail.is_null());
                        break;
                    }

                    ret.push((
                        glib::GString::from_glib_full(description),
                        glib::GString::from_glib_full(installer_detail),
                    ));
                }
                glib::ffi::g_free(descriptions as glib::ffi::gpointer);
                glib::ffi::g_free(installer_details as glib::ffi::gpointer);

                Some(ret)
            } else {
                None
            }
        }
    }
}
impl std::fmt::Debug for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Warning")
            .field("structure", &self.message().structure())
            .field("error", &self.error())
            .field("details", &self.details())
            .finish()
    }
}

declare_concrete_message!(VideoDimensionsChanged);
impl VideoDimensionsChanged {
    pub fn width(&self) -> u32 {
        self.message()
            .structure()
            .unwrap()
            .get("video-width")
            .unwrap()
    }

    pub fn height(&self) -> u32 {
        self.message()
            .structure()
            .unwrap()
            .get("video-height")
            .unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for VideoDimensionsChanged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VideoDimensionsChanged")
            .field("structure", &self.message().structure())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

declare_concrete_message!(MediaInfoUpdated);
impl MediaInfoUpdated {
    pub fn media_info(&self) -> &PlayMediaInfo {
        self.message()
            .structure()
            .unwrap()
            .get("media-info")
            .unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for MediaInfoUpdated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MediaInfoUpdated")
            .field("structure", &self.message().structure())
            .field("media_info", &self.media_info())
            .finish()
    }
}

declare_concrete_message!(VolumeChanged);
impl VolumeChanged {
    pub fn volume(&self) -> f64 {
        self.message().structure().unwrap().get("volume").unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for VolumeChanged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VolumeChanged")
            .field("structure", &self.message().structure())
            .field("volume", &self.volume())
            .finish()
    }
}

declare_concrete_message!(MuteChanged);
impl MuteChanged {
    pub fn is_muted(&self) -> bool {
        self.message().structure().unwrap().get("is-muted").unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for MuteChanged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MuteChanged")
            .field("structure", &self.message().structure())
            .field("is_muted", &self.is_muted())
            .finish()
    }
}

declare_concrete_message!(SeekDone);
impl SeekDone {
    pub fn position(&self) -> Option<gst::ClockTime> {
        self.message().structure().unwrap().get("position").unwrap()
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}
impl std::fmt::Debug for SeekDone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SeekDone")
            .field("structure", &self.message().structure())
            .field("position", &self.position())
            .finish()
    }
}

declare_concrete_message!(Other);

impl Other {
    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_play_message_get_uri")]
    pub fn uri(&self) -> glib::GString {
        use crate::ffi;
        use glib::translate::*;

        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_play_message_get_uri(mut_override(
                self.message().as_ptr(),
            )))
        }
    }
}

impl std::fmt::Debug for Other {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Other")
            .field("structure", &self.message().structure())
            .finish()
    }
}

impl PlayMessage<'_> {
    #[doc(alias = "gst_play_message_parse_uri_loaded")]
    #[doc(alias = "gst_play_message_parse_position_updated")]
    #[doc(alias = "gst_play_message_parse_duration_updated")]
    #[doc(alias = "gst_play_message_parse_duration_changed")]
    #[doc(alias = "gst_play_message_parse_state_changed")]
    #[doc(alias = "gst_play_message_parse_buffering")]
    #[doc(alias = "gst_play_message_parse_buffering_percent")]
    #[doc(alias = "gst_play_message_parse_error")]
    #[doc(alias = "gst_play_message_parse_warning")]
    #[doc(alias = "gst_play_message_parse_video_dimensions_changed")]
    #[doc(alias = "gst_play_message_parse_media_info_updated")]
    #[doc(alias = "gst_play_message_parse_muted_changed")]
    #[doc(alias = "gst_play_message_parse_volume_changed")]
    #[doc(alias = "gst_play_message_parse_seek_done")]
    pub fn parse(msg: &gst::Message) -> Result<PlayMessage<'_>, glib::error::BoolError> {
        skip_assert_initialized!();

        if !Play::is_play_message(msg) {
            return Err(glib::bool_error!("Invalid play message"));
        }

        unsafe {
            match PlayMessageType::parse_type(msg) {
                PlayMessageType::UriLoaded => Ok(UriLoaded::view(msg)),
                PlayMessageType::PositionUpdated => Ok(PositionUpdated::view(msg)),
                PlayMessageType::DurationChanged => Ok(DurationChanged::view(msg)),
                PlayMessageType::StateChanged => Ok(StateChanged::view(msg)),
                PlayMessageType::Buffering => Ok(Buffering::view(msg)),
                PlayMessageType::EndOfStream => Ok(EndOfStream::view(msg)),
                PlayMessageType::Error => Ok(Error::view(msg)),
                PlayMessageType::Warning => Ok(Warning::view(msg)),
                PlayMessageType::VideoDimensionsChanged => Ok(VideoDimensionsChanged::view(msg)),
                PlayMessageType::MediaInfoUpdated => Ok(MediaInfoUpdated::view(msg)),
                PlayMessageType::VolumeChanged => Ok(VolumeChanged::view(msg)),
                PlayMessageType::MuteChanged => Ok(MuteChanged::view(msg)),
                PlayMessageType::SeekDone => Ok(SeekDone::view(msg)),
                _ => Ok(Other::view(msg)),
            }
        }
    }
}
