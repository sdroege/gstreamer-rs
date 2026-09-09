// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::translate::*;
use gst::subclass::prelude::*;

use crate::{BaseParse, BaseParseFrame, BaseParseFrameFlags, ffi, prelude::*};

pub trait BaseParseImpl: ElementImpl + ObjectSubclass<Type: IsA<BaseParse>> {
    // rustdoc-stripper-ignore-next
    /// Whether `Self::detect` will be called or not.
    const USE_DETECT: bool = false;

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_start()
    }

    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_stop()
    }

    fn sink_caps(&self, filter: Option<&gst::Caps>) -> gst::Caps {
        self.parent_sink_caps(filter)
    }

    fn set_sink_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_sink_caps(caps)
    }

    fn handle_frame(
        &self,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
        self.parent_handle_frame(frame)
    }

    fn convert(
        &self,
        src_val: impl gst::format::FormattedValue,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        self.parent_convert(src_val, dest_format)
    }

    fn sink_event(&self, event: gst::Event) -> bool {
        self.parent_sink_event(event)
    }

    fn src_event(&self, event: gst::Event) -> bool {
        self.parent_src_event(event)
    }

    fn sink_query(&self, query: &mut gst::QueryRef) -> bool {
        self.parent_sink_query(query)
    }

    fn src_query(&self, query: &mut gst::QueryRef) -> bool {
        self.parent_src_query(query)
    }

    fn pre_push_frame(
        &self,
        frame: &mut BaseParseFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_pre_push_frame(frame)
    }

    // rustdoc-stripper-ignore-next
    /// Detect the stream format.
    ///
    /// Note that this is only called if `Self::USE_DETECT` is set to `true`.
    fn detect(&self, buffer: &gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_detect(buffer)
    }
}

pub trait BaseParseImplExt: BaseParseImpl {
    fn parent_start(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<BaseParse>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<BaseParse>()
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

    fn parent_sink_caps(&self, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            if let Some(f) = (*parent_class).get_sink_caps {
                from_glib_full(f(
                    self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                    filter.to_glib_none().0,
                ))
            } else {
                let templ_caps = self.obj().sink_pad().pad_template_caps();
                if let Some(filter) = filter {
                    filter.intersect_with_mode(&templ_caps, gst::CapsIntersectMode::First)
                } else {
                    templ_caps
                }
            }
        }
    }

    fn parent_set_sink_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .set_sink_caps
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                            caps.to_glib_none().0,
                        ),
                        gst::CAT_RUST,
                        "Parent function `set_sink_caps` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_handle_frame(
        &self,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            let mut skipsize = 0;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    let res = try_from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                        frame.to_glib_none().0,
                        &mut skipsize,
                    ));
                    (res.unwrap(), skipsize as u32)
                })
                .ok_or(gst::FlowError::Error)
        }
    }

    fn parent_convert(
        &self,
        src_val: impl gst::format::FormattedValue,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            let res = (*parent_class).convert.map(|f| {
                let mut dest_val = mem::MaybeUninit::uninit();

                let res = from_glib(f(
                    self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                    src_val.format().into_glib(),
                    src_val.into_raw_value(),
                    dest_format.into_glib(),
                    dest_val.as_mut_ptr(),
                ));
                (res, dest_val)
            });

            match res {
                Some((true, dest_val)) => Some(gst::GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                )),
                _ => None,
            }
        }
    }

    fn parent_sink_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .sink_event
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_src_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .src_event
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_sink_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .sink_query
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_src_query(&self, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .src_query
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_pre_push_frame(
        &self,
        frame: &mut BaseParseFrame,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            match (*parent_class).pre_push_frame {
                Some(f) => gst::FlowSuccess::try_from_glib(f(
                    self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                    frame.to_glib_none().0,
                )),
                // If there is no parent implementation, replicate the base
                // class behavior: clip the frame.
                None => {
                    frame.set_flags(BaseParseFrameFlags::CLIP);
                    Ok(gst::FlowSuccess::Ok)
                }
            }
        }
    }

    fn parent_detect(&self, buffer: &gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .detect
                .map(|f| {
                    gst::FlowSuccess::try_from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                        buffer.as_ptr() as *mut _,
                    ))
                })
                .unwrap_or(Ok(gst::FlowSuccess::Ok))
        }
    }
}

