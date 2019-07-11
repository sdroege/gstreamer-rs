// Copyright (C) 2017-2019 Sebastian Dröge <sebastian@centricular.com>
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
use prelude::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;

use std::mem;

use BaseTransform;
use BaseTransformClass;

pub trait BaseTransformImpl: BaseTransformImplExt + ElementImpl + Send + Sync + 'static {
    fn start(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage> {
        self.parent_start(element)
    }

    fn stop(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage> {
        self.parent_stop(element)
    }

    fn transform_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        self.parent_transform_caps(element, direction, caps, filter)
    }

    fn fixate_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps {
        self.parent_fixate_caps(element, direction, caps, othercaps)
    }

    fn set_caps(&self, element: &BaseTransform, incaps: &gst::Caps, outcaps: &gst::Caps) -> bool {
        self.parent_set_caps(element, incaps, outcaps)
    }

    fn accept_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool {
        self.parent_accept_caps(element, direction, caps)
    }

    fn query(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool {
        BaseTransformImplExt::parent_query(self, element, direction, query)
    }

    fn transform_size(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize> {
        self.parent_transform_size(element, direction, caps, size, othercaps)
    }

    fn get_unit_size(&self, element: &BaseTransform, caps: &gst::Caps) -> Option<usize> {
        self.parent_get_unit_size(element, caps)
    }

    fn sink_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        self.parent_sink_event(element, event)
    }

    fn src_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        self.parent_src_event(element, event)
    }

    fn transform(
        &self,
        element: &BaseTransform,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform(element, inbuf, outbuf)
    }

    fn transform_ip(
        &self,
        element: &BaseTransform,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_ip(element, buf)
    }

    fn transform_ip_passthrough(
        &self,
        element: &BaseTransform,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.parent_transform_ip_passthrough(element, buf)
    }
}

