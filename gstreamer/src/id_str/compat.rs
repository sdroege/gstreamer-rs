// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! The `IdStr` compatibility implementation.
//!
//! See the higher level module documentation for details.

use glib::{GStr, GString, IntoGStr};
use std::{
    cmp,
    ffi::CStr,
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
};

use kstring::KString;

// rustdoc-stripper-ignore-next
/// An UTF-8 immutable string type with optimizations for short values (len < 16).
#[derive(Clone, Debug)]
#[doc(alias = "GstIdStr")]
pub struct IdStr(KString);

impl IdStr {
    // In order to keep the same API and usability as `id_str_bindings::IdStr` regarding
    // the ability to efficiently deref to `&GStr`, the internal `KString` is always built
    // from a string with a nul terminator.

    #[doc(alias = "gst_id_str_new")]
    #[inline]
    pub const fn new() -> IdStr {
        skip_assert_initialized!();
        // Always include the nul terminator in the internal string
        IdStr(KString::from_static("\0"))
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
        let gstr = value.as_ref();
        unsafe {
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                gstr.as_ptr() as *const _,
                gstr.as_bytes_with_nul().len(),
            ));

            IdStr(KString::from_static(str_with_nul))
        }
    }

    #[doc(alias = "gst_id_str_new")]
    #[inline]
    pub fn from(value: impl AsRef<str>) -> IdStr {
        skip_assert_initialized!();
        let mut id = IdStr::new();
        id.set(value);

        id
    }

    #[doc(alias = "gst_id_str_get_len")]
    #[inline]
    pub fn len(&self) -> usize {
        // The internal string ends with a nul terminator
        self.0.len() - 1
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        // The internal string ends with a nul terminator
        self.0.len() == 1
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        // The internal string ends with a nul terminator
        &self.0.as_bytes()[..IdStr::len(self)]
    }

    #[inline]
    fn as_bytes_with_nul(&self) -> &[u8] {
        // The internal string ends with a nul terminator
        self.0.as_bytes()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            // Safety: the internal value is guaranteed to be an utf-8 string.
            std::str::from_utf8_unchecked(self.as_bytes())
        }
    }

    #[doc(alias = "gst_id_str_as_str")]
    #[inline]
    pub fn as_gstr(&self) -> &GStr {
        unsafe {
            // Safety: the internal value is guaranteed to be an utf-8 string.
            GStr::from_utf8_with_nul_unchecked(self.as_bytes_with_nul())
        }
    }

    #[doc(alias = "gst_id_str_as_str")]
    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        unsafe {
            // Safety: the internal value is guaranteed to be an utf-8 string
            // thus to not contain any nul bytes except for the terminator.
            CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul())
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
            let gstr = value.as_ref();
            // Safety: the `GStr` value is guaranteed to be an utf-8 string
            // ending with a nul terminator.
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                gstr.as_ptr() as *const _,
                gstr.as_bytes_with_nul().len(),
            ));

            self.0 = KString::from_static(str_with_nul);
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
        self.0 = value.as_ref().run_with_gstr(|gstr| unsafe {
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                gstr.as_ptr() as *const _,
                gstr.as_bytes_with_nul().len(),
            ));

            KString::from_ref(str_with_nul)
        });
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

impl AsRef<IdStr> for IdStr {
    #[inline]
    fn as_ref(&self) -> &IdStr {
        self
    }
}

impl AsRef<str> for IdStr {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
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
        value.run_with_gstr(|gstr| unsafe {
            // Safety: the `GStr` value is guaranteed to be an utf-8 string
            // ending with a nul terminator.
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                gstr.as_ptr() as *const _,
                gstr.as_bytes_with_nul().len(),
            ));

            IdStr(KString::from_ref(str_with_nul))
        })
    }
}

