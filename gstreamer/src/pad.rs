// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Pad;
use PadProbeType;
use PadProbeReturn;
use Buffer;
use BufferList;
use Format;
use FlowReturn;
use Query;
use QueryRef;
use Event;
use StaticPadTemplate;
use miniobject::MiniObject;

use std::mem::transmute;
use std::ptr;
use std::mem;
use std::cell::RefCell;

use glib;
use glib::{IsA, StaticType};
use glib::translate::{from_glib, from_glib_borrow, from_glib_full, from_glib_none, mut_override,
                      FromGlib, ToGlib, ToGlibPtr};
use glib::source::CallbackGuard;
use glib_ffi;
use glib_ffi::gpointer;
use glib::Object;

use libc;

use ffi;

impl Pad {
    pub fn new_from_static_template<'a, P: Into<Option<&'a str>>>(
        templ: &StaticPadTemplate,
        name: P,
    ) -> Pad {
        assert_initialized_main_thread!();
        let name = name.into();
        unsafe {
            from_glib_none(ffi::gst_pad_new_from_static_template(
                mut_override(templ.to_glib_none().0),
                name.to_glib_none().0,
            ))
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PadProbeId(libc::c_ulong);
pub const PAD_PROBE_ID_INVALID: PadProbeId = PadProbeId(0);

impl ToGlib for PadProbeId {
    type GlibType = libc::c_ulong;

    fn to_glib(&self) -> libc::c_ulong {
        self.0
    }
}

impl FromGlib<libc::c_ulong> for PadProbeId {
    fn from_glib(val: libc::c_ulong) -> PadProbeId {
        skip_assert_initialized!();
        PadProbeId(val)
    }
}

pub struct PadProbeInfo<'a> {
    pub mask: PadProbeType,
    pub id: PadProbeId,
    pub offset: u64,
    pub size: u32,
    pub data: Option<PadProbeData<'a>>,
}

pub enum PadProbeData<'a> {
    Buffer(Buffer),
    BufferList(BufferList),
    Query(&'a mut QueryRef),
    Event(Event),
    Unknown,
}

pub struct StreamLock(Pad);
impl Drop for StreamLock {
    fn drop(&mut self) {
        unsafe {
            let pad: *mut ffi::GstPad = self.0.to_glib_none().0;
            glib_ffi::g_rec_mutex_unlock(&mut (*pad).stream_rec_lock);
        }
    }
}

pub trait PadExtManual {
    fn add_probe<F>(&self, mask: PadProbeType, func: F) -> PadProbeId
    where
        F: Fn(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static;
    fn remove_probe(&self, id: PadProbeId);

    fn chain(&self, buffer: Buffer) -> FlowReturn;
    fn push(&self, buffer: Buffer) -> FlowReturn;

    fn chain_list(&self, list: BufferList) -> FlowReturn;
    fn push_list(&self, list: BufferList) -> FlowReturn;

    fn pull_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowReturn>;
    fn get_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowReturn>;

    fn peer_query(&self, query: &mut QueryRef) -> bool;
    fn query(&self, query: &mut QueryRef) -> bool;
    fn query_default<'a, P: IsA<Object> + 'a, Q: Into<Option<&'a P>>>(
        &self,
        parent: Q,
        query: &mut QueryRef,
    ) -> bool;
    fn proxy_query_caps(&self, query: &mut QueryRef) -> bool;
    fn proxy_query_accept_caps(&self, query: &mut QueryRef) -> bool;

    fn event_default<'a, P: IsA<::Object> + 'a, Q: Into<Option<&'a P>>>(
        &self,
        parent: Q,
        event: Event,
    ) -> bool;
    fn push_event(&self, event: Event) -> bool;
    fn send_event(&self, event: Event) -> bool;

    fn iterate_internal_links(&self) -> ::Iterator<Pad>;
    fn iterate_internal_links_default<'a, P: IsA<::Object> + 'a, Q: Into<Option<&'a P>>>(
        &self,
        parent: Q,
    ) -> ::Iterator<Pad>;

    fn stream_lock(&self) -> StreamLock;

