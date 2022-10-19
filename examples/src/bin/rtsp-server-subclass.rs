// This example demonstrates how to set up a rtsp server using GStreamer
// and extending the behaviour by subclass RTSPMediaFactory and RTSPMedia.
// For this, the example creates a videotestsrc pipeline manually to be used
// by the RTSP server for providing data, and adds a custom attribute to the
// SDP provided to the client.
//
// It also comes with a custom RTSP server/client subclass for hooking into
// the client machinery and printing some status.
#![allow(clippy::non_send_fields_in_send_ty)]

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
    let main_loop = glib::MainLoop::new(None, false);
    let server = server::Server::default();

    let mounts = mount_points::MountPoints::default();
    server.set_mount_points(Some(&mounts));

    // Much like HTTP servers, RTSP servers have multiple endpoints that
    // provide different streams. Here, we ask our server to give
    // us a reference to his list of endpoints, so we can add our
    // test endpoint, providing the pipeline from the cli.
    let mounts = server.mount_points().ok_or(NoMountPoints)?;

    // Next, we create our custom factory for the endpoint we want to create.
    // The job of the factory is to create a new pipeline for each client that
    // connects, or (if configured to do so) to reuse an existing pipeline.
    let factory = media_factory::Factory::default();
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
        server.bound_port()
    );

    // Start the mainloop. From this point on, the server will start to serve
    // our quality content to connecting clients.
    main_loop.run();

    id.remove();

    Ok(())
}

// Our custom media factory that creates a media input manually
mod media_factory {
    use super::*;

    use gst_rtsp_server::subclass::prelude::*;

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;

        // This is the private data of our factory
        #[derive(Default)]
        pub struct Factory {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        #[glib::object_subclass]
        impl ObjectSubclass for Factory {
            const NAME: &'static str = "RsRTSPMediaFactory";
            type Type = super::Factory;
            type ParentType = gst_rtsp_server::RTSPMediaFactory;
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Factory {
            fn constructed(&self) {
                self.parent_constructed();

                let factory = self.instance();
                // All media created by this factory are our custom media type. This would
                // not require a media factory subclass and can also be called on the normal
                // RTSPMediaFactory.
                factory.set_media_gtype(super::media::Media::static_type());
            }
        }

        // Implementation of gst_rtsp_server::RTSPMediaFactory virtual methods
        impl RTSPMediaFactoryImpl for Factory {
            fn create_element(&self, _url: &gst_rtsp::RTSPUrl) -> Option<gst::Element> {
                // Create a simple VP8 videotestsrc input
                let bin = gst::Bin::new(None);
                let src = gst::ElementFactory::make("videotestsrc")
                    // Configure the videotestsrc live
                    .property("is-live", true)
                    .build()
                    .unwrap();
                let enc = gst::ElementFactory::make("vp8enc")
                    // Produce encoded data as fast as possible
                    .property("deadline", 1i64)
                    .build()
                    .unwrap();

                // The names of the payloaders must be payX
                let pay = gst::ElementFactory::make("rtpvp8pay")
                    .name("pay0")
                    .build()
                    .unwrap();

                bin.add_many(&[&src, &enc, &pay]).unwrap();
                gst::Element::link_many(&[&src, &enc, &pay]).unwrap();

                Some(bin.upcast())
            }
        }
    }

    // This here defines the public interface of our factory and implements
    // the corresponding traits so that it behaves like any other RTSPMediaFactory
    glib::wrapper! {
        pub struct Factory(ObjectSubclass<imp::Factory>) @extends gst_rtsp_server::RTSPMediaFactory;
    }

    impl Default for Factory {
        // Creates a new instance of our factory
        fn default() -> Factory {
            glib::Object::new(&[])
        }
    }
}

// Our custom media subclass that adds a custom attribute to the SDP returned by DESCRIBE
mod media {
    use gst_rtsp_server::subclass::prelude::*;

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;

        // This is the private data of our media
        #[derive(Default)]
        pub struct Media {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        #[glib::object_subclass]
        impl ObjectSubclass for Media {
            const NAME: &'static str = "RsRTSPMedia";
            type Type = super::Media;
            type ParentType = gst_rtsp_server::RTSPMedia;
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Media {}

