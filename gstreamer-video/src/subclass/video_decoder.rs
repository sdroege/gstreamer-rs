// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use gst::subclass::prelude::*;

use crate::prelude::*;
use crate::video_codec_state::{Readable, VideoCodecState};
use crate::VideoCodecFrame;
use crate::VideoDecoder;

pub trait VideoDecoderImpl: VideoDecoderImplExt + ElementImpl {
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

    fn finish(&self, element: &Self::Type) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_finish(element)
    }

    fn drain(&self, element: &Self::Type) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_drain(element)
    }

    fn set_format(
        &self,
        element: &Self::Type,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_format(element, state)
    }

    fn parse(
        &self,
        element: &Self::Type,
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_parse(element, frame, adapter, at_eos)
    }

    fn handle_frame(
        &self,
        element: &Self::Type,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_handle_frame(element, frame)
    }

    fn flush(&self, element: &Self::Type) -> bool {
        self.parent_flush(element)
    }

    fn negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError> {
        self.parent_negotiate(element)
    }

    fn caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> gst::Caps {
        self.parent_caps(element, filter)
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

pub trait VideoDecoderImplExt: ObjectSubclass {
    fn parent_open(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_close(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_finish(&self, element: &Self::Type) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_drain(&self, element: &Self::Type) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_set_format(
        &self,
        element: &Self::Type,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError>;

    fn parent_parse(
        &self,
        element: &Self::Type,
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_handle_frame(
        &self,
        element: &Self::Type,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_flush(&self, element: &Self::Type) -> bool;

    fn parent_negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError>;

    fn parent_caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> gst::Caps;

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

impl<T: VideoDecoderImpl> VideoDecoderImplExt for T {
    fn parent_open(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .open
                .map(|f| {
                    if from_glib(f(element
                        .unsafe_cast_ref::<VideoDecoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .close
                .map(|f| {
                    if from_glib(f(element
                        .unsafe_cast_ref::<VideoDecoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element
                        .unsafe_cast_ref::<VideoDecoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element
                        .unsafe_cast_ref::<VideoDecoder>()
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

    fn parent_finish(&self, element: &Self::Type) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .finish
                .map(|f| {
                    gst::FlowReturn::from_glib(f(element
                        .unsafe_cast_ref::<VideoDecoder>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_drain(&self, element: &Self::Type) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .drain
                .map(|f| {
                    gst::FlowReturn::from_glib(f(element
                        .unsafe_cast_ref::<VideoDecoder>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_set_format(
        &self,
        element: &Self::Type,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .set_format
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
                            state.as_mut_ptr()
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
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .parse
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
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
        element: &Self::Type,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    gst::FlowReturn::from_glib(f(
                        element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
                        frame.to_glib_none().0,
                    ))
                })
                .unwrap_or(gst::FlowReturn::Error)
                .into_result()
        }
    }

    fn parent_flush(&self, element: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .flush
                .map(|f| {
                    from_glib(f(element
                        .unsafe_cast_ref::<VideoDecoder>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(false)
        }
    }

    fn parent_negotiate(&self, element: &Self::Type) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .negotiate
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0),
                        gst::CAT_RUST,
                        "Parent function `negotiate` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_caps(&self, element: &Self::Type, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .getcaps
                .map(|f| {
                    from_glib_full(f(
                        element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or_else(|| {
                    element
                        .unsafe_cast_ref::<VideoDecoder>()
                        .proxy_getcaps(None, filter)
                })
        }
    }

    fn parent_sink_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            let f = (*parent_class)
                .sink_event
                .expect("Missing parent function `sink_event`");
            from_glib(f(
                element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn parent_sink_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            let f = (*parent_class)
                .sink_query
                .expect("Missing parent function `sink_query`");
            from_glib(f(
                element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_src_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            let f = (*parent_class)
                .src_event
                .expect("Missing parent function `src_event`");
            from_glib(f(
                element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn parent_src_query(&self, element: &Self::Type, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            let f = (*parent_class)
                .src_query
                .expect("Missing parent function `src_query`");
            from_glib(f(
                element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .propose_allocation
                .map(|f| {
                    if from_glib(f(
                        element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    if from_glib(f(
                        element.unsafe_cast_ref::<VideoDecoder>().to_glib_none().0,
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

unsafe impl<T: VideoDecoderImpl> IsSubclassable<T> for VideoDecoder {
    fn class_init(klass: &mut glib::Class<Self>) {
        <gst::Element as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
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

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gst::Element as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn video_decoder_open<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_close<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_start<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_stop<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_finish<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        imp.finish(wrap.unsafe_cast_ref()).into()
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_drain<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        imp.drain(wrap.unsafe_cast_ref()).into()
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_set_format<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    state: *mut ffi::GstVideoCodecState,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
    ffi::gst_video_codec_state_ref(state);
    let wrap_state = VideoCodecState::<Readable>::new(state);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.set_format(wrap.unsafe_cast_ref(), &wrap_state) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_parse<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    frame: *mut ffi::GstVideoCodecFrame,
    adapter: *mut gst_base::ffi::GstAdapter,
    at_eos: glib::ffi::gboolean,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
    ffi::gst_video_codec_frame_ref(frame);
    let wrap_frame = VideoCodecFrame::new(frame, &*wrap);
    let wrap_adapter: Borrowed<gst_base::Adapter> = from_glib_borrow(adapter);
    let at_eos: bool = from_glib(at_eos);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        imp.parse(wrap.unsafe_cast_ref(), &wrap_frame, &wrap_adapter, at_eos)
            .into()
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_handle_frame<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    frame: *mut ffi::GstVideoCodecFrame,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
    let wrap_frame = VideoCodecFrame::new(frame, &*wrap);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        imp.handle_frame(wrap.unsafe_cast_ref(), wrap_frame).into()
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_flush<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        VideoDecoderImpl::flush(imp, wrap.unsafe_cast_ref())
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_negotiate<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

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

unsafe extern "C" fn video_decoder_getcaps<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::Caps::new_empty(), {
        VideoDecoderImpl::caps(
            imp,
            wrap.unsafe_cast_ref(),
            Option::<gst::Caps>::from_glib_borrow(filter)
                .as_ref()
                .as_ref(),
        )
    })
    .to_glib_full()
}

unsafe extern "C" fn video_decoder_sink_event<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.sink_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_sink_query<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.sink_query(wrap.unsafe_cast_ref(), gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_src_event<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.src_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_src_query<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.src_query(wrap.unsafe_cast_ref(), gst::QueryRef::from_mut_ptr(query))
    })
    .to_glib()
}

unsafe extern "C" fn video_decoder_propose_allocation<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
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

unsafe extern "C" fn video_decoder_decide_allocation<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoDecoder> = from_glib_borrow(ptr);
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
