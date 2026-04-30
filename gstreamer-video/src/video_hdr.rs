// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, mem, ptr, str};

#[cfg(feature = "v1_30")]
use crate::VideoHDRFormat;
use crate::ffi;
use glib::translate::*;
#[cfg(feature = "v1_30")]
use gst::{MetaAPI, MetaAPIExt};

#[doc(alias = "GstVideoContentLightLevel")]
#[derive(Copy, Clone)]
pub struct VideoContentLightLevel(ffi::GstVideoContentLightLevel);

impl VideoContentLightLevel {
    pub fn new(max_content_light_level: u16, max_frame_average_light_level: u16) -> Self {
        skip_assert_initialized!();

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
            let mut info = mem::MaybeUninit::uninit();
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
            let mut colorimetry = mem::MaybeUninit::uninit();
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
        skip_assert_initialized!();

        VideoMasteringDisplayInfo(ffi::GstVideoMasteringDisplayInfo {
            display_primaries: unsafe {
                mem::transmute::<
                    [VideoMasteringDisplayInfoCoordinate; 3],
                    [ffi::GstVideoMasteringDisplayInfoCoordinates; 3],
                >(display_primaries)
            },
            white_point: unsafe {
                mem::transmute::<
                    VideoMasteringDisplayInfoCoordinate,
                    ffi::GstVideoMasteringDisplayInfoCoordinates,
                >(white_point)
            },
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
        self.0.display_primaries = unsafe {
            mem::transmute::<
                [VideoMasteringDisplayInfoCoordinate; 3],
                [ffi::GstVideoMasteringDisplayInfoCoordinates; 3],
            >(display_primaries)
        };
    }

    pub fn white_point(&self) -> VideoMasteringDisplayInfoCoordinate {
        unsafe { mem::transmute(self.0.white_point) }
    }

    pub fn set_white_point(&mut self, white_point: VideoMasteringDisplayInfoCoordinate) {
        self.0.white_point = unsafe {
            mem::transmute::<
                VideoMasteringDisplayInfoCoordinate,
                ffi::GstVideoMasteringDisplayInfoCoordinates,
            >(white_point)
        };
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
            let mut info = mem::MaybeUninit::uninit();
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
            let mut colorimetry = mem::MaybeUninit::uninit();
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

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl fmt::Display for VideoHDRFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            glib::GString::from_glib_full(ffi::gst_video_hdr_format_to_string(self.into_glib()))
        };
        f.write_str(&s)
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl str::FromStr for VideoHDRFormat {
    type Err = glib::BoolError;

    #[doc(alias = "gst_video_hdr_format_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();

        let format = unsafe { ffi::gst_video_hdr_format_from_string(s.to_glib_none().0) };

        if format == ffi::GST_VIDEO_HDR_FORMAT_NONE && s != "none" {
            Err(glib::bool_error!("Invalid HDR format: {}", s))
        } else {
            Ok(unsafe { from_glib(format) })
        }
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[doc(alias = "GstVideoHDR10Plus")]
#[derive(Copy, Clone)]
pub struct VideoHDR10Plus(ffi::GstVideoHDR10Plus);

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl VideoHDR10Plus {
    #[doc(alias = "gst_video_hdr_parse_hdr10_plus")]
    pub fn parse(data: &[u8]) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        let mut hdr10_plus = mem::MaybeUninit::uninit();
        let ret: bool = unsafe {
            from_glib(ffi::gst_video_hdr_parse_hdr10_plus(
                data.as_ptr(),
                data.len() as _,
                hdr10_plus.as_mut_ptr(),
            ))
        };

        if ret {
            Ok(VideoHDR10Plus(unsafe { hdr10_plus.assume_init() }))
        } else {
            Err(glib::bool_error!("Failed to parse HDR10+ data"))
        }
    }

    pub fn application_identifier(&self) -> u8 {
        self.0.application_identifier
    }

    pub fn application_version(&self) -> u8 {
        self.0.application_version
    }

    pub fn num_windows(&self) -> u8 {
        self.0.num_windows
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl fmt::Debug for VideoHDR10Plus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoHDR10Plus")
            .field("application_identifier", &self.application_identifier())
            .field("application_version", &self.application_version())
            .field("num_windows", &self.num_windows())
            .finish()
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[repr(transparent)]
#[doc(alias = "GstVideoHDRMeta")]
pub struct VideoHDRMeta(ffi::GstVideoHDRMeta);

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
unsafe impl Send for VideoHDRMeta {}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
unsafe impl Sync for VideoHDRMeta {}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl VideoHDRMeta {
    #[doc(alias = "gst_buffer_add_video_hdr_meta")]
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        format: VideoHDRFormat,
        data: &[u8],
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        unsafe {
            let meta_ptr = ffi::gst_buffer_add_video_hdr_meta(
                buffer.as_mut_ptr(),
                format.into_glib(),
                data.as_ptr(),
                data.len(),
            );
            Self::from_mut_ptr(buffer, meta_ptr)
        }
    }

    #[doc(alias = "get_format")]
    pub fn format(&self) -> VideoHDRFormat {
        unsafe { from_glib(self.0.format) }
    }

    #[doc(alias = "get_data")]
    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0.data, self.0.size) }
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
unsafe impl MetaAPI for VideoHDRMeta {
    type GstType = ffi::GstVideoHDRMeta;

    #[doc(alias = "gst_video_hdr_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_hdr_meta_api_get_type()) }
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl fmt::Debug for VideoHDRMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoHDRMeta")
            .field("format", &self.format())
            .field("size", &self.0.size)
            .finish()
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[doc(alias = "GstVideoColorVolumeTransformation")]
#[derive(Copy, Clone)]
pub struct VideoColorVolumeTransformation(ffi::GstVideoColorVolumeTransformation);

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl VideoColorVolumeTransformation {
    pub fn window_upper_left_corner_x(&self) -> u16 {
        self.0.window_upper_left_corner_x
    }

    pub fn window_upper_left_corner_y(&self) -> u16 {
        self.0.window_upper_left_corner_y
    }

    pub fn window_lower_right_corner_x(&self) -> u16 {
        self.0.window_lower_right_corner_x
    }

    pub fn window_lower_right_corner_y(&self) -> u16 {
        self.0.window_lower_right_corner_y
    }

    pub fn center_of_ellipse_x(&self) -> u16 {
        self.0.center_of_ellipse_x
    }

    pub fn center_of_ellipse_y(&self) -> u16 {
        self.0.center_of_ellipse_y
    }

    pub fn rotation_angle(&self) -> u8 {
        self.0.rotation_angle
    }

    pub fn semimajor_axis_internal_ellipse(&self) -> u16 {
        self.0.semimajor_axis_internal_ellipse
    }

    pub fn semimajor_axis_external_ellipse(&self) -> u16 {
        self.0.semimajor_axis_external_ellipse
    }

    pub fn semiminor_axis_external_ellipse(&self) -> u16 {
        self.0.semiminor_axis_external_ellipse
    }

    pub fn overlap_process_option(&self) -> u8 {
        self.0.overlap_process_option
    }

    pub fn maxscl(&self) -> &[u32; 3] {
        &self.0.maxscl
    }

    pub fn average_maxrgb(&self) -> u32 {
        self.0.average_maxrgb
    }

    pub fn num_distributions(&self) -> u8 {
        self.0.num_distributions
    }

    pub fn distribution_index(&self) -> &[u8; 16] {
        &self.0.distribution_index
    }

    pub fn distribution_values(&self) -> &[u32; 16] {
        &self.0.distribution_values
    }

    pub fn fraction_bright_pixels(&self) -> u16 {
        self.0.fraction_bright_pixels
    }

    pub fn tone_mapping_flag(&self) -> u8 {
        self.0.tone_mapping_flag
    }

    pub fn knee_point_x(&self) -> u16 {
        self.0.knee_point_x
    }

    pub fn knee_point_y(&self) -> u16 {
        self.0.knee_point_y
    }

    pub fn num_bezier_curve_anchors(&self) -> u8 {
        self.0.num_bezier_curve_anchors
    }

    pub fn bezier_curve_anchors(&self) -> &[u16; 16] {
        &self.0.bezier_curve_anchors
    }

    pub fn color_saturation_mapping_flag(&self) -> u8 {
        self.0.color_saturation_mapping_flag
    }

    pub fn color_saturation_weight(&self) -> u8 {
        self.0.color_saturation_weight
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl fmt::Debug for VideoColorVolumeTransformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoColorVolumeTransformation")
            .field(
                "window_upper_left_corner_x",
                &self.window_upper_left_corner_x(),
            )
            .field(
                "window_upper_left_corner_y",
                &self.window_upper_left_corner_y(),
            )
            .field(
                "window_lower_right_corner_x",
                &self.window_lower_right_corner_x(),
            )
            .field(
                "window_lower_right_corner_y",
                &self.window_lower_right_corner_y(),
            )
            .field("center_of_ellipse_x", &self.center_of_ellipse_x())
            .field("center_of_ellipse_y", &self.center_of_ellipse_y())
            .field("rotation_angle", &self.rotation_angle())
            .field(
                "semimajor_axis_internal_ellipse",
                &self.semimajor_axis_internal_ellipse(),
            )
            .field(
                "semimajor_axis_external_ellipse",
                &self.semimajor_axis_external_ellipse(),
            )
            .field(
                "semiminor_axis_external_ellipse",
                &self.semiminor_axis_external_ellipse(),
            )
            .field("overlap_process_option", &self.overlap_process_option())
            .field("maxscl", self.maxscl())
            .field("average_maxrgb", &self.average_maxrgb())
            .field("num_distributions", &self.num_distributions())
            .field("distribution_index", self.distribution_index())
            .field("distribution_values", self.distribution_values())
            .field("fraction_bright_pixels", &self.fraction_bright_pixels())
            .field("tone_mapping_flag", &self.tone_mapping_flag())
            .field("knee_point_x", &self.knee_point_x())
            .field("knee_point_y", &self.knee_point_y())
            .field("num_bezier_curve_anchors", &self.num_bezier_curve_anchors())
            .field("bezier_curve_anchors", self.bezier_curve_anchors())
            .field(
                "color_saturation_mapping_flag",
                &self.color_saturation_mapping_flag(),
            )
            .field("color_saturation_weight", &self.color_saturation_weight())
            .finish()
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl PartialEq for VideoColorVolumeTransformation {
    fn eq(&self, other: &Self) -> bool {
        self.0.window_upper_left_corner_x == other.0.window_upper_left_corner_x
            && self.0.window_upper_left_corner_y == other.0.window_upper_left_corner_y
            && self.0.window_lower_right_corner_x == other.0.window_lower_right_corner_x
            && self.0.window_lower_right_corner_y == other.0.window_lower_right_corner_y
            && self.0.center_of_ellipse_x == other.0.center_of_ellipse_x
            && self.0.center_of_ellipse_y == other.0.center_of_ellipse_y
            && self.0.rotation_angle == other.0.rotation_angle
            && self.0.semimajor_axis_internal_ellipse == other.0.semimajor_axis_internal_ellipse
            && self.0.semimajor_axis_external_ellipse == other.0.semimajor_axis_external_ellipse
            && self.0.semiminor_axis_external_ellipse == other.0.semiminor_axis_external_ellipse
            && self.0.overlap_process_option == other.0.overlap_process_option
            && self.0.maxscl == other.0.maxscl
            && self.0.average_maxrgb == other.0.average_maxrgb
            && self.0.num_distributions == other.0.num_distributions
            && self.0.distribution_index == other.0.distribution_index
            && self.0.distribution_values == other.0.distribution_values
            && self.0.fraction_bright_pixels == other.0.fraction_bright_pixels
            && self.0.tone_mapping_flag == other.0.tone_mapping_flag
            && self.0.knee_point_x == other.0.knee_point_x
            && self.0.knee_point_y == other.0.knee_point_y
            && self.0.num_bezier_curve_anchors == other.0.num_bezier_curve_anchors
            && self.0.bezier_curve_anchors == other.0.bezier_curve_anchors
            && self.0.color_saturation_mapping_flag == other.0.color_saturation_mapping_flag
            && self.0.color_saturation_weight == other.0.color_saturation_weight
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl Eq for VideoColorVolumeTransformation {}
