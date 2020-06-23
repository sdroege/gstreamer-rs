// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use futures_channel::mpsc::{self, UnboundedReceiver};
use futures_core::Stream;
use futures_util::{future, StreamExt};
use glib;
use glib::prelude::*;
use glib::source::{Continue, Priority, SourceId};
use glib::translate::*;
use glib_sys;
use glib_sys::{gboolean, gpointer};
use gst_sys;
use std::cell::RefCell;
use std::mem::transmute;
use std::pin::Pin;
use std::task::{Context, Poll};

use Bus;
use BusSyncReply;
use Message;
use MessageType;

unsafe extern "C" fn trampoline_watch<F: FnMut(&Bus, &Message) -> Continue + 'static>(
    bus: *mut gst_sys::GstBus,
    msg: *mut gst_sys::GstMessage,
    func: gpointer,
) -> gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (&mut *func.borrow_mut())(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).to_glib()
}

unsafe extern "C" fn destroy_closure_watch<F: FnMut(&Bus, &Message) -> Continue + 'static>(
    ptr: gpointer,
) {
    Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

fn into_raw_watch<F: FnMut(&Bus, &Message) -> Continue + 'static>(func: F) -> gpointer {
    #[allow(clippy::type_complexity)]
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline_sync<
    F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
>(
    bus: *mut gst_sys::GstBus,
    msg: *mut gst_sys::GstMessage,
    func: gpointer,
) -> gst_sys::GstBusSyncReply {
    let f: &F = &*(func as *const F);
    let res = f(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).to_glib();

    if res == gst_sys::GST_BUS_DROP {
        gst_sys::gst_mini_object_unref(msg as *mut _);
    }

    res
}

unsafe extern "C" fn destroy_closure_sync<
    F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
>(
    ptr: gpointer,
) {
    Box::<F>::from_raw(ptr as *mut _);
}

fn into_raw_sync<F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>(
    func: F,
) -> gpointer {
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as gpointer
}

impl Bus {
    pub fn add_signal_watch_full(&self, priority: Priority) {
        unsafe {
            gst_sys::gst_bus_add_signal_watch_full(self.to_glib_none().0, priority.to_glib());
        }
    }

    pub fn create_watch<F>(&self, name: Option<&str>, priority: Priority, func: F) -> glib::Source
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let source = gst_sys::gst_bus_create_watch(self.to_glib_none().0);
            glib_sys::g_source_set_callback(
                source,
                Some(transmute::<
                    _,
                    unsafe extern "C" fn(glib_sys::gpointer) -> i32,
                >(trampoline_watch::<F> as *const ())),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );
            glib_sys::g_source_set_priority(source, priority.to_glib());

            if let Some(name) = name {
                glib_sys::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    pub fn add_watch<F>(&self, func: F) -> Result<SourceId, glib::BoolError>
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        unsafe {
            let res = gst_sys::gst_bus_add_watch_full(
                self.to_glib_none().0,
                glib_sys::G_PRIORITY_DEFAULT,
                Some(trampoline_watch::<F>),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );

            if res == 0 {
                Err(glib_bool_error!("Bus already has a watch"))
            } else {
                Ok(from_glib(res))
            }
        }
    }

    pub fn add_watch_local<F>(&self, func: F) -> Result<SourceId, glib::BoolError>
    where
        F: FnMut(&Bus, &Message) -> Continue + 'static,
    {
        unsafe {
            assert!(glib::MainContext::ref_thread_default().is_owner());

            let res = gst_sys::gst_bus_add_watch_full(
                self.to_glib_none().0,
                glib_sys::G_PRIORITY_DEFAULT,
                Some(trampoline_watch::<F>),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );

            if res == 0 {
                Err(glib_bool_error!("Bus already has a watch"))
            } else {
                Ok(from_glib(res))
            }
        }
    }

    pub fn set_sync_handler<F>(&self, func: F)
    where
        F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
    {
        use once_cell::sync::Lazy;
        static SET_ONCE_QUARK: Lazy<glib::Quark> =
            Lazy::new(|| glib::Quark::from_string("gstreamer-rs-sync-handler"));

        unsafe {
            let bus = self.to_glib_none().0;

            // This is not thread-safe before 1.16.3, see
            // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/merge_requests/416
            if ::version() < (1, 16, 3, 0) {
                if !gobject_sys::g_object_get_qdata(bus as *mut _, SET_ONCE_QUARK.to_glib())
                    .is_null()
                {
                    panic!("Bus sync handler can only be set once");
                }

                gobject_sys::g_object_set_qdata(
                    bus as *mut _,
                    SET_ONCE_QUARK.to_glib(),
                    1 as *mut _,
                );
            }

            gst_sys::gst_bus_set_sync_handler(
                bus,
                Some(trampoline_sync::<F>),
                into_raw_sync(func),
                Some(destroy_closure_sync::<F>),
            )
        }
    }

    pub fn unset_sync_handler(&self) {
        // This is not thread-safe before 1.16.3, see
        // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/merge_requests/416
        if ::version() < (1, 16, 3, 0) {
            return;
        }

        unsafe {
            use std::ptr;

            gst_sys::gst_bus_set_sync_handler(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }

    pub fn iter(&self) -> Iter {
        self.iter_timed(0.into())
    }

    pub fn iter_timed(&self, timeout: ::ClockTime) -> Iter {
        Iter { bus: self, timeout }
    }

    pub fn iter_filtered<'a>(
        &'a self,
        msg_types: &'a [MessageType],
    ) -> impl Iterator<Item = Message> + 'a {
        self.iter_timed_filtered(0.into(), msg_types)
    }

    pub fn iter_timed_filtered<'a>(
        &'a self,
        timeout: ::ClockTime,
        msg_types: &'a [MessageType],
    ) -> impl Iterator<Item = Message> + 'a {
        self.iter_timed(timeout)
            .filter(move |msg| msg_types.contains(&msg.get_type()))
    }

    pub fn timed_pop_filtered(
        &self,
        timeout: ::ClockTime,
        msg_types: &[MessageType],
    ) -> Option<Message> {
        loop {
            let msg = self.timed_pop(timeout)?;
            if msg_types.contains(&msg.get_type()) {
                return Some(msg);
            }
        }
    }

    pub fn pop_filtered(&self, msg_types: &[MessageType]) -> Option<Message> {
        loop {
            let msg = self.pop()?;
            if msg_types.contains(&msg.get_type()) {
                return Some(msg);
            }
        }
    }

    pub fn stream(&self) -> BusStream {
        BusStream::new(self)
    }

    pub fn stream_filtered<'a>(
        &self,
        message_types: &'a [MessageType],
    ) -> impl Stream<Item = Message> + Unpin + Send + 'a {
        self.stream().filter(move |message| {
            let message_type = message.get_type();

            future::ready(message_types.contains(&message_type))
        })
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    bus: &'a Bus,
    timeout: ::ClockTime,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Message;

    fn next(&mut self) -> Option<Message> {
        self.bus.timed_pop(self.timeout)
    }
}

