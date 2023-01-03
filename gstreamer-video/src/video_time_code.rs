// Take a look at the license at the top of the repository in the LICENSE file.

use std::{cmp, fmt, mem, str};

use glib::translate::*;
use gst::prelude::*;

use crate::{VideoTimeCodeFlags, VideoTimeCodeInterval};

glib::wrapper! {
    #[doc(alias = "GstVideoTimeCode")]
    pub struct VideoTimeCode(BoxedInline<ffi::GstVideoTimeCode>);

    match fn {
        copy => |ptr| ffi::gst_video_time_code_copy(ptr),
        free => |ptr| ffi::gst_video_time_code_free(ptr),
        init => |_ptr| (),
        copy_into => |dest, src| {
            *dest = *src;
            if !(*dest).config.latest_daily_jam.is_null() {
                glib::ffi::g_date_time_ref((*dest).config.latest_daily_jam);
            }
        },
        clear => |ptr| {
            if !(*ptr).config.latest_daily_jam.is_null() {
                glib::ffi::g_date_time_unref((*ptr).config.latest_daily_jam);
            }
        },
        type_ => || ffi::gst_video_time_code_get_type(),
    }
}

glib::wrapper! {
    #[doc(alias = "GstVideoTimeCode")]
    pub struct ValidVideoTimeCode(BoxedInline<ffi::GstVideoTimeCode>);

    match fn {
        copy => |ptr| ffi::gst_video_time_code_copy(ptr),
        free => |ptr| ffi::gst_video_time_code_free(ptr),
        init => |_ptr| (),
        copy_into => |dest, src| {
            *dest = *src;
            if !(*dest).config.latest_daily_jam.is_null() {
                glib::ffi::g_date_time_ref((*dest).config.latest_daily_jam);
            }
        },
        clear => |ptr| {
            if !(*ptr).config.latest_daily_jam.is_null() {
                glib::ffi::g_date_time_unref((*ptr).config.latest_daily_jam);
            }
        },
    }
}

impl VideoTimeCode {
    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut v = mem::MaybeUninit::zeroed();
            ffi::gst_video_time_code_clear(v.as_mut_ptr());
            Self {
                inner: v.assume_init(),
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fps: gst::Fraction,
        latest_daily_jam: Option<&glib::DateTime>,
        flags: VideoTimeCodeFlags,
        hours: u32,
        minutes: u32,
        seconds: u32,
        frames: u32,
        field_count: u32,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut v = mem::MaybeUninit::zeroed();
            ffi::gst_video_time_code_init(
                v.as_mut_ptr(),
                fps.numer() as u32,
                fps.denom() as u32,
                latest_daily_jam.to_glib_none().0,
                flags.into_glib(),
                hours,
                minutes,
                seconds,
                frames,
                field_count,
            );

            Self {
                inner: v.assume_init(),
            }
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_video_time_code_init_from_date_time_full")]
    pub fn from_date_time(
        fps: gst::Fraction,
        dt: &glib::DateTime,
        flags: VideoTimeCodeFlags,
        field_count: u32,
    ) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        assert!(fps.denom() > 0);
        unsafe {
            let mut v = mem::MaybeUninit::zeroed();
            let res = ffi::gst_video_time_code_init_from_date_time_full(
                v.as_mut_ptr(),
                fps.numer() as u32,
                fps.denom() as u32,
                dt.to_glib_none().0,
                flags.into_glib(),
                field_count,
            );

            if res == glib::ffi::GFALSE {
                Err(glib::bool_error!("Failed to init video time code"))
            } else {
                Ok(Self {
                    inner: v.assume_init(),
                })
            }
        }
    }

    #[doc(alias = "gst_video_time_code_is_valid")]
    pub fn is_valid(&self) -> bool {
        unsafe { from_glib(ffi::gst_video_time_code_is_valid(self.to_glib_none().0)) }
    }

    pub fn set_fps(&mut self, fps: gst::Fraction) {
        self.inner.config.fps_n = fps.numer() as u32;
        self.inner.config.fps_d = fps.denom() as u32;
    }

    pub fn set_flags(&mut self, flags: VideoTimeCodeFlags) {
        self.inner.config.flags = flags.into_glib()
    }

    pub fn set_hours(&mut self, hours: u32) {
        self.inner.hours = hours
    }

    pub fn set_minutes(&mut self, minutes: u32) {
        assert!(minutes < 60);
        self.inner.minutes = minutes
    }

    pub fn set_seconds(&mut self, seconds: u32) {
        assert!(seconds < 60);
        self.inner.seconds = seconds
    }

    pub fn set_frames(&mut self, frames: u32) {
        self.inner.frames = frames
    }

    pub fn set_field_count(&mut self, field_count: u32) {
        assert!(field_count <= 2);
        self.inner.field_count = field_count
    }
}

impl TryFrom<VideoTimeCode> for ValidVideoTimeCode {
    type Error = VideoTimeCode;

