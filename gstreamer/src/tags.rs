// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::mem;
use std::marker::PhantomData;
use std::ffi::CStr;

use ffi;
use glib;
use glib::StaticType;
use glib::value::{FromValueOptional, SetValue, ToSendValue, TypedValue, Value};
use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr, ToGlibPtrMut};

use miniobject::*;

use TagMergeMode;
use Sample;

pub trait Tag<'a> {
    type TagType: FromValueOptional<'a> + SetValue + Send;
    fn tag_name<'b>() -> &'b str;
}

macro_rules! impl_tag(
    ($name:ident, $t:ty, $tag:expr) => {
        pub struct $name;
        impl<'a> Tag<'a> for $name {
            type TagType = $t;
            fn tag_name<'b>() -> &'b str {
                $tag
            }
        }
    };
);

impl_tag!(Title, &'a str, *TAG_TITLE);
impl_tag!(TitleSortname, &'a str, *TAG_TITLE_SORTNAME);
impl_tag!(Artist, &'a str, *TAG_ARTIST);
impl_tag!(ArtistSortname, &'a str, *TAG_ARTIST_SORTNAME);
impl_tag!(Album, &'a str, *TAG_ALBUM);
impl_tag!(AlbumSortname, &'a str, *TAG_ALBUM_SORTNAME);
impl_tag!(AlbumArtist, &'a str, *TAG_ALBUM_ARTIST);
impl_tag!(AlbumArtistSortname, &'a str, *TAG_ALBUM_ARTIST_SORTNAME);
impl_tag!(Date, glib::Date, *TAG_DATE);
impl_tag!(DateTime, ::auto::DateTime, *TAG_DATE_TIME);
impl_tag!(Genre, &'a str, *TAG_GENRE);
impl_tag!(Comment, &'a str, *TAG_COMMENT);
impl_tag!(ExtendedComment, &'a str, *TAG_EXTENDED_COMMENT);
impl_tag!(TrackNumber, u32, *TAG_TRACK_NUMBER);
impl_tag!(TrackCount, u32, *TAG_TRACK_COUNT);
impl_tag!(AlbumVolumeNumber, u32, *TAG_ALBUM_VOLUME_NUMBER);
impl_tag!(AlbumVolumeCount, u32, *TAG_ALBUM_VOLUME_COUNT);
impl_tag!(Location, &'a str, *TAG_LOCATION);
impl_tag!(Homepage, &'a str, *TAG_HOMEPAGE);
impl_tag!(Description, &'a str, *TAG_DESCRIPTION);
impl_tag!(Version, &'a str, *TAG_VERSION);
impl_tag!(ISRC, &'a str, *TAG_ISRC);
impl_tag!(Organization, &'a str, *TAG_ORGANIZATION);
impl_tag!(Copyright, &'a str, *TAG_COPYRIGHT);
impl_tag!(CopyrightUri, &'a str, *TAG_COPYRIGHT_URI);
impl_tag!(EncodedBy, &'a str, *TAG_ENCODED_BY);
impl_tag!(Composer, &'a str, *TAG_COMPOSER);
impl_tag!(Conductor, &'a str, *TAG_CONDUCTOR);
impl_tag!(Contact, &'a str, *TAG_CONTACT);
impl_tag!(License, &'a str, *TAG_LICENSE);
impl_tag!(LicenseUri, &'a str, *TAG_LICENSE_URI);
impl_tag!(Performer, &'a str, *TAG_PERFORMER);
impl_tag!(Duration, u64, *TAG_DURATION);
impl_tag!(Codec, &'a str, *TAG_CODEC);
impl_tag!(VideoCodec, &'a str, *TAG_VIDEO_CODEC);
impl_tag!(AudioCodec, &'a str, *TAG_AUDIO_CODEC);
impl_tag!(SubtitleCodec, &'a str, *TAG_SUBTITLE_CODEC);
impl_tag!(ContainerFormat, &'a str, *TAG_CONTAINER_FORMAT);
impl_tag!(Bitrate, u32, *TAG_BITRATE);
impl_tag!(NominalBitrate, u32, *TAG_NOMINAL_BITRATE);
impl_tag!(MinimumBitrate, u32, *TAG_MINIMUM_BITRATE);
impl_tag!(MaximumBitrate, u32, *TAG_MAXIMUM_BITRATE);
impl_tag!(Serial, u32, *TAG_SERIAL);
impl_tag!(Encoder, &'a str, *TAG_ENCODER);
impl_tag!(EncoderVersion, u32, *TAG_ENCODER_VERSION);
impl_tag!(TrackGain, f64, *TAG_TRACK_GAIN);
impl_tag!(TrackPeak, f64, *TAG_TRACK_PEAK);
impl_tag!(AlbumGain, f64, *TAG_ALBUM_GAIN);
impl_tag!(AlbumPeak, f64, *TAG_ALBUM_PEAK);
impl_tag!(ReferenceLevel, f64, *TAG_REFERENCE_LEVEL);
// TODO: Should ideally enforce this to be ISO-639
impl_tag!(LanguageCode, &'a str, *TAG_LANGUAGE_CODE);
impl_tag!(LanguageName, &'a str, *TAG_LANGUAGE_NAME);
impl_tag!(Image, Sample, *TAG_IMAGE);
impl_tag!(PreviewImage, Sample, *TAG_PREVIEW_IMAGE);
impl_tag!(Attachment, Sample, *TAG_ATTACHMENT);
impl_tag!(BeatsPerMinute, f64, *TAG_BEATS_PER_MINUTE);
impl_tag!(Keywords, &'a str, *TAG_KEYWORDS);
impl_tag!(GeoLocationName, &'a str, *TAG_GEO_LOCATION_NAME);
impl_tag!(GeoLocationLatitude, f64, *TAG_GEO_LOCATION_LATITUDE);
impl_tag!(GeoLocationLongitute, f64, *TAG_GEO_LOCATION_LONGITUDE);
impl_tag!(GeoLocationElevation, f64, *TAG_GEO_LOCATION_ELEVATION);
impl_tag!(GeoLocationCity, &'a str, *TAG_GEO_LOCATION_CITY);
impl_tag!(GeoLocationCountry, &'a str, *TAG_GEO_LOCATION_COUNTRY);
impl_tag!(
    GeoLocationSublocation,
    &'a str,
    *TAG_GEO_LOCATION_SUBLOCATION
);
impl_tag!(
    GeoLocationHorizontalError,
    f64,
    *TAG_GEO_LOCATION_HORIZONTAL_ERROR
);
impl_tag!(
    GeoLocationMovementDirection,
    f64,
    *TAG_GEO_LOCATION_MOVEMENT_DIRECTION
);
impl_tag!(
    GeoLocationMovementSpeed,
    f64,
    *TAG_GEO_LOCATION_MOVEMENT_SPEED
);
impl_tag!(
    GeoLocationCaptureDirection,
    f64,
    *TAG_GEO_LOCATION_CAPTURE_DIRECTION
);
impl_tag!(ShowName, &'a str, *TAG_SHOW_NAME);
impl_tag!(ShowSortname, &'a str, *TAG_SHOW_SORTNAME);
impl_tag!(ShowEpisodeNumber, u32, *TAG_SHOW_EPISODE_NUMBER);
impl_tag!(ShowSeasonNumber, u32, *TAG_SHOW_SEASON_NUMBER);
impl_tag!(Lyrics, &'a str, *TAG_LYRICS);
impl_tag!(ComposerSortname, &'a str, *TAG_COMPOSER_SORTNAME);
impl_tag!(Grouping, &'a str, *TAG_GROUPING);
impl_tag!(UserRating, u32, *TAG_USER_RATING);
impl_tag!(DeviceManufacturer, &'a str, *TAG_DEVICE_MANUFACTURER);
impl_tag!(DeviceModel, &'a str, *TAG_DEVICE_MODEL);
impl_tag!(ApplicationName, &'a str, *TAG_APPLICATION_NAME);
impl_tag!(ApplicationData, Sample, *TAG_APPLICATION_DATA);
impl_tag!(ImageOrientation, &'a str, *TAG_IMAGE_ORIENTATION);
impl_tag!(Publisher, &'a str, *TAG_PUBLISHER);
impl_tag!(InterpretedBy, &'a str, *TAG_INTERPRETED_BY);
impl_tag!(MidiBaseNote, &'a str, *TAG_MIDI_BASE_NOTE);
impl_tag!(PrivateData, Sample, *TAG_PRIVATE_DATA);

lazy_static!{
    static ref TAG_TITLE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_TITLE).to_str().unwrap() };
    static ref TAG_TITLE_SORTNAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_TITLE_SORTNAME).to_str().unwrap() };
    static ref TAG_ARTIST: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ARTIST).to_str().unwrap() };
    static ref TAG_ARTIST_SORTNAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ARTIST_SORTNAME).to_str().unwrap() };
    static ref TAG_ALBUM: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM).to_str().unwrap() };
    static ref TAG_ALBUM_SORTNAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM_SORTNAME).to_str().unwrap() };
    static ref TAG_ALBUM_ARTIST: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM_ARTIST).to_str().unwrap() };
    static ref TAG_ALBUM_ARTIST_SORTNAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM_ARTIST_SORTNAME).to_str().unwrap() };
    static ref TAG_DATE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_DATE).to_str().unwrap() };
    static ref TAG_DATE_TIME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_DATE_TIME).to_str().unwrap() };
    static ref TAG_GENRE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GENRE).to_str().unwrap() };
    static ref TAG_COMMENT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_COMMENT).to_str().unwrap() };
    static ref TAG_EXTENDED_COMMENT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_EXTENDED_COMMENT).to_str().unwrap() };
    static ref TAG_TRACK_NUMBER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_TRACK_NUMBER).to_str().unwrap() };
    static ref TAG_TRACK_COUNT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_TRACK_COUNT).to_str().unwrap() };
    static ref TAG_ALBUM_VOLUME_NUMBER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM_VOLUME_NUMBER).to_str().unwrap() };
    static ref TAG_ALBUM_VOLUME_COUNT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM_VOLUME_COUNT).to_str().unwrap() };
    static ref TAG_LOCATION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_LOCATION).to_str().unwrap() };
    static ref TAG_HOMEPAGE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_HOMEPAGE).to_str().unwrap() };
    static ref TAG_DESCRIPTION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_DESCRIPTION).to_str().unwrap() };
    static ref TAG_VERSION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_VERSION).to_str().unwrap() };
    static ref TAG_ISRC: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ISRC).to_str().unwrap() };
    static ref TAG_ORGANIZATION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ORGANIZATION).to_str().unwrap() };
    static ref TAG_COPYRIGHT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_COPYRIGHT).to_str().unwrap() };
    static ref TAG_COPYRIGHT_URI: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_COPYRIGHT_URI).to_str().unwrap() };
    static ref TAG_ENCODED_BY: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ENCODED_BY).to_str().unwrap() };
    static ref TAG_COMPOSER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_COMPOSER).to_str().unwrap() };
    static ref TAG_CONDUCTOR: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_CONDUCTOR).to_str().unwrap() };
    static ref TAG_CONTACT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_CONTACT).to_str().unwrap() };
    static ref TAG_LICENSE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_LICENSE).to_str().unwrap() };
    static ref TAG_LICENSE_URI: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_LICENSE_URI).to_str().unwrap() };
    static ref TAG_PERFORMER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_PERFORMER).to_str().unwrap() };
    static ref TAG_DURATION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_DURATION).to_str().unwrap() };
    static ref TAG_CODEC: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_CODEC).to_str().unwrap() };
    static ref TAG_VIDEO_CODEC: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_VIDEO_CODEC).to_str().unwrap() };
    static ref TAG_AUDIO_CODEC: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_AUDIO_CODEC).to_str().unwrap() };
    static ref TAG_SUBTITLE_CODEC: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_SUBTITLE_CODEC).to_str().unwrap() };
    static ref TAG_CONTAINER_FORMAT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_CONTAINER_FORMAT).to_str().unwrap() };
    static ref TAG_BITRATE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_BITRATE).to_str().unwrap() };
    static ref TAG_NOMINAL_BITRATE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_NOMINAL_BITRATE).to_str().unwrap() };
    static ref TAG_MINIMUM_BITRATE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_MINIMUM_BITRATE).to_str().unwrap() };
    static ref TAG_MAXIMUM_BITRATE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_MAXIMUM_BITRATE).to_str().unwrap() };
    static ref TAG_SERIAL: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_SERIAL).to_str().unwrap() };
    static ref TAG_ENCODER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ENCODER).to_str().unwrap() };
    static ref TAG_ENCODER_VERSION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ENCODER_VERSION).to_str().unwrap() };
    static ref TAG_TRACK_GAIN: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_TRACK_GAIN).to_str().unwrap() };
    static ref TAG_TRACK_PEAK: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_TRACK_PEAK).to_str().unwrap() };
    static ref TAG_ALBUM_GAIN: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM_GAIN).to_str().unwrap() };
    static ref TAG_ALBUM_PEAK: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM_PEAK).to_str().unwrap() };
    static ref TAG_REFERENCE_LEVEL: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_REFERENCE_LEVEL).to_str().unwrap() };
    static ref TAG_LANGUAGE_CODE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_LANGUAGE_CODE).to_str().unwrap() };
    static ref TAG_LANGUAGE_NAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_LANGUAGE_NAME).to_str().unwrap() };
    static ref TAG_IMAGE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_IMAGE).to_str().unwrap() };
    static ref TAG_PREVIEW_IMAGE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_PREVIEW_IMAGE).to_str().unwrap() };
    static ref TAG_ATTACHMENT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ATTACHMENT).to_str().unwrap() };
    static ref TAG_BEATS_PER_MINUTE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_BEATS_PER_MINUTE).to_str().unwrap() };
    static ref TAG_KEYWORDS: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_KEYWORDS).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_NAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_NAME).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_LATITUDE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_LATITUDE).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_LONGITUDE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_LONGITUDE).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_ELEVATION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_ELEVATION).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_CITY: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_CITY).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_COUNTRY: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_COUNTRY).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_SUBLOCATION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_SUBLOCATION).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_HORIZONTAL_ERROR: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_HORIZONTAL_ERROR).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_MOVEMENT_DIRECTION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_MOVEMENT_DIRECTION).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_MOVEMENT_SPEED: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_MOVEMENT_SPEED).to_str().unwrap() };
    static ref TAG_GEO_LOCATION_CAPTURE_DIRECTION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GEO_LOCATION_CAPTURE_DIRECTION).to_str().unwrap() };
    static ref TAG_SHOW_NAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_SHOW_NAME).to_str().unwrap() };
    static ref TAG_SHOW_SORTNAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_SHOW_SORTNAME).to_str().unwrap() };
    static ref TAG_SHOW_EPISODE_NUMBER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_SHOW_EPISODE_NUMBER).to_str().unwrap() };
    static ref TAG_SHOW_SEASON_NUMBER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_SHOW_SEASON_NUMBER).to_str().unwrap() };
    static ref TAG_LYRICS: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_LYRICS).to_str().unwrap() };
    static ref TAG_COMPOSER_SORTNAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_COMPOSER_SORTNAME).to_str().unwrap() };
    static ref TAG_GROUPING: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_GROUPING).to_str().unwrap() };
    static ref TAG_USER_RATING: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_USER_RATING).to_str().unwrap() };
    static ref TAG_DEVICE_MANUFACTURER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_DEVICE_MANUFACTURER).to_str().unwrap() };
    static ref TAG_DEVICE_MODEL: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_DEVICE_MODEL).to_str().unwrap() };
    static ref TAG_APPLICATION_NAME: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_APPLICATION_NAME).to_str().unwrap() };
    static ref TAG_APPLICATION_DATA: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_APPLICATION_DATA).to_str().unwrap() };
    static ref TAG_IMAGE_ORIENTATION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_IMAGE_ORIENTATION).to_str().unwrap() };
    static ref TAG_PUBLISHER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_PUBLISHER).to_str().unwrap() };
    static ref TAG_INTERPRETED_BY: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_INTERPRETED_BY).to_str().unwrap() };
    static ref TAG_MIDI_BASE_NOTE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_MIDI_BASE_NOTE).to_str().unwrap() };
    static ref TAG_PRIVATE_DATA: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_PRIVATE_DATA).to_str().unwrap() };
}

