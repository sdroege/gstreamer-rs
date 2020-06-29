// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::mem;

use once_cell::sync::Lazy;

use glib;
use glib::translate::{
    from_glib, from_glib_full, FromGlibPtrFull, ToGlib, ToGlibPtr, ToGlibPtrMut,
};
use glib::value::{FromValueOptional, SendValue, SetValue, ToSendValue, TypedValue, Value};
use glib::StaticType;
use gobject_sys;
use gst_sys;

use Sample;
use TagError;
use TagMergeMode;
use TagScope;

pub trait Tag<'a> {
    type TagType: FromValueOptional<'a> + SetValue + Send;
    fn tag_name<'b>() -> &'b str;
}

macro_rules! impl_tag(
    ($name:ident, $t:ty, $rust_tag:ident, $gst_tag:ident) => {
        pub enum $name {}
        impl<'a> Tag<'a> for $name {
            type TagType = $t;
            fn tag_name<'b>() -> &'b str {
                *$rust_tag
            }
        }

        pub(crate) static $rust_tag: Lazy<&'static str> = Lazy::new(||
            unsafe { CStr::from_ptr(gst_sys::$gst_tag).to_str().unwrap() });
    };
);

impl_tag!(Title, &'a str, TAG_TITLE, GST_TAG_TITLE);
impl_tag!(
    TitleSortname,
    &'a str,
    TAG_TITLE_SORTNAME,
    GST_TAG_TITLE_SORTNAME
);
impl_tag!(Artist, &'a str, TAG_ARTIST, GST_TAG_ARTIST);
impl_tag!(
    ArtistSortname,
    &'a str,
    TAG_ARTIST_SORTNAME,
    GST_TAG_ARTIST_SORTNAME
);
impl_tag!(Album, &'a str, TAG_ALBUM, GST_TAG_ALBUM);
impl_tag!(
    AlbumSortname,
    &'a str,
    TAG_ALBUM_SORTNAME,
    GST_TAG_ALBUM_SORTNAME
);
impl_tag!(AlbumArtist, &'a str, TAG_ALBUM_ARTIST, GST_TAG_ALBUM_ARTIST);
impl_tag!(
    AlbumArtistSortname,
    &'a str,
    TAG_ALBUM_ARTIST_SORTNAME,
    GST_TAG_ALBUM_ARTIST_SORTNAME
);
impl_tag!(Date, glib::Date, TAG_DATE, GST_TAG_DATE);
impl_tag!(DateTime, ::auto::DateTime, TAG_DATE_TIME, GST_TAG_DATE_TIME);
impl_tag!(Genre, &'a str, TAG_GENRE, GST_TAG_GENRE);
impl_tag!(Comment, &'a str, TAG_COMMENT, GST_TAG_COMMENT);
impl_tag!(
    ExtendedComment,
    &'a str,
    TAG_EXTENDED_COMMENT,
    GST_TAG_EXTENDED_COMMENT
);
impl_tag!(TrackNumber, u32, TAG_TRACK_NUMBER, GST_TAG_TRACK_NUMBER);
impl_tag!(TrackCount, u32, TAG_TRACK_COUNT, GST_TAG_TRACK_COUNT);
impl_tag!(
    AlbumVolumeNumber,
    u32,
    TAG_ALBUM_VOLUME_NUMBER,
    GST_TAG_ALBUM_VOLUME_NUMBER
);
impl_tag!(
    AlbumVolumeCount,
    u32,
    TAG_ALBUM_VOLUME_COUNT,
    GST_TAG_ALBUM_VOLUME_COUNT
);
impl_tag!(Location, &'a str, TAG_LOCATION, GST_TAG_LOCATION);
impl_tag!(Homepage, &'a str, TAG_HOMEPAGE, GST_TAG_HOMEPAGE);
impl_tag!(Description, &'a str, TAG_DESCRIPTION, GST_TAG_DESCRIPTION);
impl_tag!(Version, &'a str, TAG_VERSION, GST_TAG_VERSION);
impl_tag!(ISRC, &'a str, TAG_ISRC, GST_TAG_ISRC);
impl_tag!(
    Organization,
    &'a str,
    TAG_ORGANIZATION,
    GST_TAG_ORGANIZATION
);
impl_tag!(Copyright, &'a str, TAG_COPYRIGHT, GST_TAG_COPYRIGHT);
impl_tag!(
    CopyrightUri,
    &'a str,
    TAG_COPYRIGHT_URI,
    GST_TAG_COPYRIGHT_URI
);
impl_tag!(EncodedBy, &'a str, TAG_ENCODED_BY, GST_TAG_ENCODED_BY);
impl_tag!(Composer, &'a str, TAG_COMPOSER, GST_TAG_COMPOSER);
impl_tag!(Conductor, &'a str, TAG_CONDUCTOR, GST_TAG_CONDUCTOR);
impl_tag!(Contact, &'a str, TAG_CONTACT, GST_TAG_CONTACT);
impl_tag!(License, &'a str, TAG_LICENSE, GST_TAG_LICENSE);
impl_tag!(LicenseUri, &'a str, TAG_LICENSE_URI, GST_TAG_LICENSE_URI);
impl_tag!(Performer, &'a str, TAG_PERFORMER, GST_TAG_PERFORMER);
impl_tag!(Duration, ::ClockTime, TAG_DURATION, GST_TAG_DURATION);
impl_tag!(Codec, &'a str, TAG_CODEC, GST_TAG_CODEC);
impl_tag!(VideoCodec, &'a str, TAG_VIDEO_CODEC, GST_TAG_VIDEO_CODEC);
impl_tag!(AudioCodec, &'a str, TAG_AUDIO_CODEC, GST_TAG_AUDIO_CODEC);
impl_tag!(
    SubtitleCodec,
    &'a str,
    TAG_SUBTITLE_CODEC,
    GST_TAG_SUBTITLE_CODEC
);
impl_tag!(
    ContainerFormat,
    &'a str,
    TAG_CONTAINER_FORMAT,
    GST_TAG_CONTAINER_FORMAT
);
impl_tag!(Bitrate, u32, TAG_BITRATE, GST_TAG_BITRATE);
impl_tag!(
    NominalBitrate,
    u32,
    TAG_NOMINAL_BITRATE,
    GST_TAG_NOMINAL_BITRATE
);
impl_tag!(
    MinimumBitrate,
    u32,
    TAG_MINIMUM_BITRATE,
    GST_TAG_MINIMUM_BITRATE
);
impl_tag!(
    MaximumBitrate,
    u32,
    TAG_MAXIMUM_BITRATE,
    GST_TAG_MAXIMUM_BITRATE
);
impl_tag!(Serial, u32, TAG_SERIAL, GST_TAG_SERIAL);
impl_tag!(Encoder, &'a str, TAG_ENCODER, GST_TAG_ENCODER);
impl_tag!(
    EncoderVersion,
    u32,
    TAG_ENCODER_VERSION,
    GST_TAG_ENCODER_VERSION
);
impl_tag!(TrackGain, f64, TAG_TRACK_GAIN, GST_TAG_TRACK_GAIN);
impl_tag!(TrackPeak, f64, TAG_TRACK_PEAK, GST_TAG_TRACK_PEAK);
impl_tag!(AlbumGain, f64, TAG_ALBUM_GAIN, GST_TAG_ALBUM_GAIN);
impl_tag!(AlbumPeak, f64, TAG_ALBUM_PEAK, GST_TAG_ALBUM_PEAK);
impl_tag!(
    ReferenceLevel,
    f64,
    TAG_REFERENCE_LEVEL,
    GST_TAG_REFERENCE_LEVEL
);
// TODO: Should ideally enforce this to be ISO-639
impl_tag!(
    LanguageCode,
    &'a str,
    TAG_LANGUAGE_CODE,
    GST_TAG_LANGUAGE_CODE
);
impl_tag!(
    LanguageName,
    &'a str,
    TAG_LANGUAGE_NAME,
    GST_TAG_LANGUAGE_NAME
);
impl_tag!(Image, Sample, TAG_IMAGE, GST_TAG_IMAGE);
impl_tag!(
    PreviewImage,
    Sample,
    TAG_PREVIEW_IMAGE,
    GST_TAG_PREVIEW_IMAGE
);
impl_tag!(Attachment, Sample, TAG_ATTACHMENT, GST_TAG_ATTACHMENT);
impl_tag!(
    BeatsPerMinute,
    f64,
    TAG_BEATS_PER_MINUTE,
    GST_TAG_BEATS_PER_MINUTE
);
impl_tag!(Keywords, &'a str, TAG_KEYWORDS, GST_TAG_KEYWORDS);
impl_tag!(
    GeoLocationName,
    &'a str,
    TAG_GEO_LOCATION_NAME,
    GST_TAG_GEO_LOCATION_NAME
);
impl_tag!(
    GeoLocationLatitude,
    f64,
    TAG_GEO_LOCATION_LATITUDE,
    GST_TAG_GEO_LOCATION_LATITUDE
);
impl_tag!(
    GeoLocationLongitute,
    f64,
    TAG_GEO_LOCATION_LONGITUDE,
    GST_TAG_GEO_LOCATION_LONGITUDE
);
impl_tag!(
    GeoLocationElevation,
    f64,
    TAG_GEO_LOCATION_ELEVATION,
    GST_TAG_GEO_LOCATION_ELEVATION
);
impl_tag!(
    GeoLocationCity,
    &'a str,
    TAG_GEO_LOCATION_CITY,
    GST_TAG_GEO_LOCATION_CITY
);
impl_tag!(
    GeoLocationCountry,
    &'a str,
    TAG_GEO_LOCATION_COUNTRY,
    GST_TAG_GEO_LOCATION_COUNTRY
);
impl_tag!(
    GeoLocationSublocation,
    &'a str,
    TAG_GEO_LOCATION_SUBLOCATION,
    GST_TAG_GEO_LOCATION_SUBLOCATION
);
impl_tag!(
    GeoLocationHorizontalError,
    f64,
    TAG_GEO_LOCATION_HORIZONTAL_ERROR,
    GST_TAG_GEO_LOCATION_HORIZONTAL_ERROR
);
impl_tag!(
    GeoLocationMovementDirection,
    f64,
    TAG_GEO_LOCATION_MOVEMENT_DIRECTION,
    GST_TAG_GEO_LOCATION_MOVEMENT_DIRECTION
);
impl_tag!(
    GeoLocationMovementSpeed,
    f64,
    TAG_GEO_LOCATION_MOVEMENT_SPEED,
    GST_TAG_GEO_LOCATION_MOVEMENT_SPEED
);
impl_tag!(
    GeoLocationCaptureDirection,
    f64,
    TAG_GEO_LOCATION_CAPTURE_DIRECTION,
    GST_TAG_GEO_LOCATION_CAPTURE_DIRECTION
);
impl_tag!(ShowName, &'a str, TAG_SHOW_NAME, GST_TAG_SHOW_NAME);
impl_tag!(
    ShowSortname,
    &'a str,
    TAG_SHOW_SORTNAME,
    GST_TAG_SHOW_SORTNAME
);
impl_tag!(
    ShowEpisodeNumber,
    u32,
    TAG_SHOW_EPISODE_NUMBER,
    GST_TAG_SHOW_EPISODE_NUMBER
);
impl_tag!(
    ShowSeasonNumber,
    u32,
    TAG_SHOW_SEASON_NUMBER,
    GST_TAG_SHOW_SEASON_NUMBER
);
impl_tag!(Lyrics, &'a str, TAG_LYRICS, GST_TAG_LYRICS);
impl_tag!(
    ComposerSortname,
    &'a str,
    TAG_COMPOSER_SORTNAME,
    GST_TAG_COMPOSER_SORTNAME
);
impl_tag!(Grouping, &'a str, TAG_GROUPING, GST_TAG_GROUPING);
impl_tag!(UserRating, u32, TAG_USER_RATING, GST_TAG_USER_RATING);
impl_tag!(
    DeviceManufacturer,
    &'a str,
    TAG_DEVICE_MANUFACTURER,
    GST_TAG_DEVICE_MANUFACTURER
);
impl_tag!(DeviceModel, &'a str, TAG_DEVICE_MODEL, GST_TAG_DEVICE_MODEL);
impl_tag!(
    ApplicationName,
    &'a str,
    TAG_APPLICATION_NAME,
    GST_TAG_APPLICATION_NAME
);
impl_tag!(
    ApplicationData,
    Sample,
    TAG_APPLICATION_DATA,
    GST_TAG_APPLICATION_DATA
);
impl_tag!(
    ImageOrientation,
    &'a str,
    TAG_IMAGE_ORIENTATION,
    GST_TAG_IMAGE_ORIENTATION
);
impl_tag!(Publisher, &'a str, TAG_PUBLISHER, GST_TAG_PUBLISHER);
impl_tag!(
    InterpretedBy,
    &'a str,
    TAG_INTERPRETED_BY,
    GST_TAG_INTERPRETED_BY
);
impl_tag!(
    MidiBaseNote,
    &'a str,
    TAG_MIDI_BASE_NOTE,
    GST_TAG_MIDI_BASE_NOTE
);
impl_tag!(PrivateData, Sample, TAG_PRIVATE_DATA, GST_TAG_PRIVATE_DATA);

gst_define_mini_object_wrapper!(TagList, TagListRef, gst_sys::GstTagList, || {
    gst_sys::gst_tag_list_get_type()
});

impl TagList {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_sys::gst_tag_list_new_empty()) }
    }
}

impl Default for TagList {
    fn default() -> Self {
        Self::new()
    }
}

impl TagListRef {
    pub fn add<'a, T: Tag<'a>>(&mut self, value: &T::TagType, mode: TagMergeMode)
    where
        T::TagType: ToSendValue,
    {
        // result can be safely ignored here as `value`'s type is tied to `T::tag_name()`
        let _res = self.add_generic(T::tag_name(), value, mode);
    }

    pub fn add_generic<T>(
        &mut self,
        tag_name: &str,
        value: &T,
        mode: TagMergeMode,
    ) -> Result<(), TagError>
    where
        T: ToSendValue,
    {
        unsafe {
            let v = value.to_send_value();

            let tag_name = tag_name.to_glib_none();

            let tag_type: glib::Type = from_glib(gst_sys::gst_tag_get_type(tag_name.0));
            if tag_type != v.type_() {
                return Err(TagError::TypeMismatch);
            }

            gst_sys::gst_tag_list_add_value(
                self.as_mut_ptr(),
                mode.to_glib(),
                tag_name.0,
                v.to_glib_none().0,
            );
        }

        Ok(())
    }

    pub fn get<'a, T: Tag<'a>>(&self) -> Option<TypedValue<T::TagType>> {
        self.get_generic(T::tag_name()).map(|value| {
            value.downcast().unwrap_or_else(|_| {
                panic!("TagListRef::get type mismatch for tag {}", T::tag_name())
            })
        })
    }

    pub fn get_generic(&self, tag_name: &str) -> Option<SendValue> {
        unsafe {
            let mut value: mem::MaybeUninit<SendValue> = mem::MaybeUninit::zeroed();

            let found: bool = from_glib(gst_sys::gst_tag_list_copy_value(
                (*value.as_mut_ptr()).to_glib_none_mut().0,
                self.as_ptr(),
                tag_name.to_glib_none().0,
            ));

            if !found {
                return None;
            }

            Some(value.assume_init())
        }
    }

    pub fn n_tags(&self) -> i32 {
        unsafe { gst_sys::gst_tag_list_n_tags(self.as_ptr()) }
    }

    pub fn nth_tag_name(&self, idx: u32) -> &str {
        unsafe {
            CStr::from_ptr(gst_sys::gst_tag_list_nth_tag_name(self.as_ptr(), idx))
                .to_str()
                .unwrap()
        }
    }

    pub fn get_index<'a, T: Tag<'a>>(&'a self, idx: u32) -> Option<&'a TypedValue<T::TagType>> {
        self.get_index_generic(T::tag_name(), idx)
            .and_then(|value| value.downcast_ref())
    }

    pub fn get_index_generic<'a>(&'a self, tag_name: &str, idx: u32) -> Option<&'a SendValue> {
        unsafe {
            let value = gst_sys::gst_tag_list_get_value_index(
                self.as_ptr(),
                tag_name.to_glib_none().0,
                idx,
            );

            if value.is_null() {
                return None;
            }

            Some(&*(value as *const SendValue))
        }
    }

    pub fn get_size<'a, T: Tag<'a>>(&self) -> u32 {
        self.get_size_by_name(T::tag_name())
    }

    pub fn get_size_by_name(&self, tag_name: &str) -> u32 {
        unsafe { gst_sys::gst_tag_list_get_tag_size(self.as_ptr(), tag_name.to_glib_none().0) }
    }

    pub fn iter_tag<'a, T: Tag<'a>>(&'a self) -> TagIter<'a, T> {
        TagIter::new(self)
    }

    pub fn iter_tag_generic<'a>(&'a self, tag_name: &'a str) -> GenericTagIter<'a> {
        GenericTagIter::new(self, tag_name)
    }

    pub fn iter_generic(&self) -> GenericIter {
        GenericIter::new(self)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn insert(&mut self, other: &TagListRef, mode: TagMergeMode) {
        unsafe { gst_sys::gst_tag_list_insert(self.as_mut_ptr(), other.as_ptr(), mode.to_glib()) }
    }

    pub fn merge(&self, other: &TagListRef, mode: TagMergeMode) -> TagList {
        unsafe {
            from_glib_full(gst_sys::gst_tag_list_merge(
                self.as_ptr(),
                other.as_ptr(),
                mode.to_glib(),
            ))
        }
    }

    pub fn get_scope(&self) -> TagScope {
        unsafe { from_glib(gst_sys::gst_tag_list_get_scope(self.as_ptr())) }
    }

    pub fn set_scope(&mut self, scope: TagScope) {
        unsafe { gst_sys::gst_tag_list_set_scope(self.as_mut_ptr(), scope.to_glib()) }
    }
}

impl fmt::Debug for TagList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <TagListRef as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for TagList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <TagListRef as fmt::Display>::fmt(self, f)
    }
}

impl PartialEq for TagList {
    fn eq(&self, other: &TagList) -> bool {
        TagListRef::eq(self, other)
    }
}

impl Eq for TagList {}

impl fmt::Debug for TagListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("TagList").field(&self.to_string()).finish()
    }
}

impl fmt::Display for TagListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            glib::GString::from_glib_full(gst_sys::gst_tag_list_to_string(self.as_ptr()))
        };
        f.write_str(&s)
    }
}

impl PartialEq for TagListRef {
    fn eq(&self, other: &TagListRef) -> bool {
        unsafe {
            from_glib(gst_sys::gst_tag_list_is_equal(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }
}

impl Eq for TagListRef {}

#[derive(Debug)]
pub struct TagIter<'a, T: Tag<'a>> {
    taglist: &'a TagListRef,
    idx: u32,
    size: u32,
    phantom: PhantomData<T>,
}

impl<'a, T: Tag<'a>> TagIter<'a, T> {
    fn new(taglist: &'a TagListRef) -> TagIter<'a, T> {
        skip_assert_initialized!();
        TagIter {
            taglist,
            idx: 0,
            size: taglist.get_size::<T>(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Tag<'a>> Iterator for TagIter<'a, T>
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

impl<'a, T: Tag<'a>> DoubleEndedIterator for TagIter<'a, T>
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

impl<'a, T: Tag<'a>> ExactSizeIterator for TagIter<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
}

#[derive(Debug)]
pub struct GenericTagIter<'a> {
    taglist: &'a TagListRef,
    name: &'a str,
    idx: u32,
    size: u32,
}

impl<'a> GenericTagIter<'a> {
    fn new(taglist: &'a TagListRef, name: &'a str) -> GenericTagIter<'a> {
        skip_assert_initialized!();
        GenericTagIter {
            taglist,
            name,
            idx: 0,
            size: taglist.get_size_by_name(name),
        }
    }
}

impl<'a> Iterator for GenericTagIter<'a> {
    type Item = &'a SendValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.taglist.get_index_generic(self.name, self.idx);
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

impl<'a> DoubleEndedIterator for GenericTagIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        self.taglist.get_index_generic(self.name, self.size)
    }
}

impl<'a> ExactSizeIterator for GenericTagIter<'a> {}

#[derive(Debug)]
pub struct GenericIter<'a> {
    taglist: &'a TagListRef,
    idx: u32,
    size: u32,
}

impl<'a> GenericIter<'a> {
    fn new(taglist: &'a TagListRef) -> GenericIter<'a> {
        skip_assert_initialized!();
        let size = taglist.n_tags();
        GenericIter {
            taglist,
            idx: 0,
            size: if size > 0 { size as u32 } else { 0 },
        }
    }
}

impl<'a> Iterator for GenericIter<'a> {
    type Item = (&'a str, GenericTagIter<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let name = self.taglist.nth_tag_name(self.idx);
        let item = (name, self.taglist.iter_tag_generic(name));
        self.idx += 1;

        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.size {
            return (0, Some(0));
        }

        let remaining = (self.size - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for GenericIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        let name = self.taglist.nth_tag_name(self.idx);
        Some((name, self.taglist.iter_tag_generic(name)))
    }
}

impl<'a> ExactSizeIterator for GenericIter<'a> {}

#[derive(Debug)]
pub struct Iter<'a> {
    taglist: &'a TagListRef,
    idx: u32,
    size: u32,
}

impl<'a> Iter<'a> {
    fn new(taglist: &'a TagListRef) -> Iter<'a> {
        skip_assert_initialized!();
        let size = taglist.n_tags();
        Iter {
            taglist,
            idx: 0,
            size: if size > 0 { size as u32 } else { 0 },
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, glib::SendValue);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let name = self.taglist.nth_tag_name(self.idx);
        let item = (name, self.taglist.get_generic(name).unwrap());
        self.idx += 1;

        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.size {
            return (0, Some(0));
        }

        let remaining = (self.size - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        let name = self.taglist.nth_tag_name(self.idx);
        Some((name, self.taglist.get_generic(name).unwrap()))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

pub fn tag_exists(name: &str) -> bool {
    skip_assert_initialized!();
    unsafe { from_glib(gst_sys::gst_tag_exists(name.to_glib_none().0)) }
}

pub fn tag_get_type(name: &str) -> glib::Type {
    skip_assert_initialized!();
    unsafe { from_glib(gst_sys::gst_tag_get_type(name.to_glib_none().0)) }
}

pub fn tag_get_nick<'b>(name: &str) -> Option<&'b str> {
    skip_assert_initialized!();
    unsafe {
        let ptr = gst_sys::gst_tag_get_nick(name.to_glib_none().0);

        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_str().unwrap())
        }
    }
}

pub fn tag_get_description<'b>(name: &str) -> Option<&'b str> {
    skip_assert_initialized!();
    unsafe {
        let ptr = gst_sys::gst_tag_get_description(name.to_glib_none().0);

        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_str().unwrap())
        }
    }
}

pub fn tag_get_flag(name: &str) -> ::TagFlag {
    skip_assert_initialized!();
    unsafe { from_glib(gst_sys::gst_tag_get_flag(name.to_glib_none().0)) }
}

pub trait CustomTag<'a>: Tag<'a> {
    const FLAG: ::TagFlag;
    const NICK: &'static str;
    const DESCRIPTION: &'static str;

    fn merge_func(src: &Value) -> Value {
        skip_assert_initialized!();
        merge_use_first(src)
    }
}

pub fn register<T: for<'a> CustomTag<'a>>() {
    assert!(!tag_exists(T::tag_name()));

    unsafe extern "C" fn merge_func_trampoline<T: for<'a> CustomTag<'a>>(
        dest: *mut gobject_sys::GValue,
        src: *const gobject_sys::GValue,
    ) {
        *dest = T::merge_func(&*(src as *const Value)).into_raw();
    }

    unsafe {
        gst_sys::gst_tag_register(
            T::tag_name().to_glib_none().0,
            T::FLAG.to_glib(),
            T::TagType::static_type().to_glib(),
            T::NICK.to_glib_none().0,
            T::DESCRIPTION.to_glib_none().0,
            Some(merge_func_trampoline::<T>),
        )
    }
}

pub fn merge_use_first(src: &Value) -> Value {
    skip_assert_initialized!();
    assert_eq!(src.type_(), ::List::static_type());

    unsafe {
        use glib::translate::Uninitialized;

        let mut res = Value::uninitialized();
        gst_sys::gst_tag_merge_use_first(res.to_glib_none_mut().0, src.to_glib_none().0);
        res
    }
}

pub fn merge_strings_with_comma(src: &Value) -> Value {
    skip_assert_initialized!();
    assert_eq!(src.type_(), ::List::static_type());

    unsafe {
        use glib::translate::Uninitialized;

        let mut res = Value::uninitialized();
        gst_sys::gst_tag_merge_strings_with_comma(res.to_glib_none_mut().0, src.to_glib_none().0);
        res
    }
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
            tags.add::<Title>(&"some title", TagMergeMode::Append);
            tags.add::<Duration>(&(::SECOND * 120), TagMergeMode::Append);
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
            tags.add::<Title>(&"some title", TagMergeMode::Append);
            tags.add::<Duration>(&(::SECOND * 120), TagMergeMode::Append);
        }

        assert_eq!(tags.get::<Title>().unwrap().get(), Some("some title"));
        assert_eq!(tags.get::<Duration>().unwrap().get_some(), (::SECOND * 120));
        assert_eq!(
            tags.get_index::<Title>(0).unwrap().get(),
            Some("some title")
        );
        assert_eq!(
            tags.get_index::<Duration>(0).unwrap().get_some(),
            (::SECOND * 120)
        );
    }

    #[test]
    fn test_scope() {
        ::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.get_scope(), TagScope::Stream);
        {
            let tags = tags.get_mut().unwrap();
            tags.set_scope(TagScope::Global);
        }
        assert_eq!(tags.get_scope(), TagScope::Global);
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_generic() {
        ::init().unwrap();

        let mut tags = TagList::new();
        {
            let tags = tags.get_mut().unwrap();
            assert!(tags
                .add_generic(&TAG_TITLE, &"some title", TagMergeMode::Append)
                .is_ok());
            assert!(tags
                .add_generic(&TAG_TITLE, &"second title", TagMergeMode::Append)
                .is_ok());
            assert!(tags
                .add_generic(&TAG_DURATION, &(::SECOND * 120), TagMergeMode::Append)
                .is_ok());
            assert!(tags
                .add_generic(&TAG_TITLE, &"third title", TagMergeMode::Append)
                .is_ok());

            assert_eq!(
                tags.add_generic(
                    &TAG_IMAGE,
                    &"`&[str] instead of `Sample`",
                    TagMergeMode::Append
                ),
                Err(TagError::TypeMismatch),
            );
        }

        assert_eq!(
            tags.get_index_generic(&TAG_TITLE, 0).unwrap().get(),
            Ok(Some("some title"))
        );
        assert_eq!(
            tags.get_index_generic(&TAG_TITLE, 1).unwrap().get(),
            Ok(Some("second title"))
        );
        assert_eq!(
            tags.get_index_generic(&TAG_DURATION, 0).unwrap().get(),
            Ok(Some(::SECOND * 120))
        );
        assert_eq!(
            tags.get_index_generic(&TAG_TITLE, 2).unwrap().get(),
            Ok(Some("third title"))
        );

        assert_eq!(
            tags.get_generic(&TAG_TITLE).unwrap().get(),
            Ok(Some("some title, second title, third title"))
        );

        assert_eq!(tags.n_tags(), 2);
        assert_eq!(tags.nth_tag_name(0), *TAG_TITLE);
        assert_eq!(tags.get_size_by_name(&TAG_TITLE), 3);
        assert_eq!(tags.nth_tag_name(1), *TAG_DURATION);
        assert_eq!(tags.get_size_by_name(&TAG_DURATION), 1);

        // GenericTagIter
        let mut title_iter = tags.iter_tag_generic(&TAG_TITLE);
        assert_eq!(title_iter.size_hint(), (3, Some(3)));
        let first_title = title_iter.next().unwrap();
        assert_eq!(first_title.get(), Ok(Some("some title")));
        let second_title = title_iter.next().unwrap();
        assert_eq!(second_title.get(), Ok(Some("second title")));
        let third_title = title_iter.next().unwrap();
        assert_eq!(third_title.get(), Ok(Some("third title")));
        assert!(title_iter.next().is_none());

        // GenericIter
        let mut tag_list_iter = tags.iter_generic();
        assert_eq!(tag_list_iter.size_hint(), (2, Some(2)));

        let (tag_name, mut tag_iter) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, *TAG_TITLE);
        let first_title = tag_iter.next().unwrap();
        assert_eq!(first_title.get(), Ok(Some("some title")));
        let second_title = tag_iter.next().unwrap();
        assert_eq!(second_title.get(), Ok(Some("second title")));
        let third_title = tag_iter.next().unwrap();
        assert_eq!(third_title.get(), Ok(Some("third title")));
        assert!(tag_iter.next().is_none());

        let (tag_name, mut tag_iter) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, *TAG_DURATION);
        let first_duration = tag_iter.next().unwrap();
        assert_eq!(first_duration.get_some(), Ok(::SECOND * 120));
        assert!(tag_iter.next().is_none());

        // Iter
        let mut tag_list_iter = tags.iter();
        assert_eq!(tag_list_iter.size_hint(), (2, Some(2)));

        let (tag_name, tag_value) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, *TAG_TITLE);
        assert_eq!(
            tag_value.get(),
            Ok(Some("some title, second title, third title"))
        );

        let (tag_name, tag_value) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, *TAG_DURATION);
        assert_eq!(tag_value.get_some(), Ok(::SECOND * 120));
        assert!(tag_iter.next().is_none());
    }

    #[test]
    fn test_custom_tags() {
        ::init().unwrap();

        enum MyCustomTag {};

        impl<'a> Tag<'a> for MyCustomTag {
            type TagType = &'a str;
            fn tag_name<'b>() -> &'b str {
                "my-custom-tag"
            }
        }

        impl<'a> CustomTag<'a> for MyCustomTag {
            const FLAG: ::TagFlag = ::TagFlag::Meta;
            const NICK: &'static str = "my custom tag";
            const DESCRIPTION: &'static str = "My own custom tag type for testing";

            fn merge_func(src: &Value) -> Value {
                skip_assert_initialized!();
                merge_strings_with_comma(src)
            }
        }

        register::<MyCustomTag>();

        assert!(tag_exists(MyCustomTag::tag_name()));
        assert_eq!(
            tag_get_type(MyCustomTag::tag_name()),
            <MyCustomTag as Tag>::TagType::static_type()
        );
        assert_eq!(
            tag_get_nick(MyCustomTag::tag_name()),
            Some(MyCustomTag::NICK)
        );
        assert_eq!(
            tag_get_description(MyCustomTag::tag_name()),
            Some(MyCustomTag::DESCRIPTION)
        );
        assert_eq!(tag_get_flag(MyCustomTag::tag_name()), MyCustomTag::FLAG);

        let mut tags = TagList::new();
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<MyCustomTag>(&"first one", TagMergeMode::Append);
        }

        assert_eq!(tags.get::<MyCustomTag>().unwrap().get(), Some("first one"));

        {
            let tags = tags.get_mut().unwrap();
            tags.add::<MyCustomTag>(&"second one", TagMergeMode::Append);
        }

        assert_eq!(
            tags.get::<MyCustomTag>().unwrap().get(),
            Some("first one, second one")
        );
    }

    #[test]
    fn test_display() {
        ::init().unwrap();

        format!("{}", TagList::new());
    }
}
