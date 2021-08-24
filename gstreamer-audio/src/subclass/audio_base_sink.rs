// Take a look at the license at the top of the repository in the LICENSE file.

use gst_base::subclass::prelude::*;

use crate::AudioBaseSink;

pub trait AudioBaseSinkImpl: BaseSinkImpl {}

unsafe impl<T: AudioBaseSinkImpl> IsSubclassable<T> for AudioBaseSink {}
