// This example demonstrates how to draw an overlay on a video stream using
// Direct2D/DirectWrite/WIC and the overlay composition element.

// {videotestsrc} - {overlaycomposition} - {capsfilter} - {videoconvert} - {autovideosink}
// The capsfilter element allows us to dictate the video resolution we want for the
// videotestsrc and the overlaycomposition element.

use std::sync::{Arc, Mutex};

use byte_slice_cast::*;

use anyhow::Error;
use derive_more::{Display, Error};
use gst::prelude::*;
use windows::{
    Foundation::Numerics::*,
    Win32::{
        Graphics::{
            Direct2D::{Common::*, *},
            DirectWrite::*,
            Dxgi::Common::*,
            Imaging::*,
        },
        System::Com::*,
    },
};

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

struct DrawingContext {
    // Factory for creating render target
    d2d_factory: ID2D1Factory,

    // Used to create WIC bitmap surface
    wic_factory: IWICImagingFactory,

    // text layout holding text information (string, font, size, etc)
    text_layout: IDWriteTextLayout,

    // Holding rendred image
    bitmap: Option<IWICBitmap>,

    // Bound to bitmap and used to actual Direct2D rendering
    render_target: Option<ID2D1RenderTarget>,

    info: Option<gst_video::VideoInfo>,
}

