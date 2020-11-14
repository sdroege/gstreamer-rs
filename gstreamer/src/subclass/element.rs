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
use Event;
use PadTemplate;
use QueryRef;
use StateChange;
use StateChangeError;
use StateChangeReturn;
use StateChangeSuccess;

pub trait ElementImpl: ElementImplExt + ObjectImpl + Send + Sync {
    fn change_state(
        &self,
        element: &Self::Type,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        self.parent_change_state(element, transition)
    }

    fn request_new_pad(
        &self,
        element: &Self::Type,
        templ: &::PadTemplate,
        name: Option<String>,
        caps: Option<&::Caps>,
    ) -> Option<::Pad> {
        self.parent_request_new_pad(element, templ, name, caps)
    }

    fn release_pad(&self, element: &Self::Type, pad: &::Pad) {
        self.parent_release_pad(element, pad)
    }

    fn send_event(&self, element: &Self::Type, event: Event) -> bool {
        self.parent_send_event(element, event)
    }

    fn query(&self, element: &Self::Type, query: &mut QueryRef) -> bool {
        self.parent_query(element, query)
    }

    fn set_context(&self, element: &Self::Type, context: &::Context) {
        self.parent_set_context(element, context)
    }

    fn set_clock(&self, element: &Self::Type, clock: Option<&::Clock>) -> bool {
        self.parent_set_clock(element, clock)
    }

    fn provide_clock(&self, element: &Self::Type) -> Option<::Clock> {
        self.parent_provide_clock(element)
    }

    fn post_message(&self, element: &Self::Type, msg: ::Message) -> bool {
        self.parent_post_message(element, msg)
    }
}

pub trait ElementImplExt: ObjectSubclass {
    fn parent_change_state(
        &self,
        element: &Self::Type,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError>;

    fn parent_request_new_pad(
        &self,
        element: &Self::Type,
        templ: &::PadTemplate,
        name: Option<String>,
        caps: Option<&::Caps>,
    ) -> Option<::Pad>;

    fn parent_release_pad(&self, element: &Self::Type, pad: &::Pad);

    fn parent_send_event(&self, element: &Self::Type, event: Event) -> bool;

    fn parent_query(&self, element: &Self::Type, query: &mut QueryRef) -> bool;

    fn parent_set_context(&self, element: &Self::Type, context: &::Context);

    fn parent_set_clock(&self, element: &Self::Type, clock: Option<&::Clock>) -> bool;

    fn parent_provide_clock(&self, element: &Self::Type) -> Option<::Clock>;

    fn parent_post_message(&self, element: &Self::Type, msg: ::Message) -> bool;

    fn catch_panic<R, F: FnOnce(&Self) -> R, G: FnOnce() -> R, P: IsA<Element>>(
        &self,
        element: &P,
        fallback: G,
        f: F,
    ) -> R;

    fn catch_panic_pad_function<R, F: FnOnce(&Self, &Self::Type) -> R, G: FnOnce() -> R>(
        parent: Option<&::Object>,
        fallback: G,
        f: F,
    ) -> R;
}

impl<T: ElementImpl> ElementImplExt for T
where
    T::Instance: PanicPoison,
{
    fn parent_change_state(
        &self,
        element: &Self::Type,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            let f = (*parent_class)
                .change_state
                .expect("Missing parent function `change_state`");
            StateChangeReturn::from_glib(f(
                element.unsafe_cast_ref::<Element>().to_glib_none().0,
                transition.to_glib(),
            ))
            .into_result()
        }
    }

