// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::RefCell;
use std::mem::transmute;
use ffi;
use glib;
use glib::translate::*;
use glib::source::{CallbackGuard, Continue, Priority, SourceId};
use glib_ffi;
use glib_ffi::{gboolean, gpointer};
use std::ptr;

use Bus;
use BusSyncReply;
use Message;

unsafe extern "C" fn trampoline_watch(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> gboolean {
    let _guard = CallbackGuard::new();
    let func: &RefCell<Box<FnMut(&Bus, &Message) -> Continue + Send + 'static>> = transmute(func);
    (&mut *func.borrow_mut())(&from_glib_none(bus), &Message::from_glib_none(msg)).to_glib()
}

unsafe extern "C" fn destroy_closure_watch(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<RefCell<Box<FnMut(&Bus, &Message) -> Continue + Send + 'static>>>::from_raw(
        ptr as *mut _,
    );
}

fn into_raw_watch<F: FnMut(&Bus, &Message) -> Continue + Send + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<Box<FnMut(&Bus, &Message) -> Continue + Send + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline_sync(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> ffi::GstBusSyncReply {
    let _guard = CallbackGuard::new();
    let f: &&(Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static) = transmute(func);
    f(&from_glib_none(bus), &Message::from_glib_none(msg)).to_glib()
}

unsafe extern "C" fn destroy_closure_sync(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<Box<Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>>::from_raw(ptr as *mut _);
}

fn into_raw_sync<F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>(
    func: F,
) -> gpointer {
    let func: Box<Box<Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>> =
        Box::new(Box::new(func));
    Box::into_raw(func) as gpointer
}

impl Bus {
    pub fn create_watch<'a, N: Into<Option<&'a str>>, F>(
        &self,
        name: N,
        priority: Priority,
        func: F,
    ) -> Option<glib::Source>
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        unsafe {
            let source = ffi::gst_bus_create_watch(self.to_glib_none().0);
            let trampoline = trampoline_watch as gpointer;
            glib_ffi::g_source_set_callback(
                source,
                Some(transmute(trampoline)),
                into_raw_watch(func),
                Some(destroy_closure_watch),
            );
            glib_ffi::g_source_set_priority(source, priority.to_glib());

            let name = name.into();
            if let Some(name) = name {
                glib_ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    pub fn add_watch<F>(&self, func: F) -> SourceId
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        unsafe {
            from_glib(ffi::gst_bus_add_watch_full(
                self.to_glib_none().0,
                glib_ffi::G_PRIORITY_DEFAULT,
                Some(trampoline_watch),
                into_raw_watch(func),
                Some(destroy_closure_watch),
            ))
        }
    }

    pub fn set_sync_handler<F>(&self, func: F)
    where
        F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
    {
        unsafe {
            ffi::gst_bus_set_sync_handler(
                self.to_glib_none().0,
                Some(trampoline_sync),
                into_raw_sync(func),
                Some(destroy_closure_sync),
            )
        }
    }

    pub fn unset_sync_handler(&self) {
        unsafe { ffi::gst_bus_set_sync_handler(self.to_glib_none().0, None, ptr::null_mut(), None) }
    }
}
