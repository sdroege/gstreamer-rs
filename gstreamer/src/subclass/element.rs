// Copyright (C) 2017,2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;

use ffi;
use glib_ffi;

use super::prelude::*;
use glib;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use prelude::*;

use Element;
use ElementClass;
use Event;
use PadTemplate;
use QueryRef;
use StateChange;
use StateChangeError;
use StateChangeReturn;
use StateChangeSuccess;

pub trait ElementImpl: ObjectImpl + Send + Sync + 'static {
    fn change_state(
        &self,
        element: &::Element,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        self.parent_change_state(element, transition)
    }

    fn request_new_pad(
        &self,
        _element: &::Element,
        _templ: &::PadTemplate,
        _name: Option<String>,
        _caps: Option<&::CapsRef>,
    ) -> Option<::Pad> {
        None
    }

    fn release_pad(&self, _element: &::Element, _pad: &::Pad) {}

    fn send_event(&self, element: &::Element, event: Event) -> bool {
        self.parent_send_event(element, event)
    }

    fn query(&self, element: &::Element, query: &mut QueryRef) -> bool {
        self.parent_query(element, query)
    }

    fn set_context(&self, element: &::Element, context: &::Context) {
        self.parent_set_context(element, context)
    }

    fn parent_change_state(
        &self,
        element: &::Element,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .change_state
                .map(|f| from_glib(f(element.to_glib_none().0, transition.to_glib())))
                .unwrap_or(::StateChangeReturn::Success)
                .into_result()
        }
    }

    fn parent_send_event(&self, element: &::Element, event: Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .send_event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_query(&self, element: &::Element, query: &mut QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .query
                .map(|f| from_glib(f(element.to_glib_none().0, query.as_mut_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_set_context(&self, element: &::Element, context: &::Context) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .set_context
                .map(|f| f(element.to_glib_none().0, context.to_glib_none().0))
                .unwrap_or(())
        }
    }
}

pub trait ElementImplExt {
    fn catch_panic<
        R,
        F: FnOnce(&Self) -> R,
        G: FnOnce() -> R,
        P: IsA<::Element> + IsA<glib::Object> + glib::value::SetValue,
    >(
        &self,
        element: &P,
        fallback: G,
        f: F,
    ) -> R;

    fn catch_panic_pad_function<R, F: FnOnce(&Self, &::Element) -> R, G: FnOnce() -> R>(
        parent: &Option<::Object>,
        fallback: G,
        f: F,
    ) -> R;
}

impl<T: ElementImpl + ObjectSubclass> ElementImplExt for T
where
    T::Instance: PanicPoison,
{
    fn catch_panic<
        R,
        F: FnOnce(&Self) -> R,
        G: FnOnce() -> R,
        P: IsA<::Element> + IsA<glib::Object> + glib::value::SetValue,
    >(
        &self,
        element: &P,
        fallback: G,
        f: F,
    ) -> R {
        unsafe {
            assert!(element.get_type().is_a(&T::get_type()));
            let ptr: *mut ffi::GstElement = element.to_glib_none().0;
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();

            gst_panic_to_error!(element, &instance.panicked(), fallback(), { f(&imp) })
        }
    }

    fn catch_panic_pad_function<R, F: FnOnce(&Self, &::Element) -> R, G: FnOnce() -> R>(
        parent: &Option<::Object>,
        fallback: G,
        f: F,
    ) -> R {
        unsafe {
            let wrap = parent
                .as_ref()
                .unwrap()
                .downcast_ref::<::Element>()
                .unwrap();
            assert!(wrap.get_type().is_a(&T::get_type()));
            let ptr: *mut ffi::GstElement = wrap.to_glib_none().0;
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();

            gst_panic_to_error!(wrap, &instance.panicked(), fallback(), { f(&imp, &wrap) })
        }
    }
}

pub unsafe trait ElementClassSubclassExt: Sized + 'static {
    fn add_pad_template(&mut self, pad_template: PadTemplate) {
        unsafe {
            ffi::gst_element_class_add_pad_template(
                self as *const Self as *mut ffi::GstElementClass,
                pad_template.to_glib_none().0,
            );
        }
    }

    fn set_metadata(
        &mut self,
        long_name: &str,
        classification: &str,
        description: &str,
        author: &str,
    ) {
        unsafe {
            ffi::gst_element_class_set_metadata(
                self as *const Self as *mut ffi::GstElementClass,
                long_name.to_glib_none().0,
                classification.to_glib_none().0,
                description.to_glib_none().0,
                author.to_glib_none().0,
            );
        }
    }
}

unsafe impl ElementClassSubclassExt for ElementClass {}

unsafe impl<T: ObjectSubclass + ElementImpl> IsSubclassable<T> for ElementClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);

        unsafe {
            let klass = &mut *(self as *const Self as *mut ffi::GstElementClass);
            klass.change_state = Some(element_change_state::<T>);
            klass.request_new_pad = Some(element_request_new_pad::<T>);
            klass.release_pad = Some(element_release_pad::<T>);
            klass.send_event = Some(element_send_event::<T>);
            klass.query = Some(element_query::<T>);
            klass.set_context = Some(element_set_context::<T>);
        }
    }
}

