// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    prelude::*,
    translate::{from_glib, ToGlibPtr},
    FlagsClass,
};
use gst::bitflags_serde_impl;

bitflags_serde_impl!(crate::MarkerFlags, "v1_20");
bitflags_serde_impl!(crate::MetaFlag);
bitflags_serde_impl!(crate::PipelineFlags);
bitflags_serde_impl!(crate::TrackType);

#[cfg(test)]
mod tests {
    macro_rules! check_serialize {
        ($flags:expr, $expected:expr) => {
            let actual = serde_json::to_string(&$flags).unwrap();
            assert_eq!(actual, $expected);
        };
    }

    macro_rules! check_deserialize {
        ($ty:ty, $expected:expr, $json:expr) => {
            let actual: $ty = serde_json::from_str(&$json).unwrap();
            assert_eq!(actual, $expected);
        };
    }

    macro_rules! check_roundtrip {
        ($ty:ty, $flags:expr) => {
            let json = serde_json::to_string(&$flags).unwrap();
            let deserialized: $ty = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, $flags);
        };
    }

    #[test]
    fn test_serialize() {
        gst::init().unwrap();

        #[cfg(feature = "v1_20")]
        check_serialize!(crate::MarkerFlags::all(), "\"snappable\"");
        check_serialize!(crate::MetaFlag::all(), "\"readable+writable\"");
        check_serialize!(
            crate::PipelineFlags::all(),
            "\"audio_preview+video_preview+render+smart_render\""
        );
        check_serialize!(
            crate::TrackType::all(),
            "\"unknown+audio+video+text+custom\""
        );
    }

    #[test]
    fn test_deserialize() {
        gst::init().unwrap();

        #[cfg(feature = "v1_20")]
        check_deserialize!(
            crate::MarkerFlags,
            crate::MarkerFlags::all(),
            "\"snappable\""
        );
        check_deserialize!(
            crate::MetaFlag,
            crate::MetaFlag::all(),
            "\"readable+writable\""
        );
        check_deserialize!(
            crate::PipelineFlags,
            crate::PipelineFlags::all(),
            "\"audio_preview+video_preview+render+smart_render\""
        );
        check_deserialize!(
            crate::TrackType,
            crate::TrackType::all(),
            "\"unknown+audio+video+text+custom\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        gst::init().unwrap();

        #[cfg(feature = "v1_20")]
        check_roundtrip!(crate::MarkerFlags, crate::MarkerFlags::all());
        check_roundtrip!(crate::MetaFlag, crate::MetaFlag::all());
        check_roundtrip!(crate::PipelineFlags, crate::PipelineFlags::all());
        check_roundtrip!(crate::TrackType, crate::TrackType::all());
    }
}
