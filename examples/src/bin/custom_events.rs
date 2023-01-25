// This example demonstrates how custom application-specific events can be
// created, sent in a pipeline and retrieved later.
//
// It uses a queue that contains several seconds worth of data. When the event
// is sent on the sink pad, we expect to see it emerge on the other side when
// the data in front of it has exited.

use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug)]
pub struct ExampleCustomEvent {
    pub send_eos: bool,
}

impl ExampleCustomEvent {
    const EVENT_NAME: &'static str = "example-custom-event";

    #[allow(clippy::new_ret_no_self)]
    pub fn new(send_eos: bool) -> gst::Event {
        let s = gst::Structure::builder(Self::EVENT_NAME)
            .field("send_eos", send_eos)
            .build();
        gst::event::CustomDownstream::new(s)
    }

    pub fn parse(ev: &gst::EventRef) -> Option<ExampleCustomEvent> {
        match ev.view() {
            gst::EventView::CustomDownstream(e) => {
                let s = match e.structure() {
                    Some(s) if s.name() == Self::EVENT_NAME => s,
                    _ => return None, // No structure in this event, or the name didn't match
                };

                let send_eos = s.get::<bool>("send_eos").unwrap();
                Some(ExampleCustomEvent { send_eos })
            }
            _ => None, // Not a custom event
        }
    }
}

fn example_main() {
    gst::init().unwrap();

    let main_loop = glib::MainLoop::new(None, false);

    // This creates a pipeline by parsing the gst-launch pipeline syntax.
    let pipeline = gst::parse_launch(
        "audiotestsrc name=src ! queue max-size-time=2000000000 ! fakesink name=sink sync=true",
    )
    .unwrap();
    let bus = pipeline.bus().unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    let sink = pipeline.by_name("sink").unwrap();
    let sinkpad = sink.static_pad("sink").unwrap();

    // Need to move a new reference into the closure.
    // !!ATTENTION!!:
    // It might seem appealing to use pipeline.clone() here, because that greatly
    // simplifies the code within the callback. What this actually does, however, is creating
    // a memory leak. The clone of a pipeline is a new strong reference on the pipeline.
    // Storing this strong reference of the pipeline within the callback (we are moving it in!),
    // which is in turn stored in another strong reference on the pipeline is creating a
    // reference cycle.
    // DO NOT USE pipeline.clone() TO USE THE PIPELINE WITHIN A CALLBACK
    let pipeline_weak = pipeline.downgrade();
    // Add a pad probe on the sink pad and catch the custom event we sent, then send
    // an EOS event on the pipeline.
    sinkpad.add_probe(gst::PadProbeType::EVENT_DOWNSTREAM, move |_, probe_info| {
        match probe_info.data {
            Some(gst::PadProbeData::Event(ref ev))
                if ev.type_() == gst::EventType::CustomDownstream =>
            {
                if let Some(custom_event) = ExampleCustomEvent::parse(ev) {
                    if let Some(pipeline) = pipeline_weak.upgrade() {
                        if custom_event.send_eos {
                            /* Send EOS event to shut down the pipeline, but from an async callback, as we're
                             * in a pad probe blocking the stream thread here... */
                            println!("Got custom event with send_eos=true. Sending EOS");
                            let ev = gst::event::Eos::new();
                            let pipeline_weak = pipeline_weak.clone();
                            pipeline.call_async(move |_| {
                                if let Some(pipeline) = pipeline_weak.upgrade() {
                                    pipeline.send_event(ev);
                                }
                            });
                        } else {
                            println!("Got custom event, with send_eos=false. Ignoring");
                        }
                    }
                }
            }
            _ => (),
        }
        gst::PadProbeReturn::Ok
    });

    println!("Pipeline is running. Waiting 2 seconds");

    /* Send 2 events into the pipeline - one with send_eos = false, followed
     * by 1 with send_eos = true. Use a timeout event to send them in a few seconds
     * when the pipeline has filled. */
    for (i, send_eos) in [false, true].iter().enumerate() {
        let pipeline_weak = pipeline.downgrade();
        glib::timeout_add_seconds(2 + i as u32, move || {
            // Here we temporarily retrieve a strong reference on the pipeline from the weak one
            // we moved into this callback.
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return glib::Continue(false),
            };
            println!("Sending custom event to the pipeline with send_eos={send_eos}");
            let ev = ExampleCustomEvent::new(*send_eos);
            if !pipeline.send_event(ev) {
                println!("Warning: Failed to send custom event");
            }
            // Remove this handler, the pipeline will shutdown once our pad probe catches the custom
            // event and sends EOS
            glib::Continue(false)
        });
    }

    let main_loop_clone = main_loop.clone();
    // This sets the bus's signal handler (don't be mislead by the "add", there can only be one).
    // Every message from the bus is passed through this function. Its returnvalue determines
    // whether the handler wants to be called again. If glib::Continue(false) is returned, the
    // handler is removed and will never be called again. The mainloop still runs though.
    bus.add_watch(move |_, msg| {
        use gst::MessageView;

        let main_loop = &main_loop_clone;
        match msg.view() {
            MessageView::Eos(..) => {
                println!("received eos");
                // An EndOfStream event was sent to the pipeline, so we tell our main loop
                // to stop execution here.
                main_loop.quit()
            }
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                main_loop.quit();
            }
            _ => (),
        };

        // Tell the mainloop to continue executing this callback.
        glib::Continue(true)
    })
    .expect("Failed to add bus watch");

    // Operate GStreamer's bus, facilitating GLib's mainloop here.
    // This function call will block until you tell the mainloop to quit
    // (see above for how to do this).
    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    // Remove the watch function from the bus.
    // Again: There can always only be one watch function.
    // Thus we don't have to tell him which function to remove.
    bus.remove_watch().unwrap();
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