    fn try_from(v: VideoTimeCode) -> Result<Self, VideoTimeCode> {
        skip_assert_initialized!();
        if v.is_valid() {
            // Use ManuallyDrop here to prevent the Drop impl of VideoTimeCode
            // from running as we don't move v.0 out here but copy it.
            // GstVideoTimeCode implements Copy.
            let v = mem::ManuallyDrop::new(v);
            Ok(Self { inner: v.inner })
        } else {
            Err(v)
        }
    }
}

impl ValidVideoTimeCode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fps: gst::Fraction,
        latest_daily_jam: Option<&glib::DateTime>,
        flags: VideoTimeCodeFlags,
        hours: u32,
        minutes: u32,
        seconds: u32,
        frames: u32,
        field_count: u32,
    ) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        let tc = VideoTimeCode::new(
            fps,
            latest_daily_jam,
            flags,
            hours,
            minutes,
            seconds,
            frames,
            field_count,
        );
        match tc.try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(glib::bool_error!("Failed to create new ValidVideoTimeCode")),
        }
    }

    //    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    //    pub fn from_date_time(
    //        fps: gst::Fraction,
    //        dt: &glib::DateTime,
    //        flags: VideoTimeCodeFlags,
    //        field_count: u32,
    //    ) -> Option<VideoTimeCode> {
    //        let tc = VideoTimeCode::from_date_time(fps, dt, flags, field_count);
    //        tc.and_then(|tc| tc.try_into().ok())
    //    }

    #[doc(alias = "gst_video_time_code_add_frames")]
    pub fn add_frames(&mut self, frames: i64) {
        skip_assert_initialized!();
        unsafe {
            ffi::gst_video_time_code_add_frames(self.to_glib_none_mut().0, frames);
        }
    }

    #[doc(alias = "gst_video_time_code_add_interval")]
    pub fn add_interval(
        &self,
        tc_inter: &VideoTimeCodeInterval,
    ) -> Result<Self, glib::error::BoolError> {
        unsafe {
            match from_glib_full(ffi::gst_video_time_code_add_interval(
                self.to_glib_none().0,
                tc_inter.to_glib_none().0,
            )) {
                Some(i) => Ok(i),
                None => Err(glib::bool_error!("Failed to add interval")),
            }
        }
    }

    #[doc(alias = "gst_video_time_code_compare")]
    fn compare(&self, tc2: &Self) -> i32 {
        unsafe { ffi::gst_video_time_code_compare(self.to_glib_none().0, tc2.to_glib_none().0) }
    }

    #[doc(alias = "gst_video_time_code_frames_since_daily_jam")]
    pub fn frames_since_daily_jam(&self) -> u64 {
        unsafe { ffi::gst_video_time_code_frames_since_daily_jam(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_video_time_code_increment_frame")]
    pub fn increment_frame(&mut self) {
        unsafe {
            ffi::gst_video_time_code_increment_frame(self.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gst_video_time_code_nsec_since_daily_jam")]
    #[doc(alias = "nsec_since_daily_jam")]
    pub fn time_since_daily_jam(&self) -> gst::ClockTime {
        gst::ClockTime::from_nseconds(unsafe {
            ffi::gst_video_time_code_nsec_since_daily_jam(self.to_glib_none().0)
        })
    }

    #[doc(alias = "gst_video_time_code_to_date_time")]
    pub fn to_date_time(&self) -> Result<glib::DateTime, glib::error::BoolError> {
        unsafe {
            match from_glib_full(ffi::gst_video_time_code_to_date_time(self.to_glib_none().0)) {
                Some(d) => Ok(d),
                None => Err(glib::bool_error!(
                    "Failed to convert VideoTimeCode to date time"
                )),
            }
        }
    }
}

macro_rules! generic_impl {
    ($name:ident) => {
        impl $name {
            pub fn hours(&self) -> u32 {
                self.inner.hours
            }

            pub fn minutes(&self) -> u32 {
                self.inner.minutes
            }

            pub fn seconds(&self) -> u32 {
                self.inner.seconds
            }

            pub fn frames(&self) -> u32 {
                self.inner.frames
            }

            pub fn field_count(&self) -> u32 {
                self.inner.field_count
            }

            pub fn fps(&self) -> gst::Fraction {
                (
                    self.inner.config.fps_n as i32,
                    self.inner.config.fps_d as i32,
                )
                    .into()
            }

            pub fn flags(&self) -> VideoTimeCodeFlags {
                unsafe { from_glib(self.inner.config.flags) }
            }

            pub fn latest_daily_jam(&self) -> Option<glib::DateTime> {
                unsafe { from_glib_none(self.inner.config.latest_daily_jam) }
            }

            pub fn set_latest_daily_jam(&mut self, latest_daily_jam: Option<&glib::DateTime>) {
                unsafe {
                    if !self.inner.config.latest_daily_jam.is_null() {
                        glib::ffi::g_date_time_unref(self.inner.config.latest_daily_jam);
                    }

                    self.inner.config.latest_daily_jam = latest_daily_jam.to_glib_full()
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("fps", &self.fps())
                    .field("flags", &self.flags())
                    .field("latest_daily_jam", &self.latest_daily_jam())
                    .field("hours", &self.hours())
                    .field("minutes", &self.minutes())
                    .field("seconds", &self.seconds())
                    .field("frames", &self.frames())
                    .field("field_count", &self.field_count())
                    .finish()
            }
        }

        impl fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let s = unsafe {
                    glib::GString::from_glib_full(ffi::gst_video_time_code_to_string(
                        self.to_glib_none().0,
                    ))
                };
                f.write_str(&s)
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
    };
}

generic_impl!(VideoTimeCode);
generic_impl!(ValidVideoTimeCode);

impl StaticType for ValidVideoTimeCode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_time_code_get_type()) }
    }
}

#[doc(hidden)]
impl glib::value::ToValue for ValidVideoTimeCode {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<VideoTimeCode>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[doc(hidden)]
impl glib::value::ToValueOptional for ValidVideoTimeCode {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<VideoTimeCode>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            )
        }
        value
    }
}

