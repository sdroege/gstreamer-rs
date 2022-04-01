// Take a look at the license at the top of the repository in the LICENSE file.

use gst_base::subclass::prelude::*;

use super::prelude::VideoAggregatorPadImpl;
use crate::VideoAggregatorConvertPad;

pub trait VideoAggregatorConvertPadImpl: VideoAggregatorPadImpl {}

unsafe impl<T: VideoAggregatorConvertPadImpl> IsSubclassable<T> for VideoAggregatorConvertPad {}
