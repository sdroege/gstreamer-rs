// Take a look at the license at the top of the repository in the LICENSE file.

use gst::prelude::*;

use crate::{auto::AudioVisualizer, subclass::AudioVisualizerSetupToken};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AudioVisualizer>> Sealed for T {}
}

pub trait AudioVisualizerExtManual:
    sealed::Sealed + IsA<AudioVisualizer> + IsA<gst::Element> + 'static
{
    // rustdoc-stripper-ignore-next
    /// Returns the number of samples per frame required before calling the render method
    fn req_spf(&self) -> u32 {
        let sinkpad = self.static_pad("sink").expect("sink pad presence");
        let _stream_lock = sinkpad.stream_lock();

        let ptr = self.as_ptr() as *mut ffi::GstAudioVisualizer;
        unsafe { (*ptr).req_spf }
    }

    // rustdoc-stripper-ignore-next
    /// Modify the request of samples per frame required to be present in buffer before calling
    /// the render method
    fn set_req_spf(&self, spf: u32, token: &AudioVisualizerSetupToken) {
        assert_eq!(
            self.as_ptr() as *mut ffi::GstAudioVisualizer,
            token.0.as_ptr()
        );

        let sinkpad = self.static_pad("sink").expect("sink pad presence");
        let _stream_lock = sinkpad.stream_lock();

        let ptr = self.as_ptr() as *mut ffi::GstAudioVisualizer;
        unsafe {
            (*ptr).req_spf = spf;
        }
    }

    fn audio_info(&self) -> gst_audio::AudioInfo {
        let sinkpad = self.static_pad("sink").expect("sink pad presence");
        let _stream_lock = sinkpad.stream_lock();

        let ptr = self.as_ptr() as *mut ffi::GstAudioVisualizer;
        unsafe {
            let info = &(*ptr).ainfo;
            glib::translate::from_glib_none(glib::translate::mut_override(
                info as *const gst_audio::ffi::GstAudioInfo,
            ))
        }
    }

    fn video_info(&self) -> gst_video::VideoInfo {
        let srcpad = self.static_pad("src").expect("src pad presence");
        let _stream_lock = srcpad.stream_lock();

        let ptr = self.as_ptr() as *mut ffi::GstAudioVisualizer;
        unsafe {
            let info = &(*ptr).vinfo;
            glib::translate::from_glib_none(glib::translate::mut_override(
                info as *const gst_video::ffi::GstVideoInfo,
            ))
        }
    }
}

impl<O: IsA<AudioVisualizer> + IsA<gst::Element>> AudioVisualizerExtManual for O {}
