// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

use super::prelude::*;
use crate::GhostPad;

pub trait GhostPadImpl: ProxyPadImpl + ObjectSubclass<Type: IsA<GhostPad>> {}

unsafe impl<T: GhostPadImpl> IsSubclassable<T> for GhostPad {}
