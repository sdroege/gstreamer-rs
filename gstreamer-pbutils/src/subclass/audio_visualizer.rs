// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;
use gst::subclass::prelude::*;
use gst::{result_from_gboolean, LoggableError, CAT_RUST};

use crate::AudioVisualizer;

pub struct AudioVisualizerSetupToken<'a>(pub(crate) &'a AudioVisualizer);

pub trait AudioVisualizerImpl: AudioVisualizerImplExt + ElementImpl {
    fn setup(
        &self,
        element: &Self::Type,
        token: &AudioVisualizerSetupToken,
    ) -> Result<(), LoggableError> {
        self.parent_setup(element, token)
    }

    fn render(
        &self,
        element: &Self::Type,
        audio_buffer: &gst::BufferRef,
        video_frame: &mut gst_video::VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<(), LoggableError> {
        self.parent_render(element, audio_buffer, video_frame)
    }

    fn decide_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        self.parent_decide_allocation(element, query)
    }
}

pub trait AudioVisualizerImplExt: ObjectSubclass {
    fn parent_setup(
        &self,
        element: &Self::Type,
        token: &AudioVisualizerSetupToken,
    ) -> Result<(), LoggableError>;

    fn parent_render(
        &self,
        element: &Self::Type,
        audio_buffer: &gst::BufferRef,
        video_frame: &mut gst_video::VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<(), LoggableError>;

    fn parent_decide_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError>;
}

impl<T: AudioVisualizerImpl> AudioVisualizerImplExt for T {
    fn parent_setup(
        &self,
        element: &Self::Type,
        token: &AudioVisualizerSetupToken,
    ) -> Result<(), LoggableError> {
        assert_eq!(
            element.as_ptr() as *mut ffi::GstAudioVisualizer,
            token.0.as_ptr() as *mut ffi::GstAudioVisualizer
        );

        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioVisualizerClass;
            (*parent_class)
                .setup
                .map(|f| {
                    result_from_gboolean!(
                        f(element
                            .unsafe_cast_ref::<AudioVisualizer>()
                            .to_glib_none()
                            .0,),
                        CAT_RUST,
                        "Parent function `setup` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_render(
        &self,
        element: &Self::Type,
        audio_buffer: &gst::BufferRef,
        video_frame: &mut gst_video::VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioVisualizerClass;
            (*parent_class)
                .render
                .map(|f| {
                    result_from_gboolean!(
                        f(
                            element
                                .unsafe_cast_ref::<AudioVisualizer>()
                                .to_glib_none()
                                .0,
                            audio_buffer.as_mut_ptr(),
                            video_frame.as_mut_ptr(),
                        ),
                        CAT_RUST,
                        "Parent function `render` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_decide_allocation(
        &self,
        element: &Self::Type,
        query: &mut gst::query::Allocation,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioVisualizerClass;
            (*parent_class)
                .decide_allocation
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            element
                                .unsafe_cast_ref::<AudioVisualizer>()
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

unsafe impl<T: AudioVisualizerImpl> IsSubclassable<T> for AudioVisualizer {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.setup = Some(audio_visualizer_setup::<T>);
        klass.render = Some(audio_visualizer_render::<T>);
        klass.decide_allocation = Some(audio_visualizer_decide_allocation::<T>);
    }
}

unsafe extern "C" fn audio_visualizer_setup<T: AudioVisualizerImpl>(
    ptr: *mut ffi::GstAudioVisualizer,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<AudioVisualizer> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        let token = AudioVisualizerSetupToken(&*wrap);

        match imp.setup(wrap.unsafe_cast_ref(), &token) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audio_visualizer_render<T: AudioVisualizerImpl>(
    ptr: *mut ffi::GstAudioVisualizer,
    audio_buffer: *mut gst::ffi::GstBuffer,
    video_frame: *mut gst_video::ffi::GstVideoFrame,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<AudioVisualizer> = from_glib_borrow(ptr);
    let buffer = gst::BufferRef::from_ptr(audio_buffer);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.render(
            wrap.unsafe_cast_ref(),
            buffer,
            &mut gst_video::VideoFrameRef::from_glib_borrow_mut(video_frame),
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

unsafe extern "C" fn audio_visualizer_decide_allocation<T: AudioVisualizerImpl>(
    ptr: *mut ffi::GstAudioVisualizer,
    query: *mut gst::ffi::GstQuery,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<AudioVisualizer> = from_glib_borrow(ptr);
    let query = match gst::QueryRef::from_mut_ptr(query).view_mut() {
        gst::QueryViewMut::Allocation(allocation) => allocation,
        _ => unreachable!(),
    };

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.decide_allocation(wrap.unsafe_cast_ref(), query) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}
