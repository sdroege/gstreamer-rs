// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;

use glib::translate::*;

use gst::gst_warning;
use gst::subclass::prelude::*;

use std::mem;
use std::ptr;

use crate::BaseTransform;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BaseTransformMode {
    AlwaysInPlace,
    NeverInPlace,
    Both,
}

pub trait BaseTransformImpl: BaseTransformImplExt + ElementImpl {
    const MODE: BaseTransformMode;
    const PASSTHROUGH_ON_SAME_CAPS: bool;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool;

    fn start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn transform_caps(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        self.parent_transform_caps(element, direction, caps, filter)
    }

    fn fixate_caps(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps {
        self.parent_fixate_caps(element, direction, caps, othercaps)
    }

    fn set_caps(
        &self,
        element: &Self::Type,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(element, incaps, outcaps)
    }

    fn accept_caps(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool {
        self.parent_accept_caps(element, direction, caps)
    }

    fn query(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool {
        BaseTransformImplExt::parent_query(self, element, direction, query)
    }

    fn transform_size(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize> {
        self.parent_transform_size(element, direction, caps, size, othercaps)
    }

    fn unit_size(&self, element: &Self::Type, caps: &gst::Caps) -> Option<usize> {
        self.parent_unit_size(element, caps)
    }

    fn sink_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        self.parent_sink_event(element, event)
    }

    fn src_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        self.parent_src_event(element, event)
    }

    fn prepare_output_buffer(
        &self,
        element: &Self::Type,
        inbuf: &gst::BufferRef,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError> {
        self.parent_prepare_output_buffer(element, inbuf)
    }

    fn transform(
        &self,
        element: &Self::Type,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform(element, inbuf, outbuf)
    }

    fn transform_ip(
        &self,
        element: &Self::Type,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_ip(element, buf)
    }

    fn transform_ip_passthrough(
        &self,
        element: &Self::Type,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_ip_passthrough(element, buf)
    }

    fn copy_metadata(
        &self,
        element: &Self::Type,
        inbuf: &gst::BufferRef,
        outbuf: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_copy_metadata(element, inbuf, outbuf)
    }

    fn transform_meta<'a>(
        &self,
        element: &Self::Type,
        outbuf: &mut gst::BufferRef,
        meta: gst::MetaRef<'a, gst::Meta>,
        inbuf: &'a gst::BufferRef,
    ) -> bool {
        self.parent_transform_meta(element, outbuf, meta, inbuf)
    }

    fn before_transform(&self, element: &Self::Type, inbuf: &gst::BufferRef) {
        self.parent_before_transform(element, inbuf);
    }

    fn submit_input_buffer(
        &self,
        element: &Self::Type,
        is_discont: bool,
        inbuf: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_submit_input_buffer(element, is_discont, inbuf)
    }

    fn generate_output(
        &self,
        element: &Self::Type,
    ) -> Result<GenerateOutputSuccess, gst::FlowError> {
        self.parent_generate_output(element)
    }
}

pub trait BaseTransformImplExt: ObjectSubclass {
    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_transform_caps(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps>;

    fn parent_fixate_caps(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps;

    fn parent_set_caps(
        &self,
        element: &Self::Type,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;

    fn parent_accept_caps(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool;

    fn parent_query(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool;

    fn parent_transform_size(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize>;

    fn parent_unit_size(&self, element: &Self::Type, caps: &gst::Caps) -> Option<usize>;

    fn parent_sink_event(&self, element: &Self::Type, event: gst::Event) -> bool;

    fn parent_src_event(&self, element: &Self::Type, event: gst::Event) -> bool;

    fn parent_prepare_output_buffer(
        &self,
        element: &Self::Type,
        inbuf: &gst::BufferRef,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError>;

    fn parent_transform(
        &self,
        element: &Self::Type,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_transform_ip(
        &self,
        element: &Self::Type,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_transform_ip_passthrough(
        &self,
        element: &Self::Type,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_copy_metadata(
        &self,
        element: &Self::Type,
        inbuf: &gst::BufferRef,
        outbuf: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError>;

    fn parent_transform_meta<'a>(
        &self,
        element: &Self::Type,
        outbuf: &mut gst::BufferRef,
        meta: gst::MetaRef<'a, gst::Meta>,
        inbuf: &'a gst::BufferRef,
    ) -> bool;

    fn parent_before_transform(&self, element: &Self::Type, inbuf: &gst::BufferRef);

    fn parent_submit_input_buffer(
        &self,
        element: &Self::Type,
        is_discont: bool,
        inbuf: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_generate_output(
        &self,
        element: &Self::Type,
    ) -> Result<GenerateOutputSuccess, gst::FlowError>;

    fn take_queued_buffer(&self) -> Option<gst::Buffer>
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<BaseTransform>;

    #[doc(alias = "get_queued_buffer")]
    fn queued_buffer(&self) -> Option<gst::Buffer>
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<BaseTransform>;
}

impl<T: BaseTransformImpl> BaseTransformImplExt for T {
    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element
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

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element
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
        element: &Self::Type,
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
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
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
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            match (*parent_class).fixate_caps {
                Some(f) => from_glib_full(f(
                    element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                    direction.into_glib(),
                    caps.to_glib_none().0,
                    othercaps.into_ptr(),
                )),
                None => othercaps,
            }
        }
    }

    fn parent_set_caps(
        &self,
        element: &Self::Type,
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
                            element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
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

    fn parent_accept_caps(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .accept_caps
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                        direction.into_glib(),
                        caps.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_query(
        &self,
        element: &Self::Type,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                        direction.into_glib(),
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_transform_size(
        &self,
        element: &Self::Type,
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
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
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

    fn parent_unit_size(&self, element: &Self::Type, caps: &gst::Caps) -> Option<usize> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class).get_unit_size.unwrap_or_else(|| {
                if !element.unsafe_cast_ref::<BaseTransform>().is_in_place() {
                    unimplemented!(concat!(
                        "Missing parent function `get_unit_size`. Required because ",
                        "transform element doesn't operate in-place"
                    ))
                } else {
                    unreachable!(concat!(
                        "parent `get_unit_size` called ",
                        "while transform element operates in-place"
                    ))
                }
            });

            let mut size = mem::MaybeUninit::uninit();
            if from_glib(f(
                element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                caps.to_glib_none().0,
                size.as_mut_ptr(),
            )) {
                Some(size.assume_init())
            } else {
                None
            }
        }
    }

    fn parent_sink_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .sink_event
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                        event.into_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_src_event(&self, element: &Self::Type, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .src_event
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                        event.into_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_prepare_output_buffer(
        &self,
        element: &Self::Type,
        inbuf: &gst::BufferRef,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .prepare_output_buffer
                .map(|f| {
                    let mut outbuf: *mut gst::ffi::GstBuffer = ptr::null_mut();
                    // FIXME: Wrong signature in FFI
                    gst::FlowSuccess::try_from_glib(f(
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                        inbuf.as_ptr() as *mut gst::ffi::GstBuffer,
                        (&mut outbuf) as *mut *mut gst::ffi::GstBuffer as *mut gst::ffi::GstBuffer,
                    ))
                    .map(|_| {
                        if outbuf == inbuf.as_ptr() as *mut _ {
                            PrepareOutputBufferSuccess::InputBuffer
                        } else {
                            PrepareOutputBufferSuccess::Buffer(from_glib_full(outbuf))
                        }
                    })
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_transform(
        &self,
        element: &Self::Type,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .transform
                .map(|f| {
                    gst::FlowSuccess::try_from_glib(f(
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                        inbuf.to_glib_none().0,
                        outbuf.as_mut_ptr(),
                    ))
                })
                .unwrap_or_else(|| {
                    if !element.unsafe_cast_ref::<BaseTransform>().is_in_place() {
                        Err(gst::FlowError::NotSupported)
                    } else {
                        unreachable!(concat!(
                            "parent `transform` called ",
                            "while transform element operates in-place"
                        ));
                    }
                })
        }
    }

    fn parent_transform_ip(
        &self,
        element: &Self::Type,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class).transform_ip.unwrap_or_else(|| {
                if element.unsafe_cast_ref::<BaseTransform>().is_in_place() {
                    panic!(concat!(
                        "Missing parent function `transform_ip`. Required because ",
                        "transform element operates in-place"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform` called ",
                        "while transform element doesn't operate in-place"
                    ));
                }
            });

            gst::FlowSuccess::try_from_glib(f(
                element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                buf.as_mut_ptr() as *mut _,
            ))
        }
    }

    fn parent_transform_ip_passthrough(
        &self,
        element: &Self::Type,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class).transform_ip.unwrap_or_else(|| {
                if element.unsafe_cast_ref::<BaseTransform>().is_in_place() {
                    panic!(concat!(
                        "Missing parent function `transform_ip`. Required because ",
                        "transform element operates in-place (passthrough mode)"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform_ip` called ",
                        "while transform element doesn't operate in-place (passthrough mode)"
                    ));
                }
            });

            // FIXME: Wrong signature in FFI
            let buf: *mut gst::ffi::GstBuffer = buf.to_glib_none().0;
            gst::FlowSuccess::try_from_glib(f(
                element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                buf as *mut _,
            ))
        }
    }

    fn parent_copy_metadata(
        &self,
        element: &Self::Type,
        inbuf: &gst::BufferRef,
        outbuf: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            if let Some(ref f) = (*parent_class).copy_metadata {
                gst::result_from_gboolean!(
                    f(
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
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
        element: &Self::Type,
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
                        element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                        outbuf.as_mut_ptr(),
                        meta.as_ptr() as *mut _,
                        inbuf.as_ptr() as *mut _,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_before_transform(&self, element: &Self::Type, inbuf: &gst::BufferRef) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            if let Some(ref f) = (*parent_class).before_transform {
                f(
                    element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                    inbuf.as_ptr() as *mut _,
                );
            }
        }
    }

    fn parent_submit_input_buffer(
        &self,
        element: &Self::Type,
        is_discont: bool,
        inbuf: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class)
                .submit_input_buffer
                .expect("Missing parent function `submit_input_buffer`");

            gst::FlowSuccess::try_from_glib(f(
                element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                is_discont.into_glib(),
                inbuf.into_ptr(),
            ))
        }
    }

    fn parent_generate_output(
        &self,
        element: &Self::Type,
    ) -> Result<GenerateOutputSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseTransformClass;
            let f = (*parent_class)
                .generate_output
                .expect("Missing parent function `generate_output`");

            let mut outbuf = ptr::null_mut();
            gst::FlowSuccess::try_from_glib(f(
                element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0,
                &mut outbuf,
            ))
            .map(|res| {
                if res == crate::BASE_TRANSFORM_FLOW_DROPPED {
                    GenerateOutputSuccess::Dropped
                } else if res != gst::FlowSuccess::Ok || outbuf.is_null() {
                    GenerateOutputSuccess::NoOutput
                } else {
                    GenerateOutputSuccess::Buffer(from_glib_full(outbuf))
                }
            })
        }
    }

    fn take_queued_buffer(&self) -> Option<gst::Buffer>
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<BaseTransform>,
    {
        unsafe {
            let element = self.instance();
            let ptr: *mut ffi::GstBaseTransform =
                element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0;
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
            let element = self.instance();
            let ptr: *mut ffi::GstBaseTransform =
                element.unsafe_cast_ref::<BaseTransform>().to_glib_none().0;
            let sinkpad: Borrowed<gst::Pad> = from_glib_borrow((*ptr).sinkpad);
            let _stream_lock = sinkpad.stream_lock();
            let buffer = (*ptr).queued_buf;
            from_glib_none(buffer)
        }
    }
}

unsafe impl<T: BaseTransformImpl> IsSubclassable<T> for BaseTransform {
    fn class_init(klass: &mut glib::Class<Self>) {
        <gst::Element as IsSubclassable<T>>::class_init(klass);
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

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gst::Element as IsSubclassable<T>>::instance_init(instance);
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

unsafe extern "C" fn base_transform_start<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.start(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.stop(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), None, {
        let filter: Borrowed<Option<gst::Caps>> = from_glib_borrow(filter);

        imp.transform_caps(
            wrap.unsafe_cast_ref(),
            from_glib(direction),
            &from_glib_borrow(caps),
            filter.as_ref().as_ref(),
        )
    })
    .map(|caps| caps.into_ptr())
    .unwrap_or(std::ptr::null_mut())
}

unsafe extern "C" fn base_transform_fixate_caps<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst::ffi::GstPadDirection,
    caps: *mut gst::ffi::GstCaps,
    othercaps: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::Caps::new_empty(), {
        imp.fixate_caps(
            wrap.unsafe_cast_ref(),
            from_glib(direction),
            &from_glib_borrow(caps),
            from_glib_full(othercaps),
        )
    })
    .into_ptr()
}

unsafe extern "C" fn base_transform_set_caps<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    incaps: *mut gst::ffi::GstCaps,
    outcaps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.set_caps(
            wrap.unsafe_cast_ref(),
            &from_glib_borrow(incaps),
            &from_glib_borrow(outcaps),
        ) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.accept_caps(
            wrap.unsafe_cast_ref(),
            from_glib(direction),
            &from_glib_borrow(caps),
        )
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_query<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst::ffi::GstPadDirection,
    query: *mut gst::ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        BaseTransformImpl::query(
            imp,
            wrap.unsafe_cast_ref(),
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.transform_size(
            wrap.unsafe_cast_ref(),
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.unit_size(wrap.unsafe_cast_ref(), &from_glib_borrow(caps)) {
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    // FIXME: Wrong signature in FFI
    let outbuf = outbuf as *mut *mut gst::ffi::GstBuffer;

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        match imp.prepare_output_buffer(wrap.unsafe_cast_ref(), gst::BufferRef::from_ptr(inbuf)) {
            Ok(PrepareOutputBufferSuccess::InputBuffer) => {
                *outbuf = inbuf;
                gst::FlowReturn::Ok
            }
            Ok(PrepareOutputBufferSuccess::Buffer(buf)) => {
                *outbuf = buf.into_ptr();
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.sink_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_src_event<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.src_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_transform<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    inbuf: *mut gst::ffi::GstBuffer,
    outbuf: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        imp.transform(
            wrap.unsafe_cast_ref(),
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    // FIXME: Wrong signature in FFI
    let buf = buf as *mut gst::ffi::GstBuffer;

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        if from_glib(ffi::gst_base_transform_is_passthrough(ptr)) {
            imp.transform_ip_passthrough(wrap.unsafe_cast_ref(), &from_glib_borrow(buf))
                .into()
        } else {
            imp.transform_ip(wrap.unsafe_cast_ref(), gst::BufferRef::from_mut_ptr(buf))
                .into()
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    let inbuf = gst::BufferRef::from_ptr(inbuf);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        imp.transform_meta(
            wrap.unsafe_cast_ref(),
            gst::BufferRef::from_mut_ptr(outbuf),
            gst::Meta::from_ptr(inbuf, meta),
            inbuf,
        )
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_copy_metadata<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    inbuf: *mut gst::ffi::GstBuffer,
    outbuf: *mut gst::ffi::GstBuffer,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    if gst::ffi::gst_mini_object_is_writable(outbuf as *mut _) == glib::ffi::GFALSE {
        gst_warning!(
            gst::CAT_RUST,
            obj: &*wrap,
            "buffer {:?} not writable",
            outbuf
        );
        return glib::ffi::GFALSE;
    }

    gst::panic_to_error!(&wrap, &imp.panicked(), true, {
        match imp.copy_metadata(
            wrap.unsafe_cast_ref(),
            gst::BufferRef::from_ptr(inbuf),
            gst::BufferRef::from_mut_ptr(outbuf),
        ) {
            Ok(_) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
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
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), (), {
        imp.before_transform(wrap.unsafe_cast_ref(), gst::BufferRef::from_ptr(inbuf));
    })
}

unsafe extern "C" fn base_transform_submit_input_buffer<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    is_discont: glib::ffi::gboolean,
    buf: *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        imp.submit_input_buffer(
            wrap.unsafe_cast_ref(),
            from_glib(is_discont),
            from_glib_full(buf),
        )
        .into()
    })
    .into_glib()
}

unsafe extern "C" fn base_transform_generate_output<T: BaseTransformImpl>(
    ptr: *mut ffi::GstBaseTransform,
    buf: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    *buf = ptr::null_mut();

    gst::panic_to_error!(&wrap, &imp.panicked(), gst::FlowReturn::Error, {
        match imp.generate_output(wrap.unsafe_cast_ref()) {
            Ok(GenerateOutputSuccess::Dropped) => crate::BASE_TRANSFORM_FLOW_DROPPED.into(),
            Ok(GenerateOutputSuccess::NoOutput) => gst::FlowReturn::Ok,
            Ok(GenerateOutputSuccess::Buffer(outbuf)) => {
                *buf = outbuf.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => err.into(),
        }
    })
    .into_glib()
}
