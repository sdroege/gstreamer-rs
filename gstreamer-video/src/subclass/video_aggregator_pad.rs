// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, ptr};

use glib::translate::*;
use gst_base::{prelude::*, subclass::prelude::*};

use crate::{subclass::AggregateFramesToken, VideoAggregator, VideoAggregatorPad};

pub trait VideoAggregatorPadImpl: VideoAggregatorPadImplExt + AggregatorPadImpl {
    fn update_conversion_info(&self) {
        self.parent_update_conversion_info()
    }

    fn prepare_frame(
        &self,
        aggregator: &crate::VideoAggregator,
        token: &AggregateFramesToken,
        buffer: &gst::Buffer,
    ) -> Option<crate::VideoFrame<crate::video_frame::Readable>> {
        self.parent_prepare_frame(aggregator, token, buffer)
    }

    fn clean_frame(
        &self,
        aggregator: &crate::VideoAggregator,
        token: &AggregateFramesToken,
        frame: Option<crate::VideoFrame<crate::video_frame::Readable>>,
    ) {
        self.parent_clean_frame(aggregator, token, frame)
    }
}

pub trait VideoAggregatorPadImplExt: ObjectSubclass {
    fn parent_update_conversion_info(&self);

    fn parent_prepare_frame(
        &self,
        aggregator: &crate::VideoAggregator,
        token: &AggregateFramesToken,
        buffer: &gst::Buffer,
    ) -> Option<crate::VideoFrame<crate::video_frame::Readable>>;

    fn parent_clean_frame(
        &self,
        aggregator: &crate::VideoAggregator,
        token: &AggregateFramesToken,
        frame: Option<crate::VideoFrame<crate::video_frame::Readable>>,
    );
}

impl<T: VideoAggregatorPadImpl> VideoAggregatorPadImplExt for T {
    fn parent_update_conversion_info(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoAggregatorPadClass;
            if let Some(f) = (*parent_class).update_conversion_info {
                f(self
                    .obj()
                    .unsafe_cast_ref::<VideoAggregatorPad>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_prepare_frame(
        &self,
        aggregator: &crate::VideoAggregator,
        token: &AggregateFramesToken,
        buffer: &gst::Buffer,
    ) -> Option<crate::VideoFrame<crate::video_frame::Readable>> {
        assert_eq!(
            aggregator.as_ptr() as *mut ffi::GstVideoAggregator,
            token.0.as_ptr() as *mut ffi::GstVideoAggregator
        );

        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoAggregatorPadClass;
            if let Some(f) = (*parent_class).prepare_frame {
                let mut prepared_frame = mem::MaybeUninit::zeroed();

                f(
                    self.obj()
                        .unsafe_cast_ref::<VideoAggregatorPad>()
                        .to_glib_none()
                        .0,
                    aggregator.to_glib_none().0,
                    buffer.as_mut_ptr(),
                    prepared_frame.as_mut_ptr(),
                );

                let prepared_frame = prepared_frame.assume_init();
                if prepared_frame.buffer.is_null() {
                    None
                } else {
                    Some(crate::VideoFrame::from_glib_full(prepared_frame))
                }
            } else {
                None
            }
        }
    }

    fn parent_clean_frame(
        &self,
        aggregator: &crate::VideoAggregator,
        token: &AggregateFramesToken,
        frame: Option<crate::VideoFrame<crate::video_frame::Readable>>,
    ) {
        assert_eq!(
            aggregator.as_ptr() as *mut ffi::GstVideoAggregator,
            token.0.as_ptr() as *mut ffi::GstVideoAggregator
        );

        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoAggregatorPadClass;
            if let Some(f) = (*parent_class).clean_frame {
                let mut prepared_frame = if let Some(frame) = frame {
                    frame.into_raw()
                } else {
                    mem::zeroed()
                };

                f(
                    self.obj()
                        .unsafe_cast_ref::<VideoAggregatorPad>()
                        .to_glib_none()
                        .0,
                    aggregator.to_glib_none().0,
                    &mut prepared_frame,
                );
            }
        }
    }
}

unsafe impl<T: VideoAggregatorPadImpl> IsSubclassable<T> for VideoAggregatorPad {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);

        let klass = klass.as_mut();
        klass.update_conversion_info = Some(video_aggregator_pad_update_conversion_info::<T>);
        klass.prepare_frame = Some(video_aggregator_pad_prepare_frame::<T>);
        klass.clean_frame = Some(video_aggregator_pad_clean_frame::<T>);
    }
}

unsafe extern "C" fn video_aggregator_pad_update_conversion_info<T: VideoAggregatorPadImpl>(
    ptr: *mut ffi::GstVideoAggregatorPad,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.update_conversion_info();
}

unsafe extern "C" fn video_aggregator_pad_prepare_frame<T: VideoAggregatorPadImpl>(
    ptr: *mut ffi::GstVideoAggregatorPad,
    aggregator: *mut ffi::GstVideoAggregator,
    buffer: *mut gst::ffi::GstBuffer,
    prepared_frame: *mut ffi::GstVideoFrame,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let aggregator: Borrowed<VideoAggregator> = from_glib_borrow(aggregator);

    let token = AggregateFramesToken(&aggregator);

    match imp.prepare_frame(&aggregator, &token, &from_glib_borrow(buffer)) {
        Some(frame) => {
            *prepared_frame = frame.into_raw();
        }
        None => {
            ptr::write(prepared_frame, mem::zeroed());
        }
    }

    glib::ffi::GTRUE
}

unsafe extern "C" fn video_aggregator_pad_clean_frame<T: VideoAggregatorPadImpl>(
    ptr: *mut ffi::GstVideoAggregatorPad,
    aggregator: *mut ffi::GstVideoAggregator,
    prepared_frame: *mut ffi::GstVideoFrame,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let aggregator: Borrowed<VideoAggregator> = from_glib_borrow(aggregator);

    let token = AggregateFramesToken(&aggregator);

    let frame = if (*prepared_frame).buffer.is_null() {
        None
    } else {
        let frame = crate::VideoFrame::from_glib_full(*prepared_frame);
        ptr::write(prepared_frame, mem::zeroed());
        Some(frame)
    };

    imp.clean_frame(&aggregator, &token, frame);
}
