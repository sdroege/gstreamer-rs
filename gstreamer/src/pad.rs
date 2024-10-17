// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    mem,
    num::NonZeroU64,
    ops::ControlFlow,
    panic::{self, AssertUnwindSafe},
    ptr,
};

use glib::{ffi::gpointer, prelude::*, translate::*};

use crate::{
    ffi,
    format::{FormattedValue, SpecificFormattedValueFullRange, SpecificFormattedValueIntrinsic},
    prelude::*,
    Buffer, BufferList, Event, FlowError, FlowReturn, FlowSuccess, Format, GenericFormattedValue,
    LoggableError, Pad, PadFlags, PadProbeReturn, PadProbeType, Query, QueryRef, StaticPadTemplate,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PadProbeId(NonZeroU64);

impl IntoGlib for PadProbeId {
    type GlibType = libc::c_ulong;

    #[inline]
    fn into_glib(self) -> libc::c_ulong {
        self.0.get() as libc::c_ulong
    }
}

impl FromGlib<libc::c_ulong> for PadProbeId {
    #[inline]
    unsafe fn from_glib(val: libc::c_ulong) -> PadProbeId {
        skip_assert_initialized!();
        debug_assert_ne!(val, 0);
        PadProbeId(NonZeroU64::new_unchecked(val as _))
    }
}

impl PadProbeId {
    #[inline]
    pub fn as_raw(&self) -> libc::c_ulong {
        self.0.get() as libc::c_ulong
    }
}

#[doc(alias = "GstPadProbeInfo")]
#[derive(Debug)]
pub struct PadProbeInfo<'a> {
    pub mask: PadProbeType,
    pub id: Option<PadProbeId>,
    pub offset: u64,
    pub size: u32,
    pub data: Option<PadProbeData<'a>>,
    pub flow_res: Result<FlowSuccess, FlowError>,
}

impl<'a> PadProbeInfo<'a> {
    pub fn buffer(&self) -> Option<&Buffer> {
        match self.data {
            Some(PadProbeData::Buffer(ref buffer)) => Some(buffer),
            _ => None,
        }
    }

    pub fn buffer_mut(&mut self) -> Option<&mut Buffer> {
        match self.data {
            Some(PadProbeData::Buffer(ref mut buffer)) => Some(buffer),
            _ => None,
        }
    }

    pub fn buffer_list(&self) -> Option<&BufferList> {
        match self.data {
            Some(PadProbeData::BufferList(ref buffer_list)) => Some(buffer_list),
            _ => None,
        }
    }

    pub fn buffer_list_mut(&mut self) -> Option<&mut BufferList> {
        match self.data {
            Some(PadProbeData::BufferList(ref mut buffer_list)) => Some(buffer_list),
            _ => None,
        }
    }

    pub fn query(&self) -> Option<&QueryRef> {
        match self.data {
            Some(PadProbeData::Query(ref query)) => Some(*query),
            _ => None,
        }
    }

    pub fn query_mut(&mut self) -> Option<&mut QueryRef> {
        match self.data {
            Some(PadProbeData::Query(ref mut query)) => Some(*query),
            _ => None,
        }
    }

    pub fn event(&self) -> Option<&Event> {
        match self.data {
            Some(PadProbeData::Event(ref event)) => Some(event),
            _ => None,
        }
    }

