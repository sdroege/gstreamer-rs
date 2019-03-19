// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use glib::IsA;
use gst_gl_sys;
use libc::uintptr_t;
use GLContext;
use GLDisplay;
use GLPlatform;
use GLAPI;

impl GLContext {
    pub unsafe fn new_wrapped<T: IsA<GLDisplay>>(
        display: &T,
        handle: uintptr_t,
        context_type: GLPlatform,
        available_apis: GLAPI,
    ) -> Option<GLContext> {
        from_glib_full(gst_gl_sys::gst_gl_context_new_wrapped(
            display.as_ref().to_glib_none().0,
            handle,
            context_type.to_glib(),
            available_apis.to_glib(),
        ))
    }

    pub fn get_current_gl_context(context_type: GLPlatform) -> uintptr_t {
        unsafe {
            gst_gl_sys::gst_gl_context_get_current_gl_context(context_type.to_glib()) as uintptr_t
        }
    }

    pub fn get_proc_address_with_platform(
        context_type: GLPlatform,
        gl_api: GLAPI,
        name: &str,
    ) -> uintptr_t {
        unsafe {
            gst_gl_sys::gst_gl_context_get_proc_address_with_platform(
                context_type.to_glib(),
                gl_api.to_glib(),
                name.to_glib_none().0,
            ) as uintptr_t
        }
    }
}

pub trait GLContextExtManual: 'static {
    fn get_gl_context(&self) -> uintptr_t;

    fn get_proc_address(&self, name: &str) -> uintptr_t;
}

impl<O: IsA<GLContext>> GLContextExtManual for O {
    fn get_gl_context(&self) -> uintptr_t {
        unsafe {
            gst_gl_sys::gst_gl_context_get_gl_context(self.as_ref().to_glib_none().0) as uintptr_t
        }
    }

    fn get_proc_address(&self, name: &str) -> uintptr_t {
        unsafe {
            gst_gl_sys::gst_gl_context_get_proc_address(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ) as uintptr_t
        }
    }
}