impl From<&String> for IdStr {
    #[inline]
    fn from(value: &String) -> IdStr {
        skip_assert_initialized!();
        value.run_with_gstr(|gstr| unsafe {
            // Safety: the `GStr` value is guaranteed to be an utf-8 string
            // ending with a nul terminator.
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                gstr.as_ptr() as *const _,
                gstr.as_bytes_with_nul().len(),
            ));

            IdStr(KString::from_ref(str_with_nul))
        })
    }
}

impl From<String> for IdStr {
    #[inline]
    fn from(value: String) -> IdStr {
        skip_assert_initialized!();
        value.run_with_gstr(|gstr| unsafe {
            // Safety: the `GStr` value is guaranteed to be an utf-8 string
            // ending with a nul terminator.
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                gstr.as_ptr() as *const _,
                gstr.as_bytes_with_nul().len(),
            ));

            IdStr(KString::from_ref(str_with_nul))
        })
    }
}

impl From<&GStr> for IdStr {
    #[inline]
    fn from(value: &GStr) -> IdStr {
        skip_assert_initialized!();
        unsafe {
            // Safety: the `GStr` value is guaranteed to be an utf-8 string
            // ending with a nul terminator.
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                value.as_ptr() as *const _,
                value.as_bytes_with_nul().len(),
            ));

            IdStr(KString::from_ref(str_with_nul))
        }
    }
}

impl From<&GString> for IdStr {
    #[inline]
    fn from(value: &GString) -> IdStr {
        skip_assert_initialized!();
        unsafe {
            // Safety: the `GString` value is guaranteed to be an utf-8 string
            // ending with a nul terminator.
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                value.as_ptr() as *const _,
                value.len() + 1,
            ));

            IdStr(KString::from_ref(str_with_nul))
        }
    }
}

impl From<GString> for IdStr {
    #[inline]
    fn from(value: GString) -> IdStr {
        skip_assert_initialized!();
        unsafe {
            // Safety: the `GString` value is guaranteed to be an utf-8 string
            // ending with a nul terminator.
            let str_with_nul = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                value.as_ptr() as *const _,
                value.len() + 1,
            ));

            IdStr(KString::from_ref(str_with_nul))
        }
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
        self.0 == other.0
    }
}

impl PartialOrd<&IdStr> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &&IdStr) -> Option<cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl PartialEq<&IdStr> for IdStr {
    #[inline]
    fn eq(&self, other: &&IdStr) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<IdStr> for &IdStr {
    #[inline]
    fn partial_cmp(&self, other: &IdStr) -> Option<cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl PartialEq<IdStr> for &IdStr {
    #[inline]
    fn eq(&self, other: &IdStr) -> bool {
        self.0 == other.0
    }
}

impl Ord for IdStr {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for IdStr {}

impl PartialOrd<&GStr> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &&GStr) -> Option<cmp::Ordering> {
        self.as_gstr().partial_cmp(*other)
    }
}

impl PartialEq<&GStr> for IdStr {
    #[inline]
    fn eq(&self, other: &&GStr) -> bool {
        self.as_gstr() == *other
    }
}

impl PartialOrd<GStr> for IdStr {
    #[inline]
    fn partial_cmp(&self, other: &GStr) -> Option<cmp::Ordering> {
        self.as_gstr().partial_cmp(other)
    }
}

impl PartialEq<GStr> for IdStr {
    #[inline]
    fn eq(&self, other: &GStr) -> bool {
        self.as_gstr() == other
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
        (*self) == other.as_gstr()
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
        self == other.as_gstr()
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
        self.as_gstr() == *other
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
        self.as_gstr() == other
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
        (*self) == other.as_gstr()
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
        self == other.as_gstr()
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
        self.as_gstr() == other
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
        self == other.as_gstr()
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
        self.as_gstr() == other
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
        self == other.as_gstr()
    }
}

impl Hash for IdStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_gstr().hash(state)
    }
}

unsafe impl Send for IdStr {}
unsafe impl Sync for IdStr {}

// Tests are mutualised between this implementation and the one in id_str_bindings
// See gstreamer/id_str/mod.rs