    fn parent_request_new_pad(
        &self,
        element: &Self::Type,
        templ: &::PadTemplate,
        name: Option<String>,
        caps: Option<&::Caps>,
    ) -> Option<::Pad> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .request_new_pad
                .map(|f| {
                    from_glib_none(f(
                        element.unsafe_cast_ref::<Element>().to_glib_none().0,
                        templ.to_glib_none().0,
                        name.to_glib_full(),
                        caps.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_release_pad(&self, element: &Self::Type, pad: &::Pad) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .release_pad
                .map(|f| {
                    f(
                        element.unsafe_cast_ref::<Element>().to_glib_none().0,
                        pad.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_send_event(&self, element: &Self::Type, event: Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .send_event
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<Element>().to_glib_none().0,
                        event.into_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_query(&self, element: &Self::Type, query: &mut QueryRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<Element>().to_glib_none().0,
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_set_context(&self, element: &Self::Type, context: &::Context) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .set_context
                .map(|f| {
                    f(
                        element.unsafe_cast_ref::<Element>().to_glib_none().0,
                        context.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_set_clock(&self, element: &Self::Type, clock: Option<&::Clock>) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .set_clock
                .map(|f| {
                    from_glib(f(
                        element.unsafe_cast_ref::<Element>().to_glib_none().0,
                        clock.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_provide_clock(&self, element: &Self::Type) -> Option<::Clock> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            (*parent_class)
                .provide_clock
                .map(|f| from_glib_none(f(element.unsafe_cast_ref::<Element>().to_glib_none().0)))
                .unwrap_or(None)
        }
    }

    fn parent_post_message(&self, element: &Self::Type, msg: ::Message) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstElementClass;

            if let Some(f) = (*parent_class).post_message {
                from_glib(f(
                    element.unsafe_cast_ref::<Element>().to_glib_none().0,
                    msg.into_ptr(),
                ))
            } else {
                false
            }
        }
    }

    fn catch_panic<R, F: FnOnce(&Self) -> R, G: FnOnce() -> R, P: IsA<Element>>(
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

    fn catch_panic_pad_function<R, F: FnOnce(&Self, &Self::Type) -> R, G: FnOnce() -> R>(
        parent: Option<&::Object>,
        fallback: G,
        f: F,
    ) -> R {
        unsafe {
            let wrap = parent.as_ref().unwrap().downcast_ref::<Element>().unwrap();
            assert!(wrap.get_type().is_a(&T::get_type()));
            let ptr: *mut gst_sys::GstElement = wrap.to_glib_none().0;
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();

            gst_panic_to_error!(wrap, &instance.panicked(), fallback(), {
                f(&imp, wrap.unsafe_cast_ref())
            })
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

    fn add_metadata(&mut self, key: &str, value: &str) {
        unsafe {
            gst_sys::gst_element_class_add_metadata(
                self as *mut Self as *mut gst_sys::GstElementClass,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

unsafe impl ElementClassSubclassExt for glib::Class<Element> {}

unsafe impl<T: ElementImpl> IsSubclassable<T> for Element
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::override_vfuncs(klass);
        let klass = klass.as_mut();
        klass.change_state = Some(element_change_state::<T>);
        klass.request_new_pad = Some(element_request_new_pad::<T>);
        klass.release_pad = Some(element_release_pad::<T>);
        klass.send_event = Some(element_send_event::<T>);
        klass.query = Some(element_query::<T>);
        klass.set_context = Some(element_set_context::<T>);
        klass.set_clock = Some(element_set_clock::<T>);
        klass.provide_clock = Some(element_provide_clock::<T>);
        klass.post_message = Some(element_post_message::<T>);
    }
}

unsafe extern "C" fn element_change_state<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
    transition: gst_sys::GstStateChange,
) -> gst_sys::GstStateChangeReturn
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

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
        imp.change_state(wrap.unsafe_cast_ref(), transition).into()
    })
    .to_glib()
}

unsafe extern "C" fn element_request_new_pad<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
    templ: *mut gst_sys::GstPadTemplate,
    name: *const libc::c_char,
    caps: *const gst_sys::GstCaps,
) -> *mut gst_sys::GstPad
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    let caps = Option::<::Caps>::from_glib_borrow(caps);

    // XXX: This is effectively unsafe but the best we can do
    // See https://bugzilla.gnome.org/show_bug.cgi?id=791193
    let pad = gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.request_new_pad(
            wrap.unsafe_cast_ref(),
            &from_glib_borrow(templ),
            from_glib_none(name),
            caps.as_ref().as_ref(),
        )
    });

    // Ensure that the pad is owned by the element now, if a pad was returned
    if let Some(ref pad) = pad {
        assert_eq!(
            pad.get_parent().as_ref(),
            Some(&*::Object::from_glib_borrow(ptr as *mut gst_sys::GstObject))
        );
    }

    pad.to_glib_none().0
}

unsafe extern "C" fn element_release_pad<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
    pad: *mut gst_sys::GstPad,
) where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    // If we get a floating reference passed simply return here. It can't be stored inside this
    // element, and if we continued to use it we would take ownership of this floating reference.
    if gobject_sys::g_object_is_floating(pad as *mut gobject_sys::GObject) != glib_sys::GFALSE {
        return;
    }

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.release_pad(wrap.unsafe_cast_ref(), &from_glib_none(pad))
    })
}

unsafe extern "C" fn element_send_event<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.send_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn element_query<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);
    let query = QueryRef::from_mut_ptr(query);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.query(wrap.unsafe_cast_ref(), query)
    })
    .to_glib()
}

unsafe extern "C" fn element_set_context<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
    context: *mut gst_sys::GstContext,
) where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.set_context(wrap.unsafe_cast_ref(), &from_glib_borrow(context))
    })
}

