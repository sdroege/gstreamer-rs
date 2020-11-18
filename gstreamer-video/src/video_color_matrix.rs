use std::mem;

use glib::translate::ToGlib;

impl crate::VideoColorMatrix {
    pub fn get_kr_kb(&self) -> Result<(f64, f64), glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut kr = mem::MaybeUninit::uninit();
            let mut kb = mem::MaybeUninit::uninit();
            glib::glib_result_from_gboolean!(
                ffi::gst_video_color_matrix_get_Kr_Kb(
                    self.to_glib(),
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
