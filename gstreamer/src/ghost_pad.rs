// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{
    prelude::*, FlowError, FlowSuccess, GhostPad, LoggableError, Pad, PadBuilder, PadFlags,
    PadGetRangeSuccess, PadMode, StaticPadTemplate,
};

impl GhostPad {
    #[doc(alias = "gst_ghost_pad_activate_mode_default")]
    pub fn activate_mode_default<P: IsA<GhostPad>>(
        pad: &P,
        parent: Option<&impl IsA<crate::Object>>,
        mode: PadMode,
        active: bool,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_ghost_pad_activate_mode_default(
                    pad.to_glib_none().0 as *mut ffi::GstPad,
                    parent.map(|p| p.as_ref()).to_glib_none().0,
                    mode.into_glib(),
                    active.into_glib(),
                ),
                "Failed to invoke the default activate mode function of the ghost pad"
            )
        }
    }

    #[doc(alias = "gst_ghost_pad_internal_activate_mode_default")]
    pub fn internal_activate_mode_default<P: IsA<GhostPad>>(
        pad: &P,
        parent: Option<&impl IsA<crate::Object>>,
        mode: PadMode,
        active: bool,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_ghost_pad_internal_activate_mode_default(
                    pad.to_glib_none().0 as *mut ffi::GstPad,
                    parent.map(|p| p.as_ref()).to_glib_none().0,
                    mode.into_glib(),
                    active.into_glib(),
                ),
                concat!(
                    "Failed to invoke the default activate mode function of a proxy pad ",
                    "that is owned by the ghost pad"
                )
            )
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] with an automatically generated name.
    ///
    /// Use [`GhostPad::builder_from_template()`] to get a [`PadBuilder`](crate::PadBuilder)
    /// and define options.
    #[doc(alias = "gst_ghost_pad_new_no_target")]
    pub fn new(direction: crate::PadDirection) -> Self {
        skip_assert_initialized!();
        Self::builder(direction).build()
    }

    // rustdoc-stripper-ignore-next
    /// Creates a [`PadBuilder`](crate::PadBuilder) for a [`PadBuilder`] with an automatically generated name.
    ///
    /// Use [`PadBuilder::name`](crate::PadBuilder::name) or [`PadBuilder::maybe_name`](crate::PadBuilder::maybe_name)
    /// to specify a different name.
    #[doc(alias = "gst_ghost_pad_new_no_target")]
    pub fn builder(direction: crate::PadDirection) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::new(direction)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] from the [`StaticPadTemplate`](crate::StaticPadTemplate).
    ///
    /// If the [`StaticPadTemplate`](crate::StaticPadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `GhostPad` will automatically be named after the `name_template`.
    ///
    /// # Panics
    ///
    /// Panics if the `name_template` is a wildcard-name.
    ///
    /// Use [`GhostPad::builder_from_template()`] to get a [`PadBuilder`](crate::PadBuilder)
    /// and define options.
    #[doc(alias = "gst_ghost_pad_new_no_target_from_static_template")]
    pub fn from_static_template(templ: &StaticPadTemplate) -> Self {
        skip_assert_initialized!();
        Self::builder_from_static_template(templ).build()
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`PadBuilder`](crate::PadBuilder) for a [`GhostPad`] from the [`StaticPadTemplate`](crate::StaticPadTemplate).
    ///
    /// If the [`StaticPadTemplate`](crate::StaticPadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `GhostPad` will automatically be named after the `name_template`.
    ///
    /// Use [`PadBuilder::name`](crate::PadBuilder::name) or [`PadBuilder::maybe_name`](crate::PadBuilder::maybe_name)
    /// to specify a different name.
    #[doc(alias = "gst_ghost_pad_new_no_target_from_static_template")]
    pub fn builder_from_static_template(templ: &StaticPadTemplate) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_static_template(templ)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] from the [`PadTemplate`](crate::PadTemplate).
    ///
    /// If the [`PadTemplate`](crate::PadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `GhostPad` will automatically be named after the `name_template`.
    ///
    /// # Panics
    ///
    /// Panics if the `name_template` is a wildcard-name.
    ///
    /// Use [`GhostPad::builder_from_template()`] to get a [`PadBuilder`](crate::PadBuilder)
    /// and define options.
    #[doc(alias = "gst_ghost_pad_new_no_target_from_template")]
    pub fn from_template(templ: &crate::PadTemplate) -> Self {
        skip_assert_initialized!();
        Self::builder_from_template(templ).build()
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`PadBuilder`](crate::PadBuilder) for a [`GhostPad`] from the [`PadTemplate`](crate::PadTemplate).
    ///
    /// If the [`PadTemplate`](crate::PadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `GhostPad` will automatically be named after the `name_template`.
    ///
    /// Use [`PadBuilder::name`](crate::PadBuilder::name) or [`PadBuilder::maybe_name`](crate::PadBuilder::maybe_name)
    /// to specify a different name.
    #[doc(alias = "gst_ghost_pad_new_no_target_from_template")]
    pub fn builder_from_template(templ: &crate::PadTemplate) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_template(templ)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] from the specified `target` `Pad`.
    ///
    /// The `GhostPad` will automatically be named after the `target` `name`.
    ///
    /// Use [`GhostPad::builder_with_target()`] to get a [`PadBuilder`](crate::PadBuilder)
    /// and define options.
    #[doc(alias = "gst_ghost_pad_new")]
    pub fn with_target<P: IsA<Pad> + IsA<crate::Object>>(
        target: &P,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();
        Ok(Self::builder_with_target(target)?.build())
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`PadBuilder`](crate::PadBuilder) for a [`GhostPad`] from the specified `target` `Pad`.
    ///
    /// The `GhostPad` will automatically be named after the `target` `name`.
    ///
    /// Use [`PadBuilder::name`](crate::PadBuilder::name) or [`PadBuilder::maybe_name`](crate::PadBuilder::maybe_name)
    /// to specify a different name.
    #[doc(alias = "gst_ghost_pad_new_no_target_from_template")]
    pub fn builder_with_target<P: IsA<Pad> + IsA<crate::Object>>(
        target: &P,
    ) -> Result<PadBuilder<Self>, glib::BoolError> {
        skip_assert_initialized!();
        let mut builder = Self::builder(target.direction());
        builder.needs_specific_name = true;
        builder.with_target(target)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] from the [`PadTemplate`](crate::PadTemplate)
    /// with the specified `target` `Pad`.
    ///
    /// If the [`PadTemplate`](crate::PadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `GhostPad` will automatically be named after the `name_template`.
    ///
    /// If the `name_template` is a wildcard-name, then the `target` `name` is used,
    /// if it is compatible. Otherwise, a specific name must be provided using
    /// [`PadBuilder::name`](crate::PadBuilder::name) or [`PadBuilder::maybe_name`](crate::PadBuilder::maybe_name).
    #[doc(alias = "gst_ghost_pad_new_from_template")]
    pub fn from_template_with_target<P: IsA<Pad> + IsA<crate::Object>>(
        templ: &crate::PadTemplate,
        target: &P,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();
        Ok(Self::builder_from_template_with_target(templ, target)?.build())
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`PadBuilder`](crate::PadBuilder) for a [`GhostPad`] from the [`PadTemplate`](crate::PadTemplate)
    /// with the specified `target` `Pad`.
    ///
    /// If the [`PadTemplate`](crate::PadTemplate) has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `GhostPad` will automatically be named after the `name_template`.
    ///
    /// If the `name_template` is a wildcard-name, then the `target` `name` is used,
    /// if it is compatible. Otherwise, a specific name must be provided using
    /// [`PadBuilder::name`](crate::PadBuilder::name) or [`PadBuilder::maybe_name`](crate::PadBuilder::maybe_name).
    #[doc(alias = "gst_ghost_pad_new_from_template")]
    pub fn builder_from_template_with_target<P: IsA<Pad> + IsA<crate::Object>>(
        templ: &crate::PadTemplate,
        target: &P,
    ) -> Result<PadBuilder<Self>, glib::BoolError> {
        skip_assert_initialized!();

        if target.direction() != templ.direction() {
            return Err(glib::bool_error!(
                "Template and target have different directions"
            ));
        }

        Self::builder_from_template(templ).with_target(target)
    }
}

