// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_base_sys;
use gst_sys;

use glib::translate::*;
use gst;

use glib::subclass::prelude::*;
use gst::subclass::prelude::*;

use Aggregator;
use AggregatorPad;
use AggregatorPadClass;

pub trait AggregatorPadImpl: AggregatorPadImplExt + PadImpl + Send + Sync + 'static {
    fn flush(
        &self,
        aggregator_pad: &AggregatorPad,
        aggregator: &Aggregator,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_flush(aggregator_pad, aggregator)
    }

    fn skip_buffer(
        &self,
        aggregator_pad: &AggregatorPad,
        aggregator: &Aggregator,
        buffer: &gst::Buffer,
    ) -> bool {
        self.parent_skip_buffer(aggregator_pad, aggregator, buffer)
    }
}

pub trait AggregatorPadImplExt {
    fn parent_flush(
        &self,
        aggregator_pad: &AggregatorPad,
        aggregator: &Aggregator,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_skip_buffer(
        &self,
        aggregator_pad: &AggregatorPad,
        aggregator: &Aggregator,
        buffer: &gst::Buffer,
    ) -> bool;
}

impl<T: AggregatorPadImpl + ObjectImpl> AggregatorPadImplExt for T {
    fn parent_flush(
        &self,
        aggregator_pad: &AggregatorPad,
        aggregator: &Aggregator,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstAggregatorPadClass;
            (*parent_class)
                .flush
                .map(|f| {
                    from_glib(f(
                        aggregator_pad.to_glib_none().0,
                        aggregator.to_glib_none().0,
                    ))
                })
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_skip_buffer(
        &self,
        aggregator_pad: &AggregatorPad,
        aggregator: &Aggregator,
        buffer: &gst::Buffer,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstAggregatorPadClass;
            (*parent_class)
                .skip_buffer
                .map(|f| {
                    from_glib(f(
                        aggregator_pad.to_glib_none().0,
                        aggregator.to_glib_none().0,
                        buffer.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }
}
unsafe impl<T: ObjectSubclass + AggregatorPadImpl> IsSubclassable<T> for AggregatorPadClass {
    fn override_vfuncs(&mut self) {
        <gst::PadClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstAggregatorPadClass);
            klass.flush = Some(aggregator_pad_flush::<T>);
            klass.skip_buffer = Some(aggregator_pad_skip_buffer::<T>);
        }
    }
}

unsafe extern "C" fn aggregator_pad_flush<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstAggregatorPad,
    aggregator: *mut gst_base_sys::GstAggregator,
) -> gst_sys::GstFlowReturn
where
    T: AggregatorPadImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AggregatorPad> = from_glib_borrow(ptr);

    let res: gst::FlowReturn = imp.flush(&wrap, &from_glib_borrow(aggregator)).into();
    res.to_glib()
}

unsafe extern "C" fn aggregator_pad_skip_buffer<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstAggregatorPad,
    aggregator: *mut gst_base_sys::GstAggregator,
    buffer: *mut gst_sys::GstBuffer,
) -> glib_sys::gboolean
where
    T: AggregatorPadImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AggregatorPad> = from_glib_borrow(ptr);

    imp.skip_buffer(
        &wrap,
        &from_glib_borrow(aggregator),
        &from_glib_borrow(buffer),
    )
    .to_glib()
}
