// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::GhostPad;

pub trait GhostPadImpl: PadImpl {}

unsafe impl<T: GhostPadImpl> IsSubclassable<T> for GhostPad {
    fn class_init(klass: &mut glib::Class<Self>) {
        <crate::Pad as IsSubclassable<T>>::class_init(klass);
        let _klass = klass.as_mut();
        // Nothing to do here
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <crate::Pad as IsSubclassable<T>>::instance_init(instance);
    }
}
