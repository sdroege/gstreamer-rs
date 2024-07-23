use crate::{callsite::GstCallsiteKind, state_desc, PadFlags};
use gstreamer::{
    ffi::{
        gst_debug_add_log_function, gst_debug_category_get_name, gst_debug_message_get,
        gst_debug_remove_log_function, gst_element_get_type, gst_object_get_type, gst_pad_get_type,
        GST_LEVEL_COUNT, GST_LEVEL_DEBUG, GST_LEVEL_ERROR, GST_LEVEL_FIXME, GST_LEVEL_INFO,
        GST_LEVEL_LOG, GST_LEVEL_MEMDUMP, GST_LEVEL_TRACE, GST_LEVEL_WARNING,
    },
    glib::{
        gobject_ffi::{g_object_get_qdata, g_type_is_a, g_type_name},
        translate::{FromGlib, IntoGlib},
    },
    prelude::{IsA, ObjectExt},
};
use libc::{c_char, c_int, c_void};
use once_cell::sync::OnceCell;
use std::{convert::TryFrom, ffi::CStr};
use tracing_core::{Callsite, Event, Level};

unsafe extern "C" fn log_callback(
    category: *mut gstreamer::ffi::GstDebugCategory,
    level: gstreamer::ffi::GstDebugLevel,
    file: *const c_char,
    module: *const c_char,
    line: c_int,
    gobject: *mut gstreamer::glib::gobject_ffi::GObject,
    message: *mut gstreamer::ffi::GstDebugMessage,
    _: *mut c_void,
) {
    // SAFETY: unwinding back into C land is UB. We abort if there was a panic inside.
    std::panic::catch_unwind(move || {
        let level = match level {
            GST_LEVEL_ERROR => Level::ERROR,
            GST_LEVEL_WARNING | GST_LEVEL_FIXME => Level::WARN,
            GST_LEVEL_INFO => Level::INFO,
            GST_LEVEL_DEBUG | GST_LEVEL_LOG => Level::DEBUG,
            GST_LEVEL_TRACE | GST_LEVEL_MEMDUMP | GST_LEVEL_COUNT => Level::TRACE,
            _ => return,
        };
        // Take extra care to make sure nothing sketchy is going on.
        if category.is_null() || message.is_null() {
            return;
        }
        let file = unsafe {
            // SAFETY: Users of the GStreamer `gst_debug_log` API are required to pass in a
            // null terminated string.
            CStr::from_ptr(file.as_ref().expect("`file` string is nullptr"))
        }
        .to_string_lossy();
        let module = unsafe {
            // SAFETY: Users of the GStreamer `gst_debug_log` API are required to pass in a
            // null terminated string.
            CStr::from_ptr(module.as_ref().expect("`function` string is nullptr"))
        }
        .to_string_lossy();
        let line = u32::try_from(line).expect("`line` is not a valid u32");
        let category_name = unsafe {
            // SAFETY: Users of the GStreamer `gst_debug_log` API are required to pass in a
            // valid `GstDebugCategory`. `gst_debug_category_get_name` shall return a valid
            // null terminated string.
            CStr::from_ptr(
                gst_debug_category_get_name(category)
                    .as_ref()
                    .expect("`category` has no name?"),
            )
        }
        .to_string_lossy();
        let callsite = crate::callsite::DynamicCallsites::get().callsite_for(
            level,
            "",
            &category_name,
            Some(&file),
            Some(&module),
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
            let message = unsafe {
                // SAFETY: This function has no soundness invariants. It can return a null pointer,
                // which we handle by `as_ref`. A valid pointer to a null-terminated C string must
                // be returned.
                gst_debug_message_get(message)
                    .as_ref()
                    .map(|v| CStr::from_ptr(v).to_string_lossy())
            };
            let message_value = message.as_deref();
            let gobject = unsafe {
                // SAFETY: we check for null pointer before we dereference it. While the object
                // contents might not otherwise be super defined while ref_count is 0, reading the
                // ref_count itself here should be good, still.
                if gobject.is_null() || (*gobject).ref_count == 0 {
                    None
                } else {
                    Some(gobject)
                }
            };
            let gobject_address_value = gobject.map(|obj| obj as usize);
            let gobject_with_ty = gobject.and_then(|obj| unsafe {
                Some((obj, obj.as_ref()?.g_type_instance.g_class.as_ref()?.g_type))
            });
            let gobject_type_value = gobject_with_ty.and_then(|(_, ty)| unsafe {
                Some(
                    // SAFETY: The returned type name may be nullptr, and we check for it with
                    // `as_ref`. The returned string is guaranteed to be valid otherwise.
                    CStr::from_ptr(g_type_name(ty).as_ref()?).to_string_lossy(),
                )
            });
            let gobject_type_value = gobject_type_value.as_deref();
            let gstobject = gobject_with_ty.and_then(|(obj, ty)| unsafe {
                // SAFETY: g_type_is_a is provided valid types.
                if bool::from_glib(g_type_is_a(ty, gst_object_get_type())) {
                    let gstobject = obj as *mut gstreamer::ffi::GstObject;

                    Some(gstobject)
                } else {
                    None
                }
            });

            let gstobject_name = gstobject.as_ref().and_then(|gstobject| unsafe {
                // SAFETY: GstObject type has been verified above, `name` can be null and we
                // check for it. It "should" be valid null-terminated string if not null,
                // however.
                Some(CStr::from_ptr((*(*gstobject)).name.as_ref()?).to_string_lossy())
            });

            let user_span = gstobject.as_ref().and_then(|gstobject| unsafe {
                let quark = span_quark().into_glib();
                let mut obj = *gstobject;

                let span = loop {
                    // lookup object parents until one has a span associated
                    let span = g_object_get_qdata(obj.cast(), quark);

                    if span.is_null() {
                        // do not call gst_object_get_parent() as it gets the OBJECT which could be already hold by the caller of the log
                        obj = (*obj).parent;
                        if obj.is_null() {
                            break std::ptr::null();
                        }
                    } else {
                        break span;
                    }
                };

                let span = span.cast::<tracing::Span>().as_ref();

                span
            });

            let gstobject_name_value = gstobject_name.as_deref();
            let gstelement = gobject_with_ty.and_then(|(obj, ty)| unsafe {
                // SAFETY: g_type_is_a is provided valid types.
                if bool::from_glib(g_type_is_a(ty, gst_element_get_type())) {
                    Some(obj as *mut gstreamer::ffi::GstElement)
                } else {
                    None
                }
            });
            let gstelement_states = gstelement.map(|e| unsafe {
                let (curr, pend) = ((*e).current_state, (*e).pending_state);
                (state_desc(curr), state_desc(pend))
            });
            let gstelement_state_value = gstelement_states.map(|(c, _)| c);
            let gstelement_pending_state_value = gstelement_states.map(|(_, p)| p);
            let gstpad = gobject_with_ty.and_then(|(obj, ty)| unsafe {
                // SAFETY: g_type_is_a is provided valid types.
                if bool::from_glib(g_type_is_a(ty, gst_pad_get_type())) {
                    Some(obj as *mut gstreamer::ffi::GstPad)
                } else {
                    None
                }
            });
            let gstpad_flags = gstpad.map(|p| unsafe {
                // SAFETY: `p` is a valid pointer.
                tracing_core::field::display(PadFlags((*p).object.flags))
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
                Some(CStr::from_ptr((*obj).name.as_ref()?).to_string_lossy())
            });
            let gstpad_parent_name_value = gstpad_parent_name.as_deref();

            let gstpad_parent_states = gstpad_parent.and_then(|obj| unsafe {
                let ty = (*obj).object.g_type_instance.g_class.as_ref()?.g_type;
                if bool::from_glib(g_type_is_a(ty, gst_element_get_type())) {
                    let e = obj as *mut gstreamer::ffi::GstElement;
                    let (curr, pend) = ((*e).current_state, (*e).pending_state);
                    Some((state_desc(curr), state_desc(pend)))
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
    })
    .unwrap_or_else(|_e| std::process::abort());
}

pub(crate) fn debug_add_log_function() {
    unsafe {
        // SAFETY: this function has no soundness invariants.
        gst_debug_add_log_function(Some(log_callback), std::ptr::null_mut(), None);
    }
}

pub(crate) fn debug_remove_log_function() {
    unsafe {
        // SAFETY: this function has no soundness invariants.
        gst_debug_remove_log_function(Some(log_callback));
    }
}

#[inline]
fn span_quark() -> &'static gstreamer::glib::Quark {
    static ELEMENT_SPAN_QUARK: OnceCell<gstreamer::glib::Quark> = OnceCell::new();
    // Generate a unique TypeId specifically for Span quark’s name. This gives some probabilistic
    // security against users of this library overwriting our span with their own types, making
    // attach_span unsound.
    //
    // We’re still going to be storing `tracing::Span` within the objects directly, because that’s
    // just more convenient.
    #[allow(dead_code)]
    struct QDataTracingSpan(tracing::Span);

    ELEMENT_SPAN_QUARK.get_or_init(|| {
        let type_id = std::any::TypeId::of::<QDataTracingSpan>();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&type_id, &mut hasher);
        let type_id_hash = std::hash::Hasher::finish(&hasher);
        let key = format!("tracing-gstreamer:{}\0", type_id_hash);
        let gstr = gstreamer::glib::GStr::from_utf8_with_nul(key.as_bytes()).unwrap();
        gstreamer::glib::Quark::from_str(gstr)
    })
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
/// gstreamer::init().unwrap();
///
/// let pipeline = gstreamer::Pipeline::new();
/// let gst_log_span = tracing::info_span!(
///     parent: None,
///     "gst log",
///     pipeline = "encoding",
///     id = 42,
/// );
/// unsafe {
///     tracing_gstreamer::attach_span(&pipeline, gst_log_span);
/// }
/// ```
pub unsafe fn attach_span<O: IsA<gstreamer::Object>>(object: &O, span: tracing::Span) {
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
