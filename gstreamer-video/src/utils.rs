// Take a look at the license at the top of the repository in the LICENSE file.

pub trait HasStreamLock {
    fn get_stream_lock(&self) -> *mut glib::ffi::GRecMutex;
    fn get_element_as_ptr(&self) -> *const gst::ffi::GstElement;
}
