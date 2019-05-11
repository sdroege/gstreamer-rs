// Copyright (C) 2017-2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;

use glib_sys;
use gst_sys;

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

pub trait ElementImpl: ElementImplExt + ObjectImpl + Send + Sync + 'static {
    fn change_state(
        &self,
        element: &::Element,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        self.parent_change_state(element, transition)
    }

    fn request_new_pad(
        &self,
        element: &::Element,
        templ: &::PadTemplate,
        name: Option<String>,
        caps: Option<&::CapsRef>,
    ) -> Option<::Pad> {
        self.parent_request_new_pad(element, templ, name, caps)
    }

    fn release_pad(&self, element: &::Element, pad: &::Pad) {
        self.parent_release_pad(element, pad)
    }

    fn send_event(&self, element: &::Element, event: Event) -> bool {
        self.parent_send_event(element, event)
    }

    fn query(&self, element: &::Element, query: &mut QueryRef) -> bool {
        self.parent_query(element, query)
    }

    fn set_context(&self, element: &::Element, context: &::Context) {
        self.parent_set_context(element, context)
    }
}

pub trait ElementImplExt {
    fn parent_change_state(
        &self,
        element: &::Element,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError>;

    fn parent_request_new_pad(
        &self,
        element: &::Element,
        templ: &::PadTemplate,
        name: Option<String>,
        caps: Option<&::CapsRef>,
    ) -> Option<::Pad>;

    fn parent_release_pad(&self, element: &::Element, pad: &::Pad);

    fn parent_send_event(&self, element: &::Element, event: Event) -> bool;

    fn parent_query(&self, element: &::Element, query: &mut QueryRef) -> bool;

    fn parent_set_context(&self, element: &::Element, context: &::Context);

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
    fn parent_change_state(
        &self,
        element: &::Element,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            let f = (*parent_class)
                .change_state
                .expect("Missing parent function `change_state`");
            StateChangeReturn::from_glib(f(element.to_glib_none().0, transition.to_glib()))
                .into_result()
        }
    }

