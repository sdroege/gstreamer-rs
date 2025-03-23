// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;

// rustdoc-stripper-ignore-next
/// Trait that allows accessing `Display` implementation on types external to this crate.
pub trait Displayable {
    type DisplayImpl: std::fmt::Display;

    fn display(self) -> Self::DisplayImpl;
}

#[must_use = "if unused the object lock will immediately be released"]
pub struct ObjectLockGuard<'a, T: ?Sized> {
    obj: &'a T,
    mutex: &'a mut glib::ffi::GMutex,
}

impl<'a, T> ObjectLockGuard<'a, T>
where
    T: IsA<crate::Object>,
{
    #[inline]
    pub fn acquire(obj: &'a T) -> ObjectLockGuard<'a, T> {
        skip_assert_initialized!();
        unsafe {
            let mutex = &mut (*obj.as_ref().as_ptr()).lock;
            glib::ffi::g_mutex_lock(mutex);
            Self { obj, mutex }
        }
    }
}

impl<T> AsRef<T> for ObjectLockGuard<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.obj
    }
}

impl<T> std::ops::Deref for ObjectLockGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.obj
    }
}

impl<T> std::fmt::Debug for ObjectLockGuard<'_, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.obj.fmt(f)
    }
}

impl<T> std::cmp::PartialEq for ObjectLockGuard<'_, T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.obj.eq(other)
    }
}

impl<T> std::cmp::Eq for ObjectLockGuard<'_, T> where T: std::cmp::Eq {}

impl<T> Drop for ObjectLockGuard<'_, T>
where
    T: ?Sized,
{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            glib::ffi::g_mutex_unlock(self.mutex);
        }
    }
}

macro_rules! define_fixed_size_iter(
    ($name:ident, $typ:ty, $ityp:ty, $get_len:expr, $get_item:expr) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            pub(crate) collection: $typ,
            idx: usize,
            size: usize,
        }

        impl<'a> $name<'a> {
            #[inline]
            fn new(collection: $typ) -> $name<'a> {
                skip_assert_initialized!();
                let size = $get_len(collection) as usize;
                $name {
                    collection,
                    idx: 0,
                    size,
                }
            }
        }

        impl<'a> Iterator for $name<'a> {
            type Item = $ityp;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.idx >= self.size {
                    return None;
                }

                let item = $get_item(self.collection, self.idx);
                self.idx += 1;

                Some(item)
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let remaining = self.size - self.idx;

                (remaining, Some(remaining))
            }

            #[inline]
            fn count(self) -> usize {
                self.size - self.idx
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let (end, overflow) = self.idx.overflowing_add(n);
                if end >= self.size || overflow {
                    self.idx = self.size;
                    None
                } else {
                    self.idx = end + 1;
                    Some($get_item(self.collection, end))
                }
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                if self.idx == self.size {
                    None
                } else {
                    Some($get_item(self.collection, self.size - 1))
                }
            }
        }

        impl DoubleEndedIterator for $name<'_> {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.idx == self.size {
                    return None;
                }

                self.size -= 1;
                Some($get_item(self.collection, self.size))
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                let (end, overflow) = self.size.overflowing_sub(n);
                if end <= self.idx || overflow {
                    self.idx = self.size;
                    None
                } else {
                    self.size = end - 1;
                    Some($get_item(self.collection, self.size))
                }
            }
        }

        impl ExactSizeIterator for $name<'_> {}

        impl std::iter::FusedIterator for $name<'_> {}
    }
);

pub(crate) use define_fixed_size_iter;
