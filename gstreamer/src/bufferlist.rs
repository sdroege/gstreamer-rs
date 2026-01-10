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

    fn buffer_range_to_start_end_idx(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
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

        (start_idx, end_idx)
    }

    #[doc(alias = "gst_buffer_list_remove")]
    pub fn remove(&mut self, range: impl RangeBounds<usize>) {
        let (start_idx, end_idx) = self.buffer_range_to_start_end_idx(range);

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

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }

    pub fn iter_owned(&self) -> IterOwned<'_> {
        IterOwned::new(self)
    }

    #[doc(alias = "gst_buffer_list_foreach")]
    pub fn foreach<F: FnMut(&Buffer, usize) -> ControlFlow<(), ()>>(&self, func: F) {
        unsafe extern "C" fn trampoline<F: FnMut(&Buffer, usize) -> ControlFlow<(), ()>>(
            buffer: *mut *mut ffi::GstBuffer,
            idx: u32,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            unsafe {
                let func = user_data as *mut F;
                let res = (*func)(&Buffer::from_glib_borrow(*buffer), idx as usize);

                matches!(res, ControlFlow::Continue(_)).into_glib()
            }
        }

        unsafe {
            let mut func = func;
            let func_ptr: &mut F = &mut func;

            let _ = ffi::gst_buffer_list_foreach(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *mut _ as *mut _,
            );
        }
    }

    #[doc(alias = "gst_buffer_list_foreach")]
    pub fn foreach_mut<F: FnMut(Buffer, usize) -> ControlFlow<Option<Buffer>, Option<Buffer>>>(
        &mut self,
        func: F,
    ) {
        unsafe extern "C" fn trampoline<
            F: FnMut(Buffer, usize) -> ControlFlow<Option<Buffer>, Option<Buffer>>,
        >(
            buffer: *mut *mut ffi::GstBuffer,
            idx: u32,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            unsafe {
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
        }

        unsafe {
            let mut func = func;
            let func_ptr: &mut F = &mut func;

            let _ = ffi::gst_buffer_list_foreach(
                self.as_ptr() as *mut _,
                Some(trampoline::<F>),
                func_ptr as *mut _ as *mut _,
            );
        }
    }

    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> Drain<'_> {
        let (start_idx, end_idx) = self.buffer_range_to_start_end_idx(range);
        Drain {
            list: self,
            start_idx,
            end_idx,
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
    ($name:ident, $styp:ty, $get_item:expr_2021) => {
        crate::utils::define_fixed_size_iter!(
            $name, &'a BufferListRef, $styp,
            |collection: &BufferListRef| collection.len(),
            $get_item
        );
    }
);

define_iter!(Iter, &'a BufferRef, |list: &BufferListRef, idx| unsafe {
    let ptr = ffi::gst_buffer_list_get(list.as_mut_ptr(), idx as u32);
    BufferRef::from_ptr(ptr)
});

define_iter!(IterOwned, Buffer, |list: &BufferListRef, idx| unsafe {
    let ptr = ffi::gst_buffer_list_get(list.as_mut_ptr(), idx as u32);
    from_glib_none(ptr)
});

#[derive(Debug)]
pub struct Drain<'a> {
    list: &'a mut BufferListRef,
    start_idx: usize,
    end_idx: usize,
}

impl Iterator for Drain<'_> {
    type Item = Buffer;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start_idx >= self.end_idx {
            return None;
        }

        let buffer = unsafe {
            let buffer = Buffer::from_glib_none(ffi::gst_buffer_list_get(
                self.list.as_mut_ptr(),
                self.start_idx as u32,
            ));
            ffi::gst_buffer_list_remove(self.list.as_mut_ptr(), self.start_idx as u32, 1);
            buffer
        };

        self.end_idx -= 1;

        Some(buffer)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end_idx - self.start_idx;

        (remaining, Some(remaining))
    }

    #[inline]
    fn count(self) -> usize {
        self.end_idx - self.start_idx
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.start_idx.overflowing_add(n);
        if end >= self.end_idx || overflow {
            unsafe {
                ffi::gst_buffer_list_remove(
                    self.list.as_mut_ptr(),
                    self.start_idx as u32,
                    (self.end_idx - self.start_idx) as u32,
                );
            }
            self.start_idx = self.end_idx;
            None
        } else {
            let buffer = unsafe {
                let buffer = Buffer::from_glib_none(ffi::gst_buffer_list_get(
                    self.list.as_mut_ptr(),
                    end as u32,
                ));
                ffi::gst_buffer_list_remove(
                    self.list.as_mut_ptr(),
                    self.start_idx as u32,
                    n as u32,
                );
                buffer
            };
            self.end_idx -= n;
            Some(buffer)
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        if self.start_idx == self.end_idx {
            None
        } else {
            let buffer = unsafe {
                let buffer = Buffer::from_glib_none(ffi::gst_buffer_list_get(
                    self.list.as_mut_ptr(),
                    self.end_idx as u32 - 1,
                ));
                ffi::gst_buffer_list_remove(
                    self.list.as_mut_ptr(),
                    self.start_idx as u32,
                    (self.end_idx - self.start_idx) as u32,
                );
                buffer
            };
            self.end_idx = self.start_idx;
            Some(buffer)
        }
    }
}

