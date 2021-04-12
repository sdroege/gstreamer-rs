// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Buffer;
use crate::BufferList;
use crate::Event;
use crate::FlowError;
use crate::FlowReturn;
use crate::FlowSuccess;
use crate::Format;
use crate::FormattedValue;
use crate::GenericFormattedValue;
use crate::LoggableError;
use crate::Pad;
use crate::PadFlags;
use crate::PadLinkCheck;
use crate::PadLinkError;
use crate::PadLinkReturn;
use crate::PadLinkSuccess;
use crate::PadProbeReturn;
use crate::PadProbeType;
use crate::Query;
use crate::QueryRef;
use crate::SpecificFormattedValue;
use crate::StaticPadTemplate;

use std::cell::RefCell;
use std::mem;
use std::num::NonZeroU64;
use std::ptr;

use glib::ffi::gpointer;
use glib::object::{Cast, IsA};
use glib::translate::{
    from_glib, from_glib_borrow, from_glib_full, FromGlib, FromGlibPtrBorrow, ToGlib, ToGlibPtr,
};
use glib::StaticType;

#[derive(Debug, PartialEq, Eq)]
pub struct PadProbeId(NonZeroU64);

impl ToGlib for PadProbeId {
    type GlibType = libc::c_ulong;

    fn to_glib(&self) -> libc::c_ulong {
        self.0.get() as libc::c_ulong
    }
}

impl FromGlib<libc::c_ulong> for PadProbeId {
    unsafe fn from_glib(val: libc::c_ulong) -> PadProbeId {
        skip_assert_initialized!();
        assert_ne!(val, 0);
        PadProbeId(NonZeroU64::new_unchecked(val as u64))
    }
}

#[derive(Debug)]
pub struct PadProbeInfo<'a> {
    pub mask: PadProbeType,
    pub id: Option<PadProbeId>,
    pub offset: u64,
    pub size: u32,
    pub data: Option<PadProbeData<'a>>,
    pub flow_res: Result<FlowSuccess, FlowError>,
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

