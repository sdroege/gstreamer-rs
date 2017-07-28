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
use FlowReturn;
use miniobject::MiniObject;

use std::cell::RefCell;
use std::mem::transmute;
use std::ptr;

use glib::{IsA, StaticType};
use glib::translate::{ToGlib, FromGlib, from_glib, from_glib_none, from_glib_borrow,
                      from_glib_full};
use glib::source::CallbackGuard;
use glib_ffi::gpointer;

use libc;

use ffi;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
        PadProbeId(val)
    }
}

pub struct PadProbeInfo {
    pub mask: PadProbeType,
    pub id: PadProbeId,
    pub offset: u64,
    pub size: u32,
    pub data: Option<PadProbeData>,
}

pub enum PadProbeData {
    Buffer(Buffer),
    BufferList(BufferList),
    // Query(&Query),
    // Event(&Event),
    Unknown,
}

pub trait PadExtManual {
    fn add_probe<F>(&self, mask: PadProbeType, func: F) -> PadProbeId
    where
        F: FnMut(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static;
    fn remove_probe(&self, id: PadProbeId);

    fn chain(&self, buffer: Buffer) -> FlowReturn;
    fn push(&self, buffer: Buffer) -> FlowReturn;

    fn pull_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowReturn>;
    fn get_range(&self, offset: u64, size: u32) -> Result<Buffer, FlowReturn>;
}

impl<O: IsA<Pad>> PadExtManual for O {
    fn add_probe<F>(&self, mask: PadProbeType, func: F) -> PadProbeId
    where
        F: FnMut(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
    {
        unsafe {
            let id = ffi::gst_pad_add_probe(
                self.to_glib_none().0,
                mask.to_glib(),
                Some(trampoline_pad_probe),
                into_raw_pad_probe(func),
                Some(destroy_closure_pad_probe),
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
}

unsafe extern "C" fn trampoline_pad_probe(
    pad: *mut ffi::GstPad,
    info: *mut ffi::GstPadProbeInfo,
    func: gpointer,
) -> ffi::GstPadProbeReturn {
    let _guard = CallbackGuard::new();
    let func: &RefCell<
        Box<FnMut(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static>,
    > = transmute(func);
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
                    from_glib_borrow(data as *const ffi::GstBuffer),
                ))
            } else if (*data).type_ == BufferList::static_type().to_glib() {
                data_type = Some(BufferList::static_type());
                Some(PadProbeData::BufferList(
                    from_glib_borrow(data as *const ffi::GstBufferList),
                ))
            } else {
                Some(PadProbeData::Unknown)
            }
        },
    };

    let ret = (&mut *func.borrow_mut())(&from_glib_none(pad), &mut probe_info).to_glib();

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
        None => {
            if !(*info).data.is_null() {
                ffi::gst_mini_object_unref((*info).data as *mut _);
                (*info).data = ptr::null_mut();
            }
        }
        _ => (),
    }

    ret
}

unsafe extern "C" fn destroy_closure_pad_probe(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<RefCell<Box<FnMut(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static>>>::from_raw(ptr as *mut _);
}

fn into_raw_pad_probe<
    F: FnMut(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static,
>(
    func: F,
) -> gpointer {
    let func: Box<
        RefCell<Box<FnMut(&Pad, &mut PadProbeInfo) -> PadProbeReturn + Send + Sync + 'static>>,
    > = Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}