impl DoubleEndedIterator for Drain<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start_idx == self.end_idx {
            return None;
        }

        self.end_idx -= 1;
        let buffer = unsafe {
            let buffer = Buffer::from_glib_none(ffi::gst_buffer_list_get(
                self.list.as_mut_ptr(),
                self.end_idx as u32,
            ));
            ffi::gst_buffer_list_remove(self.list.as_mut_ptr(), self.end_idx as u32, 1);
            buffer
        };

        Some(buffer)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.end_idx.overflowing_sub(n);
        if end <= self.start_idx || overflow {
            unsafe {
                ffi::gst_buffer_list_remove(
                    self.list.as_mut_ptr(),
                    self.start_idx as u32,
                    (self.end_idx - self.start_idx) as u32,
                );
            }
            self.start_idx = self.end_idx;
            None
        } else {
            self.end_idx = end - 1;
            let buffer = unsafe {
                let buffer = Buffer::from_glib_none(ffi::gst_buffer_list_get(
                    self.list.as_mut_ptr(),
                    self.end_idx as u32,
                ));
                ffi::gst_buffer_list_remove(self.list.as_mut_ptr(), self.end_idx as u32, n as u32);
                buffer
            };

            Some(buffer)
        }
    }
}

impl ExactSizeIterator for Drain<'_> {}

impl std::iter::FusedIterator for Drain<'_> {}

impl Drop for Drain<'_> {
    fn drop(&mut self) {
        if self.start_idx >= self.end_idx {
            return;
        }

        unsafe {
            ffi::gst_buffer_list_remove(
                self.list.as_mut_ptr(),
                self.start_idx as u32,
                (self.end_idx - self.start_idx) as u32,
            );
        }
    }
}

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
            let keep_packet = !(buf.pts().unwrap() / ClockTime::SECOND).is_multiple_of(3);
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

    #[test]
    fn test_drain() {
        crate::init().unwrap();

        let mut buffer_list = make_buffer_list(10);

        let buffers_removed = buffer_list
            .make_mut()
            .drain(0..2)
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_removed, &[0, 1]);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[2, 3, 4, 5, 6, 7, 8, 9]);

        let buffers_removed = buffer_list
            .make_mut()
            .drain(0..=2)
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_removed, &[2, 3, 4]);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[5, 6, 7, 8, 9]);

        let buffers_removed = buffer_list
            .make_mut()
            .drain(2..)
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_removed, &[7, 8, 9]);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[5, 6]);

        let buffers_removed = buffer_list
            .make_mut()
            .drain(..)
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_removed, &[5, 6]);

        assert!(buffer_list.is_empty());
    }

    #[test]
    fn test_drain_drop() {
        crate::init().unwrap();

        let mut buffer_list = make_buffer_list(10);

        buffer_list.make_mut().drain(0..2);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[2, 3, 4, 5, 6, 7, 8, 9]);

        buffer_list.make_mut().drain(0..=2);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[5, 6, 7, 8, 9]);

        buffer_list.make_mut().drain(2..);

        let buffers_left = buffer_list
            .iter()
            .map(|buf| buf.pts().unwrap() / ClockTime::SECOND)
            .collect::<Vec<_>>();

        assert_eq!(buffers_left, &[5, 6]);

        buffer_list.make_mut().drain(..);

        assert!(buffer_list.is_empty());
    }
}
