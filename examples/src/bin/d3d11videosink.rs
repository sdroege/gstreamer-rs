// This example demonstrates the use of the d3d11videosink's "present"
// signal and the use of Direct2D/DirectWrite APIs in Rust.
//
// Application can perform various hardware-accelerated 2D graphics operation
// (e.g., like cairo can support) and text rendering via the Windows APIs.
// In this example, 2D graphics operation and text rendering will happen
// directly to the on the DXGI swapchain's backbuffer via Windows API in
// strictly zero-copy manner

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use gst::{glib, prelude::*};
use windows::{
    core::*,
    Win32::Graphics::{
        Direct2D::{Common::*, *},
        Direct3D11::*,
        DirectWrite::*,
        Dxgi::{Common::*, *},
    },
};

struct OverlayContext {
    d2d_factory: ID2D1Factory,
    dwrite_factory: IDWriteFactory,
    text_format: IDWriteTextFormat,
    texture_desc: D3D11_TEXTURE2D_DESC,
    text_layout: Option<IDWriteTextLayout>,
    timestamp_queue: VecDeque<SystemTime>,
    avg_fps: f32,
    display_fps: f32,
    font_size: f32,
}

fn create_overlay_context() -> Arc<Mutex<OverlayContext>> {
    // Lots of DirectX APIs are marked as unsafe but the below operations
    // are not expected to be failed unless GPU hang or device remove condition
    // happens
    let d2d_factory = unsafe {
        D2D1CreateFactory::<ID2D1Factory>(D2D1_FACTORY_TYPE_MULTI_THREADED, None).unwrap()
    };
    let dwrite_factory =
        unsafe { DWriteCreateFactory::<IDWriteFactory>(DWRITE_FACTORY_TYPE_SHARED).unwrap() };

    // Font size can be updated later
    let text_format = unsafe {
        dwrite_factory
            .CreateTextFormat(
                w!("Consolas"),
                None,
                DWRITE_FONT_WEIGHT_REGULAR,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                12f32,
                w!("en-us"),
            )
            .unwrap()
    };

    Arc::new(Mutex::new(OverlayContext {
        d2d_factory,
        dwrite_factory,
        text_format,
        texture_desc: D3D11_TEXTURE2D_DESC::default(),
        text_layout: None,
        timestamp_queue: VecDeque::with_capacity(10),
        avg_fps: 0f32,
        display_fps: 0f32,
        font_size: 12f32,
    }))
}

