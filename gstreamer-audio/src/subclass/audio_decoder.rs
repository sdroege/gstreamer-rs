// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;

use gst::subclass::prelude::*;

use std::mem;
use std::ptr;

use crate::prelude::*;

use crate::AudioDecoder;

pub trait AudioDecoderImpl: AudioDecoderImplExt + ElementImpl {
    fn open(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_open(element)
    }

    fn close(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_close(element)
    }

    fn start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn set_format(&self, element: &Self::Type, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_format(element, caps)
    }

    fn parse(
        &self,
        element: &Self::Type,
        adapter: &gst_base::Adapter,
    ) -> Result<(u32, u32), gst::FlowError> {
        self.parent_parse(element, adapter)
    }

    fn handle_frame(
        &self,
        element: &Self::Type,
        buffer: Option<&gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_handle_frame(element, buffer)
    }

    fn pre_push(
        &self,
        element: &Self::Type,
        buffer: gst::Buffer,
    ) -> Result<Option<gst::Buffer>, gst::FlowError> {
        self.parent_pre_push(element, buffer)
    }

    fn flush(&self, element: &Self::Type, hard: bool) {
        self.parent_flush(element, hard)
    }

    fn negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError> {
        self.parent_negotiate(element)
    }

    fn get_caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> gst::Caps {
        self.parent_get_caps(element, filter)
    }

    fn sink_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        self.parent_sink_event(element, event)
    }

    fn sink_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        self.parent_sink_query(element, query)
    }

    fn src_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        self.parent_src_event(element, event)
    }

    fn src_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        self.parent_src_query(element, query)
    }

    fn propose_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        self.parent_propose_allocation(element, query)
    }

    fn decide_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        self.parent_decide_allocation(element, query)
    }
}

