// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! The `IdStr` bindings of the C type `GstIdStr`.
//!
//! See the higher level module documentation for details.

use crate::ffi;
use glib::{translate::*, GStr, GString};
use std::{
    cmp,
    ffi::{c_char, CStr},
    fmt,
    hash::{Hash, Hasher},
    mem,
    ops::Deref,
    ptr::NonNull,
};

glib::wrapper! {
    // rustdoc-stripper-ignore-next
    /// An UTF-8 immutable string type with optimizations for short values (len < 16).
    #[derive(Debug)]
    #[doc(alias = "GstIdStr")]
    pub struct IdStr(BoxedInline<ffi::GstIdStr>);

    match fn {
        copy => |ptr| ffi::gst_id_str_copy(ptr),
        free => |ptr| ffi::gst_id_str_free(ptr),
        init => |ptr| ffi::gst_id_str_init(ptr),
        copy_into => |dest, src| ffi::gst_id_str_copy_into(dest, src),
        clear => |ptr| ffi::gst_id_str_clear(ptr),
    }
}

impl IdStr {
    #[doc(alias = "gst_id_str_new")]
    #[inline]
    pub const fn new() -> IdStr {
        skip_assert_initialized!();
        unsafe {
            // Safety: empty inlined string consists in the type being all zeroed
            IdStr {
                inner: mem::zeroed(),
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Builds an `IdStr` from the given static `GStr`.
    ///
    /// This constructor performs optimizations which other constructors can't rely on.
    ///
    /// To build an `IdStr` from a string literal, use the [`idstr`](crate::idstr) macro.
    #[inline]
    pub fn from_static<T: AsRef<GStr> + ?Sized>(value: &'static T) -> IdStr {
        skip_assert_initialized!();
        let mut ret = IdStr::new();
        ret.set_static(value);

        ret
    }

    #[doc(alias = "gst_id_str_new")]
    #[inline]
    pub fn from<T: AsRef<str>>(value: T) -> IdStr {
        skip_assert_initialized!();
        let mut id = IdStr::new();
        id.set(value);

        id
    }

    #[doc(alias = "gst_id_str_get_len")]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { ffi::gst_id_str_get_len(self.to_glib_none().0) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // rustdoc-stripper-ignore-next
    /// Returns the pointer to the nul terminated string `value` represented by this `IdStr`.
    #[inline]
    fn as_char_ptr(&self) -> NonNull<c_char> {
        unsafe {
            let ptr = ffi::gst_id_str_as_str(self.to_glib_none().0);
            debug_assert!(!ptr.is_null());
            let nn = NonNull::<c_char>::new_unchecked(ptr as *mut _);

            debug_assert_eq!(*nn.as_ptr().add(self.len()), 0, "expecting nul terminator");

            nn
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            // Safety: `as_char_ptr()` returns a non-null pointer to a nul terminated string.
            std::slice::from_raw_parts(self.as_char_ptr().as_ptr() as *const _, self.len())
        }
    }

    #[inline]
    fn as_bytes_with_nul(&self) -> &[u8] {
        unsafe {
            // Safety: `as_char_ptr()` returns a non-null pointer to a nul terminated string.
            std::slice::from_raw_parts(self.as_char_ptr().as_ptr() as *const _, self.len() + 1)
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                std::str::from_utf8(self.as_bytes()).unwrap()
            } else {
                unsafe {
                    std::str::from_utf8_unchecked(self.as_bytes())
                }
            }
        }
    }

    #[doc(alias = "gst_id_str_as_str")]
    #[inline]
    pub fn as_gstr(&self) -> &GStr {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                GStr::from_utf8_with_nul(self.as_bytes_with_nul()).unwrap()
            } else {
                unsafe {
                    GStr::from_utf8_with_nul_unchecked(self.as_bytes_with_nul())
                }
            }
        }
    }

    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                CStr::from_bytes_with_nul(self.as_bytes_with_nul()).unwrap()
            } else {
                unsafe {
                    CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul())
                }
            }
        }
    }

    #[doc(alias = "gst_id_str_is_equal")]
    #[inline]
    fn is_equal(&self, s2: &IdStr) -> bool {
        unsafe {
            from_glib(ffi::gst_id_str_is_equal(
                self.to_glib_none().0,
                s2.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_id_str_is_equal_to_str_with_len")]
    #[inline]
    fn is_equal_to_str(&self, s2: impl AsRef<str>) -> bool {
        unsafe {
            let s2 = s2.as_ref();
            from_glib(ffi::gst_id_str_is_equal_to_str_with_len(
                self.to_glib_none().0,
                s2.as_ptr() as *const c_char,
                s2.len(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets `self` to the static string `value`.
    ///
    /// This function performs optimizations which [IdStr::set] can't rely on.
    ///
    /// To build an `IdStr` from a string literal, use the [`idstr`](crate::idstr) macro.
    #[doc(alias = "gst_id_str_set_static_str")]
    #[doc(alias = "gst_id_str_set_static_str_with_len")]
    #[inline]
    pub fn set_static<T: AsRef<GStr> + ?Sized>(&mut self, value: &'static T) {
        unsafe {
            let v = value.as_ref();
            ffi::gst_id_str_set_static_str_with_len(
                self.to_glib_none_mut().0,
                v.to_glib_none().0,
                v.len(),
            );
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets `self` to the string `value`.
    ///
    /// For a static value, use [IdStr::set_static] which can perform optimizations.
    ///
    /// To build an `IdStr` from a string literal, use the [`idstr`](crate::idstr) macro.
    #[doc(alias = "gst_id_str_set")]
    #[doc(alias = "gst_id_str_set_with_len")]
    #[inline]
    pub fn set(&mut self, value: impl AsRef<str>) {
        unsafe {
            let v = value.as_ref();
            ffi::gst_id_str_set_with_len(
                self.to_glib_none_mut().0,
                v.as_ptr() as *const c_char,
                v.len(),
            );
        }
    }
}

impl Default for IdStr {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for IdStr {
    type Target = GStr;

    fn deref(&self) -> &Self::Target {
        self.as_gstr()
    }
}

impl AsRef<str> for IdStr {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<IdStr> for IdStr {
    #[inline]
    fn as_ref(&self) -> &IdStr {
        self
    }
}

impl AsRef<GStr> for IdStr {
    #[inline]
    fn as_ref(&self) -> &GStr {
        self.as_gstr()
    }
}

impl AsRef<CStr> for IdStr {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_cstr()
    }
}

impl From<&str> for IdStr {
    #[inline]
    fn from(value: &str) -> IdStr {
        skip_assert_initialized!();
        let mut ret = IdStr::new();
        ret.set(value);

        ret
    }
}

impl From<&String> for IdStr {
    #[inline]
    fn from(value: &String) -> IdStr {
        skip_assert_initialized!();
        let mut ret = IdStr::new();
        ret.set(value);

        ret
    }
}

impl From<String> for IdStr {
    #[inline]
    fn from(value: String) -> IdStr {
        skip_assert_initialized!();
        let mut ret = IdStr::new();
        ret.set(&value);

        ret
    }
}

impl From<&GStr> for IdStr {
    #[inline]
    fn from(value: &GStr) -> IdStr {
        // assert checked in new()
        skip_assert_initialized!();
        let mut ret = IdStr::new();
        ret.set(value);

        ret
    }
}

impl From<&GString> for IdStr {
    #[inline]
    fn from(value: &GString) -> IdStr {
        skip_assert_initialized!();
        let mut ret = IdStr::new();
        ret.set(value);

        ret
    }
}

impl From<GString> for IdStr {
    #[inline]
    fn from(value: GString) -> IdStr {
        skip_assert_initialized!();
        let mut ret = IdStr::new();
        ret.set(&value);

        ret
    }
}

impl fmt::Display for IdStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_gstr())
    }
}

impl PartialOrd for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for IdStr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl PartialOrd<&IdStr> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &&IdStr) -> Option<cmp::Ordering> {
        Some(self.cmp(*other))
    }
}

impl PartialEq<&IdStr> for IdStr {
    #[inline]
    fn eq(&self, other: &&IdStr) -> bool {
        self.is_equal(other)
    }
}

impl Ord for IdStr {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_cstr().cmp(other.as_cstr())
    }
}

impl Eq for IdStr {}

impl PartialOrd<&GStr> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &&GStr) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
}

impl PartialEq<&GStr> for IdStr {
    #[inline]
    fn eq(&self, other: &&GStr) -> bool {
        self.is_equal_to_str(other)
    }
}

impl PartialOrd<GStr> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &GStr) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl PartialEq<GStr> for IdStr {
    #[inline]
    fn eq(&self, other: &GStr) -> bool {
        self.is_equal_to_str(other)
    }
}

impl PartialOrd<IdStr> for &GStr {
    #[inline]
    fn partial_cmp(&self, other: &IdStr) -> Option<cmp::Ordering> {
        (*self).partial_cmp(other.as_gstr())
    }
}

impl PartialEq<IdStr> for &GStr {
    #[inline]
    fn eq(&self, other: &IdStr) -> bool {
        other.is_equal_to_str(self)
    }
}

impl PartialOrd<IdStr> for GStr {
    #[inline]
    fn partial_cmp(&self, other: &IdStr) -> Option<cmp::Ordering> {
        self.partial_cmp(other.as_gstr())
    }
}

impl PartialEq<IdStr> for GStr {
    #[inline]
    fn eq(&self, other: &IdStr) -> bool {
        other.is_equal_to_str(self)
    }
}

impl PartialOrd<&str> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &&str) -> Option<cmp::Ordering> {
        self.as_gstr().partial_cmp(*other)
    }
}

