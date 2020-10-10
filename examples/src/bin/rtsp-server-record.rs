// This example demonstrates how to set up a rtsp server using GStreamer.
// While the "rtsp-server" example is about streaming media to connecting
// clients, this example is mainly about recording media that clients
// send to the server. For this, the launch syntax pipeline, that is passed
// to this example's cli is spawned and the client's media is streamed into it.

use std::env;
use std::ptr;

use glib::translate::*;
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

    // Mostly analog to the rtsp-server example, the server is created
    // and the factory for our test mount is configured.
    let main_loop = glib::MainLoop::new(None, false);
    let server = gst_rtsp_server::RTSPServer::new();
    // Much like HTTP servers, RTSP servers have multiple endpoints that
    // provide or take different streams. Here, we ask our server to give
    // us a reference to its list of endpoints, so we can add our
    // test endpoint.
    let mounts = server.get_mount_points().ok_or(NoMountPoints)?;
    // Next, we create a factory for the endpoint we want to create.
    // The job of the factory is to create a new pipeline for each client that
    // connects, or (if configured to do so) to reuse an existing pipeline.
    let factory = gst_rtsp_server::RTSPMediaFactory::new();
    // Here we configure a method of authentication that we want the
    // server to require from clients.
    let auth = gst_rtsp_server::RTSPAuth::new();
    let token = gst_rtsp_server::RTSPToken::new(&[(
        *gst_rtsp_server::RTSP_TOKEN_MEDIA_FACTORY_ROLE,
        &"user",
    )]);
    let basic = gst_rtsp_server::RTSPAuth::make_basic("user", "password");
    // For propery authentication, we want to use encryption. And there's no
    // encryption without a certificate!
    let cert = gio::TlsCertificate::from_pem(
        "-----BEGIN CERTIFICATE-----\
         MIICJjCCAY+gAwIBAgIBBzANBgkqhkiG9w0BAQUFADCBhjETMBEGCgmSJomT8ixk\
         ARkWA0NPTTEXMBUGCgmSJomT8ixkARkWB0VYQU1QTEUxHjAcBgNVBAsTFUNlcnRp\
         ZmljYXRlIEF1dGhvcml0eTEXMBUGA1UEAxMOY2EuZXhhbXBsZS5jb20xHTAbBgkq\
         hkiG9w0BCQEWDmNhQGV4YW1wbGUuY29tMB4XDTExMDExNzE5NDcxN1oXDTIxMDEx\
         NDE5NDcxN1owSzETMBEGCgmSJomT8ixkARkWA0NPTTEXMBUGCgmSJomT8ixkARkW\
         B0VYQU1QTEUxGzAZBgNVBAMTEnNlcnZlci5leGFtcGxlLmNvbTBcMA0GCSqGSIb3\
         DQEBAQUAA0sAMEgCQQDYScTxk55XBmbDM9zzwO+grVySE4rudWuzH2PpObIonqbf\
         hRoAalKVluG9jvbHI81eXxCdSObv1KBP1sbN5RzpAgMBAAGjIjAgMAkGA1UdEwQC\
         MAAwEwYDVR0lBAwwCgYIKwYBBQUHAwEwDQYJKoZIhvcNAQEFBQADgYEAYx6fMqT1\
         Gvo0jq88E8mc+bmp4LfXD4wJ7KxYeadQxt75HFRpj4FhFO3DOpVRFgzHlOEo3Fwk\
         PZOKjvkT0cbcoEq5whLH25dHoQxGoVQgFyAP5s+7Vp5AlHh8Y/vAoXeEVyy/RCIH\
         QkhUlAflfDMcrrYjsmwoOPSjhx6Mm/AopX4=\
         -----END CERTIFICATE-----\
         -----BEGIN PRIVATE KEY-----\
         MIIBVAIBADANBgkqhkiG9w0BAQEFAASCAT4wggE6AgEAAkEA2EnE8ZOeVwZmwzPc\
         88DvoK1ckhOK7nVrsx9j6TmyKJ6m34UaAGpSlZbhvY72xyPNXl8QnUjm79SgT9bG\
         zeUc6QIDAQABAkBRFJZ32VbqWMP9OVwDJLiwC01AlYLnka0mIQZbT/2xq9dUc9GW\
         U3kiVw4lL8v/+sPjtTPCYYdzHHOyDen6znVhAiEA9qJT7BtQvRxCvGrAhr9MS022\
         tTdPbW829BoUtIeH64cCIQDggG5i48v7HPacPBIH1RaSVhXl8qHCpQD3qrIw3FMw\
         DwIga8PqH5Sf5sHedy2+CiK0V4MRfoU4c3zQ6kArI+bEgSkCIQCLA1vXBiE31B5s\
         bdHoYa1BXebfZVd+1Hd95IfEM5mbRwIgSkDuQwV55BBlvWph3U8wVIMIb4GStaH8\
         W535W8UBbEg=-----END PRIVATE KEY-----",
    )?;

    // Bindable versions were added in b1f515178a363df0322d7adbd5754e1f6e2083c9
    // This declares that the user "user" (once authenticated) has a role that
    // allows them to access and construct media factories.
    unsafe {
        gst_rtsp_server_sys::gst_rtsp_media_factory_add_role(
            factory.to_glib_none().0,
            "user".to_glib_none().0,
            gst_rtsp_server::RTSP_PERM_MEDIA_FACTORY_ACCESS
                .to_glib_none()
                .0,
            <bool as StaticType>::static_type().to_glib() as *const u8,
            true.to_glib() as *const u8,
            gst_rtsp_server::RTSP_PERM_MEDIA_FACTORY_CONSTRUCT.as_ptr() as *const u8,
            <bool as StaticType>::static_type().to_glib() as *const u8,
            true.to_glib() as *const u8,
            ptr::null_mut::<u8>(),
        );
    }

    auth.set_tls_certificate(Some(&cert));
    auth.add_basic(basic.as_str(), &token);
    // Here, we tell the RTSP server about the authentication method we
    // configured above.
    server.set_auth(Some(&auth));

    factory.set_launch(args[1].as_str());
    // Tell the RTSP server that we want to work in RECORD mode (clients send)
    // data to us.
    factory.set_transport_mode(gst_rtsp_server::RTSPTransportMode::RECORD);
    // The RTSP protocol allows a couple of different profiles for the actually
    // used protocol of data-transmission. With this, we can limit the selection
    // from which connecting clients have to choose.
    // SAVP/SAVPF are via SRTP (encrypted), that's what the S is for.
    // The F in the end is for feedback (an extension that allows more bidirectional
    // feedback between sender and receiver). AV is just Audio/Video, P is Profile :)
    // The default, old RTP profile is AVP
    factory.set_profiles(gst_rtsp::RTSPProfile::SAVP | gst_rtsp::RTSPProfile::SAVPF);

    // Now we add a new mount-point and tell the RTSP server to use the factory
    // we configured beforehand. This factory will take on the job of creating
    // a pipeline, which will take on the incoming data of connected clients.
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
        "Stream ready at rtsps://127.0.0.1:{}/test",
        server.get_bound_port()
    );

    // Start the mainloop. From this point on, the server will start to take
    // incoming connections from clients.
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
