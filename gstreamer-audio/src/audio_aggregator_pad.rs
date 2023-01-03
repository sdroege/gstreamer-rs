use glib::{object::IsA, translate::*};

use crate::auto::AudioAggregatorPad;

pub trait AudioAggregatorPadExtManual: 'static {
    fn audio_info(&self) -> Option<crate::AudioInfo>;
}

impl<O: IsA<AudioAggregatorPad>> AudioAggregatorPadExtManual for O {
    fn audio_info(&self) -> Option<crate::AudioInfo> {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstAudioAggregatorPad;
            let _guard = crate::utils::MutexGuard::lock(&(*(ptr as *mut gst::ffi::GstObject)).lock);

            let info = &(*ptr).info;

            if !info.finfo.is_null() && info.channels > 0 && info.rate > 0 && info.bpf > 0 {
                return None;
            }

            Some(from_glib_none(mut_override(
                info as *const ffi::GstAudioInfo,
            )))
        }
    }
}
