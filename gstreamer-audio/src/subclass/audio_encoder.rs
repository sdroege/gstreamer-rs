// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_audio_sys;
use gst_sys;

use glib::translate::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;

use std::ptr;

use crate::prelude::*;

use AudioEncoder;
use AudioInfo;

pub trait AudioEncoderImpl: AudioEncoderImplExt + ElementImpl {
    fn open(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage> {
        self.parent_open(element)
    }

    fn close(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage> {
        self.parent_close(element)
    }

    fn start(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn set_format(
        &self,
        element: &AudioEncoder,
        info: &AudioInfo,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_format(element, info)
    }

    fn handle_frame(
        &self,
        element: &AudioEncoder,
        buffer: Option<&gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_handle_frame(element, buffer)
    }

    fn pre_push(
        &self,
        element: &AudioEncoder,
        buffer: gst::Buffer,
    ) -> Result<Option<gst::Buffer>, gst::FlowError> {
        self.parent_pre_push(element, buffer)
    }

    fn flush(&self, element: &AudioEncoder) {
        self.parent_flush(element)
    }

    fn negotiate(&self, element: &AudioEncoder) -> Result<(), gst::LoggableError> {
        self.parent_negotiate(element)
    }

    fn get_caps(&self, element: &AudioEncoder, filter: Option<&gst::Caps>) -> gst::Caps {
        self.parent_get_caps(element, filter)
    }

    fn sink_event(&self, element: &AudioEncoder, event: gst::Event) -> bool {
        self.parent_sink_event(element, event)
    }

    fn sink_query(&self, element: &AudioEncoder, query: &mut gst::QueryRef) -> bool {
        self.parent_sink_query(element, query)
    }

    fn src_event(&self, element: &AudioEncoder, event: gst::Event) -> bool {
        self.parent_src_event(element, event)
    }

    fn src_query(&self, element: &AudioEncoder, query: &mut gst::QueryRef) -> bool {
        self.parent_src_query(element, query)
    }

    fn propose_allocation(
        &self,
        element: &AudioEncoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        self.parent_propose_allocation(element, query)
    }

    fn decide_allocation(
        &self,
        element: &AudioEncoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        self.parent_decide_allocation(element, query)
    }
}

pub trait AudioEncoderImplExt {
    fn parent_open(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage>;

    fn parent_close(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage>;

    fn parent_start(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage>;

    fn parent_set_format(
        &self,
        element: &AudioEncoder,
        info: &AudioInfo,
    ) -> Result<(), gst::LoggableError>;

    fn parent_handle_frame(
        &self,
        element: &AudioEncoder,
        buffer: Option<&gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_pre_push(
        &self,
        element: &AudioEncoder,
        buffer: gst::Buffer,
    ) -> Result<Option<gst::Buffer>, gst::FlowError>;

    fn parent_flush(&self, element: &AudioEncoder);

    fn parent_negotiate(&self, element: &AudioEncoder) -> Result<(), gst::LoggableError>;

    fn parent_get_caps(&self, element: &AudioEncoder, filter: Option<&gst::Caps>) -> gst::Caps;

    fn parent_sink_event(&self, element: &AudioEncoder, event: gst::Event) -> bool;

    fn parent_sink_query(&self, element: &AudioEncoder, query: &mut gst::QueryRef) -> bool;

    fn parent_src_event(&self, element: &AudioEncoder, event: gst::Event) -> bool;

    fn parent_src_query(&self, element: &AudioEncoder, query: &mut gst::QueryRef) -> bool;

    fn parent_propose_allocation(
        &self,
        element: &AudioEncoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage>;

    fn parent_decide_allocation(
        &self,
        element: &AudioEncoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage>;
}

impl<T: AudioEncoderImpl> AudioEncoderImplExt for T {
    fn parent_open(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .open
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `open` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_close(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .close
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `close` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_start(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self, element: &AudioEncoder) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `stop` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_set_format(
        &self,
        element: &AudioEncoder,
        info: &AudioInfo,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .set_format
                .map(|f| {
                    gst_result_from_gboolean!(
                        f(element.to_glib_none().0, info.to_glib_none().0 as *mut _),
                        gst::CAT_RUST,
                        "parent function `set_format` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_handle_frame(
        &self,
        element: &AudioEncoder,
        buffer: Option<&gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.to_glib_none().0,
                        buffer
                            .map(|buffer| buffer.as_mut_ptr() as *mut *mut gst_sys::GstBuffer)
                            .unwrap_or(ptr::null_mut()),
                    ))
                })
                .unwrap_or(gst::FlowReturn::Error)
                .into_result()
        }
    }

    fn parent_pre_push(
        &self,
        element: &AudioEncoder,
        buffer: gst::Buffer,
    ) -> Result<Option<gst::Buffer>, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            if let Some(f) = (*parent_class).pre_push {
                let mut buffer = buffer.into_ptr();
                match gst::FlowReturn::from_glib(f(element.to_glib_none().0, &mut buffer))
                    .into_result()
                {
                    Ok(_) => Ok(from_glib_full(buffer)),
                    Err(err) => Err(err),
                }
            } else {
                Ok(Some(buffer))
            }
        }
    }

    fn parent_flush(&self, element: &AudioEncoder) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .flush
                .map(|f| f(element.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_negotiate(&self, element: &AudioEncoder) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .negotiate
                .map(|f| {
                    gst_result_from_gboolean!(
                        f(element.to_glib_none().0),
                        gst::CAT_RUST,
                        "Parent function `negotiate` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_get_caps(&self, element: &AudioEncoder, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .getcaps
                .map(|f| from_glib_full(f(element.to_glib_none().0, filter.to_glib_none().0)))
                .unwrap_or_else(|| element.proxy_getcaps(None, filter))
        }
    }

    fn parent_sink_event(&self, element: &AudioEncoder, event: gst::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            let f = (*parent_class)
                .sink_event
                .expect("Missing parent function `sink_event`");
            from_glib(f(element.to_glib_none().0, event.into_ptr()))
        }
    }

    fn parent_sink_query(&self, element: &AudioEncoder, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            let f = (*parent_class)
                .sink_query
                .expect("Missing parent function `sink_query`");
            from_glib(f(element.to_glib_none().0, query.as_mut_ptr()))
        }
    }

    fn parent_src_event(&self, element: &AudioEncoder, event: gst::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            let f = (*parent_class)
                .src_event
                .expect("Missing parent function `src_event`");
            from_glib(f(element.to_glib_none().0, event.into_ptr()))
        }
    }

    fn parent_src_query(&self, element: &AudioEncoder, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            let f = (*parent_class)
                .src_query
                .expect("Missing parent function `src_query`");
            from_glib(f(element.to_glib_none().0, query.as_mut_ptr()))
        }
    }

    fn parent_propose_allocation(
        &self,
        element: &AudioEncoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .propose_allocation
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0, query.as_mut_ptr())) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `propose_allocation` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_decide_allocation(
        &self,
        element: &AudioEncoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioEncoderClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0, query.as_mut_ptr())) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `decide_allocation` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }
}

unsafe impl<T: AudioEncoderImpl> IsSubclassable<T> for AudioEncoder
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <gst::Element as IsSubclassable<T>>::override_vfuncs(klass);
        unsafe {
            let klass = &mut *(klass.as_mut() as *mut gst_audio_sys::GstAudioEncoderClass);
            klass.open = Some(audio_encoder_open::<T>);
            klass.close = Some(audio_encoder_close::<T>);
            klass.start = Some(audio_encoder_start::<T>);
            klass.stop = Some(audio_encoder_stop::<T>);
            klass.set_format = Some(audio_encoder_set_format::<T>);
            klass.handle_frame = Some(audio_encoder_handle_frame::<T>);
            klass.pre_push = Some(audio_encoder_pre_push::<T>);
            klass.flush = Some(audio_encoder_flush::<T>);
            klass.negotiate = Some(audio_encoder_negotiate::<T>);
            klass.getcaps = Some(audio_encoder_getcaps::<T>);
            klass.sink_event = Some(audio_encoder_sink_event::<T>);
            klass.src_event = Some(audio_encoder_src_event::<T>);
            klass.sink_query = Some(audio_encoder_sink_query::<T>);
            klass.src_query = Some(audio_encoder_src_query::<T>);
            klass.propose_allocation = Some(audio_encoder_propose_allocation::<T>);
            klass.decide_allocation = Some(audio_encoder_decide_allocation::<T>);
        }
    }
}

unsafe extern "C" fn audio_encoder_open<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.open(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_close<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.close(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_start<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.start(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_stop<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.stop(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_set_format<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    info: *mut gst_audio_sys::GstAudioInfo,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.set_format(&wrap, &from_glib_none(info)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_handle_frame<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    buffer: *mut *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T::Instance: PanicPoison,
{
    // FIXME: Misgenerated in gstreamer-audio-sys
    let buffer = buffer as *mut gst_sys::GstBuffer;
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.handle_frame(
            &wrap,
            Option::<gst::Buffer>::from_glib_none(buffer).as_ref(),
        )
        .into()
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_pre_push<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    buffer: *mut *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match imp.pre_push(&wrap, from_glib_full(*buffer)) {
            Ok(Some(new_buffer)) => {
                *buffer = new_buffer.into_ptr();
                Ok(gst::FlowSuccess::Ok)
            }
            Ok(None) => {
                *buffer = ptr::null_mut();
                Ok(gst::FlowSuccess::Ok)
            }
            Err(err) => Err(err),
        }
        .into()
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_flush<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
) where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        AudioEncoderImpl::flush(imp, &wrap)
    })
}

unsafe extern "C" fn audio_encoder_negotiate<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.negotiate(&wrap) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_getcaps<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    filter: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        AudioEncoderImpl::get_caps(
            imp,
            &wrap,
            Option::<gst::Caps>::from_glib_borrow(filter)
                .as_ref()
                .as_ref(),
        )
    })
    .to_glib_full()
}

unsafe extern "C" fn audio_encoder_sink_event<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.sink_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_sink_query<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.sink_query(&wrap, gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_src_event<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.src_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_src_query<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.src_query(&wrap, gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_propose_allocation<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.propose_allocation(&wrap, query) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_encoder_decide_allocation<T: AudioEncoderImpl>(
    ptr: *mut gst_audio_sys::GstAudioEncoder,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioEncoder> = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.decide_allocation(&wrap, query) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}
