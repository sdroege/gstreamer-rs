//! This example prepares a vp9 encoding pipeline, instrumented via tracing.

use gst::{prelude::*, ClockTime, MessageView, State};

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    tracing_subscriber::fmt::init();
    gst_tracing::integrate_events();
    gst::log::remove_default_log_function();
    gst::log::set_default_threshold(gst::DebugLevel::Memdump);
    gst::init().expect("gst init");
    // gst_tracing::integrate_spans() needs to be called after gst::init() because
    // it creates a gst::Tracer internally which requires gst to be initialized.
    gst_tracing::integrate_spans();

    let pipeline = gst::parse::launch(
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
        .set_state(State::Playing)
        .expect("could not start the pipeline");
    loop {
        let msg: gst::Message = match bus.timed_pop(ClockTime::NONE) {
            None => break,
            Some(msg) => msg,
        };
        tracing::info!(message = "bus message", bus_message = ?msg);
        match msg.view() {
            MessageView::Eos(_) => break,
            MessageView::Error(e) => break tracing::error!("{}", e.error()),
            MessageView::Warning(w) => tracing::warn!("{:?}", w),
            _ => {}
        }
    }
    pipeline
        .set_state(State::Null)
        .expect("could not stop the pipeline");
}

fn main() {
    // examples_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