unsafe extern "C" fn element_set_clock<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
    clock: *mut gst_sys::GstClock,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    let clock = Option::<::Clock>::from_glib_borrow(clock);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.set_clock(wrap.unsafe_cast_ref(), clock.as_ref().as_ref())
    })
    .to_glib()
}

unsafe extern "C" fn element_provide_clock<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
) -> *mut gst_sys::GstClock
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.provide_clock(wrap.unsafe_cast_ref())
    })
    .to_glib_full()
}

unsafe extern "C" fn element_post_message<T: ElementImpl>(
    ptr: *mut gst_sys::GstElement,
    msg: *mut gst_sys::GstMessage,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    // Can't catch panics here as posting the error message would cause
    // this code to be called again recursively forever.
    imp.post_message(wrap.unsafe_cast_ref(), from_glib_full(msg))
        .to_glib()
}

#[cfg(test)]
mod tests {
    use super::*;
    use glib;
    use glib::subclass;
    use std::sync::atomic;

    use ElementFactory;

    pub mod imp {
        use super::*;

        pub struct TestElement {
            pub(super) srcpad: ::Pad,
            pub(super) sinkpad: ::Pad,
            pub(super) n_buffers: atomic::AtomicU32,
            pub(super) reached_playing: atomic::AtomicBool,
        }

        impl TestElement {
            fn sink_chain(
                &self,
                _pad: &::Pad,
                _element: &super::TestElement,
                buffer: ::Buffer,
            ) -> Result<::FlowSuccess, ::FlowError> {
                self.n_buffers.fetch_add(1, atomic::Ordering::SeqCst);
                self.srcpad.push(buffer)
            }

            fn sink_event(
                &self,
                _pad: &::Pad,
                _element: &super::TestElement,
                event: ::Event,
            ) -> bool {
                self.srcpad.push_event(event)
            }

            fn sink_query(
                &self,
                _pad: &::Pad,
                _element: &super::TestElement,
                query: &mut ::QueryRef,
            ) -> bool {
                self.srcpad.peer_query(query)
            }

            fn src_event(
                &self,
                _pad: &::Pad,
                _element: &super::TestElement,
                event: ::Event,
            ) -> bool {
                self.sinkpad.push_event(event)
            }

            fn src_query(
                &self,
                _pad: &::Pad,
                _element: &super::TestElement,
                query: &mut ::QueryRef,
            ) -> bool {
                self.sinkpad.peer_query(query)
            }
        }

