// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    fmt,
    ops::{Bound, RangeBounds},
};

pub trait ByteSliceExt {
    #[doc(alias = "gst_util_dump_mem")]
    fn dump(&self) -> Dump;
    #[doc(alias = "gst_util_dump_mem")]
    fn dump_range(&self, range: impl RangeBounds<usize>) -> Dump;
}

impl<'a> ByteSliceExt for &'a [u8] {
    fn dump(&self) -> Dump {
        self.dump_range(..)
    }

    fn dump_range(&self, range: impl RangeBounds<usize>) -> Dump {
        Dump {
            data: self,
            start: range.start_bound().cloned(),
            end: range.end_bound().cloned(),
        }
    }
}

pub struct Dump<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) start: Bound<usize>,
    pub(crate) end: Bound<usize>,
}

impl<'a> Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter, debug: bool) -> fmt::Result {
        use std::fmt::Write;

        let data = self.data;
        let len = data.len();

        // Kind of re-implementation of slice indexing to allow handling out of range values better
        // with specific output strings
        let mut start_idx = match self.start {
            Bound::Included(idx) if idx >= len => {
                write!(f, "<start out of range>")?;
                return Ok(());
            }
            Bound::Excluded(idx) if idx.checked_add(1).map_or(true, |idx| idx >= len) => {
                write!(f, "<start out of range>")?;
                return Ok(());
            }
            Bound::Included(idx) => idx,
            Bound::Excluded(idx) => idx + 1,
            Bound::Unbounded => 0,
        };

        let end_idx = match self.end {
            Bound::Included(idx) if idx.checked_add(1).map_or(true, |idx| idx > len) => {
                write!(f, "<end out of range>")?;
                return Ok(());
            }
            Bound::Excluded(idx) if idx > len => {
                write!(f, "<end out of range>")?;
                return Ok(());
            }
            Bound::Included(idx) => idx + 1,
            Bound::Excluded(idx) => idx,
            Bound::Unbounded => len,
        };

        if start_idx >= end_idx {
            write!(f, "<empty range>")?;
            return Ok(());
        }

        let data = &data[start_idx..end_idx];

        if debug {
            for line in data.chunks(16) {
                match end_idx {
                    0x00_00..=0xff_ff => write!(f, "{:04x}:  ", start_idx)?,
                    0x01_00_00..=0xff_ff_ff => write!(f, "{:06x}:  ", start_idx)?,
                    0x01_00_00_00..=0xff_ff_ff_ff => write!(f, "{:08x}:  ", start_idx)?,
                    _ => write!(f, "{:016x}:  ", start_idx)?,
                }

                for (i, v) in line.iter().enumerate() {
                    if i > 0 {
                        write!(f, " {:02x}", v)?;
                    } else {
                        write!(f, "{:02x}", v)?;
                    }
                }

                for _ in line.len()..16 {
                    write!(f, "   ")?;
                }
                write!(f, "   ")?;

                for v in line {
                    if v.is_ascii() && !v.is_ascii_control() {
                        f.write_char((*v).into())?;
                    } else {
                        f.write_char('.')?;
                    }
                }

                start_idx = start_idx.saturating_add(16);
                if start_idx < end_idx {
                    writeln!(f)?;
                }
            }

            Ok(())
        } else {
            for line in data.chunks(16) {
                for (i, v) in line.iter().enumerate() {
                    if i > 0 {
                        write!(f, " {:02x}", v)?;
                    } else {
                        write!(f, "{:02x}", v)?;
                    }
                }

                start_idx = start_idx.saturating_add(16);
                if start_idx < end_idx {
                    writeln!(f)?;
                }
            }

            Ok(())
        }
    }
}

impl<'a> fmt::Display for Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f, false)
    }
}

impl<'a> fmt::Debug for Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f, true)
    }
}
