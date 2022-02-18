// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::prelude::*;
use crate::RTSPOnvifServer;

pub trait RTSPOnvifServerImpl: RTSPServerImpl + Send + Sync {}

unsafe impl<T: RTSPOnvifServerImpl> IsSubclassable<T> for RTSPOnvifServer {}
