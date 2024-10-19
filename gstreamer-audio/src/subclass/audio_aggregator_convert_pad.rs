// Take a look at the license at the top of the repository in the LICENSE file.

use gst_base::{prelude::*, subclass::prelude::*};

use super::prelude::AudioAggregatorPadImpl;
use crate::AudioAggregatorConvertPad;

pub trait AudioAggregatorConvertPadImpl:
    AudioAggregatorPadImpl + ObjectSubclass<Type: IsA<AudioAggregatorConvertPad>>
{
}

unsafe impl<T: AudioAggregatorConvertPadImpl> IsSubclassable<T> for AudioAggregatorConvertPad {}