// Required for IWICBitmap
unsafe impl Send for DrawingContext {}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::default();

    // The videotestsrc supports multiple test patterns. In this example, we will use the
    // pattern with a white ball moving around the video's center point.
    let src = gst::ElementFactory::make("videotestsrc")
        .property_from_str("pattern", "ball")
        .build()?;

    let overlay = gst::ElementFactory::make("overlaycomposition").build()?;

    let caps = gst_video::VideoCapsBuilder::new()
        .width(800)
        .height(800)
        .framerate((30, 1).into())
        .build();
    let capsfilter = gst::ElementFactory::make("capsfilter")
        .property("caps", &caps)
        .build()?;

    let videoconvert = gst::ElementFactory::make("videoconvert").build()?;
    let sink = gst::ElementFactory::make("autovideosink").build()?;

    pipeline.add_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;
    gst::Element::link_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;

    // Most Direct2D/DirectWrite APIs (including factory methods) are marked as
    // "unsafe", but they shouldn't fail in practice
    let drawer = unsafe {
        let d2d_factory =
            D2D1CreateFactory::<ID2D1Factory>(D2D1_FACTORY_TYPE_MULTI_THREADED, None).unwrap();
        let dwrite_factory =
            DWriteCreateFactory::<IDWriteFactory>(DWRITE_FACTORY_TYPE_SHARED).unwrap();
        let text_format = dwrite_factory
            .CreateTextFormat(
                windows::w!("Arial"),
                None,
                DWRITE_FONT_WEIGHT_BOLD,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                32f32,
                windows::w!("en-us"),
            )
            .unwrap();
        let text_layout = dwrite_factory
            .CreateTextLayout(
                windows::w!("GStreamer").as_wide(),
                &text_format,
                // Size will be updated later on "caps-changed" signal
                800f32,
                800f32,
            )
            .unwrap();

        // Top (default) and center alignment
        text_layout
            .SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)
            .unwrap();

        let wic_factory: IWICImagingFactory =
            CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_ALL).unwrap();

        Arc::new(Mutex::new(DrawingContext {
            d2d_factory,
            wic_factory,
            text_layout,
            bitmap: None,
            render_target: None,
            info: None,
        }))
    };

    overlay.connect_closure(
        "draw",
        false,
        glib::closure!(@strong drawer => move |_overlay: &gst::Element,
                                               sample: &gst::Sample| {
            use std::f64::consts::PI;

            let drawer = drawer.lock().unwrap();

            let buffer = sample.buffer().unwrap();
            let timestamp = buffer.pts().unwrap();

            let info = drawer.info.as_ref().unwrap();
            let text_layout = &drawer.text_layout;
            let bitmap = drawer.bitmap.as_ref().unwrap();
            let render_target = drawer.render_target.as_ref().unwrap();

            let global_angle = 360. * (timestamp % (10 * gst::ClockTime::SECOND)).nseconds() as f64
                / (10.0 * gst::ClockTime::SECOND.nseconds() as f64);
            let center_x = (info.width() / 2) as f32;
            let center_y = (info.height() / 2) as f32;
            let top_margin = (info.height() / 20) as f32;

            unsafe {
                // Begin drawing
                render_target.BeginDraw();

                // Clear background
                render_target.Clear(Some(&D2D1_COLOR_F {
                    r: 0f32,
                    g: 0f32,
                    b: 0f32,
                    a: 0f32,
                }));

                // This loop will render 10 times the string "GStreamer" in a circle
                for i in 0..10 {
                    let angle = (360. * f64::from(i)) / 10.0;
                    let red = ((1.0 + f64::cos((angle - 60.0) * PI / 180.0)) / 2.0) as f32;
                    let text_brush = render_target
                        .CreateSolidColorBrush(
                            &D2D1_COLOR_F {
                                r: red,
                                g: 0f32,
                                b: 1f32 - red,
                                a: 1f32,
                            },
                            None,
                        )
                        .unwrap();

                    let angle = (angle + global_angle) as f32;
                    let matrix = Matrix3x2::rotation(angle, center_x, center_y);
                    render_target.SetTransform(&matrix);
                    render_target.DrawTextLayout(
                        D2D_POINT_2F { x: 0f32, y: top_margin },
                        text_layout,
                        &text_brush,
                        D2D1_DRAW_TEXT_OPTIONS_NONE,
                    );
                }

                // EndDraw may not be successful for some reasons.
                // Ignores any error in this example
                let _ = render_target.EndDraw(None, None);

                // Make sure all operations is completed before copying
                // bitmap to buffer
                let _ = render_target.Flush(None::<*mut u64>, None::<*mut u64>);
            }

            let mut buffer = gst::Buffer::with_size((info.width() * info.height() * 4) as usize).unwrap();
            {
                let buffer_mut = buffer.get_mut().unwrap();
                let mut map = buffer_mut.map_writable().unwrap();
                let dst = map.as_mut_slice_of::<u8>().unwrap();

                unsafe {
                    // Bitmap size is equal to the background image size.
                    // Copy entire memory
                    bitmap.CopyPixels(std::ptr::null(), info.width() * 4, dst).unwrap();
                }
            }

            gst_video::VideoMeta::add_full(
                buffer.get_mut().unwrap(),
                gst_video::VideoFrameFlags::empty(),
                gst_video::VideoFormat::Bgra,
                info.width(),
                info.height(),
                &[0],
                &[(info.width() * 4) as i32],
            )
            .unwrap();

            // Turn the buffer into a VideoOverlayRectangle, then place
            // that into a VideoOverlayComposition and return it.
            //
            // A VideoOverlayComposition can take a Vec of such rectangles
            // spaced around the video frame, but we're just outputting 1
            // here
            let rect = gst_video::VideoOverlayRectangle::new_raw(
                &buffer,
                0,
                0,
                info.width(),
                info.height(),
                gst_video::VideoOverlayFormatFlags::PREMULTIPLIED_ALPHA,
            );

            gst_video::VideoOverlayComposition::new(Some(&rect))
                .unwrap()
        }),
    );

    // Add a signal handler to the overlay's "caps-changed" signal. This could e.g.
    // be called when the sink that we render to does not support resizing the image
    // itself - but the user just changed the window-size. The element after the overlay
    // will then change its caps and we use the notification about this change to
    // resize our canvas's size.
    // Another possibility for when this might happen is, when our video is a network
    // stream that dynamically changes resolution when enough bandwidth is available.
    overlay.connect_closure(
        "caps-changed",
        false,
        glib::closure!(move |_overlay: &gst::Element,
                             caps: &gst::Caps,
                             _width: u32,
                             _height: u32| {
            let mut drawer = drawer.lock().unwrap();
            let info = gst_video::VideoInfo::from_caps(caps).unwrap();

            unsafe {
                // Update text layout to be identical to new video resolution
                drawer.text_layout.SetMaxWidth(info.width() as f32).unwrap();
                drawer
                    .text_layout
                    .SetMaxHeight(info.height() as f32)
                    .unwrap();

                // Create new WIC bitmap with PBGRA format (pre-multiplied BGRA)
                let bitmap = drawer
                    .wic_factory
                    .CreateBitmap(
                        info.width(),
                        info.height(),
                        &GUID_WICPixelFormat32bppPBGRA,
                        WICBitmapCacheOnDemand,
                    )
                    .unwrap();

                let render_target = drawer
                    .d2d_factory
                    .CreateWicBitmapRenderTarget(
                        &bitmap,
                        &D2D1_RENDER_TARGET_PROPERTIES {
                            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                            pixelFormat: D2D1_PIXEL_FORMAT {
                                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                            },
                            // zero means default DPI
                            dpiX: 0f32,
                            dpiY: 0f32,
                            usage: D2D1_RENDER_TARGET_USAGE_NONE,
                            minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
                        },
                    )
                    .unwrap();

                drawer.render_target = Some(render_target);
                drawer.bitmap = Some(bitmap);
            }
            drawer.info = Some(info);
        }),
    );

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

fn main() {
    // WIC requires COM initialization
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
    }

    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }

    unsafe {
        CoUninitialize();
    }
}
