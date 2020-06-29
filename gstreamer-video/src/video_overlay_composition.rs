// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::mem;

use gst;
use gst_video_sys;

use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, ToGlib, ToGlibPtr};

gst_define_mini_object_wrapper!(
    VideoOverlayRectangle,
    VideoOverlayRectangleRef,
    gst_video_sys::GstVideoOverlayRectangle,
    || gst_video_sys::gst_video_overlay_rectangle_get_type()
);

impl fmt::Debug for VideoOverlayRectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        VideoOverlayRectangleRef::fmt(self, f)
    }
}

impl fmt::Debug for VideoOverlayRectangleRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoOverlayRectangle")
            .field("flags", &self.get_flags())
            .field("global_alpha", &self.get_global_alpha())
            .field("render_rectangle", &self.get_render_rectangle())
            .finish()
    }
}

impl VideoOverlayRectangle {
    pub fn new_raw(
        buffer: &gst::Buffer,
        render_x: i32,
        render_y: i32,
        render_width: u32,
        render_height: u32,
        flags: ::VideoOverlayFormatFlags,
    ) -> Self {
        assert_initialized_main_thread!();
        assert!(buffer.get_meta::<::VideoMeta>().is_some());
        unsafe {
            from_glib_full(gst_video_sys::gst_video_overlay_rectangle_new_raw(
                buffer.to_glib_none().0,
                render_x,
                render_y,
                render_width,
                render_height,
                flags.to_glib(),
            ))
        }
    }
}

impl VideoOverlayRectangleRef {
    pub fn get_flags(&self) -> ::VideoOverlayFormatFlags {
        unsafe {
            from_glib(gst_video_sys::gst_video_overlay_rectangle_get_flags(
                self.as_mut_ptr(),
            ))
        }
    }

    pub fn get_global_alpha(&self) -> f32 {
        unsafe { gst_video_sys::gst_video_overlay_rectangle_get_global_alpha(self.as_mut_ptr()) }
    }

    pub fn set_global_alpha(&mut self, alpha: f32) {
        unsafe {
            gst_video_sys::gst_video_overlay_rectangle_set_global_alpha(self.as_mut_ptr(), alpha)
        }
    }

    pub fn get_seqnum(&self) -> u32 {
        unsafe { gst_video_sys::gst_video_overlay_rectangle_get_seqnum(self.as_mut_ptr()) }
    }

    pub fn get_render_rectangle(&self) -> (i32, i32, u32, u32) {
        unsafe {
            let mut render_x = mem::MaybeUninit::uninit();
            let mut render_y = mem::MaybeUninit::uninit();
            let mut render_width = mem::MaybeUninit::uninit();
            let mut render_height = mem::MaybeUninit::uninit();

            gst_video_sys::gst_video_overlay_rectangle_get_render_rectangle(
                self.as_mut_ptr(),
                render_x.as_mut_ptr(),
                render_y.as_mut_ptr(),
                render_width.as_mut_ptr(),
                render_height.as_mut_ptr(),
            );

            (
                render_x.assume_init(),
                render_y.assume_init(),
                render_width.assume_init(),
                render_height.assume_init(),
            )
        }
    }

    pub fn set_render_rectangle(
        &mut self,
        render_x: i32,
        render_y: i32,
        render_width: u32,
        render_height: u32,
    ) {
        unsafe {
            gst_video_sys::gst_video_overlay_rectangle_set_render_rectangle(
                self.as_mut_ptr(),
                render_x,
                render_y,
                render_width,
                render_height,
            )
        }
    }

    pub fn get_pixels_unscaled_raw(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(
                gst_video_sys::gst_video_overlay_rectangle_get_pixels_unscaled_raw(
                    self.as_mut_ptr(),
                    flags.to_glib(),
                ),
            )
        }
    }

    pub fn get_pixels_unscaled_ayuv(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(
                gst_video_sys::gst_video_overlay_rectangle_get_pixels_unscaled_ayuv(
                    self.as_mut_ptr(),
                    flags.to_glib(),
                ),
            )
        }
    }

    pub fn get_pixels_unscaled_argb(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(
                gst_video_sys::gst_video_overlay_rectangle_get_pixels_unscaled_argb(
                    self.as_mut_ptr(),
                    flags.to_glib(),
                ),
            )
        }
    }

    pub fn get_pixels_raw(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(gst_video_sys::gst_video_overlay_rectangle_get_pixels_raw(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_pixels_ayuv(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(gst_video_sys::gst_video_overlay_rectangle_get_pixels_ayuv(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_pixels_argb(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(gst_video_sys::gst_video_overlay_rectangle_get_pixels_argb(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }
}

gst_define_mini_object_wrapper!(
    VideoOverlayComposition,
    VideoOverlayCompositionRef,
    gst_video_sys::GstVideoOverlayComposition,
    || gst_video_sys::gst_video_overlay_composition_get_type()
);

impl fmt::Debug for VideoOverlayComposition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        VideoOverlayCompositionRef::fmt(self, f)
    }
}

impl fmt::Debug for VideoOverlayCompositionRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoOverlayComposition").finish()
    }
}

impl VideoOverlayComposition {
    pub fn new<'a, T: IntoIterator<Item = &'a VideoOverlayRectangle>>(
        rects: T,
    ) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut iter = rects.into_iter();

            let first = match iter.next() {
                None => return Err(glib_bool_error!("Failed to create VideoOverlayComposition")),
                Some(first) => first,
            };

            let composition = Self::from_glib_full(
                gst_video_sys::gst_video_overlay_composition_new(first.as_mut_ptr()),
            );

            for rect in iter {
                gst_video_sys::gst_video_overlay_composition_add_rectangle(
                    composition.as_mut_ptr(),
                    rect.as_mut_ptr(),
                );
            }

            Ok(composition)
        }
    }
}

impl VideoOverlayCompositionRef {
    pub fn n_rectangles(&self) -> u32 {
        unsafe { gst_video_sys::gst_video_overlay_composition_n_rectangles(self.as_mut_ptr()) }
    }

    pub fn get_rectangle(&self, idx: u32) -> Result<VideoOverlayRectangle, glib::error::BoolError> {
        if idx >= self.n_rectangles() {
            return Err(glib_bool_error!("Invalid index"));
        }

        unsafe {
            match from_glib_none(gst_video_sys::gst_video_overlay_composition_get_rectangle(
                self.as_mut_ptr(),
                idx,
            )) {
                Some(r) => Ok(r),
                None => Err(glib_bool_error!("Failed to get rectangle")),
            }
        }
    }

    pub fn get_seqnum(&self) -> u32 {
        unsafe { gst_video_sys::gst_video_overlay_composition_get_seqnum(self.as_mut_ptr()) }
    }

    pub fn blend(
        &self,
        frame: &mut ::VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_video_sys::gst_video_overlay_composition_blend(
                    self.as_mut_ptr(),
                    frame.as_mut_ptr()
                ),
                "Failed to blend overlay composition",
            )
        }
    }
}
