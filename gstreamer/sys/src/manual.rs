use glib_sys::gpointer;
use std::{fmt, mem};

cfg_if::cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        const GST_ID_STR_PADDING_LEN: usize = 8;
    } else if #[cfg(target_pointer_width = "32")] {
        const GST_ID_STR_PADDING_LEN: usize = 12;
    } else {
        panic!("Only 32 bit and 64 bit pointers supported currently");
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstIdStr {
    // Internal representation is private
    pointer: gpointer,
    padding: [u8; GST_ID_STR_PADDING_LEN],
}

impl fmt::Debug for GstIdStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fmt::Write;

        unsafe {
            let pointer =
                &*(&self.pointer as *const *mut _ as *const [u8; mem::size_of::<gpointer>()]);

            let is_pretty = f.alternate();

            f.write_str("GstIdStr(")?;
            if is_pretty {
                f.write_str("\n    ")?;
            }

            f.write_str("ascii: \"")?;
            for &b in pointer.iter().chain(self.padding.iter()) {
                match b {
                    0 => break,
                    c if c.is_ascii() => f.write_char(char::from(b))?,
                    _ => f.write_char('�')?,
                }
            }

            if is_pretty {
                f.write_str("\",\n    ")?;
            } else {
                f.write_str("\", ")?;
            }

            f.write_str("hex: ")?;
            for (i, b) in pointer.iter().chain(self.padding.iter()).enumerate() {
                if i > 0 {
                    f.write_char(' ')?;
                }
                f.write_fmt(format_args!("{b:02x}"))?;
            }

            if is_pretty {
                f.write_str(",\n")?;
            }

            f.write_char(')')
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "v1_26")]
    #[test]
    fn gstidstr_debug() {
        unsafe {
            use std::{ffi::c_char, mem};

            let mut s = mem::MaybeUninit::uninit();
            crate::gst_id_str_init(s.as_mut_ptr());
            let mut s = s.assume_init();

            crate::gst_id_str_set(&mut s as *mut _, b"short\0" as *const u8 as *const c_char);
            assert_eq!(
                format!("{s:?}"),
                r#"GstIdStr(ascii: "short", hex: 73 68 6f 72 74 00 00 00 00 00 00 00 00 00 00 00)"#,
            );
            assert_eq!(
                format!("{s:#?}"),
                r#"GstIdStr(
    ascii: "short",
    hex: 73 68 6f 72 74 00 00 00 00 00 00 00 00 00 00 00,
)"#
            );

            crate::gst_id_str_set(
                &mut s as *mut _,
                b"utf8\xc3\xa7\0" as *const u8 as *const c_char,
            );
            assert_eq!(
                format!("{s:?}"),
                r#"GstIdStr(ascii: "utf8��", hex: 75 74 66 38 c3 a7 00 00 00 00 00 00 00 00 00 00)"#,
            );
            assert_eq!(
                format!("{s:#?}"),
                r#"GstIdStr(
    ascii: "utf8��",
    hex: 75 74 66 38 c3 a7 00 00 00 00 00 00 00 00 00 00,
)"#
            );
        }
    }
}
