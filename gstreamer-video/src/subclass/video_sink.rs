// Copyright (C) 2020 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;
use gst_video_sys;

use glib::translate::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;
use gst_base::subclass::prelude::*;

use VideoSink;
use VideoSinkClass;

pub trait VideoSinkImpl:
    VideoSinkImplExt + BaseSinkImpl + ElementImpl + Send + Sync + 'static
{
    fn show_frame(
        &self,
        element: &VideoSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_show_frame(element, buffer)
    }
}

pub trait VideoSinkImplExt {
    fn parent_show_frame(
        &self,
        element: &VideoSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<T: VideoSinkImpl + ObjectImpl> VideoSinkImplExt for T {
    fn parent_show_frame(
        &self,
        element: &VideoSink,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoSinkClass;
            (*parent_class)
                .show_frame
                .map(|f| {
                    gst::FlowReturn::from_glib(f(element.to_glib_none().0, buffer.to_glib_none().0))
                })
                .unwrap_or(gst::FlowReturn::Error)
                .into_result()
        }
    }
}

unsafe impl<T: ObjectSubclass + VideoSinkImpl> IsSubclassable<T> for VideoSinkClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst_base::BaseSinkClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_video_sys::GstVideoSinkClass);
            klass.show_frame = Some(video_sink_show_frame::<T>);
        }
    }
}

unsafe extern "C" fn video_sink_show_frame<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoSink,
    buffer: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: VideoSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoSink> = from_glib_borrow(ptr);
    let buffer = from_glib_borrow(buffer);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.show_frame(&wrap, &buffer).into()
    })
    .to_glib()
}
