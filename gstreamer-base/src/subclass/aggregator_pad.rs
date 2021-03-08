// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use gst::subclass::prelude::*;

use crate::Aggregator;
use crate::AggregatorPad;

pub trait AggregatorPadImpl: AggregatorPadImplExt + PadImpl {
    fn flush(
        &self,
        aggregator_pad: &Self::Type,
        aggregator: &Aggregator,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_flush(aggregator_pad, aggregator)
    }

    fn skip_buffer(
        &self,
        aggregator_pad: &Self::Type,
        aggregator: &Aggregator,
        buffer: &gst::Buffer,
    ) -> bool {
        self.parent_skip_buffer(aggregator_pad, aggregator, buffer)
    }
}

pub trait AggregatorPadImplExt: ObjectSubclass {
    fn parent_flush(
        &self,
        aggregator_pad: &Self::Type,
        aggregator: &Aggregator,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_skip_buffer(
        &self,
        aggregator_pad: &Self::Type,
        aggregator: &Aggregator,
        buffer: &gst::Buffer,
    ) -> bool;
}

impl<T: AggregatorPadImpl> AggregatorPadImplExt for T {
    fn parent_flush(
        &self,
        aggregator_pad: &Self::Type,
        aggregator: &Aggregator,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorPadClass;
            (*parent_class)
                .flush
                .map(|f| {
                    from_glib(f(
                        aggregator_pad
                            .unsafe_cast_ref::<AggregatorPad>()
                            .to_glib_none()
                            .0,
                        aggregator.to_glib_none().0,
                    ))
                })
                .unwrap_or(gst::FlowReturn::Ok)
                .into_result()
        }
    }

    fn parent_skip_buffer(
        &self,
        aggregator_pad: &Self::Type,
        aggregator: &Aggregator,
        buffer: &gst::Buffer,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstAggregatorPadClass;
            (*parent_class)
                .skip_buffer
                .map(|f| {
                    from_glib(f(
                        aggregator_pad
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
unsafe impl<T: AggregatorPadImpl> IsSubclassable<T> for AggregatorPad {
    fn class_init(klass: &mut glib::Class<Self>) {
        <gst::Pad as IsSubclassable<T>>::class_init(klass);
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
    let imp = instance.get_impl();
    let wrap: Borrowed<AggregatorPad> = from_glib_borrow(ptr);

    let res: gst::FlowReturn = imp
        .flush(wrap.unsafe_cast_ref(), &from_glib_borrow(aggregator))
        .into();
    res.to_glib()
}

unsafe extern "C" fn aggregator_pad_skip_buffer<T: AggregatorPadImpl>(
    ptr: *mut ffi::GstAggregatorPad,
    aggregator: *mut ffi::GstAggregator,
    buffer: *mut gst::ffi::GstBuffer,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AggregatorPad> = from_glib_borrow(ptr);

    imp.skip_buffer(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(aggregator),
        &from_glib_borrow(buffer),
    )
    .to_glib()
}
