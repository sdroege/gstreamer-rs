//! This example demonstrates how to output GL textures, within an EGL/X11 context provided by the
//! application, and render those textures in the GL application.
//!
//! This example follow common patterns from `glutin`:
//! <https://github.com/rust-windowing/glutin/blob/master/glutin_examples/src/lib.rs>

// {videotestsrc} - { glsinkbin }

use std::{
    ffi::{CStr, CString},
    mem,
    num::NonZeroU32,
    ptr,
};

use anyhow::{Context, Result};
use glutin::{
    config::GetGlConfig as _,
    context::AsRawContext as _,
    display::{AsRawDisplay as _, GetGlDisplay as _},
    prelude::*,
};
use glutin_winit::GlWindow as _;
use gst::{element_error, PadProbeReturn, PadProbeType, QueryViewMut};
use gst_gl::prelude::*;
use raw_window_handle::HasWindowHandle as _;

#[rustfmt::skip]
static VERTICES: [f32; 20] = [
     1.0,  1.0, 0.0, 1.0, 0.0,
    -1.0,  1.0, 0.0, 0.0, 0.0,
    -1.0, -1.0, 0.0, 0.0, 1.0,
     1.0, -1.0, 0.0, 1.0, 1.0,
];

static INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

#[rustfmt::skip]
static IDENTITY: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];

const VS_SRC: &[u8] = c"
uniform mat4 u_transformation;
attribute vec4 a_position;
attribute vec2 a_texcoord;
varying vec2 v_texcoord;

void main() {
    gl_Position = u_transformation * a_position;
    v_texcoord = a_texcoord;
}"
.to_bytes();

const FS_SRC: &[u8] = c"
#ifdef GL_ES
precision mediump float;
#endif
varying vec2 v_texcoord;
uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, v_texcoord);
}"
.to_bytes();

#[allow(clippy::unreadable_literal)]
#[allow(clippy::unused_unit)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::manual_non_exhaustive)]
#[allow(clippy::upper_case_acronyms)]
#[allow(clippy::missing_transmute_annotations)]
pub(crate) mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/test_gl_bindings.rs"));
}

struct Gl {
    gl: gl::Gl,
    program: gl::types::GLuint,
    attr_position: gl::types::GLint,
    attr_texture: gl::types::GLint,
    vao: Option<gl::types::GLuint>,
    vertex_buffer: gl::types::GLuint,
    vbo_indices: gl::types::GLuint,
}

impl Gl {
    fn draw_frame(&self, texture_id: gl::types::GLuint) {
        unsafe {
            // render
            self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.gl.BlendColor(0.0, 0.0, 0.0, 1.0);
            if self.gl.BlendFuncSeparate.is_loaded() {
                self.gl.BlendFuncSeparate(
                    gl::SRC_ALPHA,
                    gl::CONSTANT_COLOR,
                    gl::ONE,
                    gl::ONE_MINUS_SRC_ALPHA,
                );
            } else {
                self.gl.BlendFunc(gl::SRC_ALPHA, gl::CONSTANT_COLOR);
            }
            self.gl.BlendEquation(gl::FUNC_ADD);
            self.gl.Enable(gl::BLEND);

            self.gl.UseProgram(self.program);

            if self.gl.BindVertexArray.is_loaded() {
                self.gl.BindVertexArray(self.vao.unwrap());
            }

            {
                self.gl
                    .BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vbo_indices);
                self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);

                // Load the vertex position
                self.gl.VertexAttribPointer(
                    self.attr_position as gl::types::GLuint,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (5 * mem::size_of::<f32>()) as gl::types::GLsizei,
                    ptr::null(),
                );

                // Load the texture coordinate
                self.gl.VertexAttribPointer(
                    self.attr_texture as gl::types::GLuint,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (5 * mem::size_of::<f32>()) as gl::types::GLsizei,
                    (3 * mem::size_of::<f32>()) as *const () as *const _,
                );

                self.gl.EnableVertexAttribArray(self.attr_position as _);
                self.gl.EnableVertexAttribArray(self.attr_texture as _);
            }

            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, texture_id);

            let location = self
                .gl
                .GetUniformLocation(self.program, c"tex".as_ptr() as *const _);
            self.gl.Uniform1i(location, 0);

            let location = self
                .gl
                .GetUniformLocation(self.program, c"u_transformation".as_ptr() as *const _);
            self.gl
                .UniformMatrix4fv(location, 1, gl::FALSE, IDENTITY.as_ptr() as *const _);

            self.gl
                .DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, ptr::null());

            self.gl.BindTexture(gl::TEXTURE_2D, 0);
            self.gl.UseProgram(0);

            if self.gl.BindVertexArray.is_loaded() {
                self.gl.BindVertexArray(0);
            }

            {
                self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
                self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);

                self.gl.DisableVertexAttribArray(self.attr_position as _);
                self.gl.DisableVertexAttribArray(self.attr_texture as _);
            }
        }
    }

    fn resize(&self, size: winit::dpi::PhysicalSize<u32>) {
        unsafe {
            self.gl
                .Viewport(0, 0, size.width as i32, size.height as i32);
        }
    }
}