pub trait BaseTransformImplExt {
    fn parent_start(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage>;

    fn parent_stop(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage>;

    fn parent_transform_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps>;

    fn parent_fixate_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps;

    fn parent_set_caps(
        &self,
        element: &BaseTransform,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> bool;

    fn parent_accept_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool;

    fn parent_query(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool;

    fn parent_transform_size(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize>;

    fn parent_get_unit_size(&self, element: &BaseTransform, caps: &gst::Caps) -> Option<usize>;

    fn parent_sink_event(&self, element: &BaseTransform, event: gst::Event) -> bool;

    fn parent_src_event(&self, element: &BaseTransform, event: gst::Event) -> bool;

    fn parent_transform(
        &self,
        element: &BaseTransform,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_transform_ip(
        &self,
        element: &BaseTransform,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn parent_transform_ip_passthrough(
        &self,
        element: &BaseTransform,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<T: BaseTransformImpl + ObjectImpl> BaseTransformImplExt for T {
    fn parent_start(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
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

    fn parent_stop(&self, element: &BaseTransform) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
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

    fn parent_transform_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> Option<gst::Caps> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .transform_caps
                .map(|f| {
                    from_glib_full(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
                        caps.to_glib_none().0,
                        filter.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_fixate_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        othercaps: gst::Caps,
    ) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            match (*parent_class).fixate_caps {
                Some(f) => from_glib_full(f(
                    element.to_glib_none().0,
                    direction.to_glib(),
                    caps.to_glib_none().0,
                    othercaps.into_ptr(),
                )),
                None => othercaps,
            }
        }
    }

    fn parent_set_caps(
        &self,
        element: &BaseTransform,
        incaps: &gst::Caps,
        outcaps: &gst::Caps,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    from_glib(f(
                        element.to_glib_none().0,
                        incaps.to_glib_none().0,
                        outcaps.to_glib_none().0,
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_accept_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .accept_caps
                .map(|f| {
                    from_glib(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
                        caps.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_query(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        query: &mut gst::QueryRef,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_transform_size(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        size: usize,
        othercaps: &gst::Caps,
    ) -> Option<usize> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .transform_size
                .map(|f| {
                    let mut othersize = mem::MaybeUninit::uninit();
                    let res: bool = from_glib(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
                        caps.to_glib_none().0,
                        size,
                        othercaps.to_glib_none().0,
                        othersize.as_mut_ptr(),
                    ));
                    if res {
                        Some(othersize.assume_init())
                    } else {
                        None
                    }
                })
                .unwrap_or(None)
        }
    }

    fn parent_get_unit_size(&self, element: &BaseTransform, caps: &gst::Caps) -> Option<usize> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            let f = (*parent_class).get_unit_size.unwrap_or_else(|| {
                if !element.is_in_place() {
                    unimplemented!(concat!(
                        "Missing parent function `get_unit_size`. Required because ",
                        "transform element doesn't operate in-place"
                    ))
                } else {
                    unreachable!(concat!(
                        "parent `get_unit_size` called ",
                        "while transform element operates in-place"
                    ))
                }
            });

            let mut size = mem::MaybeUninit::uninit();
            if from_glib(f(
                element.to_glib_none().0,
                caps.to_glib_none().0,
                size.as_mut_ptr(),
            )) {
                Some(size.assume_init())
            } else {
                None
            }
        }
    }

    fn parent_sink_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .sink_event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(true)
        }
    }

    fn parent_src_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .src_event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(true)
        }
    }

    fn parent_transform(
        &self,
        element: &BaseTransform,
        inbuf: &gst::Buffer,
        outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            (*parent_class)
                .transform
                .map(|f| {
                    from_glib(f(
                        element.to_glib_none().0,
                        inbuf.to_glib_none().0,
                        outbuf.as_mut_ptr(),
                    ))
                })
                .unwrap_or_else(|| {
                    if !element.is_in_place() {
                        gst::FlowReturn::NotSupported
                    } else {
                        unreachable!(concat!(
                            "parent `transform` called ",
                            "while transform element operates in-place"
                        ));
                    }
                })
                .into_result()
        }
    }

    fn parent_transform_ip(
        &self,
        element: &BaseTransform,
        buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            let f = (*parent_class).transform_ip.unwrap_or_else(|| {
                if element.is_in_place() {
                    panic!(concat!(
                        "Missing parent function `transform_ip`. Required because ",
                        "transform element operates in-place"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform` called ",
                        "while transform element doesn't operate in-place"
                    ));
                }
            });

            gst::FlowReturn::from_glib(f(element.to_glib_none().0, buf.as_mut_ptr() as *mut _))
                .into_result()
        }
    }

    fn parent_transform_ip_passthrough(
        &self,
        element: &BaseTransform,
        buf: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_base_sys::GstBaseTransformClass;
            let f = (*parent_class).transform_ip.unwrap_or_else(|| {
                if element.is_in_place() {
                    panic!(concat!(
                        "Missing parent function `transform_ip`. Required because ",
                        "transform element operates in-place (passthrough mode)"
                    ));
                } else {
                    unreachable!(concat!(
                        "parent `transform_ip` called ",
                        "while transform element doesn't operate in-place (passthrough mode)"
                    ));
                }
            });

            // FIXME: Wrong signature in FFI
            let buf: *mut gst_sys::GstBuffer = buf.to_glib_none().0;
            gst::FlowReturn::from_glib(f(element.to_glib_none().0, buf as *mut _)).into_result()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BaseTransformMode {
    AlwaysInPlace,
    NeverInPlace,
    Both,
}

unsafe impl<T: ObjectSubclass + BaseTransformImpl> IsSubclassable<T> for BaseTransformClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstBaseTransformClass);
            klass.start = Some(base_transform_start::<T>);
            klass.stop = Some(base_transform_stop::<T>);
            klass.transform_caps = Some(base_transform_transform_caps::<T>);
            klass.fixate_caps = Some(base_transform_fixate_caps::<T>);
            klass.set_caps = Some(base_transform_set_caps::<T>);
            klass.accept_caps = Some(base_transform_accept_caps::<T>);
            klass.query = Some(base_transform_query::<T>);
            klass.transform_size = Some(base_transform_transform_size::<T>);
            klass.get_unit_size = Some(base_transform_get_unit_size::<T>);
            klass.sink_event = Some(base_transform_sink_event::<T>);
            klass.src_event = Some(base_transform_src_event::<T>);
        }
    }
}

pub unsafe trait BaseTransformClassSubclassExt: Sized + 'static {
    fn configure<T: ObjectSubclass + BaseTransformImpl>(
        &mut self,
        mode: BaseTransformMode,
        passthrough_on_same_caps: bool,
        transform_ip_on_passthrough: bool,
    ) where
        Self: ClassStruct<Type = T>,
        <T as ObjectSubclass>::Instance: PanicPoison,
    {
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_base_sys::GstBaseTransformClass);

            klass.passthrough_on_same_caps = passthrough_on_same_caps.to_glib();
            klass.transform_ip_on_passthrough = transform_ip_on_passthrough.to_glib();

            match mode {
                BaseTransformMode::AlwaysInPlace => {
                    klass.transform_ip = Some(base_transform_transform_ip::<T>);
                }
                BaseTransformMode::NeverInPlace => {
                    klass.transform = Some(base_transform_transform::<T>);
                }
                BaseTransformMode::Both => {
                    klass.transform = Some(base_transform_transform::<T>);
                    klass.transform_ip = Some(base_transform_transform_ip::<T>);
                }
            }
        }
    }
}

unsafe impl<T: ClassStruct> BaseTransformClassSubclassExt for T
where
    T::Type: ObjectSubclass + BaseTransformImpl,
    <T::Type as ObjectSubclass>::Instance: PanicPoison,
{
}

unsafe extern "C" fn base_transform_start<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.start(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(&err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_stop<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.stop(&wrap) {
            Ok(()) => true,
            Err(err) => {
                wrap.post_error_message(&err);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_transform_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    caps: *mut gst_sys::GstCaps,
    filter: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        let filter = if filter.is_null() {
            None
        } else {
            Some(from_glib_borrow(filter))
        };

        imp.transform_caps(
            &wrap,
            from_glib(direction),
            &from_glib_borrow(caps),
            filter.as_ref(),
        )
    })
    .map(|caps| caps.into_ptr())
    .unwrap_or(std::ptr::null_mut())
}

unsafe extern "C" fn base_transform_fixate_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    caps: *mut gst_sys::GstCaps,
    othercaps: *mut gst_sys::GstCaps,
) -> *mut gst_sys::GstCaps
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
        imp.fixate_caps(
            &wrap,
            from_glib(direction),
            &from_glib_borrow(caps),
            from_glib_full(othercaps),
        )
    })
    .into_ptr()
}

unsafe extern "C" fn base_transform_set_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    incaps: *mut gst_sys::GstCaps,
    outcaps: *mut gst_sys::GstCaps,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.set_caps(&wrap, &from_glib_borrow(incaps), &from_glib_borrow(outcaps))
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_accept_caps<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    caps: *mut gst_sys::GstCaps,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.accept_caps(&wrap, from_glib(direction), &from_glib_borrow(caps))
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_query<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        BaseTransformImpl::query(
            imp,
            &wrap,
            from_glib(direction),
            gst::QueryRef::from_mut_ptr(query),
        )
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_transform_size<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    direction: gst_sys::GstPadDirection,
    caps: *mut gst_sys::GstCaps,
    size: usize,
    othercaps: *mut gst_sys::GstCaps,
    othersize: *mut usize,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.transform_size(
            &wrap,
            from_glib(direction),
            &from_glib_borrow(caps),
            size,
            &from_glib_borrow(othercaps),
        ) {
            Some(s) => {
                *othersize = s;
                true
            }
            None => false,
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_get_unit_size<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    caps: *mut gst_sys::GstCaps,
    size: *mut usize,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.get_unit_size(&wrap, &from_glib_borrow(caps)) {
            Some(s) => {
                *size = s;
                true
            }
            None => false,
        }
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_sink_event<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.sink_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_src_event<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.src_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_transform<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    inbuf: *mut gst_sys::GstBuffer,
    outbuf: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        imp.transform(
            &wrap,
            &from_glib_borrow(inbuf),
            gst::BufferRef::from_mut_ptr(outbuf),
        )
        .into()
    })
    .to_glib()
}

unsafe extern "C" fn base_transform_transform_ip<T: ObjectSubclass>(
    ptr: *mut gst_base_sys::GstBaseTransform,
    buf: *mut *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    // FIXME: Wrong signature in FFI
    let buf = buf as *mut gst_sys::GstBuffer;

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        if from_glib(gst_base_sys::gst_base_transform_is_passthrough(ptr)) {
            imp.transform_ip_passthrough(&wrap, &from_glib_borrow(buf))
                .into()
        } else {
            imp.transform_ip(&wrap, gst::BufferRef::from_mut_ptr(buf))
                .into()
        }
    })
    .to_glib()
}
