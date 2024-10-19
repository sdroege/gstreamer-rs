// Take a look at the license at the top of the repository in the LICENSE file.

use gst_base::{prelude::*, subclass::prelude::*};

use crate::AudioBaseSink;

pub trait AudioBaseSinkImpl: BaseSinkImpl + ObjectSubclass<Type: IsA<AudioBaseSink>> {}

unsafe impl<T: AudioBaseSinkImpl> IsSubclassable<T> for AudioBaseSink {}
