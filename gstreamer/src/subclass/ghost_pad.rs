// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::GhostPad;

pub trait GhostPadImpl: PadImpl {}

unsafe impl<T: GhostPadImpl> IsSubclassable<T> for GhostPad {}
