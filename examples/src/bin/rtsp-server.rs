// This example demonstrates how to set up a rtsp server using GStreamer.
// For this, the example parses an arbitrary pipeline in launch syntax
// from the cli and provides this pipeline's output as stream, served
// using GStreamers rtsp server.

use std::env;

use gst_rtsp_server::prelude::*;

use anyhow::Error;
use derive_more::{Display, Error};

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display(fmt = "Could not get mount points")]
struct NoMountPoints;

#[derive(Debug, Display, Error)]
#[display(fmt = "Usage: {} LAUNCH_LINE", _0)]
struct UsageError(#[error(not(source))] String);

fn main_loop() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::from(UsageError(args[0].clone())));
    }

    let main_loop = glib::MainLoop::new(None, false);
    let server = gst_rtsp_server::RTSPServer::new();
    // Much like HTTP servers, RTSP servers have multiple endpoints that
    // provide different streams. Here, we ask our server to give
    // us a reference to his list of endpoints, so we can add our
    // test endpoint, providing the pipeline from the cli.
    let mounts = server.get_mount_points().ok_or(NoMountPoints)?;

    // Next, we create a factory for the endpoint we want to create.
    // The job of the factory is to create a new pipeline for each client that
    // connects, or (if configured to do so) to reuse an existing pipeline.
    let factory = gst_rtsp_server::RTSPMediaFactory::new();
    // Here we tell the media factory the media we want to serve.
    // This is done in the launch syntax. When the first client connects,
    // the factory will use this syntax to create a new pipeline instance.
    factory.set_launch(args[1].as_str());
    // This setting specifies whether each connecting client gets the output
    // of a new instance of the pipeline, or whether all connected clients share
    // the output of the same pipeline.
    // If you want to stream a fixed video you have stored on the server to any
    // client, you would not set this to shared here (since every client wants
    // to start at the beginning of the video). But if you want to distribute
    // a live source, you will probably want to set this to shared, to save
    // computing and memory capacity on the server.
    factory.set_shared(true);

    // Now we add a new mount-point and tell the RTSP server to serve the content
    // provided by the factory we configured above, when a client connects to
    // this specific path.
    mounts.add_factory("/test", &factory);

    // Attach the server to our main context.
    // A main context is the thing where other stuff is registering itself for its
    // events (e.g. sockets, GStreamer bus, ...) and the main loop is something that
    // polls the main context for its events and dispatches them to whoever is
    // interested in them. In this example, we only do have one, so we can
    // leave the context parameter empty, it will automatically select
    // the default one.
    let id = server.attach(None)?;

    println!(
        "Stream ready at rtsp://127.0.0.1:{}/test",
        server.get_bound_port()
    );

    // Start the mainloop. From this point on, the server will start to serve
    // our quality content to connecting clients.
    main_loop.run();

    glib::source_remove(id);

    Ok(())
}

fn example_main() -> Result<(), Error> {
    gst::init()?;
    main_loop()
}

fn main() {
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
