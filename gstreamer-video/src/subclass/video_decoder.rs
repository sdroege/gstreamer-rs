// Copyright (C) 2019 Philippe Normand <philn@igalia.com>
// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_sys;
use gst_video_sys;

use glib::translate::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;
use gst_base;

use crate::prelude::*;
use video_codec_state::{Readable, VideoCodecState};
use VideoCodecFrame;
use VideoDecoder;
use VideoDecoderClass;

pub trait VideoDecoderImpl: VideoDecoderImplExt + ElementImpl + Send + Sync + 'static {
    fn open(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage> {
        self.parent_open(element)
    }

    fn close(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage> {
        self.parent_close(element)
    }

    fn start(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn finish(&self, element: &VideoDecoder) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_finish(element)
    }

    fn drain(&self, element: &VideoDecoder) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_drain(element)
    }

    fn set_format(
        &self,
        element: &VideoDecoder,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_format(element, state)
    }

    fn parse(
        &self,
        element: &VideoDecoder,
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_parse(element, frame, adapter, at_eos)
    }

    fn handle_frame(
        &self,
        element: &VideoDecoder,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_handle_frame(element, frame)
    }

    fn flush(&self, element: &VideoDecoder) -> bool {
        self.parent_flush(element)
    }

    fn negotiate(&self, element: &VideoDecoder) -> Result<(), gst::LoggableError> {
        self.parent_negotiate(element)
    }

    fn get_caps(&self, element: &VideoDecoder, filter: Option<&gst::Caps>) -> gst::Caps {
        self.parent_get_caps(element, filter)
    }

    fn sink_event(&self, element: &VideoDecoder, event: gst::Event) -> bool {
        self.parent_sink_event(element, event)
    }

    fn sink_query(&self, element: &VideoDecoder, query: &mut gst::QueryRef) -> bool {
        self.parent_sink_query(element, query)
    }

    fn src_event(&self, element: &VideoDecoder, event: gst::Event) -> bool {
        self.parent_src_event(element, event)
    }

    fn src_query(&self, element: &VideoDecoder, query: &mut gst::QueryRef) -> bool {
        self.parent_src_query(element, query)
    }

    fn propose_allocation(
        &self,
        element: &VideoDecoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        self.parent_propose_allocation(element, query)
    }

    fn decide_allocation(
        &self,
        element: &VideoDecoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        self.parent_decide_allocation(element, query)
    }
}

