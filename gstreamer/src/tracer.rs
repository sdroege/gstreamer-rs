// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[cfg(feature = "v1_30")]
use std::{ffi::CStr, num::NonZeroU64, ptr};

#[cfg(feature = "v1_30")]
use crate::StructureRef;
use crate::{Plugin, Tracer};

impl Tracer {
    #[doc(alias = "gst_tracer_register")]
    pub fn register(
        plugin: Option<&Plugin>,
        name: &str,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib::result_from_gboolean!(
                crate::ffi::gst_tracer_register(
                    plugin.to_glib_none().0,
                    name.to_glib_none().0,
                    type_.into_glib()
                ),
                "Failed to register tracer factory"
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// Identifier of an open custom trace span, as delivered to the
/// `TracerImpl::span_begin` / `span_end` hooks.
///
/// It wraps a [`NonZeroU64`], so a `TraceSpanId` always refers to a real span:
/// the C value `0` ("no span") is delivered as `None`, and `Option<TraceSpanId>`
/// stays the same size as the raw id.
#[cfg(feature = "v1_30")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[doc(alias = "GstTraceSpanId")]
pub struct TraceSpanId(NonZeroU64);

#[cfg(feature = "v1_30")]
impl TraceSpanId {
    #[inline]
    pub fn get(self) -> u64 {
        self.0.get()
    }

    // rustdoc-stripper-ignore-next
    /// A raw id of `0` means "no span" and maps to `None`; every non-zero id is
    /// a live span.
    #[inline]
    pub(crate) fn from_glib(span_id: crate::ffi::GstTraceSpanId) -> Option<Self> {
        skip_assert_initialized!();
        NonZeroU64::new(span_id).map(Self)
    }
}

#[cfg(feature = "v1_30")]
#[repr(transparent)]
#[doc(alias = "GstTraceFormat")]
#[derive(Clone, Copy)]
pub struct TraceFormat(ptr::NonNull<crate::ffi::GstTraceFormat>);

#[cfg(feature = "v1_30")]
unsafe impl Send for TraceFormat {}
#[cfg(feature = "v1_30")]
unsafe impl Sync for TraceFormat {}

#[cfg(feature = "v1_30")]
impl std::fmt::Debug for TraceFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TraceFormat")
            .field("name", &self.name())
            .field("description", &self.description())
            .field(
                "fields",
                &self.fields().map(|field| field.name()).collect::<Vec<_>>(),
            )
            .finish()
    }
}

#[cfg(feature = "v1_30")]
impl TraceFormat {
    #[inline]
    pub(crate) unsafe fn from_glib_borrow(ptr: *mut crate::ffi::GstTraceFormat) -> Self {
        skip_assert_initialized!();
        debug_assert!(!ptr.is_null());
        Self(unsafe { ptr::NonNull::new_unchecked(ptr) })
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&self) -> *mut crate::ffi::GstTraceFormat {
        self.0.as_ptr()
    }

    // rustdoc-stripper-ignore-next
    /// Stable opaque pointer identity, suitable as a hash-map key while the
    /// format lives (it is valid until `gst_deinit()`).
    #[inline]
    pub fn as_ptr_id(&self) -> usize {
        self.0.as_ptr() as usize
    }

    #[doc(alias = "gst_trace_format_get_name")]
    pub fn name(&self) -> &str {
        unsafe {
            let ptr = crate::ffi::gst_trace_format_get_name(self.as_mut_ptr());
            if ptr.is_null() {
                return "";
            }
            CStr::from_ptr(ptr).to_str().unwrap_or("")
        }
    }

