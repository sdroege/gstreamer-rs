// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[doc(alias = "gst_validate_init")]
pub fn init() {
    unsafe {
        ffi::gst_validate_init();
        crate::INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

#[doc(alias = "gst_validate_init_debug")]
pub fn init_debug() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_validate_init_debug();
    }
}

#[doc(alias = "gst_validate_setup_test_file")]
pub fn setup_test_file(test_file: &str, use_fakesinks: bool) -> gst::Structure {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_validate_setup_test_file(
            test_file.to_glib_none().0,
            use_fakesinks as i32,
        ))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_init() {
        gst::init().unwrap();
        crate::init();
    }
}
