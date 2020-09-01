// This example demonstrates how to set up a rtsp server using GStreamer
// and extending the behaviour by subclass RTSPMediaFactory and RTSPMedia.
// For this, the example creates a videotestsrc pipeline manually to be used
// by the RTSP server for providing data, and adds a custom attribute to the
// SDP provided to the client.
//
// It also comes with a custom RTSP server/client subclass for hooking into
// the client machinery and printing some status.

extern crate gstreamer as gst;
extern crate gstreamer_rtsp as gst_rtsp;
extern crate gstreamer_rtsp_server as gst_rtsp_server;
extern crate gstreamer_sdp as gst_sdp;

use gst_rtsp_server::prelude::*;

use glib::glib_object_subclass;
use glib::glib_wrapper;

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
    let server = server::Server::new();
    // Much like HTTP servers, RTSP servers have multiple endpoints that
    // provide different streams. Here, we ask our server to give
    // us a reference to his list of endpoints, so we can add our
    // test endpoint, providing the pipeline from the cli.
    let mounts = server.get_mount_points().ok_or(NoMountPoints)?;

    // Next, we create our custom factory for the endpoint we want to create.
    // The job of the factory is to create a new pipeline for each client that
    // connects, or (if configured to do so) to reuse an existing pipeline.
    let factory = media_factory::Factory::new();
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
    let id = server.attach(None);

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

// Our custom media factory that creates a media input manually
mod media_factory {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    extern crate gstreamer_rtsp_server as gst_rtsp_server;
    use gst_rtsp_server::subclass::prelude::*;

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;

        // This is the private data of our factory
        pub struct Factory {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        impl ObjectSubclass for Factory {
            const NAME: &'static str = "RsRTSPMediaFactory";
            type ParentType = gst_rtsp_server::RTSPMediaFactory;
            type Instance = gst::subclass::ElementInstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            // This macro provides some boilerplate
            glib_object_subclass!();

            // Called when a new instance is to be created. We need to return an instance
            // of our struct here.
            fn new() -> Self {
                Self {}
            }
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Factory {
            fn constructed(&self, obj: &glib::Object) {
                self.parent_constructed(obj);

                let factory = obj
                    .downcast_ref::<gst_rtsp_server::RTSPMediaFactory>()
                    .unwrap();

                // All media created by this factory are our custom media type. This would
                // not require a media factory subclass and can also be called on the normal
                // RTSPMediaFactory.
                factory.set_media_gtype(super::media::Media::static_type());
            }
        }

        // Implementation of gst_rtsp_server::RTSPMediaFactory virtual methods
        impl RTSPMediaFactoryImpl for Factory {
            fn create_element(
                &self,
                _factory: &gst_rtsp_server::RTSPMediaFactory,
                _url: &gst_rtsp::RTSPUrl,
            ) -> Option<gst::Element> {
                // Create a simple VP8 videotestsrc input
                let bin = gst::Bin::new(None);
                let src = gst::ElementFactory::make("videotestsrc", None).unwrap();
                let enc = gst::ElementFactory::make("vp8enc", None).unwrap();

                // The names of the payloaders must be payX
                let pay = gst::ElementFactory::make("rtpvp8pay", Some("pay0")).unwrap();

                // Configure the videotestsrc live
                src.set_property("is-live", &true).unwrap();

                // Produce encoded data as fast as possible
                enc.set_property("deadline", &1i64).unwrap();

                bin.add_many(&[&src, &enc, &pay]).unwrap();
                gst::Element::link_many(&[&src, &enc, &pay]).unwrap();

                Some(bin.upcast())
            }
        }
    }

    // This here defines the public interface of our factory and implements
    // the corresponding traits so that it behaves like any other RTSPMediaFactory
    glib_wrapper! {
        pub struct Factory(
            Object<
                gst::subclass::ElementInstanceStruct<imp::Factory>,
                subclass::simple::ClassStruct<imp::Factory>,
                FactoryClass
            >
        ) @extends gst_rtsp_server::RTSPMediaFactory;

        match fn {
            get_type => || imp::Factory::get_type().to_glib(),
        }
    }

    // Factories must be Send+Sync, and ours is
    unsafe impl Send for Factory {}
    unsafe impl Sync for Factory {}

    impl Factory {
        // Creates a new instance of our factory
        pub fn new() -> Factory {
            glib::Object::new(Self::static_type(), &[])
                .expect("Failed to create factory")
                .downcast()
                .expect("Created factory is of wrong type")
        }
    }
}

// Our custom media subclass that adds a custom attribute to the SDP returned by DESCRIBE
mod media {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    extern crate gstreamer_rtsp_server as gst_rtsp_server;
    use gst_rtsp_server::subclass::prelude::*;

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;

        // This is the private data of our media
        pub struct Media {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        impl ObjectSubclass for Media {
            const NAME: &'static str = "RsRTSPMedia";
            type ParentType = gst_rtsp_server::RTSPMedia;
            type Instance = gst::subclass::ElementInstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            // This macro provides some boilerplate
            glib_object_subclass!();

            // Called when a new instance is to be created. We need to return an instance
            // of our struct here.
            fn new() -> Self {
                Self {}
            }
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Media {}

