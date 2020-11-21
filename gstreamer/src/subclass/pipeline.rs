// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