    pub fn event_mut(&mut self) -> Option<&mut Event> {
        match self.data {
            Some(PadProbeData::Event(ref mut event)) => Some(event),
            _ => None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Takes over the buffer in the probe info. As the data is no longer valid for the caller, the
    /// probe will be considered dropped after this point.
    pub fn take_buffer(&mut self) -> Option<Buffer> {
        if matches!(self.data, Some(PadProbeData::Buffer(..))) {
            match self.data.take() {
                Some(PadProbeData::Buffer(b)) => Some(b),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    // rustdoc-stripper-ignore-next
    /// Takes over the buffer in the probe info. As the data is no longer valid for the caller, the
    /// probe will be considered dropped after this point.
    pub fn take_buffer_list(&mut self) -> Option<BufferList> {
        if matches!(self.data, Some(PadProbeData::BufferList(..))) {
            match self.data.take() {
                Some(PadProbeData::BufferList(b)) => Some(b),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
    // rustdoc-stripper-ignore-next
    /// Takes over the event in the probe info. As the data is no longer valid for the caller, the
    /// probe will be considered dropped after this point.
    pub fn take_event(&mut self) -> Option<Event> {
        if matches!(self.data, Some(PadProbeData::Event(..))) {
            match self.data.take() {
                Some(PadProbeData::Event(e)) => Some(e),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum PadProbeData<'a> {
    Buffer(Buffer),
    BufferList(BufferList),
    Query(&'a mut QueryRef),
    Event(Event),
    #[doc(hidden)]
    __Unknown(*mut ffi::GstMiniObject),
}

unsafe impl<'a> Send for PadProbeData<'a> {}
unsafe impl<'a> Sync for PadProbeData<'a> {}

#[derive(Debug)]
#[must_use = "if unused the StreamLock will immediately unlock"]
pub struct StreamLock<'a>(&'a Pad);
impl<'a> Drop for StreamLock<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let pad: *mut ffi::GstPad = self.0.to_glib_none().0;
            glib::ffi::g_rec_mutex_unlock(&mut (*pad).stream_rec_lock);
        }
    }
}

#[derive(Debug)]
pub enum PadGetRangeSuccess {
    FilledBuffer,
    NewBuffer(crate::Buffer),
}

#[derive(Debug)]
pub enum EventForeachAction {
    Keep,
    Remove,
    Replace(Event),
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Pad>> Sealed for T {}
}

pub trait PadExtManual: sealed::Sealed + IsA<Pad> + 'static {
    #[doc(alias = "gst_pad_add_probe")]
    fn add_probe<F>(&self, mask: PadProbeType, func: F) -> Option<PadProbeId>
    where
        F: Fn(&Self, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<F> = Box::new(func);
            let id = ffi::gst_pad_add_probe(
                self.as_ref().to_glib_none().0,
                mask.into_glib(),
                Some(trampoline_pad_probe::<Self, F>),
                Box::into_raw(func_box) as gpointer,
                Some(destroy_closure::<F>),
            );

            if id == 0 {
                None
            } else {
                Some(from_glib(id))
            }
        }
    }

    #[doc(alias = "gst_pad_remove_probe")]
    fn remove_probe(&self, id: PadProbeId) {
        unsafe {
            ffi::gst_pad_remove_probe(self.as_ref().to_glib_none().0, id.into_glib());
        }
    }

    #[doc(alias = "gst_pad_pull_range")]
    fn pull_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowError> {
        unsafe {
            let mut buffer = ptr::null_mut();
            FlowSuccess::try_from_glib(ffi::gst_pad_pull_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ))
            .map(|_| from_glib_full(buffer))
        }
    }

    fn pull_range_fill(
        &self,
        offset: u64,
        buffer: &mut crate::BufferRef,
        size: u32,
    ) -> Result<(), FlowError> {
        assert!(buffer.size() >= size as usize);

        unsafe {
            let mut buffer_ref = buffer.as_mut_ptr();
            FlowSuccess::try_from_glib(ffi::gst_pad_pull_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer_ref,
            ))
            .and_then(|_| {
                if buffer.as_mut_ptr() != buffer_ref {
                    ffi::gst_mini_object_unref(buffer_ref as *mut _);
                    Err(crate::FlowError::Error)
                } else {
                    Ok(())
                }
            })
        }
    }

    #[doc(alias = "get_range")]
    #[doc(alias = "gst_pad_get_range")]
    fn range(&self, offset: u64, size: u32) -> Result<Buffer, FlowError> {
        unsafe {
            let mut buffer = ptr::null_mut();
            FlowSuccess::try_from_glib(ffi::gst_pad_get_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ))
            .map(|_| from_glib_full(buffer))
        }
    }

    #[doc(alias = "get_range_fill")]
    fn range_fill(
        &self,
        offset: u64,
        buffer: &mut crate::BufferRef,
        size: u32,
    ) -> Result<(), FlowError> {
        assert!(buffer.size() >= size as usize);

        unsafe {
            let mut buffer_ref = buffer.as_mut_ptr();
            FlowSuccess::try_from_glib(ffi::gst_pad_get_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer_ref,
            ))
            .and_then(|_| {
                if buffer.as_mut_ptr() != buffer_ref {
                    ffi::gst_mini_object_unref(buffer_ref as *mut _);
                    Err(crate::FlowError::Error)
                } else {
                    Ok(())
                }
            })
        }
    }

    #[doc(alias = "gst_pad_peer_query")]
    fn peer_query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_peer_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_pad_query")]
    fn query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_pad_proxy_query_caps")]
    fn proxy_query_caps(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_proxy_query_caps(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_pad_proxy_query_accept_caps")]
    fn proxy_query_accept_caps(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_proxy_query_accept_caps(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_pad_push_event")]
    fn push_event(&self, event: impl Into<Event>) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_push_event(
                self.as_ref().to_glib_none().0,
                event.into().into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_pad_send_event")]
    fn send_event(&self, event: impl Into<Event>) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_send_event(
                self.as_ref().to_glib_none().0,
                event.into().into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_pad_iterate_internal_links")]
    fn iterate_internal_links(&self) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_pad_iterate_internal_links(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn stream_lock(&self) -> StreamLock {
        unsafe {
            let ptr: &mut ffi::GstPad = &mut *(self.as_ptr() as *mut _);
            glib::ffi::g_rec_mutex_lock(&mut ptr.stream_rec_lock);
            StreamLock(self.upcast_ref())
        }
    }

    #[doc(alias = "gst_pad_set_activate_function")]
    #[doc(alias = "gst_pad_set_activate_function_full")]
    unsafe fn set_activate_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_activate_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_activate_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_activatemode_function")]
    #[doc(alias = "gst_pad_set_activatemode_function_full")]
    unsafe fn set_activatemode_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::PadMode, bool) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_activatemode_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_activatemode_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_chain_function")]
    #[doc(alias = "gst_pad_set_chain_function_full")]
    unsafe fn set_chain_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::Buffer) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_chain_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_chain_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_chain_list_function")]
    #[doc(alias = "gst_pad_set_chain_list_function_full")]
    unsafe fn set_chain_list_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::BufferList) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_chain_list_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_chain_list_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_event_function")]
    #[doc(alias = "gst_pad_set_event_function_full")]
    unsafe fn set_event_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::Event) -> bool + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_event_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_event_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_event_full_function")]
    #[doc(alias = "gst_pad_set_event_full_function_full")]
    unsafe fn set_event_full_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::Event) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_event_full_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_event_full_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_getrange_function")]
    #[doc(alias = "gst_pad_set_getrange_function_full")]
    unsafe fn set_getrange_function<F>(&self, func: F)
    where
        F: Fn(
                &Self,
                Option<&crate::Object>,
                u64,
                Option<&mut crate::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, crate::FlowError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_getrange_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_getrange_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_iterate_internal_links_function")]
    #[doc(alias = "gst_pad_set_iterate_internal_links_function_full")]
    unsafe fn set_iterate_internal_links_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>) -> crate::Iterator<Pad> + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_iterate_internal_links_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_iterate_internal_links_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_link_function")]
    #[doc(alias = "gst_pad_set_link_function_full")]
    unsafe fn set_link_function<F>(&self, func: F)
    where
        F: Fn(
                &Self,
                Option<&crate::Object>,
                &Pad,
            ) -> Result<crate::PadLinkSuccess, crate::PadLinkError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_link_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_link_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_query_function")]
    #[doc(alias = "gst_pad_set_query_function_full")]
    unsafe fn set_query_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, &mut crate::QueryRef) -> bool + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_query_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_query_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_set_unlink_function")]
    #[doc(alias = "gst_pad_set_unlink_function_full")]
    unsafe fn set_unlink_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>) + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        ffi::gst_pad_set_unlink_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_unlink_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    #[doc(alias = "gst_pad_start_task")]
    fn start_task<F: FnMut() + Send + 'static>(&self, func: F) -> Result<(), glib::BoolError> {
        unsafe extern "C" fn trampoline_pad_task<F: FnMut() + Send + 'static>(func: gpointer) {
            let (func, pad) = &mut *(func as *mut (F, *mut ffi::GstPad));
            let pad = Pad::from_glib_borrow(*pad);
            let result = panic::catch_unwind(AssertUnwindSafe(func));

            if let Err(err) = result {
                let element = match pad.parent_element() {
                    Some(element) => element,
                    None => panic::resume_unwind(err),
                };

                if pad.pause_task().is_err() {
                    crate::error!(crate::CAT_RUST, "could not stop pad task on panic");
                }

                crate::subclass::post_panic_error_message(&element, pad.upcast_ref(), Some(err));
            }
        }

        fn into_raw_pad_task<F: FnMut() + Send + 'static>(
            func: F,
            pad: *mut ffi::GstPad,
        ) -> gpointer {
            #[allow(clippy::type_complexity)]
            let func: Box<(F, *mut ffi::GstPad)> = Box::new((func, pad));
            Box::into_raw(func) as gpointer
        }