    fn parent_request_new_pad(
        &self,
        element: &::Element,
        templ: &::PadTemplate,
        name: Option<String>,
        caps: Option<&::CapsRef>,
    ) -> Option<::Pad> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .request_new_pad
                .map(|f| {
                    from_glib_none(f(
                        element.to_glib_none().0,
                        templ.to_glib_none().0,
                        name.to_glib_full(),
                        caps.map(|caps| caps.as_ptr()).unwrap_or(std::ptr::null()),
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_release_pad(&self, element: &::Element, pad: &::Pad) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .release_pad
                .map(|f| f(element.to_glib_none().0, pad.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_send_event(&self, element: &::Element, event: Event) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .send_event
                .map(|f| from_glib(f(element.to_glib_none().0, event.into_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_query(&self, element: &::Element, query: &mut QueryRef) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .query
                .map(|f| from_glib(f(element.to_glib_none().0, query.as_mut_ptr())))
                .unwrap_or(false)
        }
    }

    fn parent_set_context(&self, element: &::Element, context: &::Context) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .set_context
                .map(|f| f(element.to_glib_none().0, context.to_glib_none().0))
                .unwrap_or(())
        }
    }

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
            let ptr: *mut gst_sys::GstElement = element.as_ptr() as *mut _;
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
            let ptr: *mut gst_sys::GstElement = wrap.to_glib_none().0;
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();

            gst_panic_to_error!(wrap, &instance.panicked(), fallback(), { f(&imp, &wrap) })
        }
    }
}

pub unsafe trait ElementClassSubclassExt: Sized + 'static {
    fn add_pad_template(&mut self, pad_template: PadTemplate) {
        unsafe {
            gst_sys::gst_element_class_add_pad_template(
                self as *mut Self as *mut gst_sys::GstElementClass,
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
            gst_sys::gst_element_class_set_metadata(
                self as *mut Self as *mut gst_sys::GstElementClass,
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
            let klass = &mut *(self as *mut Self as *mut gst_sys::GstElementClass);
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
    ptr: *mut gst_sys::GstElement,
    transition: gst_sys::GstStateChange,
) -> gst_sys::GstStateChangeReturn
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
    ptr: *mut gst_sys::GstElement,
    templ: *mut gst_sys::GstPadTemplate,
    name: *const libc::c_char,
    caps: *const gst_sys::GstCaps,
) -> *mut gst_sys::GstPad
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
            Some(::Object::from_glib_borrow(ptr as *mut gst_sys::GstObject))
        );
    }

    pad.to_glib_none().0
}

unsafe extern "C" fn element_release_pad<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstElement,
    pad: *mut gst_sys::GstPad,
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
    ptr: *mut gst_sys::GstElement,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
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
    ptr: *mut gst_sys::GstElement,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
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
    ptr: *mut gst_sys::GstElement,
    context: *mut gst_sys::GstContext,
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

#[cfg(test)]
mod tests {
    use super::*;
    use glib;
    use glib::subclass;
    use std::sync::Mutex;

    struct TestElement {
        srcpad: ::Pad,
        sinkpad: ::Pad,
        n_buffers: Mutex<u32>,
        reached_playing: Mutex<bool>,
    }

    impl TestElement {
        fn set_pad_functions(sinkpad: &::Pad, srcpad: &::Pad) {
            sinkpad.set_chain_function(|pad, parent, buffer| {
                TestElement::catch_panic_pad_function(
                    parent,
                    || Err(::FlowError::Error),
                    |identity, element| identity.sink_chain(pad, element, buffer),
                )
            });
            sinkpad.set_event_function(|pad, parent, event| {
                TestElement::catch_panic_pad_function(
                    parent,
                    || false,
                    |identity, element| identity.sink_event(pad, element, event),
                )
            });
            sinkpad.set_query_function(|pad, parent, query| {
                TestElement::catch_panic_pad_function(
                    parent,
                    || false,
                    |identity, element| identity.sink_query(pad, element, query),
                )
            });

            srcpad.set_event_function(|pad, parent, event| {
                TestElement::catch_panic_pad_function(
                    parent,
                    || false,
                    |identity, element| identity.src_event(pad, element, event),
                )
            });
            srcpad.set_query_function(|pad, parent, query| {
                TestElement::catch_panic_pad_function(
                    parent,
                    || false,
                    |identity, element| identity.src_query(pad, element, query),
                )
            });
        }

        fn sink_chain(
            &self,
            _pad: &::Pad,
            _element: &::Element,
            buffer: ::Buffer,
        ) -> Result<::FlowSuccess, ::FlowError> {
            *self.n_buffers.lock().unwrap() += 1;
            self.srcpad.push(buffer)
        }

        fn sink_event(&self, _pad: &::Pad, _element: &::Element, event: ::Event) -> bool {
            self.srcpad.push_event(event)
        }

        fn sink_query(&self, _pad: &::Pad, _element: &::Element, query: &mut ::QueryRef) -> bool {
            self.srcpad.peer_query(query)
        }

        fn src_event(&self, _pad: &::Pad, _element: &::Element, event: ::Event) -> bool {
            self.sinkpad.push_event(event)
        }

        fn src_query(&self, _pad: &::Pad, _element: &::Element, query: &mut ::QueryRef) -> bool {
            self.sinkpad.peer_query(query)
        }
    }

    impl ObjectSubclass for TestElement {
        const NAME: &'static str = "TestElement";
        type ParentType = ::Element;
        type Instance = ::subclass::ElementInstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new_with_class(klass: &subclass::simple::ClassStruct<Self>) -> Self {
            let templ = klass.get_pad_template("sink").unwrap();
            let sinkpad = ::Pad::new_from_template(&templ, Some("sink"));
            let templ = klass.get_pad_template("src").unwrap();
            let srcpad = ::Pad::new_from_template(&templ, Some("src"));

            TestElement::set_pad_functions(&sinkpad, &srcpad);

            Self {
                n_buffers: Mutex::new(0),
                reached_playing: Mutex::new(false),
                srcpad,
                sinkpad,
            }
        }

        fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
            klass.set_metadata(
                "Test Element",
                "Generic",
                "Does nothing",
                "Sebastian Dröge <sebastian@centricular.com>",
            );

            let caps = ::Caps::new_any();
            let src_pad_template =
                ::PadTemplate::new("src", ::PadDirection::Src, ::PadPresence::Always, &caps)
                    .unwrap();
            klass.add_pad_template(src_pad_template);

            let sink_pad_template =
                ::PadTemplate::new("sink", ::PadDirection::Sink, ::PadPresence::Always, &caps)
                    .unwrap();
            klass.add_pad_template(sink_pad_template);
        }
    }

    impl ObjectImpl for TestElement {
        glib_object_impl!();

        fn constructed(&self, obj: &glib::Object) {
            self.parent_constructed(obj);

            let element = obj.downcast_ref::<::Element>().unwrap();
            element.add_pad(&self.sinkpad).unwrap();
            element.add_pad(&self.srcpad).unwrap();
        }
    }

    impl ElementImpl for TestElement {
        fn change_state(
            &self,
            element: &::Element,
            transition: ::StateChange,
        ) -> Result<::StateChangeSuccess, ::StateChangeError> {
            let res = self.parent_change_state(element, transition)?;

            if transition == ::StateChange::PausedToPlaying {
                *self.reached_playing.lock().unwrap() = true;
            }

            Ok(res)
        }
    }

    #[test]
    fn test_element_subclass() {
        ::init().unwrap();

        let element = glib::Object::new(TestElement::get_type(), &[("name", &"test")])
            .unwrap()
            .downcast::<::Element>()
            .unwrap();

        assert_eq!(element.get_name(), "test");

        assert_eq!(
            element.get_metadata(&::ELEMENT_METADATA_LONGNAME),
            Some("Test Element")
        );

        let pipeline = ::Pipeline::new(None);
        let src = ::ElementFactory::make("fakesrc", None).unwrap();
        let sink = ::ElementFactory::make("fakesink", None).unwrap();

        src.set_property("num-buffers", &100i32).unwrap();

        pipeline.add_many(&[&src, &element, &sink]).unwrap();
        ::Element::link_many(&[&src, &element, &sink]).unwrap();

        pipeline.set_state(::State::Playing).unwrap();
        let bus = pipeline.get_bus().unwrap();

        let eos = bus.timed_pop_filtered(::CLOCK_TIME_NONE, &[::MessageType::Eos]);
        assert!(eos.is_some());

        pipeline.set_state(::State::Null).unwrap();

        let imp = TestElement::from_instance(&element);
        assert_eq!(*imp.n_buffers.lock().unwrap(), 100);
        assert_eq!(*imp.reached_playing.lock().unwrap(), true);
    }
}
