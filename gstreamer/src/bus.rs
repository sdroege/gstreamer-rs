// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    future,
    mem::transmute,
    pin::Pin,
    task::{Context, Poll},
};

use futures_channel::mpsc::{self, UnboundedReceiver};
use futures_core::Stream;
use futures_util::{stream::FusedStream, StreamExt};
use glib::{
    ffi::{gboolean, gpointer},
    prelude::*,
    source::{Continue, Priority, SourceId},
    translate::*,
};

use crate::{Bus, BusSyncReply, Message, MessageType};

unsafe extern "C" fn trampoline_watch<F: FnMut(&Bus, &Message) -> Continue + Send + 'static>(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> gboolean {
    let func: &mut F = &mut *(func as *mut F);
    func(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).into_glib()
}

unsafe extern "C" fn destroy_closure_watch<
    F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
>(
    ptr: gpointer,
) {
    let _ = Box::<F>::from_raw(ptr as *mut _);
}

fn into_raw_watch<F: FnMut(&Bus, &Message) -> Continue + Send + 'static>(func: F) -> gpointer {
    #[allow(clippy::type_complexity)]
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline_watch_local<F: FnMut(&Bus, &Message) -> Continue + 'static>(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> gboolean {
    let func: &mut glib::thread_guard::ThreadGuard<F> =
        &mut *(func as *mut glib::thread_guard::ThreadGuard<F>);
    (func.get_mut())(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).into_glib()
}

unsafe extern "C" fn destroy_closure_watch_local<F: FnMut(&Bus, &Message) -> Continue + 'static>(
    ptr: gpointer,
) {
    let _ = Box::<glib::thread_guard::ThreadGuard<F>>::from_raw(ptr as *mut _);
}

fn into_raw_watch_local<F: FnMut(&Bus, &Message) -> Continue + 'static>(func: F) -> gpointer {
    #[allow(clippy::type_complexity)]
    let func: Box<glib::thread_guard::ThreadGuard<F>> =
        Box::new(glib::thread_guard::ThreadGuard::new(func));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline_sync<
    F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
>(
    bus: *mut ffi::GstBus,
    msg: *mut ffi::GstMessage,
    func: gpointer,
) -> ffi::GstBusSyncReply {
    let f: &F = &*(func as *const F);
    let res = f(&from_glib_borrow(bus), &Message::from_glib_borrow(msg)).into_glib();

    if res == ffi::GST_BUS_DROP {
        ffi::gst_mini_object_unref(msg as *mut _);
    }

    res
}

unsafe extern "C" fn destroy_closure_sync<
    F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
>(
    ptr: gpointer,
) {
    let _ = Box::<F>::from_raw(ptr as *mut _);
}

fn into_raw_sync<F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static>(
    func: F,
) -> gpointer {
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as gpointer
}

impl Bus {
    #[doc(alias = "gst_bus_add_signal_watch")]
    #[doc(alias = "gst_bus_add_signal_watch_full")]
    pub fn add_signal_watch_full(&self, priority: Priority) {
        unsafe {
            ffi::gst_bus_add_signal_watch_full(self.to_glib_none().0, priority.into_glib());
        }
    }

    #[doc(alias = "gst_bus_create_watch")]
    pub fn create_watch<F>(&self, name: Option<&str>, priority: Priority, func: F) -> glib::Source
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let source = ffi::gst_bus_create_watch(self.to_glib_none().0);
            glib::ffi::g_source_set_callback(
                source,
                Some(transmute::<
                    _,
                    unsafe extern "C" fn(glib::ffi::gpointer) -> i32,
                >(trampoline_watch::<F> as *const ())),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );
            glib::ffi::g_source_set_priority(source, priority.into_glib());

