// Take a look at the license at the top of the repository in the LICENSE file.

pub trait HasStreamLock {
    #[doc(alias = "get_stream_lock")]
    fn stream_lock(&self) -> *mut glib::ffi::GRecMutex;
    #[doc(alias = "get_element_as_ptr")]
    fn element_as_ptr(&self) -> *const gst::ffi::GstElement;
}
