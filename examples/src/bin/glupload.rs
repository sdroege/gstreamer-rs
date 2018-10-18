#[macro_use]
extern crate gstreamer as gst;
use gst::prelude::*;
use gst::MessageView;

extern crate gstreamer_app as gst_app;
extern crate gstreamer_gl as gst_gl;

extern crate glib;

use std::error::Error as StdError;
use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::thread;

extern crate failure;
use failure::Error;

#[macro_use]
extern crate failure_derive;

extern crate glutin;
use glutin::GlContext;

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Fail)]
#[fail(display = "Missing element {}", _0)]
struct MissingElement(&'static str);

#[derive(Debug, Fail)]
#[fail(
    display = "Received error from {}: {} (debug: {:?})",
    src,
    error,
    debug
)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    #[cause]
    cause: glib::Error,
}

#[cfg(target_endian = "big")]
const CAPS_FORMAT: &str = "{ BGRx, BGRA }";
#[cfg(target_endian = "little")]
const CAPS_FORMAT: &str = "{ xRGB, ARGB }";

static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 0.5, 0.0, 1.0, 0.0, 0.5, -0.5, 0.0, 0.0, 1.0,
];

const VS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/test_gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
}

pub fn load(gl_context: &glutin::Context) -> Gl {
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

    unsafe {
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
        gl.UseProgram(program);

        let mut vb = mem::uninitialized();
        gl.GenBuffers(1, &mut vb);
        gl.BindBuffer(gl::ARRAY_BUFFER, vb);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        if gl.BindVertexArray.is_loaded() {
            let mut vao = mem::uninitialized();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
        let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
        gl.VertexAttribPointer(
            pos_attrib as gl::types::GLuint,
            2,
            gl::FLOAT,
            0,
            5 * mem::size_of::<f32>() as gl::types::GLsizei,
            ptr::null(),
        );
        gl.VertexAttribPointer(
            color_attrib as gl::types::GLuint,
            3,
            gl::FLOAT,
            0,
            5 * mem::size_of::<f32>() as gl::types::GLsizei,
            (2 * mem::size_of::<f32>()) as *const () as *const _,
        );
        gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
        gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);
    }

    Gl { gl: gl }
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::new(None);
    let src =
        gst::ElementFactory::make("videotestsrc", None).ok_or(MissingElement("videotestsrc"))?;
    let upload = gst::ElementFactory::make("glupload", None).ok_or(MissingElement("glupload"))?;
    let colorconvert = gst::ElementFactory::make("glcolorconvert", None)
        .ok_or(MissingElement("glcolorconvert"))?;
    let sink = gst::ElementFactory::make("appsink", None).ok_or(MissingElement("appsink"))?;

    pipeline.add_many(&[&src, &upload, &colorconvert, &sink])?;
    gst::Element::link_many(&[&src, &upload, &colorconvert, &sink])?;

    let appsink = sink
        .dynamic_cast::<gst_app::AppSink>()
        .expect("Sink element is expected to be an appsink!");

    appsink.set_property("enable-last-sample", &false.to_value())?;
    appsink.set_property("emit-signals", &false.to_value())?;
    appsink.set_property("max-buffers", &1u32.to_value())?;

    let mediatype = format!(
        "video/x-raw({}), format = (string) {}",
        &gst_gl::CAPS_FEATURE_MEMORY_GL_MEMORY.to_string(),
        &CAPS_FORMAT.to_string()
    );
    appsink.set_caps(&gst::Caps::from_string(&mediatype.as_str()));

    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::new()
            .new_sample(|appsink| {
                let sample = match appsink.pull_sample() {
                    None => return gst::FlowReturn::Eos,
                    Some(sample) => sample,
                };

                let _buffer = if let Some(buffer) = sample.get_buffer() {
                    buffer
                } else {
                    gst_element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    return gst::FlowReturn::Error;
                };

                gst::FlowReturn::Ok
            })
            .build(),
    );

    Ok(pipeline)
}

fn gst_loop(bus: gst::Bus) -> Result<(), Error> {
    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                Err(ErrorMessage {
                    src: err
                        .get_src()
                        .map(|s| s.get_path_string())
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().description().into(),
                    debug: err.get_debug(),
                    cause: err.get_error(),
                })?;
            }
            MessageView::NeedContext(ctxt) => {
                // ensure_gst_gl_context()
                let context_type = ctxt.get_context_type();
                if context_type == *gst_gl::GL_DISPLAY_CONTEXT_TYPE {
                    unimplemented!();
                }
                if context_type == "gst.gl.app_context" {
                    unimplemented!();
                }
            }
            _ => (),
        }
    }

    Ok(())
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("GL rendering");
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    println!(
        "Pixel format of the window's GL context {:?}",
        gl_window.get_pixel_format()
    );

    let gl = load(&gl_window.context());

    pipeline.set_state(gst::State::Playing).into_result()?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    let bus_clone = bus.clone();
    let bus_handler = thread::spawn(move || {
        let ret = gst_loop(bus_clone);
        if ret.is_err() {
            eprintln!("ERROR! {:?}", ret);
        }
    });

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            //println!("{:?}", event);
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => (),
                },
                _ => (),
            }
        });

        let _ = gl_window.swap_buffers();
    }

    pipeline.set_state(gst::State::Null).into_result()?;

    Ok(())
}

fn example_main() {
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}

fn main() {
    examples_common::run(example_main);
}