fn main() -> Result<()> {
    gst::init().unwrap();

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("URI must be specified");
        return Ok(());
    }

    let main_loop = glib::MainLoop::new(None, false);

    let overlay_context = create_overlay_context();
    let overlay_context_weak = Arc::downgrade(&overlay_context);
    // Needs BGRA or RGBA swapchain for D2D interop,
    // and "present" signal must be explicitly enabled
    let videosink = gst::ElementFactory::make("d3d11videosink")
        .property("emit-present", true)
        .property_from_str("display-format", "DXGI_FORMAT_B8G8R8A8_UNORM")
        .build()
        .unwrap();

    // Listen "present" signal and draw overlay from the callback
    // Required operations here:
    // 1) Gets IDXGISurface and ID3D11Texture2D interface from
    //    given ID3D11RenderTargetView COM object
    //   - ID3D11Texture2D: To get texture resolution
    //   - IDXGISurface: To create Direct2D render target
    // 2) Creates or reuses IDWriteTextLayout interface
    //   - This object represents text layout we want to draw on render target
    // 3) Draw rectangle (overlay background) and text on render target
    //
    // NOTE: ID2D1Factory, IDWriteFactory, IDWriteTextFormat, and
    // IDWriteTextLayout objects are device-independent. Which can be created
    // earlier instead of creating them in the callback.
    // But ID2D1RenderTarget is a device-dependent resource.
    // The client should not hold the d2d render target object outside of
    // this callback scope because the resource must be cleared before
    // releasing/resizing DXGI swapchain.
    videosink.connect_closure(
        "present",
        false,
        glib::closure!(move |_sink: &gst::Element,
                             _device: &gst::Object,
                             rtv_raw: glib::Pointer| {
            let overlay_context = overlay_context_weak.upgrade().unwrap();
            let mut context = overlay_context.lock().unwrap();
            let dwrite_factory = context.dwrite_factory.clone();
            let d2d_factory = context.d2d_factory.clone();

            // SAFETY: transmute() below is clearly unsafe operation here.
            // Regarding the other part of the below block, all DirectX
            // APIs are marked as unsafe, except for cast.
            //
            // In theory, all the Direct3D/Direct2D APIs could fail for
            // some reasons (it's hardware!), but in practice, it's very unexpected
            // situation and any of failure below would mean we are doing
            // something in wrong way or driver bug or so.
            unsafe {
                let rtv = ID3D11RenderTargetView::from_raw_borrowed(&rtv_raw).unwrap();
                let resource = rtv.GetResource().unwrap();

                let texture = resource.cast::<ID3D11Texture2D>().unwrap();
                let desc = {
                    let mut desc = D3D11_TEXTURE2D_DESC::default();
                    texture.GetDesc(&mut desc);
                    desc
                };

                // Window size was updated, creates new text layout
                let calculate_font_size = if desc != context.texture_desc {
                    context.texture_desc = desc;
                    context.text_layout = None;
                    true
                } else {
                    false
                };

                // New fps, creates new layout
                if context.avg_fps != context.display_fps {
                    context.display_fps = context.avg_fps;
                    context.text_layout = None;
                }

                if context.text_layout.is_none() {
                    let overlay_string = format!("TextOverlay, Fps {:.1}", context.display_fps);
                    let overlay_wstring = overlay_string.encode_utf16().collect::<Vec<_>>();
                    let layout = dwrite_factory
                        .CreateTextLayout(
                            &overlay_wstring,
                            &context.text_format,
                            desc.Width as f32,
                            desc.Height as f32 / 5f32,
                        )
                        .unwrap();

                    // Adjust alignment
                    layout
                        .SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)
                        .unwrap();
                    layout
                        .SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)
                        .unwrap();

                    // XXX: This is not an efficient approach.
                    // The font size can be pre-calculated for a pre-defined
                    // window size and string length
                    let mut range = DWRITE_TEXT_RANGE {
                        startPosition: 0u32,
                        length: overlay_wstring.len() as u32,
                    };

                    if calculate_font_size {
                        let mut font_size = 12f32;
                        let mut was_decreased = false;

                        loop {
                            let mut metrics = DWRITE_TEXT_METRICS::default();
                            layout.GetMetrics(&mut metrics).unwrap();
                            layout
                                .GetFontSize2(0, &mut font_size, Some(&mut range))
                                .unwrap();

                            if metrics.widthIncludingTrailingWhitespace >= desc.Width as f32 {
                                if font_size > 1f32 {
                                    font_size -= 0.5f32;
                                    was_decreased = true;
                                    layout.SetFontSize(font_size, range).unwrap();
                                    continue;
                                }

                                break;
                            }

                            if was_decreased {
                                break;
                            }

                            if metrics.widthIncludingTrailingWhitespace < desc.Width as f32 {
                                if metrics.widthIncludingTrailingWhitespace
                                    >= desc.Width as f32 * 0.7f32
                                {
                                    break;
                                }

                                font_size += 0.5f32;
                                layout.SetFontSize(font_size, range).unwrap();
                            }
                        }

                        context.font_size = font_size;
                    } else {
                        layout.SetFontSize(context.font_size, range).unwrap();
                    }

                    context.text_layout = Some(layout);
                };

                let dxgi_surf = resource.cast::<IDXGISurface>().unwrap();
                let render_target = d2d_factory
                    .CreateDxgiSurfaceRenderTarget(
                        &dxgi_surf,
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
                let text_brush = render_target
                    .CreateSolidColorBrush(
                        &D2D1_COLOR_F {
                            r: 0f32,
                            g: 0f32,
                            b: 0f32,
                            a: 1f32,
                        },
                        None,
                    )
                    .unwrap();
                let overlay_brush = render_target
                    .CreateSolidColorBrush(
                        &D2D1_COLOR_F {
                            r: 0f32,
                            g: 0.5f32,
                            b: 0.5f32,
                            a: 0.3f32,
                        },
                        None,
                    )
                    .unwrap();

                render_target.BeginDraw();
                // Draws overlay background. It will blend overlay's background
                // color with already rendred video frame
                render_target.FillRectangle(
                    &D2D_RECT_F {
                        left: 0f32,
                        top: 0f32,
                        right: desc.Width as f32,
                        bottom: desc.Height as f32 / 5f32,
                    },
                    &overlay_brush,
                );

                // Then, renders text
                render_target.DrawTextLayout(
                    D2D_POINT_2F { x: 0f32, y: 0f32 },
                    context.text_layout.as_ref(),
                    &text_brush,
                    D2D1_DRAW_TEXT_OPTIONS_NONE,
                );

                // EndDraw may not be successful for some reasons.
                // Ignores any error in this example
                let _ = render_target.EndDraw(None, None);
            }
        }),
    );

    // Add pad probe to calculate framerate
    let sinkpad = videosink.static_pad("sink").unwrap();
    let overlay_context_weak = Arc::downgrade(&overlay_context);
    sinkpad.add_probe(gst::PadProbeType::BUFFER, move |_, probe_info| {
        if let Some(gst::PadProbeData::Buffer(_)) = probe_info.data {
            let overlay_context = overlay_context_weak.upgrade().unwrap();
            let mut context = overlay_context.lock().unwrap();
            context.timestamp_queue.push_back(SystemTime::now());
            // Updates framerate per 10 frames
            if context.timestamp_queue.len() >= 10 {
                let now = context.timestamp_queue.back().unwrap();
                let front = context.timestamp_queue.front().unwrap();
                let duration = now.duration_since(*front).unwrap().as_millis() as f32;
                context.avg_fps = 1000f32 * (context.timestamp_queue.len() - 1) as f32 / duration;
                context.timestamp_queue.clear();
            }
        }
        gst::PadProbeReturn::Ok
    });

    let playbin = gst::ElementFactory::make("playbin")
        .property("uri", &args[1])
        .property("video-sink", &videosink)
        .build()
        .unwrap();

    let main_loop_clone = main_loop.clone();
    let bus = playbin.bus().unwrap();
    let _bus_watch = bus
        .add_watch(move |_, msg| {
            use gst::MessageView;

            let main_loop = &main_loop_clone;
            match msg.view() {
                MessageView::Eos(..) => {
                    println!("received eos");
                    main_loop.quit()
                }
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    main_loop.quit();
                }
                _ => (),
            };

            glib::ControlFlow::Continue
        })
        .unwrap();

    playbin.set_state(gst::State::Playing).unwrap();

    main_loop.run();

    playbin.set_state(gst::State::Null).unwrap();

    Ok(())
}