impl PartialEq<&str> for IdStr {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.is_equal_to_str(*other)
    }
}

impl PartialOrd<str> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.as_gstr().partial_cmp(other)
    }
}

impl PartialEq<str> for IdStr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.is_equal_to_str(other)
    }
}

impl PartialOrd<IdStr> for &str {
    #[inline]
    fn partial_cmp(&self, other: &IdStr) -> Option<cmp::Ordering> {
        (*self).partial_cmp(other.as_gstr())
    }
}

impl PartialEq<IdStr> for &str {
    #[inline]
    fn eq(&self, other: &IdStr) -> bool {
        other.is_equal_to_str(self)
    }
}

impl PartialOrd<IdStr> for str {
    #[inline]
    fn partial_cmp(&self, other: &IdStr) -> Option<cmp::Ordering> {
        self.partial_cmp(other.as_gstr())
    }
}

impl PartialEq<IdStr> for str {
    #[inline]
    fn eq(&self, other: &IdStr) -> bool {
        other.is_equal_to_str(self)
    }
}

impl PartialOrd<GString> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &GString) -> Option<cmp::Ordering> {
        self.as_gstr().partial_cmp(other)
    }
}

impl PartialEq<GString> for IdStr {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        self.is_equal_to_str(other)
    }
}

impl PartialOrd<IdStr> for GString {
    #[inline]
    fn partial_cmp(&self, other: &IdStr) -> Option<cmp::Ordering> {
        self.partial_cmp(other.as_gstr())
    }
}

impl PartialEq<IdStr> for GString {
    #[inline]
    fn eq(&self, other: &IdStr) -> bool {
        other.is_equal_to_str(self)
    }
}

impl PartialOrd<String> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        self.as_gstr().partial_cmp(other)
    }
}

impl PartialEq<String> for IdStr {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.is_equal_to_str(other)
    }
}

impl PartialOrd<IdStr> for String {
    #[inline]
    fn partial_cmp(&self, other: &IdStr) -> Option<cmp::Ordering> {
        self.partial_cmp(other.as_gstr())
    }
}

impl PartialEq<IdStr> for String {
    #[inline]
    fn eq(&self, other: &IdStr) -> bool {
        other.is_equal_to_str(self)
    }
}

impl Hash for IdStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_gstr().hash(state)
    }
}

unsafe impl Send for IdStr {}
unsafe impl Sync for IdStr {}

// Tests are mutualised between this implementation and the one in id_str_compat
// See gstreamer/id_str/mod.rs