impl<T: BaseParseImpl> BaseParseImplExt for T {}

unsafe impl<T: BaseParseImpl> IsSubclassable<T> for BaseParse {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.start = Some(base_parse_start::<T>);
        klass.stop = Some(base_parse_stop::<T>);
        klass.set_sink_caps = Some(base_parse_set_sink_caps::<T>);
        klass.get_sink_caps = Some(base_parse_get_sink_caps::<T>);
        klass.handle_frame = Some(base_parse_handle_frame::<T>);
        klass.convert = Some(base_parse_convert::<T>);
        klass.sink_event = Some(base_parse_sink_event::<T>);
        klass.src_event = Some(base_parse_src_event::<T>);
        klass.sink_query = Some(base_parse_sink_query::<T>);
        klass.src_query = Some(base_parse_src_query::<T>);
        klass.pre_push_frame = Some(base_parse_pre_push_frame::<T>);
        if T::USE_DETECT {
            klass.detect = Some(base_parse_detect::<T>);
        }
    }
}

unsafe extern "C" fn base_parse_start<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, false, {
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
}

unsafe extern "C" fn base_parse_stop<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, false, {
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
}

unsafe extern "C" fn base_parse_set_sink_caps<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let caps: Borrowed<gst::Caps> = from_glib_borrow(caps);

        gst::element_panic_to_error!(imp, false, {
            match imp.set_sink_caps(&caps) {
                Ok(()) => true,
                Err(err) => {
                    err.log_with_imp(imp);
                    false
                }
            }
        })
        .into_glib()
    }
}

unsafe extern "C" fn base_parse_get_sink_caps<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let filter: Borrowed<Option<gst::Caps>> = from_glib_borrow(filter);

        gst::element_panic_to_error!(imp, gst::Caps::new_empty(), {
            imp.sink_caps(filter.as_ref().as_ref())
        })
        .into_glib_ptr()
    }
}

unsafe extern "C" fn base_parse_handle_frame<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    frame: *mut ffi::GstBaseParseFrame,
    skipsize: *mut i32,
) -> gst::ffi::GstFlowReturn {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let instance = imp.obj();
        let instance = instance.unsafe_cast_ref::<BaseParse>();
        let wrap_frame = BaseParseFrame::new(frame, instance);

        let res = gst::element_panic_to_error!(imp, Err(gst::FlowError::Error), {
            imp.handle_frame(wrap_frame)
        });

        match res {
            Ok((flow, skip)) => {
                *skipsize = i32::try_from(skip).expect("skip is higher than i32::MAX");
                gst::FlowReturn::from_ok(flow)
            }
            Err(flow) => gst::FlowReturn::from_error(flow),
        }
        .into_glib()
    }
}

unsafe extern "C" fn base_parse_convert<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    source_format: gst::ffi::GstFormat,
    source_value: i64,
    dest_format: gst::ffi::GstFormat,
    dest_value: *mut i64,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let source = gst::GenericFormattedValue::new(from_glib(source_format), source_value);

        let res = gst::element_panic_to_error!(imp, None, {
            imp.convert(source, from_glib(dest_format))
        });

        match res {
            Some(dest) => {
                *dest_value = dest.into_raw_value();
                true
            }
            _ => false,
        }
        .into_glib()
    }
}

unsafe extern "C" fn base_parse_sink_event<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, false, { imp.sink_event(from_glib_full(event)) })
            .into_glib()
    }
}

unsafe extern "C" fn base_parse_src_event<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, false, { imp.src_event(from_glib_full(event)) })
            .into_glib()
    }
}

unsafe extern "C" fn base_parse_sink_query<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    query_ptr: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let query = gst::QueryRef::from_mut_ptr(query_ptr);

        gst::element_panic_to_error!(imp, false, { imp.sink_query(query) }).into_glib()
    }
}

unsafe extern "C" fn base_parse_src_query<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    query_ptr: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let query = gst::QueryRef::from_mut_ptr(query_ptr);

        gst::element_panic_to_error!(imp, false, { imp.src_query(query) }).into_glib()
    }
}

