use std::cell::RefCell;

use glib::subclass::basic;
use gstreamer::{
    glib,
    prelude::PadExtManual,
    subclass::prelude::*,
    traits::{GstObjectExt, PadExt},
    Buffer, FlowReturn, Object, Pad, Query, Tracer,
};
use tracing::{dispatcher, span::Attributes, Callsite, Dispatch, Id};
use tracing_core::Kind;

struct EnteredSpan {
    id: Id,
    dispatch: Dispatch,
}

pub struct TracingTracerPriv {
    span_stack: thread_local::ThreadLocal<RefCell<Vec<EnteredSpan>>>,
}

impl TracingTracerPriv {
    fn push_span(&self, dispatch: Dispatch, attributes: Attributes) {
        let span_id = dispatch.new_span(&attributes);
        dispatch.enter(&span_id);
        self.span_stack
            .get_or(|| RefCell::new(Vec::new()))
            .borrow_mut()
            .push(EnteredSpan {
                id: span_id,
                dispatch,
            })
    }
    fn pop_span(&self) -> Option<()> {
        self.span_stack
            .get_or(|| RefCell::new(Vec::new()))
            .borrow_mut()
            .pop()
            .map(|span| {
                span.dispatch.exit(&span.id);
                span.dispatch.try_close(span.id);
            })
    }

    fn pad_pre(&self, name: &'static str, pad: &Pad) {
        let callsite = crate::callsite::DynamicCallsites::get().callsite_for(
            tracing::Level::ERROR,
            name,
            name,
            None,
            None,
            None,
            Kind::SPAN,
            &["gstpad.state", "gstpad.parent.name"],
        );
        let interest = callsite.interest();
        if interest.is_never() {
            return;
        }
        let meta = callsite.metadata();
        let dispatch = tracing_core::dispatcher::get_default(move |dispatch| dispatch.clone());
        if !dispatch.enabled(meta) {
            return;
        }
        let gstpad_flags_value = Some(tracing_core::field::display(crate::PadFlags(
            pad.pad_flags().bits(),
        )));
        let gstpad_parent = pad.parent_element();
        let gstpad_parent_name_value = gstpad_parent.map(|p| p.name());
        let gstpad_parent_name_value = gstpad_parent_name_value.as_ref().map(|n| n.as_str());
        let fields = meta.fields();
        let mut fields_iter = fields.into_iter();
        let values = field_values![fields_iter =>
            // /!\ /!\ /!\ Must be in the same order as the field list above /!\ /!\ /!\
            "gstpad.flags" = gstpad_flags_value;
            "gstpad.parent.name" = gstpad_parent_name_value;
        ];
        let valueset = fields.value_set(&values);
        let attrs = tracing::span::Attributes::new_root(meta, &valueset);
        self.push_span(dispatch, attrs);
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
        self.register_hook(TracerHook::PadPushListPost);
        self.register_hook(TracerHook::PadPushListPre);
        self.register_hook(TracerHook::PadQueryPost);
        self.register_hook(TracerHook::PadQueryPre);
        self.register_hook(TracerHook::PadPushEventPost);
        self.register_hook(TracerHook::PadPushEventPre);
        self.register_hook(TracerHook::PadPullRangePost);
        self.register_hook(TracerHook::PadPullRangePre);
    }
}

impl TracerImpl for TracingTracerPriv {
    fn pad_push_pre(&self, _: u64, pad: &Pad, _: &Buffer) {
        self.pad_pre("pad_push", pad);
    }

    fn pad_push_list_pre(&self, _: u64, pad: &Pad, _: &gstreamer::BufferList) {
        self.pad_pre("pad_push_list", pad);
    }

    fn pad_query_pre(&self, _: u64, pad: &Pad, _: &Query) {
        self.pad_pre("pad_query", pad);
    }

    fn pad_push_event_pre(&self, _: u64, pad: &Pad, _: &gstreamer::Event) {
        self.pad_pre("pad_event", pad);
    }

    fn pad_pull_range_pre(&self, _: u64, pad: &Pad, _: u64, _: u32) {
        self.pad_pre("pad_pull_range", pad);
    }

    fn pad_pull_range_post(&self, _: u64, _: &Pad, _: &Buffer, _: FlowReturn) {
        self.pop_span();
    }

    fn pad_push_event_post(&self, _: u64, _: &Pad, _: bool) {
        self.pop_span();
    }

    fn pad_push_list_post(&self, _: u64, _: &Pad, _: FlowReturn) {
        self.pop_span();
    }

    fn pad_push_post(&self, _: u64, _: &Pad, _: FlowReturn) {
        self.pop_span();
    }

    fn pad_query_post(&self, _: u64, _: &Pad, _: &Query, _: bool) {
        self.pop_span();
    }
}
