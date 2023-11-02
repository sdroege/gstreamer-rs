// This example demonstrates how to output GL textures, within an
// EGL/X11 context provided by the application, and render those
// textures in the GL application.

// {videotestsrc} - { glsinkbin }

use std::{ffi::CStr, mem, ptr, sync};

use anyhow::Error;
use derive_more::{Display, Error};
use gst::element_error;
use gst_gl::prelude::*;

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

    fn resize(&self, size: glutin::dpi::PhysicalSize<u32>) {
        unsafe {
            self.gl
                .Viewport(0, 0, size.width as i32, size.height as i32);
        }
    }
}

fn load(gl_context: &glutin::WindowedContext<glutin::PossiblyCurrent>) -> Gl {
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
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
            let mut success: gl::types::GLint = 1;
            gl.GetProgramiv(fs, gl::LINK_STATUS, &mut success);
            assert!(success != 0);
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
    event_loop: glutin::event_loop::EventLoop<Message>,
    windowed_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    shared_context: gst_gl::GLContext,
}

impl App {
    pub(crate) fn new(gl_element: Option<&gst::Element>) -> Result<App, Error> {
        gst::init()?;

        let (pipeline, appsink) = App::create_pipeline(gl_element)?;
        let bus = pipeline
            .bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        let event_loop = glutin::event_loop::EventLoopBuilder::with_user_event().build();
        let window = glutin::window::WindowBuilder::new().with_title("GL rendering");
        let windowed_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window, &event_loop)?;

        let windowed_context = unsafe { windowed_context.make_current().map_err(|(_, err)| err)? };

        #[cfg(any(feature = "gst-gl-x11"))]
        let inner_window = windowed_context.window();

        let shared_context: gst_gl::GLContext;
        if cfg!(target_os = "linux") {
            #[cfg(any(feature = "gst-gl-x11"))]
            use glutin::platform::unix::WindowExtUnix;
            use glutin::platform::{unix::RawHandle, ContextTraitExt};

            let api = App::map_gl_api(windowed_context.get_api());

            let (gl_context, gl_display, platform) = match unsafe { windowed_context.raw_handle() }
            {
                #[cfg(any(feature = "gst-gl-egl"))]
                RawHandle::Egl(egl_context) => {
                    let gl_display =
                        if let Some(display) = unsafe { windowed_context.get_egl_display() } {
                            unsafe { gst_gl_egl::GLDisplayEGL::with_egl_display(display as usize) }
                                .unwrap()
                        } else {
                            panic!("EGL window without EGL Display")
                        };

                    (
                        egl_context as usize,
                        gl_display.upcast::<gst_gl::GLDisplay>(),
                        gst_gl::GLPlatform::EGL,
                    )
                }
                #[cfg(feature = "gst-gl-x11")]
                RawHandle::Glx(glx_context) => {
                    let gl_display = if let Some(display) = inner_window.xlib_display() {
                        unsafe { gst_gl_x11::GLDisplayX11::with_display(display as usize) }.unwrap()
                    } else {
                        panic!("X11 window without X Display")
                    };

                    (
                        glx_context as usize,
                        gl_display.upcast::<gst_gl::GLDisplay>(),
                        gst_gl::GLPlatform::GLX,
                    )
                }
                #[allow(unreachable_patterns)]
                handler => panic!("Unsupported platform: {handler:?}."),
            };

            shared_context =
                unsafe { gst_gl::GLContext::new_wrapped(&gl_display, gl_context, platform, api) }
                    .unwrap();

            shared_context
                .activate(true)
                .expect("Couldn't activate wrapped GL context");

            shared_context.fill_info()?;

            let gl_context = shared_context.clone();
            let event_proxy = sync::Mutex::new(event_loop.create_proxy());

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
                                context.set_gl_display(&gl_display);
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

                if let Err(e) = event_proxy.lock().unwrap().send_event(Message::BusEvent) {
                    eprintln!("Failed to send BusEvent to event proxy: {e}")
                }

                gst::BusSyncReply::Pass
            });
        } else {
            panic!("This example only has Linux support");
        }

        Ok(App {
            pipeline,
            appsink,
            bus,
            event_loop,
            windowed_context,
            shared_context,
        })
    }

    fn setup(&self, event_loop: &glutin::event_loop::EventLoop<Message>) -> Result<(), Error> {
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

        self.pipeline.set_state(gst::State::Playing)?;

        Ok(())
    }

    fn map_gl_api(api: glutin::Api) -> gst_gl::GLAPI {
        match api {
            glutin::Api::OpenGl => gst_gl::GLAPI::OPENGL3,
            glutin::Api::OpenGlEs => gst_gl::GLAPI::GLES2,
            _ => gst_gl::GLAPI::empty(),
        }
    }

    fn create_pipeline(
        gl_element: Option<&gst::Element>,
    ) -> Result<(gst::Pipeline, gst_app::AppSink), Error> {
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

    fn handle_messages(bus: &gst::Bus) -> Result<(), Error> {
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

pub(crate) fn main_loop(app: App) -> Result<(), Error> {
    app.setup(&app.event_loop)?;

    println!(
        "Pixel format of the window's GL context {:?}",
        app.windowed_context.get_pixel_format()
    );

    let gl = load(&app.windowed_context);

    let mut curr_frame: Option<gst_gl::GLVideoFrame<gst_gl::gl_video_frame::Readable>> = None;

    let App {
        bus,
        event_loop,
        pipeline,
        shared_context,
        windowed_context,
        ..
    } = app;

    event_loop.run(move |event, _, cf| {
        *cf = glutin::event_loop::ControlFlow::Wait;

        let mut needs_redraw = false;
        match event {
            glutin::event::Event::LoopDestroyed => {
                pipeline.send_event(gst::event::Eos::new());
                pipeline.set_state(gst::State::Null).unwrap();
            }
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested
                | glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            state: glutin::event::ElementState::Released,
                            virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *cf = glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    gl.resize(physical_size);
                }
                _ => (),
            },
            glutin::event::Event::RedrawRequested(_) => needs_redraw = true,
            // Receive a frame
            glutin::event::Event::UserEvent(Message::Frame(info, buffer)) => {
                if let Ok(frame) = gst_gl::GLVideoFrame::from_buffer_readable(buffer, &info) {
                    curr_frame = Some(frame);
                    needs_redraw = true;
                }
            }
            // Handle all pending messages when we are awaken by set_sync_handler
            glutin::event::Event::UserEvent(Message::BusEvent) => {
                App::handle_messages(&bus).unwrap();
            }
            _ => (),
        }

        if needs_redraw {
            if let Some(frame) = curr_frame.as_ref() {
                let sync_meta = frame.buffer().meta::<gst_gl::GLSyncMeta>().unwrap();
                sync_meta.wait(&shared_context);
                if let Ok(texture) = frame.texture_id(0) {
                    gl.draw_frame(texture as gl::types::GLuint);
                }
            }
            windowed_context.swap_buffers().unwrap();
        }
    })
}
