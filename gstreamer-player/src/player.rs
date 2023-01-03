// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, mem::transmute};

use glib::{
    object::ObjectType,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};

use crate::Player;

impl Player {
    #[doc(alias = "get_config")]
    #[doc(alias = "gst_player_get_config")]
    pub fn config(&self) -> crate::PlayerConfig {
        unsafe { from_glib_full(ffi::gst_player_get_config(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_player_set_config")]
    pub fn set_config(&self, config: crate::PlayerConfig) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_player_set_config(self.to_glib_none().0, config.into_glib_ptr()),
                "Failed to set config",
            )
        }
    }

    pub fn connect_duration_changed<F: Fn(&Player, Option<gst::ClockTime>) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"duration-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    duration_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_position_updated<F: Fn(&Player, Option<gst::ClockTime>) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"position-updated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    position_updated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_seek_done<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"seek-done\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    seek_done_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "gst_player_get_video_snapshot")]
    #[doc(alias = "get_video_snapshot")]
    pub fn video_snapshot(
        &self,
        format: crate::PlayerSnapshotFormat,
        config: Option<&gst::StructureRef>,
    ) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_player_get_video_snapshot(
                self.to_glib_none().0,
                format.into_glib(),
                mut_override(config.map(|c| c.as_ptr()).unwrap_or(std::ptr::null())),
            ))
        }
    }
}

unsafe extern "C" fn duration_changed_trampoline<
    F: Fn(&Player, Option<gst::ClockTime>) + Send + 'static,
>(
    this: *mut ffi::GstPlayer,
    object: u64,
    f: glib::ffi::gpointer,
) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this), FromGlib::from_glib(object))
}

unsafe extern "C" fn position_updated_trampoline<
    F: Fn(&Player, Option<gst::ClockTime>) + Send + 'static,
>(
    this: *mut ffi::GstPlayer,
    object: u64,
    f: glib::ffi::gpointer,
) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this), FromGlib::from_glib(object))
}

unsafe extern "C" fn seek_done_trampoline<F: Fn(&Player, gst::ClockTime) + Send + 'static>(
    this: *mut ffi::GstPlayer,
    object: u64,
    f: glib::ffi::gpointer,
) {
    let f: &F = &*(f as *const F);
    f(
        &from_glib_borrow(this),
        try_from_glib(object).expect("undefined seek position"),
    )
}
