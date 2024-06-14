// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{translate::*, value::ToSendValue};
use gst::{ffi as gst_ffi, prelude::*, Element, Message, Seqnum};

use crate::ffi;

macro_rules! message_builder_generic_impl {
    ($new_fn:expr) => {
        #[allow(clippy::needless_update)]
        pub fn src<O: IsA<Element> + Cast + Clone>(self, src: &O) -> Self {
            Self {
                builder: self.builder.src(src),
                ..self
            }
        }

        #[allow(clippy::needless_update)]
        pub fn src_if<O: IsA<Element> + Cast + Clone>(self, src: &O, predicate: bool) -> Self {
            if predicate {
                self.src(src)
            } else {
                self
            }
        }

        #[allow(clippy::needless_update)]
        pub fn src_if_some<O: IsA<Element> + Cast + Clone>(self, src: Option<&O>) -> Self {
            if let Some(src) = src {
                self.src(src)
            } else {
                self
            }
        }

        #[doc(alias = "gst_message_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum(self, seqnum: Seqnum) -> Self {
            Self {
                builder: self.builder.seqnum(seqnum),
                ..self
            }
        }

        #[doc(alias = "gst_message_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum_if(self, seqnum: Seqnum, predicate: bool) -> Self {
            if predicate {
                self.seqnum(seqnum)
            } else {
                self
            }
        }

        #[doc(alias = "gst_message_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum_if_some(self, seqnum: Option<Seqnum>) -> Self {
            if let Some(seqnum) = seqnum {
                self.seqnum(seqnum)
            } else {
                self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_message_set_details")]
        pub fn details(self, details: gst::Structure) -> Self {
            Self {
                builder: self.builder.details(details),
                ..self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_message_set_details")]
        pub fn details_if(self, details: gst::Structure, predicate: bool) -> Self {
            if predicate {
                self.details(details)
            } else {
                self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_message_set_details")]
        pub fn details_if_some(self, details: Option<gst::Structure>) -> Self {
            if let Some(details) = details {
                self.details(details)
            } else {
                self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_missing_plugin_message_set_stream_id")]
        pub fn stream_id(self, stream_id: &'a str) -> Self {
            Self {
                stream_id: Some(stream_id),
                ..self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_missing_plugin_message_set_stream_id")]
        pub fn stream_id_if(self, stream_id: &'a str, predicate: bool) -> Self {
            if predicate {
                self.stream_id(stream_id)
            } else {
                self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_missing_plugin_message_set_stream_id")]
        pub fn stream_id_if_some(self, stream_id: Option<&'a str>) -> Self {
            if let Some(stream_id) = stream_id {
                self.stream_id(stream_id)
            } else {
                self
            }
        }

        // rustdoc-stripper-ignore-next
        /// Sets field `name` to the given value `value`.
        ///
        /// Overrides any default or previously defined value for `name`.
        pub fn other_field(self, name: &'a str, value: impl ToSendValue) -> Self {
            Self {
                builder: self.builder.other_field(name, value),
                ..self
            }
        }

        gst::impl_builder_gvalue_extra_setters!(other_field);

        #[deprecated = "use builder.other_field() instead"]
        #[allow(clippy::needless_update)]
        pub fn other_fields(
            self,
            other_fields: &[(&'a str, &'a (dyn ToSendValue + Sync))],
        ) -> Self {
            Self {
                builder: self.builder.other_fields(other_fields),
                ..self
            }
        }

        #[must_use = "Building the message without using it has no effect"]
        #[allow(clippy::redundant_closure_call)]
        pub fn build(mut self) -> Message {
            skip_assert_initialized!();
            unsafe {
                let src = self.builder.src.to_glib_none().0;
                let msg = $new_fn(&mut self, src);
                if let Some(seqnum) = self.builder.seqnum {
                    gst_ffi::gst_message_set_seqnum(msg, seqnum.into_glib());
                }

                #[cfg(feature = "v1_26")]
                if let Some(details) = self.builder.details {
                    gst_ffi::gst_message_set_details(msg, details.into_glib_ptr());
                }

                if !self.builder.other_fields.is_empty() {
                    let structure = gst_ffi::gst_message_writable_structure(msg);

                    if !structure.is_null() {
                        let structure =
                            gst::StructureRef::from_glib_borrow_mut(structure as *mut _);

                        for (k, v) in self.builder.other_fields {
                            structure.set_value(k, v);
                        }
                    }
                }

                from_glib_full(msg)
            }
        }
    };
}

struct MessageBuilder<'a> {
    src: Option<Element>,
    seqnum: Option<Seqnum>,
    #[cfg(feature = "v1_26")]
    details: Option<gst::Structure>,
    other_fields: Vec<(&'a str, glib::SendValue)>,
}

impl<'a> MessageBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            #[cfg(feature = "v1_26")]
            details: None,
            other_fields: Vec::new(),
        }
    }

    fn src<O: IsA<Element> + Cast + Clone>(self, src: &O) -> Self {
        Self {
            src: Some(src.clone().upcast::<Element>()),
            ..self
        }
    }

    fn seqnum(self, seqnum: Seqnum) -> Self {
        Self {
            seqnum: Some(seqnum),
            ..self
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_message_set_details")]
    fn details(self, details: gst::Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    fn other_field(self, name: &'a str, value: impl ToSendValue) -> Self {
        let mut other_fields = self.other_fields;
        other_fields.push((name, value.to_send_value()));

        Self {
            other_fields,
            ..self
        }
    }

    fn other_fields(self, other_fields: &[(&'a str, &'a (dyn ToSendValue + Sync))]) -> Self {
        let mut s = self;

        for (name, value) in other_fields {
            s = s.other_field(name, value.to_send_value());
        }

        s
    }
}

enum MessageBuilderDetail<'a> {
    Decoder(&'a gst::Caps),
    Encoder(&'a gst::Caps),
    Element(&'a str),
    Sink(&'a str),
    Src(&'a str),
}

#[must_use = "The builder must be built to be used"]
pub struct MissingPluginMessageBuilder<'a> {
    builder: MessageBuilder<'a>,
    detail: MessageBuilderDetail<'a>,
    #[cfg(feature = "v1_26")]
    stream_id: Option<&'a str>,
}

impl<'a> MissingPluginMessageBuilder<'a> {
    fn new(detail: MessageBuilderDetail<'a>) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            detail,
            #[cfg(feature = "v1_26")]
            stream_id: None,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| {
        let msg = match s.detail {
            MessageBuilderDetail::Decoder(caps) => {
                ffi::gst_missing_decoder_message_new(src, caps.to_glib_none().0)
            }
            MessageBuilderDetail::Encoder(caps) => {
                ffi::gst_missing_encoder_message_new(src, caps.to_glib_none().0)
            }
            MessageBuilderDetail::Element(name) => {
                ffi::gst_missing_element_message_new(src, name.to_glib_none().0)
            }
            MessageBuilderDetail::Sink(protocol) => {
                ffi::gst_missing_uri_sink_message_new(src, protocol.to_glib_none().0)
            }
            MessageBuilderDetail::Src(protocol) => {
                ffi::gst_missing_uri_source_message_new(src, protocol.to_glib_none().0)
            }
        };

        #[cfg(feature = "v1_26")]
        if let Some(stream_id) = s.stream_id {
            ffi::gst_missing_plugin_message_set_stream_id(msg, stream_id.to_glib_none().0);
        }

        msg
    });
}

#[derive(Clone, Debug)]
pub struct MissingPluginMessage<'a> {
    pub msg: &'a gst::MessageRef,
}

impl<'a> MissingPluginMessage<'a> {
    #[doc(alias = "gst_missing_decoder_message_new")]
    #[allow(clippy::new_ret_no_self)]
    pub fn for_decoder(caps: &'a gst::Caps) -> gst::Message {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Decoder(caps)).build()
    }

    #[doc(alias = "gst_missing_decoder_message_new")]
    pub fn builder_for_decoder(caps: &'a gst::Caps) -> MissingPluginMessageBuilder<'a> {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Decoder(caps))
    }

    #[doc(alias = "gst_missing_encoder_message_new")]
    #[allow(clippy::new_ret_no_self)]
    pub fn for_encoder(caps: &'a gst::Caps) -> gst::Message {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Encoder(caps)).build()
    }

    #[doc(alias = "gst_missing_encoder_message_new")]
    pub fn builder_for_encoder(caps: &'a gst::Caps) -> MissingPluginMessageBuilder<'a> {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Encoder(caps))
    }

    #[doc(alias = "gst_missing_element_message_new")]
    #[allow(clippy::new_ret_no_self)]
    pub fn for_element(name: &'a str) -> gst::Message {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Element(name)).build()
    }

    #[doc(alias = "gst_missing_element_message_new")]
    pub fn builder_for_element(name: &'a str) -> MissingPluginMessageBuilder<'a> {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Element(name))
    }

    #[doc(alias = "gst_missing_uri_source_message_new")]
    #[allow(clippy::new_ret_no_self)]
    pub fn for_uri_source(protocol: &'a str) -> gst::Message {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Src(protocol)).build()
    }

    #[doc(alias = "gst_missing_uri_source_message_new")]
    pub fn builder_for_uri_source(protocol: &'a str) -> MissingPluginMessageBuilder<'a> {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Src(protocol))
    }

    #[doc(alias = "gst_missing_uri_sink_message_new")]
    #[allow(clippy::new_ret_no_self)]
    pub fn for_uri_sink(protocol: &'a str) -> gst::Message {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Sink(protocol)).build()
    }

    #[doc(alias = "gst_missing_uri_sink_message_new")]
    pub fn builder_for_uri_sink(protocol: &'a str) -> MissingPluginMessageBuilder<'a> {
        skip_assert_initialized!();
        MissingPluginMessageBuilder::new(MessageBuilderDetail::Sink(protocol))
    }

    #[doc(alias = "gst_is_missing_plugin_message")]
    pub fn is(msg: &gst::MessageRef) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_is_missing_plugin_message(mut_override(
                msg.as_ptr(),
            )))
        }
    }

    pub fn parse(msg: &'a gst::MessageRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        if Self::is(msg) {
            Ok(MissingPluginMessage { msg })
        } else {
            Err(glib::bool_error!("Invalid missing plugin message"))
        }
    }

    #[doc(alias = "gst_missing_plugin_message_get_description")]
    pub fn description(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_missing_plugin_message_get_description(
                mut_override(self.msg.as_ptr()),
            ))
        }
    }

    #[doc(alias = "gst_missing_plugin_message_get_installer_detail")]
    pub fn installer_detail(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_missing_plugin_message_get_installer_detail(
                mut_override(self.msg.as_ptr()),
            ))
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_missing_plugin_message_get_stream_id")]
    pub fn stream_id(&self) -> Option<&glib::GStr> {
        unsafe {
            let stream_id =
                ffi::gst_missing_plugin_message_get_stream_id(mut_override(self.msg.as_ptr()));
            if stream_id.is_null() {
                None
            } else {
                Some(glib::GStr::from_ptr(stream_id))
            }
        }
    }
}

#[doc(alias = "gst_missing_decoder_installer_detail_new")]
pub fn missing_decoder_installer_detail_new(caps: &gst::Caps) -> glib::GString {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gst_missing_decoder_installer_detail_new(mut_override(
            caps.as_ptr(),
        )))
    }
}

#[doc(alias = "gst_missing_encoder_installer_detail_new")]
pub fn missing_encoder_installer_detail_new(caps: &gst::Caps) -> glib::GString {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gst_missing_encoder_installer_detail_new(mut_override(
            caps.as_ptr(),
        )))
    }
}

#[doc(alias = "gst_missing_element_installer_detail_new")]
pub fn missing_element_installer_detail_new(name: &str) -> glib::GString {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gst_missing_element_installer_detail_new(mut_override(
            name.to_glib_none().0,
        )))
    }
}

