extern crate gstreamer as gst;

use gst::GstObjectExt;
use gst::ElementExt;
use gst::MessageView;

fn main() {
    // Initialize GStreamer
    gst::init().unwrap();

    if let (Some(source), Some(sink)) = (
    // Create the elements
    gst::ElementFactory::make("videotestsrc", "source"),
    gst::ElementFactory::make("autovideosink", "sink")) {
        // Create the empty pipeline
        let pipeline = gst::Pipeline::new("test-pipeline");

        // Build the pipeline
        gst::BinExtManual::add_many(
            &pipeline,
            &[&source, &sink]).unwrap();
        if let Err(_) = gst::ElementExt::link(&source, &sink) {
            eprintln!("Elements could not be linked.");
            ::std::process::exit(-1);
        }

        // Modify the source's properties
        gst::GObjectExtManualGst::set_property_from_str(
            &source,
            "pattern",
            "0");

        // Start playing
        let ret = pipeline.set_state(gst::State::Playing);
        if ret == gst::StateChangeReturn::Failure {
            eprintln!("Unable to set the pipeline to the playing state.");
            ::std::process::exit(-1);
        }

        // Wait until error or EOS
        let bus = pipeline.get_bus().unwrap();
        if let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
            match msg.view() {
                MessageView::Error(err) => {
                    eprintln!("Error received from element {}: {}",
                             msg.get_src().get_path_string(), err.get_error());
                    eprintln!("Debugging information: {:?}", err.get_debug());
                },
                MessageView::Eos(..) => println!("End-Of-Stream reached."),
                // We should not reach here because we only asked for ERRORs and EOS
                _ => eprintln!("Unexpected message received."),
            }
        }
    }
    else {
        println!("Not all elements could be created.");
        ::std::process::exit(-1);
    }
}
