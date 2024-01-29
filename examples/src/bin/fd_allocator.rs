// This example demonstrates the use of the FdMemory allocator.
// It operates the following two pipelines:

// sender:   {videotestsrc} - {appsink}
// receiver: {appsrc} - {FdMemoryVideoFilter} - {videoconvert} - {queue} - {autovideosink}

// The sender creates shared memory files from the appsink which are sent
// to the receiver using a unix domain socket.
// The receiver creates buffers in the appsrc using the FdMemoryAllocator from
// the received file descriptors.

// Additional to demonstrating how the FdMemoryAllocator can be used to share
// file descriptors the example implements a custom VideoFilter demonstrating
// how the file descriptor of FdMemory can be accessed in a pipeline.
// Note that instead of manual mapping the file descriptor it is also possible
// to use map_writable, which will also map the file descriptor.
use std::{
    os::unix::{net::UnixStream, prelude::AsRawFd},
    sync::{Arc, Mutex},
};

use anyhow::Error;
use futures::StreamExt;
use gst::{element_error, prelude::*};
use memmap2::MmapMut;
use uds::UnixStreamExt;

#[path = "../examples-common.rs"]
mod examples_common;

fn create_receiver_pipeline(
    video_info: &gst_video::VideoInfo,
    receiver: UnixStream,
) -> Result<gst::Pipeline, Error> {
    let caps = video_info.to_caps()?;

    let pipeline = gst::Pipeline::default();
    let src = gst_app::AppSrc::builder()
        .caps(&caps)
        .do_timestamp(true)
        .is_live(true)
        .build();
    let filter = video_filter::FdMemoryFadeInVideoFilter::default().upcast::<gst::Element>();
    let convert = gst::ElementFactory::make("videoconvert").build()?;
    let queue = gst::ElementFactory::make("queue").build()?;
    let sink = gst::ElementFactory::make("autovideosink").build()?;

    pipeline.add_many([src.upcast_ref(), &filter, &convert, &queue, &sink])?;
    gst::Element::link_many([src.upcast_ref(), &filter, &convert, &queue, &sink])?;

    let fd_allocator = gst_allocators::FdAllocator::new();
    let video_info = video_info.clone();
    let mut fd_buf = [-1; 253];

    src.set_callbacks(
        gst_app::AppSrcCallbacks::builder()
            .need_data(move |appsrc, _| {
                // Read the next fds from the socket, if 0
                // is returned the sender has closed the stream
                // which is handled as EOS here.
                let fds = match receiver.recv_fds(&mut [0u8; 1], &mut fd_buf) {
                    Ok((_, 0)) => {
                        let _ = appsrc.end_of_stream();
                        return;
                    }
                    Ok((_, fds)) => fds,
                    Err(err) => {
                        gst::error_msg!(
                            gst::StreamError::Failed,
                            ("failed to receive fds: {}", err)
                        );
                        return;
                    }
                };

                for fd in &fd_buf[0..fds] {
                    // Allocate a new FdMemory for the received file descriptor.
                    // It is important that the size matches the size of the
                    // actual backing storage. In this example we just use the
                    // same video info in both sides, sending and receiving.
                    // Pass FdMemoryFlags::NONE to make the FdMemory take
                    // ownership of the passed file descriptor. The file descriptor
                    // will be closed when the memory is released.
                    let memory = unsafe {
                        fd_allocator
                            .alloc(*fd, video_info.size(), gst_allocators::FdMemoryFlags::NONE)
                            .unwrap()
                    };
                    let mut buffer = gst::Buffer::new();
                    let buffer_mut = buffer.make_mut();
                    buffer_mut.append_memory(memory);
                    let _ = appsrc.push_buffer(buffer);
                }
            })
            .build(),
    );

    Ok(pipeline)
}

