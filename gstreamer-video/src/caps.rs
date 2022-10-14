use crate::VideoFormat;
use gst::prelude::*;
use gst::Caps;
use std::ops::Bound::*;
use std::ops::RangeBounds;

pub struct VideoCapsBuilder<T> {
    builder: gst::caps::Builder<T>,
}

impl VideoCapsBuilder<gst::caps::NoFeature> {
    pub fn new() -> Self {
        let builder = Caps::builder("video/x-raw");
        let builder = VideoCapsBuilder { builder };
        builder
            .format_list(VideoFormat::iter_raw())
            .width_range(..)
            .height_range(..)
            .framerate_range(..)
    }

    pub fn any_features(self) -> VideoCapsBuilder<gst::caps::HasFeatures> {
        VideoCapsBuilder {
            builder: self.builder.any_features(),
        }
    }

    pub fn features(self, features: &[&str]) -> VideoCapsBuilder<gst::caps::HasFeatures> {
        VideoCapsBuilder {
            builder: self.builder.features(features),
        }
    }
}

impl Default for VideoCapsBuilder<gst::caps::NoFeature> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> VideoCapsBuilder<T> {
    pub fn format(self, format: VideoFormat) -> Self {
        Self {
            builder: self.builder.field("format", format.to_str()),
        }
    }

    pub fn format_list(self, formats: impl IntoIterator<Item = VideoFormat>) -> Self {
        Self {
            builder: self.builder.field(
                "format",
                gst::List::new(formats.into_iter().map(|f| f.to_str())),
            ),
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            builder: self.builder.field("width", width),
        }
    }

    pub fn width_range(self, widths: impl RangeBounds<i32>) -> Self {
        let (start, end) = range_bounds_i32_start_end(widths);
        let gst_widths: gst::IntRange<i32> = gst::IntRange::new(start, end);
        Self {
            builder: self.builder.field("width", gst_widths),
        }
    }

    pub fn width_list(self, widths: impl IntoIterator<Item = i32>) -> Self {
        Self {
            builder: self.builder.field("width", gst::List::new(widths)),
        }
    }

    pub fn height(self, height: i32) -> Self {
        Self {
            builder: self.builder.field("height", height),
        }
    }

    pub fn height_range(self, heights: impl RangeBounds<i32>) -> Self {
        let (start, end) = range_bounds_i32_start_end(heights);
        let gst_heights: gst::IntRange<i32> = gst::IntRange::new(start, end);
        Self {
            builder: self.builder.field("height", gst_heights),
        }
    }

    pub fn height_list(self, heights: impl IntoIterator<Item = i32>) -> Self {
        Self {
            builder: self.builder.field("height", gst::List::new(heights)),
        }
    }

    pub fn framerate(self, framerate: gst::Fraction) -> Self {
        Self {
            builder: self.builder.field("framerate", framerate),
        }
    }

    pub fn framerate_range(self, framerates: impl RangeBounds<gst::Fraction>) -> Self {
        let start = match framerates.start_bound() {
            Unbounded => gst::Fraction::new(0, 1),
            Excluded(n) => next_fraction(*n),
            Included(n) => {
                assert!(n.numer() >= 0);
                *n
            }
        };
        let end = match framerates.end_bound() {
            Unbounded => gst::Fraction::new(i32::MAX, 1),
            Excluded(n) => previous_fraction(*n),
            Included(n) => {
                assert!(n.numer() >= 0);
                *n
            }
        };
        assert!(start <= end);
        let framerates: gst::FractionRange = gst::FractionRange::new(start, end);
        Self {
            builder: self.builder.field("framerate", framerates),
        }
    }

    pub fn framerate_list(self, framerates: impl IntoIterator<Item = gst::Fraction>) -> Self {
        Self {
            builder: self.builder.field("framerate", gst::List::new(framerates)),
        }
    }

    pub fn pixel_aspect_ratio(self, pixel_aspect_ratio: gst::Fraction) -> Self {
        Self {
            builder: self.builder.field("pixel-aspect-ratio", pixel_aspect_ratio),
        }
    }

    pub fn pixel_aspect_ratio_range(
        self,
        pixel_aspect_ratios: impl RangeBounds<gst::Fraction>,
    ) -> Self {
        let start = match pixel_aspect_ratios.start_bound() {
            Unbounded => gst::Fraction::new(1, i32::MAX),
            Excluded(n) => next_fraction(*n),
            Included(n) => {
                assert!(n.numer() >= 0);
                *n
            }
        };
        let end = match pixel_aspect_ratios.end_bound() {
            Unbounded => gst::Fraction::new(i32::MAX, 1),
            Excluded(n) => previous_fraction(*n),
            Included(n) => {
                assert!(n.numer() >= 0);
                *n
            }
        };
        assert!(start <= end);
        let pixel_aspect_ratios: gst::FractionRange = gst::FractionRange::new(start, end);
        Self {
            builder: self
                .builder
                .field("pixel-aspect-ratio", pixel_aspect_ratios),
        }
    }

