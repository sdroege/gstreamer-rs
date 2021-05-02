// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::HasStreamLock;
use glib::translate::*;
use std::fmt;
use std::marker::PhantomData;
use std::ptr;

use crate::video_info::VideoInfo;

pub trait VideoCodecStateContext<'a> {
    #[doc(alias = "get_element")]
    fn element(&self) -> Option<&'a dyn HasStreamLock>;
    #[doc(alias = "get_element_as_ptr")]
    fn element_as_ptr(&self) -> *const gst::ffi::GstElement;
}

pub struct InNegotiation<'a> {
    /* GstVideoCodecState API isn't safe so protect the state using the
     * element (decoder or encoder) stream lock */
    element: &'a dyn HasStreamLock,
}
pub struct Readable {}

impl<'a> VideoCodecStateContext<'a> for InNegotiation<'a> {
    fn element(&self) -> Option<&'a dyn HasStreamLock> {
        Some(self.element)
    }

    fn element_as_ptr(&self) -> *const gst::ffi::GstElement {
        self.element.element_as_ptr()
    }
}

impl<'a> VideoCodecStateContext<'a> for Readable {
    fn element(&self) -> Option<&'a dyn HasStreamLock> {
        None
    }

    fn element_as_ptr(&self) -> *const gst::ffi::GstElement {
        ptr::null()
    }
}

pub struct VideoCodecState<'a, T: VideoCodecStateContext<'a>> {
    state: *mut ffi::GstVideoCodecState,
    pub(crate) context: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: VideoCodecStateContext<'a>> fmt::Debug for VideoCodecState<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoCodecState")
            .field("info", &self.info())
            .field("caps", &self.caps())
            .field("codec_data", &self.codec_data())
            .field("allocation_caps", &self.allocation_caps())
            .finish()
    }
}

impl<'a> VideoCodecState<'a, Readable> {
    // Take ownership of @state
    pub(crate) unsafe fn new(state: *mut ffi::GstVideoCodecState) -> Self {
        skip_assert_initialized!();
        Self {
            state,
            context: Readable {},
            phantom: PhantomData,
        }
    }
}

impl<'a> VideoCodecState<'a, InNegotiation<'a>> {
    // Take ownership of @state
    pub(crate) unsafe fn new<T: HasStreamLock>(
        state: *mut ffi::GstVideoCodecState,
        element: &'a T,
    ) -> Self {
        skip_assert_initialized!();
        let stream_lock = element.stream_lock();
        glib::ffi::g_rec_mutex_lock(stream_lock);
        Self {
            state,
            context: InNegotiation { element },
            phantom: PhantomData,
        }
    }
}

impl<'a, T: VideoCodecStateContext<'a>> VideoCodecState<'a, T> {
    #[doc(alias = "get_info")]
    pub fn info(&self) -> VideoInfo {
        unsafe {
            let ptr = &((*self.as_mut_ptr()).info) as *const _ as usize as *mut _;
            VideoInfo::from_glib_none(ptr)
        }
    }

    #[doc(alias = "get_caps")]
    pub fn caps(&self) -> Option<&gst::CapsRef> {
        unsafe {
            let ptr = (*self.as_mut_ptr()).caps;

            if ptr.is_null() {
                None
            } else {
                Some(gst::CapsRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_codec_data")]
    pub fn codec_data(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.as_mut_ptr()).codec_data;

            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_allocation_caps")]
    pub fn allocation_caps(&self) -> Option<&gst::CapsRef> {
        unsafe {
            let ptr = (*self.as_mut_ptr()).allocation_caps;

            if ptr.is_null() {
                None
            } else {
                Some(gst::CapsRef::from_ptr(ptr))
            }
        }
    }
    #[doc(hidden)]
    pub fn as_mut_ptr(&self) -> *mut ffi::GstVideoCodecState {
        self.state
    }
}

impl<'a, T: VideoCodecStateContext<'a>> Drop for VideoCodecState<'a, T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(element) = self.context.element() {
                let stream_lock = element.stream_lock();
                glib::ffi::g_rec_mutex_unlock(stream_lock);
            }
            ffi::gst_video_codec_state_unref(self.state);
        }
    }
}

impl<'a> VideoCodecState<'a, InNegotiation<'a>> {
    pub fn set_info(&mut self, info: VideoInfo) {
        unsafe {
            ptr::write(&mut (*self.as_mut_ptr()).info, *(info.to_glib_none().0));
        }
    }

    pub fn set_caps(&mut self, caps: &gst::Caps) {
        unsafe {
            let prev = (*self.as_mut_ptr()).caps;

            if !prev.is_null() {
                gst::ffi::gst_mini_object_unref(prev as *mut gst::ffi::GstMiniObject)
            }

            ptr::write(
                &mut (*self.as_mut_ptr()).caps,
                gst::ffi::gst_mini_object_ref(caps.as_mut_ptr() as *mut _) as *mut _,
            );
        }
    }

    pub fn set_codec_data(&mut self, codec_data: &gst::Buffer) {
        unsafe {
            let prev = (*self.as_mut_ptr()).codec_data;

            if !prev.is_null() {
                gst::ffi::gst_mini_object_unref(prev as *mut gst::ffi::GstMiniObject)
            }

            ptr::write(
                &mut (*self.as_mut_ptr()).codec_data,
                gst::ffi::gst_mini_object_ref(codec_data.as_mut_ptr() as *mut _) as *mut _,
            );
        }
    }

    pub fn set_allocation_caps(&mut self, allocation_caps: &gst::Caps) {
        unsafe {
            let prev = (*self.as_mut_ptr()).allocation_caps;

            if !prev.is_null() {
                gst::ffi::gst_mini_object_unref(prev as *mut gst::ffi::GstMiniObject)
            }

            ptr::write(
                &mut (*self.as_mut_ptr()).allocation_caps,
                gst::ffi::gst_mini_object_ref(allocation_caps.as_mut_ptr() as *mut _) as *mut _,
            );
        }
    }
}

impl<'a> Clone for VideoCodecState<'a, Readable> {
    fn clone(&self) -> Self {
        unsafe {
            let state = ffi::gst_video_codec_state_ref(self.state);
            Self::new(state)
        }
    }
}

unsafe impl<'a> Send for VideoCodecState<'a, Readable> {}
unsafe impl<'a> Sync for VideoCodecState<'a, Readable> {}