    #[doc(alias = "gst_trace_format_get_description")]
    pub fn description(&self) -> Option<&str> {
        unsafe {
            let ptr = crate::ffi::gst_trace_format_get_description(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "gst_trace_format_get_n_fields")]
    pub fn n_fields(&self) -> usize {
        unsafe { crate::ffi::gst_trace_format_get_n_fields(self.as_mut_ptr()) as usize }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the name of the field at `index`, or `None` if `index` is out
    /// of range (the C getter returns NULL past the last field).
    #[doc(alias = "gst_trace_format_get_field_name")]
    pub(crate) fn field_name(&self, index: usize) -> Option<&str> {
        unsafe {
            let ptr = crate::ffi::gst_trace_format_get_field_name(self.as_mut_ptr(), index as u32);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the type of the field at `index`. Iterate via
    /// [`TraceFormat::fields`] to stay in range; the C getter falls back to
    /// `Boolean` for an out-of-range index.
    #[doc(alias = "gst_trace_format_get_field_type")]
    pub(crate) fn field_type(&self, index: usize) -> crate::TracerFieldType {
        unsafe {
            from_glib(crate::ffi::gst_trace_format_get_field_type(
                self.as_mut_ptr(),
                index as u32,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the field's description, or `None` if no description was set or
    /// `index` is out of range.
    #[doc(alias = "gst_trace_format_get_field_description")]
    pub(crate) fn field_description(&self, index: usize) -> Option<&str> {
        unsafe {
            let ptr =
                crate::ffi::gst_trace_format_get_field_description(self.as_mut_ptr(), index as u32);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "gst_trace_format_get_field_structure")]
    pub(crate) fn field_structure(&self, index: usize) -> Option<&StructureRef> {
        unsafe {
            let ptr =
                crate::ffi::gst_trace_format_get_field_structure(self.as_mut_ptr(), index as u32);
            if ptr.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(ptr))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Iterate over the format's declared fields.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = TraceFormatField<'_>> + '_ {
        (0..self.n_fields()).map(move |index| TraceFormatField {
            format: self,
            index,
        })
    }
}

// rustdoc-stripper-ignore-next
/// A single field declared on a [`TraceFormat`], yielded by
/// [`TraceFormat::fields`].
#[cfg(feature = "v1_30")]
#[derive(Debug, Clone, Copy)]
pub struct TraceFormatField<'a> {
    format: &'a TraceFormat,
    index: usize,
}

#[cfg(feature = "v1_30")]
impl<'a> TraceFormatField<'a> {
    pub fn name(&self) -> &'a str {
        self.format.field_name(self.index).unwrap()
    }

    pub fn type_(&self) -> crate::TracerFieldType {
        self.format.field_type(self.index)
    }

    pub fn description(&self) -> Option<&'a str> {
        self.format.field_description(self.index)
    }

    pub fn structure(&self) -> Option<&'a StructureRef> {
        self.format.field_structure(self.index)
    }
}

// rustdoc-stripper-ignore-next
/// Reads a raw `GstTraceValue` union as `field_type`, returning `None` for a
/// field type this version does not recognize.
///
/// # Safety
/// `field_type` must be the value's format-declared type; reading a pointer
/// variant as the wrong type is UB.
#[cfg(feature = "v1_30")]
unsafe fn read_trace_value(
    value: &crate::ffi::GstTraceValue,
    field_type: crate::TracerFieldType,
) -> Option<TraceValue<'_>> {
    use crate::TracerFieldType::*;
    unsafe {
        match field_type {
            Boolean => Some(TraceValue::Boolean(from_glib(value.v_boolean))),
            Int => Some(TraceValue::Int(value.v_int)),
            Uint => Some(TraceValue::Uint(value.v_uint)),
            Int64 => Some(TraceValue::Int64(value.v_int64)),
            Uint64 | ClockTime => Some(TraceValue::Uint64(value.v_uint64)),
            Double => Some(TraceValue::Double(value.v_double)),
            String => Some(TraceValue::String(
                value
                    .v_string
                    .as_ref()
                    .map(|_| glib::GStr::from_ptr(value.v_string)),
            )),
            Structure => Some(TraceValue::Structure(
                value
                    .v_structure
                    .as_ref()
                    .map(|_| StructureRef::from_glib_borrow(value.v_structure)),
            )),
            Object => Some(TraceValue::Object(
                value
                    .v_object
                    .as_ref()
                    .map(|_| from_glib_none(value.v_object)),
            )),
            _ => None,
        }
    }
}

// rustdoc-stripper-ignore-next
/// A custom span value read as its format-declared type, yielded by
/// [`TraceValues::iter`].
#[cfg(feature = "v1_30")]
#[derive(Debug)]
pub enum TraceValue<'a> {
    Boolean(bool),
    Int(i32),
    Uint(u32),
    Int64(i64),
    Uint64(u64),
    Double(f64),
    String(Option<&'a glib::GStr>),
    Structure(Option<&'a StructureRef>),
    Object(Option<glib::Object>),
}

// rustdoc-stripper-ignore-next
/// The values of one custom span together with the [`TraceFormat`] that
/// describes them, as delivered to the `TracerImpl::span_begin` hook.
///
/// Because the format and values are paired here, [`iter`](Self::iter) can read
/// each value at its declared type without any `unsafe` in the caller.
#[cfg(feature = "v1_30")]
pub struct TraceValues<'a> {
    format: TraceFormat,
    values: &'a [crate::ffi::GstTraceValue],
}

#[cfg(feature = "v1_30")]
impl<'a> TraceValues<'a> {
    // rustdoc-stripper-ignore-next
    /// # Safety
    /// `values`/`len` must be the values of `format`, as delivered together by
    /// the span-begin hook; [`iter`](Self::iter) reads them at that format's
    /// types.
    #[inline]
    pub(crate) unsafe fn from_glib(
        format: TraceFormat,
        values: *const crate::ffi::GstTraceValue,
        len: usize,
    ) -> Self {
        skip_assert_initialized!();
        let values = if len == 0 {
            &[]
        } else {
            debug_assert!(!values.is_null());
            unsafe { std::slice::from_raw_parts(values, len) }
        };
        Self { format, values }
    }

    // rustdoc-stripper-ignore-next
    /// The format describing these values.
    #[inline]
    pub fn format(&self) -> TraceFormat {
        self.format
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    // rustdoc-stripper-ignore-next
    /// Read each value at its field's declared type, in field order. `None` is
    /// yielded for a field type this version does not recognize.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Option<TraceValue<'a>>> + '_ {
        let format = self.format;
        self.values.iter().enumerate().map(move |(index, value)| {
            // SAFETY: `value` is the value of the field at `index`, so that
            // field's declared type is its type.
            unsafe { read_trace_value(value, format.field_type(index)) }
        })
    }
}