    fn set_activate_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>) -> bool + Send + Sync + 'static;

    fn set_activatemode_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::PadMode, bool) -> bool + Send + Sync + 'static;

    fn set_chain_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::Buffer) -> ::FlowReturn + Send + Sync + 'static;

    fn set_chain_list_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::BufferList) -> ::FlowReturn + Send + Sync + 'static;

    fn set_event_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::Event) -> bool + Send + Sync + 'static;

    fn set_event_full_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::Event) -> ::FlowReturn + Send + Sync + 'static;

    fn set_getrange_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, u64, u32) -> Result<::Buffer, ::FlowReturn>
            + Send
            + Sync
            + 'static;

    fn set_iterate_internal_links_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>) -> ::Iterator<Pad> + Send + Sync + 'static;

    fn set_link_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, &Pad) -> ::PadLinkReturn + Send + Sync + 'static;

    fn set_query_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, &mut ::QueryRef) -> bool + Send + Sync + 'static;

    fn set_unlink_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>) + Send + Sync + 'static;

    fn start_task<F: FnMut() + Send + 'static>(&self, func: F) -> Result<(), glib::BoolError>;

    fn peer_query_convert<V: Into<::FormatValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<::FormatValue>;
    fn peer_query_duration(&self, format: Format) -> Option<::FormatValue>;
    fn peer_query_position(&self, format: Format) -> Option<::FormatValue>;
    fn query_convert<V: Into<::FormatValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<::FormatValue>;
    fn query_duration(&self, format: Format) -> Option<::FormatValue>;
    fn query_position(&self, format: Format) -> Option<::FormatValue>;
}

