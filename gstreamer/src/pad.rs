// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Buffer;
use BufferList;
use Event;
use FlowError;
use FlowReturn;
use FlowSuccess;
use Format;
use FormattedValue;
use GenericFormattedValue;
use LoggableError;
use Pad;
use PadFlags;
use PadLinkCheck;
use PadLinkError;
use PadLinkReturn;
use PadLinkSuccess;
use PadProbeReturn;
use PadProbeType;
use Query;
use QueryRef;
use SpecificFormattedValue;
use StaticPadTemplate;

use std::cell::RefCell;
use std::mem;
use std::num::NonZeroU64;
use std::ptr;

use glib;
use glib::object::{Cast, IsA};
use glib::translate::{
    from_glib, from_glib_borrow, from_glib_full, FromGlib, FromGlibPtrBorrow, ToGlib, ToGlibPtr,
};
use glib::StaticType;
use glib_sys;
use glib_sys::gpointer;

use libc;

use gst_sys;

#[derive(Debug, PartialEq, Eq)]
pub struct PadProbeId(NonZeroU64);

impl ToGlib for PadProbeId {
    type GlibType = libc::c_ulong;

    fn to_glib(&self) -> libc::c_ulong {
        self.0.get() as libc::c_ulong
    }
}

impl FromGlib<libc::c_ulong> for PadProbeId {
    fn from_glib(val: libc::c_ulong) -> PadProbeId {
        skip_assert_initialized!();
        assert_ne!(val, 0);
        PadProbeId(unsafe { NonZeroU64::new_unchecked(val as u64) })
    }
}

#[derive(Debug)]
pub struct PadProbeInfo<'a> {
    pub mask: PadProbeType,
    pub id: PadProbeId,
    pub offset: u64,
    pub size: u32,
    pub data: Option<PadProbeData<'a>>,
    pub flow_ret: FlowReturn,
}

#[derive(Debug)]
pub enum PadProbeData<'a> {
    Buffer(Buffer),
    BufferList(BufferList),
    Query(&'a mut QueryRef),
    Event(Event),
    #[doc(hidden)]
    __Unknown(*mut gst_sys::GstMiniObject),
}

unsafe impl<'a> Send for PadProbeData<'a> {}
unsafe impl<'a> Sync for PadProbeData<'a> {}

#[derive(Debug)]
#[must_use = "if unused the StreamLock will immediately unlock"]
pub struct StreamLock<'a>(&'a Pad);
impl<'a> Drop for StreamLock<'a> {
    fn drop(&mut self) {
        unsafe {
            let pad: *mut gst_sys::GstPad = self.0.to_glib_none().0;
            glib_sys::g_rec_mutex_unlock(&mut (*pad).stream_rec_lock);
        }
    }
}

