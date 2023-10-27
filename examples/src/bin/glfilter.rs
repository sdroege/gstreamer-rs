#![allow(clippy::non_send_fields_in_send_ty)]

use anyhow::Result;

#[path = "../glupload.rs"]
mod glupload;
use glupload::*;

#[path = "../examples-common.rs"]
pub mod examples_common;

/// The fragment shader used for transforming GL textures travelling through the
/// pipeline. This fragment shader links against the default vertex shader
/// provided by [`GLSLStage::new_default_vertex`].
const FRAGMENT_SHADER: &str = r#"
#ifdef GL_ES
precision mediump float;
#endif

// The filter draws a fullscreen quad and provides its coordinates here:
varying vec2 v_texcoord;

// The input texture is bound on a uniform sampler named `tex`:
uniform sampler2D tex;

void main () {
    // Flip texture read coordinate on the x axis to create a mirror effect:
    gl_FragColor = texture2D(tex, vec2(1.0 - v_texcoord.x, v_texcoord.y));
}
"#;

mod mirror {
    use std::sync::Mutex;

    use glib::once_cell::sync::Lazy;
    use gst_base::subclass::BaseTransformMode;
    use gst_gl::{
        prelude::*,
        subclass::{prelude::*, GLFilterMode},
        *,
    };

    use super::{gl, FRAGMENT_SHADER};

    pub static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
        gst::DebugCategory::new(
            "rsglmirrorfilter",
            gst::DebugColorFlags::empty(),
            Some("Rust GL Mirror Filter"),
        )
    });

    glib::wrapper! {
        pub struct GLMirrorFilter(ObjectSubclass<imp::GLMirrorFilter>) @extends gst_gl::GLFilter, gst_gl::GLBaseFilter, gst_base::BaseTransform, gst::Element, gst::Object;
    }

    impl GLMirrorFilter {
        pub fn new(name: Option<&str>) -> Self {
            glib::Object::builder().property("name", name).build()
        }
    }

    mod imp {
        use super::*;

        /// Private data consists of the transformation shader which is compiled
        /// in advance to running the actual filter.
        #[derive(Default)]
        pub struct GLMirrorFilter {
            shader: Mutex<Option<GLShader>>,
        }

        impl GLMirrorFilter {
            fn create_shader(&self, context: &GLContext) -> Result<(), gst::LoggableError> {
                let shader = GLShader::new(context);

                let vertex = GLSLStage::new_default_vertex(context);
                vertex.compile().unwrap();
                shader.attach_unlocked(&vertex)?;

                gst::debug!(
                    CAT,
                    imp: self,
                    "Compiling fragment shader {}",
                    FRAGMENT_SHADER
                );

                let fragment = GLSLStage::with_strings(
                    context,
                    gl::FRAGMENT_SHADER,
                    // new_default_vertex is compiled with this version and profile:
                    GLSLVersion::None,
                    GLSLProfile::ES | GLSLProfile::COMPATIBILITY,
                    &[FRAGMENT_SHADER],
                );
                fragment.compile().unwrap();
                shader.attach_unlocked(&fragment)?;
                shader.link().unwrap();

                gst::debug!(
                    CAT,
                    imp: self,
                    "Successfully compiled and linked {:?}",
                    shader
                );

                *self.shader.lock().unwrap() = Some(shader);
                Ok(())
            }
        }

        // See `subclass.rs` for general documentation on creating a subclass. Extended
        // information like element metadata have been omitted for brevity.
        #[glib::object_subclass]
        impl ObjectSubclass for GLMirrorFilter {
            const NAME: &'static str = "RsGLMirrorFilter";
            type Type = super::GLMirrorFilter;
            type ParentType = gst_gl::GLFilter;
        }

        impl ElementImpl for GLMirrorFilter {}
        impl GstObjectImpl for GLMirrorFilter {}
        impl ObjectImpl for GLMirrorFilter {}
        impl BaseTransformImpl for GLMirrorFilter {
            const MODE: BaseTransformMode = BaseTransformMode::NeverInPlace;
            const PASSTHROUGH_ON_SAME_CAPS: bool = false;
            const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;
        }
        impl GLBaseFilterImpl for GLMirrorFilter {
            fn gl_start(&self) -> Result<(), gst::LoggableError> {
                let filter = self.obj();

                // Create a shader when GL is started, knowing that the OpenGL context is
                // available.
                let context = GLBaseFilterExt::context(&*filter).unwrap();
                self.create_shader(&context)?;
                self.parent_gl_start()
            }
        }
        impl GLFilterImpl for GLMirrorFilter {
            const MODE: GLFilterMode = GLFilterMode::Texture;

            fn filter_texture(
                &self,
                input: &gst_gl::GLMemory,
                output: &gst_gl::GLMemory,
            ) -> Result<(), gst::LoggableError> {
                let filter = self.obj();

                let shader = self.shader.lock().unwrap();
                // Use the underlying filter implementation to transform the input texture into
                // an output texture with the shader.
                filter.render_to_target_with_shader(
                    input,
                    output,
                    shader
                        .as_ref()
                        .expect("No shader, call `create_shader` first!"),
                );
                self.parent_filter_texture(input, output)
            }
        }
    }
}

fn example_main() -> Result<()> {
    gst::init().unwrap();
    let glfilter = mirror::GLMirrorFilter::new(Some("foo"));
    App::new(Some(glfilter.as_ref())).and_then(main_loop)
}

fn main() -> Result<()> {
    examples_common::run(example_main)
}