impl<T: IsA<GhostPad> + IsA<Pad>> PadBuilder<T> {
    #[doc(alias = "gst_pad_set_activate_function")]
    pub fn proxy_pad_activate_function<F>(self, func: F) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_activate_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_activate_function")]
    pub fn proxy_pad_activate_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_activate_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_activatemode_function")]
    pub fn proxy_pad_activatemode_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                crate::PadMode,
                bool,
            ) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_activatemode_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_activatemode_function")]
    pub fn proxy_pad_activatemode_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                crate::PadMode,
                bool,
            ) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_activatemode_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_chain_function")]
    pub fn proxy_pad_chain_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                crate::Buffer,
            ) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_chain_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_chain_function")]
    pub fn proxy_pad_chain_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                crate::Buffer,
            ) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_chain_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_chain_list_function")]
    pub fn proxy_pad_chain_list_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                crate::BufferList,
            ) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_chain_list_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_chain_list_function")]
    pub fn proxy_pad_chain_list_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                crate::BufferList,
            ) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_chain_list_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_event_function")]
    pub fn proxy_pad_event_function<F>(self, func: F) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>, crate::Event) -> bool
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_event_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_event_function")]
    pub fn proxy_pad_event_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>, crate::Event) -> bool
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_event_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_event_full_function")]
    pub fn proxy_pad_event_full_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                crate::Event,
            ) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_event_full_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_event_full_function")]
    pub fn proxy_pad_event_full_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                crate::Event,
            ) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_event_full_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_getrange_function")]
    pub fn proxy_pad_getrange_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                u64,
                Option<&mut crate::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, crate::FlowError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_getrange_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_getrange_function")]
    pub fn proxy_pad_getrange_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                u64,
                Option<&mut crate::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, crate::FlowError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_getrange_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_iterate_internal_links_function")]
    pub fn proxy_pad_iterate_internal_links_function<F>(self, func: F) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>) -> crate::Iterator<Pad>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_iterate_internal_links_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_iterate_internal_links_function")]
    pub fn proxy_pad_iterate_internal_links_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>) -> crate::Iterator<Pad>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_iterate_internal_links_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_link_function")]
    pub fn proxy_pad_link_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                &Pad,
            ) -> Result<crate::PadLinkSuccess, crate::PadLinkError>
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_link_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_link_function")]
    pub fn proxy_pad_link_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(
                &crate::ProxyPad,
                Option<&crate::Object>,
                &Pad,
            ) -> Result<crate::PadLinkSuccess, crate::PadLinkError>
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_link_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_query_function")]
    pub fn proxy_pad_query_function<F>(self, func: F) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>, &mut crate::QueryRef) -> bool
            + Send
            + Sync
            + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_query_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_query_function")]
    pub fn proxy_pad_query_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>, &mut crate::QueryRef) -> bool
            + Send
            + Sync
            + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_query_function(func)
        } else {
            self
        }
    }

    #[doc(alias = "gst_pad_set_unlink_function")]
    pub fn proxy_pad_unlink_function<F>(self, func: F) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>) + Send + Sync + 'static,
    {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_unlink_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_unlink_function")]
    pub fn proxy_pad_unlink_function_if_some<F>(self, func: Option<F>) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>) + Send + Sync + 'static,
    {
        if let Some(func) = func {
            self.proxy_pad_unlink_function(func)
        } else {
            self
        }
    }

    pub fn proxy_pad_flags(self, flags: PadFlags) -> Self {
        unsafe {
            let proxy = self
                .pad
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_pad_flags(flags);
        }

        self
    }

    pub fn proxy_pad_flags_if_some(self, flags: Option<PadFlags>) -> Self {
        if let Some(flags) = flags {
            self.proxy_pad_flags(flags)
        } else {
            self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Specifies a `target` [`Pad`](crate::Pad) for the [`GhostPad`].
    ///
    /// If the [`PadBuilder`](crate::PadBuilder) was created from
    /// a [`PadTemplate`](crate::PadTemplate) and the `PadTemplate` has a specific `name_template`,
    /// i.e. if it's not a wildcard-name containing `%u`, `%s` or `%d`,
    /// the `GhostPad` will automatically be named after the `name_template`.
    ///
    /// If the `name_template` is a wildcard-name, then the `target` `name` is used,
    /// if it is compatible. Otherwise, a specific name must be provided using
    /// [`PadBuilder::name`](crate::PadBuilder::name) or [`PadBuilder::maybe_name`](crate::PadBuilder::maybe_name).
    pub fn with_target<P: IsA<Pad> + IsA<crate::Object>>(
        mut self,
        target: &P,
    ) -> Result<Self, glib::BoolError> {
        assert_eq!(self.pad.direction(), target.direction());

        self.pad.set_target(Some(target))?;

        if self.needs_specific_name {
            let mut can_assign_target_name = true;

            if let Some(pad_template) = self.pad.pad_template() {
                if pad_template.presence() == crate::PadPresence::Request {
                    // Check if the target name is compatible with the name template.
                    use crate::CAT_RUST;

                    let target_name = target.name();
                    let mut target_parts = target_name.split('_');
                    for template_part in pad_template.name_template().split('_') {
                        let Some(target_part) = target_parts.next() else {
                            crate::debug!(
                                CAT_RUST,
                                "Not using target Pad name '{target_name}': not enough parts compared to template '{}'",
                                pad_template.name_template(),
                            );
                            can_assign_target_name = false;
                            break;
                        };

                        if let Some(conv_spec_start) = template_part.find('%') {
                            if conv_spec_start > 0
                                && !target_part.starts_with(&template_part[..conv_spec_start])
                            {
                                crate::debug!(
                                    CAT_RUST,
                                    "Not using target Pad name '{target_name}': mismatch template '{}' prefix",
                                    pad_template.name_template(),
                                );
                                can_assign_target_name = false;
                                break;
                            }

                            let conv_spec_pos = conv_spec_start + 1;
                            match template_part.get(conv_spec_pos..=conv_spec_pos) {
                                Some("s") => {
                                    // *There can be only one* %s
                                    break;
                                }
                                Some("u") => {
                                    if target_part
                                        .get(conv_spec_start..)
                                        .map_or(true, |s| s.parse::<u32>().is_err())
                                    {
                                        crate::debug!(
                                            CAT_RUST,
                                            "Not using target Pad name '{target_name}': can't parse '%u' from '{target_part}' (template '{}')",
                                            pad_template.name_template(),
                                        );

                                        can_assign_target_name = false;
                                        break;
                                    }
                                }
                                Some("d") => {
                                    if target_part
                                        .get(conv_spec_start..)
                                        .map_or(true, |s| s.parse::<i32>().is_err())
                                    {
                                        crate::debug!(
                                            CAT_RUST,
                                            "Not using target Pad name '{target_name}': can't parse '%i' from '{target_part}' (template '{}')",
                                            pad_template.name_template(),
                                        );

                                        can_assign_target_name = false;
                                        break;
                                    }
                                }
                                other => unreachable!("Unexpected conversion specifier {other:?}"),
                            }
                        } else if target_part != template_part {
                            can_assign_target_name = false;
                            break;
                        }
                    }
                }
            }

            if can_assign_target_name {
                self.pad.set_property("name", target.name());
                self.needs_specific_name = false;
            }
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_template_no_target() {
        crate::init().unwrap();

        let ghost_pad = GhostPad::new(crate::PadDirection::Sink);
        assert!(ghost_pad.name().starts_with("ghostpad"));

        let ghost_pad = GhostPad::builder(crate::PadDirection::Sink).build();
        assert!(ghost_pad.name().starts_with("ghostpad"));

        let ghost_pad = GhostPad::builder(crate::PadDirection::Sink)
            .name("sink")
            .build();
        assert_eq!(ghost_pad.name(), "sink");
    }

    #[test]
    fn from_template() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let wildcard_templ = crate::PadTemplate::new(
            "sink_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        let ghost_pad = GhostPad::builder_from_template(&wildcard_templ)
            .name("my-ghostpad")
            .build();
        assert_eq!(ghost_pad.name(), "my-ghostpad");

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "sink",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let ghost_pad = GhostPad::from_template(&templ);
        assert_eq!(ghost_pad.name(), "sink");

        let ghost_pad = GhostPad::builder_from_template(&templ).build();
        assert!(ghost_pad.name().starts_with("sink"));

        let ghost_pad = GhostPad::builder_from_template(&templ)
            .name("my-sink")
            .build();
        assert_eq!(ghost_pad.name(), "my-sink");
    }

    #[test]
    #[should_panic]
    fn from_template_missing_name() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "audio_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        // Panic: attempt to build from a wildcard-named template
        //        without providing a name.
        let _ghost_pad = GhostPad::from_template(&templ);
    }

    #[test]
    #[should_panic]
    fn from_template_builder_missing_name() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "audio_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        // Panic: attempt to build from a wildcard-named template
        //        without providing a name.
        let _ghost_pad = GhostPad::builder_from_template(&templ).build();
    }

    #[test]
    fn with_target() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "test",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::builder_from_template(&templ).build();
        let ghost_pad = GhostPad::with_target(&target).unwrap();
        assert_eq!(ghost_pad.name(), "test");

        let target = crate::Pad::from_template(&templ);
        let ghost_pad = GhostPad::builder_with_target(&target).unwrap().build();
        assert_eq!(ghost_pad.name(), "test");

        let target = crate::Pad::from_template(&templ);
        let ghost_pad = GhostPad::builder_with_target(&target)
            .unwrap()
            .name("ghost_test")
            .build();
        assert_eq!(ghost_pad.name(), "ghost_test");
    }

    #[test]
    fn from_template_with_target() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let sink_templ = crate::PadTemplate::new(
            "sink",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        // # No conversion specifier, Always template
        let ghost_templ = crate::PadTemplate::new(
            "ghost_sink",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::from_template(&sink_templ);
        let ghost_pad = GhostPad::from_template_with_target(&ghost_templ, &target).unwrap();
        assert_eq!(ghost_pad.name(), "ghost_sink");

        let target = crate::Pad::from_template(&sink_templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&ghost_templ, &target)
            .unwrap()
            .build();
        assert_eq!(ghost_pad.name(), "ghost_sink");

        let target = crate::Pad::from_template(&sink_templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&ghost_templ, &target)
            .unwrap()
            .name("my-sink")
            .build();
        assert_eq!(ghost_pad.name(), "my-sink");

        // # Request template %u
        let wildcard_u_templ = crate::PadTemplate::new(
            "sink_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        // ## Incompatible target but specific name
        let target = crate::Pad::from_template(&sink_templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_u_templ, &target)
            .unwrap()
            .name("sink_0")
            .build();
        assert_eq!(ghost_pad.name(), "sink_0");

        // ## Compatible target
        let sink_0_templ = crate::PadTemplate::new(
            "sink_0",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();
        let target = crate::Pad::from_template(&sink_0_templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_u_templ, &target)
            .unwrap()
            .build();
        assert_eq!(ghost_pad.name(), "sink_0");

        // # Request template %d_%u
        let wildcard_u_templ = crate::PadTemplate::new(
            "sink_%d_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        // ## Incompatible target but specific name
        let target = crate::Pad::from_template(&sink_templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_u_templ, &target)
            .unwrap()
            .name("sink_-1_0")
            .build();
        assert_eq!(ghost_pad.name(), "sink_-1_0");

        // ## Compatible target
        let sink_m2_0_templ = crate::PadTemplate::new(
            "sink_-2_0",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();
        let target = crate::Pad::from_template(&sink_m2_0_templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_u_templ, &target)
            .unwrap()
            .build();
        assert_eq!(ghost_pad.name(), "sink_-2_0");

        // # Request template %s
        let wildcard_s_templ = crate::PadTemplate::new(
            "sink_%s",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        // ## Incompatible target but specific name
        let target = crate::Pad::from_template(&sink_templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_s_templ, &target)
            .unwrap()
            .name("sink_ghost_test")
            .build();
        assert_eq!(ghost_pad.name(), "sink_ghost_test");

        // ## Compatible target
        let sink_test_templ = crate::PadTemplate::new(
            "sink_test",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();
        let target = crate::Pad::from_template(&sink_test_templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_s_templ, &target)
            .unwrap()
            .build();
        assert_eq!(ghost_pad.name(), "sink_test");
    }

    #[test]
    #[should_panic]
    fn from_template_with_target_incompatible_prefix() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let wildcard_templ = crate::PadTemplate::new(
            "sink_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        let templ = crate::PadTemplate::new(
            "audio_0",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::from_template(&templ);
        // Panic: attempt to build from a wildcard-named template
        //        with a target name with a different prefix
        //        without providing a name.
        let _ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_templ, &target)
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic]
    fn from_template_with_target_missing_part() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let wildcard_templ = crate::PadTemplate::new(
            "sink_%u_%s",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        let templ = crate::PadTemplate::new(
            "sink_0",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::from_template(&templ);
        // Panic: attempt to build from a wildcard-named template
        //        with a target name missing a part
        //        without providing a name.
        let _ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_templ, &target)
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic]
    fn from_template_with_target_incompatible_conversion_unsigned() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let wildcard_templ = crate::PadTemplate::new(
            "sink_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        let templ = crate::PadTemplate::new(
            "sink_-1",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::from_template(&templ);
        // Panic: attempt to build from a wildcard-named template
        //        with a target name %d, expecting %u
        //        without providing a name.
        let _ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_templ, &target)
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic]
    fn from_template_with_target_incompatible_conversion_decimal() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let wildcard_templ = crate::PadTemplate::new(
            "sink_%u",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        let templ = crate::PadTemplate::new(
            "sink_test",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::from_template(&templ);
        // Panic: attempt to build from a wildcard-named template
        //        with a target name with %s, expecting %d
        //        without providing a name.
        let _ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_templ, &target)
            .unwrap()
            .build();
    }

    #[test]
    #[should_panic]
    fn from_template_with_target_incompatible_missing_decimal() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let wildcard_templ = crate::PadTemplate::new(
            "sink_%d",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        let templ = crate::PadTemplate::new(
            "sink_",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::from_template(&templ);
        // Panic: attempt to build from a wildcard-named template
        //        with a target name missing a number, expecting %d
        //        without providing a name.
        let _ghost_pad = GhostPad::builder_from_template_with_target(&wildcard_templ, &target)
            .unwrap()
            .build();
    }
}
