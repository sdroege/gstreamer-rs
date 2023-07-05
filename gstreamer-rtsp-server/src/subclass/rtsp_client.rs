// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::RTSPClient;

pub trait RTSPClientImpl: RTSPClientImplExt + ObjectImpl + Send + Sync {
    fn create_sdp(&self, media: &crate::RTSPMedia) -> Option<gst_sdp::SDPMessage> {
        self.parent_create_sdp(media)
    }

    fn configure_client_media(
        &self,
        media: &crate::RTSPMedia,
        stream: &crate::RTSPStream,
        ctx: &crate::RTSPContext,
    ) -> Result<(), gst::LoggableError> {
        self.parent_configure_client_media(media, stream, ctx)
    }

    // TODO: configure_client_transport

    fn params_set(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPResult {
        self.parent_params_set(ctx)
    }

    fn params_get(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPResult {
        self.parent_params_get(ctx)
    }

    fn make_path_from_uri(&self, url: &gst_rtsp::RTSPUrl) -> Option<glib::GString> {
        self.parent_make_path_from_uri(url)
    }

    fn closed(&self) {
        self.parent_closed();
    }

    fn new_session(&self, session: &crate::RTSPSession) {
        self.parent_new_session(session);
    }

    fn options_request(&self, ctx: &crate::RTSPContext) {
        self.parent_options_request(ctx);
    }

    fn describe_request(&self, ctx: &crate::RTSPContext) {
        self.parent_describe_request(ctx);
    }

    fn setup_request(&self, ctx: &crate::RTSPContext) {
        self.parent_setup_request(ctx);
    }

    fn play_request(&self, ctx: &crate::RTSPContext) {
        self.parent_play_request(ctx);
    }

    fn pause_request(&self, ctx: &crate::RTSPContext) {
        self.parent_pause_request(ctx);
    }

    fn teardown_request(&self, ctx: &crate::RTSPContext) {
        self.parent_teardown_request(ctx);
    }

    fn set_parameter_request(&self, ctx: &crate::RTSPContext) {
        self.parent_set_parameter_request(ctx);
    }

    fn parameter_request(&self, ctx: &crate::RTSPContext) {
        self.parent_parameter_request(ctx);
    }

    fn announce_request(&self, ctx: &crate::RTSPContext) {
        self.parent_announce_request(ctx);
    }

    fn record_request(&self, ctx: &crate::RTSPContext) {
        self.parent_record_request(ctx);
    }

    fn handle_response(&self, ctx: &crate::RTSPContext) {
        self.parent_handle_response(ctx);
    }

    // TODO: tunnel_http_response
    // TODO: send_message

    fn handle_sdp(
        &self,
        ctx: &crate::RTSPContext,
        media: &crate::RTSPMedia,
        sdp: &gst_sdp::SDPMessageRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_handle_sdp(ctx, media, sdp)
    }

    fn check_requirements(
        &self,
        ctx: &crate::RTSPContext,
        arr: &[String],
    ) -> Option<glib::GString> {
        self.parent_check_requirements(ctx, arr)
    }

    fn pre_options_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_options_request(ctx)
    }

    fn pre_describe_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_describe_request(ctx)
    }

    fn pre_setup_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_setup_request(ctx)
    }

    fn pre_play_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_play_request(ctx)
    }

    fn pre_pause_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_pause_request(ctx)
    }

    fn pre_teardown_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_teardown_request(ctx)
    }

    fn pre_set_parameter_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_set_parameter_request(ctx)
    }

    fn pre_get_parameter_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_get_parameter_request(ctx)
    }

    fn pre_announce_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_announce_request(ctx)
    }

    fn pre_record_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        self.parent_pre_record_request(ctx)
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    fn adjust_error_code(
        &self,
        ctx: &crate::RTSPContext,
        status_code: gst_rtsp::RTSPStatusCode,
    ) -> gst_rtsp::RTSPStatusCode {
        self.parent_adjust_error_code(ctx, status_code)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::RTSPClientImplExt> Sealed for T {}
}

