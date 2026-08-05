// Take a look at the license at the top of the repository in the LICENSE file.

use futures_core::Stream;
use glib::object::ObjectExt as _;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Waker,
    task::{Context, Poll},
};

use crate::{AppSink, AppSinkCallbacks};

#[derive(Debug)]
pub struct AppSinkStream {
    app_sink: glib::WeakRef<AppSink>,
    waker_reference: Arc<Mutex<Option<Waker>>>,
}

impl AppSinkStream {
    pub(crate) fn new(app_sink: &AppSink) -> Self {
        skip_assert_initialized!();

        let waker_reference = Arc::new(Mutex::new(None as Option<Waker>));

        app_sink.set_callbacks(
            AppSinkCallbacks::builder()
                .new_sample({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }

                        Ok(gst::FlowSuccess::Ok)
                    }
                })
                .eos({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }
                    }
                })
                .build(),
        );

        Self {
            app_sink: app_sink.downgrade(),
            waker_reference,
        }
    }
}

impl Drop for AppSinkStream {
    fn drop(&mut self) {
        #[cfg(not(feature = "v1_18"))]
        {
            // This is not thread-safe before 1.16.3, see
            // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/merge_requests/570
            if gst::version() >= (1, 16, 3, 0)
                && let Some(app_sink) = self.app_sink.upgrade()
            {
                app_sink.set_callbacks(AppSinkCallbacks::builder().build());
            }
        }
    }
}
impl Stream for AppSinkStream {
    type Item = gst::Sample;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        let mut waker = self.waker_reference.lock().unwrap();

        let Some(app_sink) = self.app_sink.upgrade() else {
            return Poll::Ready(None);
        };

        app_sink
            .try_pull_sample(gst::ClockTime::ZERO)
            .map(|sample| Poll::Ready(Some(sample)))
            .unwrap_or_else(|| {
                if app_sink.is_eos() {
                    return Poll::Ready(None);
                }

                waker.replace(context.waker().to_owned());

                Poll::Pending
            })
    }
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;
    use gst::prelude::*;

    use super::*;

    #[test]
    fn test_app_sink_stream() {
        gst::init().unwrap();

        let videotestsrc = gst::ElementFactory::make("videotestsrc")
            .property("num-buffers", 5)
            .build()
            .unwrap();
        let appsink = gst::ElementFactory::make("appsink").build().unwrap();

        let pipeline = gst::Pipeline::new();
        pipeline.add(&videotestsrc).unwrap();
        pipeline.add(&appsink).unwrap();

        videotestsrc.link(&appsink).unwrap();

        let app_sink_stream = appsink.dynamic_cast::<AppSink>().unwrap().stream();
        let samples_future = app_sink_stream.collect::<Vec<gst::Sample>>();

        pipeline.set_state(gst::State::Playing).unwrap();
        let samples = futures_executor::block_on(samples_future);
        pipeline.set_state(gst::State::Null).unwrap();

        assert_eq!(samples.len(), 5);
    }
}
