// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*};

use super::prelude::*;
use crate::Pipeline;

pub trait PipelineImpl: BinImpl + ObjectSubclass<Type: IsA<Pipeline>> {}

unsafe impl<T: PipelineImpl> IsSubclassable<T> for Pipeline {}
