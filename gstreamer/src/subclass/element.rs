// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use crate::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use crate::Element;
use crate::Event;
use crate::PadTemplate;
use crate::QueryRef;
use crate::StateChange;
use crate::StateChangeError;
use crate::StateChangeReturn;
use crate::StateChangeSuccess;

#[derive(Debug, Clone)]
pub struct ElementMetadata {
    long_name: String,
    classification: String,
    description: String,
    author: String,
    additional: Vec<(String, String)>,
}

impl ElementMetadata {
    pub fn new(long_name: &str, classification: &str, description: &str, author: &str) -> Self {
        Self {
            long_name: long_name.into(),
            classification: classification.into(),
            description: description.into(),
            author: author.into(),
            additional: vec![],
        }
    }

    pub fn with_additional(
        long_name: &str,
        classification: &str,
        description: &str,
        author: &str,
        additional: &[(&str, &str)],
    ) -> Self {
        Self {
            long_name: long_name.into(),
            classification: classification.into(),
            description: description.into(),
            author: author.into(),
            additional: additional
                .iter()
                .copied()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        }
    }
}

pub trait ElementImpl: ElementImplExt + ObjectImpl + Send + Sync {
    fn metadata() -> Option<&'static ElementMetadata> {
        None
    }

    fn pad_templates() -> &'static [PadTemplate] {
        &[]
    }

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
        templ: &crate::PadTemplate,
        name: Option<String>,
        caps: Option<&crate::Caps>,
    ) -> Option<crate::Pad> {
        self.parent_request_new_pad(element, templ, name, caps)
    }

    fn release_pad(&self, element: &Self::Type, pad: &crate::Pad) {
        self.parent_release_pad(element, pad)
    }

    fn send_event(&self, element: &Self::Type, event: Event) -> bool {
        self.parent_send_event(element, event)
    }

    fn query(&self, element: &Self::Type, query: &mut QueryRef) -> bool {
        self.parent_query(element, query)
    }

    fn set_context(&self, element: &Self::Type, context: &crate::Context) {
        self.parent_set_context(element, context)
    }

    fn set_clock(&self, element: &Self::Type, clock: Option<&crate::Clock>) -> bool {
        self.parent_set_clock(element, clock)
    }

    fn provide_clock(&self, element: &Self::Type) -> Option<crate::Clock> {
        self.parent_provide_clock(element)
    }

    fn post_message(&self, element: &Self::Type, msg: crate::Message) -> bool {
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
        templ: &crate::PadTemplate,
        name: Option<String>,
        caps: Option<&crate::Caps>,
    ) -> Option<crate::Pad>;

    fn parent_release_pad(&self, element: &Self::Type, pad: &crate::Pad);

    fn parent_send_event(&self, element: &Self::Type, event: Event) -> bool;

    fn parent_query(&self, element: &Self::Type, query: &mut QueryRef) -> bool;

    fn parent_set_context(&self, element: &Self::Type, context: &crate::Context);

    fn parent_set_clock(&self, element: &Self::Type, clock: Option<&crate::Clock>) -> bool;

    fn parent_provide_clock(&self, element: &Self::Type) -> Option<crate::Clock>;

    fn parent_post_message(&self, element: &Self::Type, msg: crate::Message) -> bool;

    fn catch_panic<R, F: FnOnce(&Self) -> R, G: FnOnce() -> R, P: IsA<Element>>(
        &self,
        element: &P,
        fallback: G,
        f: F,
    ) -> R;

    fn catch_panic_pad_function<R, F: FnOnce(&Self, &Self::Type) -> R, G: FnOnce() -> R>(
        parent: Option<&crate::Object>,
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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

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
        templ: &crate::PadTemplate,
        name: Option<String>,
        caps: Option<&crate::Caps>,
    ) -> Option<crate::Pad> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

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

    fn parent_release_pad(&self, element: &Self::Type, pad: &crate::Pad) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

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

    fn parent_set_context(&self, element: &Self::Type, context: &crate::Context) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

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

    fn parent_set_clock(&self, element: &Self::Type, clock: Option<&crate::Clock>) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

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

    fn parent_provide_clock(&self, element: &Self::Type) -> Option<crate::Clock> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .provide_clock
                .map(|f| from_glib_none(f(element.unsafe_cast_ref::<Element>().to_glib_none().0)))
                .unwrap_or(None)
        }
    }

    fn parent_post_message(&self, element: &Self::Type, msg: crate::Message) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstElementClass;

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
            let ptr: *mut ffi::GstElement = element.as_ptr() as *mut _;
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();

            panic_to_error!(element, &instance.panicked(), fallback(), { f(&imp) })
        }
    }

    fn catch_panic_pad_function<R, F: FnOnce(&Self, &Self::Type) -> R, G: FnOnce() -> R>(
        parent: Option<&crate::Object>,
        fallback: G,
        f: F,
    ) -> R {
        unsafe {
            let wrap = parent.as_ref().unwrap().downcast_ref::<Element>().unwrap();
            assert!(wrap.get_type().is_a(&T::get_type()));
            let ptr: *mut ffi::GstElement = wrap.to_glib_none().0;
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();

            panic_to_error!(wrap, &instance.panicked(), fallback(), {
                f(&imp, wrap.unsafe_cast_ref())
            })
        }
    }
}

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

        unsafe {
            for pad_template in T::pad_templates() {
                ffi::gst_element_class_add_pad_template(klass, pad_template.to_glib_none().0);
            }

            if let Some(metadata) = T::metadata() {
                ffi::gst_element_class_set_metadata(
                    klass,
                    metadata.long_name.to_glib_none().0,
                    metadata.classification.to_glib_none().0,
                    metadata.description.to_glib_none().0,
                    metadata.author.to_glib_none().0,
                );

                for (key, value) in &metadata.additional {
                    ffi::gst_element_class_add_metadata(
                        klass,
                        key.to_glib_none().0,
                        value.to_glib_none().0,
                    );
                }
            }
        }
    }
}

