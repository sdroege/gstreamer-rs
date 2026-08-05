// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures_channel::mpsc::{self, UnboundedReceiver};
use futures_core::Stream;
use futures_util::{StreamExt, stream::FusedStream};
use glib::object::ObjectExt as _;

use crate::{Bus, BusSyncReply, Message};

#[derive(Debug)]
pub struct BusStream {
    bus: glib::WeakRef<Bus>,
    receiver: UnboundedReceiver<Message>,
}

impl BusStream {
    pub(crate) fn new(bus: &Bus) -> Self {
        skip_assert_initialized!();

        let mutex = Arc::new(Mutex::new(()));
        let (sender, receiver) = mpsc::unbounded();

        // Use a mutex to ensure that the sync handler is not putting any messages into the sender
        // until we have removed all previously queued messages from the bus.
        // This makes sure that the messages are staying in order.
        //
        // We could use the bus' object lock here but a separate mutex seems safer.
        let _mutex_guard = mutex.lock().unwrap();
        bus.set_sync_handler({
            let sender = sender.clone();
            let mutex = mutex.clone();

            move |_bus, message| {
                let _mutex_guard = mutex.lock().unwrap();

                let _ = sender.unbounded_send(message.to_owned());

                BusSyncReply::Drop
            }
        });

        // First pop all messages that might've been previously queued before creating the bus stream.
        while let Some(message) = bus.pop() {
            let _ = sender.unbounded_send(message);
        }

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
    use super::*;

    #[test]
    fn test_bus_stream() {
        crate::init().unwrap();

        let bus = Bus::new();
        let bus_stream = bus.stream();

        let eos_message = crate::message::Eos::new();
        bus.post(eos_message).unwrap();

        let bus_future = StreamExt::into_future(bus_stream);
        let (message, _) = futures_executor::block_on(bus_future);

        match message.unwrap().view() {
            crate::MessageView::Eos(_) => (),
            _ => unreachable!(),
        }
    }
}
