use crate::callsite::GstCallsiteKind;
use crate::log::span_quark;
use gst::glib::Properties;
use gst::{Buffer, FlowError, FlowSuccess, Pad, Tracer, glib, prelude::*, subclass::prelude::*};
#[cfg(feature = "v1_30")]
use smallvec::SmallVec;
use std::{cell::RefCell, collections::HashMap, sync::Mutex};
use tracing::{Callsite, Dispatch, Id, info, span::Attributes};
#[cfg(feature = "v1_30")]
use tracing_core::field::Value;

struct EnteredSpan {
    id: Id,
    dispatch: Dispatch,
}

#[cfg(feature = "v1_30")]
struct SpanFormat {
    callsite: &'static crate::callsite::GstCallsite,
}

#[cfg(feature = "v1_30")]
enum FieldValue<'a> {
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    Str(&'a str),
    Text(String),
}

#[cfg(feature = "v1_30")]
impl FieldValue<'_> {
    // `tracing::Value` is sealed, so we can't implement it for the enum; instead
    // hand out a reference to the inner value, whose concrete type already is a
    // `Value` (bool/i64/u64/f64/&str/String).
    fn as_value(&self) -> &dyn Value {
        match self {
            FieldValue::Bool(v) => v,
            FieldValue::I64(v) => v,
            FieldValue::U64(v) => v,
            FieldValue::F64(v) => v,
            FieldValue::Str(v) => v,
            FieldValue::Text(v) => v,
        }
    }
}

#[cfg(feature = "v1_30")]
fn span_field_value(value: Option<gst::TraceValue<'_>>) -> FieldValue<'_> {
    use gst::TraceValue::*;
    match value {
        Some(Boolean(v)) => FieldValue::Bool(v),
        Some(Int(v)) => FieldValue::I64(i64::from(v)),
        Some(Uint(v)) => FieldValue::U64(u64::from(v)),
        Some(Int64(v)) => FieldValue::I64(v),
        Some(Uint64(v)) => FieldValue::U64(v),
        Some(Double(v)) => FieldValue::F64(v),
        Some(String(v)) => FieldValue::Str(v.map(|s| s.as_str()).unwrap_or("<null>")),
        Some(Structure(v)) => FieldValue::Text(
            v.map(|s| s.to_string())
                .unwrap_or_else(|| "<null>".to_string()),
        ),
        Some(Object(v)) => FieldValue::Text(
            v.map(|o| format!("{o:?}"))
                .unwrap_or_else(|| "<null>".to_string()),
        ),
        None => FieldValue::Str("<unknown>"),
    }
}

#[derive(Default)]
struct Settings {
    log_level: Option<String>,
}

#[derive(Properties)]
#[properties(wrapper_type = super::TracingTracer)]
pub struct TracingTracer {
    span_stack: thread_local::ThreadLocal<RefCell<Vec<Option<EnteredSpan>>>>,
    #[cfg(feature = "v1_30")]
    span_formats: Mutex<HashMap<usize, SpanFormat>>,
    #[cfg(feature = "v1_30")]
    spans: Mutex<HashMap<gst::TraceSpanId, EnteredSpan>>,
    #[property(
        name = "log-level",
        get,
        set = Self::set_log_level,
        type = Option<String>,
        member = log_level,
        blurb = "GStreamer log level to integrate as tracing events",
    )]
    settings: Mutex<Settings>,
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
    fn set_log_level(&self, log_level: Option<String>) {
        let mut settings = self.settings.lock().unwrap();
        settings.log_level = log_level.clone();
        if let Some(ref level) = log_level {
            info!("Integrating `{level}` GStreamer logs as part of our tracing");
            crate::integrate_events();
            gst::log::remove_default_log_function();
            gst::log::set_threshold_from_string(level, true);
        }
    }

    // Every *_pre hook pushes exactly one stack entry and every *_post hook
    // pops exactly one, so the stack stays balanced even when a callsite is
    // disabled and no span is created (None is pushed). Without this, a
    // disabled _pre followed by an unconditional _post would pop an unrelated
    // (outer) span.
    fn push_span(&self, dispatch: Dispatch, attributes: Attributes) {
        let span_id = dispatch.new_span(&attributes);
        dispatch.enter(&span_id);
        self.push_entry(Some(EnteredSpan {
            id: span_id,
            dispatch,
        }));
    }
    fn push_no_span(&self) {
        self.push_entry(None);
    }
    fn push_entry(&self, span: Option<EnteredSpan>) {
        self.span_stack
            .get_or(|| RefCell::new(Vec::new()))
            .borrow_mut()
            .push(span);
    }
    fn pop_span(&self) {
        if let Some(Some(span)) = self
            .span_stack
            .get_or(|| RefCell::new(Vec::new()))
            .borrow_mut()
            .pop()
        {
            span.dispatch.exit(&span.id);
            span.dispatch.try_close(span.id);
        }
    }

    #[cfg(feature = "v1_30")]
    fn span_format(&self, format: gst::TraceFormat) -> SpanFormat {
        let span_name = format.name();
        let n_fields = format.n_fields();
        let mut field_names = Vec::with_capacity(n_fields);

        // The interned callsite needs a &'static [&'static str], so both the
        // names and the slice are leaked. Bounded, not growing: memoised per
        // format (see caller), and formats live until gst_deinit().
        for field in format.fields() {
            field_names.push(Box::leak(field.name().to_owned().into_boxed_str()) as &'static str);
        }

        let field_names = Box::leak(field_names.into_boxed_slice());
        let callsite = crate::callsite::DynamicCallsites::get().callsite_for(
            tracing::Level::INFO,
            span_name,
            span_name,
            None,
            None,
            None,
            GstCallsiteKind::Span,
            field_names,
        );

        SpanFormat { callsite }
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
            self.push_no_span();
            return;
        }
        let meta = callsite.metadata();
        let dispatch = tracing_core::dispatcher::get_default(move |dispatch| dispatch.clone());
        if !dispatch.enabled(meta) {
            self.push_no_span();
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
            #[cfg(feature = "v1_30")]
            span_formats: Mutex::new(HashMap::new()),
            #[cfg(feature = "v1_30")]
            spans: Mutex::new(HashMap::new()),
            settings: Mutex::new(Settings::default()),
        }
    }
}

