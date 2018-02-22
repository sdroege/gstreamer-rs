#[cfg(feature = "gst-rtsp-server")]
use std::env;

extern crate gstreamer as gst;

#[cfg(feature = "gst-rtsp-server")]
extern crate gstreamer_rtsp_server as gst_rtsp_server;
#[cfg(feature = "gst-rtsp-server")]
use gst_rtsp_server::prelude::*;

extern crate glib;

extern crate failure;
use failure::Error;

#[cfg(feature = "gst-rtsp-server")]
#[macro_use]
extern crate failure_derive;

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Fail)]
#[fail(display = "Could not get mount points")]
#[cfg(feature = "gst-rtsp-server")]
struct NoMountPoints;

#[derive(Debug, Fail)]
#[fail(display = "Usage: {} LAUNCH_LINE", _0)]
#[cfg(feature = "gst-rtsp-server")]
struct UsageError(String);

#[cfg(feature = "gst-rtsp-server")]
fn main_loop() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::from(UsageError(args[0].clone())));
    }

    let main_loop = glib::MainLoop::new(None, false);
    let server = gst_rtsp_server::RTSPServer::new();
    let factory = gst_rtsp_server::RTSPMediaFactory::new();
    let mounts = server.get_mount_points().ok_or(NoMountPoints)?;

    factory.set_launch(args[1].as_str());
    factory.set_shared(true);

    mounts.add_factory("/test", &factory);

    server.attach(None);

    println!(
        "Stream ready at rtsp://127.0.0.1:{}/test",
        server.get_bound_port()
    );

    main_loop.run();

    Ok(())
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    #[cfg(not(feature = "gst-rtsp-server"))]
    {
        eprintln!(
            "Feature gst-rtsp-server is required. Please rebuild with --features gst-rtsp-server"
        );
        std::process::exit(-1)
    }

    #[cfg(feature = "gst-rtsp-server")]
    main_loop()
}

fn main() {
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