pub trait AudioDecoderImplExt: ObjectSubclass {
    fn parent_open(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_close(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_set_format(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;

    fn parent_parse(
        &self,
        element: &Self::Type,
        adapter: &gst_base::Adapter,
    ) -> Result<(u32, u32), gst::FlowError>;

    fn parent_handle_frame(
        &self,
        element: &Self::Type,
        buffer: Option<&gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_pre_push(
        &self,
        element: &Self::Type,
        buffer: gst::Buffer,
    ) -> Result<Option<gst::Buffer>, gst::FlowError>;

    fn parent_flush(&self, element: &Self::Type, hard: bool);

    fn parent_negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError>;

    fn parent_get_caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> gst::Caps;

    fn parent_sink_event(&self, element: &Self::Type, event: gst::Event) -> bool;

    fn parent_sink_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool;

    fn parent_src_event(&self, element: &Self::Type, event: gst::Event) -> bool;

    fn parent_src_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool;

    fn parent_propose_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage>;

    fn parent_decide_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage>;
}

impl<T: AudioDecoderImpl> AudioDecoderImplExt for T {
    fn parent_open(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .open
                .map(|f| {
                    if from_glib(f(element
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `open` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_close(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .close
                .map(|f| {
                    if from_glib(f(element
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `close` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element
                        .unsafe_cast_ref::<AudioDecoder>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
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
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .set_format
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                            caps.to_glib_none().0
                        ),
                        gst::CAT_RUST,
                        "parent function `set_format` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_parse(
        &self,
        element: &Self::Type,
        adapter: &gst_base::Adapter,
    ) -> Result<(u32, u32), gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .parse
                .map(|f| {
                    let mut offset = mem::MaybeUninit::uninit();
                    let mut len = mem::MaybeUninit::uninit();
                    match gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                        adapter.to_glib_none().0,
                        offset.as_mut_ptr(),
                        len.as_mut_ptr(),
                    ))
                    .into_result()
                    {
                        Ok(_) => {
                            let offset = offset.assume_init();
                            let len = len.assume_init();
                            assert!(offset >= 0);
                            assert!(len >= 0);
                            Ok((offset as u32, len as u32))
                        }
                        Err(err) => Err(err),
                    }
                })
                .unwrap_or_else(|| Ok((0, adapter.available() as u32)))
        }
    }

    fn parent_handle_frame(
        &self,
        element: &Self::Type,
        buffer: Option<&gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                        buffer
                            .map(|buffer| buffer.as_mut_ptr() as *mut *mut gst::ffi::GstBuffer)
                            .unwrap_or(ptr::null_mut()),
                    ))
                })
                .unwrap_or(gst::FlowReturn::Error)
                .into_result()
        }
    }

    fn parent_pre_push(
        &self,
        element: &Self::Type,
        buffer: gst::Buffer,
    ) -> Result<Option<gst::Buffer>, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            if let Some(f) = (*parent_class).pre_push {
                let mut buffer = buffer.into_ptr();
                match gst::FlowReturn::from_glib(f(
                    element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                    &mut buffer,
                ))
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

    fn parent_flush(&self, element: &Self::Type, hard: bool) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .flush
                .map(|f| {
                    f(
                        element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                        hard.to_glib(),
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .negotiate
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0),
                        gst::CAT_RUST,
                        "Parent function `negotiate` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_get_caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .getcaps
                .map(|f| {
                    from_glib_full(f(
                        element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or_else(|| {
                    element
                        .unsafe_cast_ref::<AudioDecoder>()
                        .proxy_getcaps(None, filter)
                })
        }
    }

    fn parent_sink_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            let f = (*parent_class)
                .sink_event
                .expect("Missing parent function `sink_event`");
            from_glib(f(
                element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn parent_sink_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            let f = (*parent_class)
                .sink_query
                .expect("Missing parent function `sink_query`");
            from_glib(f(
                element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_src_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            let f = (*parent_class)
                .src_event
                .expect("Missing parent function `src_event`");
            from_glib(f(
                element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn parent_src_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            let f = (*parent_class)
                .src_query
                .expect("Missing parent function `src_query`");
            from_glib(f(
                element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_propose_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .propose_allocation
                .map(|f| {
                    if from_glib(f(
                        element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                        query.as_mut_ptr(),
                    )) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
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
        element: &Self::Type,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioDecoderClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    if from_glib(f(
                        element.unsafe_cast_ref::<AudioDecoder>().to_glib_none().0,
                        query.as_mut_ptr(),
                    )) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `decide_allocation` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }
}

unsafe impl<T: AudioDecoderImpl> IsSubclassable<T> for AudioDecoder {
    fn class_init(klass: &mut glib::Class<Self>) {
        <gst::Element as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.open = Some(audio_decoder_open::<T>);
        klass.close = Some(audio_decoder_close::<T>);
        klass.start = Some(audio_decoder_start::<T>);
        klass.stop = Some(audio_decoder_stop::<T>);
        klass.set_format = Some(audio_decoder_set_format::<T>);
        klass.parse = Some(audio_decoder_parse::<T>);
        klass.handle_frame = Some(audio_decoder_handle_frame::<T>);
        klass.pre_push = Some(audio_decoder_pre_push::<T>);
        klass.flush = Some(audio_decoder_flush::<T>);
        klass.negotiate = Some(audio_decoder_negotiate::<T>);
        klass.getcaps = Some(audio_decoder_getcaps::<T>);
        klass.sink_event = Some(audio_decoder_sink_event::<T>);
        klass.src_event = Some(audio_decoder_src_event::<T>);
        klass.sink_query = Some(audio_decoder_sink_query::<T>);
        klass.src_query = Some(audio_decoder_src_query::<T>);
        klass.propose_allocation = Some(audio_decoder_propose_allocation::<T>);
        klass.decide_allocation = Some(audio_decoder_decide_allocation::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gst::Element as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn audio_decoder_open<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.open(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_close<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.close(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_start<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.start(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_stop<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.stop(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_set_format<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.set_format(wrap.unsafe_cast_ref(), &from_glib_borrow(caps)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_parse<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    adapter: *mut gst_base::ffi::GstAdapter,
    offset: *mut i32,
    len: *mut i32,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        match imp.parse(wrap.unsafe_cast_ref(), &from_glib_borrow(adapter)) {
            Ok((new_offset, new_len)) => {
                assert!(new_offset <= std::i32::MAX as u32);
                assert!(new_len <= std::i32::MAX as u32);
                *offset = new_offset as i32;
                *len = new_len as i32;
                Ok(gst::FlowSuccess::Ok)
            }
            Err(err) => Err(err),
        }
        .into()
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_handle_frame<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    buffer: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    // FIXME: Misgenerated in gstreamer-audio-sys
    let buffer = buffer as *mut gst::ffi::GstBuffer;
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        imp.handle_frame(
            wrap.unsafe_cast_ref(),
            Option::<gst::Buffer>::from_glib_none(buffer).as_ref(),
        )
        .into()
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_pre_push<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    buffer: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        match imp.pre_push(wrap.unsafe_cast_ref(), from_glib_full(*buffer)) {
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

unsafe extern "C" fn audio_decoder_flush<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    hard: glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), (), {
        AudioDecoderImpl::flush(imp, wrap.unsafe_cast_ref(), from_glib(hard))
    })
}

unsafe extern "C" fn audio_decoder_negotiate<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.negotiate(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_getcaps<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::Caps::new_empty(), {
        AudioDecoderImpl::get_caps(
            imp,
            wrap.unsafe_cast_ref(),
            Option::<gst::Caps>::from_glib_borrow(filter)
                .as_ref()
                .as_ref(),
        )
    })
    .to_glib_full()
}

unsafe extern "C" fn audio_decoder_sink_event<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.sink_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_sink_query<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.sink_query(wrap.unsafe_cast_ref(), gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_src_event<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.src_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_src_query<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.src_query(wrap.unsafe_cast_ref(), gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_propose_allocation<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.propose_allocation(wrap.unsafe_cast_ref(), query) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audio_decoder_decide_allocation<T: AudioDecoderImpl>(
    ptr: *mut ffi::GstAudioDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<AudioDecoder> = from_glib_borrow(ptr);
    let query = gst::QueryRef::from_mut_ptr(query);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.decide_allocation(wrap.unsafe_cast_ref(), query) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}