unsafe extern "C" fn base_parse_pre_push_frame<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    frame_ptr: *mut ffi::GstBaseParseFrame,
) -> gst::ffi::GstFlowReturn {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let instance = imp.obj();
        let instance = instance.unsafe_cast_ref::<BaseParse>();
        let mut frame = BaseParseFrame::new(frame_ptr, instance);

        let res = gst::element_panic_to_error!(imp, Err(gst::FlowError::Error), {
            imp.pre_push_frame(&mut frame)
        });

        match res {
            Ok(flow) => gst::FlowReturn::from_ok(flow),
            Err(flow) => gst::FlowReturn::from_error(flow),
        }
        .into_glib()
    }
}

unsafe extern "C" fn base_parse_detect<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    buffer: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();
        let buffer = gst::BufferRef::from_ptr(buffer);

        let res =
            gst::element_panic_to_error!(imp, Err(gst::FlowError::Error), { imp.detect(buffer) });

        match res {
            Ok(flow) => gst::FlowReturn::from_ok(flow),
            Err(flow) => gst::FlowReturn::from_error(flow),
        }
        .into_glib()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{self, AtomicBool, AtomicU32};

    const MIN_FRAME_SIZE: u32 = 8;

    mod imp {
        use super::*;

        fn test_pad_templates() -> &'static [gst::PadTemplate] {
            static PAD_TEMPLATES: std::sync::OnceLock<Vec<gst::PadTemplate>> =
                std::sync::OnceLock::new();

            PAD_TEMPLATES.get_or_init(|| {
                let caps = gst::Caps::new_any();
                vec![
                    gst::PadTemplate::new(
                        "src",
                        gst::PadDirection::Src,
                        gst::PadPresence::Always,
                        &caps,
                    )
                    .unwrap(),
                    gst::PadTemplate::new(
                        "sink",
                        gst::PadDirection::Sink,
                        gst::PadPresence::Always,
                        &caps,
                    )
                    .unwrap(),
                ]
            })
        }

        #[derive(Default)]
        pub struct TestParse {
            sink_caps_set: AtomicBool,
            pub handle_frame_count: AtomicU32,
            pub pre_push_frame_count: AtomicU32,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestParse {
            const NAME: &'static str = "TestParse";
            type Type = super::TestParse;
            type ParentType = crate::BaseParse;
        }

        impl ObjectImpl for TestParse {}

        impl GstObjectImpl for TestParse {}

        impl ElementImpl for TestParse {
            fn pad_templates() -> &'static [gst::PadTemplate] {
                test_pad_templates()
            }
        }

        impl BaseParseImpl for TestParse {
            fn start(&self) -> Result<(), gst::ErrorMessage> {
                self.obj().set_min_frame_size(MIN_FRAME_SIZE);
                self.parent_start()
            }

            fn handle_frame(
                &self,
                frame: BaseParseFrame,
            ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
                self.handle_frame_count
                    .fetch_add(1, atomic::Ordering::SeqCst);
                let instance = self.obj();
                if !self.sink_caps_set.swap(true, atomic::Ordering::SeqCst) {
                    let caps = instance
                        .sink_pad()
                        .current_caps()
                        .expect("sink pad has no caps");
                    instance.src_pad().push_event(gst::event::Caps::new(&caps));
                }

                let flow = instance.finish_frame(frame, MIN_FRAME_SIZE)?;
                Ok((flow, 0))
            }

            fn pre_push_frame(
                &self,
                frame: &mut BaseParseFrame,
            ) -> Result<gst::FlowSuccess, gst::FlowError> {
                self.pre_push_frame_count
                    .fetch_add(1, atomic::Ordering::SeqCst);
                self.parent_pre_push_frame(frame)
            }
        }

        #[derive(Default)]
        pub struct TestParseDetect {
            sink_caps_set: AtomicBool,
            detected: AtomicBool,
            pub detect_count: AtomicU32,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestParseDetect {
            const NAME: &'static str = "TestParseDetect";
            type Type = super::TestParseDetect;
            type ParentType = crate::BaseParse;
        }

        impl ObjectImpl for TestParseDetect {}

        impl GstObjectImpl for TestParseDetect {}

        impl ElementImpl for TestParseDetect {
            fn pad_templates() -> &'static [gst::PadTemplate] {
                test_pad_templates()
            }
        }

        impl BaseParseImpl for TestParseDetect {
            const USE_DETECT: bool = true;

            fn start(&self) -> Result<(), gst::ErrorMessage> {
                self.obj().set_min_frame_size(MIN_FRAME_SIZE);
                self.parent_start()
            }

            fn handle_frame(
                &self,
                frame: BaseParseFrame,
            ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
                let instance = self.obj();
                if !self.sink_caps_set.swap(true, atomic::Ordering::SeqCst) {
                    let caps = instance
                        .sink_pad()
                        .current_caps()
                        .expect("sink pad has no caps");
                    instance.src_pad().push_event(gst::event::Caps::new(&caps));
                }

                let flow = instance.finish_frame(frame, MIN_FRAME_SIZE)?;
                Ok((flow, 0))
            }

            fn detect(&self, _buffer: &gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
                self.detect_count.fetch_add(1, atomic::Ordering::SeqCst);

                // Require two buffers for detection
                if self.detected.swap(true, atomic::Ordering::SeqCst) {
                    Ok(gst::FlowSuccess::Ok)
                } else {
                    Err(gst::FlowError::NotNegotiated)
                }
            }
        }
    }

    glib::wrapper! {
        pub struct TestParse(ObjectSubclass<imp::TestParse>) @extends crate::BaseParse, gst::Element, gst::Object;
    }

    glib::wrapper! {
        pub struct TestParseDetect(ObjectSubclass<imp::TestParseDetect>) @extends crate::BaseParse, gst::Element, gst::Object;
    }

    fn run_pipeline(element: &glib::Object, num_buffers: i32) -> u64 {
        let element = element.downcast_ref::<gst::Element>().unwrap();
        let pipeline = gst::Pipeline::new();
        let src = gst::ElementFactory::make("audiotestsrc")
            .property("num-buffers", num_buffers)
            .build()
            .unwrap();
        let sink = gst::ElementFactory::make("fakesink").build().unwrap();

        pipeline.add_many([&src, element, &sink]).unwrap();
        gst::Element::link_many([&src, element, &sink]).unwrap();

        pipeline.set_state(gst::State::Playing).unwrap();
        let bus = pipeline.bus().unwrap();

        let msg = bus
            .timed_pop_filtered(
                gst::ClockTime::NONE,
                &[gst::MessageType::Eos, gst::MessageType::Error],
            )
            .expect("no EOS or Error message received");
        assert_eq!(
            msg.type_(),
            gst::MessageType::Eos,
            "expected EOS, got {:?}",
            msg
        );

        let rendered = sink
            .property::<gst::Structure>("stats")
            .get::<u64>("rendered")
            .unwrap();
        pipeline.set_state(gst::State::Null).unwrap();

        rendered
    }

    #[test]
    fn test_parse_subclass() {
        gst::init().unwrap();

        let element = glib::Object::new::<TestParse>();
        let rendered = run_pipeline(element.upcast_ref(), 10);

        assert_eq!(rendered, 2560);
        assert!(
            element
                .imp()
                .handle_frame_count
                .load(atomic::Ordering::SeqCst)
                > 0,
            "handle_frame must be invoked"
        );
        assert_eq!(
            element
                .imp()
                .pre_push_frame_count
                .load(atomic::Ordering::SeqCst) as u64,
            rendered,
            "every finished frame must be pushed exactly once"
        );
    }

    // Ignored until this is merged:
    // https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/12459
    #[test]
    #[ignore]
    fn test_parse_subclass_detect() {
        gst::init().unwrap();

        let element = glib::Object::new::<TestParseDetect>();
        let rendered = run_pipeline(element.upcast_ref(), 3);

        assert_eq!(rendered, 768, "expected output frames, got {rendered}");
        assert_eq!(
            element.imp().detect_count.load(atomic::Ordering::SeqCst),
            2,
            "detect is invoked until it reports the stream format"
        );
    }
}
