// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::Pipeline;

pub trait PipelineImpl: BinImpl {}

unsafe impl<T: PipelineImpl> IsSubclassable<T> for Pipeline
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <crate::Bin as IsSubclassable<T>>::override_vfuncs(klass);
        let _klass = klass.as_mut();
        // Nothing to do here
    }
}