unsafe extern "C" fn element_change_state<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    transition: ffi::GstStateChange,
) -> ffi::GstStateChangeReturn
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

    panic_to_error!(&wrap, &instance.panicked(), fallback, {
        imp.change_state(wrap.unsafe_cast_ref(), transition).into()
    })
    .to_glib()
}

unsafe extern "C" fn element_request_new_pad<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    templ: *mut ffi::GstPadTemplate,
    name: *const libc::c_char,
    caps: *const ffi::GstCaps,
) -> *mut ffi::GstPad
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    let caps = Option::<crate::Caps>::from_glib_borrow(caps);

    // XXX: This is effectively unsafe but the best we can do
    // See https://bugzilla.gnome.org/show_bug.cgi?id=791193
    let pad = panic_to_error!(&wrap, &instance.panicked(), None, {
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
            Some(&*crate::Object::from_glib_borrow(
                ptr as *mut ffi::GstObject
            ))
        );
    }

    pad.to_glib_none().0
}

unsafe extern "C" fn element_release_pad<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    pad: *mut ffi::GstPad,
) where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    // If we get a floating reference passed simply return here. It can't be stored inside this
    // element, and if we continued to use it we would take ownership of this floating reference.
    if glib::gobject_ffi::g_object_is_floating(pad as *mut glib::gobject_ffi::GObject)
        != glib::ffi::GFALSE
    {
        return;
    }

    panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.release_pad(wrap.unsafe_cast_ref(), &from_glib_none(pad))
    })
}

unsafe extern "C" fn element_send_event<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    event: *mut ffi::GstEvent,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.send_event(wrap.unsafe_cast_ref(), from_glib_full(event))
    })
    .to_glib()
}

unsafe extern "C" fn element_query<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    query: *mut ffi::GstQuery,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);
    let query = QueryRef::from_mut_ptr(query);

    panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.query(wrap.unsafe_cast_ref(), query)
    })
    .to_glib()
}

unsafe extern "C" fn element_set_context<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    context: *mut ffi::GstContext,
) where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.set_context(wrap.unsafe_cast_ref(), &from_glib_borrow(context))
    })
}

unsafe extern "C" fn element_set_clock<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    clock: *mut ffi::GstClock,
) -> glib::ffi::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    let clock = Option::<crate::Clock>::from_glib_borrow(clock);

    panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.set_clock(wrap.unsafe_cast_ref(), clock.as_ref().as_ref())
    })
    .to_glib()
}

unsafe extern "C" fn element_provide_clock<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
) -> *mut ffi::GstClock
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Element> = from_glib_borrow(ptr);

    panic_to_error!(&wrap, &instance.panicked(), None, {
        imp.provide_clock(wrap.unsafe_cast_ref())
    })
    .to_glib_full()
}