        // Implementation of gst_rtsp_server::RTSPMedia virtual methods
        impl RTSPMediaImpl for Media {
            fn setup_sdp(
                &self,
                media: &gst_rtsp_server::RTSPMedia,
                sdp: &mut gst_sdp::SDPMessageRef,
                info: &gst_rtsp_server::subclass::SDPInfo,
            ) -> Result<(), gst::LoggableError> {
                self.parent_setup_sdp(media, sdp, info)?;

                sdp.add_attribute("my-custom-attribute", Some("has-a-value"));

                Ok(())
            }
        }
    }

    // This here defines the public interface of our factory and implements
    // the corresponding traits so that it behaves like any other RTSPMedia
    glib_wrapper! {
        pub struct Media(
            Object<
                gst::subclass::ElementInstanceStruct<imp::Media>,
                subclass::simple::ClassStruct<imp::Media>,
                MediaClass
            >
        ) @extends gst_rtsp_server::RTSPMedia;

        match fn {
            get_type => || imp::Media::get_type().to_glib(),
        }
    }

    // Medias must be Send+Sync, and ours is
    unsafe impl Send for Media {}
    unsafe impl Sync for Media {}
}

// Our custom RTSP server subclass that reports when clients are connecting and uses
// our custom RTSP client subclass for each client
mod server {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    extern crate gstreamer_rtsp_server as gst_rtsp_server;
    use gst_rtsp_server::subclass::prelude::*;

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;

        // This is the private data of our server
        pub struct Server {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        impl ObjectSubclass for Server {
            const NAME: &'static str = "RsRTSPServer";
            type ParentType = gst_rtsp_server::RTSPServer;
            type Instance = gst::subclass::ElementInstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            // This macro provides some boilerplate
            glib_object_subclass!();

            // Called when a new instance is to be created. We need to return an instance
            // of our struct here.
            fn new() -> Self {
                Self {}
            }
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Server {}

        // Implementation of gst_rtsp_server::RTSPServer virtual methods
        impl RTSPServerImpl for Server {
            fn create_client(
                &self,
                server: &gst_rtsp_server::RTSPServer,
            ) -> Option<gst_rtsp_server::RTSPClient> {
                let client = super::client::Client::new();

                // Duplicated from the default implementation
                client.set_session_pool(server.get_session_pool().as_ref());
                client.set_mount_points(server.get_mount_points().as_ref());
                client.set_auth(server.get_auth().as_ref());
                client.set_thread_pool(server.get_thread_pool().as_ref());

                Some(client.upcast())
            }

            fn client_connected(
                &self,
                server: &gst_rtsp_server::RTSPServer,
                client: &gst_rtsp_server::RTSPClient,
            ) {
                self.parent_client_connected(server, client);
                println!("Client {:?} connected", client);
            }
        }
    }

    // This here defines the public interface of our factory and implements
    // the corresponding traits so that it behaves like any other RTSPServer
    glib_wrapper! {
        pub struct Server(
            Object<
                gst::subclass::ElementInstanceStruct<imp::Server>,
                subclass::simple::ClassStruct<imp::Server>,
                ServerClass
            >
        ) @extends gst_rtsp_server::RTSPServer;

        match fn {
            get_type => || imp::Server::get_type().to_glib(),
        }
    }

    // Servers must be Send+Sync, and ours is
    unsafe impl Send for Server {}
    unsafe impl Sync for Server {}

    impl Server {
        // Creates a new instance of our factory
        pub fn new() -> Server {
            glib::Object::new(Self::static_type(), &[])
                .expect("Failed to create server")
                .downcast()
                .expect("Created server is of wrong type")
        }
    }
}

// Our custom RTSP client subclass.
mod client {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;
    use glib::translate::*;

    extern crate gstreamer_rtsp_server as gst_rtsp_server;
    use gst_rtsp_server::subclass::prelude::*;

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;

        // This is the private data of our server
        pub struct Client {}

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        impl ObjectSubclass for Client {
            const NAME: &'static str = "RsRTSPClient";
            type ParentType = gst_rtsp_server::RTSPClient;
            type Instance = gst::subclass::ElementInstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            // This macro provides some boilerplate
            glib_object_subclass!();

            // Called when a new instance is to be created. We need to return an instance
            // of our struct here.
            fn new() -> Self {
                Self {}
            }
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Client {}

        // Implementation of gst_rtsp_server::RTSPClient virtual methods
        impl RTSPClientImpl for Client {
            fn closed(&self, client: &gst_rtsp_server::RTSPClient) {
                self.parent_closed(client);
                println!("Client {:?} closed", client);
            }
        }
    }

    // This here defines the public interface of our factory and implements
    // the corresponding traits so that it behaves like any other RTSPClient
    glib_wrapper! {
        pub struct Client(
            Object<
                gst::subclass::ElementInstanceStruct<imp::Client>,
                subclass::simple::ClassStruct<imp::Client>,
                ClientClass
            >
        ) @extends gst_rtsp_server::RTSPClient;

        match fn {
            get_type => || imp::Client::get_type().to_glib(),
        }
    }

    // Clients must be Send+Sync, and ours is
    unsafe impl Send for Client {}
    unsafe impl Sync for Client {}

    impl Client {
        // Creates a new instance of our factory
        pub fn new() -> Client {
            glib::Object::new(Self::static_type(), &[])
                .expect("Failed to create client")
                .downcast()
                .expect("Created client is of wrong type")
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
