// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::translate::*;
use gst_base::{prelude::*, subclass::prelude::*};

use crate::AudioAggregatorPad;

pub trait AudioAggregatorPadImpl: AudioAggregatorPadImplExt + AggregatorPadImpl {
    const HANDLE_CONVERSION: bool = false;

    fn update_conversion_info(&self) {
        self.parent_update_conversion_info()
    }

    fn convert_buffer(
        &self,
        in_info: &crate::AudioInfo,
        out_info: &crate::AudioInfo,
        buffer: &gst::Buffer,
    ) -> Option<gst::Buffer> {
        self.parent_convert_buffer(in_info, out_info, buffer)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AudioAggregatorPadImplExt> Sealed for T {}
}

pub trait AudioAggregatorPadImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_update_conversion_info(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioAggregatorPadClass;
            if let Some(f) = (*parent_class).update_conversion_info {
                f(self
                    .obj()
                    .unsafe_cast_ref::<AudioAggregatorPad>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_convert_buffer(
        &self,
        in_info: &crate::AudioInfo,
        out_info: &crate::AudioInfo,
        buffer: &gst::Buffer,
    ) -> Option<gst::Buffer> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioAggregatorPadClass;
            let f = (*parent_class)
                .convert_buffer
                .expect("Missing parent function `convert_buffer`");
            from_glib_full(f(
                self.obj()
                    .unsafe_cast_ref::<AudioAggregatorPad>()
                    .to_glib_none()
                    .0,
                mut_override(in_info.to_glib_none().0),
                mut_override(out_info.to_glib_none().0),
                buffer.as_mut_ptr(),
            ))
        }
    }
}

impl<T: AudioAggregatorPadImpl> AudioAggregatorPadImplExt for T {}

unsafe impl<T: AudioAggregatorPadImpl> IsSubclassable<T> for AudioAggregatorPad {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);

        let klass = klass.as_mut();
        if T::HANDLE_CONVERSION {
            klass.update_conversion_info = Some(audio_aggregator_pad_update_conversion_info::<T>);
            klass.convert_buffer = Some(audio_aggregator_pad_convert_buffer::<T>);
        }
    }
}

unsafe extern "C" fn audio_aggregator_pad_update_conversion_info<T: AudioAggregatorPadImpl>(
    ptr: *mut ffi::GstAudioAggregatorPad,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.update_conversion_info();
}

unsafe extern "C" fn audio_aggregator_pad_convert_buffer<T: AudioAggregatorPadImpl>(
    ptr: *mut ffi::GstAudioAggregatorPad,
    in_info: *mut ffi::GstAudioInfo,
    out_info: *mut ffi::GstAudioInfo,
    buffer: *mut gst::ffi::GstBuffer,
) -> *mut gst::ffi::GstBuffer {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.convert_buffer(
        &from_glib_none(in_info),
        &from_glib_none(out_info),
        &from_glib_borrow(buffer),
    )
    .map(|buffer| buffer.into_glib_ptr())
    .unwrap_or(ptr::null_mut())
}
