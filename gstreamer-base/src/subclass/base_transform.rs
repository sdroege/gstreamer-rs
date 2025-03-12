// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, ptr};

use glib::translate::*;
use gst::subclass::prelude::*;

use crate::{ffi, prelude::*, BaseTransform};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BaseTransformMode {
    AlwaysInPlace,
    NeverInPlace,
    Both,
}

pub trait BaseTransformImpl: ElementImpl + ObjectSubclass<Type: IsA<BaseTransform>> {
    const MODE: BaseTransformMode;
    const PASSTHROUGH_ON_SAME_CAPS: bool;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool;

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_start()
    }

    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_stop()
    }

    fn transform_caps(
        &self,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        self.parent_transform_caps(direction, caps, filter)
    }

    fn fixate_caps(
        &self,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps {
        self.parent_fixate_caps(direction, caps, othercaps)
    }

    fn set_caps(&self, incaps: &gst::Caps, outcaps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(incaps, outcaps)
    }

    fn accept_caps(&self, direction: gst::PadDirection, caps: &gst::Caps) -> bool {
        self.parent_accept_caps(direction, caps)
    }

    fn query(&self, direction: gst::PadDirection, query: &mut gst::QueryRef) -> bool {
        BaseTransformImplExt::parent_query(self, direction, query)
    }

    fn transform_size(
        &self,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize> {
        self.parent_transform_size(direction, caps, size, othercaps)
    }

    fn unit_size(&self, caps: &gst::Caps) -> Option<usize> {
        self.parent_unit_size(caps)
    }

    fn sink_event(&self, event: gst::Event) -> bool {
        self.parent_sink_event(event)
    }

    fn src_event(&self, event: gst::Event) -> bool {
        self.parent_src_event(event)
    }

    fn prepare_output_buffer(
        &self,
        inbuf: InputBuffer,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError> {
        self.parent_prepare_output_buffer(inbuf)
    }

    fn transform(
        &self,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform(inbuf, outbuf)
    }

    fn transform_ip(&self, buf: &mut gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_ip(buf)
    }

    fn transform_ip_passthrough(
        &self,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_ip_passthrough(buf)
    }

    fn propose_allocation(
        &self,
        decide_query: Option<&gst::query::Allocation>,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_propose_allocation(decide_query, query)
    }

    fn decide_allocation(
        &self,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_decide_allocation(query)
    }

    fn copy_metadata(
        &self,
        inbuf: &gst::BufferRef,
        outbuf: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_copy_metadata(inbuf, outbuf)
    }

    fn transform_meta<'a>(
        &self,
        outbuf: &mut gst::BufferRef,
        meta: gst::MetaRef<'a, gst::Meta>,
        inbuf: &'a gst::BufferRef,
    ) -> bool {
        self.parent_transform_meta(outbuf, meta, inbuf)
    }

    fn before_transform(&self, inbuf: &gst::BufferRef) {
        self.parent_before_transform(inbuf);
    }

    fn submit_input_buffer(
        &self,
        is_discont: bool,
        inbuf: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_submit_input_buffer(is_discont, inbuf)
    }

    fn generate_output(&self) -> Result<GenerateOutputSuccess, gst::FlowError> {
        self.parent_generate_output()
    }
}

pub trait BaseTransformImplExt: BaseTransformImpl {
    fn parent_start(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<BaseTransform>()
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<BaseTransform>()
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

    fn parent_transform_caps(
        &self,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .transform_caps
                .map(|f| {
                    from_glib_full(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        direction.into_glib(),
                        caps.to_glib_none().0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_fixate_caps(
        &self,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            match (*parent_class).fixate_caps {
                Some(f) => from_glib_full(f(
                    self.obj()
                        .unsafe_cast_ref::<BaseTransform>()
                        .to_glib_none()
                        .0,
                    direction.into_glib(),
                    caps.to_glib_none().0,
                    othercaps.into_glib_ptr(),
                )),
                None => othercaps,
            }
        }
    }

    fn parent_set_caps(
        &self,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<BaseTransform>()
                                .to_glib_none()
                                .0,
                            incaps.to_glib_none().0,
                            outcaps.to_glib_none().0,
                        ),
                        gst::CAT_RUST,
                        "Parent function `set_caps` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_accept_caps(&self, direction: gst::PadDirection, caps: &gst::Caps) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .accept_caps
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        direction.into_glib(),
                        caps.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_query(&self, direction: gst::PadDirection, query: &mut gst::QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        direction.into_glib(),
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_transform_size(
        &self,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .transform_size
                .map(|f| {
                    let mut othersize = mem::MaybeUninit::uninit();
                    let res: bool = from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        direction.into_glib(),
                        caps.to_glib_none().0,
                        size,
                        othercaps.to_glib_none().0,
                        othersize.as_mut_ptr(),
                    ));
                    if res {
                        Some(othersize.assume_init())
                    } else {
                        None
                    }
                })
                .unwrap_or(None)
        }
    }

    fn parent_unit_size(&self, caps: &gst::Caps) -> Option<usize> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class).get_unit_size.unwrap_or_else(|| {
                if !self.obj().unsafe_cast_ref::<BaseTransform>().is_in_place() {
                    unimplemented!(concat!(
                        "Missing parent function `get_unit_size`. Required because ",
                        "transform doesn't operate in-place"
                    ))
                } else {
                    unreachable!(concat!(
                        "parent `get_unit_size` called while transform operates in-place"
                    ))
                }
            });

            let mut size = mem::MaybeUninit::uninit();
            if from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<BaseTransform>()
                    .to_glib_none()
                    .0,
                caps.to_glib_none().0,
                size.as_mut_ptr(),
            )) {
                Some(size.assume_init())
            } else {
                None
            }
        }
    }

    fn parent_sink_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .sink_event
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_src_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .src_event
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_prepare_output_buffer(
        &self,
        inbuf: InputBuffer,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let buf = match inbuf {
                InputBuffer::Readable(inbuf_r) => inbuf_r.as_ptr(),
                InputBuffer::Writable(inbuf_w) => inbuf_w.as_mut_ptr(),
            };
            (*parent_class)
                .prepare_output_buffer
                .map(|f| {
                    let mut outbuf: *mut gst::ffi::GstBuffer = ptr::null_mut();
                    // FIXME: Wrong signature in FFI
                    gst::FlowSuccess::try_from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        buf as *mut gst::ffi::GstBuffer,
                        (&mut outbuf) as *mut *mut gst::ffi::GstBuffer as *mut gst::ffi::GstBuffer,
                    ))
                    .map(|_| {
                        if outbuf == buf as *mut _ {
                            PrepareOutputBufferSuccess::InputBuffer
                        } else {
                            PrepareOutputBufferSuccess::Buffer(from_glib_full(outbuf))
                        }
                    })
                    .inspect_err(|_err| {
                        if outbuf != buf as *mut _ {
                            drop(Option::<gst::Buffer>::from_glib_full(outbuf));
                        }
                    })
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_transform(
        &self,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .transform
                .map(|f| {
                    try_from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        inbuf.to_glib_none().0,
                        outbuf.as_mut_ptr(),
                    ))
                })
                .unwrap_or_else(|| {
                    if !self.obj().unsafe_cast_ref::<BaseTransform>().is_in_place() {
                        Err(gst::FlowError::NotSupported)
                    } else {
                        unreachable!(concat!(
                            "parent `transform` called while transform operates in-place"
                        ));
                    }
                })
        }
    }

    fn parent_transform_ip(
        &self,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class).transform_ip.unwrap_or_else(|| {
                if self.obj().unsafe_cast_ref::<BaseTransform>().is_in_place() {
                    panic!(concat!(
                        "Missing parent function `transform_ip`. Required because ",
                        "transform operates in-place"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform` called while transform doesn't operate in-place"
                    ));
                }
            });

            try_from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<BaseTransform>()
                    .to_glib_none()
                    .0,
                buf.as_mut_ptr() as *mut _,
            ))
        }
    }

    fn parent_transform_ip_passthrough(
        &self,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class).transform_ip.unwrap_or_else(|| {
                if self.obj().unsafe_cast_ref::<BaseTransform>().is_in_place() {
                    panic!(concat!(
                        "Missing parent function `transform_ip`. Required because ",
                        "transform operates in-place (passthrough mode)"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform_ip` called ",
                        "while transform doesn't operate in-place (passthrough mode)"
                    ));
                }
            });

            // FIXME: Wrong signature in FFI
            let buf: *mut gst::ffi::GstBuffer = buf.to_glib_none().0;
            try_from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<BaseTransform>()
                    .to_glib_none()
                    .0,
                buf as *mut _,
            ))
        }
    }

    fn parent_propose_allocation(
        &self,
        decide_query: Option<&gst::query::Allocation>,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .propose_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<BaseTransform>()
                                .to_glib_none()
                                .0,
                            decide_query
                                .as_ref()
                                .map(|q| q.as_mut_ptr())
                                .unwrap_or(ptr::null_mut()),
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
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<BaseTransform>()
                                .to_glib_none()
                                .0,
                            query.as_mut_ptr(),
                        ),
                        gst::CAT_RUST,
                        "Parent function `decide_allocation` failed,"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_copy_metadata(
        &self,
        inbuf: &gst::BufferRef,
        outbuf: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            if let Some(ref f) = (*parent_class).copy_metadata {
                gst::result_from_gboolean!(
                    f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        inbuf.as_ptr() as *mut _,
                        outbuf.as_mut_ptr()
                    ),
                    gst::CAT_RUST,
                    "Parent function `copy_metadata` failed"
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_transform_meta<'a>(
        &self,
        outbuf: &mut gst::BufferRef,
        meta: gst::MetaRef<'a, gst::Meta>,
        inbuf: &'a gst::BufferRef,
    ) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .transform_meta
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<BaseTransform>()
                            .to_glib_none()
                            .0,
                        outbuf.as_mut_ptr(),
                        meta.as_ptr() as *mut _,
                        inbuf.as_ptr() as *mut _,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_before_transform(&self, inbuf: &gst::BufferRef) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            if let Some(ref f) = (*parent_class).before_transform {
                f(
                    self.obj()
                        .unsafe_cast_ref::<BaseTransform>()
                        .to_glib_none()
                        .0,
                    inbuf.as_ptr() as *mut _,
                );
            }
        }
    }

    fn parent_submit_input_buffer(
        &self,
        is_discont: bool,
        inbuf: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class)
                .submit_input_buffer
                .expect("Missing parent function `submit_input_buffer`");

            try_from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<BaseTransform>()
                    .to_glib_none()
                    .0,
                is_discont.into_glib(),
                inbuf.into_glib_ptr(),
            ))
        }
    }

    fn parent_generate_output(&self) -> Result<GenerateOutputSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class)
                .generate_output
                .expect("Missing parent function `generate_output`");

            let mut outbuf = ptr::null_mut();
            let res = gst::FlowSuccess::try_from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<BaseTransform>()
                    .to_glib_none()
                    .0,
                &mut outbuf,
            ));

            let outbuf = Option::<gst::Buffer>::from_glib_full(outbuf);

            res.map(move |res| match (res, outbuf) {
                (crate::BASE_TRANSFORM_FLOW_DROPPED, _) => GenerateOutputSuccess::Dropped,
                (gst::FlowSuccess::Ok, Some(outbuf)) => GenerateOutputSuccess::Buffer(outbuf),
                _ => GenerateOutputSuccess::NoOutput,
            })
        }
    }

    fn take_queued_buffer(&self) -> Option<gst::Buffer>
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<BaseTransform>,
    {
        unsafe {
            let instance = self.obj();
            let ptr: *mut ffi::GstBaseTransform =
                instance.unsafe_cast_ref::<BaseTransform>().to_glib_none().0;
            let sinkpad: Borrowed<gst::Pad> = from_glib_borrow((*ptr).sinkpad);
            let _stream_lock = sinkpad.stream_lock();
            let buffer = (*ptr).queued_buf;
            (*ptr).queued_buf = ptr::null_mut();
            from_glib_full(buffer)
        }
    }

    fn queued_buffer(&self) -> Option<gst::Buffer>
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<BaseTransform>,
    {
        unsafe {
            let instance = self.obj();
            let ptr: *mut ffi::GstBaseTransform =
                instance.unsafe_cast_ref::<BaseTransform>().to_glib_none().0;
            let sinkpad: Borrowed<gst::Pad> = from_glib_borrow((*ptr).sinkpad);
            let _stream_lock = sinkpad.stream_lock();
            let buffer = (*ptr).queued_buf;
            from_glib_none(buffer)
        }
    }
}