#[derive(Debug)]
pub enum PadGetRangeSuccess {
    FilledBuffer,
    NewBuffer(::Buffer),
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
        buffer: &mut ::BufferRef,
        size: u32,
    ) -> Result<(), FlowError>;
    fn get_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowError>;
    fn get_range_fill(
        &self,
        offset: u64,
        buffer: &mut ::BufferRef,
        size: u32,
    ) -> Result<(), FlowError>;

    fn peer_query(&self, query: &mut QueryRef) -> bool;
    fn query(&self, query: &mut QueryRef) -> bool;
    fn query_default<P: IsA<::Object>>(&self, parent: Option<&P>, query: &mut QueryRef) -> bool;
    fn proxy_query_caps(&self, query: &mut QueryRef) -> bool;
    fn proxy_query_accept_caps(&self, query: &mut QueryRef) -> bool;

    fn event_default<P: IsA<::Object>>(&self, parent: Option<&P>, event: Event) -> bool;
    fn push_event(&self, event: Event) -> bool;
    fn send_event(&self, event: Event) -> bool;

    fn get_last_flow_return(&self) -> Result<FlowSuccess, FlowError>;

    fn iterate_internal_links(&self) -> ::Iterator<Pad>;
    fn iterate_internal_links_default<P: IsA<::Object>>(
        &self,
        parent: Option<&P>,
    ) -> ::Iterator<Pad>;

    fn link<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<PadLinkSuccess, PadLinkError>;
    fn link_full<P: IsA<Pad>>(
        &self,
        sinkpad: &P,
        flags: PadLinkCheck,
    ) -> Result<PadLinkSuccess, PadLinkError>;

    fn stream_lock(&self) -> StreamLock;

    unsafe fn set_activate_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>) -> Result<(), LoggableError> + Send + Sync + 'static;

    unsafe fn set_activatemode_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::PadMode, bool) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_chain_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::Buffer) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_chain_list_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::BufferList) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_event_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::Event) -> bool + Send + Sync + 'static;

    unsafe fn set_event_full_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::Event) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_getrange_function<F>(&self, func: F)
    where
        F: Fn(
                &Self,
                Option<&::Object>,
                u64,
                Option<&mut ::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, ::FlowError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_iterate_internal_links_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>) -> ::Iterator<Pad> + Send + Sync + 'static;

    unsafe fn set_link_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, &Pad) -> Result<::PadLinkSuccess, ::PadLinkError>
            + Send
            + Sync
            + 'static;

    unsafe fn set_query_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, &mut ::QueryRef) -> bool + Send + Sync + 'static;

    unsafe fn set_unlink_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>) + Send + Sync + 'static;

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

    fn get_mode(&self) -> ::PadMode;

    fn sticky_events_foreach<F: FnMut(Event) -> Result<Option<Event>, Option<Event>>>(
        &self,
        func: F,
    );

    fn store_sticky_event(&self, event: &Event) -> Result<FlowSuccess, FlowError>;

    fn set_pad_flags(&self, flags: PadFlags);

    fn unset_pad_flags(&self, flags: PadFlags);

    fn get_pad_flags(&self) -> PadFlags;
}

impl<O: IsA<Pad>> PadExtManual for O {
    fn add_probe<F>(&self, mask: PadProbeType, func: F) -> Option<PadProbeId>
    where
        F: Fn(&Self, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<F> = Box::new(func);
            let id = gst_sys::gst_pad_add_probe(
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
            gst_sys::gst_pad_remove_probe(self.as_ref().to_glib_none().0, id.to_glib());
        }
    }

    fn chain(&self, buffer: Buffer) -> Result<FlowSuccess, FlowError> {
        unsafe {
            FlowReturn::from_glib(gst_sys::gst_pad_chain(
                self.as_ref().to_glib_none().0,
                buffer.into_ptr(),
            ))
            .into_result()
        }
    }

    fn push(&self, buffer: Buffer) -> Result<FlowSuccess, FlowError> {
        unsafe {
            FlowReturn::from_glib(gst_sys::gst_pad_push(
                self.as_ref().to_glib_none().0,
                buffer.into_ptr(),
            ))
            .into_result()
        }
    }

    fn chain_list(&self, list: BufferList) -> Result<FlowSuccess, FlowError> {
        unsafe {
            FlowReturn::from_glib(gst_sys::gst_pad_chain_list(
                self.as_ref().to_glib_none().0,
                list.into_ptr(),
            ))
            .into_result()
        }
    }

    fn push_list(&self, list: BufferList) -> Result<FlowSuccess, FlowError> {
        unsafe {
            FlowReturn::from_glib(gst_sys::gst_pad_push_list(
                self.as_ref().to_glib_none().0,
                list.into_ptr(),
            ))
            .into_result()
        }
    }

