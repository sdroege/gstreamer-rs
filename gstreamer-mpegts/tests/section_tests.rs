#[cfg(feature = "v1_20")]
use gstreamer_mpegts::Section;

#[cfg(feature = "v1_20")]
#[test]
fn test_section_new() {
    gstreamer_mpegts::gst::init().expect("Failed to initialize GStreamer");
    gstreamer_mpegts::init();

    // Create a minimal section with valid structure
    // This is a simple test to ensure the function doesn't crash
    // Actual section validation is done by the C library
    let data = &[
        0x00, 0xB0, 0x11, 0x00, 0x00, 0xc1, 0x00, 0x00, 0x00, 0x00, 0xe0, 0x30, 0x00, 0x01, 0xe0,
        0x31, 0x98, 0xdf, 0x37, 0xc4,
    ];

    // The C library validates the section, may return NULL for invalid sections
    // This test just ensures the function works without crashing
    assert!(Section::new(0, data).is_some());

    // Test bogus data and make sure it fails
    let fail = &[0x42, 0xff, 0xff];
    assert!(Section::new(0, fail).is_none());
}

#[cfg(feature = "v1_20")]
#[test]
fn test_event_new_mpegts_section() {
    gstreamer_mpegts::gst::init().expect("Failed to initialize GStreamer");
    gstreamer_mpegts::init();

    let data = &[
        0x00, 0xB0, 0x11, 0x00, 0x00, 0xc1, 0x00, 0x00, 0x00, 0x00, 0xe0, 0x30, 0x00, 0x01, 0xe0,
        0x31, 0x98, 0xdf, 0x37, 0xc4,
    ];

    let section = Section::new(0, data);
    if let Some(section) = section {
        let event = gstreamer_mpegts::event_new_mpegts_section(&section);
        // Now try to convert back to section
        assert!(gstreamer_mpegts::event_parse_mpegts_section(&event).is_some());
    }
}
