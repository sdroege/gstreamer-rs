// Take a look at the license at the top of the repository in the LICENSE file.

use std::{borrow::Cow, future::Future, sync::atomic};

use glib::{subclass::prelude::*, translate::*};

use super::prelude::*;
use crate::{
    prelude::*, Element, Event, PadTemplate, QueryRef, StateChange, StateChangeError,
    StateChangeReturn, StateChangeSuccess,
};

#[derive(Debug, Clone)]
pub struct ElementMetadata {
    long_name: Cow<'static, str>,
    classification: Cow<'static, str>,
    description: Cow<'static, str>,
    author: Cow<'static, str>,
    additional: Cow<'static, [(Cow<'static, str>, Cow<'static, str>)]>,
}

impl ElementMetadata {
    pub fn new(long_name: &str, classification: &str, description: &str, author: &str) -> Self {
        Self {
            long_name: Cow::Owned(long_name.into()),
            classification: Cow::Owned(classification.into()),
            description: Cow::Owned(description.into()),
            author: Cow::Owned(author.into()),
            additional: Cow::Borrowed(&[]),
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
            long_name: Cow::Owned(long_name.into()),
            classification: Cow::Owned(classification.into()),
            description: Cow::Owned(description.into()),
            author: Cow::Owned(author.into()),
            additional: additional
                .iter()
                .copied()
                .map(|(key, value)| (Cow::Owned(key.into()), Cow::Owned(value.into())))
                .collect(),
        }
    }

    pub const fn with_cow(
        long_name: Cow<'static, str>,
        classification: Cow<'static, str>,
        description: Cow<'static, str>,
        author: Cow<'static, str>,
        additional: Cow<'static, [(Cow<'static, str>, Cow<'static, str>)]>,
    ) -> Self {
        Self {
            long_name,
            classification,
            description,
            author,
            additional,
        }
    }
}

pub trait ElementImpl: ElementImplExt + GstObjectImpl + Send + Sync {
    fn metadata() -> Option<&'static ElementMetadata> {
        None
    }

    fn pad_templates() -> &'static [PadTemplate] {
        &[]
    }

    fn change_state(
        &self,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        self.parent_change_state(transition)
    }

    fn request_new_pad(
        &self,
        templ: &crate::PadTemplate,
        name: Option<&str>,
        caps: Option<&crate::Caps>,
    ) -> Option<crate::Pad> {
        self.parent_request_new_pad(templ, name, caps)
    }

    fn release_pad(&self, pad: &crate::Pad) {
        self.parent_release_pad(pad)
    }

    fn send_event(&self, event: Event) -> bool {
        self.parent_send_event(event)
    }

    fn query(&self, query: &mut QueryRef) -> bool {
        self.parent_query(query)
    }

    fn set_context(&self, context: &crate::Context) {
        self.parent_set_context(context)
    }

    fn set_clock(&self, clock: Option<&crate::Clock>) -> bool {
        self.parent_set_clock(clock)
    }

    fn provide_clock(&self) -> Option<crate::Clock> {
        self.parent_provide_clock()
    }

    fn post_message(&self, msg: crate::Message) -> bool {
        self.parent_post_message(msg)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ElementImplExt> Sealed for T {}
}