    fn get_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowError> {
        unsafe {
            let mut buffer = ptr::null_mut();
            let ret: FlowReturn = from_glib(gst_sys::gst_pad_get_range(
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
        buffer: &mut ::BufferRef,
        size: u32,
    ) -> Result<(), FlowError> {
        assert!(buffer.get_size() >= size as usize);

        unsafe {
            let mut buffer_ref = buffer.as_mut_ptr();
            let ret: FlowReturn = from_glib(gst_sys::gst_pad_get_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer_ref,
            ));
            match ret.into_result_value(|| ()) {
                Ok(_) => {
                    if buffer.as_mut_ptr() != buffer_ref {
                        gst_sys::gst_mini_object_unref(buffer_ref as *mut _);
                        Err(::FlowError::Error)
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
            let ret: FlowReturn = from_glib(gst_sys::gst_pad_pull_range(
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
        buffer: &mut ::BufferRef,
        size: u32,
    ) -> Result<(), FlowError> {
        assert!(buffer.get_size() >= size as usize);

        unsafe {
            let mut buffer_ref = buffer.as_mut_ptr();
            let ret: FlowReturn = from_glib(gst_sys::gst_pad_pull_range(
                self.as_ref().to_glib_none().0,
                offset,
                size,
                &mut buffer_ref,
            ));
            match ret.into_result_value(|| ()) {
                Ok(_) => {
                    if buffer.as_mut_ptr() != buffer_ref {
                        gst_sys::gst_mini_object_unref(buffer_ref as *mut _);
                        Err(::FlowError::Error)
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
            from_glib(gst_sys::gst_pad_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn peer_query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_peer_query(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn query_default<P: IsA<::Object>>(&self, parent: Option<&P>, query: &mut QueryRef) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(gst_sys::gst_pad_query_default(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn proxy_query_accept_caps(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_proxy_query_accept_caps(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn proxy_query_caps(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_proxy_query_accept_caps(
                self.as_ref().to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn event_default<P: IsA<::Object>>(&self, parent: Option<&P>, event: Event) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(gst_sys::gst_pad_event_default(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn push_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_push_event(
                self.as_ref().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn send_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_send_event(
                self.as_ref().to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn get_last_flow_return(&self) -> Result<FlowSuccess, FlowError> {
        let ret: FlowReturn = unsafe {
            from_glib(gst_sys::gst_pad_get_last_flow_return(
                self.as_ref().to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    fn iterate_internal_links(&self) -> ::Iterator<Pad> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_iterate_internal_links(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_internal_links_default<P: IsA<::Object>>(
        &self,
        parent: Option<&P>,
    ) -> ::Iterator<Pad> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_iterate_internal_links_default(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    fn link<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<PadLinkSuccess, PadLinkError> {
        let ret: PadLinkReturn = unsafe {
            from_glib(gst_sys::gst_pad_link(
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
            from_glib(gst_sys::gst_pad_link_full(
                self.as_ref().to_glib_none().0,
                sinkpad.as_ref().to_glib_none().0,
                flags.to_glib(),
            ))
        };
        ret.into_result()
    }

    fn stream_lock(&self) -> StreamLock {
        unsafe {
            let ptr: &mut gst_sys::GstPad = &mut *(self.as_ptr() as *mut _);
            glib_sys::g_rec_mutex_lock(&mut ptr.stream_rec_lock);
            StreamLock(self.upcast_ref())
        }
    }

    unsafe fn set_activate_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_activate_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_activate_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_activatemode_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::PadMode, bool) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_activatemode_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_activatemode_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_chain_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::Buffer) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_chain_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_chain_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_chain_list_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::BufferList) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_chain_list_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_chain_list_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_event_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::Event) -> bool + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_event_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_event_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_event_full_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, ::Event) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_event_full_function_full(
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
                Option<&::Object>,
                u64,
                Option<&mut ::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, ::FlowError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_getrange_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_getrange_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_iterate_internal_links_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>) -> ::Iterator<Pad> + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_iterate_internal_links_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_iterate_internal_links_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_link_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, &Pad) -> Result<::PadLinkSuccess, ::PadLinkError>
            + Send
            + Sync
            + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_link_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_link_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_query_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>, &mut ::QueryRef) -> bool + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_query_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_query_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    unsafe fn set_unlink_function<F>(&self, func: F)
    where
        F: Fn(&Self, Option<&::Object>) + Send + Sync + 'static,
    {
        let func_box: Box<F> = Box::new(func);
        gst_sys::gst_pad_set_unlink_function_full(
            self.as_ref().to_glib_none().0,
            Some(trampoline_unlink_function::<Self, F>),
            Box::into_raw(func_box) as gpointer,
            Some(destroy_closure::<F>),
        );
    }

    fn start_task<F: FnMut() + Send + 'static>(&self, func: F) -> Result<(), glib::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_pad_start_task(
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
            let ret = from_glib(gst_sys::gst_pad_peer_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.get_format().to_glib(),
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
            let ret = from_glib(gst_sys::gst_pad_peer_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.get_format().to_glib(),
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
            let ret = from_glib(gst_sys::gst_pad_peer_query_duration(
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
            let ret = from_glib(gst_sys::gst_pad_peer_query_duration(
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
            let ret = from_glib(gst_sys::gst_pad_peer_query_position(
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
            let ret = from_glib(gst_sys::gst_pad_peer_query_position(
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
            let ret = from_glib(gst_sys::gst_pad_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.get_format().to_glib(),
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
            let ret = from_glib(gst_sys::gst_pad_query_convert(
                self.as_ref().to_glib_none().0,
                src_val.get_format().to_glib(),
                src_val.get_value(),
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
            let ret = from_glib(gst_sys::gst_pad_query_duration(
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
            let ret = from_glib(gst_sys::gst_pad_query_duration(
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
            let ret = from_glib(gst_sys::gst_pad_query_position(
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
            let ret = from_glib(gst_sys::gst_pad_query_position(
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

    fn get_mode(&self) -> ::PadMode {
        unsafe {
            let ptr: &gst_sys::GstPad = &*(self.as_ptr() as *const _);
            from_glib(ptr.mode)
        }
    }

    fn sticky_events_foreach<F: FnMut(Event) -> Result<Option<Event>, Option<Event>>>(
        &self,
        func: F,
    ) {
        unsafe extern "C" fn trampoline(
            _pad: *mut gst_sys::GstPad,
            event: *mut *mut gst_sys::GstEvent,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let func =
                user_data as *mut &mut (dyn FnMut(Event) -> Result<Option<Event>, Option<Event>>);
            let res = (*func)(from_glib_full(*event));

            match res {
                Ok(Some(ev)) => {
                    *event = ev.into_ptr();
                    glib_sys::GTRUE
                }
                Err(Some(ev)) => {
                    *event = ev.into_ptr();
                    glib_sys::GFALSE
                }
                Ok(None) => {
                    *event = ptr::null_mut();
                    glib_sys::GTRUE
                }
                Err(None) => {
                    *event = ptr::null_mut();
                    glib_sys::GFALSE
                }
            }
        }

        unsafe {
            let mut func = func;
            let func_obj: &mut (dyn FnMut(Event) -> Result<Option<Event>, Option<Event>>) =
                &mut func;
            let func_ptr = &func_obj
                as *const &mut (dyn FnMut(Event) -> Result<Option<Event>, Option<Event>>)
                as glib_sys::gpointer;

            gst_sys::gst_pad_sticky_events_foreach(
                self.as_ref().to_glib_none().0,
                Some(trampoline),
                func_ptr,
            );
        }
    }

    fn store_sticky_event(&self, event: &Event) -> Result<FlowSuccess, FlowError> {
        let ret: FlowReturn = unsafe {
            from_glib(gst_sys::gst_pad_store_sticky_event(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    fn set_pad_flags(&self, flags: PadFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.to_glib();
        }
    }

    fn unset_pad_flags(&self, flags: PadFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.to_glib();
        }
    }

    fn get_pad_flags(&self) -> PadFlags {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}

unsafe fn create_probe_info<'a>(
    info: *mut gst_sys::GstPadProbeInfo,
) -> (PadProbeInfo<'a>, Option<glib::Type>) {
    let mut data_type = None;
    let info = PadProbeInfo {
        mask: from_glib((*info).type_),
        id: PadProbeId(NonZeroU64::new_unchecked((*info).id as u64)),
        offset: (*info).offset,
        size: (*info).size,
        data: if (*info).data.is_null() {
            None
        } else {
            let data = (*info).data as *mut gst_sys::GstMiniObject;
            (*info).data = ptr::null_mut();
            if (*data).type_ == Buffer::static_type().to_glib() {
                data_type = Some(Buffer::static_type());
                Some(PadProbeData::Buffer(from_glib_full(
                    data as *const gst_sys::GstBuffer,
                )))
            } else if (*data).type_ == BufferList::static_type().to_glib() {
                data_type = Some(BufferList::static_type());
                Some(PadProbeData::BufferList(from_glib_full(
                    data as *const gst_sys::GstBufferList,
                )))
            } else if (*data).type_ == Query::static_type().to_glib() {
                data_type = Some(Query::static_type());
                Some(PadProbeData::Query(QueryRef::from_mut_ptr(
                    data as *mut gst_sys::GstQuery,
                )))
            } else if (*data).type_ == Event::static_type().to_glib() {
                data_type = Some(Event::static_type());
                Some(PadProbeData::Event(from_glib_full(
                    data as *const gst_sys::GstEvent,
                )))
            } else {
                Some(PadProbeData::__Unknown(data))
            }
        },
        flow_ret: from_glib((*info).ABI.abi.flow_ret),
    };
    (info, data_type)
}

unsafe fn update_probe_info(
    ret: PadProbeReturn,
    probe_info: PadProbeInfo,
    data_type: Option<glib::Type>,
    info: *mut gst_sys::GstPadProbeInfo,
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

    (*info).ABI.abi.flow_ret = probe_info.flow_ret.to_glib();
}

unsafe extern "C" fn trampoline_pad_probe<
    T,
    F: Fn(&T, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    info: *mut gst_sys::GstPadProbeInfo,
    func: gpointer,
) -> gst_sys::GstPadProbeReturn
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
    F: Fn(&T, Option<&::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
) -> glib_sys::gboolean
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).activatedata as *const F);

    match func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
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
    F: Fn(&T, Option<&::Object>, ::PadMode, bool) -> Result<(), LoggableError> + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
    mode: gst_sys::GstPadMode,
    active: glib_sys::gboolean,
) -> glib_sys::gboolean
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).activatemodedata as *const F);

    match func(
        &&Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
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
    F: Fn(&T, Option<&::Object>, ::Buffer) -> Result<FlowSuccess, FlowError> + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
    buffer: *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).chaindata as *const F);

    let res: FlowReturn = func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(buffer),
    )
    .into();
    res.to_glib()
}

unsafe extern "C" fn trampoline_chain_list_function<
    T,
    F: Fn(&T, Option<&::Object>, ::BufferList) -> Result<FlowSuccess, FlowError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
    list: *mut gst_sys::GstBufferList,
) -> gst_sys::GstFlowReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).chainlistdata as *const F);

    let res: FlowReturn = func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(list),
    )
    .into();
    res.to_glib()
}

unsafe extern "C" fn trampoline_event_function<
    T,
    F: Fn(&T, Option<&::Object>, ::Event) -> bool + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
    event: *mut gst_sys::GstEvent,
) -> glib_sys::gboolean
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).eventdata as *const F);

    func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        from_glib_full(event),
    )
    .to_glib()
}

unsafe extern "C" fn trampoline_event_full_function<
    T,
    F: Fn(&T, Option<&::Object>, ::Event) -> Result<FlowSuccess, FlowError> + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
    event: *mut gst_sys::GstEvent,
) -> gst_sys::GstFlowReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).eventdata as *const F);

    let res: FlowReturn = func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
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
            Option<&::Object>,
            u64,
            Option<&mut ::BufferRef>,
            u32,
        ) -> Result<PadGetRangeSuccess, ::FlowError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
    offset: u64,
    length: u32,
    buffer: *mut *mut gst_sys::GstBuffer,
) -> gst_sys::GstFlowReturn
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
        Some(::BufferRef::from_mut_ptr(*buffer))
    };

    match func(
        &pad,
        Option::<::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        offset,
        passed_buffer.as_deref_mut(),
        length,
    ) {
        Ok(PadGetRangeSuccess::NewBuffer(new_buffer)) => {
            if let Some(passed_buffer) = passed_buffer {
                gst_debug!(
                    ::CAT_PERFORMANCE,
                    obj: pad.unsafe_cast_ref::<glib::Object>(),
                    "Returned new buffer from getrange function, copying into passed buffer"
                );

                let mut map = match passed_buffer.map_writable() {
                    Ok(map) => map,
                    Err(_) => {
                        gst_error!(
                            ::CAT_RUST,
                            obj: pad.unsafe_cast_ref::<glib::Object>(),
                            "Failed to map passed buffer writable"
                        );
                        return gst_sys::GST_FLOW_ERROR;
                    }
                };

                let copied_size = new_buffer.copy_to_slice(0, &mut *map);
                drop(map);

                if let Err(copied_size) = copied_size {
                    passed_buffer.set_size(copied_size);
                }

                match new_buffer.copy_into(passed_buffer, ::BUFFER_COPY_METADATA, 0, None) {
                    Ok(_) => FlowReturn::Ok.to_glib(),
                    Err(_) => {
                        gst_error!(
                            ::CAT_RUST,
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
    F: Fn(&T, Option<&::Object>) -> ::Iterator<Pad> + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
) -> *mut gst_sys::GstIterator
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).iterintlinkdata as *const F);

    // Steal the iterator and return it
    let ret = func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
    );

    ret.into_ptr()
}

unsafe extern "C" fn trampoline_link_function<
    T,
    F: Fn(&T, Option<&::Object>, &::Pad) -> Result<::PadLinkSuccess, ::PadLinkError>
        + Send
        + Sync
        + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
    peer: *mut gst_sys::GstPad,
) -> gst_sys::GstPadLinkReturn
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).linkdata as *const F);

    let res: ::PadLinkReturn = func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        &from_glib_borrow(peer),
    )
    .into();
    res.to_glib()
}

unsafe extern "C" fn trampoline_query_function<
    T,
    F: Fn(&T, Option<&::Object>, &mut ::QueryRef) -> bool + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
    query: *mut gst_sys::GstQuery,
) -> glib_sys::gboolean
where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).querydata as *const F);

    func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
            .as_ref()
            .as_ref(),
        ::QueryRef::from_mut_ptr(query),
    )
    .to_glib()
}

unsafe extern "C" fn trampoline_unlink_function<
    T,
    F: Fn(&T, Option<&::Object>) + Send + Sync + 'static,
>(
    pad: *mut gst_sys::GstPad,
    parent: *mut gst_sys::GstObject,
) where
    T: IsA<Pad>,
{
    let func: &F = &*((*pad).unlinkdata as *const F);

    func(
        &Pad::from_glib_borrow(pad).unsafe_cast_ref(),
        Option::<::Object>::from_glib_borrow(parent)
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
    pub fn new(name: Option<&str>, direction: ::PadDirection) -> Self {
        skip_assert_initialized!();
        Self::builder(name, direction).build()
    }

    pub fn builder(name: Option<&str>, direction: ::PadDirection) -> PadBuilder<Self> {
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

    pub fn from_template(templ: &::PadTemplate, name: Option<&str>) -> Self {
        skip_assert_initialized!();
        Self::builder_with_template(templ, name).build()
    }

    pub fn builder_with_template(templ: &::PadTemplate, name: Option<&str>) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_template(templ, name)
    }
}

pub struct PadBuilder<T>(pub(crate) T);

impl<T: IsA<Pad> + IsA<glib::Object>> PadBuilder<T> {
    pub fn new(name: Option<&str>, direction: ::PadDirection) -> Self {
        assert_initialized_main_thread!();

        let pad = glib::Object::new(
            T::static_type(),
            &[("name", &name), ("direction", &direction)],
        )
        .expect("Failed to create pad")
        .downcast::<T>()
        .unwrap();

        // Ghost pads are a bit special
        if let Some(pad) = pad.dynamic_cast_ref::<::GhostPad>() {
            unsafe {
                let res = gst_sys::gst_ghost_pad_construct(pad.to_glib_none().0);
                // This can't really fail...
                assert_ne!(res, glib_sys::GFALSE, "Failed to construct ghost pad");
            }
        }

        PadBuilder(pad)
    }

    pub fn from_static_template(templ: &StaticPadTemplate, name: Option<&str>) -> Self {
        assert_initialized_main_thread!();

        let templ = templ.get();
        Self::from_template(&templ, name)
    }

    pub fn from_template(templ: &::PadTemplate, name: Option<&str>) -> Self {
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

            if gtype == glib::Type::Unit {
                // Nothing to be done, we can create any kind of pad
            } else if gtype.is_a(&type_) {
                // We were asked to create a parent type of the template type, e.g. a gst::Pad for
                // a template that wants a gst_base::AggregatorPad. Not a problem: update the type
                type_ = gtype;
            } else {
                // Otherwise the requested type must be a subclass of the template pad type
                assert!(type_.is_a(&gtype));
            }
        }

        let pad = glib::Object::new(
            type_,
            &[
                ("name", &name),
                ("direction", &templ.get_property_direction()),
                ("template", templ),
            ],
        )
        .expect("Failed to create pad")
        .downcast::<T>()
        .unwrap();

        // Ghost pads are a bit special
        if let Some(pad) = pad.dynamic_cast_ref::<::GhostPad>() {
            unsafe {
                let res = gst_sys::gst_ghost_pad_construct(pad.to_glib_none().0);
                // This can't really fail...
                assert_ne!(res, glib_sys::GFALSE, "Failed to construct ghost pad");
            }
        }

        PadBuilder(pad)
    }

    pub fn activate_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_activate_function(func);
        }

        self
    }

    pub fn activatemode_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&::Object>, ::PadMode, bool) -> Result<(), LoggableError>
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
        F: Fn(&T, Option<&::Object>, ::Buffer) -> Result<FlowSuccess, FlowError>
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
        F: Fn(&T, Option<&::Object>, ::BufferList) -> Result<FlowSuccess, FlowError>
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
        F: Fn(&T, Option<&::Object>, ::Event) -> bool + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_event_function(func);
        }

        self
    }

    pub fn event_full_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&::Object>, ::Event) -> Result<FlowSuccess, FlowError>
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
                Option<&::Object>,
                u64,
                Option<&mut ::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, ::FlowError>
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
        F: Fn(&T, Option<&::Object>) -> ::Iterator<Pad> + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_iterate_internal_links_function(func);
        }

        self
    }

    pub fn link_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&::Object>, &Pad) -> Result<::PadLinkSuccess, ::PadLinkError>
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
        F: Fn(&T, Option<&::Object>, &mut ::QueryRef) -> bool + Send + Sync + 'static,
    {
        unsafe {
            self.0.set_query_function(func);
        }

        self
    }

    pub fn unlink_function<F>(self, func: F) -> Self
    where
        F: Fn(&T, Option<&::Object>) + Send + Sync + 'static,
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
    use prelude::*;
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_event_chain_functions() {
        ::init().unwrap();

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        let buffers = Arc::new(Mutex::new(Vec::new()));
        let buffers_clone = buffers.clone();
        let pad = ::Pad::builder(Some("sink"), ::PadDirection::Sink)
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

        assert!(pad.send_event(::event::StreamStart::new("test")));
        let segment = ::FormattedSegment::<::ClockTime>::new();
        assert!(pad.send_event(::event::Segment::new(segment.as_ref())));

        assert_eq!(pad.chain(::Buffer::new()), Ok(FlowSuccess::Ok));

        let events = events.lock().unwrap();
        let buffers = buffers.lock().unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(buffers.len(), 1);

        match events[0].view() {
            ::EventView::StreamStart(..) => (),
            _ => unreachable!(),
        }

        match events[1].view() {
            ::EventView::Segment(..) => (),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_getrange_function() {
        ::init().unwrap();

        let pad = ::Pad::builder(Some("src"), ::PadDirection::Src)
            .activate_function(|pad, _parent| {
                pad.activate_mode(::PadMode::Pull, true)
                    .map_err(|err| err.into())
            })
            .getrange_function(|_pad, _parent, offset, _buffer, size| {
                assert_eq!(offset, 0);
                assert_eq!(size, 5);
                let buffer = ::Buffer::from_slice(b"abcde");
                Ok(PadGetRangeSuccess::NewBuffer(buffer))
            })
            .build();
        pad.set_active(true).unwrap();

        let buffer = pad.get_range(0, 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        let mut buffer = ::Buffer::with_size(5).unwrap();
        pad.get_range_fill(0, buffer.get_mut().unwrap(), 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        pad.set_active(false).unwrap();
        drop(pad);

        let pad = ::Pad::builder(Some("src"), ::PadDirection::Src)
            .activate_function(|pad, _parent| {
                pad.activate_mode(::PadMode::Pull, true)
                    .map_err(|err| err.into())
            })
            .getrange_function(|_pad, _parent, offset, buffer, size| {
                assert_eq!(offset, 0);
                assert_eq!(size, 5);
                if let Some(buffer) = buffer {
                    buffer.copy_from_slice(0, b"fghij").unwrap();
                    Ok(PadGetRangeSuccess::FilledBuffer)
                } else {
                    let buffer = ::Buffer::from_slice(b"abcde");
                    Ok(PadGetRangeSuccess::NewBuffer(buffer))
                }
            })
            .build();
        pad.set_active(true).unwrap();

        let buffer = pad.get_range(0, 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"abcde");

        let mut buffer = ::Buffer::with_size(5).unwrap();
        pad.get_range_fill(0, buffer.get_mut().unwrap(), 5).unwrap();
        let map = buffer.map_readable().unwrap();
        assert_eq!(&*map, b"fghij");
    }

    #[test]
    fn test_task() {
        ::init().unwrap();

        let pad = ::Pad::new(Some("sink"), ::PadDirection::Sink);
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
    fn test_probe() {
        ::init().unwrap();

        let (major, minor, micro, _) = ::version();
        let pad = ::Pad::new(Some("src"), ::PadDirection::Src);
        let events = Arc::new(Mutex::new(Vec::new()));
        let buffers = Arc::new(Mutex::new(Vec::new()));

        let flow_override = if (major, minor, micro) >= (1, 16, 1) {
            FlowReturn::Eos
        } else {
            // Broken on 1.16.0
            // https://gitlab.freedesktop.org/gstreamer/gstreamer/merge_requests/151
            FlowReturn::Ok
        };

        {
            let events = events.clone();
            pad.add_probe(::PadProbeType::EVENT_DOWNSTREAM, move |_, info| {
                if let Some(PadProbeData::Event(event)) = &info.data {
                    let mut events = events.lock().unwrap();
                    events.push(event.clone());
                } else {
                    unreachable!();
                }
                ::PadProbeReturn::Ok
            });
        }

        {
            let events = events.clone();
            pad.add_probe(::PadProbeType::EVENT_UPSTREAM, move |_, info| {
                if let Some(PadProbeData::Event(event)) = info.data.take() {
                    let mut events = events.lock().unwrap();
                    events.push(event);
                } else {
                    unreachable!();
                }
                ::PadProbeReturn::Handled
            });
        }

        {
            let buffers = buffers.clone();
            let flow_override = flow_override;
            pad.add_probe(::PadProbeType::BUFFER, move |_, info| {
                if let Some(PadProbeData::Buffer(buffer)) = info.data.take() {
                    let mut buffers = buffers.lock().unwrap();
                    info.flow_ret = if buffers.is_empty() {
                        FlowReturn::Ok
                    } else {
                        flow_override
                    };
                    buffers.push(buffer);
                } else {
                    unreachable!();
                }
                ::PadProbeReturn::Handled
            });
        }

        pad.set_active(true).unwrap();

        assert!(pad.send_event(::event::Latency::new(::ClockTime::from_nseconds(10))));
        assert!(pad.push_event(::event::StreamStart::new("test")));
        let segment = ::FormattedSegment::<::ClockTime>::new();
        assert!(pad.push_event(::event::Segment::new(segment.as_ref())));

        assert_eq!(pad.push(::Buffer::new()), Ok(FlowSuccess::Ok));
        assert_eq!(pad.push(::Buffer::new()), flow_override.into_result());

        let events = events.lock().unwrap();
        let buffers = buffers.lock().unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(buffers.len(), 2);

        assert_eq!(events[0].get_type(), ::EventType::Latency);
        assert_eq!(events[1].get_type(), ::EventType::StreamStart);
        assert_eq!(events[2].get_type(), ::EventType::Segment);

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
