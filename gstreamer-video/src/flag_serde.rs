// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    translate::{from_glib, ToGlibPtr},
    FlagsClass, StaticType, ToValue,
};
use gst::bitflags_serde_impl;

bitflags_serde_impl!(crate::NavigationModifierType, "v1_22");
bitflags_serde_impl!(crate::VideoBufferFlags);
bitflags_serde_impl!(crate::VideoChromaSite);
bitflags_serde_impl!(crate::VideoCodecFrameFlags, "v1_20");
bitflags_serde_impl!(crate::VideoDecoderRequestSyncPointFlags, "v1_20");
bitflags_serde_impl!(crate::VideoFlags);
bitflags_serde_impl!(crate::VideoFormatFlags);
bitflags_serde_impl!(crate::VideoFrameFlags);
bitflags_serde_impl!(crate::VideoMultiviewFlags);
bitflags_serde_impl!(crate::VideoOverlayFormatFlags, "v1_16");
bitflags_serde_impl!(crate::VideoPackFlags);
bitflags_serde_impl!(crate::VideoTimeCodeFlags, "v1_18");

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

        #[cfg(feature = "v1_22")]
        check_serialize!(
            crate::NavigationModifierType::all(),
            concat!(
                "\"shift-mask+lock-mask+control-mask+mod1-mask+mod2-mask+mod3-mask",
                "+mod4-mask+mod5-mask+button1-mask+button2-mask+button3-mask",
                "+button4-mask+button5-mask+super-mask+hyper-mask+meta-mask\""
            )
        );
        #[cfg(feature = "v1_18")]
        check_serialize!(
            crate::VideoBufferFlags::all(),
            "\"interlaced+tff+rff+onefield+multiple-view+first-in-bundle+marker\""
        );
        check_serialize!(
            crate::VideoChromaSite::all(),
            "\"none+h-cosited+v-cosited+alt-line\""
        );
        #[cfg(feature = "v1_20")]
        check_serialize!(
            crate::VideoCodecFrameFlags::all(),
            "\"decode-only+sync-point+force-keyframe+force-keyframe-headers+corrupted\""
        );
        #[cfg(feature = "v1_20")]
        check_serialize!(
            crate::VideoDecoderRequestSyncPointFlags::all(),
            "\"discard-input+corrupt-output\""
        );
        check_serialize!(
            crate::VideoFlags::all(),
            "\"variable-fps+premultiplied-alpha\""
        );
        #[cfg(feature = "v1_22")]
        check_serialize!(
            crate::VideoFormatFlags::all(),
            "\"yuv+rgb+gray+alpha+le+palette+complex+unpack+tiled+subtiles\""
        );
        check_serialize!(
            crate::VideoFrameFlags::all(),
            "\"interlaced+tff+rff+onefield+multiple-view+first-in-bundle\""
        );
        check_serialize!(
            crate::VideoMultiviewFlags::all(),
            concat!(
                "\"right-view-first+left-flipped+left-flopped+right-flipped",
                "+right-flopped+half-aspect+mixed-mono\""
            )
        );
        #[cfg(feature = "v1_16")]
        check_serialize!(
            crate::VideoOverlayFormatFlags::all(),
            "\"premultiplied-alpha+global-alpha\""
        );
        check_serialize!(
            crate::VideoPackFlags::all(),
            "\"truncate-range+interlaced\""
        );
        #[cfg(feature = "v1_18")]
        check_serialize!(
            crate::VideoTimeCodeFlags::all(),
            "\"drop-frame+interlaced\""
        );
    }

    #[test]
    fn test_deserialize() {
        gst::init().unwrap();

        #[cfg(feature = "v1_22")]
        check_deserialize!(
            crate::NavigationModifierType,
            crate::NavigationModifierType::all(),
            concat!(
                "\"shift-mask+lock-mask+control-mask+mod1-mask+mod2-mask",
                "+mod3-mask+mod4-mask+mod5-mask+button1-mask",
                "+button2-mask+button3-mask+button4-mask+button5-mask",
                "+super-mask+hyper-mask+meta-mask\""
            )
        );
        #[cfg(feature = "v1_18")]
        check_deserialize!(
            crate::VideoBufferFlags,
            crate::VideoBufferFlags::all(),
            "\"interlaced+tff+rff+onefield+multiple-view+first-in-bundle+marker\""
        );
        check_deserialize!(
            crate::VideoChromaSite,
            crate::VideoChromaSite::all(),
            "\"none+h-cosited+v-cosited+alt-line\""
        );
        #[cfg(feature = "v1_20")]
        check_deserialize!(
            crate::VideoCodecFrameFlags,
            crate::VideoCodecFrameFlags::all(),
            "\"decode-only+sync-point+force-keyframe+force-keyframe-headers+corrupted\""
        );
        #[cfg(feature = "v1_20")]
        check_deserialize!(
            crate::VideoDecoderRequestSyncPointFlags,
            crate::VideoDecoderRequestSyncPointFlags::all(),
            "\"discard-input+corrupt-output\""
        );
        check_deserialize!(
            crate::VideoFlags,
            crate::VideoFlags::all(),
            "\"variable-fps+premultiplied-alpha\""
        );
        #[cfg(feature = "v1_22")]
        check_deserialize!(
            crate::VideoFormatFlags,
            crate::VideoFormatFlags::all(),
            "\"yuv+rgb+gray+alpha+le+palette+complex+unpack+tiled+subtiles\""
        );
        check_deserialize!(
            crate::VideoFrameFlags,
            crate::VideoFrameFlags::all(),
            "\"interlaced+tff+rff+onefield+multiple-view+first-in-bundle\""
        );
        check_deserialize!(
            crate::VideoMultiviewFlags,
            crate::VideoMultiviewFlags::all(),
            concat!(
                "\"right-view-first+left-flipped+left-flopped+right-flipped",
                "+right-flopped+half-aspect+mixed-mono\""
            )
        );
        #[cfg(feature = "v1_16")]
        check_deserialize!(
            crate::VideoOverlayFormatFlags,
            crate::VideoOverlayFormatFlags::all(),
            "\"premultiplied-alpha+global-alpha\""
        );
        check_deserialize!(
            crate::VideoPackFlags,
            crate::VideoPackFlags::all(),
            "\"truncate-range+interlaced\""
        );
        #[cfg(feature = "v1_18")]
        check_deserialize!(
            crate::VideoTimeCodeFlags,
            crate::VideoTimeCodeFlags::all(),
            "\"drop-frame+interlaced\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        gst::init().unwrap();

        #[cfg(feature = "v1_22")]
        check_roundtrip!(
            crate::NavigationModifierType,
            crate::NavigationModifierType::all()
        );
        #[cfg(feature = "v1_18")]
        check_roundtrip!(crate::VideoBufferFlags, crate::VideoBufferFlags::all());
        check_roundtrip!(crate::VideoChromaSite, crate::VideoChromaSite::all());
        #[cfg(feature = "v1_20")]
        check_roundtrip!(
            crate::VideoCodecFrameFlags,
            crate::VideoCodecFrameFlags::all()
        );
        #[cfg(feature = "v1_20")]
        check_roundtrip!(
            crate::VideoDecoderRequestSyncPointFlags,
            crate::VideoDecoderRequestSyncPointFlags::all()
        );
        check_roundtrip!(crate::VideoFlags, crate::VideoFlags::all());
        #[cfg(feature = "v1_22")]
        check_roundtrip!(crate::VideoFormatFlags, crate::VideoFormatFlags::all());
        check_roundtrip!(crate::VideoFrameFlags, crate::VideoFrameFlags::all());
        check_roundtrip!(
            crate::VideoMultiviewFlags,
            crate::VideoMultiviewFlags::all()
        );
        #[cfg(feature = "v1_16")]
        check_roundtrip!(
            crate::VideoOverlayFormatFlags,
            crate::VideoOverlayFormatFlags::all()
        );
        check_roundtrip!(crate::VideoPackFlags, crate::VideoPackFlags::all());
        #[cfg(feature = "v1_18")]
        check_roundtrip!(crate::VideoTimeCodeFlags, crate::VideoTimeCodeFlags::all());
    }
}