impl<O: IsA<Pad>> PadExtManual for O {
    fn add_probe<F>(&self, mask: PadProbeType, func: F) -> PadProbeId
    where
        F: Fn(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
            > = Box::new(func);
            let id = ffi::gst_pad_add_probe(
                self.to_glib_none().0,
                mask.to_glib(),
                Some(trampoline_pad_probe),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );

            from_glib(id)
        }
    }

    fn remove_probe(&self, id: PadProbeId) {
        unsafe {
            ffi::gst_pad_remove_probe(self.to_glib_none().0, id.to_glib());
        }
    }

    fn chain(&self, buffer: Buffer) -> FlowReturn {
        unsafe { from_glib(ffi::gst_pad_chain(self.to_glib_none().0, buffer.into_ptr())) }
    }

    fn push(&self, buffer: Buffer) -> FlowReturn {
        unsafe { from_glib(ffi::gst_pad_push(self.to_glib_none().0, buffer.into_ptr())) }
    }

    fn chain_list(&self, list: BufferList) -> FlowReturn {
        unsafe {
            from_glib(ffi::gst_pad_chain_list(
                self.to_glib_none().0,
                list.into_ptr(),
            ))
        }
    }

    fn push_list(&self, list: BufferList) -> FlowReturn {
        unsafe {
            from_glib(ffi::gst_pad_push_list(
                self.to_glib_none().0,
                list.into_ptr(),
            ))
        }
    }

    fn get_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowReturn> {
        unsafe {
            let mut buffer = ptr::null_mut();
            let ret = from_glib(ffi::gst_pad_get_range(
                self.to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ));
            if ret == FlowReturn::Ok {
                Ok(from_glib_full(buffer))
            } else {
                Err(ret)
            }
        }
    }

    fn pull_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowReturn> {
        unsafe {
            let mut buffer = ptr::null_mut();
            let ret = from_glib(ffi::gst_pad_pull_range(
                self.to_glib_none().0,
                offset,
                size,
                &mut buffer,
            ));
            if ret == FlowReturn::Ok {
                Ok(from_glib_full(buffer))
            } else {
                Err(ret)
            }
        }
    }

    fn query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_query(
                self.to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn peer_query(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_peer_query(
                self.to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn query_default<'a, P: IsA<Object> + 'a, Q: Into<Option<&'a P>>>(
        &self,
        parent: Q,
        query: &mut QueryRef,
    ) -> bool {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib(ffi::gst_pad_query_default(
                self.to_glib_none().0,
                parent.0 as *mut _,
                query.as_mut_ptr(),
            ))
        }
    }

    fn proxy_query_accept_caps(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_proxy_query_accept_caps(
                self.to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn proxy_query_caps(&self, query: &mut QueryRef) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_proxy_query_accept_caps(
                self.to_glib_none().0,
                query.as_mut_ptr(),
            ))
        }
    }

    fn event_default<'a, P: IsA<::Object> + 'a, Q: Into<Option<&'a P>>>(
        &self,
        parent: Q,
        event: Event,
    ) -> bool {
        skip_assert_initialized!();
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib(ffi::gst_pad_event_default(
                self.to_glib_none().0,
                parent.0,
                event.into_ptr(),
            ))
        }
    }

    fn push_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_push_event(
                self.to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn send_event(&self, event: Event) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_send_event(
                self.to_glib_none().0,
                event.into_ptr(),
            ))
        }
    }

    fn iterate_internal_links(&self) -> ::Iterator<Pad> {
        unsafe { from_glib_full(ffi::gst_pad_iterate_internal_links(self.to_glib_none().0)) }
    }

    fn iterate_internal_links_default<'a, P: IsA<::Object> + 'a, Q: Into<Option<&'a P>>>(
        &self,
        parent: Q,
    ) -> ::Iterator<Pad> {
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_pad_iterate_internal_links_default(
                self.to_glib_none().0,
                parent.0,
            ))
        }
    }

    fn stream_lock(&self) -> StreamLock {
        unsafe {
            let pad = self.to_glib_none().0;
            glib_ffi::g_rec_mutex_lock(&mut (*pad).stream_rec_lock);
            StreamLock(from_glib_none(pad))
        }
    }

    fn set_activate_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>) -> bool + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>) -> bool + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_activate_function_full(
                self.to_glib_none().0,
                Some(trampoline_activate_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_activatemode_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::PadMode, bool) -> bool + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>, ::PadMode, bool) -> bool + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_activatemode_function_full(
                self.to_glib_none().0,
                Some(trampoline_activatemode_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_chain_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::Buffer) -> ::FlowReturn + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>, ::Buffer) -> ::FlowReturn + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_chain_function_full(
                self.to_glib_none().0,
                Some(trampoline_chain_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_chain_list_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::BufferList) -> ::FlowReturn + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>, ::BufferList) -> ::FlowReturn + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_chain_list_function_full(
                self.to_glib_none().0,
                Some(trampoline_chain_list_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_event_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::Event) -> bool + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>, ::Event) -> bool + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_event_function_full(
                self.to_glib_none().0,
                Some(trampoline_event_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_event_full_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, ::Event) -> ::FlowReturn + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>, ::Event) -> ::FlowReturn + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_event_full_function_full(
                self.to_glib_none().0,
                Some(trampoline_event_full_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_getrange_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, u64, u32) -> Result<::Buffer, ::FlowReturn>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
            let func_box: Box<
                Fn(&Pad, &Option<::Object>, u64, u32) -> Result<::Buffer, ::FlowReturn>
                    + Send
                    + Sync
                    + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_getrange_function_full(
                self.to_glib_none().0,
                Some(trampoline_getrange_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_iterate_internal_links_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>) -> ::Iterator<Pad> + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>) -> ::Iterator<Pad> + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_iterate_internal_links_function_full(
                self.to_glib_none().0,
                Some(trampoline_iterate_internal_links_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_link_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, &Pad) -> ::PadLinkReturn + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>, &Pad) -> ::PadLinkReturn + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_link_function_full(
                self.to_glib_none().0,
                Some(trampoline_link_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_query_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>, &mut ::QueryRef) -> bool + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<
                Fn(&Pad, &Option<::Object>, &mut ::QueryRef) -> bool + Send + Sync + 'static,
            > = Box::new(func);
            ffi::gst_pad_set_query_function_full(
                self.to_glib_none().0,
                Some(trampoline_query_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn set_unlink_function<F>(&self, func: F)
    where
        F: Fn(&Pad, &Option<::Object>) + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<Fn(&Pad, &Option<::Object>) + Send + Sync + 'static> = Box::new(func);
            ffi::gst_pad_set_unlink_function_full(
                self.to_glib_none().0,
                Some(trampoline_unlink_function),
                Box::into_raw(Box::new(func_box)) as gpointer,
                Some(destroy_closure),
            );
        }
    }

    fn start_task<F: FnMut() + Send + 'static>(&self, func: F) -> Result<(), glib::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(
                ffi::gst_pad_start_task(
                    self.to_glib_none().0,
                    Some(trampoline_pad_task),
                    into_raw_pad_task(func),
                    Some(destroy_closure_pad_task),
                ),
                "Failed to start pad task",
            )
        }
    }

    fn peer_query_convert<V: Into<::FormatValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<::FormatValue> {
        let src_val = src_val.into();
        unsafe {
            let mut dest_val = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_peer_query_convert(
                self.to_glib_none().0,
                src_val.to_format().to_glib(),
                src_val.to_value(),
                dest_format.to_glib(),
                &mut dest_val,
            ));
            if ret {
                Some(::FormatValue::new(dest_format, dest_val))
            } else {
                None
            }
        }
    }

    fn peer_query_duration(&self, format: Format) -> Option<::FormatValue> {
        unsafe {
            let mut duration = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_peer_query_duration(
                self.to_glib_none().0,
                format.to_glib(),
                &mut duration,
            ));
            if ret {
                Some(::FormatValue::new(format, duration))
            } else {
                None
            }
        }
    }

    fn peer_query_position(&self, format: Format) -> Option<::FormatValue> {
        unsafe {
            let mut cur = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_peer_query_position(
                self.to_glib_none().0,
                format.to_glib(),
                &mut cur,
            ));
            if ret {
                Some(::FormatValue::new(format, cur))
            } else {
                None
            }
        }
    }

    fn query_convert<V: Into<::FormatValue>>(
        &self,
        src_val: V,
        dest_format: Format,
    ) -> Option<::FormatValue> {
        let src_val = src_val.into();

        unsafe {
            let mut dest_val = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_query_convert(
                self.to_glib_none().0,
                src_val.to_format().to_glib(),
                src_val.to_value(),
                dest_format.to_glib(),
                &mut dest_val,
            ));
            if ret {
                Some(::FormatValue::new(dest_format, dest_val))
            } else {
                None
            }
        }
    }

    fn query_duration(&self, format: Format) -> Option<::FormatValue> {
        unsafe {
            let mut duration = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_query_duration(
                self.to_glib_none().0,
                format.to_glib(),
                &mut duration,
            ));
            if ret {
                Some(::FormatValue::new(format, duration))
            } else {
                None
            }
        }
    }

    fn query_position(&self, format: Format) -> Option<::FormatValue> {
        unsafe {
            let mut cur = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_query_position(
                self.to_glib_none().0,
                format.to_glib(),
                &mut cur,
            ));
            if ret {
                Some(::FormatValue::new(format, cur))
            } else {
                None
            }
        }
    }
}

