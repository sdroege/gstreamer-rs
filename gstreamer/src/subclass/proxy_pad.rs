// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::prelude::*;
use crate::ProxyPad;

pub trait ProxyPadImpl: PadImpl {}

unsafe impl<T: ProxyPadImpl> IsSubclassable<T> for ProxyPad {}