        impl ObjectSubclass for TestElement {
            const NAME: &'static str = "TestElement";
            type Type = super::TestElement;
            type ParentType = Element;
            type Instance = ::subclass::ElementInstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            fn with_class(klass: &Self::Class) -> Self {
                let templ = klass.get_pad_template("sink").unwrap();
                let sinkpad = ::Pad::builder_with_template(&templ, Some("sink"))
                    .chain_function(|pad, parent, buffer| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || Err(::FlowError::Error),
                            |identity, element| identity.sink_chain(pad, element, buffer),
                        )
                    })
                    .event_function(|pad, parent, event| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || false,
                            |identity, element| identity.sink_event(pad, element, event),
                        )
                    })
                    .query_function(|pad, parent, query| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || false,
                            |identity, element| identity.sink_query(pad, element, query),
                        )
                    })
                    .build();

                let templ = klass.get_pad_template("src").unwrap();
                let srcpad = ::Pad::builder_with_template(&templ, Some("src"))
                    .event_function(|pad, parent, event| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || false,
                            |identity, element| identity.src_event(pad, element, event),
                        )
                    })
                    .query_function(|pad, parent, query| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || false,
                            |identity, element| identity.src_query(pad, element, query),
                        )
                    })
                    .build();

                Self {
                    n_buffers: atomic::AtomicU32::new(0),
                    reached_playing: atomic::AtomicBool::new(false),
                    srcpad,
                    sinkpad,
                }
            }

            fn class_init(klass: &mut Self::Class) {
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
            fn constructed(&self, element: &Self::Type) {
                self.parent_constructed(element);

                element.add_pad(&self.sinkpad).unwrap();
                element.add_pad(&self.srcpad).unwrap();
            }
        }

        impl ElementImpl for TestElement {
            fn change_state(
                &self,
                element: &Self::Type,
                transition: ::StateChange,
            ) -> Result<::StateChangeSuccess, ::StateChangeError> {
                let res = self.parent_change_state(element, transition)?;

                if transition == ::StateChange::PausedToPlaying {
                    self.reached_playing.store(true, atomic::Ordering::SeqCst);
                }

                Ok(res)
            }
        }
    }

    glib_wrapper! {
        pub struct TestElement(ObjectSubclass<imp::TestElement>) @extends Element, ::Object;
    }

    unsafe impl Send for TestElement {}
    unsafe impl Sync for TestElement {}

    impl TestElement {
        pub fn new(name: Option<&str>) -> Self {
            glib::Object::new(TestElement::static_type(), &[("name", &name)])
                .unwrap()
                .downcast::<Self>()
                .unwrap()
        }
    }

    #[test]
    fn test_element_subclass() {
        ::init().unwrap();

        let element = TestElement::new(Some("test"));

        assert_eq!(element.get_name(), "test");

        assert_eq!(
            element.get_metadata(&::ELEMENT_METADATA_LONGNAME),
            Some("Test Element")
        );

        let pipeline = ::Pipeline::new(None);
        let src = ElementFactory::make("fakesrc", None).unwrap();
        let sink = ElementFactory::make("fakesink", None).unwrap();

        src.set_property("num-buffers", &100i32).unwrap();

        pipeline
            .add_many(&[&src, &element.upcast_ref(), &sink])
            .unwrap();
        Element::link_many(&[&src, &element.upcast_ref(), &sink]).unwrap();

        pipeline.set_state(::State::Playing).unwrap();
        let bus = pipeline.get_bus().unwrap();

        let eos = bus.timed_pop_filtered(::CLOCK_TIME_NONE, &[::MessageType::Eos]);
        assert!(eos.is_some());

        pipeline.set_state(::State::Null).unwrap();

        let imp = imp::TestElement::from_instance(&element);
        assert_eq!(imp.n_buffers.load(atomic::Ordering::SeqCst), 100);
        assert_eq!(imp.reached_playing.load(atomic::Ordering::SeqCst), true);
    }
}
