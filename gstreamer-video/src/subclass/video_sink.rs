// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;

use gst_base::subclass::prelude::*;

use crate::VideoSink;

pub trait VideoSinkImpl: VideoSinkImplExt + BaseSinkImpl + ElementImpl {
    fn show_frame(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_show_frame(element, buffer)
    }
}

pub trait VideoSinkImplExt: ObjectSubclass {
    fn parent_show_frame(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<T: VideoSinkImpl> VideoSinkImplExt for T {
    fn parent_show_frame(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoSinkClass;
            (*parent_class)
                .show_frame
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<VideoSink>().to_glib_none().0,
                        buffer.to_glib_none().0,
                    ))
                })
                .unwrap_or(gst::FlowReturn::Error)
                .into_result()
        }
    }
}

unsafe impl<T: VideoSinkImpl> IsSubclassable<T> for VideoSink {
    fn class_init(klass: &mut glib::Class<Self>) {
        <gst_base::BaseSink as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.show_frame = Some(video_sink_show_frame::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gst_base::BaseSink as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn video_sink_show_frame<T: VideoSinkImpl>(
    ptr: *mut ffi::GstVideoSink,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoSink> = from_glib_borrow(ptr);
    let buffer = from_glib_borrow(buffer);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        imp.show_frame(wrap.unsafe_cast_ref(), &buffer).into()
    })
    .into_glib()
}