pub trait RTSPClientImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_create_sdp(&self, media: &crate::RTSPMedia) -> Option<gst_sdp::SDPMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .create_sdp
                .expect("No `create_rtpbin` virtual method implementation in parent class");

            from_glib_full(f(
                self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                media.to_glib_none().0,
            ))
        }
    }

    fn parent_configure_client_media(
        &self,
        media: &crate::RTSPMedia,
        stream: &crate::RTSPStream,
        ctx: &crate::RTSPContext,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class).configure_client_media.expect(
                "No `configure_client_media` virtual method implementation in parent class",
            );

            gst::result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
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

    fn parent_params_set(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPResult {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .params_set
                .expect("No `params_set` virtual method implementation in parent class");

            from_glib(f(
                self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                ctx.to_glib_none().0,
            ))
        }
    }

    fn parent_params_get(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPResult {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .params_get
                .expect("No `params_get` virtual method implementation in parent class");

            from_glib(f(
                self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                ctx.to_glib_none().0,
            ))
        }
    }

    fn parent_make_path_from_uri(&self, url: &gst_rtsp::RTSPUrl) -> Option<glib::GString> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .make_path_from_uri
                .expect("No `make_path_from_uri` virtual method implementation in parent class");

            from_glib_full(f(
                self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                url.to_glib_none().0,
            ))
        }
    }

    fn parent_closed(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).closed {
                f(self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0);
            }
        }
    }

    fn parent_new_session(&self, session: &crate::RTSPSession) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).new_session {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    session.to_glib_none().0,
                );
            }
        }
    }

    fn parent_options_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).options_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_describe_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).describe_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_setup_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).setup_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_play_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).play_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_pause_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pause_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_teardown_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).teardown_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_set_parameter_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).set_parameter_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_parameter_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).get_parameter_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_announce_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).announce_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_record_request(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).record_request {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    fn parent_handle_response(&self, ctx: &crate::RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).handle_response {
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                );
            }
        }
    }

    // TODO: tunnel_http_response
    // TODO: send_message

    fn parent_handle_sdp(
        &self,
        ctx: &crate::RTSPContext,
        media: &crate::RTSPMedia,
        sdp: &gst_sdp::SDPMessageRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            let f = (*parent_class)
                .handle_sdp
                .expect("No `handle_sdp` virtual method implementation in parent class");

            gst::result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
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
        ctx: &crate::RTSPContext,
        arr: &[String],
    ) -> Option<glib::GString> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).check_requirements {
                from_glib_full(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                    arr.to_glib_none().0,
                ))
            } else {
                None
            }
        }
    }

    fn parent_pre_options_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_options_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_describe_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_describe_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_setup_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_setup_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_play_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_play_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_pause_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_pause_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_teardown_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_teardown_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_set_parameter_request(
        &self,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_set_parameter_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_get_parameter_request(
        &self,
        ctx: &crate::RTSPContext,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_get_parameter_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_announce_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_announce_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    fn parent_pre_record_request(&self, ctx: &crate::RTSPContext) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).pre_record_request {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    fn parent_adjust_error_code(
        &self,
        ctx: &crate::RTSPContext,
        status_code: gst_rtsp::RTSPStatusCode,
    ) -> gst_rtsp::RTSPStatusCode {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPClientClass;
            if let Some(f) = (*parent_class).adjust_error_code {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPClient>().to_glib_none().0,
                    ctx.to_glib_none().0,
                    status_code.into_glib(),
                ))
            } else {
                gst_rtsp::RTSPStatusCode::Ok
            }
        }
    }
}

impl<T: RTSPClientImpl> RTSPClientImplExt for T {}

unsafe impl<T: RTSPClientImpl> IsSubclassable<T> for RTSPClient {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();

        // There was unintentional ABI breakage in 1.18 so let's work around that
        // for now by casting to the old struct layout.
        #[cfg(not(feature = "v1_18"))]
        {
            if gst::version() < (1, 18, 0, 0) {
                #[derive(Copy, Clone)]
                #[repr(C)]
                pub struct CompatClass {
                    pub parent_class: glib::gobject_ffi::GObjectClass,
                    pub create_sdp: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPMedia,
                        )
                            -> *mut gst_sdp::ffi::GstSDPMessage,
                    >,
                    pub configure_client_media: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPMedia,
                            *mut ffi::GstRTSPStream,
                            *mut ffi::GstRTSPContext,
                        ) -> glib::ffi::gboolean,
                    >,
                    pub configure_client_transport: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                            *mut gst_rtsp::ffi::GstRTSPTransport,
                        ) -> glib::ffi::gboolean,
                    >,
                    pub params_set: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPResult,
                    >,
                    pub params_get: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPResult,
                    >,
                    pub make_path_from_uri: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *const gst_rtsp::ffi::GstRTSPUrl,
                        ) -> *mut libc::c_char,
                    >,
                    pub closed: Option<unsafe extern "C" fn(*mut ffi::GstRTSPClient)>,
                    pub new_session: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPSession),
                    >,
                    pub options_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub describe_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub setup_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub play_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub pause_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub teardown_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub set_parameter_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub get_parameter_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub handle_response: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub tunnel_http_response: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut gst_rtsp::ffi::GstRTSPMessage,
                            *mut gst_rtsp::ffi::GstRTSPMessage,
                        ),
                    >,
                    pub send_message: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                            *mut gst_rtsp::ffi::GstRTSPMessage,
                        ),
                    >,
                    pub handle_sdp: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                            *mut ffi::GstRTSPMedia,
                            *mut gst_sdp::ffi::GstSDPMessage,
                        ) -> glib::ffi::gboolean,
                    >,
                    pub announce_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub record_request: Option<
                        unsafe extern "C" fn(*mut ffi::GstRTSPClient, *mut ffi::GstRTSPContext),
                    >,
                    pub check_requirements: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                            *mut *mut libc::c_char,
                        ) -> *mut libc::c_char,
                    >,
                    pub pre_options_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_describe_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_setup_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_play_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_pause_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_teardown_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_set_parameter_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_get_parameter_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_announce_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub pre_record_request: Option<
                        unsafe extern "C" fn(
                            *mut ffi::GstRTSPClient,
                            *mut ffi::GstRTSPContext,
                        )
                            -> gst_rtsp::ffi::GstRTSPStatusCode,
                    >,
                    pub _gst_reserved: [glib::ffi::gpointer; 4],
                }

                let klass = unsafe {
                    std::mem::transmute::<&mut ffi::GstRTSPClientClass, &mut CompatClass>(klass)
                };

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

                return;
            }
        }

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
        #[cfg(feature = "v1_22")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
        {
            klass.adjust_error_code = Some(client_adjust_error_code::<T>);
        }
    }
}

