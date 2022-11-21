// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;
use std::mem;

#[doc(alias = "gst_type_find_helper_for_data")]
pub fn type_find_helper_for_data(
    obj: Option<&impl IsA<gst::Object>>,
    data: impl AsRef<[u8]>,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let data = data.as_ref();
        let (ptr, len) = (data.as_ptr(), data.len());
        let ret = ffi::gst_type_find_helper_for_data(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(ptr),
            len,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[doc(alias = "gst_type_find_helper_for_data_with_extension")]
pub fn type_find_helper_for_data_with_extension(
    obj: Option<&impl IsA<gst::Object>>,
    data: impl AsRef<[u8]>,
    extension: Option<&str>,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let data = data.as_ref();
        let (ptr, len) = (data.as_ptr(), data.len());
        let ret = ffi::gst_type_find_helper_for_data_with_extension(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(ptr),
            len,
            extension.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[doc(alias = "gst_type_find_helper_for_buffer")]
pub fn type_find_helper_for_buffer<P: IsA<gst::Object>>(
    obj: Option<&P>,
    buf: &gst::Buffer,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let ret = ffi::gst_type_find_helper_for_buffer(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            buf.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[doc(alias = "gst_type_find_helper_for_buffer_with_extension")]
pub fn type_find_helper_for_buffer_with_extension<P: IsA<gst::Object>>(
    obj: Option<&P>,
    buf: &gst::Buffer,
    extension: Option<&str>,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let ret = ffi::gst_type_find_helper_for_buffer_with_extension(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            buf.to_glib_none().0,
            extension.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
#[doc(alias = "gst_type_find_helper_for_buffer_with_caps")]
pub fn type_find_helper_for_buffer_with_caps(
    obj: Option<&impl IsA<gst::Object>>,
    buf: &gst::BufferRef,
    caps: &gst::CapsRef,
) -> (Option<gst::Caps>, gst::TypeFindProbability) {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let ret = from_glib_full(ffi::gst_type_find_helper_for_buffer_with_caps(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(buf.as_ptr()),
            mut_override(caps.as_ptr()),
            prob.as_mut_ptr(),
        ));
        (ret, from_glib(prob.assume_init()))
    }
}

#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
#[doc(alias = "gst_type_find_helper_for_data_with_caps")]
pub fn type_find_helper_for_data_with_caps(
    obj: Option<&impl IsA<gst::Object>>,
    data: &[u8],
    caps: &gst::CapsRef,
) -> (Option<gst::Caps>, gst::TypeFindProbability) {
    assert_initialized_main_thread!();
    let size = data.len() as _;
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let ret = from_glib_full(ffi::gst_type_find_helper_for_data_with_caps(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            data.to_glib_none().0,
            size,
            mut_override(caps.as_ptr()),
            prob.as_mut_ptr(),
        ));
        (ret, from_glib(prob.assume_init()))
    }
}

#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
#[doc(alias = "gst_type_find_list_factories_for_caps")]
pub fn type_find_list_factories_for_caps(
    obj: Option<&impl IsA<gst::Object>>,
    caps: &gst::CapsRef,
) -> glib::List<gst::TypeFindFactory> {
    assert_initialized_main_thread!();
    unsafe {
        glib::collections::List::from_glib_full(ffi::gst_type_find_list_factories_for_caps(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(caps.as_ptr()),
        ))
    }
}
