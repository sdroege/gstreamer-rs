// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
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

use ffi;
use glib;
use glib::translate::{from_glib, from_glib_full, ToGlib, ToGlibPtr, ToGlibPtrMut};
use glib::value::{FromValueOptional, SendValue, SetValue, ToSendValue, TypedValue};
use glib::StaticType;

use miniobject::*;

use Sample;
use TagError;
use TagMergeMode;

pub trait Tag<'a> {
    type TagType: FromValueOptional<'a> + SetValue + Send;
    fn tag_name<'b>() -> &'b str;
}

macro_rules! impl_tag(
    ($name:ident, $t:ty, $rust_tag:ident, $gst_tag:ident) => {
        pub struct $name;
        impl<'a> Tag<'a> for $name {
            type TagType = $t;
            fn tag_name<'b>() -> &'b str {
                *$rust_tag
            }
        }

        lazy_static! {
            static ref $rust_tag: &'static str =
                unsafe { CStr::from_ptr(ffi::$gst_tag).to_str().unwrap() };
        }
    };
);

impl_tag!(Title, &'a str, TAG_TITLE, GST_TAG_TITLE);
impl_tag!(TitleSortname, &'a str, TAG_TITLE_SORTNAME, GST_TAG_TITLE_SORTNAME);
impl_tag!(Artist, &'a str, TAG_ARTIST, GST_TAG_ARTIST);
impl_tag!(ArtistSortname, &'a str, TAG_ARTIST_SORTNAME, GST_TAG_ARTIST_SORTNAME);
impl_tag!(Album, &'a str, TAG_ALBUM, GST_TAG_ARTIST_SORTNAME);
impl_tag!(AlbumSortname, &'a str, TAG_ALBUM_SORTNAME, GST_TAG_ALBUM_SORTNAME);
impl_tag!(AlbumArtist, &'a str, TAG_ALBUM_ARTIST, GST_TAG_ALBUM_ARTIST);
impl_tag!(AlbumArtistSortname, &'a str, TAG_ALBUM_ARTIST_SORTNAME, GST_TAG_ALBUM_ARTIST_SORTNAME);
impl_tag!(Date, glib::Date, TAG_DATE, GST_TAG_DATE);
impl_tag!(DateTime, ::auto::DateTime, TAG_DATE_TIME, GST_TAG_DATE_TIME);
impl_tag!(Genre, &'a str, TAG_GENRE, GST_TAG_GENRE);
impl_tag!(Comment, &'a str, TAG_COMMENT, GST_TAG_COMMENT);
impl_tag!(ExtendedComment, &'a str, TAG_EXTENDED_COMMENT, GST_TAG_EXTENDED_COMMENT);
impl_tag!(TrackNumber, u32, TAG_TRACK_NUMBER, GST_TAG_TRACK_NUMBER);
impl_tag!(TrackCount, u32, TAG_TRACK_COUNT, GST_TAG_TRACK_COUNT);
impl_tag!(AlbumVolumeNumber, u32, TAG_ALBUM_VOLUME_NUMBER, GST_TAG_ALBUM_VOLUME_NUMBER);
impl_tag!(AlbumVolumeCount, u32, TAG_ALBUM_VOLUME_COUNT, GST_TAG_ALBUM_VOLUME_COUNT);
impl_tag!(Location, &'a str, TAG_LOCATION, GST_TAG_LOCATION);
impl_tag!(Homepage, &'a str, TAG_HOMEPAGE, GST_TAG_HOMEPAGE);
impl_tag!(Description, &'a str, TAG_DESCRIPTION, GST_TAG_DESCRIPTION);
impl_tag!(Version, &'a str, TAG_VERSION, GST_TAG_VERSION);
impl_tag!(ISRC, &'a str, TAG_ISRC, GST_TAG_ISRC);
impl_tag!(Organization, &'a str, TAG_ORGANIZATION, GST_TAG_ORGANIZATION);
impl_tag!(Copyright, &'a str, TAG_COPYRIGHT, GST_TAG_COPYRIGHT);
impl_tag!(CopyrightUri, &'a str, TAG_COPYRIGHT_URI, GST_TAG_COPYRIGHT_URI);
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
impl_tag!(SubtitleCodec, &'a str, TAG_SUBTITLE_CODEC, GST_TAG_SUBTITLE_CODEC);
impl_tag!(ContainerFormat, &'a str, TAG_CONTAINER_FORMAT, GST_TAG_CONTAINER_FORMAT);
impl_tag!(Bitrate, u32, TAG_BITRATE, GST_TAG_BITRATE);
impl_tag!(NominalBitrate, u32, TAG_NOMINAL_BITRATE, GST_TAG_NOMINAL_BITRATE);
impl_tag!(MinimumBitrate, u32, TAG_MINIMUM_BITRATE, GST_TAG_MINIMUM_BITRATE);
impl_tag!(MaximumBitrate, u32, TAG_MAXIMUM_BITRATE, GST_TAG_MAXIMUM_BITRATE);
impl_tag!(Serial, u32, TAG_SERIAL, GST_TAG_SERIAL);
impl_tag!(Encoder, &'a str, TAG_ENCODER, GST_TAG_ENCODER);
impl_tag!(EncoderVersion, u32, TAG_ENCODER_VERSION, GST_TAG_ENCODER_VERSION);
impl_tag!(TrackGain, f64, TAG_TRACK_GAIN, GST_TAG_TRACK_GAIN);
impl_tag!(TrackPeak, f64, TAG_TRACK_PEAK, GST_TAG_TRACK_PEAK);
impl_tag!(AlbumGain, f64, TAG_ALBUM_GAIN, GST_TAG_ALBUM_GAIN);
impl_tag!(AlbumPeak, f64, TAG_ALBUM_PEAK, GST_TAG_ALBUM_PEAK);
impl_tag!(ReferenceLevel, f64, TAG_REFERENCE_LEVEL, GST_TAG_REFERENCE_LEVEL);
// TODO: Should ideally enforce this to be ISO-639
impl_tag!(LanguageCode, &'a str, TAG_LANGUAGE_CODE, GST_TAG_LANGUAGE_CODE);
impl_tag!(LanguageName, &'a str, TAG_LANGUAGE_NAME, GST_TAG_LANGUAGE_NAME);
impl_tag!(Image, Sample, TAG_IMAGE, GST_TAG_IMAGE);
impl_tag!(PreviewImage, Sample, TAG_PREVIEW_IMAGE, GST_TAG_PREVIEW_IMAGE);
impl_tag!(Attachment, Sample, TAG_ATTACHMENT, GST_TAG_ATTACHMENT);
impl_tag!(BeatsPerMinute, f64, TAG_BEATS_PER_MINUTE, GST_TAG_BEATS_PER_MINUTE);
impl_tag!(Keywords, &'a str, TAG_KEYWORDS, GST_TAG_KEYWORDS);
impl_tag!(GeoLocationName, &'a str, TAG_GEO_LOCATION_NAME, GST_TAG_GEO_LOCATION_NAME);
impl_tag!(GeoLocationLatitude, f64, TAG_GEO_LOCATION_LATITUDE, GST_TAG_GEO_LOCATION_LATITUDE);
impl_tag!(GeoLocationLongitute, f64, TAG_GEO_LOCATION_LONGITUDE, GST_TAG_GEO_LOCATION_LONGITUDE);
impl_tag!(GeoLocationElevation, f64, TAG_GEO_LOCATION_ELEVATION, GST_TAG_GEO_LOCATION_ELEVATION);
impl_tag!(GeoLocationCity, &'a str, TAG_GEO_LOCATION_CITY, GST_TAG_GEO_LOCATION_CITY);
impl_tag!(GeoLocationCountry, &'a str, TAG_GEO_LOCATION_COUNTRY, GST_TAG_GEO_LOCATION_COUNTRY);
impl_tag!(
    GeoLocationSublocation,
    &'a str,
    TAG_GEO_LOCATION_SUBLOCATION, GST_TAG_GEO_LOCATION_SUBLOCATION
);
impl_tag!(
    GeoLocationHorizontalError,
    f64,
    TAG_GEO_LOCATION_HORIZONTAL_ERROR, GST_TAG_GEO_LOCATION_HORIZONTAL_ERROR
);
impl_tag!(
    GeoLocationMovementDirection,
    f64,
    TAG_GEO_LOCATION_MOVEMENT_DIRECTION, GST_TAG_GEO_LOCATION_MOVEMENT_DIRECTION
);
impl_tag!(
    GeoLocationMovementSpeed,
    f64,
    TAG_GEO_LOCATION_MOVEMENT_SPEED, GST_TAG_GEO_LOCATION_MOVEMENT_SPEED
);
impl_tag!(
    GeoLocationCaptureDirection,
    f64,
    TAG_GEO_LOCATION_CAPTURE_DIRECTION, GST_TAG_GEO_LOCATION_CAPTURE_DIRECTION
);
impl_tag!(ShowName, &'a str, TAG_SHOW_NAME, GST_TAG_SHOW_NAME);
impl_tag!(ShowSortname, &'a str, TAG_SHOW_SORTNAME, GST_TAG_SHOW_SORTNAME);
impl_tag!(ShowEpisodeNumber, u32, TAG_SHOW_EPISODE_NUMBER, GST_TAG_SHOW_EPISODE_NUMBER);
impl_tag!(ShowSeasonNumber, u32, TAG_SHOW_SEASON_NUMBER, GST_TAG_SHOW_SEASON_NUMBER);
impl_tag!(Lyrics, &'a str, TAG_LYRICS, GST_TAG_LYRICS);
impl_tag!(ComposerSortname, &'a str, TAG_COMPOSER_SORTNAME, GST_TAG_COMPOSER_SORTNAME);
impl_tag!(Grouping, &'a str, TAG_GROUPING, GST_TAG_GROUPING);
impl_tag!(UserRating, u32, TAG_USER_RATING, GST_TAG_USER_RATING);
impl_tag!(DeviceManufacturer, &'a str, TAG_DEVICE_MANUFACTURER, GST_TAG_DEVICE_MANUFACTURER);
impl_tag!(DeviceModel, &'a str, TAG_DEVICE_MODEL, GST_TAG_DEVICE_MODEL);
impl_tag!(ApplicationName, &'a str, TAG_APPLICATION_NAME, GST_TAG_APPLICATION_NAME);
impl_tag!(ApplicationData, Sample, TAG_APPLICATION_DATA, GST_TAG_APPLICATION_DATA);
impl_tag!(ImageOrientation, &'a str, TAG_IMAGE_ORIENTATION, GST_TAG_IMAGE_ORIENTATION);
impl_tag!(Publisher, &'a str, TAG_PUBLISHER, GST_TAG_PUBLISHER);
impl_tag!(InterpretedBy, &'a str, TAG_INTERPRETED_BY, GST_TAG_INTERPRETED_BY);
impl_tag!(MidiBaseNote, &'a str, TAG_MIDI_BASE_NOTE, GST_TAG_MIDI_BASE_NOTE);
impl_tag!(PrivateData, Sample, TAG_PRIVATE_DATA, GST_TAG_PRIVATE_DATA);

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
    pub fn add<'a, T: Tag<'a>>(&mut self, value: &T::TagType, mode: TagMergeMode)
    where
        T::TagType: ToSendValue,
    {
        // result can be safely ignored here as `value`'s type is tied to `T::tag_name()`
        let _res = self.add_generic(T::tag_name(), value, mode);
    }

    pub fn add_generic<T>(
        &mut self,
        tag_name : &str,
        value: &T,
        mode: TagMergeMode,
    ) -> Result<(), TagError>
    where
        T: ToSendValue
    {
        unsafe {
            let v = value.to_send_value();

            let tag_type: glib::Type = from_glib(
                ffi::gst_tag_get_type(tag_name.as_ptr() as *const i8)
            );
            if tag_type != v.type_() {
                return Err(TagError::TypeMismatch)
            }

            ffi::gst_tag_list_add_value(
                self.as_mut_ptr(),
                mode.to_glib(),
                tag_name.to_glib_none().0,
                v.to_glib_none().0,
            );
        }

        Ok(())
    }

    pub fn get<'a, T: Tag<'a>>(&self) -> Option<TypedValue<T::TagType>> {
        self.get_generic(T::tag_name())
            .and_then(|value| value.downcast().ok())
    }

    pub fn get_generic(&self, tag_name: &str) -> Option<SendValue> {
        unsafe {
            let mut value: SendValue = mem::zeroed();

            let found: bool = from_glib(ffi::gst_tag_list_copy_value(
                value.to_glib_none_mut().0,
                self.as_ptr(),
                tag_name.to_glib_none().0,
            ));

            if !found {
                return None;
            }

            Some(value)
        }
    }

    pub fn n_tags(&self) -> i32 {
        unsafe { ffi::gst_tag_list_n_tags(self.as_ptr()) }
    }

    pub fn nth_tag_name(&self, idx: u32) -> &str {
        unsafe { CStr::from_ptr(ffi::gst_tag_list_nth_tag_name(self.as_ptr(), idx)).to_str().unwrap() }
    }

    pub fn get_index<'a, T: Tag<'a>>(&'a self, idx: u32) -> Option<&'a TypedValue<T::TagType>> {
        self.get_index_generic(T::tag_name(), idx)
            .and_then(|value| value.downcast_ref())
    }

    pub fn get_index_generic<'a>(&'a self, tag_name: &str, idx: u32) -> Option<&'a SendValue> {
        unsafe {
            let value = ffi::gst_tag_list_get_value_index(
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
        unsafe { ffi::gst_tag_list_get_tag_size(self.as_ptr(), tag_name.to_glib_none().0) }
    }

    pub fn iter_tag<'a, T: Tag<'a>>(&'a self) -> TagIterator<'a, T> {
        TagIterator::new(self)
    }

    pub fn iter_tag_generic<'a>(&'a self, tag_name: &'a str) -> GenericTagIterator<'a> {
        GenericTagIterator::new(self, tag_name)
    }

    pub fn iter_tag_list(&self) -> TagListIterator {
        TagListIterator::new(self)
    }

    pub fn to_string(&self) -> String {
        unsafe { from_glib_full(ffi::gst_tag_list_to_string(self.as_ptr())) }
    }

    pub fn insert(&mut self, other: &TagListRef, mode: TagMergeMode) {
        unsafe { ffi::gst_tag_list_insert(self.as_mut_ptr(), other.as_ptr(), mode.to_glib()) }
    }

    pub fn merge(&self, other: &TagListRef, mode: TagMergeMode) -> TagList {
        unsafe {
            from_glib_full(ffi::gst_tag_list_merge(
                self.as_ptr(),
                other.as_ptr(),
                mode.to_glib(),
            ))
        }
    }
}

impl fmt::Debug for TagListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("TagList").field(&self.to_string()).finish()
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
        unsafe { from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _) }
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
            taglist,
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

pub struct GenericTagIterator<'a> {
    taglist: &'a TagListRef,
    name: &'a str,
    idx: u32,
    size: u32,
}

impl<'a> GenericTagIterator<'a> {
    fn new(taglist: &'a TagListRef, name: &'a str) -> GenericTagIterator<'a> {
        skip_assert_initialized!();
        GenericTagIterator {
            taglist,
            name,
            idx: 0,
            size: taglist.get_size_by_name(name),
        }
    }
}

impl<'a> Iterator for GenericTagIterator<'a> {
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

impl<'a> DoubleEndedIterator for GenericTagIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        self.taglist.get_index_generic(self.name, self.size)
    }
}

impl<'a> ExactSizeIterator for GenericTagIterator<'a> {
}

pub struct TagListIterator<'a> {
    taglist: &'a TagListRef,
    idx: u32,
    size: u32,
}

impl<'a> TagListIterator<'a> {
    fn new(taglist: &'a TagListRef) -> TagListIterator<'a> {
        skip_assert_initialized!();
        let size = taglist.n_tags();
        TagListIterator {
            taglist,
            idx: 0,
            size: if size > 0 {
                size as u32
            } else {
                0
            },
        }
    }
}

impl<'a> Iterator for TagListIterator<'a> {
    type Item = (&'a str, GenericTagIterator<'a>);

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

impl<'a> DoubleEndedIterator for TagListIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        let name = self.taglist.nth_tag_name(self.idx);
        Some((name, self.taglist.iter_tag_generic(name)))
    }
}

impl<'a> ExactSizeIterator for TagListIterator<'a> {
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
            tags.add::<Duration>(&(::SECOND * 120).into(), TagMergeMode::Append);
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
            tags.add::<Duration>(&(::SECOND * 120).into(), TagMergeMode::Append);
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
    fn test_generic() {
        ::init().unwrap();

        let mut tags = TagList::new();
        {
            let tags = tags.get_mut().unwrap();
            assert!(tags.add_generic(&TAG_TITLE, &"some title", TagMergeMode::Append).is_ok());
            assert!(tags.add_generic(&TAG_TITLE, &"second title", TagMergeMode::Append).is_ok());
            assert!(tags.add_generic(&TAG_DURATION, &(::SECOND * 120), TagMergeMode::Append).is_ok());
            assert!(tags.add_generic(&TAG_TITLE, &"third title", TagMergeMode::Append).is_ok());

            assert_eq!(
                tags.add_generic(
                    &TAG_IMAGE,
                    &"`&[str] instead of `Sample`",
                    TagMergeMode::Append
                ),
                Err(TagError::TypeMismatch),
            );
        }

        assert_eq!(tags.get_index_generic(&TAG_TITLE, 0).unwrap().get(), Some("some title"));
        assert_eq!(tags.get_index_generic(&TAG_TITLE, 1).unwrap().get(), Some("second title"));
        assert_eq!(tags.get_index_generic(&TAG_DURATION, 0).unwrap().get(), Some(::SECOND * 120));
        assert_eq!(tags.get_index_generic(&TAG_TITLE, 2).unwrap().get(), Some("third title"));

        assert_eq!(
            tags.get_generic(&TAG_TITLE).unwrap().get(),
            Some("some title, second title, third title"),
        );

        assert_eq!(tags.n_tags(), 2);
        assert_eq!(tags.nth_tag_name(0), *TAG_TITLE);
        assert_eq!(tags.get_size_by_name(&TAG_TITLE), 3);
        assert_eq!(tags.nth_tag_name(1), *TAG_DURATION);
        assert_eq!(tags.get_size_by_name(&TAG_DURATION), 1);

        // GenericTagIterator
        let mut title_iter = tags.iter_tag_generic(&TAG_TITLE);
        assert_eq!(title_iter.size_hint(), (3, Some(3)));
        let first_title = title_iter.next().unwrap();
        assert_eq!(first_title.get(), Some("some title"));
        let second_title = title_iter.next().unwrap();
        assert_eq!(second_title.get(), Some("second title"));
        let third_title = title_iter.next().unwrap();
        assert_eq!(third_title.get(), Some("third title"));
        assert!(title_iter.next().is_none());

        // TagListIterator
        let mut tag_list_iter = tags.iter_tag_list();
        assert_eq!(tag_list_iter.size_hint(), (2, Some(2)));

        let (tag_name, mut tag_iter) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, *TAG_TITLE);
        let first_title = tag_iter.next().unwrap();
        assert_eq!(first_title.get(), Some("some title"));
        let second_title = tag_iter.next().unwrap();
        assert_eq!(second_title.get(), Some("second title"));
        let third_title = tag_iter.next().unwrap();
        assert_eq!(third_title.get(), Some("third title"));
        assert!(tag_iter.next().is_none());

        let (tag_name, mut tag_iter) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, *TAG_DURATION);
        let first_duration = tag_iter.next().unwrap();
        assert_eq!(first_duration.get(), Some(::SECOND * 120));
        assert!(tag_iter.next().is_none());
    }
}