impl<T: BaseTransformImpl> BaseTransformImplExt for T {}

unsafe impl<T: BaseTransformImpl> IsSubclassable<T> for BaseTransform {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.start = Some(base_transform_start::<T>);
        klass.stop = Some(base_transform_stop::<T>);
        klass.transform_caps = Some(base_transform_transform_caps::<T>);
        klass.fixate_caps = Some(base_transform_fixate_caps::<T>);
        klass.set_caps = Some(base_transform_set_caps::<T>);
        klass.accept_caps = Some(base_transform_accept_caps::<T>);
        klass.query = Some(base_transform_query::<T>);
        klass.transform_size = Some(base_transform_transform_size::<T>);
        klass.get_unit_size = Some(base_transform_get_unit_size::<T>);
        klass.prepare_output_buffer = Some(base_transform_prepare_output_buffer::<T>);
        klass.sink_event = Some(base_transform_sink_event::<T>);
        klass.src_event = Some(base_transform_src_event::<T>);
        klass.transform_meta = Some(base_transform_transform_meta::<T>);
        klass.propose_allocation = Some(base_transform_propose_allocation::<T>);
        klass.decide_allocation = Some(base_transform_decide_allocation::<T>);
        klass.copy_metadata = Some(base_transform_copy_metadata::<T>);
        klass.before_transform = Some(base_transform_before_transform::<T>);
        klass.submit_input_buffer = Some(base_transform_submit_input_buffer::<T>);
        klass.generate_output = Some(base_transform_generate_output::<T>);

