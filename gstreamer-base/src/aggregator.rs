// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::prelude::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib::IsA;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::Value;
use gst;
use gst_base_sys;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::mem::transmute;
use std::ptr;
use Aggregator;

pub trait AggregatorExtManual: 'static {
    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams);

    fn finish_buffer(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn finish_buffer_list(
        &self,
        bufferlist: gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_property_min_upstream_latency(&self) -> gst::ClockTime;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_property_min_upstream_latency(&self, min_upstream_latency: gst::ClockTime);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn connect_property_min_upstream_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn update_segment<F: gst::FormattedValue>(&self, segment: &gst::FormattedSegment<F>);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn selected_samples(
        &self,
        pts: gst::ClockTime,
        dts: gst::ClockTime,
        duration: gst::ClockTime,
        info: Option<&gst::StructureRef>,
    );

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn connect_samples_selected<
        P,
        F: Fn(
                &P,
                &gst::Segment,
                gst::ClockTime,
                gst::ClockTime,
                gst::ClockTime,
                Option<&gst::StructureRef>,
            ) + Send
            + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId
    where
        P: IsA<Aggregator>;
}

impl<O: IsA<Aggregator>> AggregatorExtManual for O {
    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            gst_base_sys::gst_aggregator_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn finish_buffer(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_base_sys::gst_aggregator_finish_buffer(
                self.as_ref().to_glib_none().0,
                buffer.into_ptr(),
            ))
        };
        ret.into_result()
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn finish_buffer_list(
        &self,
        bufferlist: gst::BufferList,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_base_sys::gst_aggregator_finish_buffer_list(
                self.as_ref().to_glib_none().0,
                bufferlist.into_ptr(),
            ))
        };
        ret.into_result()
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_property_min_upstream_latency(&self) -> gst::ClockTime {
        unsafe {
            let mut value = Value::from_type(<gst::ClockTime as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"min-upstream-latency\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("AggregatorExtManual::get_property_min_upstream_latency")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_property_min_upstream_latency(&self, min_upstream_latency: gst::ClockTime) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"min-upstream-latency\0".as_ptr() as *const _,
                Value::from(&min_upstream_latency).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn connect_property_min_upstream_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-upstream-latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_upstream_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn update_segment<F: gst::FormattedValue>(&self, segment: &gst::FormattedSegment<F>) {
        unsafe {
            gst_base_sys::gst_aggregator_update_segment(
                self.as_ref().to_glib_none().0,
                mut_override(segment.to_glib_none().0),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn selected_samples(
        &self,
        pts: gst::ClockTime,
        dts: gst::ClockTime,
        duration: gst::ClockTime,
        info: Option<&gst::StructureRef>,
    ) {
        unsafe {
            gst_base_sys::gst_aggregator_selected_samples(
                self.as_ref().to_glib_none().0,
                pts.to_glib(),
                dts.to_glib(),
                duration.to_glib(),
                info.as_ref()
                    .map(|s| s.as_ptr() as *mut _)
                    .unwrap_or(ptr::null_mut()),
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn connect_samples_selected<
        P,
        F: Fn(
                &P,
                &gst::Segment,
                gst::ClockTime,
                gst::ClockTime,
                gst::ClockTime,
                Option<&gst::StructureRef>,
            ) + Send
            + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId
    where
        P: IsA<Aggregator>,
    {
        unsafe extern "C" fn samples_selected_trampoline<
            P,
            F: Fn(
                    &P,
                    &gst::Segment,
                    gst::ClockTime,
                    gst::ClockTime,
                    gst::ClockTime,
                    Option<&gst::StructureRef>,
                ) + Send
                + 'static,
        >(
            this: *mut gst_base_sys::GstAggregator,
            segment: *mut gst_sys::GstSegment,
            pts: gst_sys::GstClockTime,
            dts: gst_sys::GstClockTime,
            duration: gst_sys::GstClockTime,
            info: *mut gst_sys::GstStructure,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Aggregator>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Aggregator::from_glib_borrow(this).unsafe_cast_ref(),
                &gst::Segment::from_glib_borrow(segment),
                from_glib(pts),
                from_glib(dts),
                from_glib(duration),
                if info.is_null() {
                    None
                } else {
                    Some(gst::StructureRef::from_glib_borrow(info))
                },
            )
        }

        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"samples-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    samples_selected_trampoline::<P, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
unsafe extern "C" fn notify_min_upstream_latency_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
    this: *mut gst_base_sys::GstAggregator,
    _param_spec: glib_sys::gpointer,
    f: glib_sys::gpointer,
) where
    P: IsA<Aggregator>,
{
    let f: &F = &*(f as *const F);
    f(&Aggregator::from_glib_borrow(this).unsafe_cast_ref())
}
