// Take a look at the license at the top of the repository in the LICENSE file.

use gst_base::{prelude::*, subclass::prelude::*};

use crate::AudioBaseSrc;

pub trait AudioBaseSrcImpl: BaseSrcImpl + ObjectSubclass<Type: IsA<AudioBaseSrc>> {}

unsafe impl<T: AudioBaseSrcImpl> IsSubclassable<T> for AudioBaseSrc {}
