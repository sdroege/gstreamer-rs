// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

use crate::TocSetter;

pub trait TocSetterImpl:
    super::element::ElementImpl + ObjectSubclass<Type: IsA<TocSetter>>
{
}

unsafe impl<T: TocSetterImpl> IsImplementable<T> for TocSetter {}
