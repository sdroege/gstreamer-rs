#![allow(clippy::non_send_fields_in_send_ty)]

use anyhow::Error;
use derive_more::{Display, Error};
use gst::prelude::*;

#[path = "../examples-common.rs"]
pub mod examples_common;

#[derive(Debug, Display, Error)]
#[display("Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

mod mirror {
    use gst_base::subclass::*;
    use gst_vulkan::prelude::*;
    use gst_vulkan::subclass::prelude::*;
    use std::sync::LazyLock;

    pub static CAT: LazyLock<gst::DebugCategory> = LazyLock::new(|| {
        gst::DebugCategory::new(
            "rsvulkanmirrorfilter",
            gst::DebugColorFlags::empty(),
            Some("Rust Vulkan Mirror Filter"),
        )
    });

    glib::wrapper! {
        pub struct VulkanMirrorFilter(ObjectSubclass<imp::VulkanMirrorFilter>) @extends gst_vulkan::VulkanVideoFilter, gst_base::BaseTransform, gst::Element, gst::Object;
    }

    impl VulkanMirrorFilter {
        pub fn new(name: Option<&str>) -> Self {
            glib::Object::builder().property("name", name).build()
        }
    }

    mod imp {
        use super::*;

        use std::sync::{Arc, Mutex};

        // Uses the naga crate to transform wgsl shader code to SPIR-V
        fn compile_shader(wgsl_source: &str) -> Result<Vec<u8>, gst::ErrorMessage> {
            use naga::valid::*;

            let module = naga::front::wgsl::parse_str(wgsl_source).map_err(|e| {
                gst::error_msg!(
                    gst::ResourceError::Failed,
                    ("Shader compilation error: {e:#?}")
                )
            })?;

            let info = Validator::new(ValidationFlags::all(), Capabilities::all())
                .validate(&module)
                .map_err(|e| {
                    gst::error_msg!(
                        gst::ResourceError::Failed,
                        ("Shader validation error: {e:#?}")
                    )
                })?;

            let mut words = Vec::<u32>::new();
            naga::back::spv::Writer::new(&naga::back::spv::Options::default())
                .unwrap()
                .write(&module, &info, None, &None, &mut words)
                .unwrap();

            Ok(words
                .into_iter()
                .flat_map(u32::to_be_bytes)
                .collect::<Vec<_>>())
        }

        /// Private data consists of the transformation shader which is compiled
        /// in advance to running the actual filter.
        #[derive(Default)]
        pub struct VulkanMirrorFilter {
            state: Arc<Mutex<State>>,
        }

        #[derive(Debug, Default)]
        struct State {
            render: Option<gst_vulkan::VulkanFullScreenQuad>,
        }

        // See `subclass.rs` for general documentation on creating a subclass. Extended
        // information like element metadata have been omitted for brevity.
        #[glib::object_subclass]
        impl ObjectSubclass for VulkanMirrorFilter {
            const NAME: &'static str = "RsVulkanMirrorFilter";
            type Type = super::VulkanMirrorFilter;
            type ParentType = gst_vulkan::VulkanVideoFilter;
        }

        impl ElementImpl for VulkanMirrorFilter {
            fn pad_templates() -> &'static [gst::PadTemplate] {
                static PAD_TEMPLATES: std::sync::OnceLock<Vec<gst::PadTemplate>> =
                    std::sync::OnceLock::new();

                PAD_TEMPLATES.get_or_init(|| {
                    let caps = gst::Caps::builder_full()
                        .structure_with_features(
                            gst::Structure::builder("video/x-raw")
                                .field("format", gst::List::new(["BGRA", "RGBA"]))
                                .field("width", gst::IntRange::new(1, i32::MAX))
                                .field("height", gst::IntRange::new(1, i32::MAX))
                                .build(),
                            gst_vulkan::CAPS_FEATURES_MEMORY_VULKAN_IMAGE.clone(),
                        )
                        .build();
                    vec![
                        gst::PadTemplate::new(
                            "src",
                            gst::PadDirection::Src,
                            gst::PadPresence::Always,
                            &caps,
                        )
                        .unwrap(),
                        gst::PadTemplate::new(
                            "sink",
                            gst::PadDirection::Sink,
                            gst::PadPresence::Always,
                            &caps,
                        )
                        .unwrap(),
                    ]
                })
            }
        }
        impl GstObjectImpl for VulkanMirrorFilter {}
        impl ObjectImpl for VulkanMirrorFilter {}
        impl VulkanVideoFilterImpl for VulkanMirrorFilter {}
        impl BaseTransformImpl for VulkanMirrorFilter {
            const MODE: BaseTransformMode = BaseTransformMode::NeverInPlace;
            const PASSTHROUGH_ON_SAME_CAPS: bool = false;
            const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;

