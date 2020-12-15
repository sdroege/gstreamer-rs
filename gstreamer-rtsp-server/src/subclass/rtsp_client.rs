// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use std::mem;

use crate::RTSPClient;

pub trait RTSPClientImpl: RTSPClientImplExt + ObjectImpl + Send + Sync {
    fn create_sdp(
        &self,
        client: &Self::Type,
        media: &crate::RTSPMedia,
    ) -> Option<gst_sdp::SDPMessage> {
        self.parent_create_sdp(client, media)
    }

    fn configure_client_media(
        &self,
        client: &Self::Type,
        media: &crate::RTSPMedia,
        stream: &crate::RTSPStream,
        ctx: &crate::RTSPContext,
    ) -> Result<(), gst::LoggableError> {
        self.parent_configure_client_media(client, media, stream, ctx)
    }

    // TODO: configure_client_transport

    fn params_set(&self, client: &Self::Type, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPResult {
        self.parent_params_set(client, ctx)
    }

    fn params_get(&self, client: &Self::Type, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPResult {
        self.parent_params_get(client, ctx)
    }

    fn make_path_from_uri(
        &self,
        client: &Self::Type,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString> {
        self.parent_make_path_from_uri(client, url)
    }

    fn closed(&self, client: &Self::Type) {
        self.parent_closed(client);
    }

    fn new_session(&self, client: &Self::Type, session: &crate::RTSPSession) {
        self.parent_new_session(client, session);
    }

    fn options_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_options_request(client, ctx);
    }

    fn describe_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_describe_request(client, ctx);
    }

    fn setup_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_setup_request(client, ctx);
    }

    fn play_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_play_request(client, ctx);
    }

    fn pause_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_pause_request(client, ctx);
    }

    fn teardown_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_teardown_request(client, ctx);
    }

    fn set_parameter_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_set_parameter_request(client, ctx);
    }

    fn get_parameter_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_get_parameter_request(client, ctx);
    }

    fn announce_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_announce_request(client, ctx);
    }

    fn record_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_record_request(client, ctx);
    }

    fn handle_response(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        self.parent_handle_response(client, ctx);
    }

    // TODO: tunnel_http_response
    // TODO: send_message

    fn handle_sdp(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
        media: &crate::RTSPMedia,
        sdp: &gst_sdp::SDPMessageRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_handle_sdp(client, ctx, media, sdp)
    }

    fn check_requirements(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
        arr: &[String],
    ) -> Option<glib::GString> {
        self.parent_check_requirements(client, ctx, arr)
    }

    fn pre_options_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_options_request(client, ctx)
    }

    fn pre_describe_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_describe_request(client, ctx)
    }

    fn pre_setup_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_setup_request(client, ctx)
    }

    fn pre_play_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_play_request(client, ctx)
    }

    fn pre_pause_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_pause_request(client, ctx)
    }

    fn pre_teardown_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_teardown_request(client, ctx)
    }

    fn pre_set_parameter_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_set_parameter_request(client, ctx)
    }

    fn pre_get_parameter_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_get_parameter_request(client, ctx)
    }

    fn pre_announce_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_announce_request(client, ctx)
    }

    fn pre_record_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_record_request(client, ctx)
    }
}

pub trait RTSPClientImplExt: ObjectSubclass {
    fn parent_create_sdp(
        &self,
        client: &Self::Type,
        media: &crate::RTSPMedia,
    ) -> Option<gst_sdp::SDPMessage>;

    fn parent_configure_client_media(
        &self,
        client: &Self::Type,
        media: &crate::RTSPMedia,
        stream: &crate::RTSPStream,
        ctx: &crate::RTSPContext,
    ) -> Result<(), gst::LoggableError>;

    // TODO: configure_client_transport

