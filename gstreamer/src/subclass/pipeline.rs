// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::Pipeline;

pub trait PipelineImpl: BinImpl {}

unsafe impl<T: PipelineImpl> IsSubclassable<T> for Pipeline
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn class_init(klass: &mut glib::Class<Self>) {
        <crate::Bin as IsSubclassable<T>>::class_init(klass);
        let _klass = klass.as_mut();
        // Nothing to do here
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <crate::Bin as IsSubclassable<T>>::instance_init(instance);
    }
}
