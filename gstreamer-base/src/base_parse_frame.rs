// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst;
use gst_base_sys;
use std::convert::TryFrom;
use std::fmt;
use std::marker::PhantomData;
use std::ptr;

use BaseParse;
use BaseParseFrameFlags;

pub struct BaseParseFrame<'a>(
    ptr::NonNull<gst_base_sys::GstBaseParseFrame>,
    PhantomData<&'a BaseParse>,
);

unsafe impl<'a> Send for BaseParseFrame<'a> {}
unsafe impl<'a> Sync for BaseParseFrame<'a> {}

#[derive(Debug)]
pub enum Overhead {
    None,
    Frame,
    Bytes(u32),
}

#[doc(hidden)]
impl ToGlib for Overhead {
    type GlibType = i32;

    fn to_glib(&self) -> i32 {
        match *self {
            Overhead::None => 0,
            Overhead::Frame => -1,
            Overhead::Bytes(b) => i32::try_from(b).expect("overhead is higher than i32::MAX"),
        }
    }
}

impl FromGlib<i32> for Overhead {
    #[inline]
    fn from_glib(val: i32) -> Overhead {
        skip_assert_initialized!();
        match val {
            0 => Overhead::None,
            1 => Overhead::Frame,
            b if b > 0 => Overhead::Bytes(val as u32),
            _ => panic!("overheader is lower than -1"),
        }
    }
}

#[doc(hidden)]
impl<'a> ::glib::translate::ToGlibPtr<'a, *mut gst_base_sys::GstBaseParseFrame>
    for BaseParseFrame<'a>
{
    type Storage = &'a Self;

    fn to_glib_none(
        &'a self,
    ) -> ::glib::translate::Stash<*mut gst_base_sys::GstBaseParseFrame, Self> {
        Stash(self.0.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *mut gst_base_sys::GstBaseParseFrame {
        unimplemented!()
    }
}

impl<'a> fmt::Debug for BaseParseFrame<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut b = f.debug_struct("BaseParseFrame");

        b.field("buffer", &self.get_buffer())
            .field("output_buffer", &self.get_output_buffer())
            .field("flags", &self.get_flags())
            .field("offset", &self.get_offset())
            .field("overhead", &self.get_overhead());

        b.finish()
    }
}

impl<'a> BaseParseFrame<'a> {
    pub(crate) unsafe fn new(
        frame: *mut gst_base_sys::GstBaseParseFrame,
        _parse: &'a BaseParse,
    ) -> BaseParseFrame<'a> {
        skip_assert_initialized!();
        assert!(!frame.is_null());
        BaseParseFrame(ptr::NonNull::new_unchecked(frame), PhantomData)
    }

    pub fn get_buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).buffer;
            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_buffer_mut(&mut self) -> Option<&mut gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).buffer;
            if ptr.is_null() {
                None
            } else {
                let writable: bool = from_glib(gst_sys::gst_mini_object_is_writable(
                    ptr as *const gst_sys::GstMiniObject,
                ));
                assert!(writable);

                Some(gst::BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn get_output_buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).out_buffer;
            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_output_buffer_mut(&mut self) -> Option<&mut gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).out_buffer;
            if ptr.is_null() {
                None
            } else {
                let writable: bool = from_glib(gst_sys::gst_mini_object_is_writable(
                    ptr as *const gst_sys::GstMiniObject,
                ));
                assert!(writable);

                Some(gst::BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn set_output_buffer(&mut self, output_buffer: gst::Buffer) {
        unsafe {
            let prev = (*self.to_glib_none().0).out_buffer;

            if !prev.is_null() {
                gst_sys::gst_mini_object_unref(prev as *mut gst_sys::GstMiniObject);
            }

            let ptr = output_buffer.into_ptr();
            let writable: bool = from_glib(gst_sys::gst_mini_object_is_writable(
                ptr as *const gst_sys::GstMiniObject,
            ));
            assert!(writable);

            (*self.to_glib_none().0).out_buffer = ptr;
        }
    }

    pub fn get_flags(&self) -> BaseParseFrameFlags {
        let flags = unsafe { (*self.to_glib_none().0).flags };
        BaseParseFrameFlags::from_bits_truncate(flags)
    }

    pub fn set_flags(&mut self, flags: BaseParseFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags |= flags.bits() }
    }

    pub fn unset_flags(&mut self, flags: BaseParseFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags &= !flags.bits() }
    }

    pub fn get_offset(&self) -> u64 {
        unsafe { (*self.to_glib_none().0).offset }
    }

    pub fn get_overhead(&self) -> Overhead {
        unsafe { from_glib((*self.to_glib_none().0).overhead) }
    }

    pub fn set_overhead(&mut self, overhead: Overhead) {
        unsafe {
            (*self.to_glib_none().0).overhead = overhead.to_glib();
        }
    }
}