fn create_sender_pipeline(
    video_info: &gst_video::VideoInfo,
    sender: UnixStream,
) -> Result<gst::Pipeline, Error> {
    let sender = Arc::new(Mutex::new(sender));
    let caps = video_info.to_caps()?;

    let pipeline = gst::Pipeline::default();
    let src = gst::ElementFactory::make("videotestsrc")
        .property("num-buffers", 250i32)
        .build()?;
    let sink = gst::ElementFactory::make("appsink").build()?;

    sink.downcast_ref::<gst_app::AppSink>()
        .ok_or_else(|| anyhow::anyhow!("is not a appsink"))?
        .set_caps(Some(&caps));

    pipeline.add_many([&src, &sink])?;
    gst::Element::link_many([&src, &sink])?;

    let appsink = sink
        .downcast::<gst_app::AppSink>()
        .map_err(|_| anyhow::anyhow!("is not a appsink"))?;

    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            // Add a handler to the "eos" signal
            .eos({
                let sender = sender.clone();
                move |_| {
                    // Close the sender part of the UnixSocket pair, this will automatically
                    // create a eos in the receiving part.
                    let _ = sender.lock().unwrap().shutdown(std::net::Shutdown::Write);
                }
            })
            // Add a handler to the "new-sample" signal.
            .new_sample(move |appsink| {
                // Pull the sample in question out of the appsink's buffer.
                let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or_else(|| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gst::FlowError::Error
                })?;

                if buffer.n_memory() != 1 {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Expected buffer with single memory")
                    );

                    return Err(gst::FlowError::Error);
                }

                let mem = buffer.peek_memory(0);

                // We can use downcast_memory_ref to check if the provided
                // memory is allocated by FdMemoryAllocator or a subtype of it.
                // Note: This is not used in the example, we will always copy
                // the memory to a new shared memory file.
                if let Some(fd_memory) = mem.downcast_memory_ref::<gst_allocators::FdMemory>() {
                    // As we already got a fd we can just directly send it over the socket.
                    // NOTE: Synchronization is left out of this example, in a real world
                    // application access to the memory should be synchronized.
                    // For example wayland provides a release callback to signal that
                    // the memory is no longer in use.
                    sender
                        .lock()
                        .unwrap()
                        .send_fds(&[0u8; 1], &[fd_memory.fd()])
                        .map_err(|_| {
                            element_error!(
                                appsink,
                                gst::ResourceError::Failed,
                                ("Failed to send fd over unix stream")
                            );

                            gst::FlowError::Error
                        })?;
                } else {
                    // At this point, buffer is only a reference to an existing memory region somewhere.
                    // When we want to access its content, we have to map it while requesting the required
                    // mode of access (read, read/write).
                    // This type of abstraction is necessary, because the buffer in question might not be
                    // on the machine's main memory itself, but rather in the GPU's memory.
                    // So mapping the buffer makes the underlying memory region accessible to us.
                    // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
                    let map = buffer.map_readable().map_err(|_| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to map buffer readable")
                        );

                        gst::FlowError::Error
                    })?;

                    // Note: To simplify this example we always create a new shared memory file instead
                    // of using a pool of buffers. When using a pool we need to make sure access to the
                    // file is synchronized.
                    let opts = memfd::MemfdOptions::default().allow_sealing(true);
                    let mfd = opts.create("gst-examples").map_err(|err| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to allocated fd: {}", err)
                        );

                        gst::FlowError::Error
                    })?;

                    mfd.as_file().set_len(map.size() as u64).map_err(|err| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to resize fd memory: {}", err)
                        );

                        gst::FlowError::Error
                    })?;

                    let mut seals = memfd::SealsHashSet::new();
                    seals.insert(memfd::FileSeal::SealShrink);
                    seals.insert(memfd::FileSeal::SealGrow);
                    mfd.add_seals(&seals).map_err(|err| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to add fd seals: {}", err)
                        );

                        gst::FlowError::Error
                    })?;

                    mfd.add_seal(memfd::FileSeal::SealSeal).map_err(|err| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to add fd seals: {}", err)
                        );

                        gst::FlowError::Error
                    })?;

                    unsafe {
                        let mut mmap = MmapMut::map_mut(mfd.as_file()).map_err(|_| {
                            element_error!(
                                appsink,
                                gst::ResourceError::Failed,
                                ("Failed to mmap fd")
                            );

                            gst::FlowError::Error
                        })?;

                        mmap.copy_from_slice(map.as_slice());
                    };

                    sender
                        .lock()
                        .unwrap()
                        .send_fds(&[0u8; 1], &[mfd.as_raw_fd()])
                        .map_err(|_| {
                            element_error!(
                                appsink,
                                gst::ResourceError::Failed,
                                ("Failed to send fd over unix stream")
                            );

                            gst::FlowError::Error
                        })?;
                };

                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );

    Ok(pipeline)
}

