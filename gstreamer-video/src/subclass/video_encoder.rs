// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use gst::subclass::prelude::*;

use crate::{
    prelude::*,
    video_codec_state::{Readable, VideoCodecState},
    VideoCodecFrame, VideoEncoder,
};

pub trait VideoEncoderImpl: VideoEncoderImplExt + ElementImpl {
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

    fn set_format(
        &self,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_format(state)
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
}

pub trait VideoEncoderImplExt: ObjectSubclass {
    fn parent_open(&self) -> Result<(), gst::ErrorMessage>;

    fn parent_close(&self) -> Result<(), gst::ErrorMessage>;

    fn parent_start(&self) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self) -> Result<(), gst::ErrorMessage>;

    fn parent_finish(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_set_format(
        &self,
        state: &VideoCodecState<'static, Readable>,
    ) -> Result<(), gst::LoggableError>;

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
}

impl<T: VideoEncoderImpl> VideoEncoderImplExt for T {
    fn parent_open(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .open
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .close
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .finish
                .map(|f| {
                    try_from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .set_format
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<VideoEncoder>()
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

    fn parent_handle_frame(
        &self,
        frame: VideoCodecFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    try_from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .flush
                .map(|f| {
                    from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<VideoEncoder>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(false)
        }
    }

    fn parent_negotiate(&self) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .negotiate
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(self
                            .obj()
                            .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .getcaps
                .map(|f| {
                    from_glib_full(f(
                        self.obj()
                            .unsafe_cast_ref::<VideoEncoder>()
                            .to_glib_none()
                            .0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or_else(|| {
                    self.obj()
                        .unsafe_cast_ref::<VideoEncoder>()
                        .proxy_getcaps(None, filter)
                })
        }
    }

    fn parent_sink_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            let f = (*parent_class)
                .sink_event
                .expect("Missing parent function `sink_event`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoEncoder>()
                    .to_glib_none()
                    .0,
                event.into_glib_ptr(),
            ))
        }
    }

    fn parent_sink_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            let f = (*parent_class)
                .sink_query
                .expect("Missing parent function `sink_query`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoEncoder>()
                    .to_glib_none()
                    .0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn parent_src_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            let f = (*parent_class)
                .src_event
                .expect("Missing parent function `src_event`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoEncoder>()
                    .to_glib_none()
                    .0,
                event.into_glib_ptr(),
            ))
        }
    }

    fn parent_src_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            let f = (*parent_class)
                .src_query
                .expect("Missing parent function `src_query`");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .propose_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<VideoEncoder>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoEncoderClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<VideoEncoder>()
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
}

unsafe impl<T: VideoEncoderImpl> IsSubclassable<T> for VideoEncoder {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.open = Some(video_encoder_open::<T>);
        klass.close = Some(video_encoder_close::<T>);
        klass.start = Some(video_encoder_start::<T>);
        klass.stop = Some(video_encoder_stop::<T>);
        klass.finish = Some(video_encoder_finish::<T>);
        klass.set_format = Some(video_encoder_set_format::<T>);
        klass.handle_frame = Some(video_encoder_handle_frame::<T>);
        klass.flush = Some(video_encoder_flush::<T>);
        klass.negotiate = Some(video_encoder_negotiate::<T>);
        klass.getcaps = Some(video_encoder_getcaps::<T>);
        klass.sink_event = Some(video_encoder_sink_event::<T>);
        klass.src_event = Some(video_encoder_src_event::<T>);
        klass.sink_query = Some(video_encoder_sink_query::<T>);
        klass.src_query = Some(video_encoder_src_query::<T>);
        klass.propose_allocation = Some(video_encoder_propose_allocation::<T>);
        klass.decide_allocation = Some(video_encoder_decide_allocation::<T>);
    }
}

unsafe extern "C" fn video_encoder_open<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
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

unsafe extern "C" fn video_encoder_close<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
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

unsafe extern "C" fn video_encoder_start<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
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

unsafe extern "C" fn video_encoder_stop<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
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

unsafe extern "C" fn video_encoder_finish<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, { imp.finish().into() }).into_glib()
}

unsafe extern "C" fn video_encoder_set_format<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
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

unsafe extern "C" fn video_encoder_handle_frame<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
    frame: *mut ffi::GstVideoCodecFrame,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let instance = imp.obj();
    let instance = instance.unsafe_cast_ref::<VideoEncoder>();
    let wrap_frame = VideoCodecFrame::new(frame, instance);

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.handle_frame(wrap_frame).into()
    })
    .into_glib()
}

unsafe extern "C" fn video_encoder_flush<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { VideoEncoderImpl::flush(imp) }).into_glib()
}

unsafe extern "C" fn video_encoder_negotiate<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
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

unsafe extern "C" fn video_encoder_getcaps<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::Caps::new_empty(), {
        VideoEncoderImpl::caps(
            imp,
            Option::<gst::Caps>::from_glib_borrow(filter)
                .as_ref()
                .as_ref(),
        )
    })
    .to_glib_full()
}

unsafe extern "C" fn video_encoder_sink_event<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.sink_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn video_encoder_sink_query<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        imp.sink_query(gst::QueryRef::from_mut_ptr(query))
    })
    .into_glib()
}

unsafe extern "C" fn video_encoder_src_event<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.src_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn video_encoder_src_query<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        imp.src_query(gst::QueryRef::from_mut_ptr(query))
    })
    .into_glib()
}

unsafe extern "C" fn video_encoder_propose_allocation<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
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

unsafe extern "C" fn video_encoder_decide_allocation<T: VideoEncoderImpl>(
    ptr: *mut ffi::GstVideoEncoder,
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
