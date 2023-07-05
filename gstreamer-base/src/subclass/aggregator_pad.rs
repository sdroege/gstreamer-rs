// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use gst::subclass::prelude::*;

use crate::{Aggregator, AggregatorPad};

pub trait AggregatorPadImpl: AggregatorPadImplExt + PadImpl {
    fn flush(&self, aggregator: &Aggregator) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_flush(aggregator)
    }

    fn skip_buffer(&self, aggregator: &Aggregator, buffer: &gst::Buffer) -> bool {
        self.parent_skip_buffer(aggregator, buffer)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AggregatorPadImplExt> Sealed for T {}
}

pub trait AggregatorPadImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_flush(&self, aggregator: &Aggregator) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAggregatorPadClass;
            (*parent_class)
                .flush
                .map(|f| {
                    try_from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<AggregatorPad>()
                            .to_glib_none()
                            .0,
                        aggregator.to_glib_none().0,
                    ))
                })
                .unwrap_or(Ok(gst::FlowSuccess::Ok))
        }
    }

    fn parent_skip_buffer(&self, aggregator: &Aggregator, buffer: &gst::Buffer) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAggregatorPadClass;
            (*parent_class)
                .skip_buffer
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<AggregatorPad>()
                            .to_glib_none()
                            .0,
                        aggregator.to_glib_none().0,
                        buffer.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }
}

impl<T: AggregatorPadImpl> AggregatorPadImplExt for T {}
unsafe impl<T: AggregatorPadImpl> IsSubclassable<T> for AggregatorPad {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.flush = Some(aggregator_pad_flush::<T>);
        klass.skip_buffer = Some(aggregator_pad_skip_buffer::<T>);
    }
}

unsafe extern "C" fn aggregator_pad_flush<T: AggregatorPadImpl>(
    ptr: *mut ffi::GstAggregatorPad,
    aggregator: *mut ffi::GstAggregator,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let res: gst::FlowReturn = imp.flush(&from_glib_borrow(aggregator)).into();
    res.into_glib()
}

unsafe extern "C" fn aggregator_pad_skip_buffer<T: AggregatorPadImpl>(
    ptr: *mut ffi::GstAggregatorPad,
    aggregator: *mut ffi::GstAggregator,
    buffer: *mut gst::ffi::GstBuffer,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.skip_buffer(&from_glib_borrow(aggregator), &from_glib_borrow(buffer))
        .into_glib()
}
