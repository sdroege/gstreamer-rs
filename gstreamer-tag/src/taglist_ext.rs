// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use crate::ffi;
use glib::translate::*;

// This uses fixed enum values to avoid gtk-rs's checker
// attempting to manually insert a wrong doc alias.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ExifEndian {
    #[doc(hidden)]
    Unknown,
    #[doc(alias = "glib::ffi::G_BIG_ENDIAN")]
    BigEndian = glib::ffi::G_BIG_ENDIAN as isize,
    #[doc(alias = "glib::ffi::G_LITTLE_ENDIAN")]
    LittleEndian = glib::ffi::G_LITTLE_ENDIAN as isize,
}

impl IntoGlib for ExifEndian {
    type GlibType = i32;

    fn into_glib(self) -> Self::GlibType {
        match self {
            Self::BigEndian => glib::ffi::G_BIG_ENDIAN,
            Self::LittleEndian => glib::ffi::G_LITTLE_ENDIAN,
            _ => 0,
        }
    }
}

impl FromGlib<i32> for ExifEndian {
    unsafe fn from_glib(value: ffi::GstTagImageType) -> Self {
        skip_assert_initialized!();

        match value {
            glib::ffi::G_BIG_ENDIAN => Self::BigEndian,
            glib::ffi::G_LITTLE_ENDIAN => Self::LittleEndian,
            _ => Self::Unknown,
        }
    }
}

pub trait TagListRefMutExt {
    #[doc(alias = "gst_tag_list_add_id3_image")]
    fn add_id3_image(
        &mut self,
        image_data: &[u8],
        id3_picture_type: u32,
    ) -> Result<(), glib::BoolError>;

    #[doc(alias = "gst_vorbis_tag_add")]
    fn add_vorbis_tag(&mut self, tag: &str, value: &str);
}