            fn start(&self) -> Result<(), gst::ErrorMessage> {
                self.parent_start()?;

                let vertex_spirv = compile_shader(include_str!("vulkanfilter.vert.wgsl"))?;
                let fragment_spirv = compile_shader(include_str!("vulkanfilter.frag.wgsl"))?;

                let queue = self.obj().queue().ok_or_else(|| {
                    gst::error_msg!(gst::ResourceError::NotFound, ("No Vulkan Queue!"))
                })?;
                let device = queue.device().ok_or_else(|| {
                    gst::error_msg!(gst::ResourceError::NotFound, ("No Vulkan Device!"))
                })?;
                let render = gst_vulkan::VulkanFullScreenQuad::new(&queue);
                let vertex = device.create_shader(&vertex_spirv).map_err(|e| {
                    gst::error_msg!(
                        gst::ResourceError::NotFound,
                        ("Vertex shader creation failed {e:?}")
                    )
                })?;
                let fragment = device.create_shader(&fragment_spirv).map_err(|e| {
                    gst::error_msg!(
                        gst::ResourceError::NotFound,
                        ("Fragment shader creation failed {e:?}")
                    )
                })?;
                if !render.set_shaders(&vertex, &fragment) {
                    return Err(gst::error_msg!(
                        gst::ResourceError::Failed,
                        ("Failed to set shaders")
                    ));
                }
                let mut state = self.state.lock().unwrap();
                state.render = Some(render);
                Ok(())
            }

            fn stop(&self) -> Result<(), gst::ErrorMessage> {
                self.parent_stop()?;

                let mut state = self.state.lock().unwrap();
                state.render = None;
                Ok(())
            }

            fn set_caps(
                &self,
                input: &gst::Caps,
                output: &gst::Caps,
            ) -> Result<(), gst::LoggableError> {
                self.parent_set_caps(input, output)
                    .map_err(|_| gst::loggable_error!(CAT, "parent failed set_caps"))?;

                let state = self.state.lock().unwrap();
                let render = state
                    .render
                    .as_ref()
                    .ok_or_else(|| gst::loggable_error!(CAT, "No internal renderer!"))?;
                let in_info = gst_video::VideoInfo::from_caps(input)
                    .map_err(|_| gst::loggable_error!(CAT, "Failed to parse input caps"))?;
                let out_info = gst_video::VideoInfo::from_caps(input)
                    .map_err(|_| gst::loggable_error!(CAT, "Failed to parse output caps"))?;
                if !render.set_info(&in_info, &out_info) {
                    return Err(gst::loggable_error!(CAT, "Failed to set caps on renderer"));
                }
                Ok(())
            }

            fn transform(
                &self,
                inbuf: &gst::Buffer,
                outbuf: &mut gst::BufferRef,
            ) -> Result<gst::FlowSuccess, gst::FlowError> {
                gst::info!(CAT, "start transform");
                let mut state = self.state.lock().unwrap();
                let render = state.render.as_mut().ok_or(gst::FlowError::Error)?;
                render
                    .set_input_buffer(Some(inbuf))
                    .map_err(|_| gst::FlowError::Error)?;
                gst::info!(CAT, "start draw");
                render.draw_into_output(outbuf).map_err(|e| {
                    gst::error!(CAT, "Failed to render output: {e:?}");
                    gst::FlowError::Error
                })?;
                gst::info!(CAT, "done success");
                Ok(gst::FlowSuccess::Ok)
            }
        }
    }
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init().unwrap();

    let pipeline = gst::Pipeline::new();
    let src = gst::ElementFactory::make("videotestsrc").build()?;
    let upload = gst::ElementFactory::make("vulkanupload").build()?;
    let vulkanfilter = mirror::VulkanMirrorFilter::new(Some("Mirror filter"));
    let sink = gst::ElementFactory::make("vulkansink").build()?;

    pipeline.add_many([&src, &upload, vulkanfilter.upcast_ref(), &sink])?;
    gst::Element::link_many([&src, &upload, vulkanfilter.upcast_ref(), &sink])?;

    pipeline.set_state(gst::State::Playing)?;

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
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

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn example_main() {
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}

fn main() {
    examples_common::run(example_main)
}