pub trait ElementImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_change_state(
        &self,
        transition: StateChange,
    ) -> Result<StateChangeSuccess, StateChangeError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            let f = (*parent_class)
                .change_state
                .expect("Missing parent function `change_state`");
            try_from_glib(f(
                self.obj().unsafe_cast_ref::<Element>().to_glib_none().0,
                transition.into_glib(),
            ))
        }
    }

    fn parent_request_new_pad(
        &self,
        templ: &crate::PadTemplate,
        name: Option<&str>,
        caps: Option<&crate::Caps>,
    ) -> Option<crate::Pad> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .request_new_pad
                .map(|f| {
                    from_glib_none(f(
                        self.obj().unsafe_cast_ref::<Element>().to_glib_none().0,
                        templ.to_glib_none().0,
                        name.to_glib_full(),
                        caps.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_release_pad(&self, pad: &crate::Pad) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .release_pad
                .map(|f| {
                    f(
                        self.obj().unsafe_cast_ref::<Element>().to_glib_none().0,
                        pad.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_send_event(&self, event: Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .send_event
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<Element>().to_glib_none().0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .query
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<Element>().to_glib_none().0,
                        query.as_mut_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_set_context(&self, context: &crate::Context) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .set_context
                .map(|f| {
                    f(
                        self.obj().unsafe_cast_ref::<Element>().to_glib_none().0,
                        context.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_set_clock(&self, clock: Option<&crate::Clock>) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .set_clock
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<Element>().to_glib_none().0,
                        clock.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_provide_clock(&self) -> Option<crate::Clock> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            (*parent_class)
                .provide_clock
                .map(|f| {
                    from_glib_none(f(self.obj().unsafe_cast_ref::<Element>().to_glib_none().0))
                })
                .unwrap_or(None)
        }
    }

    fn parent_post_message(&self, msg: crate::Message) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstElementClass;

            if let Some(f) = (*parent_class).post_message {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Element>().to_glib_none().0,
                    msg.into_glib_ptr(),
                ))
            } else {
                false
            }
        }
    }

    #[inline(never)]
    fn panicked(&self) -> &atomic::AtomicBool {
        self.instance_data::<atomic::AtomicBool>(crate::Element::static_type())
            .expect("instance not initialized correctly")
    }

    fn catch_panic<R, F: FnOnce(&Self) -> R, G: FnOnce() -> R>(&self, fallback: G, f: F) -> R {
        panic_to_error!(self, fallback(), { f(self) })
    }

    fn catch_panic_future<R, F: FnOnce() -> R, G: Future<Output = R>>(
        &self,
        fallback: F,
        fut: G,
    ) -> CatchPanic<Self, F, G> {
        CatchPanic {
            self_: self.ref_counted().downgrade(),
            fallback: Some(fallback),
            fut,
        }
    }

    fn catch_panic_pad_function<R, F: FnOnce(&Self) -> R, G: FnOnce() -> R>(
        parent: Option<&crate::Object>,
        fallback: G,
        f: F,
    ) -> R {
        let element = parent.unwrap().dynamic_cast_ref::<Self::Type>().unwrap();
        let imp = element.imp();

        panic_to_error!(imp, fallback(), { f(imp) })
    }

    fn post_error_message(&self, msg: crate::ErrorMessage) {
        unsafe {
            self.obj()
                .unsafe_cast_ref::<Element>()
                .post_error_message(msg)
        }
    }
}

impl<T: ElementImpl> ElementImplExt for T {}

pin_project_lite::pin_project! {
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct CatchPanic<T: glib::subclass::types::ObjectSubclass, F, G> {
        self_: glib::subclass::ObjectImplWeakRef<T>,
        fallback: Option<F>,
        #[pin]
        fut: G,
    }
}

impl<R, T: ElementImpl, F: FnOnce() -> R, G: Future<Output = R>> Future for CatchPanic<T, F, G> {
    type Output = R;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();

        let Some(self_) = this.self_.upgrade() else {
            return std::task::Poll::Ready((this
                .fallback
                .take()
                .expect("Future polled after resolving"))(
            ));
        };

        panic_to_error!(
            &*self_,
            std::task::Poll::Ready(this.fallback.take().expect("Future polled after resolving")()),
            {
                let fut = this.fut;
                fut.poll(cx)
            }
        )
    }
}

unsafe impl<T: ElementImpl> IsSubclassable<T> for Element {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
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

                for (key, value) in &metadata.additional[..] {
                    ffi::gst_element_class_add_metadata(
                        klass,
                        key.to_glib_none().0,
                        value.to_glib_none().0,
                    );
                }
            }
        }
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        Self::parent_instance_init::<T>(instance);

        instance.set_instance_data(Self::static_type(), atomic::AtomicBool::new(false));
    }
}

