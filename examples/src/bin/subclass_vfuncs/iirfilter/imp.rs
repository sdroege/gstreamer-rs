// In the imp submodule we include the actual implementation

use std::{collections::VecDeque, sync::Mutex};

use glib::prelude::*;
use gst_audio::subclass::prelude::*;
use once_cell::sync::Lazy;

use byte_slice_cast::*;

use atomic_refcell::AtomicRefCell;

// The debug category we use below for our filter
pub static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "rsiirfilter",
        gst::DebugColorFlags::empty(),
        Some("Rust IIR Filter"),
    )
});

#[derive(Default)]
// This is the state of our filter
struct State {
    a: Vec<f64>,
    b: Vec<f64>,
    x: VecDeque<f64>,
    y: VecDeque<f64>,
}

// This is the private data of our filter
#[derive(Default)]
pub struct IirFilter {
    coeffs: Mutex<Option<(Vec<f64>, Vec<f64>)>>,
    state: AtomicRefCell<State>,
}

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
#[glib::object_subclass]
impl ObjectSubclass for IirFilter {
    const NAME: &'static str = "RsIirFilter";
    const ABSTRACT: bool = true;
    type Type = super::IirFilter;
    type ParentType = gst_audio::AudioFilter;
    type Class = super::Class;

    // Here we set default implementations for all the virtual methods.
    // This is mandatory for all virtual methods that are not `Option`s.
    fn class_init(class: &mut Self::Class) {
        class.set_rate = |obj, rate| obj.imp().set_rate_default(rate);
    }
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for IirFilter {}

impl GstObjectImpl for IirFilter {}

// Implementation of gst::Element virtual methods
impl ElementImpl for IirFilter {}

// Implementation of gst_base::BaseTransform virtual methods
impl BaseTransformImpl for IirFilter {
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

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_start()?;

        *self.state.borrow_mut() = State::default();

        Ok(())
    }

    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_stop()?;

        *self.state.borrow_mut() = State::default();

        Ok(())
    }

    fn transform_ip(&self, buf: &mut gst::BufferRef) -> Result<gst::FlowSuccess, gst::FlowError> {
        let mut state = self.state.borrow_mut();

        // Update coefficients if new coefficients were set
        {
            let mut coeffs = self.coeffs.lock().unwrap();

            if let Some((a, b)) = coeffs.take() {
                state.x.clear();
                state.y.clear();
                if !a.is_empty() {
                    state.y.resize(a.len() - 1, 0.0);
                }
                if !b.is_empty() {
                    state.x.resize(b.len() - 1, 0.0);
                }
                state.a = a;
                state.b = b;
            }
        }

        if state.a.is_empty() | state.b.is_empty() {
            return Ok(gst::FlowSuccess::Ok);
        }

        let mut map = buf.map_writable().map_err(|_| {
            gst::error!(CAT, imp: self, "Failed to map buffer writable");
            gst::FlowError::Error
        })?;

        let samples = map.as_mut_slice_of::<f32>().unwrap();

        assert!(state.b.len() - 1 == state.x.len());
        assert!(state.a.len() - 1 == state.y.len());

        for sample in samples.iter_mut() {
            let mut val = state.b[0] * *sample as f64;

            for (b, x) in Iterator::zip(state.b.iter().skip(1), state.x.iter()) {
                val += b * x;
            }

            for (a, y) in Iterator::zip(state.a.iter().skip(1), state.y.iter()) {
                val -= a * y;
            }

            val /= state.a[0];

            let _ = state.x.pop_back().unwrap();
            state.x.push_front(*sample as f64);

            let _ = state.y.pop_back().unwrap();
            state.y.push_front(val);

            *sample = val as f32;
        }

        Ok(gst::FlowSuccess::Ok)
    }
}

impl AudioFilterImpl for IirFilter {
    fn allowed_caps() -> &'static gst::Caps {
        static CAPS: std::sync::OnceLock<gst::Caps> = std::sync::OnceLock::new();
        CAPS.get_or_init(|| {
            // On both of pads we can only handle F32 mono at any sample rate.
            gst_audio::AudioCapsBuilder::new_interleaved()
                .format(gst_audio::AUDIO_FORMAT_F32)
                .channels(1)
                .build()
        })
    }

    fn setup(&self, info: &gst_audio::AudioInfo) -> Result<(), gst::LoggableError> {
        self.parent_setup(info)?;

        gst::debug!(CAT, imp: self, "Rate changed to {}", info.rate());
        let obj = self.obj();
        (obj.class().as_ref().set_rate)(&obj, info.rate());

        Ok(())
    }
}

/// Wrappers for public methods and associated helper functions.
impl IirFilter {
    pub(super) fn set_coeffs(&self, a: Vec<f64>, b: Vec<f64>) {
        gst::debug!(CAT, imp: self, "Setting coefficients a: {a:?}, b: {b:?}");
        *self.coeffs.lock().unwrap() = Some((a, b));
    }
}

/// Default virtual method implementations.
impl IirFilter {
    fn set_rate_default(&self, _rate: u32) {}
}