    fn parent_params_set(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPResult;

    fn parent_params_get(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPResult;

    fn parent_make_path_from_uri(
        &self,
        client: &Self::Type,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString>;

    fn parent_closed(&self, client: &Self::Type);

    fn parent_new_session(&self, client: &Self::Type, session: &crate::RTSPSession);

    fn parent_options_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_describe_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_setup_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_play_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_pause_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_teardown_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_set_parameter_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_get_parameter_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_announce_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_record_request(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    fn parent_handle_response(&self, client: &Self::Type, ctx: &crate::RTSPContext);

    // TODO: tunnel_http_response
    // TODO: send_message

    fn parent_handle_sdp(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
        media: &crate::RTSPMedia,
        sdp: &gst_sdp::SDPMessageRef,
    ) -> Result<(), gst::LoggableError>;

    fn parent_check_requirements(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
        arr: &[String],
    ) -> Option<glib::GString>;

    fn parent_pre_options_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_describe_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_setup_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_play_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_pause_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_teardown_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_set_parameter_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_get_parameter_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_announce_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;

    fn parent_pre_record_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode;
}

impl<T: RTSPClientImpl> RTSPClientImplExt for T {
    fn parent_create_sdp(
        &self,
        client: &Self::Type,
        media: &crate::RTSPMedia,
    ) -> Option<gst_sdp::SDPMessage> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .create_sdp
                .expect("No `create_rtpbin` virtual method implementation in parent class");

            from_glib_full(f(
                client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                media.to_glib_none().0,
            ))
        }
    }

    fn parent_configure_client_media(
        &self,
        client: &Self::Type,
        media: &crate::RTSPMedia,
        stream: &crate::RTSPStream,
        ctx: &crate::RTSPContext,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class).configure_client_media.expect(
                "No `configure_client_media` virtual method implementation in parent class",
            );

            gst::gst_result_from_gboolean!(
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    media.to_glib_none().0,
                    stream.to_glib_none().0,
                    ctx.to_glib_none().0
                ),
                gst::CAT_RUST,
                "Parent function `configure_client_media` failed"
            )
        }
    }

    // TODO: configure_client_transport

    fn parent_params_set(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPResult {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .params_set
                .expect("No `params_set` virtual method implementation in parent class");

            from_glib(f(
                client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                ctx.to_glib_none().0,
            ))
        }
    }

    fn parent_params_get(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPResult {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .params_get
                .expect("No `params_get` virtual method implementation in parent class");

            from_glib(f(
                client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                ctx.to_glib_none().0,
            ))
        }
    }

    fn parent_make_path_from_uri(
        &self,
        client: &Self::Type,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .make_path_from_uri
                .expect("No `make_path_from_uri` virtual method implementation in parent class");

            from_glib_full(f(
                client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                url.to_glib_none().0,
            ))
        }
    }

    fn parent_closed(&self, client: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).closed {
                f(client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0);
            }
        }
    }

    fn parent_new_session(&self, client: &Self::Type, session: &crate::RTSPSession) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).new_session {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    session.to_glib_none().0,
                );
            }
        }
    }

    fn parent_options_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).options_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_describe_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).describe_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_setup_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).setup_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_play_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).play_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_pause_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pause_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_teardown_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).teardown_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_set_parameter_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).set_parameter_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_get_parameter_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).get_parameter_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_announce_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).announce_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_record_request(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).record_request {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_handle_response(&self, client: &Self::Type, ctx: &crate::RTSPContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).handle_response {
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    // TODO: tunnel_http_response
    // TODO: send_message

    fn parent_handle_sdp(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
        media: &crate::RTSPMedia,
        sdp: &gst_sdp::SDPMessageRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .handle_sdp
                .expect("No `handle_sdp` virtual method implementation in parent class");

            gst::gst_result_from_gboolean!(
                f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                    media.to_glib_none().0,
                    sdp as *const _ as *mut _
                ),
                gst::CAT_RUST,
                "Parent function `handle_sdp` failed"
            )
        }
    }

    fn parent_check_requirements(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
        arr: &[String],
    ) -> Option<glib::GString> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).check_requirements {
                from_glib_full(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                    arr.to_glib_none().0,
                ))
            } else {
                None
            }
        }
    }

    fn parent_pre_options_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_options_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_describe_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_describe_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_setup_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_setup_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_play_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_play_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_pause_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_pause_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_teardown_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_teardown_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_set_parameter_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_set_parameter_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_get_parameter_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_get_parameter_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_announce_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_announce_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_record_request(
        &self,
        client: &Self::Type,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_record_request {
                from_glib(f(
                    client.unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }
}
unsafe impl<T: RTSPClientImpl> IsSubclassable<T> for RTSPClient {
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::override_vfuncs(klass);
        let klass = klass.as_mut();
        klass.create_sdp = Some(client_create_sdp::<T>);
        klass.configure_client_media = Some(client_configure_client_media::<T>);
        klass.params_set = Some(client_params_set::<T>);
        klass.params_get = Some(client_params_get::<T>);
        klass.make_path_from_uri = Some(client_make_path_from_uri::<T>);
        klass.closed = Some(client_closed::<T>);
        klass.new_session = Some(client_new_session::<T>);
        klass.options_request = Some(client_options_request::<T>);
        klass.describe_request = Some(client_describe_request::<T>);
        klass.setup_request = Some(client_setup_request::<T>);
        klass.play_request = Some(client_play_request::<T>);
        klass.pause_request = Some(client_pause_request::<T>);
        klass.teardown_request = Some(client_teardown_request::<T>);
        klass.set_parameter_request = Some(client_set_parameter_request::<T>);
        klass.get_parameter_request = Some(client_get_parameter_request::<T>);
        klass.announce_request = Some(client_announce_request::<T>);
        klass.record_request = Some(client_record_request::<T>);
        klass.handle_response = Some(client_handle_response::<T>);
        klass.handle_sdp = Some(client_handle_sdp::<T>);
        klass.check_requirements = Some(client_check_requirements::<T>);
        klass.pre_options_request = Some(client_pre_options_request::<T>);
        klass.pre_describe_request = Some(client_pre_describe_request::<T>);
        klass.pre_setup_request = Some(client_pre_setup_request::<T>);
        klass.pre_play_request = Some(client_pre_play_request::<T>);
        klass.pre_pause_request = Some(client_pre_pause_request::<T>);
        klass.pre_teardown_request = Some(client_pre_teardown_request::<T>);
        klass.pre_set_parameter_request = Some(client_pre_set_parameter_request::<T>);
        klass.pre_get_parameter_request = Some(client_pre_get_parameter_request::<T>);
        klass.pre_announce_request = Some(client_pre_announce_request::<T>);
        klass.pre_record_request = Some(client_pre_record_request::<T>);
    }
}

unsafe extern "C" fn client_create_sdp<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    media: *mut ffi::GstRTSPMedia,
) -> *mut gst_sdp::ffi::GstSDPMessage {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    let sdp =
        mem::ManuallyDrop::new(imp.create_sdp(wrap.unsafe_cast_ref(), &from_glib_borrow(media)));
    let ptr = sdp.to_glib_none().0;

    ptr as *mut _
}

