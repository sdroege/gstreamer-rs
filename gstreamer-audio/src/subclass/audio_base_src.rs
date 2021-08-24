// Take a look at the license at the top of the repository in the LICENSE file.

use gst_base::subclass::prelude::*;

use crate::AudioBaseSrc;

pub trait AudioBaseSrcImpl: BaseSrcImpl {}

unsafe impl<T: AudioBaseSrcImpl> IsSubclassable<T> for AudioBaseSrc {}
