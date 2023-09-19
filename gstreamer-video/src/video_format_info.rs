// Take a look at the license at the top of the repository in the LICENSE file.

use std::{cmp::Ordering, fmt, marker::PhantomData, str};

use glib::translate::{from_glib, IntoGlib, ToGlibPtr};

#[doc(alias = "GstVideoFormatInfo")]
#[derive(Copy, Clone)]
pub struct VideoFormatInfo(&'static ffi::GstVideoFormatInfo);

impl VideoFormatInfo {
    #[inline]
    pub unsafe fn from_ptr(format_info: *const ffi::GstVideoFormatInfo) -> Self {
        debug_assert!(!format_info.is_null());
        Self(&*format_info)
    }

    #[inline]
    pub fn from_format(format: crate::VideoFormat) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let info = ffi::gst_video_format_get_info(format.into_glib());
            debug_assert!(!info.is_null());

            Self(&*info)
        }
    }

    #[inline]
    pub fn format(&self) -> crate::VideoFormat {
        unsafe { from_glib(self.0.format) }
    }

    #[inline]
    pub fn name<'a>(&self) -> &'a glib::GStr {
        unsafe { glib::GStr::from_ptr(self.0.name) }
    }

    #[inline]
    pub fn description<'a>(&self) -> &'a glib::GStr {
        unsafe { glib::GStr::from_ptr(self.0.description) }
    }

    #[inline]
    pub fn flags(&self) -> crate::VideoFormatFlags {
        unsafe { from_glib(self.0.flags) }
    }

    #[inline]
    pub fn bits(&self) -> u32 {
        self.0.bits
    }

    #[inline]
    pub fn n_components(&self) -> u32 {
        self.0.n_components
    }

    #[inline]
    pub fn shift(&self) -> &[u32] {
        &self.0.shift[0..(self.0.n_components as usize)]
    }

    #[inline]
    pub fn depth(&self) -> &[u32] {
        &self.0.depth[0..(self.0.n_components as usize)]
    }

    #[inline]
    pub fn pixel_stride(&self) -> &[i32] {
        &self.0.pixel_stride[0..(self.0.n_components as usize)]
    }

    #[inline]
    pub fn n_planes(&self) -> u32 {
        self.0.n_planes
    }

    #[inline]
    pub fn plane(&self) -> &[u32] {
        &self.0.plane[0..(self.0.n_components as usize)]
    }

    #[inline]
    pub fn poffset(&self) -> &[u32] {
        &self.0.poffset[0..(self.0.n_components as usize)]
    }

    #[inline]
    pub fn w_sub(&self) -> &[u32] {
        &self.0.w_sub[0..(self.0.n_components as usize)]
    }

    #[inline]
    pub fn h_sub(&self) -> &[u32] {
        &self.0.h_sub[0..(self.0.n_components as usize)]
    }

    #[inline]
    pub fn tile_mode(&self) -> crate::VideoTileMode {
        unsafe { from_glib(self.0.tile_mode) }
    }

    #[cfg_attr(feature = "v1_22", deprecated = "Since 1.22")]
    #[inline]
    pub fn tile_ws(&self) -> u32 {
        self.0.tile_ws
    }

    #[cfg_attr(feature = "v1_22", deprecated = "Since 1.22")]
    #[inline]
    pub fn tile_hs(&self) -> u32 {
        self.0.tile_hs
    }

    #[inline]
    pub fn unpack_format(&self) -> crate::VideoFormat {
        unsafe { from_glib(self.0.unpack_format) }
    }

    #[inline]
    pub fn pack_lines(&self) -> i32 {
        self.0.pack_lines
    }

    #[inline]
    pub fn has_alpha(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_ALPHA != 0
    }

    #[inline]
    pub fn has_palette(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_PALETTE != 0
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[inline]
    pub fn has_subtiles(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_SUBTILES != 0
    }

    #[inline]
    pub fn is_complex(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_COMPLEX != 0
    }

    #[inline]
    pub fn is_gray(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_GRAY != 0
    }

    #[inline]
    pub fn is_le(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_LE != 0
    }

    #[inline]
    pub fn is_rgb(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_RGB != 0
    }

    #[inline]
    pub fn is_tiled(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_TILED != 0
    }

    #[inline]
    pub fn is_yuv(&self) -> bool {
        self.0.flags & ffi::GST_VIDEO_FORMAT_FLAG_YUV != 0
    }

    #[inline]
    pub fn scale_width(&self, component: u8, width: u32) -> u32 {
        (-((-(i64::from(width))) >> self.w_sub()[component as usize])) as u32
    }

    #[inline]
    pub fn scale_height(&self, component: u8, height: u32) -> u32 {
        (-((-(i64::from(height))) >> self.h_sub()[component as usize])) as u32
    }

    #[allow(clippy::too_many_arguments)]
    pub fn unpack(
        &self,
        flags: crate::VideoPackFlags,
        dest: &mut [u8],
        src: &[&[u8]],
        stride: &[i32],
        x: i32,
        y: i32,
        width: i32,
    ) {
        let unpack_format = Self::from_format(self.unpack_format());

        if unpack_format.pixel_stride()[0] == 0 || self.0.unpack_func.is_none() {
            panic!("No unpack format for {self:?}");
        }

        if src.len() != self.n_planes() as usize {
            panic!(
                "Wrong number of planes provided for format: {} != {}",
                src.len(),
                self.n_planes()
            );
        }

        if stride.len() != self.n_planes() as usize {
            panic!(
                "Wrong number of strides provided for format: {} != {}",
                stride.len(),
                self.n_planes()
            );
        }

        if dest.len() < unpack_format.pixel_stride()[0] as usize * width as usize {
            panic!("Too small destination slice");
        }

        for plane in 0..(self.n_planes()) {
            if stride[plane as usize]
                < self.scale_width(plane as u8, width as u32) as i32
                    * self.pixel_stride()[plane as usize]
            {
                panic!("Too small source stride for plane {plane}");
            }

            let plane_size = y * stride[plane as usize]
                + self.scale_width(plane as u8, (x + width) as u32) as i32
                    * self.pixel_stride()[plane as usize];

            if src[plane as usize].len() < plane_size as usize {
                panic!("Too small source plane size for plane {plane}");
            }
        }

        unsafe {
            use std::ptr;

            let mut src_ptr = [ptr::null(); ffi::GST_VIDEO_MAX_PLANES as usize];
            for plane in 0..(self.n_planes()) {
                src_ptr[plane as usize] = src[plane as usize].as_ptr();
            }

            (self.0.unpack_func.as_ref().unwrap())(
                self.0,
                flags.into_glib(),
                dest.as_mut_ptr() as *mut _,
                src_ptr.as_ptr() as *const _,
                stride.as_ptr(),
                x,
                y,
                width,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pack(
        &self,
        flags: crate::VideoPackFlags,
        src: &[u8],
        src_stride: i32,
        dest: &mut [&mut [u8]],
        dest_stride: &[i32],
        chroma_site: crate::VideoChromaSite,
        y: i32,
        width: i32,
    ) {
        let unpack_format = Self::from_format(self.unpack_format());

        if unpack_format.pixel_stride()[0] == 0 || self.0.unpack_func.is_none() {
            panic!("No unpack format for {self:?}");
        }

        if dest.len() != self.n_planes() as usize {
            panic!(
                "Wrong number of planes provided for format: {} != {}",
                dest.len(),
                self.n_planes()
            );
        }

        if dest_stride.len() != self.n_planes() as usize {
            panic!(
                "Wrong number of strides provided for format: {} != {}",
                dest_stride.len(),
                self.n_planes()
            );
        }

        if src.len() < unpack_format.pixel_stride()[0] as usize * width as usize {
            panic!("Too small source slice");
        }

        for plane in 0..(self.n_planes()) {
            if dest_stride[plane as usize]
                < self.scale_width(plane as u8, width as u32) as i32
                    * self.pixel_stride()[plane as usize]
            {
                panic!("Too small destination stride for plane {plane}");
            }

            let plane_size = y * dest_stride[plane as usize]
                + self.scale_width(plane as u8, width as u32) as i32
                    * self.pixel_stride()[plane as usize];

            if dest[plane as usize].len() < plane_size as usize {
                panic!("Too small destination plane size for plane {plane}");
            }
        }

        unsafe {
            use std::ptr;

            let mut dest_ptr = [ptr::null_mut(); ffi::GST_VIDEO_MAX_PLANES as usize];
            for plane in 0..(self.n_planes()) {
                dest_ptr[plane as usize] = dest[plane as usize].as_mut_ptr();
            }

            (self.0.pack_func.as_ref().unwrap())(
                self.0,
                flags.into_glib(),
                src.as_ptr() as *mut _,
                src_stride,
                dest_ptr.as_mut_ptr() as *mut _,
                dest_stride.as_ptr(),
                chroma_site.into_glib(),
                y,
                width,
            );
        }
    }

    #[doc(alias = "gst_video_color_range_offsets")]
    pub fn range_offsets(&self, range: crate::VideoColorRange) -> ([i32; 4], [i32; 4]) {
        let mut offset = [0i32; 4];
        let mut scale = [0i32; 4];
        unsafe {
            ffi::gst_video_color_range_offsets(
                range.into_glib(),
                self.to_glib_none().0,
                &mut offset,
                &mut scale,
            )
        }
        (offset, scale)
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_video_format_info_extrapolate_stride")]
    pub fn extrapolate_stride(&self, plane: u32, stride: u32) -> u32 {
        assert!(plane < self.n_planes());

        unsafe {
            ffi::gst_video_format_info_extrapolate_stride(
                self.to_glib_none().0,
                plane as i32,
                stride as i32,
            ) as u32
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    pub fn tile_info(&self, plane: u32) -> &VideoTileInfo {
        assert!(plane < self.n_planes());

        unsafe { &*(&self.0.tile_info[plane as usize] as *const _ as *const VideoTileInfo) }
    }
}

unsafe impl Sync for VideoFormatInfo {}
unsafe impl Send for VideoFormatInfo {}

impl PartialEq for VideoFormatInfo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}

impl Eq for VideoFormatInfo {}

impl PartialOrd for VideoFormatInfo {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VideoFormatInfo {
    // See GST_VIDEO_FORMATS_ALL for the sorting algorithm
    fn cmp(&self, other: &Self) -> Ordering {
        self.n_components()
            .cmp(&other.n_components())
            .reverse()
            .then_with(|| self.depth().cmp(other.depth()).reverse())
            .then_with(|| self.w_sub().cmp(other.w_sub()))
            .then_with(|| self.h_sub().cmp(other.h_sub()))
            .then_with(|| self.n_planes().cmp(&other.n_planes()).reverse())
            .then_with(|| {
                // Format using native endianness is considered smaller
                let native_endianness = [crate::VideoFormat::Ayuv64, crate::VideoFormat::Argb64];
                let want_le = cfg!(target_endian = "little");

                match (
                    self.flags().contains(crate::VideoFormatFlags::LE) == want_le
                        || native_endianness.contains(&self.format()),
                    other.flags().contains(crate::VideoFormatFlags::LE) == want_le
                        || native_endianness.contains(&other.format()),
                ) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            })
            .then_with(|| {
                // Prefer non-complex formats
                match (
                    self.flags().contains(crate::VideoFormatFlags::COMPLEX),
                    other.flags().contains(crate::VideoFormatFlags::COMPLEX),
                ) {
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    _ => Ordering::Equal,
                }
            })
            .then_with(|| {
                // Prefer RGB over YUV
                if self.flags().contains(crate::VideoFormatFlags::RGB)
                    && other.flags().contains(crate::VideoFormatFlags::YUV)
                {
                    Ordering::Greater
                } else if self.flags().contains(crate::VideoFormatFlags::YUV)
                    && other.flags().contains(crate::VideoFormatFlags::RGB)
                {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .then_with(|| {
                // Prefer xRGB and permutations over RGB and permutations
                let xrgb = [
                    crate::VideoFormat::Xrgb,
                    crate::VideoFormat::Xbgr,
                    crate::VideoFormat::Rgbx,
                    crate::VideoFormat::Bgrx,
                ];
                let rgb = [crate::VideoFormat::Rgb, crate::VideoFormat::Bgr];

                if xrgb.contains(&self.format()) && rgb.contains(&other.format()) {
                    Ordering::Less
                } else if rgb.contains(&self.format()) && xrgb.contains(&other.format()) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .then_with(|| self.pixel_stride().cmp(other.pixel_stride()))
            .then_with(|| self.poffset().cmp(other.poffset()))
            .then_with(|| {
                // tie, sort by name
                self.name().cmp(other.name())
            })
            // and reverse the whole ordering so that "better quality" > "lower quality"
            .reverse()
    }
}

impl fmt::Debug for VideoFormatInfo {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fmt = f.debug_struct("VideoFormatInfo");

        fmt.field("format", &self.format())
            .field("name", &self.name())
            .field("description", &self.description())
            .field("flags", &self.flags())
            .field("bits", &self.bits())
            .field("n-components", &self.n_components())
            .field("shift", &self.shift())
            .field("depth", &self.depth())
            .field("pixel-stride", &self.pixel_stride())
            .field("n-planes", &self.n_planes())
            .field("plane", &self.plane())
            .field("poffset", &self.poffset())
            .field("w-sub", &self.w_sub())
            .field("h-sub", &self.h_sub())
            .field("unpack-format", &self.unpack_format())
            .field("pack-lines", &self.pack_lines())
            .field("tile-mode", &self.tile_mode())
            .field("tile-ws", &self.tile_ws())
            .field("tile-hs", &self.tile_hs());

        #[cfg(feature = "v1_22")]
        {
            fmt.field(
                "tile-info",
                &(0..self.n_planes()).map(|plane| self.tile_info(plane)),
            );
        }

        fmt.finish()
    }
}

impl fmt::Display for VideoFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl str::FromStr for crate::VideoFormatInfo {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        let format = s.parse()?;
        Ok(Self::from_format(format))
    }
}

impl From<crate::VideoFormat> for VideoFormatInfo {
    #[inline]
    fn from(f: crate::VideoFormat) -> Self {
        skip_assert_initialized!();
        Self::from_format(f)
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for VideoFormatInfo {
    type GlibType = *mut ffi::GstVideoFormatInfo;
}

#[doc(hidden)]
unsafe impl glib::translate::TransparentPtrType for VideoFormatInfo {}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstVideoFormatInfo> for VideoFormatInfo {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstVideoFormatInfo, Self> {
        glib::translate::Stash(self.0, PhantomData)
    }

    fn to_glib_full(&self) -> *const ffi::GstVideoFormatInfo {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstVideoFormatInfo> for VideoFormatInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstVideoFormatInfo) -> Self {
        Self(&*ptr)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstVideoFormatInfo> for VideoFormatInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstVideoFormatInfo) -> Self {
        Self(&*ptr)
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
#[repr(transparent)]
#[doc(alias = "GstVideoTileInfo")]
pub struct VideoTileInfo(ffi::GstVideoTileInfo);

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl fmt::Debug for VideoTileInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoTileInfo")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("stride", &self.stride())
            .field("size", &self.size())
            .finish()
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl VideoTileInfo {
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.0.height
    }

    #[inline]
    pub fn stride(&self) -> u32 {
        self.0.stride
    }

    #[inline]
    pub fn size(&self) -> u32 {
        self.0.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        gst::init().unwrap();

        let info = VideoFormatInfo::from_format(crate::VideoFormat::I420);
        assert_eq!(info.name(), "I420");

        let other_info = "I420".parse().unwrap();
        assert_eq!(info, other_info);

        assert_eq!(info.scale_width(0, 128), 128);
        assert_eq!(info.scale_width(1, 128), 64);
        assert_eq!(info.scale_width(2, 128), 64);
    }

    #[test]
    fn test_unpack() {
        gst::init().unwrap();

        // One line black 320 pixel I420
        let input = &[&[0; 320][..], &[128; 160][..], &[128; 160][..]];
        // One line of AYUV
        let intermediate = &mut [0; 320 * 4][..];
        // One line of 320 pixel I420
        let output = &mut [&mut [0; 320][..], &mut [0; 160][..], &mut [0; 160][..]];

        let info = VideoFormatInfo::from_format(crate::VideoFormat::I420);
        assert_eq!(info.unpack_format(), crate::VideoFormat::Ayuv);
        info.unpack(
            crate::VideoPackFlags::empty(),
            intermediate,
            input,
            &[320, 160, 160][..],
            0,
            0,
            320,
        );

        for pixel in intermediate.chunks_exact(4) {
            assert_eq!(&[255, 0, 128, 128][..], pixel);
        }

        info.pack(
            crate::VideoPackFlags::empty(),
            &intermediate[..(4 * 320)],
            4 * 320,
            output,
            &[320, 160, 160][..],
            crate::VideoChromaSite::NONE,
            0,
            320,
        );
        assert_eq!(input, output);
    }
}
