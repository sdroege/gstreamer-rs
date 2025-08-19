// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! This module selects one of the two `IdStr` implementations:
//!
//! * When feature `v1_26` (or later) is activated, `IdStr` implements the bindings
//!   for the C type `GstIdStr`.
//! * For earlier feature versions, a compatibility implementation is used.
//!
//! See also: <https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/7432>

cfg_if::cfg_if! {
    if #[cfg(feature = "v1_26")] {
        mod bindings;
        pub use self::bindings::IdStr;
    } else {
        mod compat;
        pub use self::compat::IdStr;
    }
}

#[cfg(feature = "serde")]
mod serde;

// rustdoc-stripper-ignore-next
/// Builds an [`IdStr`] from a string literal.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use gstreamer::{idstr, IdStr};
/// use std::sync::LazyLock;
///
/// static MY_ID_STR: LazyLock<IdStr> = LazyLock::new(|| idstr!("static id"));
/// assert_eq!(*MY_ID_STR, "static id");
///
/// let my_id_str: IdStr = idstr!("local id");
/// assert_eq!(my_id_str, "local id");
/// # }
/// ```
///
/// [`IdStr`]: crate::IdStr
#[macro_export]
macro_rules! idstr {
    ($s:literal) => {
        $crate::IdStr::from_static($crate::glib::gstr!($s))
    };
}

#[cfg(test)]
mod tests {
    use glib::{gstr, GStr, GString};
    use std::{ffi::CStr, sync::LazyLock};

    use super::IdStr;

    const STR: &str = "STR";
    static IDSTR: LazyLock<IdStr> = LazyLock::new(|| idstr!("IDSTR"));
    static GSTR: &GStr = gstr!("GSTR");
    static GSTRING: LazyLock<GString> = LazyLock::new(|| GString::from("GSTRING"));

    const LONG_STR: &str = "An STR longer than 15 bytes";
    static LONG_IDSTR: LazyLock<IdStr> = LazyLock::new(|| idstr!("An IdStr longer than 15 bytes"));
    static LONG_GSTR: &GStr = gstr!("A GSTR longer than 15 bytes");
    static LONG_GSTRING: LazyLock<GString> =
        LazyLock::new(|| GString::from("A GSTRING longer than 15 bytes"));

    #[test]
    fn new_set_static() {
        assert!(!IDSTR.is_empty());
        assert_eq!(IDSTR.len(), "IDSTR".len());
        assert_eq!(IDSTR.as_str().len(), "IDSTR".len());
        assert_eq!(*IDSTR, "IDSTR");
        // Display impl
        assert_eq!(IDSTR.to_string(), "IDSTR");
        assert_eq!(IDSTR.as_str(), "IDSTR");
        assert_eq!(IDSTR.as_gstr().len(), "IDSTR".len());
        assert_eq!(IDSTR.as_gstr(), "IDSTR");

        let id_str: IdStr = idstr!("id_str");
        assert!(!id_str.is_empty());
        assert_eq!(id_str.len(), "id_str".len());
        assert_eq!(id_str.as_str().len(), "id_str".len());
        assert_eq!(id_str, "id_str");

        let mut s = IdStr::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.as_str(), "");
        assert_eq!(s.as_gstr(), "");

        s.set_static(gstr!("str"));
        assert!(!s.is_empty());
        assert_eq!(s.len(), "str".len());
        assert_eq!(s.as_str().len(), "str".len());
        assert_eq!(s, "str");
        // Display impl
        assert_eq!(s.to_string(), "str");
        assert_eq!(s.as_str(), "str");
        assert_eq!(s.as_gstr().len(), "str".len());
        assert_eq!(s.as_gstr(), "str");

        s.set_static(GSTR);
        assert_eq!(s.as_str(), "GSTR");

        s.set_static(&*GSTRING);
        assert_eq!(s.as_str(), "GSTRING");

        assert!(!LONG_IDSTR.is_empty());
        assert_eq!(LONG_IDSTR.len(), "An IdStr longer than 15 bytes".len());
        assert_eq!(*LONG_IDSTR, "An IdStr longer than 15 bytes");
        // Display impl
        assert_eq!(LONG_IDSTR.to_string(), "An IdStr longer than 15 bytes");
        assert_eq!(
            LONG_IDSTR.as_str().len(),
            "An IdStr longer than 15 bytes".len()
        );
        assert_eq!(LONG_IDSTR.as_str(), "An IdStr longer than 15 bytes");
        assert_eq!(
            LONG_IDSTR.as_gstr().len(),
            "An IdStr longer than 15 bytes".len()
        );
        assert_eq!(LONG_IDSTR.as_gstr(), "An IdStr longer than 15 bytes");

