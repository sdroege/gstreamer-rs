// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::prelude::*;
use glib::translate::*;
use gst_sys;
use FlowError;
use FlowSuccess;
use GhostPad;
use LoggableError;
use Object;
use Pad;
use PadBuilder;
use PadExt;
use PadExtManual;
use PadFlags;
use PadGetRangeSuccess;
use PadMode;
use StaticPadTemplate;

impl GhostPad {
    pub fn activate_mode_default<P: IsA<GhostPad>, Q: IsA<Object>>(
        pad: &P,
        parent: Option<&Q>,
        mode: PadMode,
        active: bool,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_ghost_pad_activate_mode_default(
                    pad.to_glib_none().0 as *mut gst_sys::GstPad,
                    parent.map(|p| p.as_ref()).to_glib_none().0,
                    mode.to_glib(),
                    active.to_glib(),
                ),
                "Failed to invoke the default activate mode function of the ghost pad"
            )
        }
    }

    pub fn internal_activate_mode_default<P: IsA<GhostPad>, Q: IsA<Object>>(
        pad: &P,
        parent: Option<&Q>,
        mode: PadMode,
        active: bool,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_ghost_pad_internal_activate_mode_default(
                    pad.to_glib_none().0 as *mut gst_sys::GstPad,
                    parent.map(|p| p.as_ref()).to_glib_none().0,
                    mode.to_glib(),
                    active.to_glib(),
                ),
                concat!(
                    "Failed to invoke the default activate mode function of a proxy pad ",
                    "that is owned by the ghost pad"
                )
            )
        }
    }

    pub fn new(name: Option<&str>, direction: ::PadDirection) -> Self {
        skip_assert_initialized!();
        Self::builder(name, direction).build()
    }

    pub fn builder(name: Option<&str>, direction: ::PadDirection) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::new(name, direction)
    }

    pub fn from_static_template(templ: &StaticPadTemplate, name: Option<&str>) -> Self {
        skip_assert_initialized!();
        Self::builder_with_static_template(templ, name).build()
    }

    pub fn builder_with_static_template(
        templ: &StaticPadTemplate,
        name: Option<&str>,
    ) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_static_template(templ, name)
    }

    pub fn from_template(templ: &::PadTemplate, name: Option<&str>) -> Self {
        skip_assert_initialized!();
        Self::builder_with_template(templ, name).build()
    }

    pub fn builder_with_template(templ: &::PadTemplate, name: Option<&str>) -> PadBuilder<Self> {
        skip_assert_initialized!();
        PadBuilder::from_template(templ, name)
    }

    pub fn with_target<P: IsA<Pad>>(
        name: Option<&str>,
        target: &P,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();
        Self::builder(name, target.get_direction()).build_with_target(target)
    }

    pub fn from_template_with_target<P: IsA<Pad>>(
        templ: &::PadTemplate,
        name: Option<&str>,
        target: &P,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        if target.get_direction() != templ.get_property_direction() {
            return Err(glib_bool_error!(
                "Template and target have different directions"
            ));
        }

        Self::builder_with_template(templ, name).build_with_target(target)
    }
}

impl<T: IsA<GhostPad> + IsA<Pad>> PadBuilder<T> {
    pub fn proxy_pad_activate_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>) -> Result<(), LoggableError> + Send + Sync + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_activate_function(func);
        }

        self
    }

    pub fn proxy_pad_activatemode_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>, ::PadMode, bool) -> Result<(), LoggableError>
            + Send
            + Sync
            + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_activatemode_function(func);
        }

        self
    }

    pub fn proxy_pad_chain_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>, ::Buffer) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_chain_function(func);
        }

        self
    }

    pub fn proxy_pad_chain_list_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>, ::BufferList) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_chain_list_function(func);
        }

        self
    }

    pub fn proxy_pad_event_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>, ::Event) -> bool + Send + Sync + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_event_function(func);
        }

        self
    }

    pub fn proxy_pad_event_full_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>, ::Event) -> Result<FlowSuccess, FlowError>
            + Send
            + Sync
            + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_event_full_function(func);
        }

        self
    }

    pub fn proxy_pad_getrange_function<F>(self, func: F) -> Self
    where
        F: Fn(
                &::ProxyPad,
                Option<&::Object>,
                u64,
                Option<&mut ::BufferRef>,
                u32,
            ) -> Result<PadGetRangeSuccess, ::FlowError>
            + Send
            + Sync
            + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_getrange_function(func);
        }

        self
    }

    pub fn proxy_pad_iterate_internal_links_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>) -> ::Iterator<Pad> + Send + Sync + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_iterate_internal_links_function(func);
        }

        self
    }

    pub fn proxy_pad_link_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>, &Pad) -> Result<::PadLinkSuccess, ::PadLinkError>
            + Send
            + Sync
            + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_link_function(func);
        }

        self
    }

    pub fn proxy_pad_query_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>, &mut ::QueryRef) -> bool + Send + Sync + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_query_function(func);
        }

        self
    }

    pub fn proxy_pad_unlink_function<F>(self, func: F) -> Self
    where
        F: Fn(&::ProxyPad, Option<&::Object>) + Send + Sync + 'static,
    {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_unlink_function(func);
        }

        self
    }

    pub fn proxy_pad_flags(self, flags: PadFlags) -> Self {
        use ProxyPadExt;

        unsafe {
            let proxy = self
                .0
                .unsafe_cast_ref::<::ProxyPad>()
                .get_internal()
                .unwrap();
            proxy.set_pad_flags(flags);
        }

        self
    }

    pub fn build_with_target<P: IsA<Pad>>(self, target: &P) -> Result<T, glib::BoolError> {
        use GhostPadExt;

        assert_eq!(self.0.get_direction(), target.get_direction());

        self.0.set_target(Some(target))?;

        Ok(self.0)
    }
}
