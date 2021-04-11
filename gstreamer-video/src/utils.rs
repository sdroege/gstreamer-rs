// Take a look at the license at the top of the repository in the LICENSE file.

pub trait HasStreamLock {
    fn stream_lock(&self) -> *mut glib::ffi::GRecMutex;
    fn element_as_ptr(&self) -> *const gst::ffi::GstElement;
}
