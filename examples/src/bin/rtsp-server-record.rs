extern crate failure;
extern crate gio;
extern crate glib;

#[macro_use]
extern crate failure_derive;

extern crate gstreamer as gst;
extern crate gstreamer_rtsp as gst_rtsp;
extern crate gstreamer_rtsp_server as gst_rtsp_server;
extern crate gstreamer_rtsp_server_sys as ffi;

use failure::Error;
use std::env;
use std::ptr;

use glib::translate::*;
use gst_rtsp::*;
use gst_rtsp_server::prelude::*;
use gst_rtsp_server::*;

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Fail)]
#[fail(display = "Could not get mount points")]
struct NoMountPoints;

#[derive(Debug, Fail)]
#[fail(display = "Usage: {} LAUNCH_LINE", _0)]
struct UsageError(String);

fn main_loop() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::from(UsageError(args[0].clone())));
    }

    let main_loop = glib::MainLoop::new(None, false);
    let server = RTSPServer::new();
    let factory = RTSPMediaFactory::new();
    let mounts = server.get_mount_points().ok_or(NoMountPoints)?;
    let auth = RTSPAuth::new();
    let token = RTSPToken::new(&[(*RTSP_TOKEN_MEDIA_FACTORY_ROLE, &"user")]);
    let basic = RTSPAuth::make_basic("user", "password");
    let cert = gio::TlsCertificate::new_from_pem(
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
    unsafe {
        ffi::gst_rtsp_media_factory_add_role(
            factory.to_glib_none().0,
            "user".to_glib_none().0,
            RTSP_PERM_MEDIA_FACTORY_ACCESS.to_glib_none().0,
            <bool as StaticType>::static_type().to_glib() as *const u8,
            true.to_glib() as *const u8,
            RTSP_PERM_MEDIA_FACTORY_CONSTRUCT.as_ptr() as *const u8,
            <bool as StaticType>::static_type().to_glib() as *const u8,
            true.to_glib() as *const u8,
            ptr::null_mut::<u8>(),
        );
    }

    auth.set_tls_certificate(&cert);
    auth.add_basic(basic.as_str(), &token);
    server.set_auth(&auth);
    factory.set_launch(args[1].as_str());
    factory.set_transport_mode(RTSPTransportMode::RECORD);
    factory.set_profiles(RTSPProfile::SAVP | RTSPProfile::SAVPF);

    mounts.add_factory("/test", &factory);

    server.attach(None);

    println!(
        "Stream ready at rtsps://127.0.0.1:{}/test",
        server.get_bound_port()
    );

    main_loop.run();

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
