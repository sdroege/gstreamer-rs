// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
    object::ObjectType as ObjectType_,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};

use crate::{ffi, Stream, StreamCollection};

crate::utils::define_fixed_size_iter!(
    Iter,
    &'a StreamCollection,
    Stream,
    |collection: &StreamCollection| collection.len(),
    |collection: &StreamCollection, idx: usize| unsafe {
        from_glib_none(ffi::gst_stream_collection_get_stream(
            collection.to_glib_none().0,
            idx as u32,
        ))
    }
);

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

    #[doc(alias = "gst_stream_collection_add_stream")]
    pub fn stream_if(self, stream: Stream, predicate: bool) -> Self {
        if predicate {
            unsafe {
                ffi::gst_stream_collection_add_stream(
                    (self.0).to_glib_none().0,
                    stream.into_glib_ptr(),
                );
            }

            self
        } else {
            self
        }
    }

    #[doc(alias = "gst_stream_collection_add_stream")]
    pub fn stream_if_some(self, stream: Option<Stream>) -> Self {
        if let Some(stream) = stream {
            self.stream(stream)
        } else {
            self
        }
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

    pub fn streams_if(self, streams: impl IntoIterator<Item = Stream>, predicate: bool) -> Self {
        if predicate {
            for stream in streams.into_iter() {
                unsafe {
                    ffi::gst_stream_collection_add_stream(
                        (self.0).to_glib_none().0,
                        stream.into_glib_ptr(),
                    );
                }
            }

            self
        } else {
            self
        }
    }

    pub fn streams_if_some(self, streams: Option<impl IntoIterator<Item = Stream>>) -> Self {
        if let Some(streams) = streams {
            self.streams(streams)
        } else {
            self
        }
    }

    pub fn streams_if_not_empty(self, streams: impl IntoIterator<Item = Stream>) -> Self {
        let mut streams = streams.into_iter().peekable();
        if streams.peek().is_some() {
            self.streams(streams)
        } else {
            self
        }
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
            let detailed_signal_name = detail.map(|name| format!("stream-notify::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"stream-notify\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<*const (), unsafe extern "C" fn()>(
                    stream_notify_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }

    pub fn len(&self) -> usize {
        self.size() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn debug(&self) -> Debug<'_> {
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

impl fmt::Debug for Debug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Streams<'a>(&'a StreamCollection);

        impl fmt::Debug for Streams<'_> {
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
