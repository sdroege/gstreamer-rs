// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{from_glib, ToGlibPtr};
use glib::{FlagsClass, StaticType, ToValue};
use gst::bitflags_serde_impl;

bitflags_serde_impl!(crate::DiscovererSerializeFlags);
bitflags_serde_impl!(crate::PbUtilsCapsDescriptionFlags, "v1_20");

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

        check_serialize!(crate::DiscovererSerializeFlags::all(), "\"caps+tags+misc\"");
        #[cfg(feature = "v1_22")]
        check_serialize!(
            crate::PbUtilsCapsDescriptionFlags::all(),
            "\"container+audio+video+image+subtitle+tag+generic+metadata\""
        );
        #[cfg(all(feature = "v1_20", not(feature = "v1_22")))]
        check_serialize!(
            crate::PbUtilsCapsDescriptionFlags::all(),
            "\"container+audio+video+image+subtitle+tag+generic\""
        );
    }

    #[test]
    fn test_deserialize() {
        gst::init().unwrap();

        check_deserialize!(
            crate::DiscovererSerializeFlags,
            crate::DiscovererSerializeFlags::all(),
            "\"caps+tags+misc\""
        );
        #[cfg(feature = "v1_22")]
        check_deserialize!(
            crate::PbUtilsCapsDescriptionFlags,
            crate::PbUtilsCapsDescriptionFlags::all(),
            "\"container+audio+video+image+subtitle+tag+generic+metadata\""
        );
        #[cfg(all(feature = "v1_20", not(feature = "v1_22")))]
        check_deserialize!(
            crate::PbUtilsCapsDescriptionFlags,
            crate::PbUtilsCapsDescriptionFlags::all(),
            "\"container+audio+video+image+subtitle+tag+generic\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        gst::init().unwrap();

        check_roundtrip!(
            crate::DiscovererSerializeFlags,
            crate::DiscovererSerializeFlags::all()
        );
        #[cfg(feature = "v1_20")]
        check_roundtrip!(
            crate::PbUtilsCapsDescriptionFlags,
            crate::PbUtilsCapsDescriptionFlags::all()
        );
    }
}
