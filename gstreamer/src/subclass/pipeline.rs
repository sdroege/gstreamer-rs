// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::Pipeline;

pub trait PipelineImpl: BinImpl {}

unsafe impl<T: PipelineImpl> IsSubclassable<T> for Pipeline {}
