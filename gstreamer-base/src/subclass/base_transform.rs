// Copyright (C) 2017-2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_base_sys;
use gst_sys;

use glib::translate::*;
use prelude::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;

use std::mem;
use std::ptr;

use BaseTransform;
use BaseTransformClass;

pub trait BaseTransformImpl: BaseTransformImplExt + ElementImpl + Send + Sync + 'static {
    fn start(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn transform_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        self.parent_transform_caps(element, direction, caps, filter)
    }

    fn fixate_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps {
        self.parent_fixate_caps(element, direction, caps, othercaps)
    }

    fn set_caps(
        &self,
        element: &BaseTransform,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(element, incaps, outcaps)
    }

    fn accept_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool {
        self.parent_accept_caps(element, direction, caps)
    }

    fn query(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool {
        BaseTransformImplExt::parent_query(self, element, direction, query)
    }

    fn transform_size(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize> {
        self.parent_transform_size(element, direction, caps, size, othercaps)
    }

    fn get_unit_size(&self, element: &BaseTransform, caps: &gst::Caps) -> Option<usize> {
        self.parent_get_unit_size(element, caps)
    }

    fn sink_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        self.parent_sink_event(element, event)
    }

    fn src_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        self.parent_src_event(element, event)
    }

    fn prepare_output_buffer(
        &self,
        element: &BaseTransform,
        inbuf: &gst::BufferRef,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError> {
        self.parent_prepare_output_buffer(element, inbuf)
    }

    fn transform(
        &self,
        element: &BaseTransform,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform(element, inbuf, outbuf)
    }

    fn transform_ip(
        &self,
        element: &BaseTransform,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_ip(element, buf)
    }

    fn transform_ip_passthrough(
        &self,
        element: &BaseTransform,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_ip_passthrough(element, buf)
    }

    fn copy_metadata(
        &self,
        element: &BaseTransform,
        inbuf: &gst::BufferRef,
        outbuf: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_copy_metadata(element, inbuf, outbuf)
    }

    fn transform_meta<'a>(
        &self,
        element: &BaseTransform,
        outbuf: &mut gst::BufferRef,
        meta: gst::MetaRef<'a, gst::Meta>,
        inbuf: &'a gst::BufferRef,
    ) -> bool {
        self.parent_transform_meta(element, outbuf, meta, inbuf)
    }

    fn before_transform(&self, element: &BaseTransform, inbuf: &gst::BufferRef) {
        self.parent_before_transform(element, inbuf);
    }

    fn submit_input_buffer(
        &self,
        element: &BaseTransform,
        is_discont: bool,
        inbuf: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_submit_input_buffer(element, is_discont, inbuf)
    }

    fn generate_output(
        &self,
        element: &BaseTransform,
    ) -> Result<GenerateOutputSuccess, gst::FlowError> {
        self.parent_generate_output(element)
    }
}

