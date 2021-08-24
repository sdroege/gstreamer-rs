// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::ProxyPad;

pub trait ProxyPadImpl: PadImpl {}

unsafe impl<T: ProxyPadImpl> IsSubclassable<T> for ProxyPad {}