        klass.passthrough_on_same_caps = T::PASSTHROUGH_ON_SAME_CAPS.into_glib();
        klass.transform_ip_on_passthrough = T::TRANSFORM_IP_ON_PASSTHROUGH.into_glib();

        match T::MODE {
            BaseTransformMode::AlwaysInPlace => {
                klass.transform = None;
                klass.transform_ip = Some(base_transform_transform_ip::<T>);
            }
            BaseTransformMode::NeverInPlace => {
                klass.transform = Some(base_transform_transform::<T>);
                klass.transform_ip = None;
            }
            BaseTransformMode::Both => {
                klass.transform = Some(base_transform_transform::<T>);
                klass.transform_ip = Some(base_transform_transform_ip::<T>);
            }
        }
    }
}

#[derive(Debug)]
pub enum GenerateOutputSuccess {
    Buffer(gst::Buffer),
    NoOutput,
    Dropped,
}

#[derive(Debug)]
pub enum PrepareOutputBufferSuccess {
    Buffer(gst::Buffer),
    InputBuffer,
}

#[derive(Debug)]
pub enum InputBuffer<'a> {
    Writable(&'a mut gst::BufferRef),
    Readable(&'a gst::BufferRef),
}

unsafe extern "C" fn base_transform_start<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
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

unsafe extern "C" fn base_transform_stop<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
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

unsafe extern "C" fn base_transform_transform_caps<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst::ffi::GstPadDirection,
    caps: *mut gst::ffi::GstCaps,
    filter: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, None, {
        let filter: Borrowed<Option<gst::Caps>> = from_glib_borrow(filter);

        imp.transform_caps(
            from_glib(direction),
            &from_glib_borrow(caps),
            filter.as_ref().as_ref(),
        )
    })
    .map(|caps| caps.into_glib_ptr())
    .unwrap_or(std::ptr::null_mut())
}