unsafe extern "C" fn client_configure_client_media<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    media: *mut ffi::GstRTSPMedia,
    stream: *mut ffi::GstRTSPStream,
    ctx: *mut ffi::GstRTSPContext,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    match imp.configure_client_media(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(media),
        &from_glib_borrow(stream),
        &from_glib_borrow(ctx),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn client_params_set<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPResult {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.params_set(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_params_get<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPResult {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.params_get(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_make_path_from_uri<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    url: *const gst_rtsp::ffi::GstRTSPUrl,
) -> *mut std::os::raw::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.make_path_from_uri(wrap.unsafe_cast_ref(), &from_glib_borrow(url))
        .to_glib_full()
}

unsafe extern "C" fn client_closed<T: RTSPClientImpl>(ptr: *mut ffi::GstRTSPClient) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.closed(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn client_new_session<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    session: *mut ffi::GstRTSPSession,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.new_session(wrap.unsafe_cast_ref(), &from_glib_borrow(session));
}

unsafe extern "C" fn client_options_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.options_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_describe_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.describe_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_setup_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.setup_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_play_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.play_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_pause_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pause_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_teardown_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.teardown_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_set_parameter_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.set_parameter_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_get_parameter_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.get_parameter_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_announce_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.announce_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_record_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.record_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_handle_response<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.handle_response(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx));
}

unsafe extern "C" fn client_handle_sdp<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
    media: *mut ffi::GstRTSPMedia,
    sdp: *mut gst_sdp::ffi::GstSDPMessage,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    match imp.handle_sdp(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(ctx),
        &from_glib_borrow(media),
        &*(sdp as *mut _),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn client_check_requirements<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
    arr: *mut *mut std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.check_requirements(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(ctx),
        Vec::from_glib_none(arr).as_slice(),
    )
    .to_glib_full()
}

unsafe extern "C" fn client_pre_options_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_options_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_describe_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_describe_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_setup_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_setup_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_play_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_play_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_pause_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_pause_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_teardown_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_teardown_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_set_parameter_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_set_parameter_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_get_parameter_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_get_parameter_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_announce_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_announce_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}

unsafe extern "C" fn client_pre_record_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPClient> = from_glib_borrow(ptr);

    imp.pre_record_request(wrap.unsafe_cast_ref(), &from_glib_borrow(ctx))
        .to_glib()
}