fn load(gl_display: &impl glutin::display::GlDisplay) -> Gl {
    let gl = gl::Gl::load_with(|symbol| {
        let symbol = CString::new(symbol).unwrap();
        gl_display.get_proc_address(&symbol).cast()
    });

    let version = unsafe {
        let version = gl.GetString(gl::VERSION);
        assert!(!version.is_null());
        let version = CStr::from_ptr(version.cast());
        version.to_string_lossy()
    };

    println!("OpenGL version {version}");

    let (program, attr_position, attr_texture, vao, vertex_buffer, vbo_indices) = unsafe {
        let vs = gl.CreateShader(gl::VERTEX_SHADER);
        gl.ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
        gl.CompileShader(vs);

        let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
        gl.ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
        gl.CompileShader(fs);

        let program = gl.CreateProgram();
        gl.AttachShader(program, vs);
        gl.AttachShader(program, fs);
        gl.LinkProgram(program);

        {
            let mut success = 1;
            gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
            assert_ne!(success, 0);
            assert_eq!(gl.GetError(), 0);
        }

        let attr_position = gl.GetAttribLocation(program, c"a_position".as_ptr() as *const _);
        let attr_texture = gl.GetAttribLocation(program, c"a_texcoord".as_ptr() as *const _);

        let vao = if gl.BindVertexArray.is_loaded() {
            let mut vao = mem::MaybeUninit::uninit();
            gl.GenVertexArrays(1, vao.as_mut_ptr());
            let vao = vao.assume_init();
            gl.BindVertexArray(vao);
            Some(vao)
        } else {
            None
        };

        let mut vertex_buffer = mem::MaybeUninit::uninit();
        gl.GenBuffers(1, vertex_buffer.as_mut_ptr());
        let vertex_buffer = vertex_buffer.assume_init();
        gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (VERTICES.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            VERTICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let mut vbo_indices = mem::MaybeUninit::uninit();
        gl.GenBuffers(1, vbo_indices.as_mut_ptr());
        let vbo_indices = vbo_indices.assume_init();
        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_indices);
        gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDICES.len() * mem::size_of::<u16>()) as gl::types::GLsizeiptr,
            INDICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        if gl.BindVertexArray.is_loaded() {
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbo_indices);
            gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

            // Load the vertex position
            gl.VertexAttribPointer(
                attr_position as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * mem::size_of::<f32>()) as gl::types::GLsizei,
                ptr::null(),
            );

            // Load the texture coordinate
            gl.VertexAttribPointer(
                attr_texture as gl::types::GLuint,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * mem::size_of::<f32>()) as gl::types::GLsizei,
                (3 * mem::size_of::<f32>()) as *const () as *const _,
            );

            gl.EnableVertexAttribArray(attr_position as _);
            gl.EnableVertexAttribArray(attr_texture as _);

            gl.BindVertexArray(0);
        }

        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);

        assert_eq!(gl.GetError(), 0);

        (
            program,
            attr_position,
            attr_texture,
            vao,
            vertex_buffer,
            vbo_indices,
        )
    };

    Gl {
        gl,
        program,
        attr_position,
        attr_texture,
        vao,
        vertex_buffer,
        vbo_indices,
    }
}