unsafe extern "C" fn element_change_state<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    transition: ffi::GstStateChange,
) -> ffi::GstStateChangeReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    // *Never* fail downwards state changes, this causes bugs in GStreamer
    // and leads to crashes and deadlocks.
    let transition = from_glib(transition);
    let fallback = match transition {
        StateChange::PlayingToPaused | StateChange::PausedToReady | StateChange::ReadyToNull => {
            StateChangeReturn::Success
        }
        _ => StateChangeReturn::Failure,
    };

    panic_to_error!(imp, fallback, { imp.change_state(transition).into() }).into_glib()
}

unsafe extern "C" fn element_request_new_pad<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    templ: *mut ffi::GstPadTemplate,
    name: *const libc::c_char,
    caps: *const ffi::GstCaps,
) -> *mut ffi::GstPad {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let caps = Option::<crate::Caps>::from_glib_borrow(caps);
    let name = Option::<String>::from_glib_none(name);

    // XXX: This is effectively unsafe but the best we can do
    // See https://bugzilla.gnome.org/show_bug.cgi?id=791193
    let pad = panic_to_error!(imp, None, {
        imp.request_new_pad(
            &from_glib_borrow(templ),
            name.as_deref(),
            caps.as_ref().as_ref(),
        )
    });

    // Ensure that the pad is owned by the element now, if a pad was returned
    if let Some(ref pad) = pad {
        assert_eq!(
            pad.parent().as_ref(),
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
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    // If we get a floating reference passed simply return here. It can't be stored inside this
    // element, and if we continued to use it we would take ownership of this floating reference.
    if glib::gobject_ffi::g_object_is_floating(pad as *mut glib::gobject_ffi::GObject)
        != glib::ffi::GFALSE
    {
        return;
    }

    panic_to_error!(imp, (), { imp.release_pad(&from_glib_none(pad)) })
}

unsafe extern "C" fn element_send_event<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    event: *mut ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    panic_to_error!(imp, false, { imp.send_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn element_query<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    query: *mut ffi::GstQuery,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let query = QueryRef::from_mut_ptr(query);

    panic_to_error!(imp, false, { imp.query(query) }).into_glib()
}

unsafe extern "C" fn element_set_context<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    context: *mut ffi::GstContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    panic_to_error!(imp, (), { imp.set_context(&from_glib_borrow(context)) })
}

unsafe extern "C" fn element_set_clock<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    clock: *mut ffi::GstClock,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let clock = Option::<crate::Clock>::from_glib_borrow(clock);

    panic_to_error!(imp, false, { imp.set_clock(clock.as_ref().as_ref()) }).into_glib()
}

unsafe extern "C" fn element_provide_clock<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
) -> *mut ffi::GstClock {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    panic_to_error!(imp, None, { imp.provide_clock() }).into_glib_ptr()
}

unsafe extern "C" fn element_post_message<T: ElementImpl>(
    ptr: *mut ffi::GstElement,
    msg: *mut ffi::GstMessage,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    // Can't catch panics here as posting the error message would cause
    // this code to be called again recursively forever.
    imp.post_message(from_glib_full(msg)).into_glib()
}

#[cfg(test)]
mod tests {
    use std::sync::atomic;

