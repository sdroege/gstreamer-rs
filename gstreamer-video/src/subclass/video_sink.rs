// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use gst_base::subclass::prelude::*;

use crate::VideoSink;

pub trait VideoSinkImpl: VideoSinkImplExt + BaseSinkImpl + ElementImpl {
    fn show_frame(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_show_frame(buffer)
    }
}

pub trait VideoSinkImplExt: ObjectSubclass {
    fn parent_show_frame(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<T: VideoSinkImpl> VideoSinkImplExt for T {
    fn parent_show_frame(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoSinkClass;
            (*parent_class)
                .show_frame
                .map(|f| {
                    try_from_glib(f(
                        self.obj().unsafe_cast_ref::<VideoSink>().to_glib_none().0,
                        buffer.to_glib_none().0,
                    ))
                })
                .unwrap_or(Err(gst::FlowError::Error))
        }
    }
}

unsafe impl<T: VideoSinkImpl> IsSubclassable<T> for VideoSink {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.show_frame = Some(video_sink_show_frame::<T>);
    }
}

unsafe extern "C" fn video_sink_show_frame<T: VideoSinkImpl>(
    ptr: *mut ffi::GstVideoSink,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let buffer = from_glib_borrow(buffer);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.show_frame(&buffer).into()
    })
    .into_glib()
}
