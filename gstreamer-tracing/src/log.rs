use crate::callsite::GstCallsiteKind;
use gst::{
    glib::{GStr, translate::*},
    prelude::*,
};
use std::sync::{LazyLock, Mutex};
use tracing_core::{Callsite, Event, Level};

fn log_handler(
    category: gst::DebugCategory,
    level: gst::DebugLevel,
    file: &gst::glib::GStr,
    module: &gst::glib::GStr,
    line: u32,
    object: Option<&gst::LoggedObject>,
    message: &gst::DebugMessage,
) {
    let level = match level {
        gst::DebugLevel::Error => Level::ERROR,
        gst::DebugLevel::Warning | gst::DebugLevel::Fixme => Level::WARN,
        gst::DebugLevel::Info => Level::INFO,
        gst::DebugLevel::Debug | gst::DebugLevel::Log => Level::DEBUG,
        gst::DebugLevel::Trace | gst::DebugLevel::Memdump => Level::TRACE,
        _ => return,
    };
    let category_name = category.name();
    let callsite = crate::callsite::DynamicCallsites::get().callsite_for(
        level,
        "",
        category_name,
        Some(file.as_str()),
        Some(module.as_str()),
        Some(line),
        GstCallsiteKind::Event,
        &[
            "message",
            "gobject.address",
            "gobject.type",
            "gstobject.name",
            "gstelement.state",
            "gstelement.pending_state",
            "gstpad.state",
            "gstpad.parent.name",
            "gstpad.parent.state",
            "gstpad.parent.pending_state",
        ],
    );
    let interest = callsite.interest();
    if interest.is_never() {
        return;
    }
    let meta = callsite.metadata();
    tracing_core::dispatcher::get_default(move |dispatcher| {
        if !dispatcher.enabled(meta) {
            return;
        }
        let fields = meta.fields();
        let message_str = message.get();
        let message_value = message_str.as_deref().map(|g| g.as_str());

        let gobject = object.map(|o| o.as_ptr());
        let gobject = unsafe {
            // SAFETY: we check for null pointer before we dereference it. While the object
            // contents might not otherwise be super defined while ref_count is 0, reading the
            // ref_count itself here should be good, still.
            gobject.and_then(|ptr| {
                if (*ptr).ref_count == 0 {
                    None
                } else {
                    Some(ptr)
                }
            })
        };
        let gobject_address_value = gobject.map(|obj| obj as usize);
        let gobject_with_ty = gobject.and_then(|obj| unsafe {
            let ty: gst::glib::Type =
                from_glib(obj.as_ref()?.g_type_instance.g_class.as_ref()?.g_type);
            Some((obj, ty))
        });
        let gobject_type_value = gobject_with_ty.as_ref().map(|(_, ty)| ty.name());
        let gstobject = gobject_with_ty.and_then(|(obj, ty)| {
            if ty.is_a(gst::Object::static_type()) {
                Some(obj as *mut gst::ffi::GstObject)
            } else {
                None
            }
        });

        let gstobject_name = gstobject.as_ref().and_then(|gstobject| unsafe {
            // SAFETY: GstObject type has been verified above, `name` can be null and we
            // check for it. It "should" be valid null-terminated string if not null,
            // however.
            Some(GStr::from_ptr((*(*gstobject)).name.as_ref()?).as_str())
        });

        let user_span = gstobject.as_ref().and_then(|gstobject| unsafe {
            let quark = *span_quark();
            let obj: gst::Object = ref_gst_object(*gstobject);
            obj.qdata::<tracing::Span>(quark)
                .map(|s| s.as_ref().clone())
        });

        let gstobject_name_value = gstobject_name;
        let gstelement = gobject_with_ty.as_ref().and_then(|(obj, ty)| {
            if ty.is_a(gst::Element::static_type()) {
                Some(*obj as *mut gst::ffi::GstElement)
            } else {
                None
            }
        });
        let gstelement_states = gstelement.map(|e| unsafe {
            let curr: gst::State = from_glib((*e).current_state);
            let pend: gst::State = from_glib((*e).pending_state);
            (curr.name().as_str(), pend.name().as_str())
        });
        let gstelement_state_value = gstelement_states.map(|(c, _)| c);
        let gstelement_pending_state_value = gstelement_states.map(|(_, p)| p);
        let gstpad = gobject_with_ty.as_ref().and_then(|(obj, ty)| {
            if ty.is_a(gst::Pad::static_type()) {
                Some(*obj as *mut gst::ffi::GstPad)
            } else {
                None
            }
        });
        let gstpad_flags = gstpad.map(|p| unsafe {
            // SAFETY: `p` is a valid pointer.
            let flags = gst::PadFlags::from_bits_truncate((*p).object.flags);
            tracing_core::field::display(flags)
        });
        let gstpad_parent = gstpad.and_then(|p| unsafe {
            // SAFETY: `p` is a valid pointer.
            let parent = (*p).object.parent;
            if parent.is_null() || (*parent).object.ref_count == 0 {
                None
            } else {
                Some(parent)
            }
        });
        let gstpad_parent_name = gstpad_parent.and_then(|obj| unsafe {
            //SAFETY: same as for gstelement_name above.
            Some(GStr::from_ptr((*obj).name.as_ref()?).as_str())
        });
        let gstpad_parent_name_value = gstpad_parent_name;

        let gstpad_parent_states = gstpad_parent.and_then(|obj| unsafe {
            let ty: gst::glib::Type =
                from_glib((*obj).object.g_type_instance.g_class.as_ref()?.g_type);
            if ty.is_a(gst::Element::static_type()) {
                let e = obj as *mut gst::ffi::GstElement;
                let curr: gst::State = from_glib((*e).current_state);
                let pend: gst::State = from_glib((*e).pending_state);
                Some((curr.name().as_str(), pend.name().as_str()))
            } else {
                None
            }
        });
        let gstpad_parent_state_value = gstpad_parent_states.map(|(c, _)| c);
        let gstpad_parent_pending_state_value = gstpad_parent_states.map(|(_, p)| p);
        let mut fields_iter = fields.into_iter();
        let values = field_values![fields_iter =>
            // /!\ /!\ /!\ Must be in the same order as the field list above /!\ /!\ /!\
            "message" = message_value;
            "gobject.address" = gobject_address_value;
            "gobject.type" = gobject_type_value;
            "gstobject.name" = gstobject_name_value;
            "gstelement.state" = gstelement_state_value;
            "gstelement.pending_state" = gstelement_pending_state_value;
            "gstpad.flags" = gstpad_flags;
            "gstpad.parent.name" = gstpad_parent_name_value;
            "gstpad.parent.state" = gstpad_parent_state_value;
            "gstpad.parent.pending_state" = gstpad_parent_pending_state_value;
        ];
        let valueset = fields.value_set(&values);

        let event = match user_span {
            Some(user_span) => Event::new_child_of(user_span, meta, &valueset),
            None => Event::new(meta, &valueset),
        };

        dispatcher.event(&event);
    });
}