unsafe extern "C" fn trampoline_pad_probe(
    pad: *mut ffi::GstPad,
    info: *mut ffi::GstPadProbeInfo,
    func: gpointer,
) -> ffi::GstPadProbeReturn {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static) =
        transmute(func);
    let mut data_type = None;

    let mut probe_info = PadProbeInfo {
        mask: from_glib((*info).type_),
        id: PadProbeId((*info).id),
        offset: (*info).offset,
        size: (*info).size,
        data: if (*info).data.is_null() {
            None
        } else {
            let data = (*info).data as *const ffi::GstMiniObject;
            if (*data).type_ == Buffer::static_type().to_glib() {
                data_type = Some(Buffer::static_type());
                Some(PadProbeData::Buffer(
                    from_glib_none(data as *const ffi::GstBuffer),
                ))
            } else if (*data).type_ == BufferList::static_type().to_glib() {
                data_type = Some(BufferList::static_type());
                Some(PadProbeData::BufferList(
                    from_glib_none(data as *const ffi::GstBufferList),
                ))
            } else if (*data).type_ == Query::static_type().to_glib() {
                data_type = Some(Query::static_type());
                Some(PadProbeData::Query(
                    QueryRef::from_mut_ptr(data as *mut ffi::GstQuery),
                ))
            } else if (*data).type_ == Event::static_type().to_glib() {
                data_type = Some(Event::static_type());
                Some(PadProbeData::Event(
                    from_glib_none(data as *const ffi::GstEvent),
                ))
            } else {
                Some(PadProbeData::Unknown)
            }
        },
    };

    let ret = func(&from_glib_borrow(pad), &mut probe_info).to_glib();

    match probe_info.data {
        Some(PadProbeData::Buffer(buffer)) => {
            assert_eq!(data_type, Some(Buffer::static_type()));
            if (*info).data != buffer.as_mut_ptr() as *mut _ {
                ffi::gst_mini_object_unref((*info).data as *mut _);
                (*info).data = buffer.into_ptr() as *mut libc::c_void;
            }
        }
        Some(PadProbeData::BufferList(bufferlist)) => {
            assert_eq!(data_type, Some(BufferList::static_type()));
            if (*info).data != bufferlist.as_mut_ptr() as *mut _ {
                ffi::gst_mini_object_unref((*info).data as *mut _);
                (*info).data = bufferlist.into_ptr() as *mut libc::c_void;
            }
        }
        Some(PadProbeData::Event(event)) => {
            assert_eq!(data_type, Some(Event::static_type()));
            if (*info).data != event.as_mut_ptr() as *mut _ {
                ffi::gst_mini_object_unref((*info).data as *mut _);
                (*info).data = event.into_ptr() as *mut libc::c_void;
            }
        }
        None => {
            assert_ne!(data_type, Some(Query::static_type()));
            if !(*info).data.is_null() {
                ffi::gst_mini_object_unref((*info).data as *mut _);
                (*info).data = ptr::null_mut();
            }
        }
        _ => (),
    }

    ret
}

