// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use gst_base::prelude::*;
use gst_base::subclass::prelude::*;

use std::mem;
use std::ptr;

use crate::VideoAggregator;

pub struct AggregateFramesToken<'a>(pub(crate) &'a VideoAggregator);

pub trait VideoAggregatorImpl: VideoAggregatorImplExt + AggregatorImpl {
    fn update_caps(&self, caps: &gst::Caps) -> Result<gst::Caps, gst::LoggableError> {
        self.parent_update_caps(caps)
    }

    fn aggregate_frames(
        &self,
        token: &AggregateFramesToken,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_aggregate_frames(token, outbuf)
    }

    fn create_output_buffer(&self) -> Result<Option<gst::Buffer>, gst::FlowError> {
        self.parent_create_output_buffer()
    }

    fn find_best_format(&self, downstream_caps: &gst::Caps) -> Option<(crate::VideoInfo, bool)> {
        self.parent_find_best_format(downstream_caps)
    }
}

pub trait VideoAggregatorImplExt: ObjectSubclass {
    fn parent_update_caps(&self, caps: &gst::Caps) -> Result<gst::Caps, gst::LoggableError>;

    fn parent_aggregate_frames(
        &self,
        token: &AggregateFramesToken,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_create_output_buffer(&self) -> Result<Option<gst::Buffer>, gst::FlowError>;

    fn parent_find_best_format(
        &self,
        downstream_caps: &gst::Caps,
    ) -> Option<(crate::VideoInfo, bool)>;
}

impl<T: VideoAggregatorImpl> VideoAggregatorImplExt for T {
    fn parent_update_caps(&self, caps: &gst::Caps) -> Result<gst::Caps, gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoAggregatorClass;
            let f = (*parent_class)
                .update_caps
                .expect("Missing parent function `update_caps`");

            Option::<_>::from_glib_full(f(
                self.obj()
                    .unsafe_cast_ref::<VideoAggregator>()
                    .to_glib_none()
                    .0,
                caps.as_mut_ptr(),
            ))
            .ok_or_else(|| {
                gst::loggable_error!(gst::CAT_RUST, "Parent function `update_caps` failed")
            })
        }
    }

    fn parent_aggregate_frames(
        &self,
        token: &AggregateFramesToken,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        assert_eq!(
            self.obj().as_ptr() as *mut ffi::GstVideoAggregator,
            token.0.as_ptr() as *mut ffi::GstVideoAggregator
        );

        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoAggregatorClass;
            let f = (*parent_class)
                .aggregate_frames
                .expect("Missing parent function `aggregate_frames`");

            try_from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoAggregator>()
                    .to_glib_none()
                    .0,
                // FIXME: Wrong pointer type
                outbuf.as_mut_ptr() as *mut *mut gst::ffi::GstBuffer,
            ))
        }
    }

    fn parent_create_output_buffer(&self) -> Result<Option<gst::Buffer>, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoAggregatorClass;
            let f = (*parent_class)
                .create_output_buffer
                .expect("Missing parent function `create_output_buffer`");

            let mut buffer = ptr::null_mut();
            try_from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<VideoAggregator>()
                    .to_glib_none()
                    .0,
                &mut buffer,
            ))
            .map(|_: gst::FlowSuccess| from_glib_full(buffer))
        }
    }

    fn parent_find_best_format(
        &self,
        downstream_caps: &gst::Caps,
    ) -> Option<(crate::VideoInfo, bool)> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstVideoAggregatorClass;
            (*parent_class).find_best_format.and_then(|f| {
                let mut info = mem::MaybeUninit::zeroed();
                ffi::gst_video_info_init(info.as_mut_ptr());
                let mut info = info.assume_init();

                let mut at_least_one_alpha = glib::ffi::GFALSE;

                f(
                    self.obj()
                        .unsafe_cast_ref::<VideoAggregator>()
                        .to_glib_none()
                        .0,
                    downstream_caps.as_mut_ptr(),
                    &mut info,
                    &mut at_least_one_alpha,
                );

                if info.finfo.is_null() {
                    None
                } else {
                    Some((
                        from_glib_none(mut_override(&info as *const ffi::GstVideoInfo)),
                        from_glib(at_least_one_alpha),
                    ))
                }
            })
        }
    }
}

unsafe impl<T: VideoAggregatorImpl> IsSubclassable<T> for VideoAggregator {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);

        let klass = klass.as_mut();
        klass.update_caps = Some(video_aggregator_update_caps::<T>);
        klass.aggregate_frames = Some(video_aggregator_aggregate_frames::<T>);
        klass.create_output_buffer = Some(video_aggregator_create_output_buffer::<T>);
        klass.find_best_format = Some(video_aggregator_find_best_format::<T>);
    }
}

unsafe extern "C" fn video_aggregator_update_caps<T: VideoAggregatorImpl>(
    ptr: *mut ffi::GstVideoAggregator,
    caps: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, ptr::null_mut(), {
        match imp.update_caps(&from_glib_borrow(caps)) {
            Ok(caps) => caps.into_glib_ptr(),
            Err(err) => {
                err.log_with_imp(imp);
                ptr::null_mut()
            }
        }
    })
}

unsafe extern "C" fn video_aggregator_aggregate_frames<T: VideoAggregatorImpl>(
    ptr: *mut ffi::GstVideoAggregator,
    outbuf: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        let instance = imp.obj();
        let instance = instance.unsafe_cast_ref::<VideoAggregator>();
        let token = AggregateFramesToken(instance);

        imp.aggregate_frames(
            &token,
            gst::BufferRef::from_mut_ptr(
                // Wrong pointer type
                outbuf as *mut gst::ffi::GstBuffer,
            ),
        )
        .into()
    })
    .into_glib()
}

unsafe extern "C" fn video_aggregator_create_output_buffer<T: VideoAggregatorImpl>(
    ptr: *mut ffi::GstVideoAggregator,
    outbuf: *mut *mut gst::ffi::GstBuffer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, gst::FlowReturn::Error, {
        match imp.create_output_buffer() {
            Ok(buffer) => {
                *outbuf = buffer.map(|b| b.into_glib_ptr()).unwrap_or(ptr::null_mut());
                Ok(gst::FlowSuccess::Ok)
            }
            Err(err) => {
                *outbuf = ptr::null_mut();
                Err(err)
            }
        }
        .into()
    })
    .into_glib()
}

unsafe extern "C" fn video_aggregator_find_best_format<T: VideoAggregatorImpl>(
    ptr: *mut ffi::GstVideoAggregator,
    downstream_caps: *mut gst::ffi::GstCaps,
    best_info: *mut ffi::GstVideoInfo,
    at_least_one_alpha: *mut glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, (), {
        match imp.find_best_format(&from_glib_borrow(downstream_caps)) {
            None => (),
            Some((info, alpha)) => {
                *best_info = *info.to_glib_none().0;
                *at_least_one_alpha = alpha.into_glib();
            }
        }
    })
}
