use anyhow::Error;
use clap::Parser;
use ges::prelude::*;
use gst_audio::AudioChannelPosition;

#[path = "../examples-common.rs"]
mod examples_common;

/// This example demonstrates how to use GES with audioconvert's mix-matrix property
/// to split a multi-channel audio file into separate clips, one per channel.
///
/// Each clip is placed in its own layer and all clips are grouped together so they
/// can be controlled as a single unit.
///
/// It also demonstrates the select-element-track signal for selecting specific audio
/// streams in multi-stream files, allowing precise control over which stream to use.
#[derive(Parser, Debug)]
#[command(name = "ges_audio_channel_split")]
struct Args {
    /// Input audio file URI
    #[arg(value_name = "INPUT_URI")]
    input_uri: String,

    /// Optional output file path
    #[arg(value_name = "OUTPUT_FILE")]
    output_file: Option<String>,

    /// Comma-separated list of channels to silence (e.g., 'SideLeft,SideRight')
    #[arg(short = 'd', long = "drop-channels", value_name = "CHANNELS")]
    drop_channels: Option<String>,

    /// Audio stream number to use (0-based index) for multi-stream files.
    /// Default -1 uses automatic selection. Demonstrates select-element-track signal.
    #[arg(
        short = 's',
        long = "stream",
        value_name = "STREAM_NUM",
        default_value = "-1"
    )]
    stream_number: i32,
}

fn create_mix_matrix_value(num_channels: usize, channel_index: usize) -> glib::Value {
    // Create a mix matrix with N output channels and N input channels
    // Only the specified channel_index is passed through in its correct position
    // All other output channels are silent (all coefficients = 0.0)
    let array = (0..num_channels)
        .map(|output_idx| {
            (0..num_channels)
                .map(|input_index| {
                    if output_idx == channel_index && input_index == channel_index {
                        1.0f32
                    } else {
                        0.0f32
                    }
                    .to_send_value()
                })
                .collect::<gst::Array>()
                .to_send_value()
        })
        .collect::<gst::Array>();
    array.to_value()
}

