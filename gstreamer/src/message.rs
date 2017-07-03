// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use Object;
use MessageType;
use miniobject::*;
use std::ffi::CStr;

use glib;
use glib::translate::{from_glib, from_glib_none, from_glib_full, ToGlibPtr, ToGlib};

#[repr(C)]
pub struct MessageImpl(ffi::GstMessage);

pub type Message = GstRc<MessageImpl>;

unsafe impl MiniObject for MessageImpl {
    type GstType = ffi::GstMessage;
}

impl MessageImpl {
    pub fn new_eos(src: &Object) -> GstRc<Self> {
        unsafe {
            from_glib_full(ffi::gst_message_new_eos(src.to_glib_none().0))
        }
    }

    pub fn get_src(&self) -> Object {
        unsafe {
            from_glib_none((*self.as_ptr()).src)
        }
    }

    pub fn get_message_type(&self) -> MessageType {
        unsafe {
            from_glib((*self.as_ptr()).type_)
        }
    }

    pub fn get_message_type_name(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(ffi::gst_message_type_get_name(self.get_message_type().to_glib())).to_str().unwrap()
        }
    }

    pub fn get_seqnum(&self) -> u32 {
        unsafe {
            ffi::gst_message_get_seqnum(self.as_mut_ptr())
        }
    }

    pub fn set_seqnum(&mut self, seqnum: u32) {
        unsafe {
            ffi::gst_message_set_seqnum(self.as_mut_ptr(), seqnum)
        }
    }

    // TODO get_structure(), get_mut_structure()
}

impl glib::types::StaticType for GstRc<MessageImpl> {
    fn static_type() -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_message_get_type())
        }
    }
}