async fn message_loop(bus: gst::Bus) {
    let mut messages = bus.stream();

    while let Some(msg) = messages.next().await {
        use gst::MessageView;

        // Determine whether we want to quit: on EOS or error message
        // we quit, otherwise simply continue.
        match msg.view() {
            MessageView::Eos(..) => break,
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
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let video_info = gst_video::VideoInfo::builder(gst_video::VideoFormat::Bgra, 1920, 1080)
        .fps(gst::Fraction::new(30, 1))
        .build()?;

    let (sender, receiver) = std::os::unix::net::UnixStream::pair()?;
    let sender_pipeline = create_sender_pipeline(&video_info, sender)?;
    let receiver_pipeline = create_receiver_pipeline(&video_info, receiver)?;

    let receiver_bus = receiver_pipeline.bus().expect("pipeline without bus");
    receiver_pipeline.set_state(gst::State::Playing)?;

    let sender_bus = sender_pipeline.bus().expect("pipeline without bus");
    sender_pipeline.set_state(gst::State::Playing)?;

    futures::executor::block_on(futures::future::join(
        message_loop(sender_bus),
        message_loop(receiver_bus),
    ));

    sender_pipeline.set_state(gst::State::Null)?;
    receiver_pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}

// The purpose of this custom video filter is to demonstrate how
// the file descriptor of a FdMemory can be accessed.
mod video_filter {
    glib::wrapper! {
        pub struct FdMemoryFadeInVideoFilter(ObjectSubclass<imp::FdMemoryFadeInVideoFilter>) @extends gst_video::VideoFilter, gst_base::BaseTransform, gst::Element, gst::Object;
    }

    impl Default for FdMemoryFadeInVideoFilter {
        fn default() -> Self {
            glib::Object::builder().build()
        }
    }
    mod imp {
        use std::{mem::ManuallyDrop, os::unix::prelude::FromRawFd};

        use anyhow::Error;
        use gst::{subclass::prelude::*, PadDirection, PadPresence, PadTemplate};
        use gst_app::gst_base::subclass::BaseTransformMode;
        use gst_video::{prelude::*, subclass::prelude::*, VideoFrameRef};
        use memmap2::MmapMut;
        use once_cell::sync::Lazy;

        static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
            gst::DebugCategory::new(
                "fdmemoryfilter",
                gst::DebugColorFlags::empty(),
                Some("Example FdMemory filter"),
            )
        });

        #[derive(Debug, Default)]
        pub struct FdMemoryFadeInVideoFilter;

        impl FdMemoryFadeInVideoFilter {
            fn transform_fd_mem_ip(
                &self,
                frame: &mut VideoFrameRef<&mut gst::BufferRef>,
            ) -> Result<(), Error> {
                let buffer = frame.buffer();
                if buffer.n_memory() != 1 {
                    return Err(anyhow::anyhow!(
                        "only buffers with single memory are supported"
                    ));
                }
                let mem = buffer.peek_memory(0);
                if !mem.is_memory_type::<gst_allocators::FdMemory>() {
                    return Err(anyhow::anyhow!("only fd memory is supported"));
                }

                let timestamp = buffer.pts().unwrap();
                let factor = (timestamp.nseconds() as f64
                    / (5 * gst::ClockTime::SECOND).nseconds() as f64)
                    .min(1.0f64);

                // If the fade-in has finished return early
                if factor >= 1.0f64 {
                    return Ok(());
                }

                let fd = mem
                    .downcast_memory_ref::<gst_allocators::FdMemory>()
                    .unwrap()
                    .fd();

                unsafe {
                    // We wrap the Memmfd in ManuallyDrop here because from_raw_fd takes ownership of
                    // the file descriptor which would close it on drop
                    //
                    // see: https://github.com/lucab/memfd-rs/issues/29
                    let mfd = ManuallyDrop::new(memfd::Memfd::from_raw_fd(fd));
                    let mut mmap = MmapMut::map_mut(mfd.as_file())?;

                    for pixel in mmap.chunks_exact_mut(4) {
                        pixel[0] = (pixel[0] as f64 * factor).clamp(0.0, 255.0) as u8;
                        pixel[1] = (pixel[1] as f64 * factor).clamp(0.0, 255.0) as u8;
                        pixel[2] = (pixel[2] as f64 * factor).clamp(0.0, 255.0) as u8;
                    }
                }

                Ok(())
            }
        }

        impl ElementImpl for FdMemoryFadeInVideoFilter {
            fn pad_templates() -> &'static [PadTemplate] {
                static PAD_TEMPLATES: std::sync::OnceLock<Vec<PadTemplate>> =
                    std::sync::OnceLock::new();

                PAD_TEMPLATES.get_or_init(|| {
                    let caps = gst_video::VideoCapsBuilder::new()
                        .format(gst_video::VideoFormat::Bgra)
                        .build();
                    vec![
                        PadTemplate::new("sink", PadDirection::Sink, PadPresence::Always, &caps)
                            .unwrap(),
                        PadTemplate::new("src", PadDirection::Src, PadPresence::Always, &caps)
                            .unwrap(),
                    ]
                })
            }
        }

        impl BaseTransformImpl for FdMemoryFadeInVideoFilter {
            const MODE: BaseTransformMode = BaseTransformMode::AlwaysInPlace;
            const PASSTHROUGH_ON_SAME_CAPS: bool = false;
            const TRANSFORM_IP_ON_PASSTHROUGH: bool = true;
        }

        impl VideoFilterImpl for FdMemoryFadeInVideoFilter {
            fn transform_frame_ip(
                &self,
                frame: &mut VideoFrameRef<&mut gst::BufferRef>,
            ) -> Result<gst::FlowSuccess, gst::FlowError> {
                self.transform_fd_mem_ip(frame).map_err(|err| {
                    gst::error!(CAT, imp: self, "Failed to transform frame`: {}", err);
                    gst::FlowError::Error
                })?;

                Ok(gst::FlowSuccess::Ok)
            }
        }

        impl ObjectImpl for FdMemoryFadeInVideoFilter {}

        impl GstObjectImpl for FdMemoryFadeInVideoFilter {}

        #[glib::object_subclass]
        impl ObjectSubclass for FdMemoryFadeInVideoFilter {
            const NAME: &'static str = "FdMemoryVideoFilter";
            type Type = super::FdMemoryFadeInVideoFilter;
            type ParentType = gst_video::VideoFilter;
        }
    }
}
