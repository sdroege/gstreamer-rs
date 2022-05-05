// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{from_glib, ToGlibPtr};
use glib::{FlagsClass, StaticType, ToValue};
use gst::{bitflags_deserialize_impl, bitflags_serde_impl, bitflags_serialize_impl};

bitflags_serialize_impl!(crate::RTPBufferFlags, by_ones_decreasing);
bitflags_deserialize_impl!(crate::RTPBufferFlags);
bitflags_serde_impl!(crate::RTPBufferMapFlags);
// use this implementation, since serializing to "sendonly+recvonly"
// wouldn't make much sense
bitflags_serialize_impl!(
    crate::RTPHeaderExtensionDirection,
    by_ones_decreasing,
    "v1_20"
);
bitflags_deserialize_impl!(crate::RTPHeaderExtensionDirection, "v1_20");
bitflags_serde_impl!(crate::RTPHeaderExtensionFlags, "v1_20");

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

        check_serialize!(crate::RTPBufferFlags::all(), "\"retransmission+redundant\"");
        check_serialize!(crate::RTPBufferMapFlags::all(), "\"skip-padding\"");
        #[cfg(feature = "v1_20")]
        check_serialize!(
            crate::RTPHeaderExtensionDirection::all(),
            "\"sendrecv+inherited\""
        );
        #[cfg(feature = "v1_20")]
        check_serialize!(
            crate::RTPHeaderExtensionDirection::INACTIVE
                | crate::RTPHeaderExtensionDirection::SENDONLY
                | crate::RTPHeaderExtensionDirection::INHERITED,
            "\"sendonly+inherited\""
        );
        #[cfg(feature = "v1_20")]
        check_serialize!(
            crate::RTPHeaderExtensionFlags::all(),
            "\"one-byte+two-byte\""
        );
    }

    #[test]
    fn test_deserialize() {
        gst::init().unwrap();

        check_deserialize!(
            crate::RTPBufferFlags,
            crate::RTPBufferFlags::all(),
            "\"retransmission+redundant\""
        );
        check_deserialize!(
            crate::RTPBufferMapFlags,
            crate::RTPBufferMapFlags::all(),
            "\"skip-padding\""
        );
        #[cfg(feature = "v1_20")]
        check_deserialize!(
            crate::RTPHeaderExtensionDirection,
            crate::RTPHeaderExtensionDirection::all(),
            "\"sendrecv+inactive+inherited\""
        );
        #[cfg(feature = "v1_20")]
        check_deserialize!(
            crate::RTPHeaderExtensionFlags,
            crate::RTPHeaderExtensionFlags::all(),
            "\"one-byte+two-byte\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        gst::init().unwrap();

        check_roundtrip!(crate::RTPBufferFlags, crate::RTPBufferFlags::all());
        check_roundtrip!(crate::RTPBufferMapFlags, crate::RTPBufferMapFlags::all());
        #[cfg(feature = "v1_20")]
        check_roundtrip!(
            crate::RTPHeaderExtensionDirection,
            crate::RTPHeaderExtensionDirection::all()
        );
        #[cfg(feature = "v1_20")]
        check_roundtrip!(
            crate::RTPHeaderExtensionFlags,
            crate::RTPHeaderExtensionFlags::all()
        );
    }
}
