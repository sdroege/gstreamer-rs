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
        let res: bool = from_glib(ffi::gst_tag_parse_extended_comment(
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
