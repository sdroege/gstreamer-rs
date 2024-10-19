// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

use crate::Preset;

pub trait PresetImpl: super::element::ElementImpl + ObjectSubclass<Type: IsA<Preset>> {}

unsafe impl<T: PresetImpl> IsImplementable<T> for Preset {}
