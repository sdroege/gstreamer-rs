// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use gst::subclass::prelude::*;

use crate::{
    prelude::*,
    video_codec_state::{Readable, VideoCodecState},
    VideoCodecFrame, VideoDecoder,
};

pub trait VideoDecoderImpl: VideoDecoderImplExt + ElementImpl {
    fn open(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_open()
    }

    fn close(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_close()
    }

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_start()
    }

    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_stop()
    }

    fn finish(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_finish()
    }

    fn drain(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_drain()
    }

    fn set_format(
        &self,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_format(state)
    }

    fn parse(
        &self,
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_parse(frame, adapter, at_eos)
    }

    fn handle_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_handle_frame(frame)
    }

    fn flush(&self) -> bool {
        self.parent_flush()
    }

    fn negotiate(&self) -> Result<(), gst::LoggableError> {
        self.parent_negotiate()
    }

    fn caps(&self, filter: Option<&gst::Caps>) -> gst::Caps {
        self.parent_caps(filter)
    }

    fn sink_event(&self, event: gst::Event) -> bool {
        self.parent_sink_event(event)
    }

    fn sink_query(&self, query: &mut gst::QueryRef) -> bool {
        self.parent_sink_query(query)
    }

    fn src_event(&self, event: gst::Event) -> bool {
        self.parent_src_event(event)
    }

    fn src_query(&self, query: &mut gst::QueryRef) -> bool {
        self.parent_src_query(query)
    }

    fn propose_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_propose_allocation(query)
    }

    fn decide_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_decide_allocation(query)
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn handle_missing_data(
        &self,
        timestamp: gst::ClockTime,
        duration: Option<gst::ClockTime>,
    ) -> bool {
        self.parent_handle_missing_data(timestamp, duration)
    }
}

pub trait VideoDecoderImplExt: ObjectSubclass {
    fn parent_open(&self) -> Result<(), gst::ErrorMessage>;

    fn parent_close(&self) -> Result<(), gst::ErrorMessage>;

    fn parent_start(&self) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self) -> Result<(), gst::ErrorMessage>;

    fn parent_finish(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_drain(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_set_format(
        &self,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError>;

    fn parent_parse(
        &self,
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_handle_frame(
        &self,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_flush(&self) -> bool;

    fn parent_negotiate(&self) -> Result<(), gst::LoggableError>;

    fn parent_caps(&self, filter: Option<&gst::Caps>) -> gst::Caps;

    fn parent_sink_event(&self, event: gst::Event) -> bool;

    fn parent_sink_query(&self, query: &mut gst::QueryRef) -> bool;

    fn parent_src_event(&self, event: gst::Event) -> bool;

    fn parent_src_query(&self, query: &mut gst::QueryRef) -> bool;

    fn parent_propose_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError>;

    fn parent_decide_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError>;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn parent_handle_missing_data(
        &self,
        timestamp: gst::ClockTime,
        duration: Option<gst::ClockTime>,
    ) -> bool;
}

impl<T: VideoDecoderImpl> VideoDecoderImplExt for T {
    fn parent_open(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .open
                .map(|f| {
                    if from_glib(f(self
                        .obj()
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

    fn parent_close(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .close
                .map(|f| {
                    if from_glib(f(self
                        .obj()
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

    fn parent_start(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(self
                        .obj()
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

    fn parent_stop(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(self
                        .obj()
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

    fn parent_finish(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .finish
                .map(|f| {
                    try_from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoDecoder>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(Ok(gst::FlowSuccess::Ok))
        }
    }

    fn parent_drain(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .drain
                .map(|f| {
                    try_from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoDecoder>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(Ok(gst::FlowSuccess::Ok))
        }
    }

    fn parent_set_format(
        &self,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .set_format
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<VideoDecoder>()
                                .to_glib_none()
                                .0,
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
        frame: &VideoCodecFrame,
        adapter: &gst_base::Adapter,
        at_eos: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .parse
                .map(|f| {
                    try_from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<VideoDecoder>()
                            .to_glib_none()
                            .0,
                        frame.to_glib_none().0,
                        adapter.to_glib_none().0,
                        at_eos.into_glib(),
                    ))
                })
                .unwrap_or(Ok(gst::FlowSuccess::Ok))
        }
    }

    fn parent_handle_frame(
        &self,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    try_from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<VideoDecoder>()
                            .to_glib_none()
                            .0,
                        frame.to_glib_none().0,
                    ))
                })
                .unwrap_or(Err(gst::FlowError::Error))
        }
    }

    fn parent_flush(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .flush
                .map(|f| {
                    from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoDecoder>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(false)
        }
    }

    fn parent_negotiate(&self) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .negotiate
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(self
                            .obj()
                            .unsafe_cast_ref::<VideoDecoder>()
                            .to_glib_none()
                            .0),
                        gst::CAT_RUST,
                        "Parent function `negotiate` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_caps(&self, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .getcaps
                .map(|f| {
                    from_glib_full(f(
                        self.obj()
                            .unsafe_cast_ref::<VideoDecoder>()
                            .to_glib_none()
                            .0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or_else(|| {
                    self.obj()
                        .unsafe_cast_ref::<VideoDecoder>()
                        .proxy_getcaps(None, filter)
                })
        }
    }

    fn parent_sink_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            let f = (*parent_class)
                .sink_event
                .expect("Missing parent function `sink_event`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoDecoder>()
                    .to_glib_none()
                    .0,
                event.into_glib_ptr(),
            ))
        }
    }

    fn parent_sink_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            let f = (*parent_class)
                .sink_query
                .expect("Missing parent function `sink_query`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoDecoder>()
                    .to_glib_none()
                    .0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_src_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            let f = (*parent_class)
                .src_event
                .expect("Missing parent function `src_event`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoDecoder>()
                    .to_glib_none()
                    .0,
                event.into_glib_ptr(),
            ))
        }
    }

    fn parent_src_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            let f = (*parent_class)
                .src_query
                .expect("Missing parent function `src_query`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoDecoder>()
                    .to_glib_none()
                    .0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_propose_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .propose_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<VideoDecoder>()
                                .to_glib_none()
                                .0,
                            query.as_mut_ptr(),
                        ),
                        gst::CAT_RUST,
                        "Parent function `propose_allocation` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_decide_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<VideoDecoder>()
                                .to_glib_none()
                                .0,
                            query.as_mut_ptr(),
                        ),
                        gst::CAT_RUST,
                        "Parent function `decide_allocation` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn parent_handle_missing_data(
        &self,
        timestamp: gst::ClockTime,
        duration: Option<gst::ClockTime>,
    ) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoDecoderClass;
            (*parent_class)
                .handle_missing_data
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<VideoDecoder>()
                            .to_glib_none()
                            .0,
                        timestamp.into_glib(),
                        duration.into_glib(),
                    ))
                })
                .unwrap_or(true)
        }
    }
}

unsafe impl<T: VideoDecoderImpl> IsSubclassable<T> for VideoDecoder {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
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
        #[cfg(any(feature = "v1_20", feature = "dox"))]
        {
            klass.handle_missing_data = Some(video_decoder_handle_missing_data::<T>);
        }
    }
}

unsafe extern "C" fn video_decoder_open<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.open() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_close<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.close() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_start<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.start() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_stop<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.stop() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_finish<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, { imp.finish().into() }).into_glib()
}

