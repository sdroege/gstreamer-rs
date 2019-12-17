extern crate failure;
use failure::Error;

extern crate glib;
extern crate gstreamer as gst;
extern crate gstreamer_pbutils as gst_pbutils;
use gst_pbutils::{
    prelude::*, Discoverer, DiscovererContainerInfo, DiscovererInfo, DiscovererResult,
    DiscovererStreamInfo,
};

use std::env;

#[path = "../tutorials-common.rs"]
mod tutorials_common;

fn send_value_as_str(v: &glib::SendValue) -> Option<String> {
    if let Ok(Some(s)) = v.get::<&str>() {
        Some(s.to_string())
    } else if let Some(serialized) = v.serialize() {
        Some(serialized)
    } else {
        None
    }
}

fn print_stream_info(info: &DiscovererStreamInfo, depth: usize) {
    let caps_str = if let Some(caps) = info.get_caps() {
        if caps.is_fixed() {
            gst_pbutils::pb_utils_get_codec_description(&caps)
                .unwrap_or_else(|_| glib::GString::from("unknown codec"))
        } else {
            glib::GString::from(caps.to_string())
        }
    } else {
        glib::GString::from("")
    };

    let stream_nick = info.get_stream_type_nick();
    println!(
        "{stream_nick:>indent$}: {caps_str}",
        stream_nick = stream_nick,
        indent = 2 * depth + stream_nick.len(),
        caps_str = caps_str
    );

    if let Some(tags) = info.get_tags() {
        println!("{:indent$}Tags:", " ", indent = 2 * depth);
        for (tag, values) in tags.iter_generic() {
            let mut tags_str = format!(
                "{tag:>indent$}: ",
                tag = tag,
                indent = 2 * (2 + depth) + tag.len()
            );
            let mut tag_num = 0;
            for value in values {
                if let Some(s) = send_value_as_str(value) {
                    if tag_num > 0 {
                        tags_str.push_str(", ")
                    }
                    tags_str.push_str(&s[..]);
                    tag_num += 1;
                }
            }

            println!("{}", tags_str);
        }
    };
}

/* Print information regarding a stream and its substreams, if any */
fn print_topology(info: &DiscovererStreamInfo, depth: usize) {
    print_stream_info(info, depth);

    if let Some(next) = info.get_next() {
        print_topology(&next, depth + 1);
    } else if let Some(container_info) = info.downcast_ref::<DiscovererContainerInfo>() {
        for stream in container_info.get_streams() {
            print_topology(&stream, depth + 1);
        }
    }
}

fn on_discovered(
    _discoverer: &Discoverer,
    discoverer_info: &DiscovererInfo,
    error: Option<&glib::Error>,
) {
    let uri = discoverer_info.get_uri().unwrap();
    match discoverer_info.get_result() {
        DiscovererResult::Ok => println!("Discovered {}", uri),
        DiscovererResult::UriInvalid => println!("Invalid uri {}", uri),
        DiscovererResult::Error => {
            if let Some(msg) = error {
                println!("{}", msg);
            } else {
                println!("Unknown error")
            }
        }
        DiscovererResult::Timeout => println!("Timeout"),
        DiscovererResult::Busy => println!("Busy"),
        DiscovererResult::MissingPlugins => {
            if let Some(s) = discoverer_info.get_misc() {
                println!("{}", s);
            }
        }
        _ => println!("Unknown result"),
    }

    if discoverer_info.get_result() != DiscovererResult::Ok {
        return;
    }

    println!("Duration: {}", discoverer_info.get_duration());

    if let Some(tags) = discoverer_info.get_tags() {
        println!("Tags:");
        for (tag, values) in tags.iter_generic() {
            print!("  {}: ", tag);
            values.for_each(|v| {
                if let Some(s) = send_value_as_str(v) {
                    println!("{}", s)
                }
            })
        }
    }

    println!(
        "Seekable: {}",
        if discoverer_info.get_seekable() {
            "yes"
        } else {
            "no"
        }
    );

    println!("Stream information:");

    if let Some(stream_info) = discoverer_info.get_stream_info() {
        print_topology(&stream_info, 1);
    }
}

fn run_discoverer() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm"
    };

    println!("Discovering {}", uri);

    let loop_ = glib::MainLoop::new(None, false);
    let timeout = 5 * gst::SECOND;
    let discoverer = gst_pbutils::Discoverer::new(timeout)?;
    discoverer.connect_discovered(on_discovered);
    let loop_clone = loop_.clone();
    discoverer.connect_finished(move |_| {
        println!("\nFinished discovering");
        loop_clone.quit();
    });
    discoverer.start();
    discoverer.discover_uri_async(uri)?;

    loop_.run();

    discoverer.stop();

    Ok(())
}

fn tutorial_main() {
    match run_discoverer() {
        Ok(_) => {}
        Err(err) => eprintln!("Failed to run discovery: {}", err),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    tutorials_common::run(tutorial_main);
}
