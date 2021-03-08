// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use crate::TagSetter;

pub trait TagSetterImpl: super::element::ElementImpl {}

unsafe impl<T: TagSetterImpl> IsImplementable<T> for TagSetter {
    fn interface_init(_iface: &mut glib::Class<Self>) {}
}
