// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use std::convert::TryFrom;
use std::fmt;
use std::mem;
use std::ptr;
use std::str;

#[doc(alias = "GstVideoContentLightLevel")]
#[derive(Copy, Clone)]
pub struct VideoContentLightLevel(ffi::GstVideoContentLightLevel);

impl VideoContentLightLevel {
    pub fn new(max_content_light_level: u16, max_frame_average_light_level: u16) -> Self {
        assert_initialized_main_thread!();

        VideoContentLightLevel(ffi::GstVideoContentLightLevel {
            max_content_light_level,
            max_frame_average_light_level,
            _gst_reserved: [ptr::null_mut(); 4],
        })
    }

    pub fn max_content_light_level(&self) -> u16 {
        self.0.max_content_light_level
    }

    pub fn set_max_content_light_level(&mut self, max_content_light_level: u16) {
        self.0.max_content_light_level = max_content_light_level;
    }

    pub fn max_frame_average_light_level(&self) -> u16 {
        self.0.max_frame_average_light_level
    }

    pub fn set_max_frame_average_light_level(&mut self, max_frame_average_light_level: u16) {
        self.0.max_frame_average_light_level = max_frame_average_light_level;
    }

    #[doc(alias = "gst_video_content_light_level_add_to_caps")]
    pub fn add_to_caps(&self, caps: &mut gst::CapsRef) {
        unsafe {
            ffi::gst_video_content_light_level_add_to_caps(&self.0, caps.as_mut_ptr());
        }
    }

    #[doc(alias = "gst_video_content_light_level_from_caps")]
    pub fn from_caps(caps: &gst::CapsRef) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        unsafe {
            let mut info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_content_light_level_from_caps(
                info.as_mut_ptr(),
                caps.as_ptr(),
            ));

            if res {
                Ok(VideoContentLightLevel(info.assume_init()))
            } else {
                Err(glib::bool_error!(
                    "Failed to parse VideoContentLightLevel from caps"
                ))
            }
        }
    }
}

impl<'a> TryFrom<&'a gst::CapsRef> for VideoContentLightLevel {
    type Error = glib::BoolError;

    fn try_from(value: &'a gst::CapsRef) -> Result<Self, Self::Error> {
        skip_assert_initialized!();

        Self::from_caps(value)
    }
}

impl PartialEq for VideoContentLightLevel {
    #[doc(alias = "gst_video_content_light_level_is_equal")]
    fn eq(&self, other: &Self) -> bool {
        #[cfg(feature = "v1_20")]
        unsafe {
            from_glib(ffi::gst_video_content_light_level_is_equal(
                &self.0, &other.0,
            ))
        }
        #[cfg(not(feature = "v1_20"))]
        {
            self.0.max_content_light_level == other.0.max_content_light_level
                && self.0.max_frame_average_light_level == other.0.max_frame_average_light_level
        }
    }
}

impl Eq for VideoContentLightLevel {}

impl str::FromStr for VideoContentLightLevel {
    type Err = glib::error::BoolError;

    #[doc(alias = "gst_video_content_light_level_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();

        unsafe {
            let mut colorimetry = mem::MaybeUninit::zeroed();
            let valid: bool = from_glib(ffi::gst_video_content_light_level_from_string(
                colorimetry.as_mut_ptr(),
                s.to_glib_none().0,
            ));
            if valid {
                Ok(Self(colorimetry.assume_init()))
            } else {
                Err(glib::bool_error!("Invalid colorimetry info"))
            }
        }
    }
}

impl fmt::Debug for VideoContentLightLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoContentLightLevel")
            .field("max_content_light_level", &self.0.max_content_light_level)
            .field(
                "max_frame_average_light_level",
                &self.0.max_frame_average_light_level,
            )
            .finish()
    }
}

impl fmt::Display for VideoContentLightLevel {
    #[doc(alias = "gst_video_content_light_level_to_string")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            glib::GString::from_glib_full(ffi::gst_video_content_light_level_to_string(&self.0))
        };
        f.write_str(&s)
    }
}

#[doc(alias = "GstVideoMasteringDisplayInfo")]
#[derive(Copy, Clone)]
pub struct VideoMasteringDisplayInfo(ffi::GstVideoMasteringDisplayInfo);

impl VideoMasteringDisplayInfo {
    pub fn new(
        display_primaries: [VideoMasteringDisplayInfoCoordinate; 3],
        white_point: VideoMasteringDisplayInfoCoordinate,
        max_display_mastering_luminance: u32,
        min_display_mastering_luminance: u32,
    ) -> Self {
        assert_initialized_main_thread!();

        VideoMasteringDisplayInfo(ffi::GstVideoMasteringDisplayInfo {
            display_primaries: unsafe { mem::transmute(display_primaries) },
            white_point: unsafe { mem::transmute(white_point) },
            max_display_mastering_luminance,
            min_display_mastering_luminance,
            _gst_reserved: [ptr::null_mut(); 4],
        })
    }

