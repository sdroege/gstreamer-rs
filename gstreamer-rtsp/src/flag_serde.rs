// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    translate::{from_glib, ToGlibPtr},
    FlagsClass, StaticType, ToValue,
};
use gst::bitflags_serde_impl;

bitflags_serde_impl!(crate::RTSPEvent);
bitflags_serde_impl!(crate::RTSPLowerTrans);
bitflags_serde_impl!(crate::RTSPMethod);
bitflags_serde_impl!(crate::RTSPProfile);
bitflags_serde_impl!(crate::RTSPTransMode);

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

        check_serialize!(crate::RTSPEvent::all(), "\"read+write\"");
        check_serialize!(
            crate::RTSPLowerTrans::all(),
            "\"udp+udp-mcast+tcp+http+tls\""
        );
        check_serialize!(
            crate::RTSPMethod::all(),
            concat!(
                "\"describe+announce+get-parameter+options+pause+play+record",
                "+redirect+setup+set-parameter+teardown+get+post\""
            )
        );
        check_serialize!(crate::RTSPProfile::all(), "\"avp+savp+avpf+savpf\"");
        check_serialize!(crate::RTSPTransMode::all(), "\"rtp+rdt\"");
    }

    #[test]
    fn test_deserialize() {
        gst::init().unwrap();

        check_deserialize!(crate::RTSPEvent, crate::RTSPEvent::all(), "\"read+write\"");
        check_deserialize!(
            crate::RTSPLowerTrans,
            crate::RTSPLowerTrans::all(),
            "\"udp+udp-mcast+tcp+http+tls\""
        );
        check_deserialize!(
            crate::RTSPMethod,
            crate::RTSPMethod::all(),
            concat!(
                "\"describe+announce+get-parameter+options+pause+play+record",
                "+redirect+setup+set-parameter+teardown+get+post\""
            )
        );
        check_deserialize!(
            crate::RTSPProfile,
            crate::RTSPProfile::all(),
            "\"avp+savp+avpf+savpf\""
        );
        check_deserialize!(
            crate::RTSPTransMode,
            crate::RTSPTransMode::all(),
            "\"rtp+rdt\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        gst::init().unwrap();

        check_roundtrip!(crate::RTSPEvent, crate::RTSPEvent::all());
        check_roundtrip!(crate::RTSPLowerTrans, crate::RTSPLowerTrans::all());
        check_roundtrip!(crate::RTSPMethod, crate::RTSPMethod::all());
        check_roundtrip!(crate::RTSPProfile, crate::RTSPProfile::all());
        check_roundtrip!(crate::RTSPTransMode, crate::RTSPTransMode::all());
    }
}
