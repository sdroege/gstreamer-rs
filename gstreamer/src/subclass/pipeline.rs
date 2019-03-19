// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;

use super::prelude::*;
use glib::subclass::prelude::*;

use PipelineClass;

pub trait PipelineImpl: BinImpl + Send + Sync + 'static {}

unsafe impl<T: ObjectSubclass + PipelineImpl> IsSubclassable<T> for PipelineClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <::BinClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let _klass = &mut *(self as *mut Self as *mut gst_sys::GstPipelineClass);
            // Nothing to do here
        }
    }
}