pub trait BaseTransformImplExt {
    fn parent_start(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage>;

    fn parent_transform_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps>;

    fn parent_fixate_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps;

    fn parent_set_caps(
        &self,
        element: &BaseTransform,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;

    fn parent_accept_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool;

    fn parent_query(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool;

    fn parent_transform_size(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize>;

    fn parent_get_unit_size(&self, element: &BaseTransform, caps: &gst::Caps) -> Option<usize>;

    fn parent_sink_event(&self, element: &BaseTransform, event: gst::Event) -> bool;

    fn parent_src_event(&self, element: &BaseTransform, event: gst::Event) -> bool;

    fn parent_prepare_output_buffer(
        &self,
        element: &BaseTransform,
        inbuf: &gst::BufferRef,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError>;

    fn parent_transform(
        &self,
        element: &BaseTransform,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_transform_ip(
        &self,
        element: &BaseTransform,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_transform_ip_passthrough(
        &self,
        element: &BaseTransform,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_copy_metadata(
        &self,
        element: &BaseTransform,
        inbuf: &gst::BufferRef,
        outbuf: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError>;

    fn parent_transform_meta<'a>(
        &self,
        element: &BaseTransform,
        outbuf: &mut gst::BufferRef,
        meta: gst::MetaRef<'a, gst::Meta>,
        inbuf: &'a gst::BufferRef,
    ) -> bool;

    fn parent_before_transform(&self, element: &BaseTransform, inbuf: &gst::BufferRef);

    fn parent_submit_input_buffer(
        &self,
        element: &BaseTransform,
        is_discont: bool,
        inbuf: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_generate_output(
        &self,
        element: &BaseTransform,
    ) -> Result<GenerateOutputSuccess, gst::FlowError>;

    fn take_queued_buffer(&self) -> Option<gst::Buffer>
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<BaseTransform>;

    fn get_queued_buffer(&self) -> Option<gst::Buffer>
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<BaseTransform>;
}

impl<T: BaseTransformImpl + ObjectImpl> BaseTransformImplExt for T {
    fn parent_start(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
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

    fn parent_stop(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
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

    fn parent_transform_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .transform_caps
                .map(|f| {
                    from_glib_full(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
                        caps.to_glib_none().0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_fixate_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            match (*parent_class).fixate_caps {
                Some(f) => from_glib_full(f(
                    element.to_glib_none().0,
                    direction.to_glib(),
                    caps.to_glib_none().0,
                    othercaps.into_ptr(),
                )),
                None => othercaps,
            }
        }
    }

    fn parent_set_caps(
        &self,
        element: &BaseTransform,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    gst_result_from_gboolean!(
                        f(
                            element.to_glib_none().0,
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
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .accept_caps
                .map(|f| {
                    from_glib(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
                        caps.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_query(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_transform_size(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .transform_size
                .map(|f| {
                    let mut othersize = mem::MaybeUninit::uninit();
                    let res: bool = from_glib(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
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

    fn parent_get_unit_size(&self, element: &BaseTransform, caps: &gst::Caps) -> Option<usize> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            let f = (*parent_class).get_unit_size.unwrap_or_else(|| {
                if !element.is_in_place() {
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
                element.to_glib_none().0,
                caps.to_glib_none().0,
                size.as_mut_ptr(),
            )) {
                Some(size.assume_init())
            } else {
                None
            }
        }
    }

    fn parent_sink_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .sink_event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(true)
        }
    }

    fn parent_src_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .src_event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(true)
        }
    }

    fn parent_prepare_output_buffer(
        &self,
        element: &BaseTransform,
        inbuf: &gst::BufferRef,
    ) -> Result<PrepareOutputBufferSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .prepare_output_buffer
                .map(|f| {
                    let mut outbuf: *mut gst_sys::GstBuffer = ptr::null_mut();
                    // FIXME: Wrong signature in FFI
                    let res = from_glib(f(
                        element.to_glib_none().0,
                        inbuf.as_ptr() as *mut gst_sys::GstBuffer,
                        (&mut outbuf) as *mut *mut gst_sys::GstBuffer as *mut gst_sys::GstBuffer,
                    ));

                    match gst::FlowReturn::into_result(res) {
                        Err(err) => Err(err),
                        Ok(_) => {
                            if outbuf == inbuf.as_ptr() as *mut _ {
                                Ok(PrepareOutputBufferSuccess::InputBuffer)
                            } else {
                                Ok(PrepareOutputBufferSuccess::Buffer(from_glib_full(outbuf)))
                            }
                        }
                    }
                })
                .unwrap_or(Err(gst::FlowError::NotSupported))
        }
    }

    fn parent_transform(
        &self,
        element: &BaseTransform,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .transform
                .map(|f| {
                    from_glib(f(
                        element.to_glib_none().0,
                        inbuf.to_glib_none().0,
                        outbuf.as_mut_ptr(),
                    ))
                })
                .unwrap_or_else(|| {
                    if !element.is_in_place() {
                        gst::FlowReturn::NotSupported
                    } else {
                        unreachable!(concat!(
                            "parent `transform` called ",
                            "while transform element operates in-place"
                        ));
                    }
                })
                .into_result()
        }
    }

    fn parent_transform_ip(
        &self,
        element: &BaseTransform,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            let f = (*parent_class).transform_ip.unwrap_or_else(|| {
                if element.is_in_place() {
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

            gst::FlowReturn::from_glib(f(element.to_glib_none().0, buf.as_mut_ptr() as *mut _))
                .into_result()
        }
    }

    fn parent_transform_ip_passthrough(
        &self,
        element: &BaseTransform,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            let f = (*parent_class).transform_ip.unwrap_or_else(|| {
                if element.is_in_place() {
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
            let buf: *mut gst_sys::GstBuffer = buf.to_glib_none().0;
            gst::FlowReturn::from_glib(f(element.to_glib_none().0, buf as *mut _)).into_result()
        }
    }

    fn parent_copy_metadata(
        &self,
        element: &BaseTransform,
        inbuf: &gst::BufferRef,
        outbuf: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            if let Some(ref f) = (*parent_class).copy_metadata {
                gst_result_from_gboolean!(
                    f(
                        element.to_glib_none().0,
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
        element: &BaseTransform,
        outbuf: &mut gst::BufferRef,
        meta: gst::MetaRef<'a, gst::Meta>,
        inbuf: &'a gst::BufferRef,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .transform_meta
                .map(|f| {
                    from_glib(f(
                        element.to_glib_none().0,
                        outbuf.as_mut_ptr(),
                        meta.as_ptr() as *mut _,
                        inbuf.as_ptr() as *mut _,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_before_transform(&self, element: &BaseTransform, inbuf: &gst::BufferRef) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            if let Some(ref f) = (*parent_class).before_transform {
                f(element.to_glib_none().0, inbuf.as_ptr() as *mut _);
            }
        }
    }

    fn parent_submit_input_buffer(
        &self,
        element: &BaseTransform,
        is_discont: bool,
        inbuf: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            let f = (*parent_class)
                .submit_input_buffer
                .expect("Missing parent function `submit_input_buffer`");

            gst::FlowReturn::from_glib(f(
                element.to_glib_none().0,
                is_discont.to_glib(),
                inbuf.into_ptr(),
            ))
            .into_result()
        }
    }

    fn parent_generate_output(
        &self,
        element: &BaseTransform,
    ) -> Result<GenerateOutputSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            let f = (*parent_class)
                .generate_output
                .expect("Missing parent function `generate_output`");

            let mut outbuf = ptr::null_mut();
            gst::FlowReturn::from_glib(f(element.to_glib_none().0, &mut outbuf))
                .into_result()
                .map(|res| {
                    if res == ::BASE_TRANSFORM_FLOW_DROPPED {
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
            let element = self.get_instance();
            let ptr: *mut gst_base_sys::GstBaseTransform = element.to_glib_none().0 as *mut _;
            let sinkpad: Borrowed<gst::Pad> = from_glib_borrow((*ptr).sinkpad);
            let _stream_lock = sinkpad.stream_lock();
            let buffer = (*ptr).queued_buf;
            (*ptr).queued_buf = ptr::null_mut();
            from_glib_full(buffer)
        }
    }

    fn get_queued_buffer(&self) -> Option<gst::Buffer>
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<BaseTransform>,
    {
        unsafe {
            let element = self.get_instance();
            let ptr: *mut gst_base_sys::GstBaseTransform = element.to_glib_none().0 as *mut _;
            let sinkpad: Borrowed<gst::Pad> = from_glib_borrow((*ptr).sinkpad);
            let _stream_lock = sinkpad.stream_lock();
            let buffer = (*ptr).queued_buf;
            from_glib_none(buffer)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BaseTransformMode {
    AlwaysInPlace,
    NeverInPlace,
    Both,
}

unsafe impl<T: ObjectSubclass + BaseTransformImpl> IsSubclassable<T> for BaseTransformClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstBaseTransformClass);
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
        }
    }
}

pub unsafe trait BaseTransformClassSubclassExt: Sized + 'static {
    fn configure<T: ObjectSubclass + BaseTransformImpl>(
        &mut self,
        mode: BaseTransformMode,
        passthrough_on_same_caps: bool,
        transform_ip_on_passthrough: bool,
    ) where
        Self: ClassStruct<Type = T>,
        <T as ObjectSubclass>::Instance: PanicPoison,
    {
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstBaseTransformClass);

            klass.passthrough_on_same_caps = passthrough_on_same_caps.to_glib();
            klass.transform_ip_on_passthrough = transform_ip_on_passthrough.to_glib();

            match mode {
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
}

unsafe impl<T: ClassStruct> BaseTransformClassSubclassExt for T
where
    T::Type: ObjectSubclass + BaseTransformImpl,
    <T::Type as ObjectSubclass>::Instance: PanicPoison,
{
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

unsafe extern "C" fn base_transform_start<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

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

unsafe extern "C" fn base_transform_stop<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

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

unsafe extern "C" fn base_transform_transform_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    caps: *mut gst_sys::GstCaps,
    filter: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        let filter: Borrowed<Option<gst::Caps>> = from_glib_borrow(filter);

        imp.transform_caps(
            &wrap,
            from_glib(direction),
            &from_glib_borrow(caps),
            filter.as_ref().as_ref(),
        )
    })
    .map(|caps| caps.into_ptr())
    .unwrap_or(std::ptr::null_mut())
}

unsafe extern "C" fn base_transform_fixate_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    caps: *mut gst_sys::GstCaps,
    othercaps: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        imp.fixate_caps(
            &wrap,
            from_glib(direction),
            &from_glib_borrow(caps),
            from_glib_full(othercaps),
        )
    })
    .into_ptr()
}

unsafe extern "C" fn base_transform_set_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    incaps: *mut gst_sys::GstCaps,
    outcaps: *mut gst_sys::GstCaps,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.set_caps(&wrap, &from_glib_borrow(incaps), &from_glib_borrow(outcaps)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_accept_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    caps: *mut gst_sys::GstCaps,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.accept_caps(&wrap, from_glib(direction), &from_glib_borrow(caps))
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_query<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        BaseTransformImpl::query(
            imp,
            &wrap,
            from_glib(direction),
            gst::QueryRef::from_mut_ptr(query),
        )
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_transform_size<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    caps: *mut gst_sys::GstCaps,
    size: usize,
    othercaps: *mut gst_sys::GstCaps,
    othersize: *mut usize,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.transform_size(
            &wrap,
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
    .to_glib()
}

unsafe extern "C" fn base_transform_get_unit_size<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    caps: *mut gst_sys::GstCaps,
    size: *mut usize,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.get_unit_size(&wrap, &from_glib_borrow(caps)) {
            Some(s) => {
                *size = s;
                true
            }
            None => false,
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_prepare_output_buffer<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    inbuf: *mut gst_sys::GstBuffer,
    outbuf: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    // FIXME: Wrong signature in FFI
    let outbuf = outbuf as *mut *mut gst_sys::GstBuffer;

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match imp.prepare_output_buffer(&wrap, gst::BufferRef::from_ptr(inbuf)) {
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
    .to_glib()
}

unsafe extern "C" fn base_transform_sink_event<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.sink_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_src_event<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.src_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_transform<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    inbuf: *mut gst_sys::GstBuffer,
    outbuf: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.transform(
            &wrap,
            &from_glib_borrow(inbuf),
            gst::BufferRef::from_mut_ptr(outbuf),
        )
        .into()
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_transform_ip<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    buf: *mut *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    // FIXME: Wrong signature in FFI
    let buf = buf as *mut gst_sys::GstBuffer;

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        if from_glib(gst_base_sys::gst_base_transform_is_passthrough(ptr)) {
            imp.transform_ip_passthrough(&wrap, &from_glib_borrow(buf))
                .into()
        } else {
            imp.transform_ip(&wrap, gst::BufferRef::from_mut_ptr(buf))
                .into()
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_transform_meta<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    outbuf: *mut gst_sys::GstBuffer,
    meta: *mut gst_sys::GstMeta,
    inbuf: *mut gst_sys::GstBuffer,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    let inbuf = gst::BufferRef::from_ptr(inbuf);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.transform_meta(
            &wrap,
            gst::BufferRef::from_mut_ptr(outbuf),
            gst::Meta::from_ptr(inbuf, meta),
            inbuf,
        )
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_copy_metadata<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    inbuf: *mut gst_sys::GstBuffer,
    outbuf: *mut gst_sys::GstBuffer,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    if gst_sys::gst_mini_object_is_writable(outbuf as *mut _) == glib_sys::GFALSE {
        gst_warning!(
            gst::CAT_RUST,
            obj: &*wrap,
            "buffer {:?} not writable",
            outbuf
        );
        return glib_sys::GFALSE;
    }

    gst_panic_to_error!(&wrap, &instance.panicked(), true, {
        match imp.copy_metadata(
            &wrap,
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
    .to_glib()
}

unsafe extern "C" fn base_transform_before_transform<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    inbuf: *mut gst_sys::GstBuffer,
) where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.before_transform(&wrap, gst::BufferRef::from_ptr(inbuf));
    })
}

unsafe extern "C" fn base_transform_submit_input_buffer<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    is_discont: glib_sys::gboolean,
    buf: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.submit_input_buffer(&wrap, from_glib(is_discont), from_glib_full(buf))
            .into()
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_generate_output<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    buf: *mut *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseTransform> = from_glib_borrow(ptr);

    *buf = ptr::null_mut();

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        match imp.generate_output(&wrap) {
            Ok(GenerateOutputSuccess::Dropped) => ::BASE_TRANSFORM_FLOW_DROPPED.into(),
            Ok(GenerateOutputSuccess::NoOutput) => gst::FlowReturn::Ok,
            Ok(GenerateOutputSuccess::Buffer(outbuf)) => {
                *buf = outbuf.into_ptr();
                gst::FlowReturn::Ok
            }
            Err(err) => err.into(),
        }
    })
    .to_glib()
}
