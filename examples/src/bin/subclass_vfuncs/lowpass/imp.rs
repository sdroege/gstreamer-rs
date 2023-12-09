// In the imp submodule we include the actual implementation

use std::sync::Mutex;

use glib::{once_cell::sync::Lazy, prelude::*};
use gst::prelude::*;
use gst_audio::subclass::prelude::*;

use crate::iirfilter::{IirFilterExt, IirFilterImpl};

// These are the property values of our filter
pub struct Settings {
    cutoff: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { cutoff: 0.0 }
    }
}

// This is the state of our filter
#[derive(Default)]
pub struct State {
    rate: Option<u32>,
}

// This is the private data of our filter
#[derive(Default)]
pub struct Lowpass {
    settings: Mutex<Settings>,
    state: Mutex<State>,
}

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
#[glib::object_subclass]
impl ObjectSubclass for Lowpass {
    const NAME: &'static str = "RsLowpass";
    type Type = super::Lowpass;
    type ParentType = crate::iirfilter::IirFilter;
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for Lowpass {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpecFloat::builder("cutoff")
                .nick("Cutoff")
                .blurb("Cutoff frequency in Hz")
                .default_value(Settings::default().cutoff)
                .minimum(0.0)
                .mutable_playing()
                .build()]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "cutoff" => {
                self.settings.lock().unwrap().cutoff = value.get().unwrap();
                self.calculate_coeffs();
            }
            _ => unimplemented!(),
        };
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "cutoff" => self.settings.lock().unwrap().cutoff.to_value(),
            _ => unimplemented!(),
        }
    }
}

impl GstObjectImpl for Lowpass {}

// Implementation of gst::Element virtual methods
impl ElementImpl for Lowpass {
    // The element specific metadata. This information is what is visible from
    // gst-inspect-1.0 and can also be programmatically retrieved from the gst::Registry
    // after initial registration without having to load the plugin in memory.
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Lowpass Filter",
                "Filter/Effect/Audio",
                "A Lowpass audio filter",
                "Sebastian Dröge <sebastian@centricular.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }
}

// Implementation of gst_base::BaseTransform virtual methods
impl BaseTransformImpl for Lowpass {
    const MODE: gst_base::subclass::BaseTransformMode =
        <<crate::iirfilter::IirFilter as glib::object::ObjectSubclassIs>::Subclass>::MODE;
    const PASSTHROUGH_ON_SAME_CAPS: bool =
        <<crate::iirfilter::IirFilter as glib::object::ObjectSubclassIs>::Subclass>::PASSTHROUGH_ON_SAME_CAPS;
    const TRANSFORM_IP_ON_PASSTHROUGH: bool =
        <<crate::iirfilter::IirFilter as glib::object::ObjectSubclassIs>::Subclass>::TRANSFORM_IP_ON_PASSTHROUGH;

    fn start(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_start()?;

        *self.state.lock().unwrap() = State::default();

        Ok(())
    }
}

// Implement of gst_audio::AudioFilter virtual methods
impl AudioFilterImpl for Lowpass {}

// Implement of IirFilter virtual methods
impl IirFilterImpl for Lowpass {
    fn set_rate(&self, rate: u32) {
        // Could call
        //   self.parent_set_rate(rate);
        // here but chaining up is not necessary if the base class doesn't require that
        // or if the behaviour of the parent class should be completely overridden.

        self.state.lock().unwrap().rate = Some(rate);
        self.calculate_coeffs();
    }
}

impl Lowpass {
    fn calculate_coeffs(&self) {
        use std::f64;

        let Some(rate) = self.state.lock().unwrap().rate else {
            return;
        };
        let cutoff = self.settings.lock().unwrap().cutoff;

        // See Audio EQ Cookbook
        // https://www.w3.org/TR/audio-eq-cookbook
        let cutoff = cutoff as f64 / rate as f64;

        let omega = 2.0 * f64::consts::PI * cutoff;
        let q = 1.0;

        let alpha = f64::sin(omega) / (2.0 * q);

        let mut b = vec![
            (1.0 - f64::cos(omega)) / 2.0,
            1.0 - f64::cos(omega),
            (1.0 - f64::cos(omega) / 2.0),
        ];

        let mut a = vec![1.0 + alpha, -2.0 * f64::cos(omega), 1.0 - alpha];

        let a0 = a[0];
        for a in &mut a {
            *a /= a0;
        }
        for b in &mut b {
            *b /= a0;
        }

        self.obj().set_coeffs(a, b);
    }
}