unsafe extern "C" fn element_change_state<T: ObjectSubclass>(
    ptr: *mut ffi::GstElement,
    transition: ffi::GstStateChange,
) -> ffi::GstStateChangeReturn
where
    T: ElementImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Element = from_glib_borrow(ptr);

    // *Never* fail downwards state changes, this causes bugs in GStreamer
    // and leads to crashes and deadlocks.
    let transition = from_glib(transition);
    let fallback = match transition {
        StateChange::PlayingToPaused | StateChange::PausedToReady | StateChange::ReadyToNull => {
            StateChangeReturn::Success
        }
        _ => StateChangeReturn::Failure,
    };

    gst_panic_to_error!(&wrap, &instance.panicked(), fallback, {
        imp.change_state(&wrap, transition).into()
    })
    .to_glib()
}

unsafe extern "C" fn element_request_new_pad<T: ObjectSubclass>(
    ptr: *mut ffi::GstElement,
    templ: *mut ffi::GstPadTemplate,
    name: *const libc::c_char,
    caps: *const ffi::GstCaps,
) -> *mut ffi::GstPad
where
    T: ElementImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Element = from_glib_borrow(ptr);

    let caps = if caps.is_null() {
        None
    } else {
        Some(::CapsRef::from_ptr(caps))
    };

    // XXX: This is effectively unsafe but the best we can do
    // See https://bugzilla.gnome.org/show_bug.cgi?id=791193
    let pad = gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.request_new_pad(&wrap, &from_glib_borrow(templ), from_glib_none(name), caps)
    });

    // Ensure that the pad is owned by the element now, if a pad was returned
    if let Some(ref pad) = pad {
        assert_eq!(
            pad.get_parent(),
            Some(::Object::from_glib_borrow(ptr as *mut ffi::GstObject))
        );
    }

    pad.to_glib_none().0
}

unsafe extern "C" fn element_release_pad<T: ObjectSubclass>(
    ptr: *mut ffi::GstElement,
    pad: *mut ffi::GstPad,
) where
    T: ElementImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Element = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.release_pad(&wrap, &from_glib_borrow(pad))
    })
}

unsafe extern "C" fn element_send_event<T: ObjectSubclass>(
    ptr: *mut ffi::GstElement,
    event: *mut ffi::GstEvent,
) -> glib_ffi::gboolean
where
    T: ElementImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Element = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.send_event(&wrap, from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn element_query<T: ObjectSubclass>(
    ptr: *mut ffi::GstElement,
    query: *mut ffi::GstQuery,
) -> glib_ffi::gboolean
where
    T: ElementImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Element = from_glib_borrow(ptr);
    let query = QueryRef::from_mut_ptr(query);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.query(&wrap, query)
    })
    .to_glib()
}

unsafe extern "C" fn element_set_context<T: ObjectSubclass>(
    ptr: *mut ffi::GstElement,
    context: *mut ffi::GstContext,
) where
    T: ElementImpl,
    T::Instance: PanicPoison,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Element = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.set_context(&wrap, &from_glib_borrow(context))
    })
}