impl TagListRefMutExt for gst::TagListRef {
    fn add_id3_image(
        &mut self,
        image_data: &[u8],
        id3_picture_type: u32,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_tag_list_add_id3_image(
                    self.as_mut_ptr(),
                    image_data.as_ptr(),
                    image_data.len() as u32,
                    id3_picture_type,
                ),
                "Failed to add IDv3 image"
            )
        }
    }

    fn add_vorbis_tag(&mut self, tag: &str, value: &str) {
        unsafe {
            ffi::gst_vorbis_tag_add(
                self.as_mut_ptr(),
                tag.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

pub trait TagListExt: Sized {
    #[doc(alias = "gst_tag_list_from_exif_buffer")]
    fn from_exif_buffer(
        buffer: &gst::Buffer,
        byte_order: ExifEndian,
        base_offset: u32,
    ) -> Result<Self, glib::BoolError>;

    #[doc(alias = "gst_tag_list_from_exif_buffer_with_tiff_header")]
    fn from_exif_buffer_with_tiff_header(buffer: &gst::Buffer) -> Result<Self, glib::BoolError>;

    #[doc(alias = "gst_tag_list_new_from_id3v1")]
    fn from_id3v1(data: &[u8; 128]) -> Result<Self, glib::BoolError>;

    #[doc(alias = "gst_tag_list_from_id3v2_tag")]
    fn from_id3v2_tag(buffer: &gst::Buffer) -> Result<Self, glib::BoolError>;

    #[doc(alias = "gst_tag_list_from_vorbiscomment")]
    fn from_vorbiscomment(
        data: &[u8],
        id_data: &[u8],
    ) -> Result<(Self, Option<glib::GString>), glib::BoolError>;

    #[doc(alias = "gst_tag_list_from_vorbiscomment_buffer")]
    fn from_vorbiscomment_buffer(
        buffer: &gst::Buffer,
        id_data: &[u8],
    ) -> Result<(Self, Option<glib::GString>), glib::BoolError>;

    #[doc(alias = "gst_tag_list_from_xmp_buffer")]
    fn from_xmp_buffer(buffer: &gst::Buffer) -> Result<Self, glib::BoolError>;

    #[doc(alias = "gst_tag_list_to_exif_buffer")]
    fn to_exif_buffer(
        &self,
        byte_order: ExifEndian,
        base_offset: u32,
    ) -> Result<gst::Buffer, glib::BoolError>;

    #[doc(alias = "gst_tag_list_to_exif_buffer_with_tiff_header")]
    fn to_exif_buffer_with_tiff_header(&self) -> Result<gst::Buffer, glib::BoolError>;

    #[doc(alias = "gst_tag_to_vorbis_comments")]
    fn to_vorbis_comments(&self, tag: &str) -> Vec<glib::GString>;

    #[doc(alias = "gst_tag_list_to_vorbiscomment_buffer")]
    fn to_vorbiscomment_buffer(
        &self,
        id_data: &[u8],
        vendor_string: Option<&str>,
    ) -> Result<gst::Buffer, glib::BoolError>;

    #[doc(alias = "gst_tag_list_to_xmp_buffer")]
    fn to_xmp_buffer(
        &self,
        read_only: bool,
        schemas: &[&str],
    ) -> Result<gst::Buffer, glib::BoolError>;
}

impl TagListExt for gst::TagList {
    fn from_exif_buffer(
        buffer: &gst::Buffer,
        byte_order: ExifEndian,
        base_offset: u32,
    ) -> Result<gst::TagList, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_tag_list_from_exif_buffer(
                mut_override(buffer.to_glib_none().0),
                byte_order.into_glib(),
                base_offset,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to parse Exif buffer"))
        }
    }

    fn from_exif_buffer_with_tiff_header(
        buffer: &gst::Buffer,
    ) -> Result<gst::TagList, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_tag_list_from_exif_buffer_with_tiff_header(
                mut_override(buffer.to_glib_none().0),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to parse Exif buffer"))
        }
    }

    fn from_id3v1(data: &[u8; 128]) -> Result<gst::TagList, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_tag_list_new_from_id3v1(data))
                .ok_or_else(|| glib::bool_error!("Failed to parse ID3v1 buffer"))
        }
    }

    fn from_id3v2_tag(buffer: &gst::Buffer) -> Result<gst::TagList, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_tag_list_from_id3v2_tag(buffer.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to parse ID3v2 tag"))
        }
    }

    fn from_vorbiscomment(
        data: &[u8],
        id_data: &[u8],
    ) -> Result<(gst::TagList, Option<glib::GString>), glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut pspec = ptr::null_mut();
            let list = Option::<_>::from_glib_full(ffi::gst_tag_list_from_vorbiscomment(
                data.as_ptr(),
                data.len(),
                id_data.as_ptr(),
                id_data.len() as u32,
                &mut pspec,
            ));
            list.map(|v| (v, Option::<_>::from_glib_full(pspec)))
                .ok_or_else(|| glib::bool_error!("Failed to parse Vorbis comment"))
        }
    }

    fn from_vorbiscomment_buffer(
        buffer: &gst::Buffer,
        id_data: &[u8],
    ) -> Result<(gst::TagList, Option<glib::GString>), glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut pspec = ptr::null_mut();
            let list = Option::<_>::from_glib_full(ffi::gst_tag_list_from_vorbiscomment_buffer(
                buffer.to_glib_none().0,
                id_data.as_ptr(),
                id_data.len() as u32,
                &mut pspec,
            ));
            list.map(|v| (v, Option::<_>::from_glib_full(pspec)))
                .ok_or_else(|| glib::bool_error!("Failed to parse Vorbis comment buffer"))
        }
    }

    fn from_xmp_buffer(buffer: &gst::Buffer) -> Result<gst::TagList, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_tag_list_from_xmp_buffer(buffer.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to parse XMP buffer"))
        }
    }

    fn to_exif_buffer(
        &self,
        byte_order: ExifEndian,
        base_offset: u32,
    ) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_tag_list_to_exif_buffer(
                self.as_ptr(),
                byte_order.into_glib(),
                base_offset,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to serialize into Exif buffer"))
        }
    }

    fn to_exif_buffer_with_tiff_header(&self) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_tag_list_to_exif_buffer_with_tiff_header(
                self.as_ptr(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to serialize into Exif buffer"))
        }
    }

    fn to_vorbis_comments(&self, tag: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_tag_to_vorbis_comments(
                self.as_ptr(),
                tag.to_glib_none().0,
            ))
        }
    }

    fn to_vorbiscomment_buffer(
        &self,
        id_data: &[u8],
        vendor_string: Option<&str>,
    ) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_tag_list_to_vorbiscomment_buffer(
                self.as_ptr(),
                id_data.as_ptr(),
                id_data.len() as u32,
                vendor_string.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to serialize into Vorbis comment buffer"))
        }
    }

    fn to_xmp_buffer(
        &self,
        read_only: bool,
        schemas: &[&str],
    ) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_tag_list_to_xmp_buffer(
                self.as_ptr(),
                read_only.into_glib(),
                schemas.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to serialize into Vorbis comment buffer"))
        }
    }
}
