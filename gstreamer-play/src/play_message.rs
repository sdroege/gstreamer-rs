use crate::{PlayMediaInfo, PlayMessageType, PlayState};

#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
#[doc(alias = "GstPlayMessage")]
pub enum PlayMessage {
    #[doc(alias = "GST_PLAY_MESSAGE_URI_LOADED")]
    UriLoaded,
    #[doc(alias = "GST_PLAY_MESSAGE_POSITION_UPDATED")]
    PositionUpdated { position: Option<gst::ClockTime> },
    #[doc(alias = "GST_PLAY_MESSAGE_DURATION_CHANGED")]
    DurationChanged { duration: Option<gst::ClockTime> },
    #[doc(alias = "GST_PLAY_MESSAGE_STATE_CHANGED")]
    StateChanged { state: PlayState },
    #[doc(alias = "GST_PLAY_MESSAGE_BUFFERING")]
    Buffering { percent: u32 },
    #[doc(alias = "GST_PLAY_MESSAGE_END_OF_STREAM")]
    EndOfStream,
    #[doc(alias = "GST_PLAY_MESSAGE_ERROR")]
    Error {
        error: glib::Error,
        details: Option<gst::Structure>,
    },
    #[doc(alias = "GST_PLAY_MESSAGE_WARNING")]
    Warning {
        error: glib::Error,
        details: Option<gst::Structure>,
    },
    #[doc(alias = "GST_PLAY_MESSAGE_VIDEO_DIMENSIONS_CHANGED")]
    VideoDimensionsChanged { width: u32, height: u32 },
    #[doc(alias = "GST_PLAY_MESSAGE_MEDIA_INFO_UPDATED")]
    MediaInfoUpdated { info: PlayMediaInfo },
    #[doc(alias = "GST_PLAY_MESSAGE_VOLUME_CHANGED")]
    VolumeChanged { volume: f64 },
    #[doc(alias = "GST_PLAY_MESSAGE_MUTE_CHANGED")]
    MuteChanged { muted: bool },
    #[doc(alias = "GST_PLAY_MESSAGE_SEEK_DONE")]
    SeekDone,
}

impl PlayMessage {
    #[doc(alias = "gst_play_message_parse_position_updated")]
    #[doc(alias = "gst_play_message_parse_duration_updated")]
    #[doc(alias = "gst_play_message_parse_state_changed")]
    #[doc(alias = "gst_play_message_parse_buffering_percent")]
    #[doc(alias = "gst_play_message_parse_error")]
    #[doc(alias = "gst_play_message_parse_warning")]
    #[doc(alias = "gst_play_message_parse_video_dimensions_changed")]
    #[doc(alias = "gst_play_message_parse_media_info_updated")]
    #[doc(alias = "gst_play_message_parse_muted_changed")]
    #[doc(alias = "gst_play_message_parse_volume_changed")]
    pub fn parse(msg: &gst::Message) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        if msg.type_() != gst::MessageType::Application {
            return Err(glib::bool_error!("Invalid play message"));
        }
        match PlayMessageType::parse_type(msg) {
            PlayMessageType::UriLoaded => Ok(Self::UriLoaded),
            PlayMessageType::PositionUpdated => {
                let position = PlayMessageType::parse_position_updated(msg);
                Ok(Self::PositionUpdated { position })
            }
            PlayMessageType::DurationChanged => {
                let duration = PlayMessageType::parse_duration_updated(msg);
                Ok(Self::DurationChanged { duration })
            }
            PlayMessageType::StateChanged => {
                let state = PlayMessageType::parse_state_changed(msg);
                Ok(Self::StateChanged { state })
            }
            PlayMessageType::Buffering => {
                let percent = PlayMessageType::parse_buffering_percent(msg);
                Ok(Self::Buffering { percent })
            }
            PlayMessageType::EndOfStream => Ok(Self::EndOfStream),
            PlayMessageType::Error => {
                let (error, details) = PlayMessageType::parse_error(msg);
                Ok(Self::Error { error, details })
            }
            PlayMessageType::Warning => {
                let (error, details) = PlayMessageType::parse_warning(msg);
                Ok(Self::Warning { error, details })
            }
            PlayMessageType::VideoDimensionsChanged => {
                let (width, height) = PlayMessageType::parse_video_dimensions_changed(msg);
                Ok(Self::VideoDimensionsChanged { width, height })
            }
            PlayMessageType::MediaInfoUpdated => {
                let info = PlayMessageType::parse_media_info_updated(msg);
                Ok(Self::MediaInfoUpdated { info })
            }
            PlayMessageType::VolumeChanged => {
                let volume = PlayMessageType::parse_volume_changed(msg);
                Ok(Self::VolumeChanged { volume })
            }
            PlayMessageType::MuteChanged => {
                let muted = PlayMessageType::parse_muted_changed(msg);
                Ok(Self::MuteChanged { muted })
            }
            PlayMessageType::SeekDone => Ok(Self::SeekDone),
            _ => Err(glib::bool_error!("Invalid play message")),
        }
    }
}
