use std::cell::RefCell;

use glib::subclass::basic;
use gstreamer::{glib, subclass::prelude::*, Object, Tracer};
use tracing::{Callsite, Id};
use tracing_core::Kind;

pub struct TracingTracerPriv {
    span_stack: thread_local::ThreadLocal<RefCell<Vec<Id>>>,
}

impl TracingTracerPriv {
    fn push_span(&self, id: Id) {
        self.span_stack
            .get_or(|| RefCell::new(Vec::new()))
            .borrow_mut()
            .push(id)
    }
    fn pop_span(&self) -> Option<Id> {
        self.span_stack
            .get_or(|| RefCell::new(Vec::new()))
            .borrow_mut()
            .pop()
    }
    fn current(&self) -> Option<Id> {
        self.span_stack
            .get_or(|| RefCell::new(Vec::new()))
            .borrow_mut()
            .last()
            .cloned()
    }
}

glib::wrapper! {
    pub struct TracingTracer(ObjectSubclass<TracingTracerPriv>)
       @extends Tracer, Object;
}

#[glib::object_subclass]
impl ObjectSubclass for TracingTracerPriv {
    const NAME: &'static str = "TracingTracer";
    type Type = TracingTracer;
    type Class = basic::ClassStruct<Self>;
    type Instance = basic::InstanceStruct<Self>;
    type ParentType = Tracer;
    type Interfaces = ();

    fn new() -> Self {
        Self {
            span_stack: thread_local::ThreadLocal::new(),
        }
    }
}

impl ObjectImpl for TracingTracerPriv {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.register_hook(TracerHook::PadPushPost);
        self.register_hook(TracerHook::PadPushPre);
    }
}

impl TracerImpl for TracingTracerPriv {
    fn pad_push_post(&self, _: u64, _: &gstreamer::Pad, _: gstreamer::FlowReturn) {
        if let Some(id) = self.pop_span() {
            tracing::dispatcher::get_default(|dispatch| {
                dispatch.exit(&id);
                dispatch.try_close(id.clone());
                if let Some(current) = self.current() {
                    dispatch.enter(&current);
                }
            })
        }
    }

    fn pad_push_pre(&self, _: u64, _: &gstreamer::Pad, _: &gstreamer::Buffer) {
        let callsite = crate::callsite::DynamicCallsites::get().callsite_for(
            tracing::Level::ERROR,
            "pad_push",
            "pad_push",
            None,
            None,
            None,
            Kind::SPAN,
            &[],
        );
        let interest = callsite.interest();
        if interest.is_never() {
            return;
        }
        let meta = callsite.metadata();
        tracing_core::dispatcher::get_default(move |dispatch| {
            if !dispatch.enabled(meta) {
                return;
            }
            let fields = meta.fields();
            let values = [];
            let valueset = fields.value_set(&values);
            let attrs = tracing::span::Attributes::new(meta, &valueset);
            let span_id = dispatch.new_span(&attrs);
            if let Some(current) = self.current() {
                dispatch.record_follows_from(&span_id, &current);
                dispatch.exit(&current);
            }
            dispatch.enter(&span_id);
            self.push_span(span_id);
        });
    }
}
