use anyhow::Error;
use glib::FlagsClass;
use gst::prelude::*;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

fn filter_vis_features(feature: &gst::PluginFeature) -> bool {
    match feature.downcast_ref::<gst::ElementFactory>() {
        Some(factory) => {
            let klass = factory.klass();
            klass.contains("Visualization")
        }
        None => false,
    }
}

fn tutorial_main() -> Result<(), Error> {
    // Initialize GStreamer
    gst::init()?;

    // Get a list of all visualization plugins
    let registry = gst::Registry::get();
    let list = registry.features_filtered(&filter_vis_features, false);
    let mut selected_factory: Option<gst::ElementFactory> = None;

    // Print their names
    println!("Available visualization plugins:");
    for feature in list {
        let factory = feature.downcast::<gst::ElementFactory>().unwrap();
        let name = factory.longname();
        println!("  {name}");

        if selected_factory.is_none() && name.starts_with("GOOM") {
            selected_factory = Some(factory);
        }
    }

    // Don't proceed if no visualization plugins were found
    let vis_factory = selected_factory.expect("No visualization plugins found.");

    // We have now selected a factory for the visualization element
    let name = vis_factory.longname();
    println!("Selected {name}");
    let vis_plugin = vis_factory.create().build().unwrap();

    // Build the pipeline
    let pipeline = gst::parse_launch("playbin uri=http://radio.hbr1.com:19800/ambient.ogg")?;

    // Set the visualization flag
    let flags = pipeline.property_value("flags");
    let flags_class = FlagsClass::with_type(flags.type_()).unwrap();
    let flags = flags_class
        .builder_with_value(flags)
        .unwrap()
        .set_by_nick("vis")
        .build()
        .unwrap();
    pipeline.set_property_from_value("flags", &flags);

    // Set vis plugin for playbin2
    pipeline.set_property("vis-plugin", &vis_plugin);

    // Start playing
    pipeline.set_state(gst::State::Playing)?;

    // Wait until an EOS or error message appears
    let bus = pipeline.bus().unwrap();
    let _msg = bus.timed_pop_filtered(
        gst::ClockTime::NONE,
        &[gst::MessageType::Error, gst::MessageType::Eos],
    );

    // Clean up
    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match tutorials_common::run(tutorial_main) {
        Ok(_) => {}
        Err(err) => eprintln!("Failed: {err}"),
    };
}