unsafe extern "C" fn trampoline_activate_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
) -> glib_ffi::gboolean {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>) -> bool + Send + Sync + 'static) =
        transmute((*pad).activatedata);

    func(&from_glib_borrow(pad), &from_glib_borrow(parent)).to_glib()
}

unsafe extern "C" fn trampoline_activatemode_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    mode: ffi::GstPadMode,
    active: glib_ffi::gboolean,
) -> glib_ffi::gboolean {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>, ::PadMode, bool) -> bool
                         + Send
                         + Sync
                         + 'static) = transmute((*pad).activatemodedata);

    func(
        &from_glib_borrow(pad),
        &from_glib_borrow(parent),
        from_glib(mode),
        from_glib(active),
    ).to_glib()
}

unsafe extern "C" fn trampoline_chain_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    buffer: *mut ffi::GstBuffer,
) -> ffi::GstFlowReturn {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>, ::Buffer) -> ::FlowReturn
                         + Send
                         + Sync
                         + 'static) = transmute((*pad).chaindata);

    func(
        &from_glib_borrow(pad),
        &from_glib_borrow(parent),
        from_glib_full(buffer),
    ).to_glib()
}

unsafe extern "C" fn trampoline_chain_list_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    list: *mut ffi::GstBufferList,
) -> ffi::GstFlowReturn {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>, ::BufferList) -> ::FlowReturn
                         + Send
                         + Sync
                         + 'static) = transmute((*pad).chainlistdata);

    func(
        &from_glib_borrow(pad),
        &from_glib_borrow(parent),
        from_glib_full(list),
    ).to_glib()
}

unsafe extern "C" fn trampoline_event_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    event: *mut ffi::GstEvent,
) -> glib_ffi::gboolean {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>, ::Event) -> bool + Send + Sync + 'static) =
        transmute((*pad).eventdata);

    func(
        &from_glib_borrow(pad),
        &from_glib_borrow(parent),
        from_glib_full(event),
    ).to_glib()
}

unsafe extern "C" fn trampoline_event_full_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    event: *mut ffi::GstEvent,
) -> ffi::GstFlowReturn {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>, ::Event) -> ::FlowReturn
                         + Send
                         + Sync
                         + 'static) = transmute((*pad).eventdata);

    func(
        &from_glib_borrow(pad),
        &from_glib_borrow(parent),
        from_glib_full(event),
    ).to_glib()
}