fn run(
    uri: &str,
    output: Option<&String>,
    drop_channels: Option<&String>,
    stream_number: i32,
) -> Result<(), Error> {
    ges::init()?;
    gst::init()?;

    println!("Loading asset from URI: {uri}");

    // Request the URI clip asset synchronously
    let asset = ges::UriClipAsset::request_sync(uri)?;

    // Get discoverer info from the asset
    let discoverer_info = asset.info();

    // Get audio streams from the discoverer info
    let audio_streams = discoverer_info.audio_streams();
    if audio_streams.is_empty() {
        return Err(anyhow::anyhow!("No audio streams found in the file"));
    }

    // Get the stream numbers for all audio streams
    let audio_stream_numbers: Vec<i32> = audio_streams
        .iter()
        .map(|stream| stream.stream_number())
        .collect();

    println!(
        "Found {} audio stream(s) with stream numbers: {audio_stream_numbers:?}",
        audio_streams.len()
    );

    // Select audio stream based on stream_number
    let audio_info = if stream_number >= 0 {
        // Find the audio stream with the matching stream_number
        let audio_info = audio_streams
            .iter()
            .find(|stream| stream.stream_number() == stream_number)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Audio stream number {stream_number} not found (available audio streams: {audio_stream_numbers:?})"
                )
            })?;
        println!("Using audio stream {stream_number}");
        audio_info
    } else {
        println!("Using default audio stream selection");
        &audio_streams[0]
    };
    let num_channels = audio_info.channels() as usize;
    let channel_mask = audio_info.channel_mask();

    println!("Audio info:");
    println!("  Channels: {num_channels}");
    println!("  Channel mask: {channel_mask:#x}");
    println!("  Sample rate: {} Hz", audio_info.sample_rate());

    // Get channel positions from the mask
    let mut positions = vec![AudioChannelPosition::None; num_channels];
    if channel_mask != 0 {
        AudioChannelPosition::positions_from_mask(channel_mask, &mut positions)?;
    } else {
        println!("  Warning: No channel mask set, using channel indices");
    }

    println!("\nChannel positions:");
    for (i, pos) in positions.iter().enumerate() {
        println!("  Channel {i}: {pos:?}");
    }

    // Parse channels to drop
    let channels_to_drop = if let Some(drop_str) = drop_channels {
        drop_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec![]
    };

    if !channels_to_drop.is_empty() {
        println!("\nChannels to drop: {channels_to_drop:?}");
    }

    // Create timeline with only audio track (no video)
    let timeline = ges::Timeline::new();
    let audio_track = ges::AudioTrack::new();

    // Set track restriction caps to maintain the proper channel layout
    let restriction_caps = if channel_mask != 0 {
        gst::Caps::builder("audio/x-raw")
            .field("channels", num_channels as i32)
            .field("channel-mask", gst::Bitmask::new(channel_mask))
            .build()
    } else {
        gst::Caps::builder("audio/x-raw")
            .field("channels", num_channels as i32)
            .build()
    };
    audio_track.set_restriction_caps(&restriction_caps);
    println!("\nSet audio track restriction caps: {}", restriction_caps);
    timeline.add_track(&audio_track)?;

    // Create one clip per channel, each in its own layer
    let mut clips = Vec::new();

    for (channel_idx, position) in positions.iter().enumerate() {
        println!("\nCreating clip for channel {channel_idx} ({position:?})");

        // Create a new layer for this channel
        let layer = timeline.append_layer();

        // Extract a clip from the asset
        let clip = asset
            .extract()?
            .downcast::<ges::UriClip>()
            .map_err(|_| anyhow::anyhow!("Failed to downcast to UriClip"))?;

        // Set the clip name based on the channel position
        let clip_name = if channel_mask != 0 {
            position.to_string()
        } else {
            format!("Channel{channel_idx}")
        };
        let _ = clip.set_name(Some(&clip_name));

        // Connect select-element-track signal if a specific stream is requested
        let signal_handler = if stream_number >= 0 {
            let selected_stream = stream_number;
            Some(
                timeline.connect_select_element_track(move |timeline, _clip, track_element| {
                    // Get the track element's asset and check if it's the selected stream
                    track_element
                        .asset()
                        .and_then(|asset| asset.downcast::<ges::UriSourceAsset>().ok())
                        .and_then(|uri_asset| {
                            let stream_info = uri_asset.stream_info();
                            let stream_num = stream_info.stream_number();
                            if stream_num == selected_stream {
                                println!(
                                    "  SELECT STREAM: element={}, stream_number={}",
                                    track_element.name().unwrap_or_default(),
                                    stream_num
                                );
                                timeline.tracks().first().cloned()
                            } else {
                                println!(
                                    "  IGNORE STREAM: element={}, stream_number={}",
                                    track_element.name().unwrap_or_default(),
                                    stream_num
                                );
                                None
                            }
                        })
                }),
            )
        } else {
            None
        };

        // Add the clip to its layer first (needed before setting child properties)
        layer.add_clip(&clip)?;

        // Disconnect the signal handler after adding the clip
        if let Some(handler) = signal_handler {
            timeline.disconnect(handler);
        }

        // Check if this channel should be dropped (set inactive)
        let should_drop = channels_to_drop.contains(&clip_name);
        if should_drop {
            if let Some(track_element) = clip
                .children(false)
                .first()
                .and_then(|child| child.downcast_ref::<ges::TrackElement>())
            {
                track_element.set_active(false);
                println!("  Marking clip as inactive (dropped)");
            } else {
                eprintln!(
                    "  Warning: Could not find track element to deactivate for channel {}",
                    clip_name
                );
            }
        }

        // Create the mix matrix for this channel
        let mix_matrix = create_mix_matrix_value(num_channels, channel_idx);

        // Set the mix-matrix property on the clip
        // This will be applied to the internal audioconvert element
        clip.set_child_property("mix-matrix", &mix_matrix)
            .map_err(|e| anyhow::anyhow!("Failed to set mix-matrix: {}", e))?;

        println!("  Set mix-matrix to extract channel {channel_idx}");

        clips.push(clip);
    }

    println!("\nGrouping {} clips together...", clips.len());

    // Group all clips together so they can be controlled similarly to a clip
    // in the timeline
    let containers = clips
        .iter()
        .map(|c| c.clone().upcast::<ges::Container>())
        .collect::<Vec<_>>();
    let group = ges::Container::group(&containers)
        .ok_or_else(|| anyhow::anyhow!("Failed to group clips"))?;

    println!(
        "Created group: {:?}",
        group.name().unwrap_or_else(|| "Unnamed".into())
    );

    // Create the pipeline
    let pipeline = ges::Pipeline::new();
    pipeline.set_timeline(&timeline)?;

    // If output is specified, configure for rendering
    if let Some(output_path) = output {
        // Convert path to URI if needed
        let output_uri = if output_path.starts_with("file://") {
            output_path.to_string()
        } else {
            let path = std::path::Path::new(output_path);
            let absolute_path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                std::env::current_dir()?.join(path)
            };
            format!("file://{}", absolute_path.display())
        };

        println!("\nConfiguring pipeline for rendering to: {output_uri}");

        // Create encoding profile for raw audio in MP4 container
        let audio_profile =
            gst_pbutils::EncodingAudioProfile::builder(&gst::Caps::builder("audio/x-raw").build())
                .build();

        let container_profile = gst_pbutils::EncodingContainerProfile::builder(
            &gst::Caps::builder("video/quicktime").build(),
        )
        .name("mp4")
        .add_profile(audio_profile)
        .build();

        pipeline.set_render_settings(&output_uri, &container_profile)?;
        pipeline.set_mode(ges::PipelineFlags::RENDER)?;
    } else {
        println!("\nPlaying...");
    }

    // Start playing
    pipeline.set_state(gst::State::Playing)?;

    // Wait for EOS or error
    let bus = pipeline.bus().expect("Pipeline without bus");
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                println!("\nEnd of stream");
                break;
            }
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(anyhow::anyhow!(
                    "Error from {:?}: {} ({:?})",
                    msg.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                ));
            }
            MessageView::StateChanged(state_changed) => {
                if msg.src() == Some(pipeline.upcast_ref()) {
                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        state_changed.old(),
                        state_changed.current()
                    );
                }
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    println!("\n=== Summary ===");
    println!("Successfully split {num_channels} channels into individual clips");
    println!("Each clip extracts one channel using audioconvert's mix-matrix property");
    println!("All clips are grouped together for unified timeline control");

    Ok(())
}

fn example_main() {
    let args = Args::parse();

    match run(
        &args.input_uri,
        args.output_file.as_ref(),
        args.drop_channels.as_ref(),
        args.stream_number,
    ) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn main() {
    examples_common::run(example_main);
}
