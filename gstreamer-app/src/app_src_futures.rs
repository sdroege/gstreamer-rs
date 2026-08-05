// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{AppSrc, AppSrcCallbacks};
use futures_sink::Sink;
use glib::object::ObjectExt as _;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Waker,
    task::{Context, Poll},
};

#[derive(Debug)]
pub struct AppSrcSink {
    app_src: glib::WeakRef<AppSrc>,
    waker_reference: Arc<Mutex<Option<Waker>>>,
}

impl AppSrcSink {
    pub(crate) fn new(app_src: &AppSrc) -> Self {
        skip_assert_initialized!();

        let waker_reference = Arc::new(Mutex::new(None as Option<Waker>));

        app_src.set_callbacks(
            AppSrcCallbacks::builder()
                .need_data({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_, _| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }
                    }
                })
                .build(),
        );

        Self {
            app_src: app_src.downgrade(),
            waker_reference,
        }
    }
}

impl Drop for AppSrcSink {
    fn drop(&mut self) {
        #[cfg(not(feature = "v1_18"))]
        {
            // This is not thread-safe before 1.16.3, see
            // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/merge_requests/570
            if gst::version() >= (1, 16, 3, 0)
                && let Some(app_src) = self.app_src.upgrade()
            {
                app_src.set_callbacks(AppSrcCallbacks::builder().build());
            }
        }
    }
}

impl Sink<gst::Sample> for AppSrcSink {
    type Error = gst::FlowError;

    fn poll_ready(self: Pin<&mut Self>, context: &mut Context) -> Poll<Result<(), Self::Error>> {
        let mut waker = self.waker_reference.lock().unwrap();

        let Some(app_src) = self.app_src.upgrade() else {
            return Poll::Ready(Err(gst::FlowError::Eos));
        };

        let current_level_bytes = app_src.current_level_bytes();
        let max_bytes = app_src.max_bytes();

        if current_level_bytes >= max_bytes && max_bytes != 0 {
            waker.replace(context.waker().to_owned());

            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn start_send(self: Pin<&mut Self>, sample: gst::Sample) -> Result<(), Self::Error> {
        let Some(app_src) = self.app_src.upgrade() else {
            return Err(gst::FlowError::Eos);
        };

        app_src.push_sample(&sample)?;

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        let Some(app_src) = self.app_src.upgrade() else {
            return Poll::Ready(Ok(()));
        };

        app_src.end_of_stream()?;

        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use futures_util::{sink::SinkExt, stream::StreamExt};
    use gst::prelude::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use super::*;

    #[test]
    fn test_app_src_sink() {
        gst::init().unwrap();

        let appsrc = gst::ElementFactory::make("appsrc").build().unwrap();
        let fakesink = gst::ElementFactory::make("fakesink")
            .property("signal-handoffs", true)
            .build()
            .unwrap();

        let pipeline = gst::Pipeline::new();
        pipeline.add(&appsrc).unwrap();
        pipeline.add(&fakesink).unwrap();

        appsrc.link(&fakesink).unwrap();

        let mut bus_stream = pipeline.bus().unwrap().stream();
        let mut app_src_sink = appsrc.dynamic_cast::<AppSrc>().unwrap().sink();

        let sample_quantity = 5;

        let samples = (0..sample_quantity)
            .map(|_| gst::Sample::builder().buffer(&gst::Buffer::new()).build())
            .collect::<Vec<gst::Sample>>();

        let mut sample_stream = futures_util::stream::iter(samples).map(Ok);

        let handoff_count_reference = Arc::new(AtomicUsize::new(0));

        fakesink.connect("handoff", false, {
            let handoff_count_reference = Arc::clone(&handoff_count_reference);

            move |_| {
                handoff_count_reference.fetch_add(1, Ordering::AcqRel);

                None
            }
        });

        pipeline.set_state(gst::State::Playing).unwrap();

        futures_executor::block_on(app_src_sink.send_all(&mut sample_stream)).unwrap();
        futures_executor::block_on(app_src_sink.close()).unwrap();

        while let Some(message) = futures_executor::block_on(bus_stream.next()) {
            match message.view() {
                gst::MessageView::Eos(_) => break,
                gst::MessageView::Error(_) => unreachable!(),
                _ => continue,
            }
        }

        pipeline.set_state(gst::State::Null).unwrap();

        assert_eq!(
            handoff_count_reference.load(Ordering::Acquire),
            sample_quantity
        );
    }
}