        unsafe extern "C" fn destroy_closure_pad_task<F>(ptr: gpointer) {
            let _ = Box::<(F, *mut ffi::GstPad)>::from_raw(ptr as *mut _);
        }

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_start_task(
                    self.as_ref().to_glib_none().0,
                    Some(trampoline_pad_task::<F>),
                    into_raw_pad_task(func, self.upcast_ref().as_ptr()),
                    Some(destroy_closure_pad_task::<F>),
                ),
                "Failed to start pad task",
            )
        }
    }
    #[doc(alias = "gst_pad_peer_query_convert")]
    fn peer_query_convert<U: SpecificFormattedValueFullRange>(
        &self,
        src_val: impl FormattedValue,
    ) -> Option<U> {
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                U::default_format().into_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(U::from_raw(U::default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_peer_query_convert")]
    fn peer_query_convert_generic(
        &self,
        src_val: impl FormattedValue,
        dest_format: Format,
    ) -> Option<GenericFormattedValue> {
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                dest_format.into_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_peer_query_duration")]
    fn peer_query_duration<T: SpecificFormattedValueIntrinsic>(&self) -> Option<T> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_duration(
                self.as_ref().to_glib_none().0,
                T::default_format().into_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                try_from_glib(duration.assume_init()).ok()
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_peer_query_duration")]
    fn peer_query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_duration(
                self.as_ref().to_glib_none().0,
                format.into_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, duration.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_peer_query_position")]
    fn peer_query_position<T: SpecificFormattedValueIntrinsic>(&self) -> Option<T> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_position(
                self.as_ref().to_glib_none().0,
                T::default_format().into_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                try_from_glib(cur.assume_init()).ok()
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_peer_query_position")]
    fn peer_query_position_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_position(
                self.as_ref().to_glib_none().0,
                format.into_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, cur.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_query_convert")]
    fn query_convert<U: SpecificFormattedValueFullRange>(
        &self,
        src_val: impl FormattedValue,
    ) -> Option<U> {
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                U::default_format().into_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(U::from_raw(U::default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_query_convert")]
    fn query_convert_generic(
        &self,
        src_val: impl FormattedValue,
        dest_format: Format,
    ) -> Option<GenericFormattedValue> {
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().into_glib(),
                src_val.into_raw_value(),
                dest_format.into_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_query_duration")]
    fn query_duration<T: SpecificFormattedValueIntrinsic>(&self) -> Option<T> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_duration(
                self.as_ref().to_glib_none().0,
                T::default_format().into_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                try_from_glib(duration.assume_init()).ok()
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_query_duration")]
    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_duration(
                self.as_ref().to_glib_none().0,
                format.into_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, duration.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_query_position")]
    fn query_position<T: SpecificFormattedValueIntrinsic>(&self) -> Option<T> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_position(
                self.as_ref().to_glib_none().0,
                T::default_format().into_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                try_from_glib(cur.assume_init()).ok()
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_pad_query_position")]
    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_position(
                self.as_ref().to_glib_none().0,
                format.into_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, cur.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "get_mode")]
    #[doc(alias = "GST_PAD_MODE")]
    fn mode(&self) -> crate::PadMode {
        unsafe {
            let ptr: &ffi::GstPad = &*(self.as_ptr() as *const _);
            from_glib(ptr.mode)
        }
    }

    #[doc(alias = "gst_pad_sticky_events_foreach")]
    fn sticky_events_foreach<
        F: FnMut(&Event) -> ControlFlow<EventForeachAction, EventForeachAction>,
    >(
        &self,
        func: F,
    ) {
        unsafe extern "C" fn trampoline<
            F: FnMut(&Event) -> ControlFlow<EventForeachAction, EventForeachAction>,
        >(
            _pad: *mut ffi::GstPad,
            event: *mut *mut ffi::GstEvent,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *mut F;
            let res = (*func)(&from_glib_borrow(*event));

            let (do_continue, ev_action) = match res {
                ControlFlow::Continue(ev_action) => (glib::ffi::GTRUE, ev_action),
                ControlFlow::Break(ev_action) => (glib::ffi::GFALSE, ev_action),
            };

            use EventForeachAction::*;

            match ev_action {
                Keep => (), // do nothing
                Remove => {
                    ffi::gst_mini_object_unref(*event as *mut _);
                    *event = ptr::null_mut();
                }
                Replace(ev) => {
                    ffi::gst_mini_object_unref(*event as *mut _);
                    *event = ev.into_glib_ptr();
                }
            }

            do_continue
        }

        unsafe {
            let mut func = func;
            let func_ptr = &mut func as *mut F as glib::ffi::gpointer;

            ffi::gst_pad_sticky_events_foreach(
                self.as_ref().to_glib_none().0,
                Some(trampoline::<F>),
                func_ptr,
            );
        }
    }

    #[doc(alias = "gst_pad_get_sticky_event")]
    #[doc(alias = "get_sticky_event")]
    fn sticky_event<T: crate::event::StickyEventType>(&self, idx: u32) -> Option<T::Owned> {
        unsafe {
            let ptr = ffi::gst_pad_get_sticky_event(
                self.as_ref().to_glib_none().0,
                T::TYPE.into_glib(),
                idx,
            );

            if ptr.is_null() {
                None
            } else {
                Some(T::from_event(from_glib_full(ptr)))
            }
        }
    }

    fn set_pad_flags(&self, flags: PadFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_pad_flags(&self, flags: PadFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags &= !flags.into_glib();
        }
    }

    #[doc(alias = "get_pad_flags")]
    fn pad_flags(&self) -> PadFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            from_glib((*ptr).flags)
        }
    }
}

impl<O: IsA<Pad>> PadExtManual for O {}

unsafe fn create_probe_info<'a>(
    info: *mut ffi::GstPadProbeInfo,
) -> (PadProbeInfo<'a>, Option<glib::Type>) {
    let mut data_type = None;
    let flow_res = try_from_glib((*info).ABI.abi.flow_ret);
    let info = PadProbeInfo {
        mask: from_glib((*info).type_),
        id: Some(PadProbeId(NonZeroU64::new_unchecked((*info).id as _))),
        offset: (*info).offset,
        size: (*info).size,
        data: if (*info).data.is_null() {
            None
        } else {
            let data = (*info).data as *mut ffi::GstMiniObject;
            (*info).data = ptr::null_mut();
            if (*data).type_ == Buffer::static_type().into_glib() {
                data_type = Some(Buffer::static_type());
                Some(PadProbeData::Buffer(from_glib_full(
                    data as *const ffi::GstBuffer,
                )))
            } else if (*data).type_ == BufferList::static_type().into_glib() {
                data_type = Some(BufferList::static_type());
                Some(PadProbeData::BufferList(from_glib_full(
                    data as *const ffi::GstBufferList,
                )))
            } else if (*data).type_ == Query::static_type().into_glib() {
                data_type = Some(Query::static_type());
                Some(PadProbeData::Query(QueryRef::from_mut_ptr(
                    data as *mut ffi::GstQuery,
                )))
            } else if (*data).type_ == Event::static_type().into_glib() {
                data_type = Some(Event::static_type());
                Some(PadProbeData::Event(from_glib_full(
                    data as *const ffi::GstEvent,
                )))
            } else {
                Some(PadProbeData::__Unknown(data))
            }
        },
        flow_res,
    };
    (info, data_type)
}

unsafe fn update_probe_info(
    ret: PadProbeReturn,
    probe_info: PadProbeInfo,
    data_type: Option<glib::Type>,
    info: *mut ffi::GstPadProbeInfo,
) {
    if ret == PadProbeReturn::Handled {
        // Handled queries need to be returned
        // Handled buffers and buffer lists are consumed
        // No other types can safely be used here

        match probe_info.data {
            Some(PadProbeData::Query(query)) => {
                assert_eq!(data_type, Some(Query::static_type()));
                (*info).data = query.as_mut_ptr() as *mut libc::c_void;
            }
            Some(PadProbeData::Buffer(_)) => {
                assert_eq!(data_type, Some(Buffer::static_type()));
                // Buffer not consumed by probe; consume it here
            }
            Some(PadProbeData::BufferList(_)) => {
                assert_eq!(data_type, Some(BufferList::static_type()));
                // BufferList not consumed by probe; consume it here
            }
            Some(PadProbeData::Event(_)) => {
                assert_eq!(data_type, Some(Event::static_type()));
                // Event not consumed by probe; consume it here
            }
            None if data_type == Some(Buffer::static_type())
                || data_type == Some(BufferList::static_type())
                || data_type == Some(Event::static_type()) =>
            {
                // Buffer or Event consumed by probe
                (*info).data = ptr::null_mut();
            }
            other => panic!("Bad data for {data_type:?} pad probe returning Handled: {other:?}"),
        }
    } else if ret == PadProbeReturn::Drop {
        // We may have consumed the object via PadProbeInfo::take_*() functions
        match probe_info.data {
            None if data_type == Some(Buffer::static_type())
                || data_type == Some(BufferList::static_type())
                || data_type == Some(Event::static_type()) =>
            {
                (*info).data = ptr::null_mut();
            }
            _ => {
                // Nothing to do, it's going to be dropped
            }
        }
    } else {
        match probe_info.data {
            Some(PadProbeData::Buffer(buffer)) => {
                assert_eq!(data_type, Some(Buffer::static_type()));
                (*info).data = buffer.into_glib_ptr() as *mut libc::c_void;
            }
            Some(PadProbeData::BufferList(bufferlist)) => {
                assert_eq!(data_type, Some(BufferList::static_type()));
                (*info).data = bufferlist.into_glib_ptr() as *mut libc::c_void;
            }
            Some(PadProbeData::Event(event)) => {
                assert_eq!(data_type, Some(Event::static_type()));
                (*info).data = event.into_glib_ptr() as *mut libc::c_void;
            }
            Some(PadProbeData::Query(query)) => {
                assert_eq!(data_type, Some(Query::static_type()));
                (*info).data = query.as_mut_ptr() as *mut libc::c_void;
            }
            Some(PadProbeData::__Unknown(ptr)) => {
                assert_eq!(data_type, None);
                (*info).data = ptr as *mut libc::c_void;
            }
            None => {
                assert_eq!(data_type, None);
            }
        }
    }

    let flow_ret: FlowReturn = probe_info.flow_res.into();
    (*info).ABI.abi.flow_ret = flow_ret.into_glib();
}

unsafe extern "C" fn trampoline_pad_probe<
    T,
    F: Fn(&T, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
>(
    pad: *mut ffi::GstPad,
    info: *mut ffi::GstPadProbeInfo,
    func: gpointer,
) -> ffi::GstPadProbeReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*(func as *const F);

    let (mut probe_info, data_type) = create_probe_info(info);

    let ret = func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        &mut probe_info,
    );

    update_probe_info(ret, probe_info, data_type, info);

    ret.into_glib()
}

unsafe extern "C" fn trampoline_activate_function<
    T,
    F: Fn(&T, Option<&crate::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
) -> glib::ffi::gboolean
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).activatedata as *const F);

    match func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
    ) {
        Ok(()) => true,
        Err(err) => {
            err.log_with_object(&*Pad::from_glib_borrow(pad));
            false
        }
    }
    .into_glib()
}

unsafe extern "C" fn trampoline_activatemode_function<
    T,
    F: Fn(&T, Option<&crate::Object>, crate::PadMode, bool) -> Result<(), LoggableError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    mode: ffi::GstPadMode,
    active: glib::ffi::gboolean,
) -> glib::ffi::gboolean
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).activatemodedata as *const F);

    match func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib(mode),
        from_glib(active),
    ) {
        Ok(()) => true,
        Err(err) => {
            err.log_with_object(&*Pad::from_glib_borrow(pad));
            false
        }
    }
    .into_glib()
}

