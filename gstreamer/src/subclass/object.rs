// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

pub trait GstObjectImpl:
    ObjectImpl + ObjectSubclass<Type: IsA<crate::Object>> + Send + Sync
{
}

unsafe impl<T: GstObjectImpl> IsSubclassable<T> for crate::Object {}
