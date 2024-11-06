// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    cmp, fmt,
    ops::{ControlFlow, RangeBounds},
    ptr,
};

use glib::translate::*;

use crate::{ffi, Buffer, BufferRef};

mini_object_wrapper!(BufferList, BufferListRef, ffi::GstBufferList, || {
    ffi::gst_buffer_list_get_type()
});

impl BufferList {
    #[doc(alias = "gst_buffer_list_new")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_list_new()) }
    }

    #[doc(alias = "gst_buffer_list_new_sized")]
    pub fn new_sized(size: usize) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_list_new_sized(u32::try_from(size).unwrap())) }
    }
}

impl BufferListRef {
    #[doc(alias = "gst_buffer_list_insert")]
    pub fn insert(&mut self, idx: impl Into<Option<usize>>, buffer: Buffer) {
        unsafe {
            let len = self.len();
            debug_assert!(len <= u32::MAX as usize);

            let idx = idx.into();
            let idx = cmp::min(idx.unwrap_or(len), len) as i32;
            ffi::gst_buffer_list_insert(self.as_mut_ptr(), idx, buffer.into_glib_ptr());
        }
    }

    #[doc(alias = "gst_buffer_list_add")]
    pub fn add(&mut self, buffer: Buffer) {
        self.insert(None, buffer);
    }

    #[doc(alias = "gst_buffer_list_copy_deep")]
    pub fn copy_deep(&self) -> BufferList {
        unsafe { from_glib_full(ffi::gst_buffer_list_copy_deep(self.as_ptr())) }
    }

    #[doc(alias = "gst_buffer_list_remove")]
    pub fn remove(&mut self, range: impl RangeBounds<usize>) {
        let n = self.len();
        debug_assert!(n <= u32::MAX as usize);

        let start_idx = match range.start_bound() {
            std::ops::Bound::Included(idx) => *idx,
            std::ops::Bound::Excluded(idx) => idx.checked_add(1).unwrap(),
            std::ops::Bound::Unbounded => 0,
        };
        assert!(start_idx < n);

        let end_idx = match range.end_bound() {
            std::ops::Bound::Included(idx) => idx.checked_add(1).unwrap(),
            std::ops::Bound::Excluded(idx) => *idx,
            std::ops::Bound::Unbounded => n,
        };
        assert!(end_idx <= n);

        unsafe {
            ffi::gst_buffer_list_remove(
                self.as_mut_ptr(),
                start_idx as u32,
                (end_idx - start_idx) as u32,
            )
        }
    }

    #[doc(alias = "gst_buffer_list_get")]
    pub fn get(&self, idx: usize) -> Option<&BufferRef> {
        unsafe {
            if idx >= self.len() {
                return None;
            }
            let ptr = ffi::gst_buffer_list_get(self.as_mut_ptr(), idx as u32);
            Some(BufferRef::from_ptr(ptr))
        }
    }

    #[doc(alias = "gst_buffer_list_get")]
    pub fn get_owned(&self, idx: usize) -> Option<Buffer> {
        unsafe {
            if idx >= self.len() {
                return None;
            }
            let ptr = ffi::gst_buffer_list_get(self.as_mut_ptr(), idx as u32);
            Some(from_glib_none(ptr))
        }
    }

    #[doc(alias = "gst_buffer_list_get_writable")]
    #[doc(alias = "get_writable")]
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut BufferRef> {
        unsafe {
            if idx >= self.len() {
                return None;
            }
            let ptr = ffi::gst_buffer_list_get_writable(self.as_mut_ptr(), idx as u32);
            Some(BufferRef::from_mut_ptr(ptr))
        }
    }

    #[doc(alias = "gst_buffer_list_length")]
    pub fn len(&self) -> usize {
        unsafe { ffi::gst_buffer_list_length(self.as_mut_ptr()) as usize }
    }