#[derive(Debug)]
pub(crate) enum Message {
    Frame(gst_video::VideoInfo, gst::Buffer),
    BusMessage(gst::Message),
}

pub(crate) struct App {
    pipeline: gst::Pipeline,
    appsink: gst_app::AppSink,
    event_loop: Option<winit::event_loop::EventLoop<Message>>,
    window: Option<winit::window::Window>,
    not_current_gl_context: Option<glutin::context::NotCurrentContext>,
    glutin_context: gst_gl::GLContext,
    curr_frame: Option<gst_gl::GLVideoFrame<gst_gl::gl_video_frame::Readable>>,
    running_state: Option<(
        Gl,
        glutin::context::PossiblyCurrentContext,
        glutin::surface::Surface<glutin::surface::WindowSurface>,
    )>,
}

impl App {
    pub(crate) fn new(gl_element: Option<&gst::Element>) -> Result<App> {
        gst::init()?;

        let event_loop = winit::event_loop::EventLoop::with_user_event().build()?;

        let (pipeline, appsink) = App::create_pipeline(gl_element)?;
        let bus = pipeline
            .bus()
            .context("Pipeline without bus. Shouldn't happen!")?;

        // Only Windows requires the window to be present before creating a `glutin::Display`. Other
        // platforms don't really need one (and on Android, none exists until `Event::Resumed`).
        let window_attributes = cfg!(windows).then(|| {
            winit::window::Window::default_attributes()
                .with_transparent(true)
                .with_title("GL rendering")
        });

        let display_builder =
            glutin_winit::DisplayBuilder::new().with_window_attributes(window_attributes);
        // XXX on macOS/cgl only one config can be queried at a time. If transparency is needed,
        // add .with_transparency(true) to ConfigTemplateBuilder.  EGL on X11 doesn't support
        // transparency at all.
        let template = glutin::config::ConfigTemplateBuilder::new().with_alpha_size(8);
        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs
                    .reduce(|current, new_config| {
                        let prefer_transparency =
                            new_config.supports_transparency().unwrap_or(false)
                                & !current.supports_transparency().unwrap_or(false);

                        if prefer_transparency || new_config.num_samples() > current.num_samples() {
                            new_config
                        } else {
                            current
                        }
                    })
                    .unwrap()
            })
            .expect("Failed to build display");
        println!(
            "Picked a config with {} samples and transparency {}. Pixel format: {:?}",
            gl_config.num_samples(),
            gl_config.supports_transparency().unwrap_or(false),
            gl_config.color_buffer_type()
        );
        println!("Config supports GL API(s) {:?}", gl_config.api());

        // XXX The display could be obtained from any object created by it, so we can query it from
        // the config.
        let gl_display = gl_config.display();
        let raw_gl_display = gl_display.raw_display();

        println!("Using raw display connection {raw_gl_display:?}");

        let window_handle = window
            .as_ref()
            .map(|window| window.window_handle().unwrap());

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = glutin::context::ContextAttributesBuilder::new()
            .build(window_handle.map(|h| h.as_raw()));

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(None))
            .build(window_handle.map(|h| h.as_raw()));

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(
                glutin::context::Version::new(2, 1),
            )))
            .build(window_handle.map(|h| h.as_raw()));

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .or_else(|_| {
                            gl_display.create_context(&gl_config, &legacy_context_attributes)
                        })
                })
        }
        .context("failed to create context")?;

        let raw_gl_context = not_current_gl_context.raw_context();

        println!("Using raw GL context {raw_gl_context:?}");

        #[cfg(not(any(target_os = "linux", windows)))]
        compile_error!("This example only has Linux and Windows support");

        let api = App::map_gl_api(gl_config.api());

        let (raw_gl_context, gst_gl_display, platform) = match (raw_gl_display, raw_gl_context) {
            #[cfg(feature = "gst-gl-egl")]
            (
                glutin::display::RawDisplay::Egl(egl_display),
                glutin::context::RawContext::Egl(egl_context),
            ) => {
                let gl_display =
                    unsafe { gst_gl_egl::GLDisplayEGL::with_egl_display(egl_display as usize) }
                        .context("Failed to create GLDisplayEGL from raw `EGLDisplay`")?
                        .upcast::<gst_gl::GLDisplay>();
                (egl_context as usize, gl_display, gst_gl::GLPlatform::EGL)
            }
            #[cfg(feature = "gst-gl-x11")]
            (
                glutin::display::RawDisplay::Glx(glx_display),
                glutin::context::RawContext::Glx(glx_context),
            ) => {
                let gl_display =
                    unsafe { gst_gl_x11::GLDisplayX11::with_display(glx_display as usize) }
                        .context("Failed to create GLDisplayX11 from raw X11 `Display`")?
                        .upcast::<gst_gl::GLDisplay>();
                (glx_context as usize, gl_display, gst_gl::GLPlatform::GLX)
            }
            #[cfg(windows)]
            (glutin::display::RawDisplay::Wgl, glutin::context::RawContext::Wgl(wgl_context)) => {
                let gl_display = gst_gl::GLDisplay::new();
                (wgl_context as usize, gl_display, gst_gl::GLPlatform::WGL)
            }
            #[allow(unreachable_patterns)]
            handler => anyhow::bail!("Unsupported platform: {handler:?}."),
        };

        let glutin_context = unsafe {
            gst_gl::GLContext::new_wrapped(&gst_gl_display, raw_gl_context, platform, api)
        }
        .context("Couldn't wrap GL context")?;

        {
            // Make a new context that isn't the wrapped glutin context so that it can be made
            // current on a new "gstglcontext" thread (see `gst_gl_context_create_thread()`), while
            // the wrapped glutin context is made current on the winit event loop thread (this main
            // thread).
            let shared_context = gst_gl::GLContext::new(&gst_gl_display);
            shared_context
                .create(Some(&glutin_context))
                .context("Couldn't share wrapped Glutin GL context with new GL context")?;

            // Return the shared `GLContext` out of a pad probe for "gst.gl.local_context" to
            // make the underlying pipeline use it directly, instead of creating a new GL context
            // that is *shared* with the resulting context from a context `Query` (among other
            // elements) or `NeedContext` bus message for "gst.gl.app_context", as documented for
            // `gst_gl_ensure_element_data()`.
            //
            // On Windows, such context sharing calls `wglShareLists()` which fails on certain
            // drivers when one of the contexts is already current on another thread.  This would
            // happen because the pipeline and specifically the aforementioned "gstglcontext"
            // thread would be initialized asynchronously from the winit loop which makes our glutin
            // context current.  By calling `GLContext::create()` above, context sharing happens
            // directly.
            //
            // An alternative approach would be using `gst_gl::GLDisplay::add_context()` to store
            // the context inside `GLDisplay`, but the pad probe takes precedence.

            // While the pad probe could be installed anywhere, it makes logical sense to insert it
            // on the appsink where the images are extracted and displayed to a window via the same
            // GL contexts.
            appsink
                .static_pad("sink")
                .unwrap()
                .add_probe(PadProbeType::QUERY_DOWNSTREAM, move |pad, probe_info| {
                    if let Some(q) = probe_info.query_mut() {
                        if let QueryViewMut::Context(cq) = q.view_mut() {
                            if gst_gl::functions::gl_handle_context_query(
                                &pad.parent_element().unwrap(),
                                cq,
                                Some(&gst_gl_display),
                                Some(&shared_context),
                                None::<&gst_gl::GLContext>,
                            ) {
                                return PadProbeReturn::Handled;
                            }
                        }
                    }
                    PadProbeReturn::Ok
                })
                .unwrap();
        }

        let event_proxy = event_loop.create_proxy();

        #[allow(clippy::single_match)]
        bus.set_sync_handler(move |_bus, msg| {
            if let Err(e) = event_proxy
                // Forward all messages to winit's event loop
                .send_event(Message::BusMessage(msg.to_owned()))
            {
                eprintln!("Failed to send BusEvent to event proxy: {e}")
            }

            gst::BusSyncReply::Drop
        });
        let app = App {
            pipeline,
            appsink,
            event_loop: Some(event_loop),
            window,
            not_current_gl_context: Some(not_current_gl_context),
            glutin_context,
            curr_frame: None,
            running_state: None,
        };

        app.setup()?;

        Ok(app)
    }

    fn setup(&self) -> Result<()> {
        let event_loop = self.event_loop.as_ref().unwrap();
        let event_proxy = event_loop.create_proxy();
        self.appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;

                    let info = sample
                        .caps()
                        .and_then(|caps| gst_video::VideoInfo::from_caps(caps).ok())
                        .ok_or_else(|| {
                            element_error!(
                                appsink,
                                gst::ResourceError::Failed,
                                ("Failed to get video info from sample")
                            );

                            gst::FlowError::NotNegotiated
                        })?;

                    let mut buffer = sample.buffer_owned().unwrap();
                    {
                        let context = match (buffer.n_memory() > 0)
                            .then(|| buffer.peek_memory(0))
                            .and_then(|m| m.downcast_memory_ref::<gst_gl::GLBaseMemory>())
                            .map(|m| m.context())
                        {
                            Some(context) => context.clone(),
                            None => {
                                element_error!(
                                    appsink,
                                    gst::ResourceError::Failed,
                                    ("Failed to get GL context from buffer")
                                );

                                return Err(gst::FlowError::Error);
                            }
                        };

                        if let Some(meta) = buffer.meta::<gst_gl::GLSyncMeta>() {
                            meta.set_sync_point(&context);
                        } else {
                            let buffer = buffer.make_mut();
                            let meta = gst_gl::GLSyncMeta::add(buffer, &context);
                            meta.set_sync_point(&context);
                        }
                    }

                    event_proxy
                        .send_event(Message::Frame(info, buffer))
                        .map(|()| gst::FlowSuccess::Ok)
                        .map_err(|e| {
                            element_error!(
                                appsink,
                                gst::ResourceError::Failed,
                                ("Failed to send sample to event loop: {}", e)
                            );

                            gst::FlowError::Error
                        })
                })
                .build(),
        );

        Ok(())
    }

    /// Converts from <https://docs.rs/glutin/latest/glutin/config/struct.Api.html> to
    /// <https://gstreamer.freedesktop.org/documentation/gl/gstglapi.html?gi-language=c#GstGLAPI>.
    fn map_gl_api(api: glutin::config::Api) -> gst_gl::GLAPI {
        use glutin::config::Api;
        use gst_gl::GLAPI;

        let mut gst_gl_api = GLAPI::empty();
        // In gstreamer:
        // GLAPI::OPENGL: Desktop OpenGL up to and including 3.1. The compatibility profile when the OpenGL version is >= 3.2
        // GLAPI::OPENGL3: Desktop OpenGL >= 3.2 core profile
        // In glutin, API::OPENGL is set for every context API, except EGL where it is set based on
        // EGL_RENDERABLE_TYPE containing EGL_OPENGL_BIT:
        // https://registry.khronos.org/EGL/sdk/docs/man/html/eglChooseConfig.xhtml
        gst_gl_api.set(GLAPI::OPENGL | GLAPI::OPENGL3, api.contains(Api::OPENGL));
        gst_gl_api.set(GLAPI::GLES1, api.contains(Api::GLES1));
        // OpenGL ES 2.x and 3.x
        gst_gl_api.set(GLAPI::GLES2, api.intersects(Api::GLES2 | Api::GLES3));

        gst_gl_api
    }

    fn create_pipeline(
        gl_element: Option<&gst::Element>,
    ) -> Result<(gst::Pipeline, gst_app::AppSink)> {
        let pipeline = gst::Pipeline::default();
        let src = gst::ElementFactory::make("videotestsrc").build()?;

        let caps = gst_video::VideoCapsBuilder::new()
            .features([gst_gl::CAPS_FEATURE_MEMORY_GL_MEMORY])
            .format(gst_video::VideoFormat::Rgba)
            .field("texture-target", "2D")
            .build();

        let appsink = gst_app::AppSink::builder()
            .enable_last_sample(true)
            .max_buffers(1)
            .caps(&caps)
            .build();

        if let Some(gl_element) = gl_element {
            let glupload = gst::ElementFactory::make("glupload").build()?;

            pipeline.add_many([&src, &glupload])?;
            pipeline.add(gl_element)?;
            pipeline.add(&appsink)?;

            src.link(&glupload)?;
            glupload.link(gl_element)?;
            gl_element.link(&appsink)?;

            Ok((pipeline, appsink))
        } else {
            let sink = gst::ElementFactory::make("glsinkbin")
                .property("sink", &appsink)
                .build()?;

            pipeline.add_many([&src, &sink])?;
            src.link(&sink)?;

            Ok((pipeline, appsink))
        }
    }

    /// Should be called from within the event loop
    fn handle_message(msg: gst::Message) {
        use gst::MessageView;

        // Only handle error messages by panicking, to hard-stop the event loop
        if let MessageView::Error(err) = msg.view() {
            let src = msg
                .src()
                .map(|s| s.path_string())
                .unwrap_or_else(|| glib::GString::from("UNKNOWN"));
            let error = err.error();
            let debug = err.debug();
            panic!("Received error from {src}: {error} (debug: {debug:?})");
        }
    }

    /// Should be called from within the event loop
    fn redraw(&self) {
        if let Some((gl, gl_context, gl_surface)) = &self.running_state {
            if let Some(frame) = self.curr_frame.as_ref() {
                let sync_meta = frame.buffer().meta::<gst_gl::GLSyncMeta>().unwrap();
                sync_meta.wait(&self.glutin_context);
                if let Ok(texture) = frame.texture_id(0) {
                    gl.draw_frame(texture as gl::types::GLuint);
                }
            }

            gl_surface.swap_buffers(gl_context).unwrap();
        }
    }

    pub fn run(mut self) -> Result<()> {
        let Some(event_loop) = self.event_loop.take() else {
            return Ok(());
        };

        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl winit::application::ApplicationHandler<Message> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let not_current_gl_context = self
            .not_current_gl_context
            .take()
            .expect("There must be a NotCurrentContext prior to Event::Resumed");

        let gl_config = not_current_gl_context.config();
        let gl_display = gl_config.display();

        let window = self.window.get_or_insert_with(|| {
            let window_attributes =
                winit::window::Window::default_attributes().with_transparent(true);
            glutin_winit::finalize_window(event_loop, window_attributes, &gl_config).unwrap()
        });

        let attrs = window.build_surface_attributes(<_>::default()).unwrap();
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        // Make it current.
        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        // Tell GStreamer that the context has been made current (for borrowed contexts,
        // this does not try to make it current again)
        self.glutin_context.activate(true).unwrap();

        self.glutin_context
            .fill_info()
            .expect("Couldn't fill context info");

        // The context needs to be current for the Renderer to set up shaders and buffers.
        // It also performs function loading, which needs a current context on WGL.
        let gl = load(&gl_display);

        // Try setting vsync.
        if let Err(res) = gl_surface.set_swap_interval(
            &gl_context,
            glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap()),
        ) {
            eprintln!("Error setting vsync: {res:?}");
        }

        self.pipeline.set_state(gst::State::Playing).unwrap();

        assert!(self
            .running_state
            .replace((gl, gl_context, gl_surface))
            .is_none());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

        match event {
            winit::event::WindowEvent::CloseRequested
            | winit::event::WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        state: winit::event::ElementState::Released,
                        logical_key: winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                self.curr_frame = None;
                self.pipeline.send_event(gst::event::Eos::new());
                self.pipeline.set_state(gst::State::Null).unwrap();
                event_loop.exit();
            }
            winit::event::WindowEvent::Resized(size) => {
                // Some platforms like EGL require resizing GL surface to update the size
                // Notable platforms here are Wayland and macOS, other don't require it
                // and the function is no-op, but it's wise to resize it for portability
                // reasons.
                if let Some((gl, gl_context, gl_surface)) = &self.running_state {
                    gl_surface.resize(
                        gl_context,
                        // XXX Ignore minimizing
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    );
                    gl.resize(size);
                }
            }
            winit::event::WindowEvent::RedrawRequested => {
                self.redraw();
            }
            _ => (),
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: Message) {
        match event {
            // Receive a frame
            Message::Frame(info, buffer) => {
                if let Ok(frame) = gst_gl::GLVideoFrame::from_buffer_readable(buffer, &info) {
                    self.curr_frame = Some(frame);
                    self.redraw();
                }
            }
            // Handle all pending messages when we are awaken by set_sync_handler
            Message::BusMessage(msg) => App::handle_message(msg),
        }
    }
}
