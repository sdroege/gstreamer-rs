// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    prelude::*,
    translate::{from_glib, ToGlibPtr},
    FlagsClass,
};
use gst::bitflags_serde_impl;

bitflags_serde_impl!(crate::RTSPTransportMode);

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

        check_serialize!(crate::RTSPTransportMode::all(), "\"play+record\"");
    }

    #[test]
    fn test_deserialize() {
        gst::init().unwrap();

        check_deserialize!(
            crate::RTSPTransportMode,
            crate::RTSPTransportMode::all(),
            "\"play+record\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        gst::init().unwrap();

        check_roundtrip!(crate::RTSPTransportMode, crate::RTSPTransportMode::all());
    }
}
