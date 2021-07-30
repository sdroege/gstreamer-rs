// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use gst_base::prelude::*;
use gst_base::subclass::prelude::*;

use crate::VideoFilter;
use crate::VideoFrameRef;
use crate::VideoInfo;

pub trait VideoFilterImpl: VideoFilterImplExt + BaseTransformImpl {
    fn set_info(
        &self,
        element: &Self::Type,
        incaps: &gst::Caps,
        in_info: &VideoInfo,
        outcaps: &gst::Caps,
        out_info: &VideoInfo,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_info(element, incaps, in_info, outcaps, out_info)
    }

    fn transform_frame(
        &self,
        element: &Self::Type,
        inframe: &VideoFrameRef<&gst::BufferRef>,
        outframe: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_frame(element, inframe, outframe)
    }

    fn transform_frame_ip(
        &self,
        element: &Self::Type,
        frame: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_frame_ip(element, frame)
    }

    fn transform_frame_ip_passthrough(
        &self,
        element: &Self::Type,
        frame: &VideoFrameRef<&gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_frame_ip_passthrough(element, frame)
    }
}

pub trait VideoFilterImplExt: ObjectSubclass {
    fn parent_set_info(
        &self,
        element: &Self::Type,
        incaps: &gst::Caps,
        in_info: &VideoInfo,
        outcaps: &gst::Caps,
        out_info: &VideoInfo,
    ) -> Result<(), gst::LoggableError>;

    fn parent_transform_frame(
        &self,
        element: &Self::Type,
        inframe: &VideoFrameRef<&gst::BufferRef>,
        outframe: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_transform_frame_ip(
        &self,
        element: &Self::Type,
        frame: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_transform_frame_ip_passthrough(
        &self,
        element: &Self::Type,
        frame: &VideoFrameRef<&gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<T: VideoFilterImpl> VideoFilterImplExt for T {
    fn parent_set_info(
        &self,
        element: &Self::Type,
        incaps: &gst::Caps,
        in_info: &VideoInfo,
        outcaps: &gst::Caps,
        out_info: &VideoInfo,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoFilterClass;
            (*parent_class)
                .set_info
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            element.unsafe_cast_ref::<VideoFilter>().to_glib_none().0,
                            incaps.to_glib_none().0,
                            mut_override(in_info.to_glib_none().0),
                            outcaps.to_glib_none().0,
                            mut_override(out_info.to_glib_none().0),
                        ),
                        gst::CAT_RUST,
                        "Parent function `set_info` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_transform_frame(
        &self,
        element: &Self::Type,
        inframe: &VideoFrameRef<&gst::BufferRef>,
        outframe: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoFilterClass;
            (*parent_class)
                .transform_frame
                .map(|f| {
                    try_from_glib(f(
                        element.unsafe_cast_ref::<VideoFilter>().to_glib_none().0,
                        mut_override(inframe.as_ptr()),
                        outframe.as_mut_ptr(),
                    ))
                })
                .unwrap_or_else(|| {
                    if !element
                        .unsafe_cast_ref::<gst_base::BaseTransform>()
                        .is_in_place()
                    {
                        Err(gst::FlowError::NotSupported)
                    } else {
                        unreachable!(concat!(
                            "parent `transform_frame` called ",
                            "while transform element operates in-place"
                        ));
                    }
                })
        }
    }

    fn parent_transform_frame_ip(
        &self,
        element: &Self::Type,
        frame: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoFilterClass;
            let f = (*parent_class).transform_frame_ip.unwrap_or_else(|| {
                if element
                    .unsafe_cast_ref::<gst_base::BaseTransform>()
                    .is_in_place()
                {
                    panic!(concat!(
                        "Missing parent function `transform_frame_ip`. Required because ",
                        "transform element operates in-place"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform_frame` called ",
                        "while transform element doesn't operate in-place"
                    ));
                }
            });

            try_from_glib(f(
                element.unsafe_cast_ref::<VideoFilter>().to_glib_none().0,
                frame.as_mut_ptr(),
            ))
        }
    }

    fn parent_transform_frame_ip_passthrough(
        &self,
        element: &Self::Type,
        frame: &VideoFrameRef<&gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoFilterClass;
            let f = (*parent_class).transform_frame_ip.unwrap_or_else(|| {
                if element
                    .unsafe_cast_ref::<gst_base::BaseTransform>()
                    .is_in_place()
                {
                    panic!(concat!(
                        "Missing parent function `transform_frame_ip`. Required because ",
                        "transform element operates in-place (passthrough mode)"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform_frame_ip` called ",
                        "while transform element doesn't operate in-place (passthrough mode)"
                    ));
                }
            });

            try_from_glib(f(
                element.unsafe_cast_ref::<VideoFilter>().to_glib_none().0,
                mut_override(frame.as_ptr()),
            ))
        }
    }
}

unsafe impl<T: VideoFilterImpl> IsSubclassable<T> for VideoFilter {
    fn class_init(klass: &mut glib::Class<Self>) {
        use gst_base::subclass::base_transform::BaseTransformMode;

        <gst_base::BaseTransform as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.set_info = Some(video_filter_set_info::<T>);

        match T::MODE {
            BaseTransformMode::AlwaysInPlace => {
                klass.transform_frame = None;
                klass.transform_frame_ip = Some(video_filter_transform_frame_ip::<T>);
            }
            BaseTransformMode::NeverInPlace => {
                klass.transform_frame = Some(video_filter_transform_frame::<T>);
                klass.transform_frame_ip = None;
            }
            BaseTransformMode::Both => {
                klass.transform_frame = Some(video_filter_transform_frame::<T>);
                klass.transform_frame_ip = Some(video_filter_transform_frame_ip::<T>);
            }
        }
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gst_base::BaseTransform as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn video_filter_set_info<T: VideoFilterImpl>(
    ptr: *mut ffi::GstVideoFilter,
    incaps: *mut gst::ffi::GstCaps,
    in_info: *mut ffi::GstVideoInfo,
    outcaps: *mut gst::ffi::GstCaps,
    out_info: *mut ffi::GstVideoInfo,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoFilter> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.set_info(
            wrap.unsafe_cast_ref(),
            &from_glib_borrow(incaps),
            &from_glib_none(in_info),
            &from_glib_borrow(outcaps),
            &from_glib_none(out_info),
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

unsafe extern "C" fn video_filter_transform_frame<T: VideoFilterImpl>(
    ptr: *mut ffi::GstVideoFilter,
    inframe: *mut ffi::GstVideoFrame,
    outframe: *mut ffi::GstVideoFrame,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoFilter> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
        imp.transform_frame(
            wrap.unsafe_cast_ref(),
            &VideoFrameRef::from_glib_borrow(inframe),
            &mut VideoFrameRef::from_glib_borrow_mut(outframe),
        )
        .into()
    })
    .into_glib()
}

unsafe extern "C" fn video_filter_transform_frame_ip<T: VideoFilterImpl>(
    ptr: *mut ffi::GstVideoFilter,
    frame: *mut ffi::GstVideoFrame,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<VideoFilter> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), gst::FlowReturn::Error, {
        if from_glib(gst_base::ffi::gst_base_transform_is_passthrough(
            ptr as *mut gst_base::ffi::GstBaseTransform,
        )) {
            imp.transform_frame_ip_passthrough(
                wrap.unsafe_cast_ref(),
                &VideoFrameRef::from_glib_borrow(frame),
            )
            .into()
        } else {
            imp.transform_frame_ip(
                wrap.unsafe_cast_ref(),
                &mut VideoFrameRef::from_glib_borrow_mut(frame),
            )
            .into()
        }
    })
    .into_glib()
}