pub type TagList = GstRc<TagListRef>;
pub struct TagListRef(ffi::GstTagList);

unsafe impl MiniObject for TagListRef {
    type GstType = ffi::GstTagList;
}

impl GstRc<TagListRef> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_tag_list_new_empty()) }
    }
}

impl Default for GstRc<TagListRef> {
    fn default() -> Self {
        Self::new()
    }
}

impl TagListRef {
    pub fn add<'a, T: Tag<'a>>(&mut self, value: T::TagType, mode: TagMergeMode)
    where
        T::TagType: ToSendValue,
    {
        unsafe {
            let v = value.to_send_value();

            ffi::gst_tag_list_add_value(
                self.as_mut_ptr(),
                mode.to_glib(),
                T::tag_name().to_glib_none().0,
                v.to_glib_none().0,
            );
        }
    }

    pub fn get<'a, T: Tag<'a>>(&self) -> Option<TypedValue<T::TagType>> {
        unsafe {
            let mut value: Value = mem::zeroed();

            let found: bool = from_glib(ffi::gst_tag_list_copy_value(
                value.to_glib_none_mut().0,
                self.as_ptr(),
                T::tag_name().to_glib_none().0,
            ));

            if !found {
                return None;
            }

            value.downcast().ok()
        }
    }

    pub fn get_index<'a, T: Tag<'a>>(&'a self, idx: u32) -> Option<&'a TypedValue<T::TagType>> {
        unsafe {
            let value = ffi::gst_tag_list_get_value_index(
                self.as_ptr(),
                T::tag_name().to_glib_none().0,
                idx,
            );

            if value.is_null() || (*value).g_type != T::TagType::static_type().to_glib() {
                return None;
            }

            Some(&*(value as *const TypedValue<T::TagType>))
        }
    }

    pub fn get_size<'a, T: Tag<'a>>(&'a self) -> u32 {
        unsafe { ffi::gst_tag_list_get_tag_size(self.as_ptr(), T::tag_name().to_glib_none().0) }
    }

    pub fn iter_tag<'a, T: Tag<'a>>(&'a self) -> TagIterator<'a, T> {
        TagIterator::new(self)
    }

    pub fn to_string(&self) -> String {
        unsafe { from_glib_full(ffi::gst_tag_list_to_string(self.as_ptr())) }
    }

    pub fn insert(&mut self, other: &TagListRef, mode: TagMergeMode) {
        unsafe { ffi::gst_tag_list_insert(self.as_mut_ptr(), other.as_ptr(), mode.to_glib()) }
    }

    pub fn merge(&self, other: &TagListRef, mode: TagMergeMode) -> TagList {
        unsafe { from_glib_full(ffi::gst_tag_list_merge(self.as_ptr(), other.as_ptr(), mode.to_glib())) }
    }
}

impl fmt::Debug for TagListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("TagList")
            .field(&self.to_string())
            .finish()
    }
}