    pub fn display_primaries(&self) -> [VideoMasteringDisplayInfoCoordinate; 3] {
        unsafe { mem::transmute(self.0.display_primaries) }
    }

    pub fn set_display_primaries(
        &mut self,
        display_primaries: [VideoMasteringDisplayInfoCoordinate; 3],
    ) {
        self.0.display_primaries = unsafe { mem::transmute(display_primaries) };
    }

    pub fn white_point(&self) -> VideoMasteringDisplayInfoCoordinate {
        unsafe { mem::transmute(self.0.white_point) }
    }

    pub fn set_white_point(&mut self, white_point: VideoMasteringDisplayInfoCoordinate) {
        self.0.white_point = unsafe { mem::transmute(white_point) };
    }

    pub fn max_display_mastering_luminance(&self) -> u32 {
        self.0.max_display_mastering_luminance
    }

    pub fn set_max_display_mastering_luminance(&mut self, max_display_mastering_luminance: u32) {
        self.0.max_display_mastering_luminance = max_display_mastering_luminance;
    }

    pub fn min_display_mastering_luminance(&self) -> u32 {
        self.0.min_display_mastering_luminance
    }

    pub fn set_min_display_mastering_luminance(&mut self, min_display_mastering_luminance: u32) {
        self.0.min_display_mastering_luminance = min_display_mastering_luminance;
    }

    #[doc(alias = "gst_video_mastering_display_info_add_to_caps")]
    pub fn add_to_caps(&self, caps: &mut gst::CapsRef) {
        unsafe {
            ffi::gst_video_mastering_display_info_add_to_caps(&self.0, caps.as_mut_ptr());
        }
    }

    #[doc(alias = "gst_video_mastering_display_info_from_caps")]
    pub fn from_caps(caps: &gst::CapsRef) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        unsafe {
            let mut info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_mastering_display_info_from_caps(
                info.as_mut_ptr(),
                caps.as_ptr(),
            ));

            if res {
                Ok(VideoMasteringDisplayInfo(info.assume_init()))
            } else {
                Err(glib::bool_error!(
                    "Failed to parse VideoMasteringDisplayInfo from caps"
                ))
            }
        }
    }
}

impl<'a> TryFrom<&'a gst::CapsRef> for VideoMasteringDisplayInfo {
    type Error = glib::BoolError;

    fn try_from(value: &'a gst::CapsRef) -> Result<Self, Self::Error> {
        skip_assert_initialized!();

        Self::from_caps(value)
    }
}

impl PartialEq for VideoMasteringDisplayInfo {
    #[doc(alias = "gst_video_mastering_display_info_is_equal")]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_mastering_display_info_is_equal(
                &self.0, &other.0,
            ))
        }
    }
}

impl Eq for VideoMasteringDisplayInfo {}

impl str::FromStr for VideoMasteringDisplayInfo {
    type Err = glib::error::BoolError;

    #[doc(alias = "gst_video_mastering_display_info_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();

        unsafe {
            let mut colorimetry = mem::MaybeUninit::zeroed();
            let valid: bool = from_glib(ffi::gst_video_mastering_display_info_from_string(
                colorimetry.as_mut_ptr(),
                s.to_glib_none().0,
            ));
            if valid {
                Ok(Self(colorimetry.assume_init()))
            } else {
                Err(glib::bool_error!("Invalid colorimetry info"))
            }
        }
    }
}

impl fmt::Debug for VideoMasteringDisplayInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoMasteringDisplayInfo")
            .field("display_primaries", &self.display_primaries())
            .field("white_point", &self.white_point())
            .field(
                "max_display_mastering_luminance",
                &self.0.max_display_mastering_luminance,
            )
            .field(
                "min_display_mastering_luminance",
                &self.0.min_display_mastering_luminance,
            )
            .finish()
    }
}

impl fmt::Display for VideoMasteringDisplayInfo {
    #[doc(alias = "gst_video_mastering_display_info_to_string")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            glib::GString::from_glib_full(ffi::gst_video_mastering_display_info_to_string(&self.0))
        };
        f.write_str(&s)
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[doc(alias = "GstVideoMasteringDisplayInfoCoordinates")]
pub struct VideoMasteringDisplayInfoCoordinate {
    pub x: u16,
    pub y: u16,
}

impl VideoMasteringDisplayInfoCoordinate {
    pub fn new(x: f32, y: f32) -> Self {
        skip_assert_initialized!();

        Self {
            x: (x * 50000.0) as u16,
            y: (y * 50000.0) as u16,
        }
    }

    pub fn x(&self) -> f32 {
        self.x as f32 / 50000.0
    }

    pub fn y(&self) -> f32 {
        self.y as f32 / 50000.0
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = (x * 50000.0) as u16;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = (y * 50000.0) as u16;
    }
}

impl fmt::Debug for VideoMasteringDisplayInfoCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoMasteringDisplayInfoCoordinate")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}
