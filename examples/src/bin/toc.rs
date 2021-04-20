// This example demonstrates the use of GStreamer's ToC API. This API is used
// to manage a table of contents contained in the handled media stream.
// Chapters within a matroska file would be an example of a scenario for using
// this API. Elements that can parse ToCs from a stream (such as matroskademux)
// notify all elements in the pipeline when they encountered a ToC.
// For this, the example operates the following pipeline:

//                          /-{queue} - {fakesink}
// {filesrc} - {decodebin} - {queue} - {fakesink}
//                          \- ...

use gst::prelude::*;

use std::env;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: toc file_path");
        std::process::exit(-1)
    };

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("filesrc", None).unwrap();
    let decodebin = gst::ElementFactory::make("decodebin", None).unwrap();

    src.set_property("location", &uri).unwrap();

    pipeline.add_many(&[&src, &decodebin]).unwrap();
    gst::Element::link_many(&[&src, &decodebin]).unwrap();

    // Need to move a new reference into the closure.
    // !!ATTENTION!!:
    // It might seem appealing to use pipeline.clone() here, because that greatly
    // simplifies the code within the callback. What this actually dose, however, is creating
    // a memory leak. The clone of a pipeline is a new strong reference on the pipeline.
    // Storing this strong reference of the pipeline within the callback (we are moving it in!),
    // which is in turn stored in another strong reference on the pipeline is creating a
    // reference cycle.
    // DO NOT USE pipeline.clone() TO USE THE PIPELINE WITHIN A CALLBACK
    let pipeline_weak = pipeline.downgrade();
    // Connect to decodebin's pad-added signal, that is emitted whenever it found another stream
    // from the input file and found a way to decode it to its raw format.
    decodebin.connect_pad_added(move |_, src_pad| {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return,
        };

        // In this example, we are only interested about parsing the ToC, so
        // we simply pipe every encountered stream into a fakesink, essentially
        // throwing away the data.
        let queue = gst::ElementFactory::make("queue", None).unwrap();
        let sink = gst::ElementFactory::make("fakesink", None).unwrap();

        let elements = &[&queue, &sink];
        pipeline.add_many(elements).unwrap();
        gst::Element::link_many(elements).unwrap();

        for e in elements {
            e.sync_state_with_parent().unwrap();
        }

        let sink_pad = queue.static_pad("sink").unwrap();
        src_pad
            .link(&sink_pad)
            .expect("Unable to link src pad to sink pad");
    });

    pipeline
        .set_state(gst::State::Paused)
        .expect("Unable to set the pipeline to the `Paused` state");

    let bus = pipeline.bus().unwrap();

    // Instead of using a main loop (like GLib's), we manually iterate over
    // GStreamer's bus messages in this example. We don't need any special
    // functionality like timeouts or GLib socket notifications, so this is sufficient.
    // The bus is manually operated by repeatedly calling timed_pop on the bus with
    // the desired timeout for when to stop waiting for new messages. (None = Wait forever)
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(_) | MessageView::AsyncDone(_) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            MessageView::Toc(msg_toc) => {
                // Some element found a ToC in the current media stream and told
                // us by posting a message to GStreamer's bus.
                let (toc, updated) = msg_toc.toc();
                println!("\nReceived toc: {:?} - updated: {}", toc.scope(), updated);
                // Get a list of tags that are ToC specific.
                if let Some(tags) = toc.tags() {
                    println!("- tags: {}", tags.to_string());
                }
                // ToCs do not have a fixed structure. Depending on the format that
                // they were parsed from, they might have different tree-like structures,
                // so applications that want to support ToCs (for example in the form
                // of jumping between chapters in a video) have to try parsing  and
                // interpreting the ToC manually.
                // In this example, we simply want to print the ToC structure, so
                // we iterate everything and don't try to interpret anything.
                for toc_entry in toc.entries() {
                    // Every entry in a ToC has its own type. One type could for
                    // example be Chapter.
                    println!("\t{:?} - {}", toc_entry.entry_type(), toc_entry.uid());
                    // Every ToC entry can have a set of timestamps (start, stop).
                    if let Some((start, stop)) = toc_entry.start_stop_times() {
                        println!("\t- start: {}, stop: {}", start, stop);
                    }
                    // Every ToC entry can have tags to it.
                    if let Some(tags) = toc_entry.tags() {
                        println!("\t- tags: {}", tags.to_string());
                    }
                    // Every ToC entry can have a set of child entries.
                    // With this structure, you can create trees of arbitrary depth.
                    for toc_sub_entry in toc_entry.sub_entries() {
                        println!(
                            "\n\t\t{:?} - {}",
                            toc_sub_entry.entry_type(),
                            toc_sub_entry.uid()
                        );
                        if let Some((start, stop)) = toc_sub_entry.start_stop_times() {
                            println!("\t\t- start: {}, stop: {}", start, stop);
                        }
                        if let Some(tags) = toc_sub_entry.tags() {
                            println!("\t\t- tags: {:?}", tags.to_string());
                        }
                    }
                }
            }
            _ => (),
        }
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