        // Implementation of gst_rtsp_server::RTSPMedia virtual methods
        impl RTSPMediaImpl for Media {
            fn setup_sdp(
                &self,
                sdp: &mut gst_sdp::SDPMessageRef,
                info: &gst_rtsp_server::subclass::SDPInfo,
            ) -> Result<(), gst::LoggableError> {
                self.parent_setup_sdp(sdp, info)?;

                sdp.add_attribute("my-custom-attribute", Some("has-a-value"));

                Ok(())
            }
        }
    }

    // This here defines the public interface of our factory and implements
    // the corresponding traits so that it behaves like any other RTSPMedia
    glib::wrapper! {
        pub struct Media(ObjectSubclass<imp::Media>) @extends gst_rtsp_server::RTSPMedia;
    }
}

// Our custom RTSP server subclass that reports when clients are connecting and uses
// our custom RTSP client subclass for each client
mod server {
    use super::*;

    use gst_rtsp_server::subclass::prelude::*;

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;

        // This is the private data of our server
        #[derive(Default)]
        pub struct Server {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        #[glib::object_subclass]
        impl ObjectSubclass for Server {
            const NAME: &'static str = "RsRTSPServer";
            type Type = super::Server;
            type ParentType = gst_rtsp_server::RTSPServer;
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Server {}

        // Implementation of gst_rtsp_server::RTSPServer virtual methods
        impl RTSPServerImpl for Server {
            fn create_client(&self) -> Option<gst_rtsp_server::RTSPClient> {
                let server = self.instance();
                let client = super::client::Client::default();

                // Duplicated from the default implementation
                client.set_session_pool(server.session_pool().as_ref());
                client.set_mount_points(server.mount_points().as_ref());
                client.set_auth(server.auth().as_ref());
                client.set_thread_pool(server.thread_pool().as_ref());

                Some(client.upcast())
            }

            fn client_connected(&self, client: &gst_rtsp_server::RTSPClient) {
                self.parent_client_connected(client);
                println!("Client {:?} connected", client);
            }
        }
    }

    // This here defines the public interface of our factory and implements
    // the corresponding traits so that it behaves like any other RTSPServer
    glib::wrapper! {
        pub struct Server(ObjectSubclass<imp::Server>) @extends gst_rtsp_server::RTSPServer;
    }

    impl Default for Server {
        // Creates a new instance of our factory
        fn default() -> Server {
            glib::Object::new(&[])
        }
    }
}

// Our custom RTSP client subclass.
mod client {
    use gst_rtsp_server::subclass::prelude::*;

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;

        // This is the private data of our server
        #[derive(Default)]
        pub struct Client {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        #[glib::object_subclass]
        impl ObjectSubclass for Client {
            const NAME: &'static str = "RsRTSPClient";
            type Type = super::Client;
            type ParentType = gst_rtsp_server::RTSPClient;
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Client {}

        // Implementation of gst_rtsp_server::RTSPClient virtual methods
        impl RTSPClientImpl for Client {
            fn closed(&self) {
                let client = self.instance();
                self.parent_closed();
                println!("Client {:?} closed", client);
            }
        }
    }

    // This here defines the public interface of our factory and implements
    // the corresponding traits so that it behaves like any other RTSPClient
    glib::wrapper! {
        pub struct Client(ObjectSubclass<imp::Client>) @extends gst_rtsp_server::RTSPClient;
    }

    impl Default for Client {
        // Creates a new instance of our factory
        fn default() -> Client {
            glib::Object::new(&[])
        }
    }
}

mod mount_points {
    use gst_rtsp_server::subclass::prelude::*;

    mod imp {
        use super::*;

        // This is the private data of our mount points
        #[derive(Default)]
        pub struct MountPoints {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        #[glib::object_subclass]
        impl ObjectSubclass for MountPoints {
            const NAME: &'static str = "RsRTSPMountPoints";
            type Type = super::MountPoints;
            type ParentType = gst_rtsp_server::RTSPMountPoints;
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for MountPoints {}

        // Implementation of gst_rtsp_server::RTSPClient virtual methods
        impl RTSPMountPointsImpl for MountPoints {
            fn make_path(&self, url: &gst_rtsp::RTSPUrl) -> Option<glib::GString> {
                println!("Make path called for {:?} ", url);
                self.parent_make_path(url)
            }
        }
    }

    glib::wrapper! {
        pub struct MountPoints(ObjectSubclass<imp::MountPoints>) @extends gst_rtsp_server::RTSPMountPoints;
    }

    impl Default for MountPoints {
        // Creates a new instance of our factory
        fn default() -> Self {
            glib::Object::new(&[])
        }
    }
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
