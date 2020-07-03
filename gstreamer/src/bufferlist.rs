// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, ToGlib};
use gst_sys;
use std::fmt;
use std::ptr;

use Buffer;
use BufferRef;

gst_define_mini_object_wrapper!(BufferList, BufferListRef, gst_sys::GstBufferList, || {
    gst_sys::gst_buffer_list_get_type()
});

impl BufferList {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_sys::gst_buffer_list_new()) }
    }

    pub fn new_sized(size: usize) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_sys::gst_buffer_list_new_sized(size as u32)) }
    }
}

impl BufferListRef {
    pub fn insert(&mut self, idx: i32, buffer: Buffer) {
        unsafe {
            gst_sys::gst_buffer_list_insert(self.as_mut_ptr(), idx, buffer.into_ptr());
        }
    }

    pub fn add(&mut self, buffer: Buffer) {
        self.insert(-1, buffer);
    }

    pub fn copy_deep(&self) -> BufferList {
        unsafe { from_glib_full(gst_sys::gst_buffer_list_copy_deep(self.as_ptr())) }
    }

    pub fn remove(&mut self, idx: u32, len: u32) {
        unsafe { gst_sys::gst_buffer_list_remove(self.as_mut_ptr(), idx, len) }
    }

    pub fn get(&self, idx: u32) -> Option<&BufferRef> {
        unsafe {
            let ptr = gst_sys::gst_buffer_list_get(self.as_mut_ptr(), idx);
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_owned(&self, idx: u32) -> Option<Buffer> {
        unsafe {
            let ptr = gst_sys::gst_buffer_list_get(self.as_mut_ptr(), idx);
            from_glib_none(ptr)
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_writable(&mut self, idx: u32) -> Option<&mut BufferRef> {
        unsafe {
            let ptr = gst_sys::gst_buffer_list_get_writable(self.as_mut_ptr(), idx);
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn len(&self) -> usize {
        unsafe { gst_sys::gst_buffer_list_length(self.as_mut_ptr()) as usize }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn calculate_size(&self) -> usize {
        unsafe { gst_sys::gst_buffer_list_calculate_size(self.as_mut_ptr()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_owned(&self) -> IterOwned {
        IterOwned::new(self)
    }

    pub fn foreach<F: FnMut(&BufferRef, u32) -> bool>(&self, func: F) -> bool {
        unsafe extern "C" fn trampoline<F: FnMut(&BufferRef, u32) -> bool>(
            buffer: *mut *mut gst_sys::GstBuffer,
            idx: u32,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let func = user_data as *const _ as usize as *mut F;
            let res = (*func)(BufferRef::from_ptr(*buffer), idx);

            res.to_glib()
        }

        unsafe {
            let func_ptr: &F = &func;

            from_glib(gst_sys::gst_buffer_list_foreach(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *const _ as usize as *mut _,
            ))
        }
    }

    pub fn foreach_mut<F: FnMut(Buffer, u32) -> Result<Option<Buffer>, Option<Buffer>>>(
        &mut self,
        func: F,
    ) -> bool {
        unsafe extern "C" fn trampoline<
            F: FnMut(Buffer, u32) -> Result<Option<Buffer>, Option<Buffer>>,
        >(
            buffer: *mut *mut gst_sys::GstBuffer,
            idx: u32,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let func = user_data as *const _ as usize as *mut F;
            let res = (*func)(Buffer::from_glib_full(*buffer), idx);

            match res {
                Ok(None) | Err(None) => {
                    *buffer = ptr::null_mut();
                    res.is_ok().to_glib()
                }
                Ok(Some(b)) => {
                    *buffer = b.into_ptr();
                    glib_sys::GTRUE
                }
                Err(Some(b)) => {
                    *buffer = b.into_ptr();
                    glib_sys::GFALSE
                }
            }
        }

        unsafe {
            let func_ptr: &F = &func;

            from_glib(gst_sys::gst_buffer_list_foreach(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *const _ as usize as *mut _,
            ))
        }
    }
}

impl Default for BufferList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BufferList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        BufferListRef::fmt(self, f)
    }
}

impl fmt::Debug for BufferListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.iter().map(|b| b.get_size()).sum::<usize>();
        let (pts, dts) = self
            .get(0)
            .map(|b| (b.get_pts(), b.get_dts()))
            .unwrap_or((::ClockTime::none(), ::ClockTime::none()));

        f.debug_struct("BufferList")
            .field("ptr", unsafe { &self.as_ptr() })
            .field("buffers", &self.len())
            .field("pts", &pts.to_string())
            .field("dts", &dts.to_string())
            .field("size", &size)
            .finish()
    }
}

macro_rules! define_iter(
    ($name:ident, $styp:ty, $get_item:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        list: &'a BufferListRef,
        idx: u32,
        size: u32,
    }

    impl<'a> $name<'a> {
        fn new(list: &'a BufferListRef) -> $name<'a> {
            skip_assert_initialized!();
            $name {
                list,
                idx: 0,
                size: list.len() as u32,
            }
        }
    }

    impl<'a> Iterator for $name<'a> {
        type Item = $styp;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.size {
                return None;
            }

            let item = $get_item(self.list, self.idx)?;
            self.idx += 1;

            Some(item)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            if self.idx == self.size {
                return (0, Some(0));
            }

            let remaining = (self.size - self.idx) as usize;

            (remaining, Some(remaining))
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.size {
                return None;
            }

            self.size -= 1;
            $get_item(self.list, self.size)
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    }
);

define_iter!(Iter, &'a BufferRef, |list: &'a BufferListRef, idx| {
    list.get(idx)
});

define_iter!(IterOwned, Buffer, |list: &BufferListRef, idx| {
    list.get_owned(idx)
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreach() {
        ::init().unwrap();

        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();
            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(::ClockTime::from(0));
            buffer_list.add(buffer);

            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(::SECOND);
            buffer_list.add(buffer);
        }

        let mut res = vec![];
        buffer_list.foreach(|buffer, idx| {
            res.push((buffer.get_pts(), idx));

            true
        });

        assert_eq!(res, &[(::ClockTime::from(0), 0), (::SECOND, 1)]);
    }

    #[test]
    fn test_foreach_mut() {
        ::init().unwrap();

        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();
            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(::ClockTime::from(0));
            buffer_list.add(buffer);

            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(::SECOND);
            buffer_list.add(buffer);

            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(2 * ::SECOND);
            buffer_list.add(buffer);
        }

        let mut res = vec![];
        buffer_list.get_mut().unwrap().foreach_mut(|buffer, idx| {
            res.push((buffer.get_pts(), idx));

            if buffer.get_pts() == ::ClockTime::from(0) {
                Ok(Some(buffer))
            } else if buffer.get_pts() == ::SECOND {
                Ok(None)
            } else {
                let mut new_buffer = Buffer::new();
                new_buffer.get_mut().unwrap().set_pts(3 * ::SECOND);
                Ok(Some(new_buffer))
            }
        });

        assert_eq!(
            res,
            &[(::ClockTime::from(0), 0), (::SECOND, 1), (2 * ::SECOND, 1)]
        );

        let mut res = vec![];
        buffer_list.foreach(|buffer, idx| {
            res.push((buffer.get_pts(), idx));

            true
        });

        assert_eq!(res, &[(::ClockTime::from(0), 0), (3 * ::SECOND, 1)]);
    }
}