    use super::*;
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
                buffer: crate::Buffer,
            ) -> Result<crate::FlowSuccess, crate::FlowError> {
                self.n_buffers.fetch_add(1, atomic::Ordering::SeqCst);
                self.srcpad.push(buffer)
            }

            fn sink_event(&self, _pad: &crate::Pad, event: crate::Event) -> bool {
                self.srcpad.push_event(event)
            }

            fn sink_query(&self, _pad: &crate::Pad, query: &mut crate::QueryRef) -> bool {
                self.srcpad.peer_query(query)
            }

            fn src_event(&self, _pad: &crate::Pad, event: crate::Event) -> bool {
                self.sinkpad.push_event(event)
            }

            fn src_query(&self, _pad: &crate::Pad, query: &mut crate::QueryRef) -> bool {
                self.sinkpad.peer_query(query)
            }
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestElement {
            const NAME: &'static str = "TestElement";
            type Type = super::TestElement;
            type ParentType = Element;

            fn with_class(klass: &Self::Class) -> Self {
                let templ = klass.pad_template("sink").unwrap();
                let sinkpad = crate::Pad::builder_from_template(&templ)
                    .chain_function(|pad, parent, buffer| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || Err(crate::FlowError::Error),
                            |identity| identity.sink_chain(pad, buffer),
                        )
                    })
                    .event_function(|pad, parent, event| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || false,
                            |identity| identity.sink_event(pad, event),
                        )
                    })
                    .query_function(|pad, parent, query| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || false,
                            |identity| identity.sink_query(pad, query),
                        )
                    })
                    .build();

                let templ = klass.pad_template("src").unwrap();
                let srcpad = crate::Pad::builder_from_template(&templ)
                    .event_function(|pad, parent, event| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || false,
                            |identity| identity.src_event(pad, event),
                        )
                    })
                    .query_function(|pad, parent, query| {
                        TestElement::catch_panic_pad_function(
                            parent,
                            || false,
                            |identity| identity.src_query(pad, query),
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
            fn constructed(&self) {
                self.parent_constructed();

                let element = self.obj();
                element.add_pad(&self.sinkpad).unwrap();
                element.add_pad(&self.srcpad).unwrap();
            }
        }

        impl GstObjectImpl for TestElement {}

        impl ElementImpl for TestElement {
            fn metadata() -> Option<&'static ElementMetadata> {
                static ELEMENT_METADATA: std::sync::OnceLock<ElementMetadata> =
                    std::sync::OnceLock::new();

                Some(ELEMENT_METADATA.get_or_init(|| {
                    ElementMetadata::new(
                        "Test Element",
                        "Generic",
                        "Does nothing",
                        "Sebastian Dröge <sebastian@centricular.com>",
                    )
                }))
            }

            fn pad_templates() -> &'static [PadTemplate] {
                static PAD_TEMPLATES: std::sync::OnceLock<Vec<PadTemplate>> =
                    std::sync::OnceLock::new();

                PAD_TEMPLATES.get_or_init(|| {
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
                })
            }

            fn change_state(
                &self,
                transition: crate::StateChange,
            ) -> Result<crate::StateChangeSuccess, crate::StateChangeError> {
                let res = self.parent_change_state(transition)?;

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

    impl TestElement {
        pub fn new(name: Option<&str>) -> Self {
            glib::Object::builder().property("name", name).build()
        }
    }

    #[test]
    fn test_element_subclass() {
        crate::init().unwrap();

        let element = TestElement::new(Some("test"));

        assert_eq!(element.name(), "test");

        assert_eq!(
            element.metadata(crate::ELEMENT_METADATA_LONGNAME),
            Some("Test Element")
        );

        let pipeline = crate::Pipeline::new();
        let src = ElementFactory::make("fakesrc")
            .property("num-buffers", 100i32)
            .build()
            .unwrap();
        let sink = ElementFactory::make("fakesink").build().unwrap();

        pipeline
            .add_many([&src, element.upcast_ref(), &sink])
            .unwrap();
        Element::link_many([&src, element.upcast_ref(), &sink]).unwrap();

        pipeline.set_state(crate::State::Playing).unwrap();
        let bus = pipeline.bus().unwrap();

        let eos = bus.timed_pop_filtered(crate::ClockTime::NONE, &[crate::MessageType::Eos]);
        assert!(eos.is_some());

        pipeline.set_state(crate::State::Null).unwrap();

        let imp = element.imp();
        assert_eq!(imp.n_buffers.load(atomic::Ordering::SeqCst), 100);
        assert!(imp.reached_playing.load(atomic::Ordering::SeqCst));
    }
}