pub trait PadExtManual: 'static {
    fn add_probe<F>(&self, mask: PadProbeType, func: F) -> Option<PadProbeId>
    where
        F: Fn(&Self, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static;
    fn remove_probe(&self, id: PadProbeId);

    fn chain(&self, buffer: Buffer) -> Result<FlowSuccess, FlowError>;
    fn push(&self, buffer: Buffer) -> Result<FlowSuccess, FlowError>;

    fn chain_list(&self, list: BufferList) -> Result<FlowSuccess, FlowError>;
    fn push_list(&self, list: BufferList) -> Result<FlowSuccess, FlowError>;

    fn pull_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowError>;
    fn pull_range_fill(
        &self,
        offset: u64,
        buffer: &mut crate::BufferRef,
        size: u32,
    ) -> Result<(), FlowError>;
    fn get_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowError>;
    fn get_range_fill(
        &self,
        offset: u64,
        buffer: &mut crate::BufferRef,
        size: u32,
    ) -> Result<(), FlowError>;

    fn peer_query(&self, query: &mut QueryRef) -> bool;
    fn query(&self, query: &mut QueryRef) -> bool;
    fn query_default<P: IsA<crate::Object>>(
        &self,
        parent: Option<&P>,
        query: &mut QueryRef,
    ) -> bool;
    fn proxy_query_caps(&self, query: &mut QueryRef) -> bool;
    fn proxy_query_accept_caps(&self, query: &mut QueryRef) -> bool;

    fn event_default<P: IsA<crate::Object>>(&self, parent: Option<&P>, event: Event) -> bool;
    fn push_event(&self, event: Event) -> bool;
    fn send_event(&self, event: Event) -> bool;

    fn last_flow_result(&self) -> Result<FlowSuccess, FlowError>;

    fn iterate_internal_links(&self) -> crate::Iterator<Pad>;
    fn iterate_internal_links_default<P: IsA<crate::Object>>(
        &self,
        parent: Option<&P>,
    ) -> crate::Iterator<Pad>;

    fn link<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<PadLinkSuccess, PadLinkError>;
    fn link_full<P: IsA<Pad>>(
        &self,
        sinkpad: &P,
        flags: PadLinkCheck,
    ) -> Result<PadLinkSuccess, PadLinkError>;

    fn stream_lock(&self) -> StreamLock;

    unsafe fn set_activate_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>) -> Result<(), LoggableError> + Send + Sync + 'static;

    unsafe fn set_activatemode_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::PadMode, bool) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_chain_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::Buffer) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_chain_list_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::BufferList) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_event_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::Event) -> bool + Send + Sync + 'static;

    unsafe fn set_event_full_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, crate::Event) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static;

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
            + 'static;

    unsafe fn set_iterate_internal_links_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>) -> crate::Iterator<Pad> + Send + Sync + 'static;

    unsafe fn set_link_function<F>(&self, func: F)
    where
        F: Fn(
                &Self,
                Option<&crate::Object>,
                &Pad,
            ) -> Result<crate::PadLinkSuccess, crate::PadLinkError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_query_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>, &mut crate::QueryRef) -> bool + Send + Sync + 'static;

    unsafe fn set_unlink_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&crate::Object>) + Send + Sync + 'static;

    fn start_task<F: FnMut() + Send + 'static>(&self, func: F) -> Result<(), glib::BoolError>;

    fn peer_query_convert<V: Into<GenericFormattedValue>, U: SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U>;
    fn peer_query_convert_generic<V: Into<GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<GenericFormattedValue>;

    fn peer_query_duration<T: SpecificFormattedValue>(&self) -> Option<T>;
    fn peer_query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    fn peer_query_position<T: SpecificFormattedValue>(&self) -> Option<T>;
    fn peer_query_position_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    fn query_convert<V: Into<GenericFormattedValue>, U: SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U>;
    fn query_convert_generic<V: Into<GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<GenericFormattedValue>;

    fn query_duration<T: SpecificFormattedValue>(&self) -> Option<T>;
    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    fn query_position<T: SpecificFormattedValue>(&self) -> Option<T>;
    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue>;

    fn mode(&self) -> crate::PadMode;

    fn sticky_events_foreach<F: FnMut(Event) -> Result<Option<Event>, Option<Event>>>(
        &self,
        func: F,
    );

    fn store_sticky_event(&self, event: &Event) -> Result<FlowSuccess, FlowError>;

    fn set_pad_flags(&self, flags: PadFlags);

    fn unset_pad_flags(&self, flags: PadFlags);

    fn pad_flags(&self) -> PadFlags;
}

impl<O: IsA<Pad>> PadExtManual for O {
    fn add_probe<F>(&self, mask: PadProbeType, func: F) -> Option<PadProbeId>
    where
        F: Fn(&Self, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<F> = Box::new(func);
            let id = ffi::gst_pad_add_probe(
                self.as_ref().to_glib_none().0,
                mask.to_glib(),
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

    fn remove_probe(&self, id: PadProbeId) {
        unsafe {
            ffi::gst_pad_remove_probe(self.as_ref().to_glib_none().0, id.to_glib());
        }
    }

    fn chain(&self, buffer: Buffer) -> Result<FlowSuccess, FlowError> {
        unsafe {
            FlowReturn::from_glib(ffi::gst_pad_chain(
                self.as_ref().to_glib_none().0,
                buffer.into_ptr(),
            ))
            .into_result()
        }
    }

    fn push(&self, buffer: Buffer) -> Result<FlowSuccess, FlowError> {
        unsafe {
            FlowReturn::from_glib(ffi::gst_pad_push(
                self.as_ref().to_glib_none().0,
                buffer.into_ptr(),
            ))
            .into_result()
        }
    }

    fn chain_list(&self, list: BufferList) -> Result<FlowSuccess, FlowError> {
        unsafe {
            FlowReturn::from_glib(ffi::gst_pad_chain_list(
                self.as_ref().to_glib_none().0,
                list.into_ptr(),
            ))
            .into_result()
        }
    }

    fn push_list(&self, list: BufferList) -> Result<FlowSuccess, FlowError> {
        unsafe {
            FlowReturn::from_glib(ffi::gst_pad_push_list(
                self.as_ref().to_glib_none().0,
                list.into_ptr(),
            ))
            .into_result()
        }
    }

    fn get_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowError> {
        unsafe {
            let mut buffer = ptr::null_mut();
            let ret: FlowReturn = from_glib(ffi::gst_pad_get_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ));
            ret.into_result_value(|| from_glib_full(buffer))
        }
    }

    fn get_range_fill(
        &self,
        offset: u64,
        buffer: &mut crate::BufferRef,
        size: u32,
    ) -> Result<(), FlowError> {
        assert!(buffer.size() >= size as usize);

        unsafe {
            let mut buffer_ref = buffer.as_mut_ptr();
            let ret: FlowReturn = from_glib(ffi::gst_pad_get_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer_ref,
            ));
            match ret.into_result_value(|| ()) {
                Ok(_) => {
                    if buffer.as_mut_ptr() != buffer_ref {
                        ffi::gst_mini_object_unref(buffer_ref as *mut _);
                        Err(crate::FlowError::Error)
                    } else {
                        Ok(())
                    }
                }
                Err(err) => Err(err),
            }
        }
    }

