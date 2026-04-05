// Take a look at the license at the top of the repository in the LICENSE file.

use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::{ptr, slice};

use glib::translate::*;

use crate::{Caps, Plugin, Rank, TypeFindFactory, TypeFindProbability, ffi};

#[repr(transparent)]
#[derive(Debug)]
#[doc(alias = "GstTypeFind")]
pub struct TypeFind(ffi::GstTypeFind);

pub trait TypeFindImpl {
    fn peek(&mut self, offset: i64, size: u32) -> Option<&[u8]>;
    fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps);
    #[doc(alias = "get_length")]
    fn length(&self) -> Option<u64> {
        None
    }
}

impl TypeFind {
    #[doc(alias = "gst_type_find_register")]
    pub fn register<F>(
        plugin: Option<&Plugin>,
        name: &str,
        rank: Rank,
        extensions: Option<&str>,
        possible_caps: Option<&Caps>,
        func: F,
    ) -> Result<(), glib::error::BoolError>
    where
        F: Fn(&mut TypeFind) + Send + Sync + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let func: Box<F> = Box::new(func);
            let func = Box::into_raw(func);

            let res = ffi::gst_type_find_register(
                plugin.to_glib_none().0,
                name.to_glib_none().0,
                rank.into_glib() as u32,
                Some(type_find_trampoline::<F>),
                extensions.to_glib_none().0,
                possible_caps.to_glib_none().0,
                func as *mut _,
                Some(type_find_closure_drop::<F>),
            );

            glib::result_from_gboolean!(res, "Failed to register typefind factory")
        }
    }

    #[doc(alias = "gst_type_find_peek")]
    pub fn peek_var(&mut self, offset: i64, size: u32) -> Option<&[u8]> {
        assert!(size > 0);

        unsafe {
            let data = ffi::gst_type_find_peek(&mut self.0, offset, size);
            if data.is_null() {
                None
            } else {
                Some(slice::from_raw_parts(data, size as usize))
            }
        }
    }

    #[doc(alias = "gst_type_find_peek")]
    pub fn peek<const S: usize>(&mut self, offset: i64) -> Option<&[u8; S]> {
        assert!(S <= u32::MAX as usize);
        assert!(S > 0);

        unsafe {
            let data = ffi::gst_type_find_peek(&mut self.0, offset, S as u32);
            if data.is_null() {
                None
            } else {
                Some(&*(data as *const [u8; S]))
            }
        }
    }

    #[doc(alias = "gst_type_find_suggest")]
    pub fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps) {
        unsafe {
            ffi::gst_type_find_suggest(
                &mut self.0,
                probability.into_glib() as u32,
                caps.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_length")]
    #[doc(alias = "gst_type_find_get_length")]
    pub fn length(&mut self) -> Option<u64> {
        unsafe {
            let len = ffi::gst_type_find_get_length(&mut self.0);
            if len == 0 { None } else { Some(len) }
        }
    }

    pub fn as_reader(&mut self) -> TypeFindReader<'_> {
        TypeFindReader::from(self)
    }
}

pub struct TypeFindReader<'a> {
    buf: &'a mut TypeFind,
    pos: u64,
}

impl<'a> TypeFindReader<'a> {
    pub fn into_typefind(self) -> &'a mut TypeFind {
        self.buf
    }

    pub fn as_typefind(&mut self) -> &mut TypeFind {
        self.buf
    }

    fn len(&mut self) -> u64 {
        self.buf.length().unwrap_or(0)
    }
}

impl<'a> From<&'a mut TypeFind> for TypeFindReader<'a> {
    fn from(buf: &'a mut TypeFind) -> Self {
        skip_assert_initialized!();
        Self { buf, pos: 0 }
    }
}

impl Read for TypeFindReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Negative positions are used for reading relative to the end
        // of the buffer, which makes no sense in this context so consider
        // this situation EOF.
        if self.pos > i64::MAX as u64 {
            return Ok(0);
        }

        // First do a unchecked peek as a fast path before calling into len()
        let max_len = buf.len().min(u32::MAX as usize);
        if let Some(v) = self.buf.peek_var(self.pos as i64, max_len as u32) {
            buf[..max_len].copy_from_slice(v);
            // pos < i64::MAX so can't possibly overflow
            self.pos += max_len as u64;
            return Ok(max_len);
        }

        // Read failed, less data might be available.
        let remaining = match self.len().checked_sub(self.pos) {
            Some(v) => v,
            None => return Ok(0),
        };

        // Try reading the remaining data.
        let remaining = remaining.min(u32::MAX as u64) as usize;
        let max_len = remaining.min(buf.len());
        if let Some(v) = self.buf.peek_var(self.pos as i64, max_len as u32) {
            buf[..max_len].copy_from_slice(v);
            // pos < i64::MAX so can't possibly overflow
            self.pos += max_len as u64;
            return Ok(max_len);
        }

        Ok(0)
    }
}

