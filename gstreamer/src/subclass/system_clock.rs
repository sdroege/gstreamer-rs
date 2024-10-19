// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

use super::prelude::*;
use crate::SystemClock;

pub trait SystemClockImpl: ClockImpl + ObjectSubclass<Type: IsA<SystemClock>> {}

unsafe impl<T: SystemClockImpl> IsSubclassable<T> for SystemClock {}