unsafe extern "C" fn base_transform_fixate_caps<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst::ffi::GstPadDirection,
    caps: *mut gst::ffi::GstCaps,
    othercaps: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::Caps::new_empty(), {
        imp.fixate_caps(
            from_glib(direction),
            &from_glib_borrow(caps),
            from_glib_full(othercaps),
        )
    })
    .into_glib_ptr()
}

unsafe extern "C" fn base_transform_set_caps<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    incaps: *mut gst::ffi::GstCaps,
    outcaps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.set_caps(&from_glib_borrow(incaps), &from_glib_borrow(outcaps)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_accept_caps<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst::ffi::GstPadDirection,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        imp.accept_caps(from_glib(direction), &from_glib_borrow(caps))
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_query<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst::ffi::GstPadDirection,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        BaseTransformImpl::query(
            imp,
            from_glib(direction),
            gst::QueryRef::from_mut_ptr(query),
        )
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_transform_size<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst::ffi::GstPadDirection,
    caps: *mut gst::ffi::GstCaps,
    size: usize,
    othercaps: *mut gst::ffi::GstCaps,
    othersize: *mut usize,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.transform_size(
            from_glib(direction),
            &from_glib_borrow(caps),
            size,
            &from_glib_borrow(othercaps),
        ) {
            Some(s) => {
                *othersize = s;
                true
            }
            None => false,
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_get_unit_size<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    caps: *mut gst::ffi::GstCaps,
    size: *mut usize,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.unit_size(&from_glib_borrow(caps)) {
            Some(s) => {
                *size = s;
                true
            }
            None => false,
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_prepare_output_buffer<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    inbuf: *mut gst::ffi::GstBuffer,
    outbuf: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    // FIXME: Wrong signature in FFI
    let outbuf = outbuf as *mut *mut gst::ffi::GstBuffer;
    let is_passthrough: bool = from_glib(ffi::gst_base_transform_is_passthrough(ptr));
    let is_in_place: bool = from_glib(ffi::gst_base_transform_is_in_place(ptr));
    let writable = is_in_place
        && !is_passthrough
        && gst::ffi::gst_mini_object_is_writable(inbuf as *mut _) != glib::ffi::GFALSE;
    let buffer = match writable {
        false => InputBuffer::Readable(gst::BufferRef::from_ptr(inbuf)),
        true => InputBuffer::Writable(gst::BufferRef::from_mut_ptr(inbuf)),
    };

    *outbuf = ptr::null_mut();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        match imp.prepare_output_buffer(buffer) {
            Ok(PrepareOutputBufferSuccess::InputBuffer) => {
                assert!(
                    is_passthrough || is_in_place,
                    "Returning InputBuffer only allowed for passthrough or in-place mode"
                );
                *outbuf = inbuf;
                gst::FlowReturn::Ok
            }
            Ok(PrepareOutputBufferSuccess::Buffer(buf)) => {
                assert!(
                    !is_passthrough,
                    "Returning Buffer not allowed for passthrough mode"
                );
                *outbuf = buf.into_glib_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => err.into(),
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_sink_event<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.sink_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn base_transform_src_event<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.src_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn base_transform_transform<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    inbuf: *mut gst::ffi::GstBuffer,
    outbuf: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.transform(
            &from_glib_borrow(inbuf),
            gst::BufferRef::from_mut_ptr(outbuf),
        )
        .into()
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_transform_ip<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    buf: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    // FIXME: Wrong signature in FFI
    let buf = buf as *mut gst::ffi::GstBuffer;

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        if from_glib(ffi::gst_base_transform_is_passthrough(ptr)) {
            imp.transform_ip_passthrough(&from_glib_borrow(buf)).into()
        } else {
            imp.transform_ip(gst::BufferRef::from_mut_ptr(buf)).into()
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_transform_meta<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    outbuf: *mut gst::ffi::GstBuffer,
    meta: *mut gst::ffi::GstMeta,
    inbuf: *mut gst::ffi::GstBuffer,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let inbuf = gst::BufferRef::from_ptr(inbuf);

    gst::panic_to_error!(imp, false, {
        imp.transform_meta(
            gst::BufferRef::from_mut_ptr(outbuf),
            gst::Meta::from_ptr(inbuf, meta),
            inbuf,
        )
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_propose_allocation<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    decide_query: *mut gst::ffi::GstQuery,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let decide_query = if decide_query.is_null() {
        None
    } else {
        match gst::QueryRef::from_ptr(decide_query).view() {
            gst::QueryView::Allocation(allocation) => Some(allocation),
            _ => unreachable!(),
        }
    };
    let query = match gst::QueryRef::from_mut_ptr(query).view_mut() {
        gst::QueryViewMut::Allocation(allocation) => allocation,
        _ => unreachable!(),
    };

    gst::panic_to_error!(imp, false, {
        match imp.propose_allocation(decide_query, query) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp_and_level(imp, gst::DebugLevel::Info);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_decide_allocation<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
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

unsafe extern "C" fn base_transform_copy_metadata<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    inbuf: *mut gst::ffi::GstBuffer,
    outbuf: *mut gst::ffi::GstBuffer,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    if gst::ffi::gst_mini_object_is_writable(outbuf as *mut _) == glib::ffi::GFALSE {
        let instance = imp.obj();
        let obj = instance.unsafe_cast_ref::<BaseTransform>();
        gst::warning!(gst::CAT_RUST, obj = obj, "buffer {:?} not writable", outbuf);
        return glib::ffi::GFALSE;
    }

    gst::panic_to_error!(imp, true, {
        match imp.copy_metadata(
            gst::BufferRef::from_ptr(inbuf),
            gst::BufferRef::from_mut_ptr(outbuf),
        ) {
            Ok(_) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_before_transform<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    inbuf: *mut gst::ffi::GstBuffer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, (), {
        imp.before_transform(gst::BufferRef::from_ptr(inbuf));
    })
}

unsafe extern "C" fn base_transform_submit_input_buffer<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    is_discont: glib::ffi::gboolean,
    buf: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.submit_input_buffer(from_glib(is_discont), from_glib_full(buf))
            .into()
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_generate_output<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    buf: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    *buf = ptr::null_mut();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        match imp.generate_output() {
            Ok(GenerateOutputSuccess::Dropped) => crate::BASE_TRANSFORM_FLOW_DROPPED.into(),
            Ok(GenerateOutputSuccess::NoOutput) => gst::FlowReturn::Ok,
            Ok(GenerateOutputSuccess::Buffer(outbuf)) => {
                *buf = outbuf.into_glib_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => err.into(),
        }
    })
    .into_glib()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub mod imp {
        use super::*;
        use std::sync::atomic::{self, AtomicBool};

        #[derive(Default)]
        pub struct TestTransform {
            drop_next: AtomicBool,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestTransform {
            const NAME: &'static str = "TestTransform";
            type Type = super::TestTransform;
            type ParentType = crate::BaseTransform;
        }

        impl ObjectImpl for TestTransform {}

        impl GstObjectImpl for TestTransform {}

        impl ElementImpl for TestTransform {
            fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
                static ELEMENT_METADATA: std::sync::OnceLock<gst::subclass::ElementMetadata> =
                    std::sync::OnceLock::new();

                Some(ELEMENT_METADATA.get_or_init(|| {
                    gst::subclass::ElementMetadata::new(
                        "Test Transform",
                        "Generic",
                        "Does nothing",
                        "Sebastian Dröge <sebastian@centricular.com>",
                    )
                }))
            }

            fn pad_templates() -> &'static [gst::PadTemplate] {
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
        }

        impl BaseTransformImpl for TestTransform {
            const MODE: BaseTransformMode = BaseTransformMode::AlwaysInPlace;

            const PASSTHROUGH_ON_SAME_CAPS: bool = false;

            const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;

            fn transform_ip(
                &self,
                _buf: &mut gst::BufferRef,
            ) -> Result<gst::FlowSuccess, gst::FlowError> {
                if self.drop_next.load(atomic::Ordering::SeqCst) {
                    self.drop_next.store(false, atomic::Ordering::SeqCst);
                    Ok(crate::BASE_TRANSFORM_FLOW_DROPPED)
                } else {
                    self.drop_next.store(true, atomic::Ordering::SeqCst);
                    Ok(gst::FlowSuccess::Ok)
                }
            }
        }
    }

    glib::wrapper! {
        pub struct TestTransform(ObjectSubclass<imp::TestTransform>) @extends crate::BaseTransform, gst::Element, gst::Object;
    }

    impl TestTransform {
        pub fn new(name: Option<&str>) -> Self {
            glib::Object::builder().property("name", name).build()
        }
    }

    #[test]
    fn test_transform_subclass() {
        gst::init().unwrap();

        let element = TestTransform::new(Some("test"));

        assert_eq!(element.name(), "test");

        let pipeline = gst::Pipeline::new();
        let src = gst::ElementFactory::make("audiotestsrc")
            .property("num-buffers", 100i32)
            .build()
            .unwrap();
        let sink = gst::ElementFactory::make("fakesink").build().unwrap();

        pipeline
            .add_many([&src, element.upcast_ref(), &sink])
            .unwrap();
        gst::Element::link_many([&src, element.upcast_ref(), &sink]).unwrap();

        pipeline.set_state(gst::State::Playing).unwrap();
        let bus = pipeline.bus().unwrap();

        let eos = bus.timed_pop_filtered(gst::ClockTime::NONE, &[gst::MessageType::Eos]);
        assert!(eos.is_some());

        let stats = sink.property::<gst::Structure>("stats");
        assert_eq!(stats.get::<u64>("rendered").unwrap(), 50);

        pipeline.set_state(gst::State::Null).unwrap();
    }
}
