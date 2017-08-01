extern crate gstreamer as gst;
use gst::*;

extern crate futures;
use futures::{Async, Poll};
use futures::task::Task;
use futures::stream::Stream;
extern crate tokio_core;
use tokio_core::reactor::Core;

use std::env;
use std::sync::{Arc, Mutex};

struct BusStream(Bus, Arc<Mutex<Option<Task>>>);

impl BusStream {
    fn new(bus: &Bus) -> Self {
        let task = Arc::new(Mutex::new(None));
        let task_clone = task.clone();

        bus.set_sync_handler(move |_, _| {
            let mut task = task_clone.lock().unwrap();
            if let Some(task) = task.take() {
                // FIXME: Force type...
                let task: Task = task;
                task.notify();
            }

            BusSyncReply::Pass
        });
        BusStream(bus.clone(), task)
    }
}

impl Drop for BusStream {
    fn drop(&mut self) {
        self.0.unset_sync_handler();
    }
}

impl Stream for BusStream {
    type Item = Message;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut task = self.1.lock().unwrap();

        let msg = self.0.pop();
        if let Some(msg) = msg {
            Ok(Async::Ready(Some(msg)))
        } else {
            *task = Some(futures::task::current());
            Ok(Async::NotReady)
        }
    }
}

fn main() {
    let pipeline_str = env::args().collect::<Vec<String>>()[1..].join(" ");

    gst::init().unwrap();

    let mut core = Core::new().unwrap();

    let pipeline = gst::parse_launch(&pipeline_str).unwrap();
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let messages = BusStream::new(&bus).for_each(|msg| {
        let quit = match msg.view() {
            MessageView::Eos(..) => true,
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                true
            }
            _ => false,
        };

        if quit {
            Err(())
        } else {
            Ok(())
        }
    });

    let _ = core.run(messages);

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
