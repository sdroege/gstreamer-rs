// Take a look at the license at the top of the repository in the LICENSE file.

use gstreamer_controller::prelude::*;

fn init() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        gst::init().unwrap();
    });
}

#[test]
fn test_interpolation_control_source_list_control_points() {
    init();

    let control_source = gstreamer_controller::InterpolationControlSource::new();

    // Initially should have no control points
    assert_eq!(control_source.count(), 0);
    assert_eq!(control_source.list_control_points().len(), 0);

    // Add some control points
    assert!(control_source.set(gst::ClockTime::ZERO, 0.0));
    assert!(control_source.set(gst::ClockTime::from_seconds(1), 0.5));
    assert!(control_source.set(gst::ClockTime::from_seconds(2), 1.0));

    // Verify count
    assert_eq!(control_source.count(), 3);

    // Get all control points
    let points = control_source.list_control_points();
    assert_eq!(points.len(), 3);

    // Verify the control points have correct values
    assert_eq!(points[0].timestamp(), gst::ClockTime::ZERO);
    assert_eq!(points[0].value(), 0.0);

    assert_eq!(points[1].timestamp(), gst::ClockTime::from_seconds(1));
    assert_eq!(points[1].value(), 0.5);

    assert_eq!(points[2].timestamp(), gst::ClockTime::from_seconds(2));
    assert_eq!(points[2].value(), 1.0);
}