impl Seek for TypeFindReader<'_> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match pos {
            SeekFrom::Start(v) => {
                if v >= i64::MAX as u64 {
                    return Err(Error::from(ErrorKind::FileTooLarge));
                }

                let len = self.len();
                let v = len.min(v);
                self.pos = v;
            }
            SeekFrom::End(v) => {
                let Some(v) = self.len().checked_add_signed(v) else {
                    return Err(Error::from(ErrorKind::InvalidInput));
                };
                if v >= i64::MAX as u64 {
                    return Err(Error::from(ErrorKind::FileTooLarge));
                }

                self.pos = v;
            }
            SeekFrom::Current(v) => {
                let Some(v) = self.pos.checked_add_signed(v) else {
                    return Err(Error::from(ErrorKind::InvalidInput));
                };
                if v >= i64::MAX as u64 {
                    return Err(Error::from(ErrorKind::FileTooLarge));
                }

                let len = self.len();
                let v = len.min(v);
                self.pos = v;
            }
        };

        Ok(self.pos)
    }
}

impl TypeFindFactory {
    #[doc(alias = "gst_type_find_factory_call_function")]
    pub fn call_function<T: TypeFindImpl + ?Sized>(&self, mut find: &mut T) {
        unsafe {
            let find_ptr = &mut find as *mut &mut T as glib::ffi::gpointer;
            let mut find = ffi::GstTypeFind {
                peek: Some(type_find_peek::<T>),
                suggest: Some(type_find_suggest::<T>),
                data: find_ptr,
                get_length: Some(type_find_get_length::<T>),
                _gst_reserved: [ptr::null_mut(); 4],
            };

            ffi::gst_type_find_factory_call_function(self.to_glib_none().0, &mut find)
        }
    }
}

unsafe extern "C" fn type_find_trampoline<F: Fn(&mut TypeFind) + Send + Sync + 'static>(
    find: *mut ffi::GstTypeFind,
    user_data: glib::ffi::gpointer,
) {
    unsafe {
        let func: &F = &*(user_data as *const F);

        let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            func(&mut *(find as *mut TypeFind));
        }));

        if let Err(err) = panic_result {
            let cause = err
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()));
            if let Some(cause) = cause {
                crate::error!(
                    crate::CAT_RUST,
                    "Failed to call typefind function due to panic: {}",
                    cause
                );
            } else {
                crate::error!(
                    crate::CAT_RUST,
                    "Failed to call typefind function due to panic"
                );
            }
        }
    }
}

unsafe extern "C" fn type_find_closure_drop<F: Fn(&mut TypeFind) + Send + Sync + 'static>(
    data: glib::ffi::gpointer,
) {
    unsafe {
        let _ = Box::<F>::from_raw(data as *mut _);
    }
}

unsafe extern "C" fn type_find_peek<T: TypeFindImpl + ?Sized>(
    data: glib::ffi::gpointer,
    offset: i64,
    size: u32,
) -> *const u8 {
    unsafe {
        let find = &mut *(data as *mut &mut T);

        let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match find.peek(offset, size) {
                None => ptr::null(),
                Some(data) => data.as_ptr(),
            }
        }));

        match panic_result {
            Ok(res) => res,
            Err(err) => {
                let cause = err
                    .downcast_ref::<&str>()
                    .copied()
                    .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()));
                if let Some(cause) = cause {
                    crate::error!(
                        crate::CAT_RUST,
                        "Failed to call typefind peek function due to panic: {}",
                        cause
                    );
                } else {
                    crate::error!(
                        crate::CAT_RUST,
                        "Failed to call typefind peek function due to panic"
                    );
                }

                ptr::null()
            }
        }
    }
}

unsafe extern "C" fn type_find_suggest<T: TypeFindImpl + ?Sized>(
    data: glib::ffi::gpointer,
    probability: u32,
    caps: *mut ffi::GstCaps,
) {
    unsafe {
        let find = &mut *(data as *mut &mut T);

        let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            find.suggest(from_glib(probability as i32), &from_glib_borrow(caps));
        }));

        if let Err(err) = panic_result {
            let cause = err
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()));
            if let Some(cause) = cause {
                crate::error!(
                    crate::CAT_RUST,
                    "Failed to call typefind suggest function due to panic: {}",
                    cause
                );
            } else {
                crate::error!(
                    crate::CAT_RUST,
                    "Failed to call typefind suggest function due to panic"
                );
            }
        }
    }
}

