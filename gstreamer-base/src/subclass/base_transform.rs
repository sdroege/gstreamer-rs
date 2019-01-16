// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib_ffi;
use gst_ffi;

use glib::translate::*;
use prelude::*;

use glib::subclass::prelude::*;
use gst;
use gst::subclass::prelude::*;

use BaseTransform;
use BaseTransformClass;

pub trait BaseTransformImpl: ElementImpl + Send + Sync + 'static {
    fn start(&self, _element: &BaseTransform) -> bool {
        true
    }

    fn stop(&self, _element: &BaseTransform) -> bool {
        true
    }

    fn transform_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> gst::Caps {
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

    fn set_caps(
        &self,
        _element: &BaseTransform,
        _incaps: &gst::Caps,
        _outcaps: &gst::Caps,
    ) -> bool {
        true
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
        BaseTransformImpl::parent_query(self, element, direction, query)
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

    fn get_unit_size(&self, _element: &BaseTransform, _caps: &gst::Caps) -> Option<usize> {
        unimplemented!();
    }

    fn sink_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        self.parent_sink_event(element, event)
    }

    fn src_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        self.parent_src_event(element, event)
    }

    fn transform(
        &self,
        _element: &BaseTransform,
        _inbuf: &gst::Buffer,
        _outbuf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unimplemented!();
    }

    fn transform_ip(
        &self,
        _element: &BaseTransform,
        _buf: &mut gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unimplemented!();
    }

    fn transform_ip_passthrough(
        &self,
        _element: &BaseTransform,
        _buf: &gst::BufferRef,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unimplemented!();
    }

    fn parent_transform_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: Option<&gst::Caps>,
    ) -> gst::Caps {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseTransformClass;
            match (*parent_class).transform_caps {
                Some(f) => from_glib_full(f(
                    element.to_glib_none().0,
                    direction.to_glib(),
                    caps.to_glib_none().0,
                    filter.to_glib_none().0,
                )),
                None => caps.clone(),
            }
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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseTransformClass;
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

    fn parent_accept_caps(
        &self,
        element: &BaseTransform,
        direction: gst::PadDirection,
        caps: &gst::Caps,
    ) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseTransformClass;
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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseTransformClass;
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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .transform_size
                .map(|f| {
                    let mut othersize = 0;
                    let res: bool = from_glib(f(
                        element.to_glib_none().0,
                        direction.to_glib(),
                        caps.to_glib_none().0,
                        size,
                        othercaps.to_glib_none().0,
                        &mut othersize,
                    ));
                    if res {
                        Some(othersize)
                    } else {
                        None
                    }
                })
                .unwrap_or(None)
        }
    }

    fn parent_sink_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .sink_event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_src_event(&self, element: &BaseTransform, event: gst::Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstBaseTransformClass;
            (*parent_class)
                .src_event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(false)
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
            let klass = &mut *(self as *mut Self as *mut ffi::GstBaseTransformClass);
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
            let klass = &mut *(self as *mut Self as *mut ffi::GstBaseTransformClass);

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
    ptr: *mut ffi::GstBaseTransform,
) -> glib_ffi::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, { imp.start(&wrap) }).to_glib()
}

unsafe extern "C" fn base_transform_stop<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseTransform,
) -> glib_ffi::gboolean
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, { imp.stop(&wrap) }).to_glib()
}

unsafe extern "C" fn base_transform_transform_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst_ffi::GstPadDirection,
    caps: *mut gst_ffi::GstCaps,
    filter: *mut gst_ffi::GstCaps,
) -> *mut gst_ffi::GstCaps
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::Caps::new_empty(), {
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
    .into_ptr()
}

unsafe extern "C" fn base_transform_fixate_caps<T: ObjectSubclass>(
    ptr: *mut ffi::GstBaseTransform,
    direction: gst_ffi::GstPadDirection,
    caps: *mut gst_ffi::GstCaps,
    othercaps: *mut gst_ffi::GstCaps,
) -> *mut gst_ffi::GstCaps
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
    ptr: *mut ffi::GstBaseTransform,
    incaps: *mut gst_ffi::GstCaps,
    outcaps: *mut gst_ffi::GstCaps,
) -> glib_ffi::gboolean
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
    ptr: *mut ffi::GstBaseTransform,
    direction: gst_ffi::GstPadDirection,
    caps: *mut gst_ffi::GstCaps,
) -> glib_ffi::gboolean
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
    ptr: *mut ffi::GstBaseTransform,
    direction: gst_ffi::GstPadDirection,
    query: *mut gst_ffi::GstQuery,
) -> glib_ffi::gboolean
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
    ptr: *mut ffi::GstBaseTransform,
    direction: gst_ffi::GstPadDirection,
    caps: *mut gst_ffi::GstCaps,
    size: usize,
    othercaps: *mut gst_ffi::GstCaps,
    othersize: *mut usize,
) -> glib_ffi::gboolean
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
    ptr: *mut ffi::GstBaseTransform,
    caps: *mut gst_ffi::GstCaps,
    size: *mut usize,
) -> glib_ffi::gboolean
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
    ptr: *mut ffi::GstBaseTransform,
    event: *mut gst_ffi::GstEvent,
) -> glib_ffi::gboolean
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
    ptr: *mut ffi::GstBaseTransform,
    event: *mut gst_ffi::GstEvent,
) -> glib_ffi::gboolean
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
    ptr: *mut ffi::GstBaseTransform,
    inbuf: *mut gst_ffi::GstBuffer,
    outbuf: *mut gst_ffi::GstBuffer,
) -> gst_ffi::GstFlowReturn
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
    ptr: *mut ffi::GstBaseTransform,
    buf: *mut *mut gst_ffi::GstBuffer,
) -> gst_ffi::GstFlowReturn
where
    T: BaseTransformImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: BaseTransform = from_glib_borrow(ptr);

    // FIXME: Wrong signature in FFI
    let buf = buf as *mut gst_ffi::GstBuffer;

    gst_panic_to_error!(&wrap, &instance.panicked(), gst::FlowReturn::Error, {
        if from_glib(ffi::gst_base_transform_is_passthrough(ptr)) {
            imp.transform_ip_passthrough(&wrap, gst::BufferRef::from_ptr(buf))
                .into()
        } else {
            imp.transform_ip(&wrap, gst::BufferRef::from_mut_ptr(buf))
                .into()
        }
    })
    .to_glib()
}
