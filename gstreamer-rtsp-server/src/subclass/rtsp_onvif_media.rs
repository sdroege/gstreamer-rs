// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

use super::prelude::*;
use crate::RTSPOnvifMedia;

pub trait RTSPOnvifMediaImpl:
    RTSPMediaImpl + ObjectSubclass<Type: IsA<RTSPOnvifMedia>> + Send + Sync
{
}

unsafe impl<T: RTSPOnvifMediaImpl> IsSubclassable<T> for RTSPOnvifMedia {}