unsafe extern "C" fn element_post_message<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    msg: *mut ffi::GstMessage,
) -> glib::ffi::gboolean
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
    use glib::subclass;
    use std::sync::atomic;

    use crate::ElementFactory;

    pub mod imp {
        use super::*;

        pub struct TestElement {
            pub(super) srcpad: crate::Pad,
            pub(super) sinkpad: crate::Pad,
            pub(super) n_buffers: atomic::AtomicU32,
            pub(super) reached_playing: atomic::AtomicBool,
        }

        impl TestElement {
            fn sink_chain(
                &self,
                _pad: &crate::Pad,
                _element: &super::TestElement,
                buffer: crate::Buffer,
            ) -> Result<crate::FlowSuccess, crate::FlowError> {
                self.n_buffers.fetch_add(1, atomic::Ordering::SeqCst);
                self.srcpad.push(buffer)
            }

            fn sink_event(
                &self,
                _pad: &crate::Pad,
                _element: &super::TestElement,
                event: crate::Event,
            ) -> bool {
                self.srcpad.push_event(event)
            }

            fn sink_query(
                &self,
                _pad: &crate::Pad,
                _element: &super::TestElement,
                query: &mut crate::QueryRef,
            ) -> bool {
                self.srcpad.peer_query(query)
            }

            fn src_event(
                &self,
                _pad: &crate::Pad,
                _element: &super::TestElement,
                event: crate::Event,
            ) -> bool {
                self.sinkpad.push_event(event)
            }

            fn src_query(
                &self,
                _pad: &crate::Pad,
                _element: &super::TestElement,
                query: &mut crate::QueryRef,
            ) -> bool {
                self.sinkpad.peer_query(query)
            }
        }

        impl ObjectSubclass for TestElement {
            const NAME: &'static str = "TestElement";
            type Type = super::TestElement;
            type ParentType = Element;
            type Interfaces = ();
            type Instance = crate::subclass::ElementInstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib::object_subclass!();

            fn with_class(klass: &Self::Class) -> Self {
                let templ = klass.get_pad_template("sink").unwrap();
                let sinkpad = crate::Pad::builder_with_template(&templ, Some("sink"))
                    .chain_function(|pad, parent, buffer| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || Err(crate::FlowError::Error),
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
                let srcpad = crate::Pad::builder_with_template(&templ, Some("src"))
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
        }

        impl ObjectImpl for TestElement {
            fn constructed(&self, element: &Self::Type) {
                self.parent_constructed(element);

                element.add_pad(&self.sinkpad).unwrap();
                element.add_pad(&self.srcpad).unwrap();
            }
        }

        impl ElementImpl for TestElement {
            fn metadata() -> Option<&'static ElementMetadata> {
                use once_cell::sync::Lazy;
                static ELEMENT_METADATA: Lazy<ElementMetadata> = Lazy::new(|| {
                    ElementMetadata::new(
                        "Test Element",
                        "Generic",
                        "Does nothing",
                        "Sebastian Dröge <sebastian@centricular.com>",
                    )
                });

                Some(&*ELEMENT_METADATA)
            }

            fn pad_templates() -> &'static [PadTemplate] {
                use once_cell::sync::Lazy;
                static PAD_TEMPLATES: Lazy<Vec<PadTemplate>> = Lazy::new(|| {
                    let caps = crate::Caps::new_any();
                    vec![
                        PadTemplate::new(
                            "src",
                            crate::PadDirection::Src,
                            crate::PadPresence::Always,
                            &caps,
                        )
                        .unwrap(),
                        PadTemplate::new(
                            "sink",
                            crate::PadDirection::Sink,
                            crate::PadPresence::Always,
                            &caps,
                        )
                        .unwrap(),
                    ]
                });

                PAD_TEMPLATES.as_ref()
            }

            fn change_state(
                &self,
                element: &Self::Type,
                transition: crate::StateChange,
            ) -> Result<crate::StateChangeSuccess, crate::StateChangeError> {
                let res = self.parent_change_state(element, transition)?;

                if transition == crate::StateChange::PausedToPlaying {
                    self.reached_playing.store(true, atomic::Ordering::SeqCst);
                }

                Ok(res)
            }
        }
    }

    glib::wrapper! {
        pub struct TestElement(ObjectSubclass<imp::TestElement>) @extends Element, crate::Object;
    }

    unsafe impl Send for TestElement {}
    unsafe impl Sync for TestElement {}

    impl TestElement {
        pub fn new(name: Option<&str>) -> Self {
            glib::Object::new(&[("name", &name)]).unwrap()
        }
    }

    #[test]
    fn test_element_subclass() {
        crate::init().unwrap();

        let element = TestElement::new(Some("test"));

        assert_eq!(element.get_name(), "test");

        assert_eq!(
            element.get_metadata(&crate::ELEMENT_METADATA_LONGNAME),
            Some("Test Element")
        );

        let pipeline = crate::Pipeline::new(None);
        let src = ElementFactory::make("fakesrc", None).unwrap();
        let sink = ElementFactory::make("fakesink", None).unwrap();

        src.set_property("num-buffers", &100i32).unwrap();

        pipeline
            .add_many(&[&src, &element.upcast_ref(), &sink])
            .unwrap();
        Element::link_many(&[&src, &element.upcast_ref(), &sink]).unwrap();

        pipeline.set_state(crate::State::Playing).unwrap();
        let bus = pipeline.get_bus().unwrap();

        let eos = bus.timed_pop_filtered(crate::CLOCK_TIME_NONE, &[crate::MessageType::Eos]);
        assert!(eos.is_some());

        pipeline.set_state(crate::State::Null).unwrap();

        let imp = imp::TestElement::from_instance(&element);
        assert_eq!(imp.n_buffers.load(atomic::Ordering::SeqCst), 100);
        assert_eq!(imp.reached_playing.load(atomic::Ordering::SeqCst), true);
    }
}