impl fmt::Display for TagListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialEq for TagListRef {
    fn eq(&self, other: &TagListRef) -> bool {
        unsafe { from_glib(ffi::gst_tag_list_is_equal(self.as_ptr(), other.as_ptr())) }
    }
}

impl Eq for TagListRef {}

impl ToOwned for TagListRef {
    type Owned = GstRc<TagListRef>;

    fn to_owned(&self) -> GstRc<TagListRef> {
        unsafe {
            from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _)
                as *mut _)
        }
    }
}

impl StaticType for TagListRef {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_tag_list_get_type()) }
    }
}

unsafe impl Sync for TagListRef {}
unsafe impl Send for TagListRef {}

pub struct TagIterator<'a, T: Tag<'a>> {
    taglist: &'a TagListRef,
    idx: u32,
    size: u32,
    phantom: PhantomData<T>,
}

impl<'a, T: Tag<'a>> TagIterator<'a, T> {
    fn new(taglist: &'a TagListRef) -> TagIterator<'a, T> {
        skip_assert_initialized!();
        TagIterator {
            taglist: taglist,
            idx: 0,
            size: taglist.get_size::<T>(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Tag<'a>> Iterator for TagIterator<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
    type Item = &'a TypedValue<T::TagType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.taglist.get_index::<T>(self.idx);
        self.idx += 1;

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.size {
            return (0, Some(0));
        }

        let remaining = (self.size - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a, T: Tag<'a>> DoubleEndedIterator for TagIterator<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        self.taglist.get_index::<T>(self.size)
    }
}

impl<'a, T: Tag<'a>> ExactSizeIterator for TagIterator<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        ::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>("some title".into(), TagMergeMode::Append);
            tags.add::<Duration>((1000u64 * 1000 * 1000 * 120).into(), TagMergeMode::Append);
        }
        assert_eq!(
            tags.to_string(),
            "taglist, title=(string)\"some\\ title\", duration=(guint64)120000000000;"
        );
    }

    #[test]
    fn test_get() {
        ::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>("some title".into(), TagMergeMode::Append);
            tags.add::<Duration>((1000u64 * 1000 * 1000 * 120).into(), TagMergeMode::Append);
        }

        assert_eq!(tags.get::<Title>().unwrap().get(), Some("some title"));
        assert_eq!(
            tags.get::<Duration>().unwrap().get_some(),
            (1000u64 * 1000 * 1000 * 120)
        );
        assert_eq!(
            tags.get_index::<Title>(0).unwrap().get(),
            Some("some title")
        );
        assert_eq!(
            tags.get_index::<Duration>(0).unwrap().get_some(),
            (1000u64 * 1000 * 1000 * 120)
        );
    }
}
