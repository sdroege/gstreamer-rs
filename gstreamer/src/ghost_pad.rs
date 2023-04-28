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
    /// Creates a new [`GhostPad`] object with a default name.
    ///
    /// Use [`GhostPad::builder()`] to get a [`PadBuilder`] and then define a specific name.
    #[doc(alias = "gst_ghost_pad_new_no_target")]
    pub fn new(direction: crate::PadDirection) -> Self {
        skip_assert_initialized!();
        Self::builder(direction).build()
    }

    #[doc(alias = "gst_ghost_pad_new_no_target")]
    pub fn builder(direction: crate::PadDirection) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::new(direction)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] object from the [`StaticPadTemplate`](crate::StaticPadTemplate) with a default name.
    ///
    /// Use [`GhostPad::builder_from_static_template()`] to get a [`PadBuilder`] and then define a specific name.
    #[doc(alias = "gst_ghost_pad_new_no_target_from_static_template")]
    pub fn from_static_template(templ: &StaticPadTemplate) -> Self {
        skip_assert_initialized!();
        Self::builder_from_static_template(templ).build()
    }

    #[doc(alias = "gst_ghost_pad_new_no_target_from_static_template")]
    pub fn builder_from_static_template(templ: &StaticPadTemplate) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_static_template(templ)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] object from the [`PadTemplate`](crate::PadTemplate) with a default name.
    ///
    /// Use [`GhostPad::builder_from_template()`] to get a [`PadBuilder`] and then define a specific name.
    #[doc(alias = "gst_ghost_pad_new_no_target_from_template")]
    pub fn from_template(templ: &crate::PadTemplate) -> Self {
        skip_assert_initialized!();
        Self::builder_from_template(templ).build()
    }

    #[doc(alias = "gst_ghost_pad_new_no_target_from_template")]
    pub fn builder_from_template(templ: &crate::PadTemplate) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_template(templ)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] object from the specified `target` `Pad` and a default name.
    ///
    /// Use [`GhostPad::builder_with_target()`] to get a [`PadBuilder`] and then define a specific name.
    #[doc(alias = "gst_ghost_pad_new")]
    pub fn with_target<P: IsA<Pad>>(target: &P) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();
        Ok(Self::builder_with_target(target)?.build())
    }

    #[doc(alias = "gst_ghost_pad_new_no_target_from_template")]
    pub fn builder_with_target<P: IsA<Pad>>(
        target: &P,
    ) -> Result<PadBuilder<Self>, glib::BoolError> {
        skip_assert_initialized!();
        Self::builder(target.direction()).with_target(target)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`GhostPad`] object from the [`PadTemplate`](crate::PadTemplate)
    /// with the specified `target` `Pad` and a default name.
    ///
    /// Returns `Err(_)` if the `PadTemplate` and the `target` directions differ.
    ///
    /// Use [`GhostPad::builder_from_template_with_target()`] to get a [`PadBuilder`] and then define a specific name.
    #[doc(alias = "gst_ghost_pad_new_from_template")]
    pub fn from_template_with_target<P: IsA<Pad>>(
        templ: &crate::PadTemplate,
        target: &P,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();
        Ok(Self::builder_from_template_with_target(templ, target)?.build())
    }

    #[doc(alias = "gst_ghost_pad_new_from_template")]
    pub fn builder_from_template_with_target<P: IsA<Pad>>(
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_activate_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_activatemode_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_chain_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_chain_list_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_event_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_event_full_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_getrange_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_iterate_internal_links_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_link_function(func);
        }

        self
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
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_query_function(func);
        }

        self
    }

    #[doc(alias = "gst_pad_set_unlink_function")]
    pub fn proxy_pad_unlink_function<F>(self, func: F) -> Self
    where
        F: Fn(&crate::ProxyPad, Option<&crate::Object>) + Send + Sync + 'static,
    {
        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_unlink_function(func);
        }

        self
    }

    pub fn proxy_pad_flags(self, flags: PadFlags) -> Self {
        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<crate::ProxyPad>()
                .internal()
                .unwrap();
            proxy.set_pad_flags(flags);
        }

        self
    }

    pub fn with_target<P: IsA<Pad>>(self, target: &P) -> Result<Self, glib::BoolError> {
        assert_eq!(self.0.direction(), target.direction());

        self.0.set_target(Some(target))?;

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_template() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "sink",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let ghost_pad = GhostPad::from_template(&templ);
        assert!(ghost_pad.name().starts_with("ghostpad"));

        let ghost_pad = GhostPad::builder_from_template(&templ).build();
        assert!(ghost_pad.name().starts_with("ghostpad"));

        let ghost_pad = GhostPad::builder_from_template(&templ).name("sink").build();
        assert_eq!(ghost_pad.name(), "sink");
    }

    #[test]
    fn with_target() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "sink",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::from_template(&templ);
        let ghost_pad = GhostPad::with_target(&target).unwrap();
        assert!(ghost_pad.name().starts_with("ghostpad"));

        let target = crate::Pad::from_template(&templ);
        let ghost_pad = GhostPad::builder_with_target(&target).unwrap().build();
        assert!(ghost_pad.name().starts_with("ghostpad"));

        let target = crate::Pad::from_template(&templ);
        let ghost_pad = GhostPad::builder_with_target(&target)
            .unwrap()
            .name("sink")
            .build();
        assert_eq!(ghost_pad.name(), "sink");
    }

    #[test]
    fn from_template_with_target() {
        crate::init().unwrap();

        let caps = crate::Caps::new_any();
        let templ = crate::PadTemplate::new(
            "sink",
            crate::PadDirection::Sink,
            crate::PadPresence::Always,
            &caps,
        )
        .unwrap();

        let ghost_templ = crate::PadTemplate::new(
            "sink",
            crate::PadDirection::Sink,
            crate::PadPresence::Request,
            &caps,
        )
        .unwrap();

        let target = crate::Pad::from_template(&templ);
        let ghost_pad = GhostPad::from_template_with_target(&ghost_templ, &target).unwrap();
        assert!(ghost_pad.name().starts_with("ghostpad"));

        let target = crate::Pad::from_template(&templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&ghost_templ, &target)
            .unwrap()
            .build();
        assert!(ghost_pad.name().starts_with("ghostpad"));

        let target = crate::Pad::from_template(&templ);
        let ghost_pad = GhostPad::builder_from_template_with_target(&ghost_templ, &target)
            .unwrap()
            .name("sink")
            .build();
        assert_eq!(ghost_pad.name(), "sink");
    }
}
