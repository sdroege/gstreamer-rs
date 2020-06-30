// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gst_base_sys;
use gst_sys;
use std::convert::TryFrom;
use std::mem;

use glib::translate::*;
use prelude::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;

use BaseParse;
use BaseParseClass;
use BaseParseFrame;

pub trait BaseParseImpl: BaseParseImplExt + ElementImpl + Send + Sync + 'static {
    fn start(&self, element: &BaseParse) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &BaseParse) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn set_sink_caps(
        &self,
        element: &BaseParse,
        caps: &gst::Caps,
    ) -> Result<(), gst::ErrorMessage> {
        self.parent_set_sink_caps(element, caps)
    }

    fn handle_frame<'a>(
        &'a self,
        element: &BaseParse,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
        self.parent_handle_frame(element, frame)
    }

    fn convert<V: Into<gst::GenericFormattedValue>>(
        &self,
        element: &BaseParse,
        src_val: V,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        self.parent_convert(element, src_val, dest_format)
    }
}

pub trait BaseParseImplExt {
    fn parent_start(&self, element: &BaseParse) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &BaseParse) -> Result<(), gst::ErrorMessage>;

    fn parent_set_sink_caps(
        &self,
        element: &BaseParse,
        caps: &gst::Caps,
    ) -> Result<(), gst::ErrorMessage>;

    fn parent_handle_frame<'a>(
        &'a self,
        element: &BaseParse,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError>;

    fn parent_convert<V: Into<gst::GenericFormattedValue>>(
        &self,
        element: &BaseParse,
        src_val: V,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue>;
}

impl<T: BaseParseImpl + ObjectImpl> BaseParseImplExt for T {
    fn parent_start(&self, element: &BaseParse) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseParseClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self, element: &BaseParse) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseParseClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `stop` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_set_sink_caps(
        &self,
        element: &BaseParse,
        caps: &gst::Caps,
    ) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseParseClass;
            (*parent_class)
                .set_sink_caps
                .map(|f| {
                    if from_glib(f(element.to_glib_none().0, caps.to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst_error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `set_sink_caps` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_handle_frame<'a>(
        &'a self,
        element: &'a BaseParse,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseParseClass;
            let mut skipsize = 0;
            let res = (*parent_class).handle_frame.map(|f| {
                let res = gst::FlowReturn::from_glib(f(
                    element.to_glib_none().0,
                    frame.to_glib_none().0,
                    &mut skipsize,
                ));
                (res, skipsize as u32)
            });

            match res {
                Some((res, skipsize)) => {
                    let res = res.into_result();
                    Ok((res.unwrap(), skipsize))
                }
                None => Err(gst::FlowError::Error),
            }
        }
    }

    fn parent_convert<V: Into<gst::GenericFormattedValue>>(
        &self,
        element: &BaseParse,
        src_val: V,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseParseClass;
            let src_val = src_val.into();
            let res = (*parent_class).convert.map(|f| {
                let mut dest_val = mem::MaybeUninit::uninit();

                let res = from_glib(f(
                    element.to_glib_none().0,
                    src_val.get_format().to_glib(),
                    src_val.to_raw_value(),
                    dest_format.to_glib(),
                    dest_val.as_mut_ptr(),
                ));
                (res, dest_val)
            });

            match res {
                Some((true, dest_val)) => Some(gst::GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                )),
                _ => None,
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + BaseParseImpl> IsSubclassable<T> for BaseParseClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstBaseParseClass);
            klass.start = Some(base_parse_start::<T>);
            klass.stop = Some(base_parse_stop::<T>);
            klass.set_sink_caps = Some(base_parse_set_sink_caps::<T>);
            klass.handle_frame = Some(base_parse_handle_frame::<T>);
            klass.convert = Some(base_parse_convert::<T>);
        }
    }
}

unsafe extern "C" fn base_parse_start<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseParse,
) -> glib_sys::gboolean
where
    T: BaseParseImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.start(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_parse_stop<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseParse,
) -> glib_sys::gboolean
where
    T: BaseParseImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.stop(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_parse_set_sink_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseParse,
    caps: *mut gst_sys::GstCaps,
) -> glib_sys::gboolean
where
    T: BaseParseImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);
    let caps: Borrowed<gst::Caps> = from_glib_borrow(caps);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.set_sink_caps(&wrap, &caps) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_parse_handle_frame<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseParse,
    frame: *mut gst_base_sys::GstBaseParseFrame,
    skipsize: *mut i32,
) -> gst_sys::GstFlowReturn
where
    T: BaseParseImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);
    let wrap_frame = BaseParseFrame::new(frame, &wrap);

    let res = gst_panic_to_error!(&wrap, &instance.panicked(), Err(gst::FlowError::Error), {
        imp.handle_frame(&wrap, wrap_frame)
    });

    match res {
        Ok((flow, skip)) => {
            *skipsize = i32::try_from(skip).expect("skip is higher than i32::MAX");
            gst::FlowReturn::from_ok(flow)
        }
        Err(flow) => gst::FlowReturn::from_error(flow),
    }
    .to_glib()
}

unsafe extern "C" fn base_parse_convert<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseParse,
    source_format: gst_sys::GstFormat,
    source_value: i64,
    dest_format: gst_sys::GstFormat,
    dest_value: *mut i64,
) -> glib_sys::gboolean
where
    T: BaseParseImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);
    let source = gst::GenericFormattedValue::new(from_glib(source_format), source_value);

    let res = gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.convert(&wrap, source, from_glib(dest_format))
    });

    match res {
        Some(dest) => {
            *dest_value = dest.to_raw_value();
            true
        }
        _ => false,
    }
    .to_glib()
}