unsafe extern "C" fn trampoline_getrange_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    offset: u64,
    length: u32,
    buffer: *mut *mut ffi::GstBuffer,
) -> ffi::GstFlowReturn {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>, u64, u32) -> Result<::Buffer, ::FlowReturn>
                         + Send
                         + Sync
                         + 'static) = transmute((*pad).getrangedata);

    match func(
        &from_glib_borrow(pad),
        &from_glib_borrow(parent),
        offset,
        length,
    ) {
        Ok(new_buffer) => {
            *buffer = new_buffer.into_ptr();
            ::FlowReturn::Ok.to_glib()
        }
        Err(ret) => ret.to_glib(),
    }
}

unsafe extern "C" fn trampoline_iterate_internal_links_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
) -> *mut ffi::GstIterator {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>) -> ::Iterator<Pad> + Send + Sync + 'static) =
        transmute((*pad).iterintlinkdata);

    // Steal the iterator and return it
    let ret = func(&from_glib_borrow(pad), &from_glib_borrow(parent));
    let ptr = ret.to_glib_none().0;
    mem::forget(ret);

    ptr as *mut _
}

unsafe extern "C" fn trampoline_link_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    peer: *mut ffi::GstPad,
) -> ffi::GstPadLinkReturn {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>, &::Pad) -> ::PadLinkReturn
                         + Send
                         + Sync
                         + 'static) = transmute((*pad).linkdata);

    func(
        &from_glib_borrow(pad),
        &from_glib_borrow(parent),
        &from_glib_borrow(peer),
    ).to_glib()
}

unsafe extern "C" fn trampoline_query_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
    query: *mut ffi::GstQuery,
) -> glib_ffi::gboolean {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>, &mut ::QueryRef) -> bool
                         + Send
                         + Sync
                         + 'static) = transmute((*pad).querydata);

    func(
        &from_glib_borrow(pad),
        &from_glib_borrow(parent),
        ::QueryRef::from_mut_ptr(query),
    ).to_glib()
}

unsafe extern "C" fn trampoline_unlink_function(
    pad: *mut ffi::GstPad,
    parent: *mut ffi::GstObject,
) {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(&Pad, &Option<::Object>) + Send + Sync + 'static) =
        transmute((*pad).unlinkdata);

    func(&from_glib_borrow(pad), &from_glib_borrow(parent))
}

unsafe extern "C" fn destroy_closure(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<Box<Fn()>>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn trampoline_pad_task(func: gpointer) {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &RefCell<Box<FnMut() + Send + 'static>> = transmute(func);
    (&mut *func.borrow_mut())()
}

unsafe extern "C" fn destroy_closure_pad_task(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<RefCell<Box<FnMut() + Send + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw_pad_task<F: FnMut() + Send + 'static>(func: F) -> gpointer {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    let func: Box<RefCell<Box<FnMut() + Send + 'static>>> = Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::channel;

    #[test]
    fn test_event_chain_functions() {
        ::init().unwrap();

        let pad = ::Pad::new("sink", ::PadDirection::Sink);

        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        pad.set_event_function(move |_, _, event| {
            let mut events = events_clone.lock().unwrap();
            events.push(event);

            true
        });

        let buffers = Arc::new(Mutex::new(Vec::new()));
        let buffers_clone = buffers.clone();
        pad.set_chain_function(move |_, _, buffer| {
            let mut buffers = buffers_clone.lock().unwrap();
            buffers.push(buffer);

            ::FlowReturn::Ok
        });

        pad.set_active(true).unwrap();

        assert!(pad.send_event(::Event::new_stream_start("test").build()));
        let mut segment = ::Segment::default();
        segment.init(::Format::Time);
        assert!(pad.send_event(::Event::new_segment(&segment).build()));

        assert_eq!(pad.chain(::Buffer::new()), ::FlowReturn::Ok);

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
    fn test_task() {
        ::init().unwrap();

        let pad = ::Pad::new("sink", ::PadDirection::Sink);
        let (sender, receiver) = channel();

        let mut i = 0;
        let pad_clone = pad.clone();
        pad.start_task(move || {
            i += 1;
            if i == 3 {
                sender.send(i).unwrap();
                pad_clone.pause_task().unwrap();
            }
        }).unwrap();

        assert_eq!(receiver.recv().unwrap(), 3);
    }
}