unsafe extern "C" fn trampoline_chain_function<
    T,
    F: Fn(&T, Option<&crate::Object>, crate::Buffer) -> Result<FlowSuccess, FlowError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    buffer: *mut ffi::GstBuffer,
) -> ffi::GstFlowReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).chaindata as *const F);

    let res: FlowReturn = func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(buffer),
    )
    .into();
    res.into_glib()
}

unsafe extern "C" fn trampoline_chain_list_function<
    T,
    F: Fn(&T, Option<&crate::Object>, crate::BufferList) -> Result<FlowSuccess, FlowError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    list: *mut ffi::GstBufferList,
) -> ffi::GstFlowReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).chainlistdata as *const F);

    let res: FlowReturn = func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(list),
    )
    .into();
    res.into_glib()
}

unsafe extern "C" fn trampoline_event_function<
    T,
    F: Fn(&T, Option<&crate::Object>, crate::Event) -> bool + Send + Sync + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    event: *mut ffi::GstEvent,
) -> glib::ffi::gboolean
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).eventdata as *const F);

    func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(event),
    )
    .into_glib()
}

unsafe extern "C" fn trampoline_event_full_function<
    T,
    F: Fn(&T, Option<&crate::Object>, crate::Event) -> Result<FlowSuccess, FlowError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    event: *mut ffi::GstEvent,
) -> ffi::GstFlowReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).eventdata as *const F);

    let res: FlowReturn = func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(event),
    )
    .into();
    res.into_glib()
}

#[allow(clippy::needless_option_as_deref)]
unsafe extern "C" fn trampoline_getrange_function<
    T,
    F: Fn(
            &T,
            Option<&crate::Object>,
            u64,
            Option<&mut crate::BufferRef>,
            u32,
        ) -> Result<PadGetRangeSuccess, crate::FlowError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    offset: u64,
    length: u32,
    buffer: *mut *mut ffi::GstBuffer,
) -> ffi::GstFlowReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).getrangedata as *const F);

    debug_assert!(!buffer.is_null());

    let pad = Pad::from_glib_borrow(pad);
    let pad = pad.unsafe_cast_ref();
    let mut passed_buffer = if (*buffer).is_null() {
        None
    } else {
        Some(crate::BufferRef::from_mut_ptr(*buffer))
    };

    match func(
        pad,
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        offset,
        passed_buffer.as_deref_mut(),
        length,
    ) {
        Ok(PadGetRangeSuccess::NewBuffer(new_buffer)) => {
            if let Some(passed_buffer) = passed_buffer {
                crate::debug!(
                    crate::CAT_PERFORMANCE,
                    obj = pad.unsafe_cast_ref::<glib::Object>(),
                    "Returned new buffer from getrange function, copying into passed buffer"
                );

                let mut map = match passed_buffer.map_writable() {
                    Ok(map) => map,
                    Err(_) => {
                        crate::error!(
                            crate::CAT_RUST,
                            obj = pad.unsafe_cast_ref::<glib::Object>(),
                            "Failed to map passed buffer writable"
                        );
                        return ffi::GST_FLOW_ERROR;
                    }
                };

                let copied_size = new_buffer.copy_to_slice(0, &mut map);
                drop(map);

                if let Err(copied_size) = copied_size {
                    passed_buffer.set_size(copied_size);
                }

                match new_buffer.copy_into(passed_buffer, crate::BUFFER_COPY_METADATA, ..) {
                    Ok(_) => FlowReturn::Ok.into_glib(),
                    Err(_) => {
                        crate::error!(
                            crate::CAT_RUST,
                            obj = pad.unsafe_cast_ref::<glib::Object>(),
                            "Failed to copy buffer metadata"
                        );

                        FlowReturn::Error.into_glib()
                    }
                }
            } else {
                *buffer = new_buffer.into_glib_ptr();
                FlowReturn::Ok.into_glib()
            }
        }
        Ok(PadGetRangeSuccess::FilledBuffer) => {
            assert!(passed_buffer.is_some());
            FlowReturn::Ok.into_glib()
        }
        Err(ret) => FlowReturn::from_error(ret).into_glib(),
    }
}

unsafe extern "C" fn trampoline_iterate_internal_links_function<
    T,
    F: Fn(&T, Option<&crate::Object>) -> crate::Iterator<Pad> + Send + Sync + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
) -> *mut ffi::GstIterator
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).iterintlinkdata as *const F);

    // Steal the iterator and return it
    let ret = func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
    );

    ret.into_glib_ptr()
}

unsafe extern "C" fn trampoline_link_function<
    T,
    F: Fn(
            &T,
            Option<&crate::Object>,
            &crate::Pad,
        ) -> Result<crate::PadLinkSuccess, crate::PadLinkError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    peer: *mut ffi::GstPad,
) -> ffi::GstPadLinkReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).linkdata as *const F);

    let res: crate::PadLinkReturn = func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        &from_glib_borrow(peer),
    )
    .into();
    res.into_glib()
}

unsafe extern "C" fn trampoline_query_function<
    T,
    F: Fn(&T, Option<&crate::Object>, &mut crate::QueryRef) -> bool + Send + Sync + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    query: *mut ffi::GstQuery,
) -> glib::ffi::gboolean
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).querydata as *const F);

    func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        crate::QueryRef::from_mut_ptr(query),
    )
    .into_glib()
}

unsafe extern "C" fn trampoline_unlink_function<
    T,
    F: Fn(&T, Option<&crate::Object>) + Send + Sync + 'static,
>(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
) where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).unlinkdata as *const F);

    func(
        Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
    )
}

unsafe extern "C" fn destroy_closure<F>(ptr: gpointer) {
    let _ = Box::<F>::from_raw(ptr as *mut _);
}

impl Pad {
    // rustdoc-stripper-ignore-next
    /// Creates a new [`Pad`] with the specified [`PadDirection`](crate::PadDirection).
    ///
    /// The [`Pad`] will be assigned the usual `gst::Object` generated unique name.
    ///
    /// Use [`Pad::builder()`] to get a [`PadBuilder`] and define options.
    #[doc(alias = "gst_pad_new")]
    pub fn new(direction: crate::PadDirection) -> Self {
        skip_assert_initialized!();
        Self::builder(direction).build()
    }

