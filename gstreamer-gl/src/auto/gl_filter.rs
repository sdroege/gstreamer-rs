// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, GLBaseFilter, GLMemory, GLShader};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstGLFilter")]
    pub struct GLFilter(Object<ffi::GstGLFilter, ffi::GstGLFilterClass>) @extends GLBaseFilter, gst_base::BaseTransform, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_filter_get_type(),
    }
}

impl GLFilter {
    pub const NONE: Option<&'static GLFilter> = None;
}

unsafe impl Send for GLFilter {}
unsafe impl Sync for GLFilter {}

pub trait GLFilterExt: IsA<GLFilter> + 'static {
    #[doc(alias = "gst_gl_filter_draw_fullscreen_quad")]
    fn draw_fullscreen_quad(&self) {
        unsafe {
            ffi::gst_gl_filter_draw_fullscreen_quad(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_filter_filter_texture")]
    fn filter_texture(
        &self,
        input: &gst::Buffer,
        output: &gst::Buffer,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_filter_filter_texture(
                    self.as_ref().to_glib_none().0,
                    input.to_glib_none().0,
                    output.to_glib_none().0
                ),
                "Failed to transform texture"
            )
        }
    }

    #[doc(alias = "gst_gl_filter_render_to_target")]
    fn render_to_target<P: FnMut(&GLFilter, &GLMemory) -> bool>(
        &self,
        input: &GLMemory,
        output: &GLMemory,
        func: P,
    ) -> Result<(), glib::error::BoolError> {
        let mut func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&GLFilter, &GLMemory) -> bool>(
            filter: *mut ffi::GstGLFilter,
            in_tex: *mut ffi::GstGLMemory,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let filter = from_glib_borrow(filter);
            let in_tex = from_glib_borrow(in_tex);
            let callback = user_data as *mut P;
            (*callback)(&filter, &in_tex).into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &mut P = &mut func_data;
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_filter_render_to_target(
                    self.as_ref().to_glib_none().0,
                    mut_override(input.to_glib_none().0),
                    mut_override(output.to_glib_none().0),
                    func,
                    super_callback0 as *mut _ as *mut _
                ),
                "`func` returned `false`"
            )
        }
    }

    #[doc(alias = "gst_gl_filter_render_to_target_with_shader")]
    fn render_to_target_with_shader(&self, input: &GLMemory, output: &GLMemory, shader: &GLShader) {
        unsafe {
            ffi::gst_gl_filter_render_to_target_with_shader(
                self.as_ref().to_glib_none().0,
                mut_override(input.to_glib_none().0),
                mut_override(output.to_glib_none().0),
                shader.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<GLFilter>> GLFilterExt for O {}
