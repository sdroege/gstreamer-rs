// This example demonstrates how to implement a custom compositor based on cairo.
#![allow(clippy::non_send_fields_in_send_ty)]

use gst::prelude::*;
use gst_base::prelude::*;

use anyhow::{Context, Error};

#[path = "../examples-common.rs"]
mod examples_common;

// Our custom compositor element is defined in this module.
mod cairo_compositor {
    use super::*;
    use gst_base::subclass::prelude::*;
    use gst_video::prelude::*;
    use gst_video::subclass::prelude::*;

    use once_cell::sync::Lazy;

    // In the imp submodule we include the actual implementation of the compositor.
    mod imp {
        use super::*;

        use std::sync::Mutex;

        // Settings of the compositor.
        #[derive(Clone)]
        struct Settings {
            background_color: u32,
        }

        impl Default for Settings {
            fn default() -> Self {
                Self {
                    background_color: 0xff_00_00_00,
                }
            }
        }

        // This is the private data of our compositor.
        #[derive(Default)]
        pub struct CairoCompositor {
            settings: Mutex<Settings>,
        }

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data.
        #[glib::object_subclass]
        impl ObjectSubclass for CairoCompositor {
            const NAME: &'static str = "CairoCompositor";
            type Type = super::CairoCompositor;
            type ParentType = gst_video::VideoAggregator;
            type Interfaces = (gst::ChildProxy,);
        }

        // Implementation of glib::Object virtual methods.
        impl ObjectImpl for CairoCompositor {
            // Specfication of the compositor properties.
            // In this case a single property for configuring the background color of the
            // composition.
            fn properties() -> &'static [glib::ParamSpec] {
                static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                    vec![glib::ParamSpecUInt::builder("background-color")
                        .nick("Background Color")
                        .blurb("Background color as 0xRRGGBB")
                        .default_value(Settings::default().background_color)
                        .build()]
                });