            if let Some(name) = name {
                glib::ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    #[doc(alias = "gst_bus_add_watch")]
    #[doc(alias = "gst_bus_add_watch_full")]
    pub fn add_watch<F>(&self, func: F) -> Result<SourceId, glib::BoolError>
    where
        F: FnMut(&Bus, &Message) -> Continue + Send + 'static,
    {
        unsafe {
            let res = ffi::gst_bus_add_watch_full(
                self.to_glib_none().0,
                glib::ffi::G_PRIORITY_DEFAULT,
                Some(trampoline_watch::<F>),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );

            if res == 0 {
                Err(glib::bool_error!("Bus already has a watch"))
            } else {
                Ok(from_glib(res))
            }
        }
    }

    #[doc(alias = "gst_bus_add_watch")]
    #[doc(alias = "gst_bus_add_watch_full")]
    pub fn add_watch_local<F>(&self, func: F) -> Result<SourceId, glib::BoolError>
    where
        F: FnMut(&Bus, &Message) -> Continue + 'static,
    {
        unsafe {
            let ctx = glib::MainContext::ref_thread_default();
            let _acquire = ctx
                .acquire()
                .expect("thread default main context already acquired by another thread");

            let res = ffi::gst_bus_add_watch_full(
                self.to_glib_none().0,
                glib::ffi::G_PRIORITY_DEFAULT,
                Some(trampoline_watch_local::<F>),
                into_raw_watch_local(func),
                Some(destroy_closure_watch_local::<F>),
            );

            if res == 0 {
                Err(glib::bool_error!("Bus already has a watch"))
            } else {
                Ok(from_glib(res))
            }
        }
    }

    #[doc(alias = "gst_bus_set_sync_handler")]
    pub fn set_sync_handler<F>(&self, func: F)
    where
        F: Fn(&Bus, &Message) -> BusSyncReply + Send + Sync + 'static,
    {
        #[cfg(not(feature = "v1_18"))]
        use once_cell::sync::Lazy;
        #[cfg(not(feature = "v1_18"))]
        static SET_ONCE_QUARK: Lazy<glib::Quark> =
            Lazy::new(|| glib::Quark::from_str("gstreamer-rs-sync-handler"));

        unsafe {
            let bus = self.to_glib_none().0;

            #[cfg(not(feature = "v1_18"))]
            {
                // This is not thread-safe before 1.16.3, see
                // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/merge_requests/416
                if crate::version() < (1, 16, 3, 0) {
                    if !glib::gobject_ffi::g_object_get_qdata(
                        bus as *mut _,
                        SET_ONCE_QUARK.into_glib(),
                    )
                    .is_null()
                    {
                        panic!("Bus sync handler can only be set once");
                    }

                    glib::gobject_ffi::g_object_set_qdata(
                        bus as *mut _,
                        SET_ONCE_QUARK.into_glib(),
                        1 as *mut _,
                    );
                }
            }

            ffi::gst_bus_set_sync_handler(
                bus,
                Some(trampoline_sync::<F>),
                into_raw_sync(func),
                Some(destroy_closure_sync::<F>),
            )
        }
    }

    pub fn unset_sync_handler(&self) {
        #[cfg(not(feature = "v1_18"))]
        {
            // This is not thread-safe before 1.16.3, see
            // https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/merge_requests/416
            if crate::version() < (1, 16, 3, 0) {
                return;
            }
        }

        unsafe {
            use std::ptr;

            ffi::gst_bus_set_sync_handler(self.to_glib_none().0, None, ptr::null_mut(), None)
        }
    }

    #[doc(alias = "gst_bus_pop")]
    pub fn iter(&self) -> Iter {
        self.iter_timed(Some(crate::ClockTime::ZERO))
    }

    #[doc(alias = "gst_bus_timed_pop")]
    pub fn iter_timed(&self, timeout: impl Into<Option<crate::ClockTime>>) -> Iter {
        Iter {
            bus: self,
            timeout: timeout.into(),
        }
    }

    #[doc(alias = "gst_bus_pop_filtered")]
    pub fn iter_filtered<'a>(
        &'a self,
        msg_types: &'a [MessageType],
    ) -> impl Iterator<Item = Message> + 'a {
        self.iter_timed_filtered(Some(crate::ClockTime::ZERO), msg_types)
    }

    #[doc(alias = "gst_bus_timed_pop_filtered")]
    pub fn iter_timed_filtered<'a>(
        &'a self,
        timeout: impl Into<Option<crate::ClockTime>>,
        msg_types: &'a [MessageType],
    ) -> impl Iterator<Item = Message> + 'a {
        self.iter_timed(timeout)
            .filter(move |msg| msg_types.contains(&msg.type_()))
    }

    #[doc(alias = "gst_bus_timed_pop_filtered")]
    pub fn timed_pop_filtered(
        &self,
        timeout: impl Into<Option<crate::ClockTime>> + Clone,
        msg_types: &[MessageType],
    ) -> Option<Message> {
        loop {
            let msg = self.timed_pop(timeout.clone())?;
            if msg_types.contains(&msg.type_()) {
                return Some(msg);
            }
        }
    }

    #[doc(alias = "gst_bus_pop_filtered")]
    pub fn pop_filtered(&self, msg_types: &[MessageType]) -> Option<Message> {
        loop {
            let msg = self.pop()?;
            if msg_types.contains(&msg.type_()) {
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
    ) -> impl Stream<Item = Message> + Unpin + FusedStream + Send + 'a {
        self.stream().filter(move |message| {
            let message_type = message.type_();

            future::ready(message_types.contains(&message_type))
        })
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    bus: &'a Bus,
    timeout: Option<crate::ClockTime>,
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

impl FusedStream for BusStream {
    fn is_terminated(&self) -> bool {
        self.receiver.is_terminated()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn test_sync_handler() {
        crate::init().unwrap();

        let bus = Bus::new();
        let msgs = Arc::new(Mutex::new(Vec::new()));
        let msgs_clone = msgs.clone();
        bus.set_sync_handler(move |_, msg| {
            msgs_clone.lock().unwrap().push(msg.clone());
            BusSyncReply::Pass
        });

        bus.post(crate::message::Eos::new()).unwrap();

        let msgs = msgs.lock().unwrap();
        assert_eq!(msgs.len(), 1);
        match msgs[0].view() {
            crate::MessageView::Eos(_) => (),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_bus_stream() {
        crate::init().unwrap();

        let bus = Bus::new();
        let bus_stream = bus.stream();

        let eos_message = crate::message::Eos::new();
        bus.post(eos_message).unwrap();

        let bus_future = bus_stream.into_future();
        let (message, _) = futures_executor::block_on(bus_future);

        match message.unwrap().view() {
            crate::MessageView::Eos(_) => (),
            _ => unreachable!(),
        }
    }
}
