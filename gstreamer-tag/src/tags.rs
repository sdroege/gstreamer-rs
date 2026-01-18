use gst::glib::translate::*;
use std::ptr;

pub struct ExtendedComment {
    pub key: Option<glib::GString>,
    pub lang: Option<glib::GString>,
    pub value: glib::GString,
}

#[doc(alias = "gst_tag_parse_extended_comment")]
pub fn tag_parse_extended_comment(
    ext_comment: &str,
    fail_if_no_key: bool,
) -> Result<ExtendedComment, gst::glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        let mut c_key = ptr::null_mut();
        let mut c_lang = ptr::null_mut();
        let mut c_value = ptr::null_mut();
        let res: bool = from_glib(crate::ffi::gst_tag_parse_extended_comment(
            ext_comment.to_glib_none().0,
            &mut c_key,
            &mut c_lang,
            &mut c_value,
            fail_if_no_key.into_glib(),
        ));
        if !res {
            Err(glib::bool_error!("Failed to parse extended comment"))
        } else {
            let key = from_glib_full(c_key);
            let lang = from_glib_full(c_lang);
            let value = from_glib_full(c_value);

            Ok(ExtendedComment { key, lang, value })
        }
    }
}

#[inline]
fn ensure_tags_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    skip_assert_initialized!();
    ONCE.call_once(
        #[inline]
        || {
            unsafe { crate::ffi::gst_tag_register_musicbrainz_tags() };
        },
    );
}

macro_rules! impl_tag(
    ($name:ident, $t:ty, $gst_tag:ident) => {
        pub enum $name {}
        impl<'a> gst::Tag<'a> for $name {
            type TagType = $t;
            const TAG_NAME: &'static glib::GStr = unsafe { glib::GStr::from_utf8_with_nul_unchecked($crate::ffi::$gst_tag) };
            #[inline]
            fn ensure() {
                crate::tags::ensure_tags_registered();
            }
        }
    };
);

pub mod acoustid {
    impl_tag!(Fingerprint, &'a str, GST_TAG_ACOUSTID_FINGERPRINT);
    impl_tag!(Id, &'a str, GST_TAG_ACOUSTID_ID);
}

pub mod capturing {
    impl_tag!(Contrast, &'a str, GST_TAG_CAPTURING_CONTRAST);
    impl_tag!(DigitalZoomRatio, f64, GST_TAG_CAPTURING_DIGITAL_ZOOM_RATIO);
    impl_tag!(
        ExposureCompensation,
        f64,
        GST_TAG_CAPTURING_EXPOSURE_COMPENSATION
    );
    impl_tag!(ExposureMode, &'a str, GST_TAG_CAPTURING_EXPOSURE_MODE);
    impl_tag!(ExposureProgram, &'a str, GST_TAG_CAPTURING_EXPOSURE_PROGRAM);
    impl_tag!(FlashFired, bool, GST_TAG_CAPTURING_FLASH_FIRED);
    impl_tag!(FlashMode, &'a str, GST_TAG_CAPTURING_FLASH_MODE);
    impl_tag!(FocalLength, f64, GST_TAG_CAPTURING_FOCAL_LENGTH);
    impl_tag!(FocalLength35MM, f64, GST_TAG_CAPTURING_FOCAL_LENGTH_35_MM);
    impl_tag!(FocalRatio, f64, GST_TAG_CAPTURING_FOCAL_RATIO);
    impl_tag!(GainAdjustment, &'a str, GST_TAG_CAPTURING_GAIN_ADJUSTMENT);
    impl_tag!(IsoSpeed, i32, GST_TAG_CAPTURING_ISO_SPEED);
    impl_tag!(LightSource, &'a str, GST_TAG_CAPTURING_LIGHT_SOURCE);
    impl_tag!(MeteringMode, &'a str, GST_TAG_CAPTURING_METERING_MODE);
    impl_tag!(Saturation, &'a str, GST_TAG_CAPTURING_SATURATION);
    impl_tag!(
        SceneCaptureType,
        &'a str,
        GST_TAG_CAPTURING_SCENE_CAPTURE_TYPE
    );
    impl_tag!(Sharpness, &'a str, GST_TAG_CAPTURING_SHARPNESS);
    impl_tag!(ShutterSpeed, gst::Fraction, GST_TAG_CAPTURING_SHUTTER_SPEED);
    impl_tag!(Source, &'a str, GST_TAG_CAPTURING_SOURCE);
    impl_tag!(WhiteBalance, &'a str, GST_TAG_CAPTURING_WHITE_BALANCE);
}

pub mod cdda {
    impl_tag!(CddbDiscid, &'a str, GST_TAG_CDDA_CDDB_DISCID);
    impl_tag!(CddbDiscidFull, &'a str, GST_TAG_CDDA_CDDB_DISCID_FULL);
    impl_tag!(MusicbrainzDiscid, &'a str, GST_TAG_CDDA_MUSICBRAINZ_DISCID);
    impl_tag!(
        MusicbrainzDiscidFull,
        &'a str,
        GST_TAG_CDDA_MUSICBRAINZ_DISCID_FULL
    );
}

pub mod image {
    impl_tag!(HorizontalPPI, f64, GST_TAG_IMAGE_HORIZONTAL_PPI);
    impl_tag!(VerticalPPI, f64, GST_TAG_IMAGE_VERTICAL_PPI);
}

impl_tag!(MusicalKey, &'a str, GST_TAG_MUSICAL_KEY);

pub mod musicbrainz {
    impl_tag!(AlbumArtistId, &'a str, GST_TAG_MUSICBRAINZ_ALBUMARTISTID);
    impl_tag!(AlbumId, &'a str, GST_TAG_MUSICBRAINZ_ALBUMID);
    impl_tag!(ArtistId, &'a str, GST_TAG_MUSICBRAINZ_ARTISTID);
    impl_tag!(ReleaseGroupId, &'a str, GST_TAG_MUSICBRAINZ_RELEASEGROUPID);
    impl_tag!(ReleaseTrackId, &'a str, GST_TAG_MUSICBRAINZ_RELEASETRACKID);
    impl_tag!(TrackId, &'a str, GST_TAG_MUSICBRAINZ_TRACKID);
    impl_tag!(TrmId, &'a str, GST_TAG_MUSICBRAINZ_TRMID);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tags_are_registered_automatically() {
        const B_MINOR: &str = "Bm";
        gst::init().unwrap();
        let mut tags = gst::TagList::new();

        tags.get_mut()
            .unwrap()
            .add::<MusicalKey>(&B_MINOR, gst::TagMergeMode::Replace);

        assert_eq!(B_MINOR, tags.get::<MusicalKey>().unwrap().get());
    }
}