unsafe extern "C" fn video_decoder_drain<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, { imp.drain().into() }).into_glib()
}

unsafe extern "C" fn video_decoder_set_format<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    state: *mut ffi::GstVideoCodecState,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    ffi::gst_video_codec_state_ref(state);
    let wrap_state = VideoCodecState::<Readable>::new(state);

    gst::panic_to_error!(imp, false, {
        match imp.set_format(&wrap_state) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_parse<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    frame: *mut ffi::GstVideoCodecFrame,
    adapter: *mut gst_base::ffi::GstAdapter,
    at_eos: glib::ffi::gboolean,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    ffi::gst_video_codec_frame_ref(frame);
    let instance = imp.obj();
    let instance = instance.unsafe_cast_ref::<VideoDecoder>();
    let wrap_frame = VideoCodecFrame::new(frame, instance);
    let wrap_adapter: Borrowed<gst_base::Adapter> = from_glib_borrow(adapter);
    let at_eos: bool = from_glib(at_eos);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.parse(&wrap_frame, &wrap_adapter, at_eos).into()
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_handle_frame<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    frame: *mut ffi::GstVideoCodecFrame,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let instance = imp.obj();
    let instance = instance.unsafe_cast_ref::<VideoDecoder>();
    let wrap_frame = VideoCodecFrame::new(frame, instance);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.handle_frame(wrap_frame).into()
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_flush<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { VideoDecoderImpl::flush(imp) }).into_glib()
}

unsafe extern "C" fn video_decoder_negotiate<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.negotiate() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_getcaps<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::Caps::new_empty(), {
        VideoDecoderImpl::caps(
            imp,
            Option::<gst::Caps>::from_glib_borrow(filter)
                .as_ref()
                .as_ref(),
        )
    })
    .into_glib_ptr()
}

unsafe extern "C" fn video_decoder_sink_event<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.sink_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn video_decoder_sink_query<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        imp.sink_query(gst::QueryRef::from_mut_ptr(query))
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_src_event<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.src_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn video_decoder_src_query<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        imp.src_query(gst::QueryRef::from_mut_ptr(query))
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_propose_allocation<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let query = match gst::QueryRef::from_mut_ptr(query).view_mut() {
        gst::QueryViewMut::Allocation(allocation) => allocation,
        _ => unreachable!(),
    };

    gst::panic_to_error!(imp, false, {
        match imp.propose_allocation(query) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn video_decoder_decide_allocation<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let query = match gst::QueryRef::from_mut_ptr(query).view_mut() {
        gst::QueryViewMut::Allocation(allocation) => allocation,
        _ => unreachable!(),
    };

    gst::panic_to_error!(imp, false, {
        match imp.decide_allocation(query) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
unsafe extern "C" fn video_decoder_handle_missing_data<T: VideoDecoderImpl>(
    ptr: *mut ffi::GstVideoDecoder,
    timestamp: gst::ffi::GstClockTime,
    duration: gst::ffi::GstClockTime,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, true, {
        imp.handle_missing_data(
            Option::<gst::ClockTime>::from_glib(timestamp).unwrap(),
            from_glib(duration),
        )
    })
    .into_glib()
}
