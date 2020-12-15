// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::GhostPad;

pub trait GhostPadImpl: PadImpl {}

unsafe impl<T: GhostPadImpl> IsSubclassable<T> for GhostPad {
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <crate::Pad as IsSubclassable<T>>::override_vfuncs(klass);
        let _klass = klass.as_mut();
        // Nothing to do here
    }
}
