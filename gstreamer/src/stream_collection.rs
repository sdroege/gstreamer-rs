// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
    object::ObjectType as ObjectType_,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};

use crate::{Stream, StreamCollection};

#[derive(Debug)]
pub struct Iter<'a> {
    collection: &'a StreamCollection,
    idx: usize,
    size: usize,
}

impl<'a> Iter<'a> {
    fn new(collection: &'a StreamCollection) -> Iter<'a> {
        skip_assert_initialized!();
        Iter {
            collection,
            idx: 0,
            size: collection.len(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Stream;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.collection.stream(self.idx as u32).unwrap();
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
            Some(self.collection.stream(end as u32).unwrap())
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.size {
            None
        } else {
            Some(self.collection.stream(self.size as u32 - 1).unwrap())
        }
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        Some(self.collection.stream(self.size as u32).unwrap())
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.size.overflowing_sub(n);
        if end <= self.idx || overflow {
            self.idx = self.size;
            None
        } else {
            self.size = end - 1;
            Some(self.collection.stream(self.size as u32).unwrap())
        }
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> std::iter::FusedIterator for Iter<'a> {}

#[derive(Debug, Clone)]
#[must_use = "The builder must be built to be used"]
pub struct StreamCollectionBuilder(StreamCollection);

impl StreamCollectionBuilder {
    #[doc(alias = "gst_stream_collection_add_stream")]
    pub fn stream(self, stream: Stream) -> Self {
        unsafe {
            ffi::gst_stream_collection_add_stream(
                (self.0).to_glib_none().0,
                stream.into_glib_ptr(),
            );
        }

        self
    }

    pub fn streams(self, streams: impl IntoIterator<Item = Stream>) -> Self {
        for stream in streams.into_iter() {
            unsafe {
                ffi::gst_stream_collection_add_stream(
                    (self.0).to_glib_none().0,
                    stream.into_glib_ptr(),
                );
            }
        }

        self
    }

    #[must_use = "Building the stream collection without using it has no effect"]
    pub fn build(self) -> StreamCollection {
        self.0
    }
}

impl StreamCollection {
    #[doc(alias = "gst_stream_collection_new")]
    pub fn builder(upstream_id: Option<&str>) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        let upstream_id = upstream_id.to_glib_none();
        let collection = unsafe { from_glib_full(ffi::gst_stream_collection_new(upstream_id.0)) };

        StreamCollectionBuilder(collection)
    }

    #[doc(alias = "stream-notify")]
    pub fn connect_stream_notify<
        F: Fn(&Self, &Stream, &glib::ParamSpec) + Send + Sync + 'static,
    >(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn stream_notify_trampoline<
            F: Fn(&StreamCollection, &Stream, &glib::ParamSpec) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstStreamCollection,
            object: *mut ffi::GstStream,
            p0: *mut glib::gobject_ffi::GParamSpec,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(object),
                &from_glib_borrow(p0),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("stream-notify::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"stream-notify\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stream_notify_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn len(&self) -> usize {
        self.size() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}

impl<'a> IntoIterator for &'a StreamCollection {
    type IntoIter = Iter<'a>;
    type Item = Stream;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Debug<'a>(&'a StreamCollection);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Streams<'a>(&'a StreamCollection);

        impl<'a> fmt::Debug for Streams<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut f = f.debug_list();

                for stream in self.0.iter() {
                    f.entry(&stream.debug());
                }

                f.finish()
            }
        }

        let streams = Streams(self.0);

        f.debug_struct("StreamCollection")
            .field("streams", &streams)
            .finish()
    }
}
