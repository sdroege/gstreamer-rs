// Take a look at the license at the top of the repository in the LICENSE file.

type GstMainFuncSimple = Option<unsafe extern "C" fn(glib::ffi::gpointer)>;

#[link(name = "gstreamer-1.0")]
extern "C" {
    #[cfg(feature = "v1_22")]
    fn gst_macos_main_simple(func: GstMainFuncSimple, user_data: glib::ffi::gpointer);
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
#[doc(alias = "gst_macos_main")]
pub fn macos_main<T, F>(func: F) -> T
where
    F: FnOnce() -> T + Send,
{
    skip_assert_initialized!();
    unsafe extern "C" fn trampoline<T, F: FnOnce() -> T + Send>(user_data: glib::ffi::gpointer) {
        let data = &mut *(user_data as *mut (Option<F>, Option<T>));
        let func = data.0.take().unwrap();
        let res = func();

        data.1 = Some(res);
    }

    let mut func: (Option<F>, Option<T>) = (Some(func), None);

    unsafe {
        gst_macos_main_simple(
            Some(trampoline::<T, F>),
            &mut func as *mut (Option<F>, Option<T>) as *mut _,
        );
    }

    func.1.unwrap()
}