    #[doc(alias = "gst_buffer_list_calculate_size")]
    pub fn calculate_size(&self) -> usize {
        unsafe { ffi::gst_buffer_list_calculate_size(self.as_mut_ptr()) }
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

    #[doc(alias = "gst_buffer_list_foreach")]
    pub fn foreach<F: FnMut(&Buffer, usize) -> ControlFlow<(), ()>>(&self, func: F) -> bool {
        unsafe extern "C" fn trampoline<F: FnMut(&Buffer, usize) -> ControlFlow<(), ()>>(
            buffer: *mut *mut ffi::GstBuffer,
            idx: u32,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *mut F;
            let res = (*func)(&Buffer::from_glib_borrow(*buffer), idx as usize);

            matches!(res, ControlFlow::Continue(_)).into_glib()
        }

        unsafe {
            let mut func = func;
            let func_ptr: &mut F = &mut func;

            from_glib(ffi::gst_buffer_list_foreach(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *mut _ as *mut _,
            ))
        }
    }

    #[doc(alias = "gst_buffer_list_foreach")]
    pub fn foreach_mut<F: FnMut(Buffer, usize) -> ControlFlow<Option<Buffer>, Option<Buffer>>>(
        &mut self,
        func: F,
    ) -> bool {
        unsafe extern "C" fn trampoline<
            F: FnMut(Buffer, usize) -> ControlFlow<Option<Buffer>, Option<Buffer>>,
        >(
            buffer: *mut *mut ffi::GstBuffer,
            idx: u32,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *mut F;
            let res = (*func)(
                Buffer::from_glib_full(ptr::replace(
                    buffer as *mut *const ffi::GstBuffer,
                    ptr::null_mut::<ffi::GstBuffer>(),
                )),
                idx as usize,
            );

            let (cont, res_buffer) = match res {
                ControlFlow::Continue(res_buffer) => (true, res_buffer),
                ControlFlow::Break(res_buffer) => (false, res_buffer),
            };

            match res_buffer {
                None => {
                    *buffer = ptr::null_mut();
                }
                Some(new_buffer) => {
                    *buffer = new_buffer.into_glib_ptr();
                }
            }

            cont.into_glib()
        }

        unsafe {
            let mut func = func;
            let func_ptr: &mut F = &mut func;

            from_glib(ffi::gst_buffer_list_foreach(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *mut _ as *mut _,
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
        use crate::{utils::Displayable, ClockTime};

        let size = self.iter().map(|b| b.size()).sum::<usize>();
        let (pts, dts) = self
            .get(0)
            .map(|b| (b.pts(), b.dts()))
            .unwrap_or((ClockTime::NONE, ClockTime::NONE));

        f.debug_struct("BufferList")
            .field("ptr", &self.as_ptr())
            .field("buffers", &self.len())
            .field("pts", &pts.display())
            .field("dts", &dts.display())
            .field("size", &size)
            .finish()
    }
}

macro_rules! define_iter(
    ($name:ident, $styp:ty, $get_item:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        list: &'a BufferListRef,
        idx: usize,
        size: usize,
    }

    impl<'a> $name<'a> {
        fn new(list: &'a BufferListRef) -> $name<'a> {
            skip_assert_initialized!();
            $name {
                list,
                idx: 0,
                size: list.len(),
            }
        }
    }

    #[allow(clippy::redundant_closure_call)]
    impl<'a> Iterator for $name<'a> {
        type Item = $styp;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.size {
                return None;
            }

            let item = $get_item(self.list, self.idx).unwrap();
            self.idx += 1;

            Some(item)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.size - self.idx;

            (remaining, Some(remaining))
        }

        fn count(self) -> usize {
            self.size - self.idx
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.idx.overflowing_add(n);
            if end >= self.size || overflow {
                self.idx = self.size;
                None
            } else {
                self.idx = end + 1;
                Some($get_item(self.list, end).unwrap())
            }
        }

        fn last(self) -> Option<Self::Item> {
            if self.idx == self.size {
                None
            } else {
                Some($get_item(self.list, self.size - 1).unwrap())
            }
        }
    }

    #[allow(clippy::redundant_closure_call)]
    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.size {
                return None;
            }

            self.size -= 1;
            Some($get_item(self.list, self.size).unwrap())
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.size.overflowing_sub(n);
            if end <= self.idx || overflow {
                self.idx = self.size;
                None
            } else {
                self.size = end - 1;
                Some($get_item(self.list, self.size).unwrap())
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    impl<'a> std::iter::FusedIterator for $name<'a> {}
    }
);

define_iter!(Iter, &'a BufferRef, |list: &'a BufferListRef, idx| {
    list.get(idx)
});

define_iter!(IterOwned, Buffer, |list: &BufferListRef, idx| {
    list.get_owned(idx)
});

impl<'a> IntoIterator for &'a BufferListRef {
    type IntoIter = Iter<'a>;
    type Item = &'a BufferRef;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<Buffer> for BufferList {
    fn from(value: Buffer) -> Self {
        skip_assert_initialized!();

        let mut list = BufferList::new_sized(1);
        {
            let list = list.get_mut().unwrap();
            list.add(value);
        }
        list
    }
}

impl<const N: usize> From<[Buffer; N]> for BufferList {
    fn from(value: [Buffer; N]) -> Self {
        skip_assert_initialized!();

        let mut list = BufferList::new_sized(N);
        {
            let list = list.get_mut().unwrap();
            value.into_iter().for_each(|b| list.add(b));
        }
        list
    }
}

impl std::iter::FromIterator<Buffer> for BufferList {
    fn from_iter<T: IntoIterator<Item = Buffer>>(iter: T) -> Self {
        assert_initialized_main_thread!();

        let iter = iter.into_iter();

        let mut list = BufferList::new_sized(iter.size_hint().0);

        {
            let list = list.get_mut().unwrap();
            iter.for_each(|b| list.add(b));
        }

        list
    }
}

impl std::iter::Extend<Buffer> for BufferListRef {
    fn extend<T: IntoIterator<Item = Buffer>>(&mut self, iter: T) {
        iter.into_iter().for_each(|b| self.add(b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClockTime;

    fn make_buffer_list(size: usize) -> BufferList {
        skip_assert_initialized!();

        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();
            for i in 0..size {
                let mut buffer = Buffer::new();
                buffer
                    .get_mut()
                    .unwrap()
                    .set_pts(ClockTime::SECOND * i as u64);
                buffer_list.add(buffer);
            }
        }
        buffer_list
    }

    #[test]
    fn test_foreach() {
        crate::init().unwrap();

        let buffer_list = make_buffer_list(2);

        let mut res = vec![];
        buffer_list.foreach(|buffer, idx| {
            res.push((buffer.pts(), idx));
            ControlFlow::Continue(())
        });

        assert_eq!(
            res,
            &[(Some(ClockTime::ZERO), 0), (Some(ClockTime::SECOND), 1)]
        );
    }

    #[test]
    fn test_foreach_mut() {
        crate::init().unwrap();

        let mut buffer_list = make_buffer_list(3);

        let mut res = vec![];
        buffer_list.get_mut().unwrap().foreach_mut(|buffer, idx| {
            res.push((buffer.pts(), idx));

            if let Some(ClockTime::ZERO) = buffer.pts() {
                ControlFlow::Continue(Some(buffer))
            } else if let Some(ClockTime::SECOND) = buffer.pts() {
                ControlFlow::Continue(None)
            } else {
                let mut new_buffer = Buffer::new();
                new_buffer.get_mut().unwrap().set_pts(3 * ClockTime::SECOND);
                ControlFlow::Continue(Some(new_buffer))
            }
        });

        assert_eq!(
            res,
            &[
                (Some(ClockTime::ZERO), 0),
                (Some(ClockTime::SECOND), 1),
                (Some(2 * ClockTime::SECOND), 1)
            ]
        );

        let mut res = vec![];
        buffer_list.foreach(|buffer, idx| {
            res.push((buffer.pts(), idx));
            ControlFlow::Continue(())
        });

        assert_eq!(
            res,
            &[(Some(ClockTime::ZERO), 0), (Some(3 * ClockTime::SECOND), 1)]
        );

        // Try removing buffers from inside foreach_mut
        let mut buffer_list = BufferList::new();
        for i in 0..10 {
            let buffer_list = buffer_list.get_mut().unwrap();
            let mut buffer = Buffer::new();
            buffer.get_mut().unwrap().set_pts(i * ClockTime::SECOND);
            buffer_list.add(buffer);
        }

        assert_eq!(buffer_list.len(), 10);

        let buffer_list_ref = buffer_list.make_mut();

        buffer_list_ref.foreach_mut(|buf, _n| {
            let keep_packet = (buf.pts().unwrap() / ClockTime::SECOND) % 3 != 0;
            ControlFlow::Continue(keep_packet.then_some(buf))
        });

        assert_eq!(buffer_list.len(), 6);

        let res = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(res, &[1, 2, 4, 5, 7, 8]);
    }

    #[test]
    fn test_remove() {
        crate::init().unwrap();

        let mut buffer_list = make_buffer_list(10);

        buffer_list.make_mut().remove(0..2);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[2, 3, 4, 5, 6, 7, 8, 9]);

        buffer_list.make_mut().remove(0..=2);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[5, 6, 7, 8, 9]);

        buffer_list.make_mut().remove(2..);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[5, 6]);

        buffer_list.make_mut().remove(..);

        assert!(buffer_list.is_empty());
    }
}
