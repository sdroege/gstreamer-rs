// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{ffi, TimedValueControlSource};

pub trait TimedValueControlSourceExtManual: IsA<TimedValueControlSource> + 'static {
    #[doc(alias = "gst_timed_value_control_source_list_control_points")]
    fn list_control_points(&self) -> glib::collections::Slice<gst::TimedValue> {
        #[cfg(feature = "v1_28")]
        unsafe {
            let mut n_control_points = std::mem::MaybeUninit::uninit();
            let ptr = ffi::gst_timed_value_control_source_list_control_points(
                self.as_ref().to_glib_none().0,
                n_control_points.as_mut_ptr(),
            );
            let len = n_control_points.assume_init() as usize;
            FromGlibContainer::from_glib_full_num(ptr, len)
        }
        #[cfg(not(feature = "v1_28"))]
        unsafe {
            // Get the GList of GstControlPoint pointers
            use std::mem;
            use std::ptr;

            let glist_head =
                ffi::gst_timed_value_control_source_get_all(self.as_ref().to_glib_none().0);
            let len = glib::ffi::g_list_length(glist_head) as usize;
            let result = glib::ffi::g_malloc(
                mem::size_of::<gst::ffi::GstTimedValue>()
                    .checked_mul(len)
                    .unwrap(),
            ) as *mut gst::ffi::GstTimedValue;

            let mut glist_ptr = glist_head;
            for i in 0..len {
                let cp_ptr = (*glist_ptr).data as *const gst::ffi::GstTimedValue;
                *result.add(i) = ptr::read(cp_ptr);
                glist_ptr = (*glist_ptr).next;
            }
            glib::ffi::g_list_free(glist_head);

            FromGlibContainer::from_glib_full_num(result, len)
        }
    }
}

impl<O: IsA<TimedValueControlSource>> TimedValueControlSourceExtManual for O {}
