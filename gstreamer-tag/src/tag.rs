// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use glib::translate::*;

pub use crate::functions::{
    tag_from_id3_tag as from_id3_tag, tag_from_id3_user_tag as from_id3_user_tag,
    tag_from_vorbis_tag as from_vorbis_tag, tag_id3_genre_count as id3_genre_count,
    tag_id3_genre_get as id3_genre, tag_parse_extended_comment as parse_extended_comment,
    tag_to_id3_tag as to_id3_tag, tag_to_vorbis_tag as to_vorbis_tag,
    tag_xmp_list_schemas as xmp_list_schemas,
};

#[doc(alias = "gst_tag_freeform_string_to_utf8")]
pub fn freeform_string_to_utf8(data: &[u8], env_vars: &[&str]) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gst_tag_freeform_string_to_utf8(
            data.as_ptr() as *const i8,
            data.len() as i32,
            env_vars.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_tag_get_id3v2_tag_size")]
pub fn id3v2_tag_size(buffer: &gst::Buffer) -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_tag_get_id3v2_tag_size(buffer.to_glib_none().0) }
}