                &*PROPERTIES
            }

            // Called by the application whenever the value of a property should be changed.
            fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
                let mut settings = self.settings.lock().unwrap();

                match pspec.name() {
                    "background-color" => {
                        settings.background_color = value.get().unwrap();
                    }
                    _ => unimplemented!(),
                };
            }

            // Called by the application whenever the value of a property should be retrieved.
            fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
                let settings = self.settings.lock().unwrap();

                match pspec.name() {
                    "background-color" => settings.background_color.to_value(),
                    _ => unimplemented!(),
                }
            }
        }

        // Implementation of gst::Object virtual methods.
        impl GstObjectImpl for CairoCompositor {}

        // Implementation of gst::Element virtual methods.
        impl ElementImpl for CairoCompositor {
            // The element specific metadata. This information is what is visible from
            // gst-inspect-1.0 and can also be programmatically retrieved from the gst::Registry
            // after initial registration without having to load the plugin in memory.
            fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
                static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
                    gst::subclass::ElementMetadata::new(
                        "Cairo Compositor",
                        "Compositor/Video",
                        "Cairo based compositor",
                        "Sebastian Dröge <sebastian@centricular.com>",
                    )
                });

                Some(&*ELEMENT_METADATA)
            }

            fn pad_templates() -> &'static [gst::PadTemplate] {
                static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
                    // Create pad templates for our sink and source pad. These are later used for
                    // actually creating the pads and beforehand already provide information to
                    // GStreamer about all possible pads that could exist for this type.

                    // On all pads we can only handle BGRx.
                    let caps = gst_video::VideoCapsBuilder::new()
                        .format(gst_video::VideoFormat::Bgrx)
                        .pixel_aspect_ratio((1, 1).into())
                        .build();

                    vec![
                        // The src pad template must be named "src" for aggregator
                        // and always be there.
                        gst::PadTemplate::new(
                            "src",
                            gst::PadDirection::Src,
                            gst::PadPresence::Always,
                            &caps,
                        )
                        .unwrap(),
                        // The sink pad template must be named "sink_%u" by default for aggregator
                        // and be requested by the application.
                        //
                        // Also declare here that it should be a pad with our custom compositor pad
                        // type that is defined further below.
                        gst::PadTemplate::with_gtype(
                            "sink_%u",
                            gst::PadDirection::Sink,
                            gst::PadPresence::Request,
                            &caps,
                            super::CairoCompositorPad::static_type(),
                        )
                        .unwrap(),
                    ]
                });

                PAD_TEMPLATES.as_ref()
            }

            // Notify via the child proxy interface whenever a new pad is added or removed.
            fn request_new_pad(
                &self,
                templ: &gst::PadTemplate,
                name: Option<&str>,
                caps: Option<&gst::Caps>,
            ) -> Option<gst::Pad> {
                let element = self.instance();
                let pad = self.parent_request_new_pad(templ, name, caps)?;
                element.child_added(&pad, &pad.name());
                Some(pad)
            }

            fn release_pad(&self, pad: &gst::Pad) {
                let element = self.instance();
                element.child_removed(pad, &pad.name());
                self.parent_release_pad(pad);
            }
        }

        // Implementation of gst_base::Aggregator virtual methods.
        impl AggregatorImpl for CairoCompositor {
            // Called whenever a query arrives at the given sink pad of the compositor.
            fn sink_query(
                &self,
                aggregator_pad: &gst_base::AggregatorPad,
                query: &mut gst::QueryRef,
            ) -> bool {
                use gst::QueryViewMut;

                // We can accept any input caps that match the pad template. By default
                // videoaggregator only allows caps that have the same format as the output.
                match query.view_mut() {
                    QueryViewMut::Caps(q) => {
                        let caps = aggregator_pad.pad_template_caps();
                        let filter = q.filter();

                        let caps = if let Some(filter) = filter {
                            filter.intersect_with_mode(&caps, gst::CapsIntersectMode::First)
                        } else {
                            caps
                        };

                        q.set_result(&caps);

                        true
                    }
                    QueryViewMut::AcceptCaps(q) => {
                        let caps = q.caps();
                        let template_caps = aggregator_pad.pad_template_caps();
                        let res = caps.is_subset(&template_caps);
                        q.set_result(res);

                        true
                    }
                    _ => self.parent_sink_query(aggregator_pad, query),
                }
            }
        }

        // Implementation of gst_video::VideoAggregator virtual methods.
        impl VideoAggregatorImpl for CairoCompositor {
            // Called by videoaggregator whenever the output format should be determined.
            fn find_best_format(
                &self,
                _downstream_caps: &gst::Caps,
            ) -> Option<(gst_video::VideoInfo, bool)> {
                // Let videoaggregator select whatever format downstream wants.
                //
                // By default videoaggregator doesn't allow a different format than the input
                // format.
                None
            }

            // Called whenever a new output frame should be produced. At this point, each pad has
            // either no frame queued up at all or the frame that should be used for this output
            // time.
            fn aggregate_frames(
                &self,
                token: &gst_video::subclass::AggregateFramesToken,
                outbuf: &mut gst::BufferRef,
            ) -> Result<gst::FlowSuccess, gst::FlowError> {
                let element = self.instance();
                let pads = element.sink_pads();

                // Map the output frame writable.
                let out_info = element.video_info().unwrap();
                let mut out_frame =
                    gst_video::VideoFrameRef::from_buffer_ref_writable(outbuf, &out_info).unwrap();

                // And then create a cairo context for drawing on the output frame.
                with_frame(&mut out_frame, |ctx| {
                    let settings = self.settings.lock().unwrap().clone();

                    // First of all, clear the background.
                    let bg = (
                        ((settings.background_color >> 16) & 0xff) as f64 / 255.0,
                        ((settings.background_color >> 8) & 0xff) as f64 / 255.0,
                        (settings.background_color & 0xff) as f64 / 255.0,
                    );
                    ctx.set_operator(cairo::Operator::Source);
                    ctx.set_source_rgb(bg.0, bg.1, bg.2);
                    ctx.paint().unwrap();

                    ctx.set_operator(cairo::Operator::Over);

                    // Then for each pad (in zorder), draw it according to the current settings.
                    for pad in pads {
                        let pad = pad.downcast_ref::<CairoCompositorPad>().unwrap();

                        let settings = pad.imp().settings.lock().unwrap().clone();

                        if settings.alpha <= 0.0 || settings.scale <= 0.0 {
                            continue;
                        }

                        let frame = match pad.prepared_frame(token) {
                            Some(frame) => frame,
                            None => continue,
                        };

                        ctx.save().unwrap();

                        ctx.translate(settings.xpos, settings.ypos);

                        ctx.scale(settings.scale, settings.scale);

                        ctx.translate(frame.width() as f64 / 2.0, frame.height() as f64 / 2.0);
                        ctx.rotate(settings.rotate / 360.0 * 2.0 * std::f64::consts::PI);
                        ctx.translate(
                            -(frame.width() as f64 / 2.0),
                            -(frame.height() as f64 / 2.0),
                        );

                        paint_frame(ctx, &frame, settings.alpha);

                        ctx.restore().unwrap();
                    }
                });

                Ok(gst::FlowSuccess::Ok)
            }
        }

        // Implementation of gst::ChildProxy virtual methods.
        //
        // This allows accessing the pads and their properties from e.g. gst-launch.
        impl ChildProxyImpl for CairoCompositor {
            fn children_count(&self) -> u32 {
                let object = self.instance();
                object.num_pads() as u32
            }

            fn child_by_name(&self, name: &str) -> Option<glib::Object> {
                let object = self.instance();
                object
                    .pads()
                    .into_iter()
                    .find(|p| p.name() == name)
                    .map(|p| p.upcast())
            }

            fn child_by_index(&self, index: u32) -> Option<glib::Object> {
                let object = self.instance();
                object
                    .pads()
                    .into_iter()
                    .nth(index as usize)
                    .map(|p| p.upcast())
            }
        }
    }

    // Creates a cairo context around the given video frame and then calls the closure to operate
    // on the cairo context. Ensures that no references to the video frame stay inside cairo.
    fn with_frame<F: FnOnce(&cairo::Context)>(
        frame: &mut gst_video::VideoFrameRef<&mut gst::BufferRef>,
        func: F,
    ) {
        // SAFETY: This is the one and only surface reference and it is dropped at the end, meaning
        // nothing from cairo is referencing the frame data anymore.
        unsafe {
            use glib::translate::*;

            let surface = cairo::ImageSurface::create_for_data_unsafe(
                frame.plane_data_mut(0).unwrap().as_mut_ptr(),
                cairo::Format::Rgb24,
                frame.width() as i32,
                frame.height() as i32,
                frame.plane_stride()[0],
            )
            .unwrap();

            let ctx = cairo::Context::new(&surface).unwrap();
            func(&ctx);
            drop(ctx);
            surface.finish();
            assert_eq!(
                cairo::ffi::cairo_surface_get_reference_count(surface.to_glib_none().0),
                1,
            );
        }
    }

    // Paints the frame with the given alpha on the cairo context at the current origin.
    // Ensures that no references to the video frame stay inside cairo.
    fn paint_frame(
        ctx: &cairo::Context,
        frame: &gst_video::VideoFrameRef<&gst::BufferRef>,
        alpha: f64,
    ) {
        // SAFETY: This is the one and only surface reference and it is dropped at the end, meaning
        // nothing from cairo is referencing the frame data anymore.
        //
        // Also nothing is ever writing to the surface from here.
        unsafe {
            use glib::translate::*;

            let surface = cairo::ImageSurface::create_for_data_unsafe(
                frame.plane_data(0).unwrap().as_ptr() as *mut u8,
                cairo::Format::Rgb24,
                frame.width() as i32,
                frame.height() as i32,
                frame.plane_stride()[0],
            )
            .unwrap();

            ctx.set_source_surface(&surface, 0.0, 0.0).unwrap();
            ctx.paint_with_alpha(alpha).unwrap();
            ctx.set_source_rgb(0.0, 0.0, 0.0);

            assert_eq!(
                cairo::ffi::cairo_surface_get_reference_count(surface.to_glib_none().0),
                1,
            );
        }
    }

    // This here defines the public interface of our element and implements
    // the corresponding traits so that it behaves like any other gst::Element.
    glib::wrapper! {
        pub struct CairoCompositor(ObjectSubclass<imp::CairoCompositor>) @extends gst_video::VideoAggregator, gst_base::Aggregator, gst::Element, gst::Object, @implements gst::ChildProxy;
    }

    impl CairoCompositor {
        // Creates a new instance of our compositor with the given name.
        pub fn new(name: Option<&str>) -> Self {
            glib::Object::new(&[("name", &name)])
        }
    }

    // In the imp submodule we include the implementation of the pad subclass.
    //
    // This doesn't implement any additional logic but only provides properties for configuring the
    // appearance of the stream corresponding to this pad and the storage of the property values.
    mod imp_pad {
        use super::*;
        use std::sync::Mutex;

        // Settings of our pad.
        #[derive(Clone)]
        pub(super) struct Settings {
            pub(super) alpha: f64,
            pub(super) scale: f64,
            pub(super) rotate: f64,
            pub(super) xpos: f64,
            pub(super) ypos: f64,
        }

        impl Default for Settings {
            fn default() -> Self {
                Self {
                    alpha: 1.0,
                    scale: 1.0,
                    rotate: 0.0,
                    xpos: 0.0,
                    ypos: 0.0,
                }
            }
        }

        // This is the private data of our pad.
        #[derive(Default)]
        pub struct CairoCompositorPad {
            pub(super) settings: Mutex<Settings>,
        }

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data.
        #[glib::object_subclass]
        impl ObjectSubclass for CairoCompositorPad {
            const NAME: &'static str = "CairoCompositorPad";
            type Type = super::CairoCompositorPad;
            type ParentType = gst_video::VideoAggregatorPad;
        }

        // Implementation of glib::Object virtual methods.
        impl ObjectImpl for CairoCompositorPad {
            // Specfication of the compositor pad properties.
            // In this case there are various properties for defining the position and otherwise
            // the appearance of the stream corresponding to this pad.
            fn properties() -> &'static [glib::ParamSpec] {
                static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                    vec![
                        glib::ParamSpecDouble::builder("alpha")
                            .nick("Alpha")
                            .blurb("Alpha value of the input")
                            .minimum(0.0)
                            .maximum(1.0)
                            .default_value(Settings::default().alpha)
                            .build(),
                        glib::ParamSpecDouble::builder("scale")
                            .nick("Scale")
                            .blurb("Scale factor of the input")
                            .minimum(0.0)
                            .maximum(f64::MAX)
                            .default_value(Settings::default().scale)
                            .build(),
                        glib::ParamSpecDouble::builder("rotate")
                            .nick("Rotate")
                            .blurb("Rotation of the input")
                            .minimum(0.0)
                            .maximum(360.0)
                            .default_value(Settings::default().rotate)
                            .build(),
                        glib::ParamSpecDouble::builder("xpos")
                            .nick("X Position")
                            .blurb("Horizontal position of the input")
                            .minimum(0.0)
                            .maximum(f64::MAX)
                            .default_value(Settings::default().xpos)
                            .build(),
                        glib::ParamSpecDouble::builder("ypos")
                            .nick("Y Position")
                            .blurb("Vertical position of the input")
                            .minimum(0.0)
                            .maximum(f64::MAX)
                            .default_value(Settings::default().ypos)
                            .build(),
                    ]
                });

                PROPERTIES.as_ref()
            }

            // Called by the application whenever the value of a property should be changed.
            fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
                let mut settings = self.settings.lock().unwrap();

                match pspec.name() {
                    "alpha" => {
                        settings.alpha = value.get().unwrap();
                    }
                    "scale" => {
                        settings.scale = value.get().unwrap();
                    }
                    "rotate" => {
                        settings.rotate = value.get().unwrap();
                    }
                    "xpos" => {
                        settings.xpos = value.get().unwrap();
                    }
                    "ypos" => {
                        settings.ypos = value.get().unwrap();
                    }
                    _ => unimplemented!(),
                };
            }

            // Called by the application whenever the value of a property should be retrieved.
            fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
                let settings = self.settings.lock().unwrap();

                match pspec.name() {
                    "alpha" => settings.alpha.to_value(),
                    "scale" => settings.scale.to_value(),
                    "rotate" => settings.rotate.to_value(),
                    "xpos" => settings.xpos.to_value(),
                    "ypos" => settings.ypos.to_value(),
                    _ => unimplemented!(),
                }
            }
        }

        // Implementation of gst::Object virtual methods.
        impl GstObjectImpl for CairoCompositorPad {}

        // Implementation of gst::Pad virtual methods.
        impl PadImpl for CairoCompositorPad {}

        // Implementation of gst_base::AggregatorPad virtual methods.
        impl AggregatorPadImpl for CairoCompositorPad {}

        // Implementation of gst_video::VideoAggregatorPad virtual methods.
        impl VideoAggregatorPadImpl for CairoCompositorPad {}
    }

    // This here defines the public interface of our element and implements
    // the corresponding traits so that it behaves like any other gst::Pad.
    glib::wrapper! {
        pub struct CairoCompositorPad(ObjectSubclass<imp_pad::CairoCompositorPad>) @extends gst_video::VideoAggregatorPad, gst_base::AggregatorPad, gst::Pad, gst::Object;
    }
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    // Create our pipeline with the compositor and two input streams.
    let pipeline = gst::Pipeline::new(None);
    let src1 = gst::ElementFactory::make("videotestsrc")
        .property_from_str("pattern", "ball")
        .build()?;
    let src2 = gst::ElementFactory::make("videotestsrc")
        .property_from_str("pattern", "smpte")
        .build()?;
    let comp = cairo_compositor::CairoCompositor::new(None);
    let conv = gst::ElementFactory::make("videoconvert").build()?;
    let sink = gst::ElementFactory::make("autovideosink").build()?;

    comp.set_property("background-color", 0xff_33_33_33u32);

    pipeline.add_many(&[&src1, &src2, comp.upcast_ref(), &conv, &sink])?;

    // Link everything together.
    src1.link_filtered(
        &comp,
        &gst::Caps::builder("video/x-raw")
            .field("width", 320i32)
            .field("height", 240i32)
            .build(),
    )
    .context("Linking source 1")?;
    src2.link_filtered(
        &comp,
        &gst::Caps::builder("video/x-raw")
            .field("width", 320i32)
            .field("height", 240i32)
            .build(),
    )
    .context("Linking source 2")?;
    comp.link_filtered(
        &conv,
        &gst::Caps::builder("video/x-raw")
            .field("width", 1280i32)
            .field("height", 720i32)
            .build(),
    )
    .context("Linking converter")?;
    conv.link(&sink).context("Linking sink")?;

    // Change positions etc of both inputs based on a timer
    let xmax = 1280.0 - 320.0f64;
    let ymax = 720.0 - 240.0f64;
    let sink_0 = comp.static_pad("sink_0").unwrap();
    sink_0.set_property("xpos", 0.0f64);
    sink_0.set_property("ypos", 0.0f64);
    let sink_1 = comp.static_pad("sink_1").unwrap();
    sink_1.set_property("xpos", xmax);
    sink_1.set_property("ypos", ymax);

    comp.set_emit_signals(true);
    comp.connect_samples_selected(move |_agg, _seg, pts, _dts, _dur, _info| {
        // Position and rotation period is 10s.
        let pos = (pts.unwrap().nseconds() % gst::ClockTime::from_seconds(10).nseconds()) as f64
            / gst::ClockTime::from_seconds(10).nseconds() as f64;

        let xpos = (1.0 + f64::sin(2.0 * std::f64::consts::PI * pos)) * xmax / 2.0;
        let ypos = (1.0 + f64::cos(2.0 * std::f64::consts::PI * pos)) * ymax / 2.0;

        sink_0.set_property("xpos", xpos);
        sink_0.set_property("ypos", ypos);

        let xpos = (1.0 + f64::cos(2.0 * std::f64::consts::PI * pos)) * xmax / 2.0;
        let ypos = (1.0 + f64::sin(2.0 * std::f64::consts::PI * pos)) * ymax / 2.0;

        sink_1.set_property("xpos", xpos);
        sink_1.set_property("ypos", ypos);

        sink_0.set_property("rotate", pos * 360.0);
        sink_1.set_property("rotate", 360.0 - pos * 360.0);

        // Alpha period is 2s.
        let pos = (pts.unwrap().nseconds() % gst::ClockTime::from_seconds(2).nseconds()) as f64
            / gst::ClockTime::from_seconds(2).nseconds() as f64;
        sink_0.set_property(
            "alpha",
            (1.0 + f64::sin(2.0 * std::f64::consts::PI * pos)) / 2.0,
        );
        sink_1.set_property(
            "alpha",
            (1.0 + f64::cos(2.0 * std::f64::consts::PI * pos)) / 2.0,
        );

        // Scale period is 20s.
        let pos = (pts.unwrap().nseconds() % gst::ClockTime::from_seconds(20).nseconds()) as f64
            / gst::ClockTime::from_seconds(20).nseconds() as f64;
        sink_0.set_property("scale", pos);
        sink_1.set_property("scale", 1.0 - pos);
    });

    Ok(pipeline)
}

// Start the pipeline and collect messages from the bus until an error or EOS.
fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");
    let mut bus_stream = bus.stream();

    let main_context = glib::MainContext::default();

    // Storage for any error so we can report it later.
    let mut error = None;
    main_context.block_on(async {
        use futures::prelude::*;

        while let Some(msg) = bus_stream.next().await {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    error = Some(anyhow::anyhow!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    ));

                    break;
                }
                _ => (),
            }
        }
    });

    // In case of error, report to the caller.
    if let Some(error) = error {
        let _ = pipeline.set_state(gst::State::Null);
        return Err(error);
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn example_main() -> Result<(), Error> {
    create_pipeline().and_then(main_loop)
}

fn main() -> Result<(), Error> {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically).
    examples_common::run(example_main)
}