#[doc(hidden)]
impl From<ValidVideoTimeCode> for glib::Value {
    fn from(v: ValidVideoTimeCode) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

impl str::FromStr for VideoTimeCode {
    type Err = glib::error::BoolError;

    #[doc(alias = "gst_video_time_code_new_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<Self>::from_glib_full(ffi::gst_video_time_code_new_from_string(
                s.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create VideoTimeCode from string"))
        }
    }
}

impl PartialEq for ValidVideoTimeCode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == 0
    }
}

impl Eq for ValidVideoTimeCode {}

impl PartialOrd for ValidVideoTimeCode {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for ValidVideoTimeCode {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl From<ValidVideoTimeCode> for VideoTimeCode {
    fn from(v: ValidVideoTimeCode) -> Self {
        skip_assert_initialized!();
        // Use ManuallyDrop here to prevent the Drop impl of VideoTimeCode
        // from running as we don't move v.0 out here but copy it.
        // GstVideoTimeCode implements Copy.
        let v = mem::ManuallyDrop::new(v);
        Self { inner: v.inner }
    }
}

#[repr(transparent)]
#[doc(alias = "GstVideoTimeCodeMeta")]
pub struct VideoTimeCodeMeta(ffi::GstVideoTimeCodeMeta);

unsafe impl Send for VideoTimeCodeMeta {}
unsafe impl Sync for VideoTimeCodeMeta {}

impl VideoTimeCodeMeta {
    #[doc(alias = "gst_buffer_add_video_time_code_meta")]
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        tc: &ValidVideoTimeCode,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_video_time_code_meta(
                buffer.as_mut_ptr(),
                tc.to_glib_none().0 as *mut _,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_tc")]
    pub fn tc(&self) -> ValidVideoTimeCode {
        unsafe { ValidVideoTimeCode::from_glib_none(&self.0.tc as *const _) }
    }

    pub fn set_tc(&mut self, tc: ValidVideoTimeCode) {
        #![allow(clippy::cast_ptr_alignment)]
        unsafe {
            ffi::gst_video_time_code_clear(&mut self.0.tc);
            // Use ManuallyDrop here to prevent the Drop impl of VideoTimeCode
            // from running as we don't move tc.0 out here but copy it.
            // GstVideoTimeCode implements Copy.
            let tc = mem::ManuallyDrop::new(tc);
            self.0.tc = tc.inner;
        }
    }
}

unsafe impl MetaAPI for VideoTimeCodeMeta {
    type GstType = ffi::GstVideoTimeCodeMeta;

    #[doc(alias = "gst_video_time_code_meta_api_get_type")]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_time_code_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoTimeCodeMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoTimeCodeMeta")
            .field("tc", &self.tc())
            .finish()
    }
}

#[cfg(feature = "v1_16")]
#[cfg(test)]
mod tests {
    #[test]
    fn test_add_get_set_meta() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::new();
        {
            let datetime =
                glib::DateTime::from_utc(2021, 2, 4, 10, 53, 17.0).expect("can't create datetime");
            let time_code = crate::VideoTimeCode::from_date_time(
                gst::Fraction::new(30, 1),
                &datetime,
                crate::VideoTimeCodeFlags::empty(),
                0,
            )
            .expect("can't create timecode");
            drop(datetime);

            let mut meta = crate::VideoTimeCodeMeta::add(
                buffer.get_mut().unwrap(),
                &time_code.try_into().expect("invalid timecode"),
            );

            let datetime =
                glib::DateTime::from_utc(2021, 2, 4, 10, 53, 17.0).expect("can't create datetime");
            let mut time_code_2 = crate::ValidVideoTimeCode::try_from(
                crate::VideoTimeCode::from_date_time(
                    gst::Fraction::new(30, 1),
                    &datetime,
                    crate::VideoTimeCodeFlags::empty(),
                    0,
                )
                .expect("can't create timecode"),
            )
            .expect("invalid timecode");

            assert_eq!(meta.tc(), time_code_2);

            time_code_2.increment_frame();

            assert_eq!(meta.tc().frames() + 1, time_code_2.frames());

            meta.set_tc(time_code_2.clone());

            assert_eq!(meta.tc(), time_code_2);
        }
    }
}