    // rustdoc-stripper-ignore-next
    /// Creates a [`PadBuilder`] with the specified [`PadDirection`](crate::PadDirection).
    #[doc(alias = "gst_pad_new")]
    pub fn builder(direction: crate::PadDirection) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::new(direction)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`Pad`] from the [`StaticPadTemplate`](crate::StaticPadTemplate).
    ///
    /// If the [`StaticPadTemplate`](crate::StaticPadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `Pad` will automatically be named after the `name_template`.
    ///
    /// Use [`Pad::builder_from_static_template()`] to get a [`PadBuilder`] and define options.
    ///
    /// Use [`generated_name()`](crate::PadBuilder::generated_name`) to keep the `gst::Object`
    /// automatically generated unique name.
    ///
    /// # Panics
    ///
    /// Panics if the `name_template` is a wildcard-name.
    #[doc(alias = "gst_pad_new_from_static_template")]
    pub fn from_static_template(templ: &StaticPadTemplate) -> Self {
        skip_assert_initialized!();
        Self::builder_from_static_template(templ).build()
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`PadBuilder`] from the [`StaticPadTemplate`](crate::StaticPadTemplate).
    ///
    /// If the [`StaticPadTemplate`](crate::StaticPadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `Pad` will automatically be named after the `name_template`.
    ///
    /// Use [`generated_name()`](crate::PadBuilder::generated_name`) to keep the `gst::Object`
    /// automatically generated unique name.
    #[doc(alias = "gst_pad_new_from_static_template")]
    pub fn builder_from_static_template(templ: &StaticPadTemplate) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_static_template(templ)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`Pad`] from the [`PadTemplate`](crate::PadTemplate).
    ///
    /// If the [`PadTemplate`](crate::PadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `Pad` will automatically be named after the `name_template`.
    ///
    /// Use [`Pad::builder_from_template()`] to get a [`PadBuilder`] and define options.
    ///
    /// # Panics
    ///
    /// Panics if the `name_template` is a wildcard-name.
    #[doc(alias = "gst_pad_new_from_template")]
    pub fn from_template(templ: &crate::PadTemplate) -> Self {
        skip_assert_initialized!();
        Self::builder_from_template(templ).build()
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`PadBuilder`] from the [`PadTemplate`](crate::PadTemplate).
    ///
    /// If the [`PadTemplate`](crate::PadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `Pad` will automatically be named after the `name_template`.
    ///
    /// Use [`generated_name()`](crate::PadBuilder::generated_name`) to keep the `gst::Object`
    /// automatically generated unique name.
    #[doc(alias = "gst_pad_new_from_template")]
    pub fn builder_from_template(templ: &crate::PadTemplate) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_template(templ)
    }

    #[doc(alias = "gst_pad_query_default")]
    pub fn query_default<O: IsA<Pad>>(
        pad: &O,
        parent: Option<&impl IsA<crate::Object>>,
        query: &mut QueryRef,
    ) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_pad_query_default(
                pad.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_pad_event_default")]
    pub fn event_default<O: IsA<Pad>>(
        pad: &O,
        parent: Option<&impl IsA<crate::Object>>,
        event: impl Into<Event>,
    ) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_pad_event_default(
                pad.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                event.into().into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_pad_iterate_internal_links_default")]
    pub fn iterate_internal_links_default<O: IsA<Pad>>(
        pad: &O,
        parent: Option<&impl IsA<crate::Object>>,
    ) -> crate::Iterator<Pad> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_pad_iterate_internal_links_default(
                pad.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

pub(crate) enum PadBuilderName {
    Undefined,
    KeepGenerated,
    UserDefined(String),
    CandidateForWildcardTemplate(String),
}

#[must_use = "The builder must be built to be used"]
pub struct PadBuilder<T> {
    pub(crate) pad: T,
    pub(crate) name: PadBuilderName,
}

impl<T: IsA<Pad> + IsA<glib::Object> + glib::object::IsClass> PadBuilder<T> {
    // rustdoc-stripper-ignore-next
    /// Creates a `PadBuilder` with the specified [`PadDirection`](crate::PadDirection).
    pub fn new(direction: crate::PadDirection) -> Self {
        assert_initialized_main_thread!();

        let pad = glib::Object::builder::<T>()
            .property("direction", direction)
            .build();

        // Ghost pads are a bit special
        if let Some(pad) = pad.dynamic_cast_ref::<crate::GhostPad>() {
            unsafe {
                let res = ffi::gst_ghost_pad_construct(pad.to_glib_none().0);
                // This can't really fail...
                debug_assert_ne!(res, glib::ffi::GFALSE, "Failed to construct ghost pad");
            }
        }

        PadBuilder {
            pad,
            name: PadBuilderName::Undefined,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a `PadBuilder` from the specified [`StaticPadTemplate`](crate::StaticPadTemplate).
    ///
    /// If the [`StaticPadTemplate`](crate::StaticPadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `Pad` will automatically be named after the `name_template`.
    ///
    /// Use [`generated_name()`](crate::PadBuilder::generated_name`) to keep the `gst::Object`
    /// automatically generated unique name.
    pub fn from_static_template(templ: &StaticPadTemplate) -> Self {
        skip_assert_initialized!();

        let templ = templ.get();
        Self::from_template(&templ)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a `PadBuilder` from the specified [`PadTemplate`](crate::PadTemplate).
    ///
    /// If the [`PadTemplate`](crate::PadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `Pad` will automatically be named after the `name_template`.
    ///
    /// Use [`generated_name()`](crate::PadBuilder::generated_name`) to keep the `gst::Object`
    /// automatically generated unique name.
    pub fn from_template(templ: &crate::PadTemplate) -> Self {
        assert_initialized_main_thread!();

        let mut type_ = T::static_type();

        // Since 1.14 templates can keep a pad GType with them, so we need to do some
        // additional checks here now
        if templ.has_property("gtype", Some(glib::Type::static_type())) {
            let gtype = templ.property::<glib::Type>("gtype");

            if gtype == glib::Type::UNIT {
                // Nothing to be done, we can create any kind of pad
            } else if gtype.is_a(type_) {
                // We were asked to create a parent type of the template type, e.g. a gst::Pad for
                // a template that wants a gst_base::AggregatorPad. Not a problem: update the type
                type_ = gtype;
            } else {
                // Otherwise the requested type must be a subclass of the template pad type
                assert!(type_.is_a(gtype));
            }
        }

        let mut properties = [
            ("direction", templ.direction().into()),
            ("template", templ.into()),
        ];

        let pad =
            unsafe { glib::Object::with_mut_values(type_, &mut properties).unsafe_cast::<T>() };

        // Ghost pads are a bit special
        if let Some(pad) = pad.dynamic_cast_ref::<crate::GhostPad>() {
            unsafe {
                let res = ffi::gst_ghost_pad_construct(pad.to_glib_none().0);
                // This can't really fail...
                debug_assert_ne!(res, glib::ffi::GFALSE, "Failed to construct ghost pad");
            }
        }

        PadBuilder {
            pad,
            name: PadBuilderName::Undefined,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Uses the `gst::Object` generated unique name.
    pub fn generated_name(mut self) -> Self {
        self.name = PadBuilderName::KeepGenerated;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Sets the name of the Pad.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = PadBuilderName::UserDefined(name.into());

        self
    }

    // rustdoc-stripper-ignore-next
    /// Optionally sets the name of the Pad.
    ///
    /// This method is convenient when the `name` is provided as an `Option`.
    /// If the `name` is `None`, this has no effect.
    pub fn maybe_name<N: Into<String>>(self, name: Option<N>) -> Self {
        if let Some(name) = name {
            self.name(name)
        } else {
            self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Optionally sets the name of the Pad.
    ///
    /// This method is convenient when the `name` is provided as an `Option`.
    /// If the `name` is `None`, this has no effect.
    pub fn name_if_some<N: Into<String>>(self, name: Option<N>) -> Self {
        if let Some(name) = name {
            self.name(name)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_activate_function")]
    pub fn activate_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
    {
        unsafe {
            self.pad.set_activate_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_activate_function")]
    pub fn activate_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
    {
        if let Some(func) = func {
            self.activate_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_activatemode_function")]
    pub fn activatemode_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::PadMode, bool) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.pad.set_activatemode_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_activatemode_function")]
    pub fn activatemode_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::PadMode, bool) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.activatemode_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_chain_function")]
    pub fn chain_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Buffer) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.pad.set_chain_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_chain_function")]
    pub fn chain_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Buffer) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.chain_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_chain_list_function")]
    pub fn chain_list_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::BufferList) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.pad.set_chain_list_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_chain_list_function")]
    pub fn chain_list_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::BufferList) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.chain_list_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_event_function")]
    pub fn event_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Event) -> bool + Send + Sync + 'static,
    {
        unsafe {
            self.pad.set_event_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_event_function")]
    pub fn event_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Event) -> bool + Send + Sync + 'static,
    {
        if let Some(func) = func {
            self.event_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_event_full_function")]
    pub fn event_full_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Event) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.pad.set_event_full_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_event_full_function")]
    pub fn event_full_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Event) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.event_full_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_getrange_function")]
    pub fn getrange_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &T,
                Option<&crate::Object>,
                u64,
                Option<&mut crate::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, crate::FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.pad.set_getrange_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_getrange_function")]
    pub fn getrange_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(
                &T,
                Option<&crate::Object>,
                u64,
                Option<&mut crate::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, crate::FlowError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.getrange_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_iterate_internal_links_function")]
    pub fn iterate_internal_links_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) -> crate::Iterator<Pad> + Send + Sync + 'static,
    {
        unsafe {
            self.pad.set_iterate_internal_links_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_iterate_internal_links_function")]
    pub fn iterate_internal_links_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) -> crate::Iterator<Pad> + Send + Sync + 'static,
    {
        if let Some(func) = func {
            self.iterate_internal_links_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_link_function")]
    pub fn link_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &T,
                Option<&crate::Object>,
                &Pad,
            ) -> Result<crate::PadLinkSuccess, crate::PadLinkError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.pad.set_link_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_link_function")]
    pub fn link_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(
                &T,
                Option<&crate::Object>,
                &Pad,
            ) -> Result<crate::PadLinkSuccess, crate::PadLinkError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.link_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_query_function")]
    pub fn query_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, &mut crate::QueryRef) -> bool + Send + Sync + 'static,
    {
        unsafe {
            self.pad.set_query_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_query_function")]
    pub fn query_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, &mut crate::QueryRef) -> bool + Send + Sync + 'static,
    {
        if let Some(func) = func {
            self.query_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_unlink_function")]
    pub fn unlink_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) + Send + Sync + 'static,
    {
        unsafe {
            self.pad.set_unlink_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_unlink_function")]
    pub fn unlink_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) + Send + Sync + 'static,
    {
        if let Some(func) = func {
            self.unlink_function(func)
        } else {
            self
        }
    }

    pub fn flags(self, flags: PadFlags) -> Self {
        self.pad.set_pad_flags(flags);

        self
    }

    pub fn flags_if_some(self, flags: Option<PadFlags>) -> Self {
        if let Some(flags) = flags {
            self.flags(flags)
        } else {
            self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Builds the [`Pad`].
    ///
    /// # Panics
    ///
    /// Panics if the [`Pad`] was built from a [`PadTemplate`](crate::PadTemplate)
    /// with a wildcard-name `name_template` (i.e. containing `%u`, `%s` or `%d`)
    /// and no specific `name` was provided using [`PadBuilder::name`]
    /// or [`PadBuilder::maybe_name`], or for [`GhostPad`s](crate::GhostPad),
    /// by defining a `target`.
    ///
    /// Use [`generated_name()`](crate::PadBuilder::generated_name`) to keep the `gst::Object`
    /// automatically generated unique name.
    #[must_use = "Building the pad without using it has no effect"]
    #[track_caller]
    pub fn build(self) -> T {
        let Self { pad, name } = self;

        let templ = pad.pad_template();

        use PadBuilderName::*;
        match (name, templ) {
            (KeepGenerated, _) => (),
            (Undefined, None) => (),
            (Undefined, Some(templ)) => {
                if templ.name().find('%').is_some() {
                    panic!(concat!(
                        "Attempt to build a Pad from a wildcard-name template",
                        " or with a target Pad with an incompatible name.",
                        " Make sure to define a specific name using PadBuilder",
                        " or opt-in to keep the automatically generated name.",
                    ));
                } else {
                    pad.set_property("name", templ.name());
                }
            }
            (UserDefined(name), _) | (CandidateForWildcardTemplate(name), None) => {
                pad.set_property("name", name);
            }
            (CandidateForWildcardTemplate(name), Some(templ)) => {
                if templ.name().find('%').is_none() {
                    // Not a widlcard template
                    pad.set_property("name", templ.name());
                } else {
                    let mut can_assign_name = true;

                    if templ.presence() == crate::PadPresence::Request {
                        // Check if the name is compatible with the name template.
                        use crate::CAT_RUST;

                        let mut name_parts = name.split('_');
                        for templ_part in templ.name_template().split('_') {
                            let Some(name_part) = name_parts.next() else {
                                crate::debug!(
                                CAT_RUST,
                                "Not using Pad name '{name}': not enough parts compared to template '{}'",
                                templ.name_template(),
                            );
                                can_assign_name = false;
                                break;
                            };

                            if let Some(conv_spec_start) = templ_part.find('%') {
                                if conv_spec_start > 0
                                    && !name_part.starts_with(&templ_part[..conv_spec_start])
                                {
                                    crate::debug!(
                                    CAT_RUST,
                                    "Not using Pad name '{name}': mismatch template '{}' prefix",
                                    templ.name_template(),
                                );
                                    can_assign_name = false;
                                    break;
                                }

                                let conv_spec_pos = conv_spec_start + 1;
                                match templ_part.get(conv_spec_pos..=conv_spec_pos) {
                                    Some("s") => {
                                        // *There can be only one* %s
                                        break;
                                    }
                                    Some("u") => {
                                        if name_part
                                            .get(conv_spec_start..)
                                            .map_or(true, |s| s.parse::<u32>().is_err())
                                        {
                                            crate::debug!(
                                            CAT_RUST,
                                            "Not using Pad name '{name}': can't parse '%u' from '{name_part}' (template '{}')",
                                            templ.name_template(),
                                        );

                                            can_assign_name = false;
                                            break;
                                        }
                                    }
                                    Some("d") => {
                                        if name_part
                                            .get(conv_spec_start..)
                                            .map_or(true, |s| s.parse::<i32>().is_err())
                                        {
                                            crate::debug!(
                                            CAT_RUST,
                                            "Not using target Pad name '{name}': can't parse '%i' from '{name_part}' (template '{}')",
                                            templ.name_template(),
                                        );

                                            can_assign_name = false;
                                            break;
                                        }
                                    }
                                    other => {
                                        unreachable!("Unexpected conversion specifier {other:?}")
                                    }
                                }
                            } else if name_part != templ_part {
                                can_assign_name = false;
                            }
                        }
                    }

                    if can_assign_name {
                        pad.set_property("name", name);
                    } else {
                        panic!(concat!(
                            "Attempt to build a Pad from a wildcard-name template",
                            " with a target Pad with an incompatible name.",
                            " Make sure to define a specific name using PadBuilder",
                            " or opt-in to keep the automatically generated name.",
                        ));
                    }
                }
            }
        }

        pad
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{atomic::AtomicUsize, mpsc::channel, Arc, Mutex};

    use super::*;

    #[test]
    fn test_event_chain_functions() {
        crate::init().unwrap();

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        let buffers = Arc::new(Mutex::new(Vec::new()));
        let buffers_clone = buffers.clone();
        let pad = crate::Pad::builder(crate::PadDirection::Sink)
            .name("sink")
            .event_function(move |_, _, event| {
                let mut events = events_clone.lock().unwrap();
                events.push(event);

                true
            })
            .chain_function(move |_, _, buffer| {
                let mut buffers = buffers_clone.lock().unwrap();
                buffers.push(buffer);

                Ok(FlowSuccess::Ok)
            })
            .build();

        pad.set_active(true).unwrap();

        assert!(pad.send_event(crate::event::StreamStart::new("test")));
        let segment = crate::FormattedSegment::<crate::ClockTime>::new();
        assert!(pad.send_event(crate::event::Segment::new(segment.as_ref())));

        assert_eq!(pad.chain(crate::Buffer::new()), Ok(FlowSuccess::Ok));

        let events = events.lock().unwrap();
        let buffers = buffers.lock().unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(buffers.len(), 1);

        match events[0].view() {
            crate::EventView::StreamStart(..) => (),
            _ => unreachable!(),
        }

        match events[1].view() {
            crate::EventView::Segment(..) => (),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_getrange_function() {
        crate::init().unwrap();

        let pad = crate::Pad::builder(crate::PadDirection::Src)
            .name("src")
            .activate_function(|pad, _parent| {
                pad.activate_mode(crate::PadMode::Pull, true)
                    .map_err(|err| err.into())
            })
            .getrange_function(|_pad, _parent, offset, _buffer, size| {
                assert_eq!(offset, 0);
                assert_eq!(size, 5);
                let buffer = crate::Buffer::from_slice(b"abcde");
                Ok(PadGetRangeSuccess::NewBuffer(buffer))
            })
            .build();
        pad.set_active(true).unwrap();

        let buffer = pad.range(0, 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        let mut buffer = crate::Buffer::with_size(5).unwrap();
        pad.range_fill(0, buffer.get_mut().unwrap(), 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        pad.set_active(false).unwrap();
        drop(pad);

        let pad = crate::Pad::builder(crate::PadDirection::Src)
            .name("src")
            .activate_function(|pad, _parent| {
                pad.activate_mode(crate::PadMode::Pull, true)
                    .map_err(|err| err.into())
            })
            .getrange_function(|_pad, _parent, offset, buffer, size| {
                assert_eq!(offset, 0);
                assert_eq!(size, 5);
                if let Some(buffer) = buffer {
                    buffer.copy_from_slice(0, b"fghij").unwrap();
                    Ok(PadGetRangeSuccess::FilledBuffer)
                } else {
                    let buffer = crate::Buffer::from_slice(b"abcde");
                    Ok(PadGetRangeSuccess::NewBuffer(buffer))
                }
            })
            .build();
        pad.set_active(true).unwrap();

        let buffer = pad.range(0, 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        let mut buffer = crate::Buffer::with_size(5).unwrap();
        pad.range_fill(0, buffer.get_mut().unwrap(), 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"fghij");
    }

    #[test]
    fn test_task() {
        crate::init().unwrap();

        let pad = crate::Pad::builder(crate::PadDirection::Sink)
            .name("sink")
            .build();
        let (sender, receiver) = channel();

        let mut i = 0;
        let pad_clone = pad.clone();
        pad.start_task(move || {
            i += 1;
            if i == 3 {
                sender.send(i).unwrap();
                pad_clone.pause_task().unwrap();
            }
        })
        .unwrap();

        assert_eq!(receiver.recv().unwrap(), 3);
    }

    #[test]
    fn test_remove_probe_from_probe() {
        crate::init().unwrap();

        let src_pad = crate::Pad::builder(crate::PadDirection::Src)
            .name("src")
            .build();
        let sink_pad = crate::Pad::builder(crate::PadDirection::Sink)
            .name("sink")
            .chain_function(|_pad, _parent, _buffer| Ok(crate::FlowSuccess::Ok))
            .build();

        src_pad.link(&sink_pad).unwrap();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        src_pad.add_probe(crate::PadProbeType::BUFFER, move |pad, info| {
            if let Some(PadProbeData::Buffer(_)) = info.data {
                counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                pad.remove_probe(info.id.take().expect("no pad probe id"));
            } else {
                unreachable!();
            }
            crate::PadProbeReturn::Handled
        });

        sink_pad.set_active(true).unwrap();
        src_pad.set_active(true).unwrap();

        assert!(src_pad.push_event(crate::event::StreamStart::new("test")));
        let segment = crate::FormattedSegment::<crate::ClockTime>::new();
        assert!(src_pad.push_event(crate::event::Segment::new(segment.as_ref())));

        assert_eq!(src_pad.push(crate::Buffer::new()), Ok(FlowSuccess::Ok));
        assert_eq!(src_pad.push(crate::Buffer::new()), Ok(FlowSuccess::Ok));

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    fn do_probe_with_return(probe_return: crate::PadProbeReturn) {
        skip_assert_initialized!();
        crate::init().unwrap();

        let (major, minor, micro, _) = crate::version();
        let pad = crate::Pad::builder(crate::PadDirection::Src)
            .name("src")
            .build();
        let events = Arc::new(Mutex::new(Vec::new()));
        let buffers = Arc::new(Mutex::new(Vec::new()));

        let flow_override = if (major, minor, micro) >= (1, 16, 1) {
            Err(FlowError::Eos)
        } else {
            // Broken on 1.16.0
            // https://gitlab.freedesktop.org/gstreamer/gstreamer/merge_requests/151
            Ok(FlowSuccess::Ok)
        };

        {
            let events = events.clone();
            pad.add_probe(crate::PadProbeType::EVENT_DOWNSTREAM, move |_, info| {
                if let Some(PadProbeData::Event(event)) = &info.data {
                    let mut events = events.lock().unwrap();
                    events.push(event.clone());
                } else {
                    unreachable!();
                }
                crate::PadProbeReturn::Ok
            });
        }

        {
            let events = events.clone();
            pad.add_probe(crate::PadProbeType::EVENT_UPSTREAM, move |_, info| {
                if let Some(event) = info.take_event() {
                    let mut events = events.lock().unwrap();
                    events.push(event);
                } else {
                    unreachable!();
                }
                probe_return
            });
        }

        {
            let buffers = buffers.clone();
            pad.add_probe(crate::PadProbeType::BUFFER, move |_, info| {
                if let Some(buffer) = info.take_buffer() {
                    let mut buffers = buffers.lock().unwrap();
                    info.flow_res = if buffers.is_empty() {
                        Ok(FlowSuccess::Ok)
                    } else {
                        flow_override
                    };
                    buffers.push(buffer);
                } else {
                    unreachable!();
                }
                probe_return
            });
        }

        pad.set_active(true).unwrap();

        assert!(
            pad.send_event(crate::event::Latency::new(crate::ClockTime::from_nseconds(
                10
            )))
        );
        assert!(pad.push_event(crate::event::StreamStart::new("test")));
        let segment = crate::FormattedSegment::<crate::ClockTime>::new();
        assert!(pad.push_event(crate::event::Segment::new(segment.as_ref())));

        assert_eq!(pad.push(crate::Buffer::new()), Ok(FlowSuccess::Ok));
        assert_eq!(
            pad.push(crate::Buffer::new()),
            // On Drop, we will get an Ok, not whatever value we returned
            if probe_return == crate::PadProbeReturn::Drop {
                Ok(FlowSuccess::Ok)
            } else {
                flow_override
            }
        );

        let events = events.lock().unwrap();
        let buffers = buffers.lock().unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(buffers.len(), 2);

        assert_eq!(events[0].type_(), crate::EventType::Latency);
        assert_eq!(events[1].type_(), crate::EventType::StreamStart);
        assert_eq!(events[2].type_(), crate::EventType::Segment);

        assert!(
            buffers.iter().all(|b| b.is_writable()),
            "A buffer ref leaked!"
        );

        drop(pad); // Need to drop the pad first to unref sticky events
        assert!(
            events.iter().all(|e| e.is_writable()),
            "An event ref leaked!"
        );
    }

    #[test]
    fn test_probe() {
        crate::init().unwrap();
        do_probe_with_return(crate::PadProbeReturn::Handled);
    }

    #[test]
    fn test_probe_drop() {
        crate::init().unwrap();
        do_probe_with_return(crate::PadProbeReturn::Drop);
    }

    #[test]
    fn test_sticky_events() {
        crate::init().unwrap();

        let pad = crate::Pad::builder(crate::PadDirection::Sink)
            .name("sink")
            .build();
        pad.set_active(true).unwrap();

        // Send some sticky events
        assert!(pad.send_event(crate::event::StreamStart::new("test")));

        let caps = crate::Caps::builder("some/x-caps").build();
        assert!(pad.send_event(crate::event::Caps::new(&caps)));

        let segment = crate::FormattedSegment::<crate::ClockTime>::new();
        assert!(pad.send_event(crate::event::Segment::new(segment.as_ref())));

        let stream_start = pad.sticky_event::<crate::event::StreamStart>(0).unwrap();
        assert_eq!(stream_start.stream_id(), "test");

        let caps2 = pad.sticky_event::<crate::event::Caps>(0).unwrap();
        assert_eq!(&*caps, caps2.caps());

        let segment = pad.sticky_event::<crate::event::Segment>(0).unwrap();
        assert_eq!(segment.segment().format(), crate::Format::Time);
    }

    #[test]
    fn test_sticky_events_foreach() {
        crate::init().unwrap();

        let pad = crate::Pad::builder(crate::PadDirection::Sink)
            .name("sink")
            .build();
        pad.set_active(true).unwrap();

        // Send some sticky events
        assert!(pad.send_event(crate::event::StreamStart::new("test")));

        let caps = crate::Caps::builder("some/x-caps").build();
        assert!(pad.send_event(crate::event::Caps::new(&caps)));

        let segment = crate::FormattedSegment::<crate::ClockTime>::new();
        assert!(pad.send_event(crate::event::Segment::new(segment.as_ref())));

        let mut sticky_events = Vec::new();
        pad.sticky_events_foreach(|event| {
            sticky_events.push(event.clone());
            ControlFlow::Continue(EventForeachAction::Keep)
        });
        assert_eq!(sticky_events.len(), 3);

        // Test early exit from foreach loop
        let mut sticky_events2 = Vec::new();
        pad.sticky_events_foreach(|event| {
            sticky_events2.push(event.clone());
            if event.type_() == crate::EventType::Caps {
                ControlFlow::Break(EventForeachAction::Keep)
            } else {
                ControlFlow::Continue(EventForeachAction::Keep)
            }
        });
        assert_eq!(sticky_events2.len(), 2);

        let mut sticky_events3 = Vec::new();
        pad.sticky_events_foreach(|event| {
            sticky_events3.push(event.clone());
            ControlFlow::Continue(EventForeachAction::Keep)
        });
        assert_eq!(sticky_events3.len(), 3);

        for (e1, e2) in sticky_events.iter().zip(sticky_events3.iter()) {
            assert_eq!(e1.as_ref() as *const _, e2.as_ref() as *const _);
        }

        // Replace segment event
        pad.sticky_events_foreach(|event| {
            let action = if event.type_() == crate::EventType::Segment {
                let byte_segment = crate::FormattedSegment::<crate::format::Bytes>::new();
                EventForeachAction::Replace(crate::event::Segment::new(&byte_segment))
            } else {
                EventForeachAction::Keep
            };
            ControlFlow::Continue(action)
        });

        // Check that segment event is different now
        let mut sticky_events4 = Vec::new();
        pad.sticky_events_foreach(|event| {
            sticky_events4.push(event.clone());
            ControlFlow::Continue(EventForeachAction::Keep)
        });
        assert_eq!(sticky_events4.len(), 3);
        assert_eq!(
            sticky_events[0].as_ref() as *const _,
            sticky_events4[0].as_ref() as *const _
        );
        assert_eq!(
            sticky_events[1].as_ref() as *const _,
            sticky_events4[1].as_ref() as *const _
        );
        assert_ne!(
            sticky_events[2].as_ref() as *const _,
            sticky_events4[2].as_ref() as *const _
        );

        // Drop caps event
        pad.sticky_events_foreach(|event| {
            let action = if event.type_() == crate::EventType::Caps {
                EventForeachAction::Remove
            } else {
                EventForeachAction::Keep
            };
            ControlFlow::Continue(action)
        });

        // Check that caps event actually got removed
        let mut sticky_events5 = Vec::new();
        pad.sticky_events_foreach(|event| {
            sticky_events5.push(event.clone());
            ControlFlow::Continue(EventForeachAction::Keep)
        });
        assert_eq!(sticky_events5.len(), 2);
        assert_eq!(
            sticky_events4[0].as_ref() as *const _,
            sticky_events5[0].as_ref() as *const _
        );
        assert_eq!(
            sticky_events4[2].as_ref() as *const _,
            sticky_events5[1].as_ref() as *const _
        );
    }

    #[test]
    fn naming() {
        crate::init().unwrap();

        let pad = crate::Pad::builder(crate::PadDirection::Sink).build();
        assert!(pad.name().starts_with("pad"));

        let pad = crate::Pad::builder(crate::PadDirection::Src).build();
        assert!(pad.name().starts_with("pad"));

        let pad = crate::Pad::builder(crate::PadDirection::Unknown).build();
        assert!(pad.name().starts_with("pad"));

        let pad = crate::Pad::builder(crate::PadDirection::Unknown)
            .generated_name()
            .build();
        assert!(pad.name().starts_with("pad"));

        let pad = crate::Pad::builder(crate::PadDirection::Unknown)
            .maybe_name(None::<&str>)
            .build();
        assert!(pad.name().starts_with("pad"));

        let pad = crate::Pad::builder(crate::PadDirection::Unknown)
            .name_if_some(None::<&str>)
            .build();
        assert!(pad.name().starts_with("pad"));

        let pad = crate::Pad::builder(crate::PadDirection::Sink)
            .name("sink_0")
            .build();
        assert_eq!(pad.name(), "sink_0");

        let pad = crate::Pad::builder(crate::PadDirection::Src)
            .name("src_0")
            .build();
        assert_eq!(pad.name(), "src_0");

        let pad = crate::Pad::builder(crate::PadDirection::Unknown)
            .name("test")
            .build();
        assert_eq!(pad.name(), "test");

        let pad = crate::Pad::builder(crate::PadDirection::Unknown)
            .maybe_name(Some("test"))
            .build();
        assert_eq!(pad.name(), "test");

        let pad = crate::Pad::builder(crate::PadDirection::Unknown)
            .name_if_some(Some("test"))
            .build();
        assert_eq!(pad.name(), "test");

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "sink",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let pad = Pad::from_template(&templ);
        assert!(pad.name().starts_with("sink"));

        let pad = Pad::builder_from_template(&templ)
            .name("audio_sink")
            .build();
        assert!(pad.name().starts_with("audio_sink"));

        let pad = Pad::builder_from_template(&templ).generated_name().build();
        assert!(pad.name().starts_with("pad"));

        let templ = crate::PadTemplate::new(
            "audio_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        let pad = Pad::builder_from_template(&templ).name("audio_0").build();
        assert!(pad.name().starts_with("audio_0"));

        let pad = Pad::builder_from_template(&templ).generated_name().build();
        assert!(pad.name().starts_with("pad"));
    }

    #[test]
    #[should_panic]
    fn missing_name() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "audio_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        // Panic: attempt to build from a wildcard-named template
        //        without providing a name.
        let _pad = Pad::from_template(&templ);
    }
}