pub trait VideoDecoderImplExt {
    fn parent_open(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage>;

    fn parent_close(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage>;

    fn parent_start(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage>;

    fn parent_finish(&self, element: &VideoDecoder) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_drain(&self, element: &VideoDecoder) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_set_format(
        &self,
        element: &VideoDecoder,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError>;

    fn parent_parse(
        &self,
        element: &VideoDecoder,
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_handle_frame(
        &self,
        element: &VideoDecoder,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_flush(&self, element: &VideoDecoder) -> bool;

    fn parent_negotiate(&self, element: &VideoDecoder) -> Result<(), gst::LoggableError>;

    fn parent_get_caps(&self, element: &VideoDecoder, filter: Option<&gst::Caps>) -> gst::Caps;

    fn parent_sink_event(&self, element: &VideoDecoder, event: gst::Event) -> bool;

    fn parent_sink_query(&self, element: &VideoDecoder, query: &mut gst::QueryRef) -> bool;

    fn parent_src_event(&self, element: &VideoDecoder, event: gst::Event) -> bool;

    fn parent_src_query(&self, element: &VideoDecoder, query: &mut gst::QueryRef) -> bool;

    fn parent_propose_allocation(
        &self,
        element: &VideoDecoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage>;

    fn parent_decide_allocation(
        &self,
        element: &VideoDecoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage>;
}

impl<T: VideoDecoderImpl + ObjectImpl> VideoDecoderImplExt for T {
    fn parent_open(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
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

    fn parent_close(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
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

    fn parent_start(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
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

    fn parent_stop(&self, element: &VideoDecoder) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
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

    fn parent_finish(&self, element: &VideoDecoder) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            (*parent_class)
                .finish
                .map(|f| gst::FlowReturn::from_glib(f(element.to_glib_none().0)))
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_drain(&self, element: &VideoDecoder) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            (*parent_class)
                .drain
                .map(|f| gst::FlowReturn::from_glib(f(element.to_glib_none().0)))
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_set_format(
        &self,
        element: &VideoDecoder,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            (*parent_class)
                .set_format
                .map(|f| {
                    gst_result_from_gboolean!(
                        f(element.to_glib_none().0, state.as_mut_ptr()),
                        gst::CAT_RUST,
                        "parent function `set_format` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_parse(
        &self,
        element: &VideoDecoder,
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            (*parent_class)
                .parse
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.to_glib_none().0,
                        frame.to_glib_none().0,
                        adapter.to_glib_none().0,
                        at_eos.to_glib(),
                    ))
                })
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_handle_frame(
        &self,
        element: &VideoDecoder,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    gst::FlowReturn::from_glib(f(element.to_glib_none().0, frame.to_glib_none().0))
                })
                .unwrap_or(gst::FlowReturn::Error)
                .into_result()
        }
    }

    fn parent_flush(&self, element: &VideoDecoder) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            (*parent_class)
                .flush
                .map(|f| from_glib(f(element.to_glib_none().0)))
                .unwrap_or(false)
        }
    }

    fn parent_negotiate(&self, element: &VideoDecoder) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
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

    fn parent_get_caps(&self, element: &VideoDecoder, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            (*parent_class)
                .getcaps
                .map(|f| from_glib_full(f(element.to_glib_none().0, filter.to_glib_none().0)))
                .unwrap_or_else(|| element.proxy_getcaps(None, filter))
        }
    }

    fn parent_sink_event(&self, element: &VideoDecoder, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            let f = (*parent_class)
                .sink_event
                .expect("Missing parent function `sink_event`");
            from_glib(f(element.to_glib_none().0, event.into_ptr()))
        }
    }

    fn parent_sink_query(&self, element: &VideoDecoder, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            let f = (*parent_class)
                .sink_query
                .expect("Missing parent function `sink_query`");
            from_glib(f(element.to_glib_none().0, query.as_mut_ptr()))
        }
    }

    fn parent_src_event(&self, element: &VideoDecoder, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            let f = (*parent_class)
                .src_event
                .expect("Missing parent function `src_event`");
            from_glib(f(element.to_glib_none().0, event.into_ptr()))
        }
    }

    fn parent_src_query(&self, element: &VideoDecoder, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
            let f = (*parent_class)
                .src_query
                .expect("Missing parent function `src_query`");
            from_glib(f(element.to_glib_none().0, query.as_mut_ptr()))
        }
    }

    fn parent_propose_allocation(
        &self,
        element: &VideoDecoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
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
        element: &VideoDecoder,
        query: &mut gst::QueryRef,
    ) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_video_sys::GstVideoDecoderClass;
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

unsafe impl<T: ObjectSubclass + VideoDecoderImpl> IsSubclassable<T> for VideoDecoderClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_video_sys::GstVideoDecoderClass);
            klass.open = Some(video_decoder_open::<T>);
            klass.close = Some(video_decoder_close::<T>);
            klass.start = Some(video_decoder_start::<T>);
            klass.stop = Some(video_decoder_stop::<T>);
            klass.finish = Some(video_decoder_finish::<T>);
            klass.drain = Some(video_decoder_drain::<T>);
            klass.set_format = Some(video_decoder_set_format::<T>);
            klass.parse = Some(video_decoder_parse::<T>);
            klass.handle_frame = Some(video_decoder_handle_frame::<T>);
            klass.flush = Some(video_decoder_flush::<T>);
            klass.negotiate = Some(video_decoder_negotiate::<T>);
            klass.getcaps = Some(video_decoder_getcaps::<T>);
            klass.sink_event = Some(video_decoder_sink_event::<T>);
            klass.src_event = Some(video_decoder_src_event::<T>);
            klass.sink_query = Some(video_decoder_sink_query::<T>);
            klass.src_query = Some(video_decoder_src_query::<T>);
            klass.propose_allocation = Some(video_decoder_propose_allocation::<T>);
            klass.decide_allocation = Some(video_decoder_decide_allocation::<T>);
        }
    }
}

unsafe extern "C" fn video_decoder_open<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_close<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_start<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_stop<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_finish<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
) -> gst_sys::GstFlowReturn
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.finish(&wrap).into()
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_drain<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
) -> gst_sys::GstFlowReturn
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.drain(&wrap).into()
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_set_format<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    state: *mut gst_video_sys::GstVideoCodecState,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
    gst_video_sys::gst_video_codec_state_ref(state);
    let wrap_state = VideoCodecState::<Readable>::new(state);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.set_format(&wrap, &wrap_state) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_parse<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    frame: *mut gst_video_sys::GstVideoCodecFrame,
    adapter: *mut gst_base_sys::GstAdapter,
    at_eos: glib_sys::gboolean,
) -> gst_sys::GstFlowReturn
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
    gst_video_sys::gst_video_codec_frame_ref(frame);
    let wrap_frame = VideoCodecFrame::new(frame, &*wrap);
    let wrap_adapter: Borrowed<gst_base::Adapter> = from_glib_borrow(adapter);
    let at_eos: bool = from_glib(at_eos);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.parse(&wrap, &wrap_frame, &wrap_adapter, at_eos).into()
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_handle_frame<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    frame: *mut gst_video_sys::GstVideoCodecFrame,
) -> gst_sys::GstFlowReturn
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
    let wrap_frame = VideoCodecFrame::new(frame, &*wrap);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.handle_frame(&wrap, wrap_frame).into()
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_flush<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        VideoDecoderImpl::flush(imp, &wrap)
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_negotiate<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_getcaps<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    filter: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        VideoDecoderImpl::get_caps(
            imp,
            &wrap,
            Option::<gst::Caps>::from_glib_borrow(filter)
                .as_ref()
                .as_ref(),
        )
    })
    .to_glib_full()
}

unsafe extern "C" fn video_decoder_sink_event<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.sink_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_sink_query<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.sink_query(&wrap, gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_src_event<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.src_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_src_query<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.src_query(&wrap, gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_propose_allocation<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
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

unsafe extern "C" fn video_decoder_decide_allocation<T: ObjectSubclass>(
    ptr: *mut gst_video_sys::GstVideoDecoder,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: VideoDecoderImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
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
