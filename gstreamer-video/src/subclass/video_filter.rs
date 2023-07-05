// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use gst_base::{prelude::*, subclass::prelude::*};

use crate::{VideoFilter, VideoFrameRef, VideoInfo};

pub trait VideoFilterImpl: VideoFilterImplExt + BaseTransformImpl {
    fn set_info(
        &self,
        incaps: &gst::Caps,
        in_info: &VideoInfo,
        outcaps: &gst::Caps,
        out_info: &VideoInfo,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_info(incaps, in_info, outcaps, out_info)
    }

    fn transform_frame(
        &self,
        inframe: &VideoFrameRef<&gst::BufferRef>,
        outframe: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_frame(inframe, outframe)
    }

    fn transform_frame_ip(
        &self,
        frame: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_frame_ip(frame)
    }

    fn transform_frame_ip_passthrough(
        &self,
        frame: &VideoFrameRef<&gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_frame_ip_passthrough(frame)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::VideoFilterImplExt> Sealed for T {}
}

pub trait VideoFilterImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_set_info(
        &self,
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
                            self.obj().unsafe_cast_ref::<VideoFilter>().to_glib_none().0,
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
                        self.obj().unsafe_cast_ref::<VideoFilter>().to_glib_none().0,
                        mut_override(inframe.as_ptr()),
                        outframe.as_mut_ptr(),
                    ))
                })
                .unwrap_or_else(|| {
                    if !self
                        .obj()
                        .unsafe_cast_ref::<gst_base::BaseTransform>()
                        .is_in_place()
                    {
                        Err(gst::FlowError::NotSupported)
                    } else {
                        unreachable!(concat!(
                            "parent `transform_frame` called while transform operates in-place"
                        ));
                    }
                })
        }
    }

    fn parent_transform_frame_ip(
        &self,
        frame: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoFilterClass;
            let f = (*parent_class).transform_frame_ip.unwrap_or_else(|| {
                if self
                    .obj()
                    .unsafe_cast_ref::<gst_base::BaseTransform>()
                    .is_in_place()
                {
                    panic!(concat!(
                        "Missing parent function `transform_frame_ip`. Required because ",
                        "transform operates in-place"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform_frame` called while transform doesn't operate in-place"
                    ));
                }
            });

            try_from_glib(f(
                self.obj().unsafe_cast_ref::<VideoFilter>().to_glib_none().0,
                frame.as_mut_ptr(),
            ))
        }
    }

    fn parent_transform_frame_ip_passthrough(
        &self,
        frame: &VideoFrameRef<&gst::BufferRef>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoFilterClass;
            let f = (*parent_class).transform_frame_ip.unwrap_or_else(|| {
                if self
                    .obj()
                    .unsafe_cast_ref::<gst_base::BaseTransform>()
                    .is_in_place()
                {
                    panic!(concat!(
                        "Missing parent function `transform_frame_ip`. Required because ",
                        "transform operates in-place (passthrough mode)"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform_frame_ip` called ",
                        "while transform doesn't operate in-place (passthrough mode)"
                    ));
                }
            });

            try_from_glib(f(
                self.obj().unsafe_cast_ref::<VideoFilter>().to_glib_none().0,
                mut_override(frame.as_ptr()),
            ))
        }
    }
}

impl<T: VideoFilterImpl> VideoFilterImplExt for T {}

unsafe impl<T: VideoFilterImpl> IsSubclassable<T> for VideoFilter {
    fn class_init(klass: &mut glib::Class<Self>) {
        use gst_base::subclass::base_transform::BaseTransformMode;

        Self::parent_class_init::<T>(klass);

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
}

unsafe extern "C" fn video_filter_set_info<T: VideoFilterImpl>(
    ptr: *mut ffi::GstVideoFilter,
    incaps: *mut gst::ffi::GstCaps,
    in_info: *mut ffi::GstVideoInfo,
    outcaps: *mut gst::ffi::GstCaps,
    out_info: *mut ffi::GstVideoInfo,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.set_info(
            &from_glib_borrow(incaps),
            &from_glib_none(in_info),
            &from_glib_borrow(outcaps),
            &from_glib_none(out_info),
        ) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
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
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        imp.transform_frame(
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
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        if from_glib(gst_base::ffi::gst_base_transform_is_passthrough(
            ptr as *mut gst_base::ffi::GstBaseTransform,
        )) {
            imp.transform_frame_ip_passthrough(&VideoFrameRef::from_glib_borrow(frame))
                .into()
        } else {
            imp.transform_frame_ip(&mut VideoFrameRef::from_glib_borrow_mut(frame))
                .into()
        }
    })
    .into_glib()
}
