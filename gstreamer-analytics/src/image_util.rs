// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

#[cfg(feature = "v1_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
#[doc(alias = "gst_analytics_image_util_iou_float")]
pub fn iou_f32(bb1: Rect<f32>, bb2: Rect<f32>) -> f32 {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_analytics_image_util_iou_float(
            bb1.x, bb1.y, bb1.w, bb1.h, bb2.x, bb2.y, bb2.w, bb2.h,
        )
    }
}

#[cfg(feature = "v1_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
#[doc(alias = "gst_analytics_image_util_iou_int")]
pub fn iou_i32(bb1: Rect<i32>, bb2: Rect<i32>) -> f32 {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_analytics_image_util_iou_int(
            bb1.x, bb1.y, bb1.w, bb1.h, bb2.x, bb2.y, bb2.w, bb2.h,
        )
    }
}

#[cfg(test)]
#[cfg(feature = "v1_28")]
mod tests {
    use super::*;

    #[test]
    fn iou_float_no_overlap() {
        gst::init().unwrap();
        // Two non-overlapping boxes
        let iou = iou_f32(
            Rect::<f32> {
                x: 0.0,
                y: 0.0,
                w: 10.0,
                h: 10.0,
            },
            Rect::<f32> {
                x: 20.0,
                y: 20.0,
                w: 10.0,
                h: 10.0,
            },
        );
        assert_eq!(iou, 0.0);
    }

    #[test]
    fn iou_float_identical_boxes() {
        gst::init().unwrap();
        let iou = iou_f32(
            Rect::<f32> {
                x: 0.0,
                y: 0.0,
                w: 10.0,
                h: 10.0,
            },
            Rect::<f32> {
                x: 0.0,
                y: 0.0,
                w: 10.0,
                h: 10.0,
            },
        );
        assert!((iou - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn iou_int_no_overlap() {
        gst::init().unwrap();
        let iou = iou_i32(
            Rect::<i32> {
                x: 0,
                y: 0,
                w: 10,
                h: 10,
            },
            Rect::<i32> {
                x: 20,
                y: 20,
                w: 10,
                h: 10,
            },
        );
        assert_eq!(iou, 0.0);
    }

    #[test]
    fn iou_int_identical_boxes() {
        gst::init().unwrap();
        let iou = iou_i32(
            Rect::<i32> {
                x: 0,
                y: 0,
                w: 10,
                h: 10,
            },
            Rect::<i32> {
                x: 0,
                y: 0,
                w: 10,
                h: 10,
            },
        );
        assert!((iou - 1.0).abs() < f32::EPSILON);
    }
}
