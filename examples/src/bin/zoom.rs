// Zoom example using navigation events and a compositor

// Use can change the video player zoom using the next keys:
// * +: Zoom in
// * -: Zoom out
// * Up/Down/Right/Left: Move the frame
// * r: reset the zoom
// Also mouse navigation events can be used for a better UX.

use gst::prelude::*;
use gst_video::video_event::NavigationEvent;
use std::sync::Mutex;

#[path = "../examples-common.rs"]
mod examples_common;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

#[derive(Default)]
struct MouseState {
    clicked: bool,
    clicked_x: f64,
    clicked_y: f64,
    clicked_xpos: i32,
    clicked_ypos: i32,
}

fn zoom(mixer_sink_pad: gst::Pad, x: i32, y: i32, zoom_in: bool) {
    let xpos = mixer_sink_pad.property::<i32>("xpos");
    let ypos = mixer_sink_pad.property::<i32>("ypos");
    let width = mixer_sink_pad.property::<i32>("width");
    let height = mixer_sink_pad.property::<i32>("height");

    let (width_offset, height_offset) = if zoom_in {
        (WIDTH / 10, HEIGHT / 10)
    } else {
        (-WIDTH / 10, -HEIGHT / 10)
    };

    if width_offset + width <= 0 {
        return;
    }

    mixer_sink_pad.set_property("width", width + width_offset);
    mixer_sink_pad.set_property("height", height + height_offset);

    let xpos_offset = ((x as f32 / WIDTH as f32) * width_offset as f32) as i32;
    let new_xpos = xpos - xpos_offset;
    let ypos_offset = ((y as f32 / HEIGHT as f32) * height_offset as f32) as i32;
    let new_ypos = ypos - ypos_offset;

    if new_xpos != xpos {
        mixer_sink_pad.set_property("xpos", new_xpos);
    }
    if new_ypos != ypos {
        mixer_sink_pad.set_property("ypos", new_ypos);
    }
}

fn reset_zoom(mixer_sink_pad: gst::Pad) {
    let xpos = mixer_sink_pad.property::<i32>("xpos");
    let ypos = mixer_sink_pad.property::<i32>("ypos");
    let width = mixer_sink_pad.property::<i32>("width");
    let height = mixer_sink_pad.property::<i32>("height");

    if 0 != xpos {
        mixer_sink_pad.set_property("xpos", 0);
    }
    if 0 != ypos {
        mixer_sink_pad.set_property("ypos", 0);
    }
    if WIDTH != width {
        mixer_sink_pad.set_property("width", WIDTH);
    }
    if HEIGHT != height {
        mixer_sink_pad.set_property("height", HEIGHT);
    }
}

fn example_main() {
    let clicked = Mutex::new(MouseState::default());

    gst::init().unwrap();

    let pipeline = gst::parse::launch(&format!(
        "compositor name=mix background=1 sink_0::xpos=0 sink_0::ypos=0 sink_0::zorder=0 sink_0::width={WIDTH} sink_0::height={HEIGHT} ! xvimagesink \
         videotestsrc  name=src ! video/x-raw,framerate=30/1,width={WIDTH},height={HEIGHT},pixel-aspect-ratio=1/1 ! queue ! mix.sink_0"
    )).unwrap().downcast::<gst::Pipeline>().unwrap();

    let mixer = pipeline.by_name("mix").unwrap();
    let mixer_src_pad = mixer.static_pad("src").unwrap();
    let mixer_sink_pad_weak = mixer.static_pad("sink_0").unwrap().downgrade();

    // Probe added in the sink pad to get direct navigation events w/o transformation done by the mixer
    mixer_src_pad.add_probe(gst::PadProbeType::EVENT_UPSTREAM, move |_, probe_info| {
        let mixer_sink_pad = mixer_sink_pad_weak.upgrade().unwrap();

        let Some(ev) = probe_info.event() else {
            return gst::PadProbeReturn::Ok;
        };

        if ev.type_() != gst::EventType::Navigation {
            return gst::PadProbeReturn::Ok;
        };

        let Ok(nav_event) = NavigationEvent::parse(ev) else {
            return gst::PadProbeReturn::Ok;
        };

        match nav_event {
            NavigationEvent::KeyPress { key, .. } => match key.as_str() {
                "Left" => {
                    let xpos = mixer_sink_pad.property::<i32>("xpos");
                    mixer_sink_pad.set_property("xpos", xpos - 10);
                }
                "Right" => {
                    let xpos = mixer_sink_pad.property::<i32>("xpos");
                    mixer_sink_pad.set_property("xpos", xpos + 10);
                }
                "Up" => {
                    let ypos = mixer_sink_pad.property::<i32>("ypos");
                    mixer_sink_pad.set_property("ypos", ypos - 10);
                }
                "Down" => {
                    let ypos = mixer_sink_pad.property::<i32>("ypos");
                    mixer_sink_pad.set_property("ypos", ypos + 10);
                }
                "plus" => {
                    zoom(mixer_sink_pad, WIDTH / 2, HEIGHT / 2, true);
                }
                "minus" => {
                    zoom(mixer_sink_pad, WIDTH / 2, HEIGHT / 2, false);
                }
                "r" => {
                    reset_zoom(mixer_sink_pad);
                }
                _ => (),
            },
            NavigationEvent::MouseMove { x, y, .. } => {
                let state = clicked.lock().unwrap();
                if state.clicked {
                    let xpos = mixer_sink_pad.property::<i32>("xpos");
                    let ypos = mixer_sink_pad.property::<i32>("ypos");

                    let new_xpos = state.clicked_xpos + (x - state.clicked_x) as i32;
                    let new_ypos = state.clicked_ypos + (y - state.clicked_y) as i32;

                    if new_xpos != xpos {
                        mixer_sink_pad.set_property("xpos", new_xpos);
                    }

                    if new_ypos != ypos {
                        mixer_sink_pad.set_property("ypos", new_ypos);
                    }
                }
            }
            NavigationEvent::MouseButtonPress { button, x, y, .. } => {
                if button == 1 || button == 272 {
                    let mut state = clicked.lock().unwrap();
                    state.clicked = true;
                    state.clicked_x = x;
                    state.clicked_y = y;
                    state.clicked_xpos = mixer_sink_pad.property("xpos");
                    state.clicked_ypos = mixer_sink_pad.property("ypos");
                } else if button == 2 || button == 3 || button == 274 || button == 273 {
                    reset_zoom(mixer_sink_pad);
                } else if button == 4 {
                    zoom(mixer_sink_pad, x as i32, y as i32, true);
                } else if button == 5 {
                    zoom(mixer_sink_pad, x as i32, y as i32, false);
                }
            }
            NavigationEvent::MouseButtonRelease { button, .. } => {
                if button == 1 || button == 272 {
                    let mut state = clicked.lock().unwrap();
                    state.clicked = false;
                }
            }
            #[cfg(feature = "v1_18")]
            NavigationEvent::MouseScroll { x, y, delta_y, .. } => {
                if delta_y > 0.0 {
                    zoom(mixer_sink_pad, x as i32, y as i32, true);
                } else if delta_y < 0.0 {
                    zoom(mixer_sink_pad, x as i32, y as i32, false);
                }
            }
            _ => (),
        }

        gst::PadProbeReturn::Ok
    });

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                println!("received eos");
                break;
            }
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        };
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
