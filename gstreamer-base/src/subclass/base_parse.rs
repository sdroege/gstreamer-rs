// Take a look at the license at the top of the repository in the LICENSE file.

use std::convert::TryFrom;
use std::mem;

use crate::prelude::*;

use glib::translate::*;

use gst::subclass::prelude::*;

use crate::BaseParse;
use crate::BaseParseFrame;

pub trait BaseParseImpl: BaseParseImplExt + ElementImpl {
    fn start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn set_sink_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::ErrorMessage> {
        self.parent_set_sink_caps(element, caps)
    }

    fn handle_frame<'a>(
        &'a self,
        element: &Self::Type,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
        self.parent_handle_frame(element, frame)
    }

    fn convert<V: Into<gst::GenericFormattedValue>>(
        &self,
        element: &Self::Type,
        src_val: V,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        self.parent_convert(element, src_val, dest_format)
    }
}

pub trait BaseParseImplExt: ObjectSubclass {
    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage>;

    fn parent_set_sink_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::ErrorMessage>;

    fn parent_handle_frame<'a>(
        &'a self,
        element: &Self::Type,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError>;

    fn parent_convert<V: Into<gst::GenericFormattedValue>>(
        &self,
        element: &Self::Type,
        src_val: V,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue>;
}

impl<T: BaseParseImpl> BaseParseImplExt for T {
    fn parent_start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseParse>().to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(element.unsafe_cast_ref::<BaseParse>().to_glib_none().0)) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
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
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .set_sink_caps
                .map(|f| {
                    if from_glib(f(
                        element.unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                        caps.to_glib_none().0,
                    )) {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
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
        element: &'a Self::Type,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            let mut skipsize = 0;
            let res = (*parent_class).handle_frame.map(|f| {
                let res = gst::FlowReturn::from_glib(f(
                    element.unsafe_cast_ref::<BaseParse>().to_glib_none().0,
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
        element: &Self::Type,
        src_val: V,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            let src_val = src_val.into();
            let res = (*parent_class).convert.map(|f| {
                let mut dest_val = mem::MaybeUninit::uninit();

                let res = from_glib(f(
                    element.unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                    src_val.format().into_glib(),
                    src_val.to_raw_value(),
                    dest_format.into_glib(),
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

unsafe impl<T: BaseParseImpl> IsSubclassable<T> for BaseParse {
    fn class_init(klass: &mut glib::Class<Self>) {
        <gst::Element as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.start = Some(base_parse_start::<T>);
        klass.stop = Some(base_parse_stop::<T>);
        klass.set_sink_caps = Some(base_parse_set_sink_caps::<T>);
        klass.handle_frame = Some(base_parse_handle_frame::<T>);
        klass.convert = Some(base_parse_convert::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gst::Element as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn base_parse_start<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.start(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_parse_stop<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.stop(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_parse_set_sink_caps<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);
    let caps: Borrowed<gst::Caps> = from_glib_borrow(caps);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.set_sink_caps(wrap.unsafe_cast_ref(), &caps) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_parse_handle_frame<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    frame: *mut ffi::GstBaseParseFrame,
    skipsize: *mut i32,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);
    let wrap_frame = BaseParseFrame::new(frame, &wrap);

    let res = gst::panic_to_error!(&wrap, &imp.panicked(), Err(gst::FlowError::Error), {
        imp.handle_frame(&wrap.unsafe_cast_ref(), wrap_frame)
    });

    match res {
        Ok((flow, skip)) => {
            *skipsize = i32::try_from(skip).expect("skip is higher than i32::MAX");
            gst::FlowReturn::from_ok(flow)
        }
        Err(flow) => gst::FlowReturn::from_error(flow),
    }
    .into_glib()
}

unsafe extern "C" fn base_parse_convert<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    source_format: gst::ffi::GstFormat,
    source_value: i64,
    dest_format: gst::ffi::GstFormat,
    dest_value: *mut i64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<BaseParse> = from_glib_borrow(ptr);
    let source = gst::GenericFormattedValue::new(from_glib(source_format), source_value);

    let res = gst::panic_to_error!(&wrap, &imp.panicked(), None, {
        imp.convert(wrap.unsafe_cast_ref(), source, from_glib(dest_format))
    });

    match res {
        Some(dest) => {
            *dest_value = dest.to_raw_value();
            true
        }
        _ => false,
    }
    .into_glib()
}
