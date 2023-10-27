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
use derive_more::{Display, Error};
use glutin::{
    config::GetGlConfig as _,
    context::AsRawContext as _,
    display::{AsRawDisplay as _, GetGlDisplay as _},
    prelude::*,
};
use glutin_winit::GlWindow as _;
use gst::element_error;
use gst_gl::prelude::*;
use raw_window_handle::HasRawWindowHandle as _;

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

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

const VS_SRC: &[u8] = b"
uniform mat4 u_transformation;
attribute vec4 a_position;
attribute vec2 a_texcoord;
varying vec2 v_texcoord;

void main() {
    gl_Position = u_transformation * a_position;
    v_texcoord = a_texcoord;
}
\0";

const FS_SRC: &[u8] = b"
#ifdef GL_ES
precision mediump float;
#endif
varying vec2 v_texcoord;
uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, v_texcoord);
}
\0";

#[allow(clippy::unreadable_literal)]
#[allow(clippy::unused_unit)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::manual_non_exhaustive)]
#[allow(clippy::upper_case_acronyms)]
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
                .GetUniformLocation(self.program, b"tex\0".as_ptr() as *const _);
            self.gl.Uniform1i(location, 0);

            let location = self
                .gl
                .GetUniformLocation(self.program, b"u_transformation\0".as_ptr() as *const _);
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

        let attr_position = gl.GetAttribLocation(program, b"a_position\0".as_ptr() as *const _);
        let attr_texture = gl.GetAttribLocation(program, b"a_texcoord\0".as_ptr() as *const _);

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
enum Message {
    Frame(gst_video::VideoInfo, gst::Buffer),
    BusEvent,
}

pub(crate) struct App {
    pipeline: gst::Pipeline,
    appsink: gst_app::AppSink,
    bus: gst::Bus,
    event_loop: winit::event_loop::EventLoop<Message>,
    window: Option<winit::window::Window>,
    not_current_gl_context: Option<glutin::context::NotCurrentContext>,
    shared_context: gst_gl::GLContext,
}

impl App {
    pub(crate) fn new(gl_element: Option<&gst::Element>) -> Result<App> {
        gst::init()?;

        let (pipeline, appsink) = App::create_pipeline(gl_element)?;
        let bus = pipeline
            .bus()
            .context("Pipeline without bus. Shouldn't happen!")?;

        let event_loop = winit::event_loop::EventLoopBuilder::with_user_event().build()?;

        // Only Windows requires the window to be present before creating a `glutin::Display`. Other
        // platforms don't really need one (and on Android, none exists until `Event::Resumed`).
        let window_builder = cfg!(windows).then(|| {
            winit::window::WindowBuilder::new()
                .with_transparent(true)
                .with_title("GL rendering")
        });

        let display_builder =
            glutin_winit::DisplayBuilder::new().with_window_builder(window_builder);
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

        println!("Using raw display connection {:?}", raw_gl_display);

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes =
            glutin::context::ContextAttributesBuilder::new().build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(None))
            .build(raw_window_handle);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(
                glutin::context::Version::new(2, 1),
            )))
            .build(raw_window_handle);

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

        println!("Using raw GL context {:?}", raw_gl_context);

        #[cfg(not(target_os = "linux"))]
        compile_error!("This example only has Linux support");

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
            #[allow(unreachable_patterns)]
            handler => anyhow::bail!("Unsupported platform: {handler:?}."),
        };

        let shared_context = unsafe {
            gst_gl::GLContext::new_wrapped(&gst_gl_display, raw_gl_context, platform, api)
        }
        .context("Couldn't wrap GL context")?;

        let gl_context = shared_context.clone();
        let event_proxy = event_loop.create_proxy();

        #[allow(clippy::single_match)]
        bus.set_sync_handler(move |_, msg| {
            match msg.view() {
                gst::MessageView::NeedContext(ctxt) => {
                    let context_type = ctxt.context_type();
                    if context_type == *gst_gl::GL_DISPLAY_CONTEXT_TYPE {
                        if let Some(el) =
                            msg.src().map(|s| s.downcast_ref::<gst::Element>().unwrap())
                        {
                            let context = gst::Context::new(context_type, true);
                            context.set_gl_display(&gst_gl_display);
                            el.set_context(&context);
                        }
                    }
                    if context_type == "gst.gl.app_context" {
                        if let Some(el) =
                            msg.src().map(|s| s.downcast_ref::<gst::Element>().unwrap())
                        {
                            let mut context = gst::Context::new(context_type, true);
                            {
                                let context = context.get_mut().unwrap();
                                let s = context.structure_mut();
                                s.set("context", &gl_context);
                            }
                            el.set_context(&context);
                        }
                    }
                }
                _ => (),
            }

            if let Err(e) = event_proxy.send_event(Message::BusEvent) {
                eprintln!("Failed to send BusEvent to event proxy: {e}")
            }

            gst::BusSyncReply::Pass
        });

        Ok(App {
            pipeline,
            appsink,
            bus,
            event_loop,
            window,
            not_current_gl_context: Some(not_current_gl_context),
            shared_context,
        })
    }

    fn setup(&self, event_loop: &winit::event_loop::EventLoop<Message>) -> Result<()> {
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

    fn handle_messages(bus: &gst::Bus) -> Result<()> {
        use gst::MessageView;

        for msg in bus.iter() {
            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    return Err(ErrorMessage {
                        src: msg
                            .src()
                            .map(|s| s.path_string())
                            .unwrap_or_else(|| glib::GString::from("UNKNOWN")),
                        error: err.error(),
                        debug: err.debug(),
                    }
                    .into());
                }
                _ => (),
            }
        }

        Ok(())
    }
}