    pub fn pixel_aspect_ratio_list(
        self,
        pixel_aspect_ratios: impl IntoIterator<Item = gst::Fraction>,
    ) -> Self {
        Self {
            builder: self
                .builder
                .field("pixel-aspect-ratio", gst::List::new(pixel_aspect_ratios)),
        }
    }

    pub fn field<V: ToSendValue + Sync>(self, name: &str, value: V) -> Self {
        Self {
            builder: self.builder.field(name, value),
        }
    }

    #[must_use]
    pub fn build(self) -> gst::Caps {
        self.builder.build()
    }
}

fn range_bounds_i32_start_end(range: impl RangeBounds<i32>) -> (i32, i32) {
    skip_assert_initialized!();
    let start = match range.start_bound() {
        Unbounded => 1,
        Excluded(n) => n + 1,
        Included(n) => *n,
    };
    let end = match range.end_bound() {
        Unbounded => i32::MAX,
        Excluded(n) => n - 1,
        Included(n) => *n,
    };
    (start, end)
}

// https://math.stackexchange.com/questions/39582/how-to-compute-next-previous-representable-rational-number/3798608#3798608

/* Extended Euclidean Algorithm: computes (g, x, y),
 * such that a*x + b*y = g = gcd(a, b) >= 0. */
fn xgcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    skip_assert_initialized!();
    let mut x0 = 0i64;
    let mut x1 = 1i64;
    let mut y0 = 1i64;
    let mut y1 = 0i64;
    while a != 0 {
        let q;
        (q, a, b) = (b / a, b % a, a);
        (y0, y1) = (y1, y0 - q * y1);
        (x0, x1) = (x1, x0 - q * x1);
    }
    if b >= 0 {
        (b, x0, y0)
    } else {
        (-b, -x0, -y0)
    }
}

/* Computes the neighbours of p/q in the Farey sequence of order n. */
fn farey_neighbours(p: i32, q: i32) -> (i32, i32, i32, i32) {
    skip_assert_initialized!();
    let n = i32::MAX as i64;
    assert!(q != 0);
    let mut p = p as i64;
    let mut q = q as i64;
    if q < 0 {
        p = -p;
        q = -q;
    }
    let (g, r, _) = xgcd(p, q);
    p /= g;
    q /= g;
    let b = ((n - r) / q) * q + r;
    let a = (b * p - 1) / q;
    let d = ((n + r) / q) * q - r;
    let c = (d * p + 1) / q;
    (a as i32, b as i32, c as i32, d as i32)
}

fn previous_fraction(fraction: gst::Fraction) -> gst::Fraction {
    skip_assert_initialized!();
    let num = fraction.numer();
    let den = fraction.denom();
    let (new_num, new_den);
    if num < den {
        (new_num, new_den, _, _) = farey_neighbours(num, den);
    } else {
        (_, _, new_den, new_num) = farey_neighbours(den, num);
    }
    gst::Fraction::new(new_num, new_den)
}

fn next_fraction(fraction: gst::Fraction) -> gst::Fraction {
    skip_assert_initialized!();
    let num = fraction.numer();
    let den = fraction.denom();
    let (new_num, new_den);
    if num < den {
        (_, _, new_num, new_den) = farey_neighbours(num, den);
    } else {
        (new_den, new_num, _, _) = farey_neighbours(den, num);
    }
    gst::Fraction::new(new_num, new_den)
}

#[test]
fn test_0_1_fraction() {
    gst::init().unwrap();
    let zero_over_one = gst::Fraction::new(0, 1);
    let prev = previous_fraction(zero_over_one);
    assert_eq!(prev.numer(), -1);
    assert_eq!(prev.denom(), i32::MAX);
    let next = next_fraction(zero_over_one);
    assert_eq!(next.numer(), 1);
    assert_eq!(next.denom(), i32::MAX);
}

#[test]
fn test_25_1() {
    gst::init().unwrap();
    let twentyfive = gst::Fraction::new(25, 1);
    let next = next_fraction(twentyfive);
    //25.000000011641532
    assert_eq!(next.numer(), 2147483626);
    assert_eq!(next.denom(), 85899345);
    let prev = previous_fraction(twentyfive);
    //24.999999988358468
    assert_eq!(prev.numer(), 2147483624);
    assert_eq!(prev.denom(), 85899345);
}
#[test]
fn test_1_25() {
    gst::init().unwrap();
    let twentyfive = gst::Fraction::new(1, 25);
    let next = next_fraction(twentyfive);
    //0.040000000018626
    assert_eq!(next.numer(), 85899345);
    assert_eq!(next.denom(), 2147483624);
    let prev = previous_fraction(twentyfive);
    //0.039999999981374
    assert_eq!(prev.numer(), 85899345);
    assert_eq!(prev.denom(), 2147483626);
}