#[glib::derived_properties]
impl ObjectImpl for TracingTracer {
    fn constructed(&self) {
        self.parent_constructed();
        #[cfg(feature = "v1_30")]
        self.register_hook(TracerHook::ObjectParentSet);
        #[cfg(feature = "v1_30")]
        {
            self.register_hook(TracerHook::SpanBegin);
            self.register_hook(TracerHook::SpanEnd);
        }
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

    #[cfg(feature = "v1_30")]
    fn span_begin(&self, _ts: u64, span_id: gst::TraceSpanId, values: gst::TraceValues<'_>) {
        let format = values.format();
        let format_key = format.as_ptr_id();
        let mut formats = self.span_formats.lock().unwrap();
        let custom_format = formats
            .entry(format_key)
            .or_insert_with(|| self.span_format(format));

        let callsite = custom_format.callsite;
        let interest = callsite.interest();
        if interest.is_never() {
            return;
        }

        let meta = callsite.metadata();
        let dispatch = tracing_core::dispatcher::get_default(move |dispatch| dispatch.clone());
        if !dispatch.enabled(meta) {
            return;
        }

        let field_values = values
            .iter()
            .map(span_field_value)
            .collect::<SmallVec<[FieldValue; 8]>>();
        drop(formats);

        // Positional values, one per field, matching the callsite's field set.
        let values = field_values
            .iter()
            .map(|v| Some(v.as_value()))
            .collect::<SmallVec<[Option<&dyn Value>; 8]>>();
        let valueset = meta.fields().value_set_all(&values);
        let attrs = tracing::span::Attributes::new_root(meta, &valueset);
        // Custom spans may begin and end on different threads, so map them to
        // the span lifetime (new_span/try_close) rather than the thread-local
        // enter/exit scope. The perfetto layer (async flavor) emits the slice
        // from new_span/close accordingly.
        let span = EnteredSpan {
            id: dispatch.new_span(&attrs),
            dispatch,
        };
        self.spans.lock().unwrap().insert(span_id, span);
    }

    #[cfg(feature = "v1_30")]
    fn span_end(&self, _ts: u64, span_id: gst::TraceSpanId) {
        if let Some(span) = self.spans.lock().unwrap().remove(&span_id) {
            span.dispatch.try_close(span.id);
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
