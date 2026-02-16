use crate::callsite::GstCallsiteKind;
use crate::log::span_quark;
use gst::{Buffer, FlowError, FlowSuccess, Pad, Tracer, glib, prelude::*, subclass::prelude::*};
use std::{cell::RefCell, str::FromStr};
use tracing::{Callsite, Dispatch, Id, error, info, span::Attributes};

struct EnteredSpan {
    id: Id,
    dispatch: Dispatch,
}

pub struct TracingTracer {
    span_stack: thread_local::ThreadLocal<RefCell<Vec<EnteredSpan>>>,
}

pub struct SpanPropagationTracer;

unsafe fn propagate_attached_span(parent: &gst::Object, child: &gst::Object) {
    let quark = *span_quark();
    if let Some(span) = unsafe { parent.qdata::<tracing::Span>(quark) } {
        unsafe {
            child.set_qdata(quark, span.as_ref().clone());
        }
    }
}

impl TracingTracer {
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
            GstCallsiteKind::Span,
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
        let gstpad_flags_value = Some(tracing_core::field::display(pad.pad_flags()));
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

#[glib::object_subclass]
impl ObjectSubclass for TracingTracer {
    const NAME: &'static str = "GstRsTracingTracer";
    const ALLOW_NAME_CONFLICT: bool = true;
    type Type = super::TracingTracer;
    type ParentType = Tracer;

    fn new() -> Self {
        Self {
            span_stack: thread_local::ThreadLocal::new(),
        }
    }
}

impl ObjectImpl for TracingTracer {
    fn constructed(&self) {
        if let Some(params) = self.obj().property::<Option<String>>("params") {
            let tmp = format!("params,{params}");
            info!("{:?} params: {:?}", self.obj(), tmp);
            let structure = gst::Structure::from_str(&tmp).unwrap_or_else(|e| {
                error!("Invalid params string: {:?}: {e:?}", tmp);
                gst::Structure::new_empty("params")
            });

            if let Ok(gst_logs_level) = structure
                .get::<String>("log-level")
                .or_else(|_| structure.get::<i32>("log-level").map(|l| l.to_string()))
            {
                info!("Integrating `{gst_logs_level}` GStreamer logs as part of our tracing");

                crate::integrate_events();
                gst::log::remove_default_log_function();
                gst::log::set_threshold_from_string(&gst_logs_level, true);
            }
        }

        self.parent_constructed();
        #[cfg(feature = "v1_30")]
        self.register_hook(TracerHook::ObjectParentSet);
        #[cfg(not(feature = "v1_30"))]
        {
            self.register_hook(TracerHook::ElementAddPad);
            self.register_hook(TracerHook::BinAddPost);
        }
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

impl GstObjectImpl for TracingTracer {}

impl TracerImpl for TracingTracer {
    const USE_STRUCTURE_PARAMS: bool = true;

    #[cfg(not(feature = "v1_30"))]
    fn element_add_pad(&self, _ts: u64, element: &gst::Element, pad: &gst::Pad) {
        unsafe {
            propagate_attached_span(element.upcast_ref(), pad.upcast_ref());
        }
    }

    #[cfg(not(feature = "v1_30"))]
    fn bin_add_post(&self, _ts: u64, bin: &gst::Bin, element: &gst::Element, _success: bool) {
        unsafe {
            propagate_attached_span(bin.upcast_ref(), element.upcast_ref());
        }
    }

    #[cfg(feature = "v1_30")]
    fn object_parent_set(&self, _ts: u64, obj: &gst::Object, parent: Option<&gst::Object>) {
        if let Some(parent) = parent {
            unsafe { propagate_attached_span(parent, obj) }
        }
    }

    fn pad_push_pre(&self, _: u64, pad: &Pad, _: &Buffer) {
        self.pad_pre("pad_push", pad);
    }

    fn pad_push_list_pre(&self, _: u64, pad: &Pad, _: &gst::BufferList) {
        self.pad_pre("pad_push_list", pad);
    }

    fn pad_query_pre(&self, _: u64, pad: &Pad, _: &gst::QueryRef) {
        self.pad_pre("pad_query", pad);
    }

    fn pad_push_event_pre(&self, _: u64, pad: &Pad, _: &gst::Event) {
        self.pad_pre("pad_event", pad);
    }

    fn pad_pull_range_pre(&self, _: u64, pad: &Pad, _: u64, _: u32) {
        self.pad_pre("pad_pull_range", pad);
    }

    fn pad_pull_range_post(&self, _: u64, _: &Pad, _: Result<&Buffer, FlowError>) {
        self.pop_span();
    }

    fn pad_push_event_post(&self, _: u64, _: &Pad, _: bool) {
        self.pop_span();
    }

    fn pad_push_list_post(&self, _: u64, _: &Pad, _: Result<FlowSuccess, FlowError>) {
        self.pop_span();
    }

    fn pad_push_post(&self, _: u64, _: &Pad, _: Result<FlowSuccess, FlowError>) {
        self.pop_span();
    }

    fn pad_query_post(&self, _: u64, _: &Pad, _: &gst::QueryRef, _: bool) {
        self.pop_span();
    }
}

impl super::TracingTracerImpl for TracingTracer {}

#[glib::object_subclass]
impl ObjectSubclass for SpanPropagationTracer {
    const NAME: &'static str = "GstRsSpanPropagationTracer";
    const ALLOW_NAME_CONFLICT: bool = true;
    type Type = super::SpanPropagationTracer;
    type ParentType = Tracer;

    fn new() -> Self {
        Self
    }
}

impl ObjectImpl for SpanPropagationTracer {
    fn constructed(&self) {
        self.parent_constructed();
        #[cfg(feature = "v1_30")]
        self.register_hook(TracerHook::ObjectParentSet);
        #[cfg(not(feature = "v1_30"))]
        {
            self.register_hook(TracerHook::ElementAddPad);
            self.register_hook(TracerHook::BinAddPost);
        }
    }
}

impl GstObjectImpl for SpanPropagationTracer {}

impl TracerImpl for SpanPropagationTracer {
    #[cfg(not(feature = "v1_30"))]
    fn element_add_pad(&self, _ts: u64, element: &gst::Element, pad: &gst::Pad) {
        unsafe {
            propagate_attached_span(element.upcast_ref(), pad.upcast_ref());
        }
    }

    #[cfg(not(feature = "v1_30"))]
    fn bin_add_post(&self, _ts: u64, bin: &gst::Bin, element: &gst::Element, _success: bool) {
        unsafe {
            propagate_attached_span(bin.upcast_ref(), element.upcast_ref());
        }
    }

    #[cfg(feature = "v1_30")]
    fn object_parent_set(&self, _ts: u64, obj: &gst::Object, parent: Option<&gst::Object>) {
        if let Some(parent) = parent {
            unsafe { propagate_attached_span(parent, obj) }
        }
    }
}