static LOG_FUNCTION: Mutex<Option<gst::log::DebugLogFunction>> = Mutex::new(None);

pub(crate) fn debug_add_log_function() {
    let handle = gst::log::add_log_function(log_handler);
    *LOG_FUNCTION.lock().unwrap() = Some(handle);
}

pub(crate) fn debug_remove_log_function() {
    if let Some(handle) = LOG_FUNCTION.lock().unwrap().take() {
        gst::log::remove_log_function(handle);
    }
}

/// Like `from_glib_none` but uses `g_object_ref` instead of `g_object_ref_sink`,
/// so it does not sink floating references. This is necessary because the log
/// callback can fire during object construction when the object still has a
/// floating ref — sinking it would cause finalization on drop.
unsafe fn ref_gst_object(ptr: *mut gst::ffi::GstObject) -> gst::Object {
    unsafe {
        gst::glib::gobject_ffi::g_object_ref(ptr as *mut gst::glib::gobject_ffi::GObject);
        from_glib_full(ptr)
    }
}

#[inline]
pub(crate) fn span_quark() -> &'static gst::glib::Quark {
    // Generate a unique TypeId specifically for Span quark's name. This gives some probabilistic
    // security against users of this library overwriting our span with their own types, making
    // attach_span unsound.
    //
    // We're still going to be storing `tracing::Span` within the objects directly, because that's
    // just more convenient.
    #[allow(dead_code)]
    struct QDataTracingSpan(tracing::Span);

    static ELEMENT_SPAN_QUARK: LazyLock<gst::glib::Quark> = LazyLock::new(|| {
        let type_id = std::any::TypeId::of::<QDataTracingSpan>();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&type_id, &mut hasher);
        let type_id_hash = std::hash::Hasher::finish(&hasher);
        let key = format!("tracing-gstreamer:{type_id_hash}\0");
        let gstr = GStr::from_utf8_with_nul(key.as_bytes()).unwrap();
        gst::glib::Quark::from_str(gstr)
    });

    &ELEMENT_SPAN_QUARK
}

/// Attach a span to a GStreamer object.
///
/// All events logged for this object and its children will have the provided span as the parent.
/// This can be used to associate extra context to a pipeline for example.
///
/// # Safety
///
/// This function is racy and must not be called when an element or any of its children may want to
/// emit a log message. It is generally safe to call this function during early initialization of a
/// pipeline, or when the code in calling this has exclusive control of the elements involved.
///
/// # Examples
///
/// ```
/// # use gstreamer_tracing as gst_tracing;
/// gst::init().unwrap();
///
/// let pipeline = gst::Pipeline::new();
/// let gst_log_span = tracing::info_span!(
///     parent: None,
///     "gst log",
///     pipeline = "encoding",
///     id = 42,
/// );
/// unsafe {
///     gst_tracing::attach_span(&pipeline, gst_log_span);
/// }
/// ```
pub unsafe fn attach_span<O: IsA<gst::Object>>(object: &O, span: tracing::Span) {
    if unsafe { gst::ffi::gst_is_initialized() == gst::glib::ffi::GTRUE } {
        static INIT_PROPAGATION_TRACER: std::sync::Once = std::sync::Once::new();
        INIT_PROPAGATION_TRACER.call_once(|| {
            // Keep the propagation tracer alive for the rest of the process.
            let tracer = gst::glib::Object::new::<crate::tracer::SpanPropagationTracer>();
            std::mem::forget(tracer);
        });
    }

    unsafe {
        // SAFETY:
        //
        // **Type safety**: We have given our best shot at making sure that no other random piece
        // of code, either our own, or any other, interacting with this crate, overwrites our
        // `qdata` with their own code. In that sense the only thing that might get stored is a
        // `tracing::Span`.
        //
        // **Datarace safety**: TODO: this function can be still called in a loop in a separate
        // thread to introduce a data race with a function that reads this data out.
        object.set_qdata(*span_quark(), span);
    }
}