    fn pull_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowError> {
        unsafe {
            let mut buffer = ptr::null_mut();
            let ret: FlowReturn = from_glib(ffi::gst_pad_pull_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ));
            ret.into_result_value(|| from_glib_full(buffer))
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
            let ret: FlowReturn = from_glib(ffi::gst_pad_pull_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer_ref,
            ));
            match ret.into_result_value(|| ()) {
                Ok(_) => {
                    if buffer.as_mut_ptr() != buffer_ref {
                        ffi::gst_mini_object_unref(buffer_ref as *mut _);
                        Err(crate::FlowError::Error)
                    } else {
                        Ok(())
                    }
                }
                Err(err) => Err(err),
            }
        }
    }

    fn query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn peer_query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_peer_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn query_default<P: IsA<crate::Object>>(
        &self,
        parent: Option<&P>,
        query: &mut QueryRef,
    ) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_pad_query_default(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn proxy_query_accept_caps(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_proxy_query_accept_caps(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn proxy_query_caps(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_proxy_query_accept_caps(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn event_default<P: IsA<crate::Object>>(&self, parent: Option<&P>, event: Event) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_pad_event_default(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn push_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_push_event(
                self.as_ref().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn send_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_send_event(
                self.as_ref().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn last_flow_result(&self) -> Result<FlowSuccess, FlowError> {
        let ret: FlowReturn = unsafe {
            from_glib(ffi::gst_pad_get_last_flow_return(
                self.as_ref().to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    fn iterate_internal_links(&self) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_pad_iterate_internal_links(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_internal_links_default<P: IsA<crate::Object>>(
        &self,
        parent: Option<&P>,
    ) -> crate::Iterator<Pad> {
        unsafe {
            from_glib_full(ffi::gst_pad_iterate_internal_links_default(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    fn link<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<PadLinkSuccess, PadLinkError> {
        let ret: PadLinkReturn = unsafe {
            from_glib(ffi::gst_pad_link(
                self.as_ref().to_glib_none().0,
                sinkpad.as_ref().to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    fn link_full<P: IsA<Pad>>(
        &self,
        sinkpad: &P,
        flags: PadLinkCheck,
    ) -> Result<PadLinkSuccess, PadLinkError> {
        let ret: PadLinkReturn = unsafe {
            from_glib(ffi::gst_pad_link_full(
                self.as_ref().to_glib_none().0,
                sinkpad.as_ref().to_glib_none().0,
                flags.to_glib(),
            ))
        };
        ret.into_result()
    }

    fn stream_lock(&self) -> StreamLock {
        unsafe {
            let ptr: &mut ffi::GstPad = &mut *(self.as_ptr() as *mut _);
            glib::ffi::g_rec_mutex_lock(&mut ptr.stream_rec_lock);
            StreamLock(self.upcast_ref())
        }
    }

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

    fn start_task<F: FnMut() + Send + 'static>(&self, func: F) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_start_task(
                    self.as_ref().to_glib_none().0,
                    Some(trampoline_pad_task::<F>),
                    into_raw_pad_task(func),
                    Some(destroy_closure_pad_task::<F>),
                ),
                "Failed to start pad task",
            )
        }
    }

    fn peer_query_convert<V: Into<GenericFormattedValue>, U: SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().to_glib(),
                src_val.to_raw_value(),
                U::get_default_format().to_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(U::from_raw(U::get_default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    fn peer_query_convert_generic<V: Into<GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<GenericFormattedValue> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().to_glib(),
                src_val.to_raw_value(),
                dest_format.to_glib(),
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

    fn peer_query_duration<T: SpecificFormattedValue>(&self) -> Option<T> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_duration(
                self.as_ref().to_glib_none().0,
                T::get_default_format().to_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(T::from_raw(T::get_default_format(), duration.assume_init()))
            } else {
                None
            }
        }
    }

    fn peer_query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_duration(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, duration.assume_init()))
            } else {
                None
            }
        }
    }

    fn peer_query_position<T: SpecificFormattedValue>(&self) -> Option<T> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_position(
                self.as_ref().to_glib_none().0,
                T::get_default_format().to_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(T::from_raw(T::get_default_format(), cur.assume_init()))
            } else {
                None
            }
        }
    }

    fn peer_query_position_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_peer_query_position(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, cur.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_convert<V: Into<GenericFormattedValue>, U: SpecificFormattedValue>(
        &self,
        src_val: V,
    ) -> Option<U> {
        let src_val = src_val.into();

        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().to_glib(),
                src_val.to_raw_value(),
                U::get_default_format().to_glib(),
                dest_val.as_mut_ptr(),
            ));
            if ret {
                Some(U::from_raw(U::get_default_format(), dest_val.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_convert_generic<V: Into<GenericFormattedValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<GenericFormattedValue> {
        let src_val = src_val.into();

        unsafe {
            let mut dest_val = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.format().to_glib(),
                src_val.value(),
                dest_format.to_glib(),
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

    fn query_duration<T: SpecificFormattedValue>(&self) -> Option<T> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_duration(
                self.as_ref().to_glib_none().0,
                T::get_default_format().to_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(T::from_raw(T::get_default_format(), duration.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_duration_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_duration(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                duration.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, duration.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_position<T: SpecificFormattedValue>(&self) -> Option<T> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_position(
                self.as_ref().to_glib_none().0,
                T::get_default_format().to_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(T::from_raw(T::get_default_format(), cur.assume_init()))
            } else {
                None
            }
        }
    }

    fn query_position_generic(&self, format: Format) -> Option<GenericFormattedValue> {
        unsafe {
            let mut cur = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_pad_query_position(
                self.as_ref().to_glib_none().0,
                format.to_glib(),
                cur.as_mut_ptr(),
            ));
            if ret {
                Some(GenericFormattedValue::new(format, cur.assume_init()))
            } else {
                None
            }
        }
    }

    fn mode(&self) -> crate::PadMode {
        unsafe {
            let ptr: &ffi::GstPad = &*(self.as_ptr() as *const _);
            from_glib(ptr.mode)
        }
    }

    fn sticky_events_foreach<F: FnMut(Event) -> Result<Option<Event>, Option<Event>>>(
        &self,
        func: F,
    ) {
        unsafe extern "C" fn trampoline(
            _pad: *mut ffi::GstPad,
            event: *mut *mut ffi::GstEvent,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func =
                user_data as *mut &mut (dyn FnMut(Event) -> Result<Option<Event>, Option<Event>>);
            let res = (*func)(from_glib_full(*event));

            match res {
                Ok(Some(ev)) => {
                    *event = ev.into_ptr();
                    glib::ffi::GTRUE
                }
                Err(Some(ev)) => {
                    *event = ev.into_ptr();
                    glib::ffi::GFALSE
                }
                Ok(None) => {
                    *event = ptr::null_mut();
                    glib::ffi::GTRUE
                }
                Err(None) => {
                    *event = ptr::null_mut();
                    glib::ffi::GFALSE
                }
            }
        }

        unsafe {
            let mut func = func;
            let func_obj: &mut (dyn FnMut(Event) -> Result<Option<Event>, Option<Event>>) =
                &mut func;
            let func_ptr = &func_obj
                as *const &mut (dyn FnMut(Event) -> Result<Option<Event>, Option<Event>>)
                as glib::ffi::gpointer;

            ffi::gst_pad_sticky_events_foreach(
                self.as_ref().to_glib_none().0,
                Some(trampoline),
                func_ptr,
            );
        }
    }

    fn store_sticky_event(&self, event: &Event) -> Result<FlowSuccess, FlowError> {
        let ret: FlowReturn = unsafe {
            from_glib(ffi::gst_pad_store_sticky_event(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    fn set_pad_flags(&self, flags: PadFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.to_glib();
        }
    }

    fn unset_pad_flags(&self, flags: PadFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.to_glib();
        }
    }

    fn pad_flags(&self) -> PadFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}

unsafe fn create_probe_info<'a>(
    info: *mut ffi::GstPadProbeInfo,
) -> (PadProbeInfo<'a>, Option<glib::Type>) {
    let mut data_type = None;
    let flow_ret: FlowReturn = from_glib((*info).ABI.abi.flow_ret);
    let info = PadProbeInfo {
        mask: from_glib((*info).type_),
        id: Some(PadProbeId(NonZeroU64::new_unchecked((*info).id as u64))),
        offset: (*info).offset,
        size: (*info).size,
        data: if (*info).data.is_null() {
            None
        } else {
            let data = (*info).data as *mut ffi::GstMiniObject;
            (*info).data = ptr::null_mut();
            if (*data).type_ == Buffer::static_type().to_glib() {
                data_type = Some(Buffer::static_type());
                Some(PadProbeData::Buffer(from_glib_full(
                    data as *const ffi::GstBuffer,
                )))
            } else if (*data).type_ == BufferList::static_type().to_glib() {
                data_type = Some(BufferList::static_type());
                Some(PadProbeData::BufferList(from_glib_full(
                    data as *const ffi::GstBufferList,
                )))
            } else if (*data).type_ == Query::static_type().to_glib() {
                data_type = Some(Query::static_type());
                Some(PadProbeData::Query(QueryRef::from_mut_ptr(
                    data as *mut ffi::GstQuery,
                )))
            } else if (*data).type_ == Event::static_type().to_glib() {
                data_type = Some(Event::static_type());
                Some(PadProbeData::Event(from_glib_full(
                    data as *const ffi::GstEvent,
                )))
            } else {
                Some(PadProbeData::__Unknown(data))
            }
        },
        flow_res: flow_ret.into_result(),
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
        // Handled buffers are consumed
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
            Some(PadProbeData::Event(_)) => {
                assert_eq!(data_type, Some(Event::static_type()));
                // Event not consumed by probe; consume it here
            }
            None if data_type == Some(Buffer::static_type())
                || data_type == Some(Event::static_type()) =>
            {
                // Buffer or Event consumed by probe
            }
            other => panic!(
                "Bad data for {:?} pad probe returning Handled: {:?}",
                data_type, other
            ),
        }
    } else {
        match probe_info.data {
            Some(PadProbeData::Buffer(buffer)) => {
                assert_eq!(data_type, Some(Buffer::static_type()));
                (*info).data = buffer.into_ptr() as *mut libc::c_void;
            }
            Some(PadProbeData::BufferList(bufferlist)) => {
                assert_eq!(data_type, Some(BufferList::static_type()));
                (*info).data = bufferlist.into_ptr() as *mut libc::c_void;
            }
            Some(PadProbeData::Event(event)) => {
                assert_eq!(data_type, Some(Event::static_type()));
                (*info).data = event.into_ptr() as *mut libc::c_void;
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
    (*info).ABI.abi.flow_ret = flow_ret.to_glib();
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        &mut probe_info,
    );

    update_probe_info(ret, probe_info, data_type, info);

    ret.to_glib()
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
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
    .to_glib()
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
        &&Pad::from_glib_borrow(pad).unsafe_cast_ref(),
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
    .to_glib()
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(buffer),
    )
    .into();
    res.to_glib()
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(list),
    )
    .into();
    res.to_glib()
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(event),
    )
    .to_glib()
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(event),
    )
    .into();
    res.to_glib()
}

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

    assert!(!buffer.is_null());

    let pad = Pad::from_glib_borrow(pad);
    let pad = pad.unsafe_cast_ref();
    let mut passed_buffer = if (*buffer).is_null() {
        None
    } else {
        Some(crate::BufferRef::from_mut_ptr(*buffer))
    };

    match func(
        &pad,
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        offset,
        passed_buffer.as_deref_mut(),
        length,
    ) {
        Ok(PadGetRangeSuccess::NewBuffer(new_buffer)) => {
            if let Some(passed_buffer) = passed_buffer {
                gst_debug!(
                    crate::CAT_PERFORMANCE,
                    obj: pad.unsafe_cast_ref::<glib::Object>(),
                    "Returned new buffer from getrange function, copying into passed buffer"
                );

                let mut map = match passed_buffer.map_writable() {
                    Ok(map) => map,
                    Err(_) => {
                        gst_error!(
                            crate::CAT_RUST,
                            obj: pad.unsafe_cast_ref::<glib::Object>(),
                            "Failed to map passed buffer writable"
                        );
                        return ffi::GST_FLOW_ERROR;
                    }
                };

                let copied_size = new_buffer.copy_to_slice(0, &mut *map);
                drop(map);

                if let Err(copied_size) = copied_size {
                    passed_buffer.set_size(copied_size);
                }

                match new_buffer.copy_into(passed_buffer, crate::BUFFER_COPY_METADATA, 0, None) {
                    Ok(_) => FlowReturn::Ok.to_glib(),
                    Err(_) => {
                        gst_error!(
                            crate::CAT_RUST,
                            obj: pad.unsafe_cast_ref::<glib::Object>(),
                            "Failed to copy buffer metadata"
                        );

                        FlowReturn::Error.to_glib()
                    }
                }
            } else {
                *buffer = new_buffer.into_ptr();
                FlowReturn::Ok.to_glib()
            }
        }
        Ok(PadGetRangeSuccess::FilledBuffer) => {
            assert!(passed_buffer.is_some());
            FlowReturn::Ok.to_glib()
        }
        Err(ret) => FlowReturn::from_error(ret).to_glib(),
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
    );

    ret.into_ptr()
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        &from_glib_borrow(peer),
    )
    .into();
    res.to_glib()
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        crate::QueryRef::from_mut_ptr(query),
    )
    .to_glib()
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
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<crate::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
    )
}

unsafe extern "C" fn destroy_closure<F>(ptr: gpointer) {
    Box::<F>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn trampoline_pad_task<F: FnMut() + Send + 'static>(func: gpointer) {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (&mut *func.borrow_mut())()
}

fn into_raw_pad_task<F: FnMut() + Send + 'static>(func: F) -> gpointer {
    #[allow(clippy::type_complexity)]
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn destroy_closure_pad_task<F>(ptr: gpointer) {
    Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

impl Pad {
    pub fn new(name: Option<&str>, direction: crate::PadDirection) -> Self {
        skip_assert_initialized!();
        Self::builder(name, direction).build()
    }

    pub fn builder(name: Option<&str>, direction: crate::PadDirection) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::new(name, direction)
    }

    pub fn from_static_template(templ: &StaticPadTemplate, name: Option<&str>) -> Self {
        skip_assert_initialized!();
        Self::builder_with_static_template(templ, name).build()
    }

    pub fn builder_with_static_template(
        templ: &StaticPadTemplate,
        name: Option<&str>,
    ) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_static_template(templ, name)
    }

    pub fn from_template(templ: &crate::PadTemplate, name: Option<&str>) -> Self {
        skip_assert_initialized!();
        Self::builder_with_template(templ, name).build()
    }

    pub fn builder_with_template(
        templ: &crate::PadTemplate,
        name: Option<&str>,
    ) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_template(templ, name)
    }
}

pub struct PadBuilder<T>(pub(crate) T);

impl<T: IsA<Pad> + IsA<glib::Object> + glib::object::IsClass> PadBuilder<T> {
    pub fn new(name: Option<&str>, direction: crate::PadDirection) -> Self {
        assert_initialized_main_thread!();

        let pad = glib::Object::new::<T>(&[("name", &name), ("direction", &direction)])
            .expect("Failed to create pad");

        // Ghost pads are a bit special
        if let Some(pad) = pad.dynamic_cast_ref::<crate::GhostPad>() {
            unsafe {
                let res = ffi::gst_ghost_pad_construct(pad.to_glib_none().0);
                // This can't really fail...
                assert_ne!(res, glib::ffi::GFALSE, "Failed to construct ghost pad");
            }
        }

        PadBuilder(pad)
    }

    pub fn from_static_template(templ: &StaticPadTemplate, name: Option<&str>) -> Self {
        assert_initialized_main_thread!();

        let templ = templ.get();
        Self::from_template(&templ, name)
    }

    pub fn from_template(templ: &crate::PadTemplate, name: Option<&str>) -> Self {
        assert_initialized_main_thread!();

        use glib::ObjectExt;

        let mut type_ = T::static_type();

        // Since 1.14 templates can keep a pad GType with them, so we need to do some
        // additional checks here now
        if templ.has_property("gtype", Some(glib::Type::static_type())) {
            let gtype = templ
                .get_property("gtype")
                .unwrap()
                .get_some::<glib::Type>()
                .unwrap();

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

        let pad = glib::Object::with_type(
            type_,
            &[
                ("name", &name),
                ("direction", &templ.direction()),
                ("template", templ),
            ],
        )
        .expect("Failed to create pad")
        .downcast::<T>()
        .unwrap();

        // Ghost pads are a bit special
        if let Some(pad) = pad.dynamic_cast_ref::<crate::GhostPad>() {
            unsafe {
                let res = ffi::gst_ghost_pad_construct(pad.to_glib_none().0);
                // This can't really fail...
                assert_ne!(res, glib::ffi::GFALSE, "Failed to construct ghost pad");
            }
        }

        PadBuilder(pad)
    }

    pub fn activate_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_activate_function(func);
        }

        self
    }

    pub fn activatemode_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::PadMode, bool) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.0.set_activatemode_function(func);
        }

        self
    }

    pub fn chain_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Buffer) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.0.set_chain_function(func);
        }

        self
    }

    pub fn chain_list_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::BufferList) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.0.set_chain_list_function(func);
        }

        self
    }

    pub fn event_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Event) -> bool + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_event_function(func);
        }

        self
    }

    pub fn event_full_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, crate::Event) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            self.0.set_event_full_function(func);
        }

        self
    }

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
            self.0.set_getrange_function(func);
        }

        self
    }

    pub fn iterate_internal_links_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) -> crate::Iterator<Pad> + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_iterate_internal_links_function(func);
        }

        self
    }

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
            self.0.set_link_function(func);
        }

        self
    }

    pub fn query_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>, &mut crate::QueryRef) -> bool + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_query_function(func);
        }

        self
    }

    pub fn unlink_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&crate::Object>) + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_unlink_function(func);
        }

        self
    }

    pub fn flags(self, flags: PadFlags) -> Self {
        self.0.set_pad_flags(flags);

        self
    }

    pub fn build(self) -> T {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::sync::{atomic::AtomicUsize, mpsc::channel};
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_event_chain_functions() {
        crate::init().unwrap();

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        let buffers = Arc::new(Mutex::new(Vec::new()));
        let buffers_clone = buffers.clone();
        let pad = crate::Pad::builder(Some("sink"), crate::PadDirection::Sink)
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

        let pad = crate::Pad::builder(Some("src"), crate::PadDirection::Src)
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

        let buffer = pad.get_range(0, 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        let mut buffer = crate::Buffer::with_size(5).unwrap();
        pad.get_range_fill(0, buffer.get_mut().unwrap(), 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        pad.set_active(false).unwrap();
        drop(pad);

        let pad = crate::Pad::builder(Some("src"), crate::PadDirection::Src)
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

        let buffer = pad.get_range(0, 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        let mut buffer = crate::Buffer::with_size(5).unwrap();
        pad.get_range_fill(0, buffer.get_mut().unwrap(), 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"fghij");
    }

    #[test]
    fn test_task() {
        crate::init().unwrap();

        let pad = crate::Pad::new(Some("sink"), crate::PadDirection::Sink);
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

        let src_pad = crate::Pad::new(Some("src"), crate::PadDirection::Src);
        let sink_pad = crate::Pad::builder(Some("sink"), crate::PadDirection::Sink)
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

    #[test]
    fn test_probe() {
        crate::init().unwrap();

        let (major, minor, micro, _) = crate::version();
        let pad = crate::Pad::new(Some("src"), crate::PadDirection::Src);
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
                if let Some(PadProbeData::Event(event)) = info.data.take() {
                    let mut events = events.lock().unwrap();
                    events.push(event);
                } else {
                    unreachable!();
                }
                crate::PadProbeReturn::Handled
            });
        }

        {
            let buffers = buffers.clone();
            let flow_override = flow_override;
            pad.add_probe(crate::PadProbeType::BUFFER, move |_, info| {
                if let Some(PadProbeData::Buffer(buffer)) = info.data.take() {
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
                crate::PadProbeReturn::Handled
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
        assert_eq!(pad.push(crate::Buffer::new()), flow_override);

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
}
