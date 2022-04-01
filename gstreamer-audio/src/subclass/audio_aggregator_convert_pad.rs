// Take a look at the license at the top of the repository in the LICENSE file.

use gst_base::subclass::prelude::*;

use super::prelude::AudioAggregatorPadImpl;
use crate::AudioAggregatorConvertPad;

pub trait AudioAggregatorConvertPadImpl: AudioAggregatorPadImpl {}

unsafe impl<T: AudioAggregatorConvertPadImpl> IsSubclassable<T> for AudioAggregatorConvertPad {}
