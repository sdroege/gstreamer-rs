// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::translate::IntoGlib;

impl crate::VideoColorMatrix {
    #[doc(alias = "get_kr_kb")]
    pub fn kr_kb(&self) -> Result<(f64, f64), glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut kr = mem::MaybeUninit::uninit();
            let mut kb = mem::MaybeUninit::uninit();
            glib::result_from_gboolean!(
                ffi::gst_video_color_matrix_get_Kr_Kb(
                    self.into_glib(),
                    kr.as_mut_ptr(),
                    kb.as_mut_ptr(),
                ),
                "{:?} is not a YUV matrix",
                self
            )?;
            let kr = kr.assume_init();
            let kb = kb.assume_init();
            Ok((kr, kb))
        }
    }
}
