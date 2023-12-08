// This example demonstrates how to set up a rtsp server using GStreamer
// and extending the default auth module behaviour by subclassing RTSPAuth
// For this, the example creates a videotestsrc pipeline manually to be used
// by the RTSP server for providing data
#![allow(clippy::non_send_fields_in_send_ty)]

use anyhow::Error;
use derive_more::{Display, Error};
use gst_rtsp_server::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display(fmt = "Could not get mount points")]
struct NoMountPoints;

fn main_loop() -> Result<(), Error> {
    let main_loop = glib::MainLoop::new(None, false);
    let server = gst_rtsp_server::RTSPServer::new();

    // We create our custom auth module.
    // The job of the auth module is to authenticate users and authorize
    // factories access/construction.
    let auth = auth::Auth::default();
    server.set_auth(Some(&auth));

    // Much like HTTP servers, RTSP servers have multiple endpoints that
    // provide different streams. Here, we ask our server to give
    // us a reference to his list of endpoints, so we can add our
    // test endpoint, providing the pipeline from the cli.
    let mounts = server.mount_points().ok_or(NoMountPoints)?;

    // Next, we create a factory for the endpoint we want to create.
    // The job of the factory is to create a new pipeline for each client that
    // connects, or (if configured to do so) to reuse an existing pipeline.
    let factory = gst_rtsp_server::RTSPMediaFactory::new();
    // Here we tell the media factory the media we want to serve.
    // This is done in the launch syntax. When the first client connects,
    // the factory will use this syntax to create a new pipeline instance.
    factory.set_launch("( videotestsrc ! vp8enc ! rtpvp8pay name=pay0 )");
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
    mounts.add_factory("/test", factory);

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
    println!("user admin/password can access stream");
    println!("user demo/demo passes authentication but receives 404");
    println!("other users do not pass pass authentication and receive 401");

    // Start the mainloop. From this point on, the server will start to serve
    // our quality content to connecting clients.
    main_loop.run();

    id.remove();

    Ok(())
}

// Our custom auth module
mod auth {
    // In the imp submodule we include the actual implementation
    mod imp {
        use gst_rtsp::{RTSPHeaderField, RTSPStatusCode};
        use gst_rtsp_server::{prelude::*, subclass::prelude::*, RTSPContext};

        // This is the private data of our auth
        #[derive(Default)]
        pub struct Auth;

        impl Auth {
            // Simulate external auth validation and user extraction
            // authorized users are admin/password and demo/demo
            fn external_auth(&self, auth: &str) -> Option<String> {
                if let Ok(decoded) = data_encoding::BASE64.decode(auth.as_bytes()) {
                    if let Ok(decoded) = std::str::from_utf8(&decoded) {
                        let tokens = decoded.split(':').collect::<Vec<_>>();
                        if tokens == vec!["admin", "password"] || tokens == vec!["demo", "demo"] {
                            return Some(tokens[0].into());
                        }
                    }
                }
                None
            }

            // Simulate external role check
            // admin user can construct and access media factory
            fn external_access_check(&self, user: &str) -> bool {
                user == "admin"
            }
        }

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        #[glib::object_subclass]
        impl ObjectSubclass for Auth {
            const NAME: &'static str = "RsRTSPAuth";
            type Type = super::Auth;
            type ParentType = gst_rtsp_server::RTSPAuth;
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for Auth {}

        // Implementation of gst_rtsp_server::RTSPAuth virtual methods
        impl RTSPAuthImpl for Auth {
            fn authenticate(&self, ctx: &RTSPContext) -> bool {
                // authenticate should always be called with a valid context request
                let req = ctx
                    .request()
                    .expect("Context without request. Should not happen !");

                if let Some(auth_credentials) = req.parse_auth_credentials().get(0) {
                    if let Some(authorization) = auth_credentials.authorization() {
                        if let Some(user) = self.external_auth(authorization) {
                            // Update context token with authenticated username
                            ctx.set_token(gst_rtsp_server::RTSPToken::new(&[("user", &user)]));
                            return true;
                        }
                    }
                }

                false
            }

            fn check(&self, ctx: &RTSPContext, role: &glib::GString) -> bool {
                // We only check media factory access
                if !role.starts_with("auth.check.media.factory") {
                    return true;
                }

                if ctx.token().is_none() {
                    // If we do not have a context token yet, check if there are any auth credentials in request
                    if !self.authenticate(ctx) {
                        // If there were no credentials, send a "401 Unauthorized" response
                        if let Some(resp) = ctx.response() {
                            resp.init_response(RTSPStatusCode::Unauthorized, ctx.request());
                            resp.add_header(
                                RTSPHeaderField::WwwAuthenticate,
                                "Basic realm=\"CustomRealm\"",
                            );
                            if let Some(client) = ctx.client() {
                                client.send_message(resp, ctx.session());
                            }
                        }
                        return false;
                    }
                }

                if let Some(token) = ctx.token() {
                    // If we already have a user token...
                    if self.external_access_check(&token.string("user").unwrap_or_default()) {
                        // grant access if user may access factory
                        return true;
                    } else {
                        // send a "404 Not Found" response if user may not access factory
                        if let Some(resp) = ctx.response() {
                            resp.init_response(RTSPStatusCode::NotFound, ctx.request());
                            if let Some(client) = ctx.client() {
                                client.send_message(resp, ctx.session());
                            }
                        }
                    }
                }

                false
            }
        }
    }

    // This here defines the public interface of our auth and implements
    // the corresponding traits so that it behaves like any other RTSPAuth
    glib::wrapper! {
        pub struct Auth(ObjectSubclass<imp::Auth>) @extends gst_rtsp_server::RTSPAuth;
    }

    impl Default for Auth {
        // Creates a new instance of our auth
        fn default() -> Self {
            glib::Object::new()
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
        Err(e) => eprintln!("Error! {e}"),
    }
}
