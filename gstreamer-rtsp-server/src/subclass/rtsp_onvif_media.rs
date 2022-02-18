// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::prelude::*;
use crate::RTSPOnvifMedia;

pub trait RTSPOnvifMediaImpl: RTSPMediaImpl + Send + Sync {}

unsafe impl<T: RTSPOnvifMediaImpl> IsSubclassable<T> for RTSPOnvifMedia {}
