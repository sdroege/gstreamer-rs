// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::translate::*;
use gst_base::{prelude::*, subclass::prelude::*};

use crate::{AudioAggregator, AudioAggregatorPad};

pub trait AudioAggregatorImpl: AudioAggregatorImplExt + AggregatorImpl {
    fn create_output_buffer(&self, num_frames: u32) -> Option<gst::Buffer> {
        self.parent_create_output_buffer(num_frames)
    }

    #[allow(clippy::too_many_arguments)]
    fn aggregate_one_buffer(
        &self,
        pad: &AudioAggregatorPad,
        inbuf: &gst::BufferRef,
        in_offset: u32,
        outbuf: &mut gst::BufferRef,
        out_offset: u32,
        num_frames: u32,
    ) -> bool {
        self.parent_aggregate_one_buffer(pad, inbuf, in_offset, outbuf, out_offset, num_frames)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AudioAggregatorImplExt> Sealed for T {}
}

pub trait AudioAggregatorImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_create_output_buffer(&self, num_frames: u32) -> Option<gst::Buffer> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioAggregatorClass;
            let f = (*parent_class)
                .create_output_buffer
                .expect("Missing parent function `create_output_buffer`");

            from_glib_full(f(
                self.obj()
                    .unsafe_cast_ref::<AudioAggregator>()
                    .to_glib_none()
                    .0,
                num_frames,
            ))
        }
    }

    fn parent_aggregate_one_buffer(
        &self,
        pad: &AudioAggregatorPad,
        inbuf: &gst::BufferRef,
        in_offset: u32,
        outbuf: &mut gst::BufferRef,
        out_offset: u32,
        num_frames: u32,
    ) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioAggregatorClass;
            let f = (*parent_class)
                .aggregate_one_buffer
                .expect("Missing parent function `aggregate_one_buffer`");

            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<AudioAggregator>()
                    .to_glib_none()
                    .0,
                pad.to_glib_none().0,
                inbuf.as_mut_ptr(),
                in_offset,
                outbuf.as_mut_ptr(),
                out_offset,
                num_frames,
            ))
        }
    }
}

impl<T: AudioAggregatorImpl> AudioAggregatorImplExt for T {}

unsafe impl<T: AudioAggregatorImpl> IsSubclassable<T> for AudioAggregator {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);

        let klass = klass.as_mut();
        klass.create_output_buffer = Some(audio_aggregator_create_output_buffer::<T>);
        klass.aggregate_one_buffer = Some(audio_aggregator_aggregate_one_buffer::<T>);
    }
}

unsafe extern "C" fn audio_aggregator_create_output_buffer<T: AudioAggregatorImpl>(
    ptr: *mut ffi::GstAudioAggregator,
    num_frames: u32,
) -> *mut gst::ffi::GstBuffer {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, None, { imp.create_output_buffer(num_frames) })
        .map(|buffer| buffer.into_glib_ptr())
        .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn audio_aggregator_aggregate_one_buffer<T: AudioAggregatorImpl>(
    ptr: *mut ffi::GstAudioAggregator,
    pad: *mut ffi::GstAudioAggregatorPad,
    inbuf: *mut gst::ffi::GstBuffer,
    in_offset: u32,
    outbuf: *mut gst::ffi::GstBuffer,
    out_offset: u32,
    num_frames: u32,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, true, {
        imp.aggregate_one_buffer(
            &from_glib_borrow(pad),
            gst::BufferRef::from_ptr(inbuf),
            in_offset,
            gst::BufferRef::from_mut_ptr(outbuf),
            out_offset,
            num_frames,
        )
    })
    .into_glib()
}
