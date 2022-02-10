use crate::{callsite::GstCallsiteKind, state_desc, PadFlags};
use gstreamer::{
    ffi::{
        gst_debug_add_log_function, gst_debug_category_get_name, gst_debug_message_get,
        gst_debug_remove_log_function, gst_element_get_type, gst_object_get_type, gst_pad_get_type,
        GST_LEVEL_COUNT, GST_LEVEL_DEBUG, GST_LEVEL_ERROR, GST_LEVEL_FIXME, GST_LEVEL_INFO,
        GST_LEVEL_LOG, GST_LEVEL_MEMDUMP, GST_LEVEL_TRACE, GST_LEVEL_WARNING,
    },
    glib::{
        gobject_ffi::{g_type_is_a, g_type_name},
        translate::FromGlib,
    },
};
use libc::{c_char, c_int, c_void};
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
            Some(line as u32),
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
            let gstobject_name = gobject_with_ty.and_then(|(obj, ty)| unsafe {
                // SAFETY: g_type_is_a is provided valid types.
                if bool::from_glib(g_type_is_a(ty, gst_object_get_type())) {
                    let gstobject = obj as *mut gstreamer::ffi::GstObject;
                    // SAFETY: GstObject type has been verified above, `name` can be null and we
                    // check for it. It "should" be valid null-terminated string if not null,
                    // however.
                    Some(CStr::from_ptr((*gstobject).name.as_ref()?).to_string_lossy())
                } else {
                    None
                }
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
            let event = Event::new(meta, &valueset);
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
