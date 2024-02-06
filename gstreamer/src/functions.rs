// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::translate::*;

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use crate::Tracer;

// import only functions which do not have their own module as namespace
pub use crate::auto::functions::{
    main_executable_path, util_get_timestamp as get_timestamp, version, version_string,
};

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
pub use crate::auto::functions::util_ceil_log2 as ceil_log2;

#[doc(alias = "gst_calculate_linear_regression")]
pub fn calculate_linear_regression(
    xy: &[(u64, u64)],
    temp: Option<&mut [(u64, u64)]>,
) -> Option<(u64, u64, u64, u64, f64)> {
    skip_assert_initialized!();
    use std::mem;

    unsafe {
        assert_eq!(mem::size_of::<u64>() * 2, mem::size_of::<(u64, u64)>());
        assert_eq!(mem::align_of::<u64>(), mem::align_of::<(u64, u64)>());
        assert!(
            temp.as_ref()
                .map(|temp| temp.len())
                .unwrap_or_else(|| xy.len())
                >= xy.len()
        );

        let mut m_num = mem::MaybeUninit::uninit();
        let mut m_denom = mem::MaybeUninit::uninit();
        let mut b = mem::MaybeUninit::uninit();
        let mut xbase = mem::MaybeUninit::uninit();
        let mut r_squared = mem::MaybeUninit::uninit();

        let res = from_glib(ffi::gst_calculate_linear_regression(
            xy.as_ptr() as *const u64,
            temp.map(|temp| temp.as_mut_ptr() as *mut u64)
                .unwrap_or(ptr::null_mut()),
            xy.len() as u32,
            m_num.as_mut_ptr(),
            m_denom.as_mut_ptr(),
            b.as_mut_ptr(),
            xbase.as_mut_ptr(),
            r_squared.as_mut_ptr(),
        ));
        if res {
            Some((
                m_num.assume_init(),
                m_denom.assume_init(),
                b.assume_init(),
                xbase.assume_init(),
                r_squared.assume_init(),
            ))
        } else {
            None
        }
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
#[doc(alias = "gst_tracing_get_active_tracers")]
pub fn active_tracers() -> glib::List<Tracer> {
    assert_initialized_main_thread!();
    unsafe { FromGlibPtrContainer::from_glib_full(ffi::gst_tracing_get_active_tracers()) }
}

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
#[doc(alias = "gst_util_filename_compare")]
pub fn filename_compare(a: &std::path::Path, b: &std::path::Path) -> std::cmp::Ordering {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gst_util_filename_compare(
            a.to_glib_none().0,
            b.to_glib_none().0,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_linear_regression() {
        crate::init().unwrap();

        let values = [(0, 0), (1, 1), (2, 2), (3, 3)];

        let (m_num, m_denom, b, xbase, _) = calculate_linear_regression(&values, None).unwrap();
        assert_eq!((m_num, m_denom, b, xbase), (10, 10, 3, 3));

        let mut temp = [(0, 0); 4];
        let (m_num, m_denom, b, xbase, _) =
            calculate_linear_regression(&values, Some(&mut temp)).unwrap();
        assert_eq!((m_num, m_denom, b, xbase), (10, 10, 3, 3));
    }
}
