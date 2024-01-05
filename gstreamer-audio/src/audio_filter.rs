// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use gst_base::prelude::*;

use crate::{AudioFilter, AudioInfo};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AudioFilter>> Sealed for T {}
}

pub trait AudioFilterExtManual: sealed::Sealed + IsA<AudioFilter> + 'static {
    fn audio_info(&self) -> Option<AudioInfo> {
        unsafe {
            let ptr: &ffi::GstAudioFilter = &*(self.as_ptr() as *const _);
            let sinkpad = self.as_ref().sink_pad();
            let _guard = sinkpad.stream_lock();

            let info = &ptr.info;

            if !info.finfo.is_null() && info.channels > 0 && info.rate > 0 && info.bpf > 0 {
                return None;
            }
            Some(from_glib_none(info as *const ffi::GstAudioInfo))
        }
    }
}

impl<O: IsA<AudioFilter>> AudioFilterExtManual for O {}