pub(crate) fn main_loop(app: App) -> Result<()> {
    app.setup(&app.event_loop)?;

    let App {
        pipeline,
        bus,
        event_loop,
        mut window,
        mut not_current_gl_context,
        shared_context,
        ..
    } = app;

    let mut curr_frame: Option<gst_video::VideoFrame<gst_video::video_frame::Readable>> = None;

    let mut running_state = None::<(
        Gl,
        glutin::context::PossiblyCurrentContext,
        glutin::surface::Surface<glutin::surface::WindowSurface>,
    )>;

    Ok(event_loop.run(move |event, window_target| {
        window_target.set_control_flow(winit::event_loop::ControlFlow::Wait);

        let mut needs_redraw = false;
        match event {
            winit::event::Event::LoopExiting => {
                pipeline.send_event(gst::event::Eos::new());
                pipeline.set_state(gst::State::Null).unwrap();
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested
                | winit::event::WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            state: winit::event::ElementState::Released,
                            logical_key:
                                winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                            ..
                        },
                    ..
                } => window_target.exit(),
                winit::event::WindowEvent::Resized(size) => {
                    // Some platforms like EGL require resizing GL surface to update the size
                    // Notable platforms here are Wayland and macOS, other don't require it
                    // and the function is no-op, but it's wise to resize it for portability
                    // reasons.
                    if let Some((gl, gl_context, gl_surface)) = &running_state {
                        gl_surface.resize(
                            gl_context,
                            // XXX Ignore minimizing
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );
                        gl.resize(size);
                    }
                }
                winit::event::WindowEvent::RedrawRequested => needs_redraw = true,
                _ => (),
            },
            // Receive a frame
            winit::event::Event::UserEvent(Message::Frame(info, buffer)) => {
                if let Ok(frame) = gst_video::VideoFrame::from_buffer_readable_gl(buffer, &info) {
                    curr_frame = Some(frame);
                    needs_redraw = true;
                }
            }
            // Handle all pending messages when we are awaken by set_sync_handler
            winit::event::Event::UserEvent(Message::BusEvent) => {
                App::handle_messages(&bus).unwrap();
            }
            winit::event::Event::Resumed => {
                let not_current_gl_context = not_current_gl_context
                    .take()
                    .expect("There must be a NotCurrentContext prior to Event::Resumed");

                let gl_config = not_current_gl_context.config();
                let gl_display = gl_config.display();

                let window = window.get_or_insert_with(|| {
                    let window_builder = winit::window::WindowBuilder::new().with_transparent(true);
                    glutin_winit::finalize_window(window_target, window_builder, &gl_config)
                        .unwrap()
                });

                let attrs = window.build_surface_attributes(<_>::default());
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
                shared_context.activate(true).unwrap();

                shared_context
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

                pipeline.set_state(gst::State::Playing).unwrap();

                assert!(running_state
                    .replace((gl, gl_context, gl_surface))
                    .is_none());
            }
            _ => (),
        }

        if needs_redraw {
            if let Some((gl, gl_context, gl_surface)) = &running_state {
                if let Some(frame) = curr_frame.as_ref() {
                    let sync_meta = frame.buffer().meta::<gst_gl::GLSyncMeta>().unwrap();
                    sync_meta.wait(&shared_context);
                    if let Some(texture) = frame.texture_id(0) {
                        gl.draw_frame(texture as gl::types::GLuint);
                    }
                }

                gl_surface.swap_buffers(gl_context).unwrap();
            }
        }
    })?)
}