#[derive(Debug)]
pub struct BusStream {
    bus: glib::WeakRef<Bus>,
    receiver: UnboundedReceiver<Message>,
}

impl BusStream {
    fn new(bus: &Bus) -> Self {
        skip_assert_initialized!();

        let (sender, receiver) = mpsc::unbounded();

        bus.set_sync_handler(move |_, message| {
            let _ = sender.unbounded_send(message.to_owned());

            BusSyncReply::Drop
        });

        Self {
            bus: bus.downgrade(),
            receiver,
        }
    }
}

impl Drop for BusStream {
    fn drop(&mut self) {
        if let Some(bus) = self.bus.upgrade() {
            bus.unset_sync_handler();
        }
    }
}

impl Stream for BusStream {
    type Item = Message;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        self.receiver.poll_next_unpin(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_sync_handler() {
        ::init().unwrap();

        let bus = Bus::new();
        let msgs = Arc::new(Mutex::new(Vec::new()));
        let msgs_clone = msgs.clone();
        bus.set_sync_handler(move |_, msg| {
            msgs_clone.lock().unwrap().push(msg.clone());
            BusSyncReply::Pass
        });

        bus.post(&::message::Eos::new()).unwrap();

        let msgs = msgs.lock().unwrap();
        assert_eq!(msgs.len(), 1);
        match msgs[0].view() {
            ::MessageView::Eos(_) => (),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_bus_stream() {
        ::init().unwrap();

        let bus = Bus::new();
        let bus_stream = bus.stream();

        let eos_message = ::message::Eos::new();
        bus.post(&eos_message).unwrap();

        let bus_future = bus_stream.into_future();
        let (message, _) = futures_executor::block_on(bus_future);

        match message.unwrap().view() {
            ::MessageView::Eos(_) => (),
            _ => unreachable!(),
        }
    }
}
