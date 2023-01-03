// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    translate::{from_glib, ToGlibPtr},
    FlagsClass, StaticType, ToValue,
};
use gst::bitflags_serde_impl;

bitflags_serde_impl!(crate::GLAPI);
bitflags_serde_impl!(crate::GLConfigSurfaceType, "v1_20");
bitflags_serde_impl!(crate::GLDisplayType);
bitflags_serde_impl!(crate::GLPlatform);
bitflags_serde_impl!(crate::GLSLProfile);

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

        check_serialize!(crate::GLAPI::all(), "\"opengl+opengl3+gles1+gles2\"");
        #[cfg(feature = "v1_20")]
        check_serialize!(
            crate::GLConfigSurfaceType::all(),
            "\"window+pbuffer+pixmap\""
        );
        #[cfg(feature = "v1_20")]
        check_serialize!(
            crate::GLDisplayType::all(),
            concat!(
                "\"x11+wayland+cocoa+win32+dispmanx+egl+viv-fb+gbm+egl-device",
                "+eagl+winrt+android\""
            )
        );
        check_serialize!(crate::GLPlatform::all(), "\"egl+glx+wgl+cgl+eagl\"");
        check_serialize!(crate::GLSLProfile::all(), "\"es+core+compatibility\"");
    }

    #[test]
    fn test_deserialize() {
        gst::init().unwrap();

        check_deserialize!(
            crate::GLAPI,
            crate::GLAPI::all(),
            "\"opengl+opengl3+gles1+gles2\""
        );
        #[cfg(feature = "v1_20")]
        check_deserialize!(
            crate::GLConfigSurfaceType,
            crate::GLConfigSurfaceType::all(),
            "\"none+window+pbuffer+pixmap\""
        );
        #[cfg(feature = "v1_20")]
        check_deserialize!(
            crate::GLDisplayType,
            crate::GLDisplayType::all(),
            concat!(
                "\"x11+wayland+cocoa+win32+dispmanx+egl+viv-fb+gbm+egl-device",
                "+eagl+winrt+android\""
            )
        );
        check_deserialize!(
            crate::GLPlatform,
            crate::GLPlatform::all(),
            "\"egl+glx+wgl+cgl+eagl\""
        );
        check_deserialize!(
            crate::GLSLProfile,
            crate::GLSLProfile::all(),
            "\"es+core+compatibility\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        gst::init().unwrap();

        check_roundtrip!(crate::GLAPI, crate::GLAPI::all());
        #[cfg(feature = "v1_20")]
        check_roundtrip!(
            crate::GLConfigSurfaceType,
            crate::GLConfigSurfaceType::all()
        );
        #[cfg(feature = "v1_20")]
        check_roundtrip!(crate::GLDisplayType, crate::GLDisplayType::all());
        check_roundtrip!(crate::GLPlatform, crate::GLPlatform::all());
        check_roundtrip!(crate::GLSLProfile, crate::GLSLProfile::all());
    }
}
