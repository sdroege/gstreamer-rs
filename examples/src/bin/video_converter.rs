// This example demonstrates how to use the GStreamer video converter
// API to configure the compositor element to do specific
// formatting of an input video.
//
use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    // This creates a pipeline by parsing the gst-launch pipeline syntax.
    let pipeline = gst::parse_launch(
        "videotestsrc name=src ! video/x-raw,width=640,height=480 ! compositor0.sink_0 \
         compositor ! video/x-raw,width=1280,height=720 ! videoconvert ! autovideosink",
    )
    .unwrap();

    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let compositor = pipeline.get_by_name("compositor0").unwrap();
    let sinkpad = compositor.get_static_pad("sink_0").unwrap();

    /* Completely contrived example that takes the 4:3 input video, cuts out a 5:4 frame
     * and then adds pillarbox borders to place it in a 16:9 target area */
    /* The output will be the full frame: */
    sinkpad.set_property("xpos", &0i32).unwrap();
    sinkpad.set_property("ypos", &0i32).unwrap();
    sinkpad.set_property("width", &1280i32).unwrap();
    sinkpad.set_property("height", &720i32).unwrap();

    let mut converter_config = gst_video::VideoConverterConfig::new();
    /* Crop the input frame to 5:4: */
    converter_config.set_src_x((640 - 512) / 2);
    converter_config.set_src_width(Some(512));
    converter_config.set_src_y(0);
    converter_config.set_src_height(Some(480));
    /* Add postbox borders to output 900x720 */
    converter_config.set_dest_x((1280 - 900) / 2);
    converter_config.set_dest_width(Some(900));
    converter_config.set_dest_y(0);
    converter_config.set_dest_height(Some(720));

    sinkpad
        .set_property("converter-config", &*converter_config)
        .unwrap();
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    /* Iterate messages on the bus until an error or EOS occurs,
     * although in this example the only error we'll hopefully
     * get is if the user closes the output window */
    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                println!("received eos");
                // An EndOfStream event was sent to the pipeline, so exit
                break;
            }
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        };
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