        let ls = idstr!("An IdStr longer than 15 bytes");
        assert!(!ls.is_empty());
        assert_eq!(ls.len(), "An IdStr longer than 15 bytes".len());
        assert_eq!(ls, "An IdStr longer than 15 bytes");

        let mut ls = IdStr::new();

        ls.set_static(gstr!("An str longer than 15 bytes"));
        assert!(!ls.is_empty());
        assert_eq!(ls.len(), "An str longer than 15 bytes".len());
        assert_eq!(ls, "An str longer than 15 bytes");

        ls.set_static(LONG_GSTR);
        assert_eq!(ls.as_str(), "A GSTR longer than 15 bytes");

        ls.set_static(&*LONG_GSTRING);
        assert_eq!(ls.as_str(), "A GSTRING longer than 15 bytes");
    }

    #[test]
    fn from_static() {
        let s = IdStr::from_static(gstr!("str"));
        assert!(!s.is_empty());
        assert_eq!(s.len(), "str".len());
        assert_eq!(s.as_str().len(), "str".len());
        assert_eq!(s, "str");
        // Display impl
        assert_eq!(s.to_string(), "str");
        assert_eq!(s.as_str(), "str");
        assert_eq!(s.as_gstr().len(), "str".len());
        assert_eq!(s.as_gstr(), "str");

        let s = idstr!("str");
        assert!(!s.is_empty());
        assert_eq!(s.len(), "str".len());
        assert_eq!(s.as_str().len(), "str".len());
        assert_eq!(s, "str");

        let s = IdStr::from_static(GSTR);
        assert_eq!(s.as_str(), "GSTR");

        let s = IdStr::from_static(&*GSTRING);
        assert_eq!(s.as_str(), "GSTRING");

        let ls = IdStr::from_static(gstr!("An str longer than 15 bytes"));
        assert!(!ls.is_empty());
        assert_eq!(ls.len(), "An str longer than 15 bytes".len());
        assert_eq!(ls, "An str longer than 15 bytes");
        // Display impl
        assert_eq!(ls.to_string(), "An str longer than 15 bytes");
        assert_eq!(ls.as_str().len(), "An str longer than 15 bytes".len());
        assert_eq!(ls.as_str(), "An str longer than 15 bytes");
        assert_eq!(ls.as_gstr().len(), "An str longer than 15 bytes".len());
        assert_eq!(ls.as_gstr(), "An str longer than 15 bytes");

        let ls = idstr!("An str longer than 15 bytes");
        assert!(!ls.is_empty());
        assert_eq!(ls.len(), "An str longer than 15 bytes".len());
        assert_eq!(ls, "An str longer than 15 bytes");

        let ls = IdStr::from_static(LONG_GSTR);
        assert_eq!(ls.as_str(), "A GSTR longer than 15 bytes");

        let ls = IdStr::from_static(&*LONG_GSTRING);
        assert_eq!(ls.as_str(), "A GSTRING longer than 15 bytes");
    }

    #[test]
    fn new_set() {
        let d = IdStr::default();
        assert!(d.is_empty());
        assert_eq!(d.len(), 0);
        assert_eq!(d.as_str(), "");
        assert_eq!(d.as_gstr(), "");

        let mut s = IdStr::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.as_str(), "");
        assert_eq!(s.as_gstr(), "");

        s.set("str");
        assert!(!s.is_empty());
        assert_eq!(s.len(), "str".len());
        assert_eq!(s.as_str().len(), "str".len());
        assert_eq!(s.as_str(), "str");
        assert_eq!(AsRef::<str>::as_ref(&s), "str");
        // Display impl
        assert_eq!(s.to_string(), "str");
        assert_eq!(s.as_gstr().len(), "str".len());
        assert_eq!(s.as_gstr(), "str");
        assert_eq!(AsRef::<GStr>::as_ref(&s), "str");
        assert_eq!(s.as_cstr().to_bytes(), b"str");
        assert_eq!(AsRef::<CStr>::as_ref(&s).to_bytes(), b"str");
        assert_eq!(s.as_bytes(), b"str");

        let string = String::from("String");
        s.set(string.as_str());
        assert_eq!(s.as_str(), "String");
        s.set(&string);
        assert_eq!(s.as_str(), "String");

        s.set(gstr!("gstr"));
        assert_eq!(s.as_str(), "gstr");

        let gstring = GString::from("GString");
        s.set(gstring.as_gstr());
        assert_eq!(s.as_str(), "GString");
        s.set(&gstring);
        assert_eq!(s.as_str(), "GString");
        s.set(gstring.as_str());
        assert_eq!(s.as_str(), "GString");

        let mut ls = IdStr::new();

        ls.set("An str longer than 15 bytes");
        assert!(!ls.is_empty());
        assert_eq!(ls.len(), "An str longer than 15 bytes".len());
        assert_eq!(ls, "An str longer than 15 bytes");
        // Display impl
        assert_eq!(ls.to_string(), "An str longer than 15 bytes");
        assert_eq!(ls.as_str().len(), "An str longer than 15 bytes".len());
        assert_eq!(ls.as_str(), "An str longer than 15 bytes");
        assert_eq!(ls.as_gstr().len(), "An str longer than 15 bytes".len());
        assert_eq!(ls.as_gstr(), "An str longer than 15 bytes");
        assert_eq!(ls.as_cstr().to_bytes(), b"An str longer than 15 bytes");
        assert_eq!(
            AsRef::<CStr>::as_ref(&ls).to_bytes(),
            b"An str longer than 15 bytes"
        );
        assert_eq!(ls.as_bytes(), b"An str longer than 15 bytes");

        ls.set(gstr!("A gstr longer than 15 bytes"));
        assert_eq!(ls.as_str(), "A gstr longer than 15 bytes");
    }

    #[test]
    fn from() {
        let s = IdStr::from("str");
        assert_eq!(s.len(), "str".len());
        assert_eq!(s.as_str().len(), "str".len());
        assert_eq!(s.as_str(), "str");
        // Display impl
        assert_eq!(s.to_string(), "str");
        assert_eq!(s.as_gstr().len(), "str".len());
        assert_eq!(s.as_gstr(), "str");

        let string = String::from("String");
        let s = IdStr::from(string.as_str());
        assert_eq!(s.as_str(), "String");
        let s: IdStr = string.as_str().into();
        assert_eq!(s.as_str(), "String");
        let s: IdStr = (&string).into();
        assert_eq!(s.as_str(), "String");
        let s: IdStr = string.into();
        assert_eq!(s.as_str(), "String");

        let s = IdStr::from(gstr!("str"));
        assert_eq!(s.as_str(), "str");

        let gstring = GString::from("GString");
        let s = IdStr::from(gstring.as_gstr());
        assert_eq!(s.as_str(), "GString");
        let s: IdStr = (&gstring).into();
        assert_eq!(s.as_str(), "GString");
        let s: IdStr = gstring.into();
        assert_eq!(s.as_str(), "GString");

        let ls = IdStr::from("An str longer than 15 bytes");
        assert!(!ls.is_empty());
        assert_eq!(ls.len(), "An str longer than 15 bytes".len());
        assert_eq!(ls, "An str longer than 15 bytes");
        // Display impl
        assert_eq!(ls.to_string(), "An str longer than 15 bytes");
        assert_eq!(ls.as_str().len(), "An str longer than 15 bytes".len());
        assert_eq!(ls.as_str(), "An str longer than 15 bytes");
        assert_eq!(ls.as_gstr().len(), "An str longer than 15 bytes".len());
        assert_eq!(ls.as_gstr(), "An str longer than 15 bytes");

        let ls = IdStr::from(gstr!("A gstr longer than 15 bytes"));
        assert_eq!(ls.as_str(), "A gstr longer than 15 bytes");
        assert_eq!(ls.as_gstr(), "A gstr longer than 15 bytes");

        let lstring = String::from("A String longer than 15 bytes");
        let ls = IdStr::from(lstring.as_str());
        assert_eq!(ls.as_str(), "A String longer than 15 bytes");
        let ls = IdStr::from(&lstring);
        assert_eq!(ls.as_str(), "A String longer than 15 bytes");

        let lgstring = String::from("A GString longer than 15 bytes");
        let ls = IdStr::from(lgstring.as_str());
        assert_eq!(ls.as_str(), "A GString longer than 15 bytes");
        let ls = IdStr::from(&lgstring);
        assert_eq!(ls.as_str(), "A GString longer than 15 bytes");
    }

    #[test]
    #[allow(clippy::cmp_owned)]
    fn eq_cmp() {
        let s1 = IdStr::from(STR);
        let s12: IdStr = STR.into();
        assert_eq!(s1, s12);
        let s2 = IdStr::from(String::from(STR));
        let s22: IdStr = String::from(STR).into();
        assert_eq!(s2, s22);
        let s3 = IdStr::from_static(gstr!("STR"));
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
        assert_eq!(s2, s3);

        assert!(s1 == gstr!("STR"));
        assert_eq!(s1, gstr!("STR"));
        assert_eq!(s1, GString::from("STR"));
        assert!(s1 == "STR");
        assert_eq!(s1, "STR");
        assert!("STR" == s1);
        assert_eq!("STR", s1);
        assert_eq!(s1, String::from("STR"));

        assert_eq!(gstr!("STR"), s1);
        assert_eq!(GString::from("STR"), s1);
        assert_eq!("STR", s1);
        assert_eq!(String::from("STR"), s1);

        let ls1 = IdStr::from(LONG_STR);
        let ls2: IdStr = String::from(LONG_STR).into();
        let ls3 = IdStr::from_static(gstr!("An STR longer than 15 bytes"));
        assert_eq!(ls1, ls2);
        assert_eq!(ls1, ls3);
        assert_eq!(ls2, ls3);

        assert!(ls1 == gstr!("An STR longer than 15 bytes"));
        assert_eq!(ls1, gstr!("An STR longer than 15 bytes"));
        assert_eq!(ls1, GString::from(LONG_STR));
        assert_eq!(ls1, LONG_STR);
        assert!(ls1 == "An STR longer than 15 bytes");
        assert_eq!(ls1, "An STR longer than 15 bytes");
        assert_eq!(ls1, String::from(LONG_STR));

        assert_eq!(gstr!("An STR longer than 15 bytes"), ls1);
        assert_eq!(GString::from(LONG_STR), ls1);
        assert_eq!(LONG_STR, ls1);
        assert_eq!("An STR longer than 15 bytes", ls1);
        assert_eq!(String::from(LONG_STR), ls1);

        assert_ne!(s1, ls1);
        assert_ne!(ls1, s1);

        let s4 = IdStr::from("STR4");
        assert_ne!(s1, s4);
        assert!(s1 < s4);
        assert!(s4 > s1);

        assert!(s1 < gstr!("STR4"));
        assert!(s1 < GString::from("STR4"));
        assert!(s1 < "STR4");
        assert!("STR4" > s1);
        assert!(s1 < String::from("STR4"));

        assert!(gstr!("STR4") > s1);
        assert!(GString::from("STR4") > s1);
        assert!("STR4" > s1);
        assert!(String::from("STR4") > s1);

        // ls1 starts with an 'A', s4 with an 'S'
        assert_ne!(ls1, s4);
        assert!(ls1 < s4);
        assert!(s4 > s1);
    }

    #[test]
    fn as_ref_idstr() {
        #[allow(clippy::nonminimal_bool)]
        fn check(c: &str, v: impl AsRef<IdStr>) {
            let v = v.as_ref();

            assert_eq!(c, v);
            assert_eq!(v, c);

            assert!(!(c > v));
            assert!(!(v > c));

            let i = IdStr::from(c);
            assert_eq!(i, v);
            assert_eq!(i, IdStr::from(c));

            assert!(!(c > i));
            assert!(!(i > c));
        }

        let v = IdStr::from(STR);
        check(STR, &v);
        check(STR, v);

        #[allow(clippy::nonminimal_bool)]
        fn check_gstr(c: &GStr, v: impl AsRef<IdStr>) {
            let v = v.as_ref();

            assert_eq!(c, v);
            assert_eq!(v, c);

            assert!(!(c > v));
            assert!(!(v > c));
        }

        let v = IdStr::from(GSTR);
        check_gstr(GSTR, &v);
        check_gstr(GSTR, v);
    }
}
