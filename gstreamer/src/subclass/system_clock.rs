// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::prelude::*;
use crate::SystemClock;

pub trait SystemClockImpl: ClockImpl {}

unsafe impl<T: SystemClockImpl> IsSubclassable<T> for SystemClock {}
