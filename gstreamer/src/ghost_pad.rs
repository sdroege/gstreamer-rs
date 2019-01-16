// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::object::Cast;
use glib::object::{IsA, IsClassFor};
use glib::translate::*;
use GhostPad;
use Object;
use Pad;
use PadMode;
use PadTemplate;

use std::ops;

impl GhostPad {
    pub fn new<'a, P: Into<Option<&'a str>>, Q: IsA<Pad>>(name: P, target: &Q) -> Option<GhostPad> {
        skip_assert_initialized!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            Option::<Pad>::from_glib_none(ffi::gst_ghost_pad_new(
                name.0,
                target.as_ref().to_glib_none().0,
            ))
            .map(|o| Cast::unsafe_cast(o))
        }
    }

    pub fn new_from_template<'a, P: Into<Option<&'a str>>, Q: IsA<Pad>>(
        name: P,
        target: &Q,
        templ: &PadTemplate,
    ) -> Option<GhostPad> {
        skip_assert_initialized!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            Option::<Pad>::from_glib_none(ffi::gst_ghost_pad_new_from_template(
                name.0,
                target.as_ref().to_glib_none().0,
                templ.to_glib_none().0,
            ))
            .map(|o| Cast::unsafe_cast(o))
        }
    }

    pub fn activate_mode_default<
        'a,
        P: IsA<GhostPad>,
        Q: IsA<Object> + 'a,
        R: Into<Option<&'a Q>>,
    >(
        pad: &P,
        parent: R,
        mode: PadMode,
        active: bool,
    ) -> bool {
        skip_assert_initialized!();
        let parent = parent.into();
        unsafe {
            from_glib(ffi::gst_ghost_pad_activate_mode_default(
                pad.to_glib_none().0 as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                mode.to_glib(),
                active.to_glib(),
            ))
        }
    }

    pub fn internal_activate_mode_default<
        'a,
        P: IsA<GhostPad>,
        Q: IsA<Object> + 'a,
        R: Into<Option<&'a Q>>,
    >(
        pad: &P,
        parent: R,
        mode: PadMode,
        active: bool,
    ) -> bool {
        skip_assert_initialized!();
        let parent = parent.into();
        unsafe {
            from_glib(ffi::gst_ghost_pad_internal_activate_mode_default(
                pad.to_glib_none().0 as *mut ffi::GstPad,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                mode.to_glib(),
                active.to_glib(),
            ))
        }
    }
}

#[repr(C)]
pub struct GhostPadClass(ffi::GstGhostPadClass);

unsafe impl IsClassFor for GhostPadClass {
    type Instance = GhostPad;
}

unsafe impl Send for GhostPadClass {}
unsafe impl Sync for GhostPadClass {}

impl ops::Deref for GhostPadClass {
    type Target = ::PadClass;

    fn deref(&self) -> &Self::Target {
        self.upcast_ref()
    }
}

impl ops::DerefMut for GhostPadClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.upcast_ref_mut()
    }
}
