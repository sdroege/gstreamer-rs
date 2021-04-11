// This example demonstrates subclassing, implementing an audio filter and providing some Rust API
// on it. It operates the following pipeline:
//
// {audiotestsrc} - {our filter} - {audioconvert} - {autoaudiosink}
//
// Our filter can only handle F32 mono and acts as a FIR filter. The filter impulse response /
// coefficients are provided via Rust API on the filter as a Vec<f32>.

use gst::prelude::*;
use gst::{element_error, gst_info, gst_trace};

use anyhow::Error;
use derive_more::{Display, Error};

#[path = "../examples-common.rs"]
mod examples_common;

// Our custom FIR filter element is defined in this module
mod fir_filter {
    use super::*;

    use glib::subclass::prelude::*;
    use gst::subclass::prelude::*;
    use gst_base::subclass::prelude::*;

    use byte_slice_cast::*;

    use once_cell::sync::Lazy;

    // The debug category we use below for our filter
    pub static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
        gst::DebugCategory::new(
            "rsfirfilter",
            gst::DebugColorFlags::empty(),
            Some("Rust FIR Filter"),
        )
    });

    // In the imp submodule we include the actual implementation
    mod imp {
        use super::*;
        use std::collections::VecDeque;
        use std::i32;
        use std::sync::Mutex;

        // This is the private data of our filter
        #[derive(Default)]
        pub struct FirFilter {
            pub(super) coeffs: Mutex<Vec<f32>>,
            history: Mutex<VecDeque<f32>>,
        }

        // This trait registers our type with the GObject object system and
        // provides the entry points for creating a new instance and setting
        // up the class data
        #[glib::object_subclass]
        impl ObjectSubclass for FirFilter {
            const NAME: &'static str = "RsFirFilter";
            type Type = super::FirFilter;
            type ParentType = gst_base::BaseTransform;
        }

        // Implementation of glib::Object virtual methods
        impl ObjectImpl for FirFilter {}

        // Implementation of gst::Element virtual methods
        impl ElementImpl for FirFilter {
            // The element specific metadata. This information is what is visible from
            // gst-inspect-1.0 and can also be programatically retrieved from the gst::Registry
            // after initial registration without having to load the plugin in memory.
            fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
                static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
                    gst::subclass::ElementMetadata::new(
                        "FIR Filter",
                        "Filter/Effect/Audio",
                        "A FIR audio filter",
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

                    // On both of pads we can only handle F32 mono at any sample rate.
                    let caps = gst::Caps::new_simple(
                        "audio/x-raw",
                        &[
                            ("format", &gst_audio::AUDIO_FORMAT_F32.to_str()),
                            ("rate", &gst::IntRange::<i32>::new(1, i32::MAX)),
                            ("channels", &1i32),
                            ("layout", &"interleaved"),
                        ],
                    );

                    vec![
                        // The src pad template must be named "src" for basetransform
                        // and specific a pad that is always there
                        gst::PadTemplate::new(
                            "src",
                            gst::PadDirection::Src,
                            gst::PadPresence::Always,
                            &caps,
                        )
                        .unwrap(),
                        // The sink pad template must be named "sink" for basetransform
                        // and specific a pad that is always there
                        gst::PadTemplate::new(
                            "sink",
                            gst::PadDirection::Sink,
                            gst::PadPresence::Always,
                            &caps,
                        )
                        .unwrap(),
                    ]
                });

                PAD_TEMPLATES.as_ref()
            }
        }

        // Implementation of gst_base::BaseTransform virtual methods
        impl BaseTransformImpl for FirFilter {
            // Configure basetransform so that we are always running in-place,
            // don't passthrough on same caps and also never call transform_ip
            // in passthrough mode (which does not matter for us here).
            //
            // The way how our processing is implemented, in-place transformation
            // is simpler.
            const MODE: gst_base::subclass::BaseTransformMode =
                gst_base::subclass::BaseTransformMode::AlwaysInPlace;
            const PASSTHROUGH_ON_SAME_CAPS: bool = false;
            const TRANSFORM_IP_ON_PASSTHROUGH: bool = false;

            // Returns the size of one processing unit (i.e. a frame in our case) corresponding
            // to the given caps. This is used for allocating a big enough output buffer and
            // sanity checking the input buffer size, among other things.
            fn get_unit_size(&self, _element: &Self::Type, caps: &gst::Caps) -> Option<usize> {
                let audio_info = gst_audio::AudioInfo::from_caps(caps).ok();
                audio_info.map(|info| info.bpf() as usize)
            }

            // Called when shutting down the element so we can release all stream-related state
            // There's also start(), which is called whenever starting the element again
            fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
                // Drop state
                self.history.lock().unwrap().clear();

                gst_info!(CAT, obj: element, "Stopped");

                Ok(())
            }

            // Does the actual transformation of the input buffer to the output buffer
            fn transform_ip(
                &self,
                element: &Self::Type,
                buf: &mut gst::BufferRef,
            ) -> Result<gst::FlowSuccess, gst::FlowError> {
                // Get coefficients and return directly if we have none
                let coeffs = self.coeffs.lock().unwrap();
                if coeffs.is_empty() {
                    gst_trace!(CAT, obj: element, "No coefficients set -- passthrough");
                    return Ok(gst::FlowSuccess::Ok);
                }

                // Try mapping the input buffer as writable
                let mut data = buf.map_writable().map_err(|_| {
                    element_error!(
                        element,
                        gst::CoreError::Failed,
                        ["Failed to map input buffer readable"]
                    );
                    gst::FlowError::Error
                })?;

                // And reinterprete it as a slice of f32
                let samples = data.as_mut_slice_of::<f32>().map_err(|err| {
                    element_error!(
                        element,
                        gst::CoreError::Failed,
                        ["Failed to cast input buffer as f32 slice: {}", err]
                    );
                    gst::FlowError::Error
                })?;

                let mut history = self.history.lock().unwrap();

                gst_trace!(
                    CAT,
                    obj: element,
                    "Transforming {} samples with filter of length {}",
                    samples.len(),
                    coeffs.len()
                );

                // Now calculate the output for each sample by doing convolution
                for sample in samples.iter_mut() {
                    history.push_front(*sample);
                    history.truncate(coeffs.len());

                    let val = history
                        .iter()
                        .copied()
                        .chain(std::iter::repeat(0.0))
                        .zip(coeffs.iter())
                        .map(|(x, a)| x * a)
                        .sum();
                    *sample = val;
                }

                Ok(gst::FlowSuccess::Ok)
            }
        }
    }

    // This here defines the public interface of our element and implements
    // the corresponding traits so that it behaves like any other gst::Element
    glib::wrapper! {
        pub struct FirFilter(ObjectSubclass<imp::FirFilter>) @extends gst_base::BaseTransform, gst::Element, gst::Object;
    }

    // GStreamer elements must be Send+Sync, and ours is
    unsafe impl Send for FirFilter {}
    unsafe impl Sync for FirFilter {}

    impl FirFilter {
        // Creates a new instance of our filter with the given name
        pub fn new(name: Option<&str>) -> FirFilter {
            glib::Object::new(&[("name", &name)]).expect("Failed to create fir filter")
        }

        // Sets the coefficients by getting access to the private
        // struct and simply setting them
        pub fn set_coeffs(&self, coeffs: Vec<f32>) {
            let imp = imp::FirFilter::from_instance(self);
            *imp.coeffs.lock().unwrap() = coeffs;
        }
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    // Create our pipeline with the custom element
    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("audiotestsrc", None)
        .map_err(|_| MissingElement("audiotestsrc"))?;
    let filter = fir_filter::FirFilter::new(None);
    let conv = gst::ElementFactory::make("audioconvert", None)
        .map_err(|_| MissingElement("audioconvert"))?;
    let sink = gst::ElementFactory::make("autoaudiosink", None)
        .map_err(|_| MissingElement("autoaudiosink"))?;

    pipeline.add_many(&[&src, filter.upcast_ref(), &conv, &sink])?;
    src.link(&filter)?;
    filter.link(&conv)?;
    conv.link(&sink)?;

    src.set_property_from_str("wave", "white-noise");

    // Create a windowed sinc lowpass filter at 1/64 sample rate,
    // i.e. 689Hz for 44.1kHz sample rate
    let w = 2.0 * std::f32::consts::PI / 64.0;
    let len = 9;
    let mut kernel = (0..len)
        .map(|i| {
            let v = if i == (len - 1) / 2 {
                w
            } else {
                let p = i as f32 - (len - 1) as f32 / 2.0;
                (w * p).sin() / p
            };

            // Hamming window
            let win =
                0.54 - 0.46 * (2.0 * std::f32::consts::PI * i as f32 / (len as f32 - 1.0)).cos();

            v * win
        })
        .collect::<Vec<f32>>();

    // Normalize
    let sum: f32 = kernel.iter().sum();
    for v in kernel.iter_mut() {
        *v /= sum;
    }

    filter.set_coeffs(kernel);

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| String::from(s.path_string()))
                        .unwrap_or_else(|| String::from("None")),
                    error: err.error().to_string(),
                    debug: err.debug(),
                    source: err.error(),
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
        Err(e) => eprintln!("Error! {}", e),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
