//! This example prepares a vp9 encoding pipeline, instrumented via tracing.
use gstreamer::{prelude::ElementExt, ClockTime, MessageView::*, State};

fn main() {
    tracing_subscriber::fmt::init();
    tracing_gstreamer::integrate_events();
    gstreamer::log::remove_default_log_function();
    gstreamer::log::set_default_threshold(gstreamer::DebugLevel::Memdump);
    gstreamer::init().expect("gst init");
    tracing_gstreamer::integrate_spans();

    let pipeline = gstreamer::parse::launch(
        r#"
        videotestsrc num-buffers=120
        ! vp9enc
        ! webmmux name=mux
        ! fakesink sync=false

        audiotestsrc num-buffers=120
        ! opusenc
        ! mux.
    "#,
    )
    .expect("construct the pipeline");
    let bus = pipeline.bus().expect("could not obtain the pipeline bus");
    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("could not start the pipeline");
    loop {
        let msg = match bus.timed_pop(ClockTime::NONE) {
            None => break,
            Some(msg) => msg,
        };
        tracing::info!(message = "bus message", bus_message = ?msg);
        match msg.view() {
            Eos(_) => break,
            Error(e) => break tracing::error!("{}", e.error()),
            Warning(w) => tracing::warn!("{:?}", w),
            _ => {}
        }
    }
    pipeline
        .set_state(State::Null)
        .expect("could not stop the pipeline");
}
