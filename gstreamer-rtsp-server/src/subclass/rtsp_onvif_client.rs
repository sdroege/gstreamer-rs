// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

use super::prelude::*;
use crate::RTSPOnvifClient;

pub trait RTSPOnvifClientImpl:
    RTSPClientImpl + ObjectSubclass<Type: IsA<RTSPOnvifClient>> + Send + Sync
{
}

unsafe impl<T: RTSPOnvifClientImpl> IsSubclassable<T> for RTSPOnvifClient {}
