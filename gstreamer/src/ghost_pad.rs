// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::FlowError;
use crate::FlowSuccess;
use crate::GhostPad;
use crate::LoggableError;
use crate::Object;
use crate::Pad;
use crate::PadBuilder;
use crate::PadFlags;
use crate::PadGetRangeSuccess;
use crate::PadMode;
use crate::StaticPadTemplate;
use glib::translate::*;

impl GhostPad {
    #[doc(alias = "gst_ghost_pad_activate_mode_default")]
    pub fn activate_mode_default<P: IsA<GhostPad>, Q: IsA<Object>>(
        pad: &P,
        parent: Option<&Q>,
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
    pub fn internal_activate_mode_default<P: IsA<GhostPad>, Q: IsA<Object>>(
        pad: &P,
        parent: Option<&Q>,
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

    #[doc(alias = "gst_ghost_pad_new_no_target")]
    pub fn new(name: Option<&str>, direction: crate::PadDirection) -> Self {
        skip_assert_initialized!();
        Self::builder(name, direction).build()
    }

    #[doc(alias = "gst_ghost_pad_new_no_target")]
    pub fn builder(name: Option<&str>, direction: crate::PadDirection) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::new(name, direction)
    }

    #[doc(alias = "gst_ghost_pad_new_no_target_from_static_template")]
    pub fn from_static_template(templ: &StaticPadTemplate, name: Option<&str>) -> Self {
        skip_assert_initialized!();
        Self::builder_with_static_template(templ, name).build()
    }

    #[doc(alias = "gst_ghost_pad_new_no_target_from_static_template")]
    pub fn builder_with_static_template(
        templ: &StaticPadTemplate,
        name: Option<&str>,
    ) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_static_template(templ, name)
    }

    #[doc(alias = "gst_ghost_pad_new_no_target_from_template")]
    pub fn from_template(templ: &crate::PadTemplate, name: Option<&str>) -> Self {
        skip_assert_initialized!();
        Self::builder_with_template(templ, name).build()
    }

    #[doc(alias = "gst_ghost_pad_new_no_target_from_template")]
    pub fn builder_with_template(
        templ: &crate::PadTemplate,
        name: Option<&str>,
    ) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_template(templ, name)
    }

    #[doc(alias = "gst_ghost_pad_new")]
    pub fn with_target<P: IsA<Pad>>(
        name: Option<&str>,
        target: &P,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();
        Self::builder(name, target.direction()).build_with_target(target)
    }

    #[doc(alias = "gst_ghost_pad_new_from_template")]
    pub fn from_template_with_target<P: IsA<Pad>>(
        templ: &crate::PadTemplate,
        name: Option<&str>,
        target: &P,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        if target.direction() != templ.direction() {
            return Err(glib::bool_error!(
                "Template and target have different directions"
            ));
        }

        Self::builder_with_template(templ, name).build_with_target(target)
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

    pub fn build_with_target<P: IsA<Pad>>(self, target: &P) -> Result<T, glib::BoolError> {
        assert_eq!(self.0.direction(), target.direction());

        self.0.set_target(Some(target))?;

        Ok(self.0)
    }
}