#[doc(alias = "gst_missing_uri_source_installer_detail_new")]
pub fn missing_uri_source_installer_detail_new(protocol: &str) -> glib::GString {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gst_missing_uri_source_installer_detail_new(
            mut_override(protocol.to_glib_none().0),
        ))
    }
}

#[doc(alias = "gst_missing_uri_sink_installer_detail_new")]
pub fn missing_uri_sink_installer_detail_new(protocol: &str) -> glib::GString {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gst_missing_uri_sink_installer_detail_new(
            mut_override(protocol.to_glib_none().0),
        ))
    }
}

#[doc(alias = "gst_install_plugins_supported")]
pub fn install_plugins_supported() -> bool {
    skip_assert_initialized!();
    unsafe { from_glib(ffi::gst_install_plugins_supported()) }
}

#[doc(alias = "gst_install_plugins_installation_in_progress")]
pub fn install_plugins_installation_in_progress() -> bool {
    skip_assert_initialized!();
    unsafe { from_glib(ffi::gst_install_plugins_installation_in_progress()) }
}

#[doc(alias = "gst_install_plugins_sync")]
pub fn install_plugins_sync(
    details: &[&str],
    ctx: Option<&crate::InstallPluginsContext>,
) -> crate::InstallPluginsReturn {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gst_install_plugins_sync(
            ToGlibPtr::<*const *mut _>::to_glib_none(&glib::StrV::from(details)).0
                as *const *const _,
            ctx.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_install_plugins_async")]
pub fn install_plugins_async<F: FnOnce(crate::InstallPluginsReturn) + Send + 'static>(
    details: &[&str],
    ctx: Option<&crate::InstallPluginsContext>,
    func: F,
) -> crate::InstallPluginsReturn {
    skip_assert_initialized!();

    let user_data: Box<Option<F>> = Box::new(Some(func));

    unsafe extern "C" fn trampoline<F: FnOnce(crate::InstallPluginsReturn) + Send + 'static>(
        ret: ffi::GstInstallPluginsReturn,
        user_data: glib::ffi::gpointer,
    ) {
        let callback = Box::from_raw(user_data as *mut F);
        callback(from_glib(ret));
    }

    unsafe {
        from_glib(ffi::gst_install_plugins_async(
            ToGlibPtr::<*const *mut _>::to_glib_none(&glib::StrV::from(details)).0
                as *const *const _,
            ctx.to_glib_none().0,
            Some(trampoline::<F>),
            Box::into_raw(user_data) as *mut _,
        ))
    }
}