unsafe extern "C" fn client_create_sdp<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    media: *mut ffi::GstRTSPMedia,
) -> *mut gst_sdp::ffi::GstSDPMessage {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.create_sdp(&from_glib_borrow(media)).into_glib_ptr()
}

unsafe extern "C" fn client_configure_client_media<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    media: *mut ffi::GstRTSPMedia,
    stream: *mut ffi::GstRTSPStream,
    ctx: *mut ffi::GstRTSPContext,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.configure_client_media(
        &from_glib_borrow(media),
        &from_glib_borrow(stream),
        &from_glib_borrow(ctx),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn client_params_set<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPResult {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.params_set(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_params_get<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPResult {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.params_get(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_make_path_from_uri<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    url: *const gst_rtsp::ffi::GstRTSPUrl,
) -> *mut std::os::raw::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.make_path_from_uri(&from_glib_borrow(url))
        .into_glib_ptr()
}

unsafe extern "C" fn client_closed<T: RTSPClientImpl>(ptr: *mut ffi::GstRTSPClient) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.closed();
}

unsafe extern "C" fn client_new_session<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    session: *mut ffi::GstRTSPSession,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.new_session(&from_glib_borrow(session));
}

unsafe extern "C" fn client_options_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.options_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_describe_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.describe_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_setup_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.setup_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_play_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.play_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_pause_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pause_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_teardown_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.teardown_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_set_parameter_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.set_parameter_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_get_parameter_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.parameter_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_announce_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.announce_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_record_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.record_request(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_handle_response<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.handle_response(&from_glib_borrow(ctx));
}

unsafe extern "C" fn client_handle_sdp<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
    media: *mut ffi::GstRTSPMedia,
    sdp: *mut gst_sdp::ffi::GstSDPMessage,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.handle_sdp(
        &from_glib_borrow(ctx),
        &from_glib_borrow(media),
        &*(sdp as *mut _),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
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
    let imp = instance.imp();

    imp.check_requirements(&from_glib_borrow(ctx), Vec::from_glib_none(arr).as_slice())
        .into_glib_ptr()
}

unsafe extern "C" fn client_pre_options_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_options_request(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_pre_describe_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_describe_request(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_pre_setup_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_setup_request(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_pre_play_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_play_request(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_pre_pause_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_pause_request(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_pre_teardown_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_teardown_request(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_pre_set_parameter_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_set_parameter_request(&from_glib_borrow(ctx))
        .into_glib()
}

unsafe extern "C" fn client_pre_get_parameter_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_get_parameter_request(&from_glib_borrow(ctx))
        .into_glib()
}

unsafe extern "C" fn client_pre_announce_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_announce_request(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn client_pre_record_request<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pre_record_request(&from_glib_borrow(ctx)).into_glib()
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
unsafe extern "C" fn client_adjust_error_code<T: RTSPClientImpl>(
    ptr: *mut ffi::GstRTSPClient,
    ctx: *mut ffi::GstRTSPContext,
    status_code: gst_rtsp::ffi::GstRTSPStatusCode,
) -> gst_rtsp::ffi::GstRTSPStatusCode {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.adjust_error_code(&from_glib_borrow(ctx), from_glib(status_code))
        .into_glib()
}
