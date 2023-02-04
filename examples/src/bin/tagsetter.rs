// This example demonstrates how to set and store metadata using
// GStreamer. Some elements support setting tags on a media stream.
// An example would be id3v2mux. The element signals this by implementing
// The GstTagsetter interface. You can query any element implementing this
// interface from the pipeline, and then tell the returned implementation
// of GstTagsetter what tags to apply to the media stream.
// This example's pipeline creates a new flac file from the testaudiosrc
// that the example application will add tags to using GstTagsetter.
// The operated pipeline looks like this:

// {audiotestsrc} - {flacenc} - {filesink}

// For example for pipelines that transcode a multimedia file, the input
// already has tags. For cases like this, the GstTagsetter has the merge
// setting, which the application can configure to tell the element
// implementing the interface whether to merge newly applied tags to the
// already existing ones, or if all existing ones should replace, etc.
// (More modes of operation are possible, see: gst::TagMergeMode)
// This merge-mode can also be supplied to any method that adds new tags.

use anyhow::{anyhow, Error};
use derive_more::{Display, Error};
use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {_0}")]
struct MissingElement(#[error(not(source))] String);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    // Parse the pipeline we want to probe from a static in-line string.
    let mut context = gst::ParseContext::new();
    let pipeline = match gst::parse_launch_full(
        "audiotestsrc wave=white-noise num-buffers=100 ! flacenc ! filesink location=test.flac",
        Some(&mut context),
        gst::ParseFlags::empty(),
    ) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            if let Some(gst::ParseError::NoSuchElement) = err.kind::<gst::ParseError>() {
                return Err(MissingElement(context.missing_elements().join(",")).into());
            } else {
                return Err(err.into());
            }
        }
    };

    let pipeline = pipeline
        .downcast::<gst::Pipeline>()
        .map_err(|_| anyhow!("Generated pipeline is no pipeline"))?;

    // Query the pipeline for elements implementing the GstTagsetter interface.
    // In our case, this will return the flacenc element.
    let tagsetter = pipeline
        .by_interface(gst::TagSetter::static_type())
        .ok_or_else(|| anyhow!("No TagSetter found"))?;
    let tagsetter = tagsetter
        .dynamic_cast::<gst::TagSetter>()
        .map_err(|_| anyhow!("No TagSetter found"))?;

    // Tell the element implementing the GstTagsetter interface how to handle already existing
    // metadata.
    tagsetter.set_tag_merge_mode(gst::TagMergeMode::KeepAll);
    // Set the "title" tag to "Special randomized white-noise".
    // The second parameter gst::TagMergeMode::Append tells the tagsetter to append this title
    // if there already is one.
    tagsetter
        .add_tag::<gst::tags::Title>(&"Special randomized white-noise", gst::TagMergeMode::Append);

    let bus = pipeline.bus().unwrap();

    pipeline.set_state(gst::State::Playing)?;

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| glib::GString::from("UNKNOWN")),
                    error: err.error(),
                    debug: err.debug(),
                }
                .into());
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}