unsafe extern "C" fn type_find_get_length<T: TypeFindImpl + ?Sized>(
    data: glib::ffi::gpointer,
) -> u64 {
    unsafe {
        let find = &*(data as *mut &mut T);

        let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            find.length().unwrap_or(u64::MAX)
        }));

        match panic_result {
            Ok(res) => res,
            Err(err) => {
                let cause = err
                    .downcast_ref::<&str>()
                    .copied()
                    .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()));
                if let Some(cause) = cause {
                    crate::error!(
                        crate::CAT_RUST,
                        "Failed to call typefind length function due to panic: {}",
                        cause
                    );
                } else {
                    crate::error!(
                        crate::CAT_RUST,
                        "Failed to call typefind length function due to panic"
                    );
                }

                u64::MAX
            }
        }
    }
}

#[derive(Debug)]
pub struct SliceTypeFind<T: AsRef<[u8]>> {
    pub probability: Option<TypeFindProbability>,
    pub caps: Option<Caps>,
    data: T,
}

impl<T: AsRef<[u8]>> SliceTypeFind<T> {
    pub fn new(data: T) -> SliceTypeFind<T> {
        assert_initialized_main_thread!();
        SliceTypeFind {
            probability: None,
            caps: None,
            data,
        }
    }

    pub fn run(&mut self) {
        let factories = TypeFindFactory::factories();

        for factory in factories {
            factory.call_function(self);
            if let Some(prob) = self.probability
                && prob >= TypeFindProbability::Maximum
            {
                break;
            }
        }
    }

    pub fn type_find(data: T) -> (TypeFindProbability, Option<Caps>) {
        assert_initialized_main_thread!();
        let mut t = SliceTypeFind {
            probability: None,
            caps: None,
            data,
        };

        t.run();

        (t.probability.unwrap_or(TypeFindProbability::None), t.caps)
    }
}

impl<T: AsRef<[u8]>> TypeFindImpl for SliceTypeFind<T> {
    fn peek(&mut self, offset: i64, size: u32) -> Option<&[u8]> {
        let data = self.data.as_ref();
        let len = data.len();

        let offset = if offset >= 0 {
            usize::try_from(offset).ok()?
        } else {
            let offset = usize::try_from(offset.unsigned_abs()).ok()?;
            if len < offset {
                return None;
            }

            len - offset
        };

        let size = usize::try_from(size).ok()?;
        let end_offset = offset.checked_add(size)?;
        if end_offset <= len {
            Some(&data[offset..end_offset])
        } else {
            None
        }
    }

    fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps) {
        match self.probability {
            None => {
                self.probability = Some(probability);
                self.caps = Some(caps.clone());
            }
            Some(old_probability) if old_probability < probability => {
                self.probability = Some(probability);
                self.caps = Some(caps.clone());
            }
            _ => (),
        }
    }
    fn length(&self) -> Option<u64> {
        Some(self.data.as_ref().len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typefind_call_function() {
        crate::init().unwrap();

        let xml_factory = TypeFindFactory::factories()
            .into_iter()
            .find(|f| {
                f.caps()
                    .map(|c| {
                        c.structure(0)
                            .map(|s| s.name() == "application/xml")
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .unwrap();

        let data = b"<?xml version=\"1.0\"?><test>test</test>";
        let data = &data[..];
        let mut typefind = SliceTypeFind::new(&data);
        xml_factory.call_function(&mut typefind);

        assert_eq!(
            typefind.caps,
            Some(Caps::builder("application/xml").build())
        );
        assert_eq!(typefind.probability, Some(TypeFindProbability::Minimum));
    }

    #[test]
    fn test_typefind_register() {
        crate::init().unwrap();

        TypeFind::register(
            None,
            "test_typefind",
            crate::Rank::PRIMARY,
            None,
            Some(&Caps::builder("test/test").build()),
            |typefind| {
                assert_eq!(typefind.length(), Some(8));
                let mut found = false;
                if let Some(data) = typefind.peek::<8>(0)
                    && data == b"abcdefgh"
                {
                    found = true;
                }

                if found {
                    typefind.suggest(
                        TypeFindProbability::Likely,
                        &Caps::builder("test/test").build(),
                    );
                }
            },
        )
        .unwrap();

        let data = b"abcdefgh";
        let data = &data[..];
        let (probability, caps) = SliceTypeFind::type_find(data);

        assert_eq!(caps, Some(Caps::builder("test/test").build()));
        assert_eq!(probability, TypeFindProbability::Likely);
    }
}
