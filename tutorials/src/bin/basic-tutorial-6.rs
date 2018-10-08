extern crate gstreamer as gst;
use gst::prelude::*;
use gst::MessageView;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

fn print_caps(caps: &gst::Caps, prefix: &str) {
    if caps.is_any() {
        println!("{}ANY", prefix);
        return;
    }

    if caps.is_empty() {
        println!("{}EMPTY", prefix);
        return;
    }

    for structure in caps.iter() {
        println!("{}{}", prefix, structure.get_name());
        for (field, value) in structure.iter() {
            println!("{}  {}:{:?}", prefix, field, value);
        }
    }
}

// Prints information about a Pad Template, including its Capabilitites
fn print_pad_template_information(factory: &gst::ElementFactory) {
    let long_name = factory
        .get_metadata("long-name")
        .expect("Failed to get long-name of element factory.");
    println!("Pad Template for {}:", long_name);

    if factory.get_num_pad_templates() == 0u32 {
        println!("  None");
        return;
    }

    for pad_template in factory.get_static_pad_templates() {
        if pad_template.direction() == gst::PadDirection::Src {
            println!("  SRC template: '{}'", pad_template.name_template());
        } else if pad_template.direction() == gst::PadDirection::Sink {
            println!("  SINK template: '{}'", pad_template.name_template());
        } else {
            println!("  UNKNOWN!!! template: '{}'", pad_template.name_template());
        }

        if pad_template.presence() == gst::PadPresence::Always {
            println!("  Availability: Always");
        } else if pad_template.presence() == gst::PadPresence::Sometimes {
            println!("  Availability: Sometimes");
        } else if pad_template.presence() == gst::PadPresence::Request {
            println!("  Availability: On request");
        } else {
            println!("  Availability: UNKNOWN!!!");
        }

        let caps = pad_template.get_caps();
        println!("  Capabilities:");
        print_caps(&caps, "    ");
    }
}

fn print_pad_capabilities(element: &gst::Element, pad_name: &str) {
    let pad = element
        .get_static_pad(pad_name)
        .expect("Could not retrieve pad");

    println!("Caps for the {} pad:", pad_name);
    match pad.get_current_caps() {
        Some(caps) => {
            print_caps(&caps, "      ");
        }
        None => {
            let caps = pad.query_caps(None).expect("Failed to query caps on pad");
            print_caps(&caps, "      ");
        }
    }
}

fn tutorial_main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create the element factories
    let source_factory =
        gst::ElementFactory::find("audiotestsrc").expect("Failed to create audiotestsrc factory.");
    let sink_factory = gst::ElementFactory::find("autoaudiosink")
        .expect("Failed to create autoaudiosink factory.");

    // Print information about the pad templates of these factories
    print_pad_template_information(&source_factory);
    print_pad_template_information(&sink_factory);

    // Ask the factories to instantiate actual elements
    let source = source_factory
        .create("source")
        .expect("Failed to create source element");
    let sink = sink_factory
        .create("sink")
        .expect("Failed to create sink element");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new("test-pipeline");

    pipeline.add_many(&[&source, &sink]).unwrap();
    source.link(&sink).expect("Elements could not be linked.");

    // Print initial negotiated caps (in NULL state)
    println!("In NULL state:");
    print_pad_capabilities(&sink, "sink");

    // Start playing
    let ret = pipeline.set_state(gst::State::Playing);
    if ret == gst::StateChangeReturn::Failure {
        eprintln!(
            "Unable to set the pipeline to the playing state (check the bus for error messages)."
        )
    }

    // Wait until error, EOS or State Change
    let bus = pipeline.get_bus().unwrap();

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        match msg.view() {
            MessageView::Error(err) => {
                println!(
                    "Error received from element {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            MessageView::Eos(..) => {
                println!("End-Of-Stream reached.");
                break;
            }
            MessageView::StateChanged(state_changed) =>
            // We are only interested in state-changed messages from the pipeline
            {
                if state_changed
                    .get_src()
                    .map(|s| s == pipeline)
                    .unwrap_or(false)
                {
                    let new_state = state_changed.get_current();
                    let old_state = state_changed.get_old();

                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        old_state, new_state
                    );
                    print_pad_capabilities(&sink, "sink");
                }
            }
            _ => (),
        }
    }

    // Shutdown pipeline
    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    tutorials_common::run(tutorial_main);
}
