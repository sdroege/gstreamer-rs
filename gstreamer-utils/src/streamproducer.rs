use std::collections::HashMap;
use std::mem;
use std::sync::{atomic, Arc, Mutex};

use once_cell::sync::Lazy;

use gst::{glib, prelude::*};

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "utilsrs-stream-producer",
        gst::DebugColorFlags::empty(),
        Some("gst_app Stream Producer interface"),
    )
});

/// The interface for transporting media data from one node
/// to another.
///
/// A producer is essentially a GStreamer `appsink` whose output
/// is sent to a set of consumers, who are essentially `appsrc` wrappers
#[derive(Debug, Clone)]
pub struct StreamProducer {
    /// The appsink to dispatch data for
    appsink: gst_app::AppSink,
    /// The consumers to dispatch data to
    consumers: Arc<Mutex<StreamConsumers>>,
}

impl PartialEq for StreamProducer {
    fn eq(&self, other: &Self) -> bool {
        self.appsink.eq(&other.appsink)
    }
}

impl Eq for StreamProducer {}

impl StreamProducer {
    /// Configure a consumer `appsrc` for later use in a `StreamProducer`
    ///
    /// This is automatically called when calling `add_consumer()`.
    pub fn configure_consumer(consumer: &gst_app::AppSrc) {
        // Latency on the appsrc is set by the publisher before the first buffer
        // and whenever it changes
        consumer.set_latency(gst::ClockTime::ZERO, gst::ClockTime::NONE);
        consumer.set_format(gst::Format::Time);
        consumer.set_is_live(true);
        consumer.set_handle_segment_change(true);
        consumer.set_max_buffers(0);
        consumer.set_max_bytes(0);
        consumer.set_max_time(500 * gst::ClockTime::MSECOND);
        consumer.set_leaky_type(gst_app::AppLeakyType::Downstream);
        consumer.set_automatic_eos(false);
    }

    /// Add an appsrc to dispatch data to
    pub fn add_consumer(&self, consumer: &gst_app::AppSrc) {
        let mut consumers = self.consumers.lock().unwrap();
        if consumers.consumers.contains_key(consumer) {
            gst::error!(CAT, obj: &self.appsink, "Consumer {} ({:?}) already added", consumer.name(), consumer);
            return;
        }

        gst::debug!(CAT, obj: &self.appsink, "Adding consumer {} ({:?})", consumer.name(), consumer);

        Self::configure_consumer(consumer);

        // Forward force-keyunit events upstream to the appsink
        let srcpad = consumer.static_pad("src").unwrap();
        let appsink = &self.appsink;
        let fku_probe_id = srcpad
            .add_probe(
                gst::PadProbeType::EVENT_UPSTREAM,
                glib::clone!(@weak appsink, @weak consumer => @default-panic, move |_pad, info| {
                    if let Some(gst::PadProbeData::Event(ref ev)) = info.data {
                        if gst_video::UpstreamForceKeyUnitEvent::parse(ev).is_ok() {
                            gst::debug!(CAT, obj: &appsink,  "Requesting keyframe");
                            let _ = appsink.send_event(ev.clone());
                        }
                    }

                    gst::PadProbeReturn::Ok
                }),
            )
            .unwrap();

        consumers.consumers.insert(
            consumer.clone(),
            StreamConsumer::new(consumer, fku_probe_id),
        );
    }

    /// Remove a consumer appsrc by id
    pub fn remove_consumer(&self, consumer: &gst_app::AppSrc) {
        let name = consumer.name();
        if self
            .consumers
            .lock()
            .unwrap()
            .consumers
            .remove(consumer)
            .is_some()
        {
            gst::debug!(CAT, obj: &self.appsink, "Removed consumer {} ({:?})", name, consumer);
        } else {
            gst::debug!(CAT, obj: &self.appsink, "Consumer {} ({:?}) not found", name, consumer);
        }
    }

    /// Stop discarding data samples and start forwarding them to the consumers.
    ///
    /// This is useful for example for prerolling live sources.
    pub fn forward(&self) {
        self.consumers.lock().unwrap().discard = false;
    }

    /// Get the GStreamer `appsink` wrapped by this producer
    pub fn appsink(&self) -> &gst_app::AppSink {
        &self.appsink
    }
}

impl<'a> From<&'a gst_app::AppSink> for StreamProducer {
    fn from(appsink: &'a gst_app::AppSink) -> Self {
        let consumers = Arc::new(Mutex::new(StreamConsumers {
            current_latency: None,
            latency_updated: false,
            consumers: HashMap::new(),
            discard: true,
        }));

        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(glib::clone!(@strong consumers => move |appsink| {
                    let mut consumers = consumers.lock().unwrap();

                    let sample = match appsink.pull_sample() {
                        Ok(sample) => sample,
                        Err(_err) => {
                            gst::debug!(CAT, obj: appsink, "Failed to pull sample");
                            return Err(gst::FlowError::Flushing);
                        }
                    };

                    if consumers.discard {
                        return Ok(gst::FlowSuccess::Ok);
                    }

                    let (is_discont, is_keyframe) = if let Some(buf) = sample.buffer() {
                        let flags = buf.flags();

                        (flags.contains(gst::BufferFlags::DISCONT),
                         !flags.contains(gst::BufferFlags::DELTA_UNIT))
                    } else {
                        (false, true)
                    };

                    gst::trace!(CAT, obj: appsink, "processing sample");

                    let latency = consumers.current_latency;
                    let latency_updated = mem::replace(&mut consumers.latency_updated, false);
                    let mut requested_keyframe = false;

                    let current_consumers = consumers
                        .consumers
                        .values()
                        .filter_map(|consumer| {
                            if let Some(latency) = latency {
                                if consumer.forwarded_latency
                                    .compare_exchange(
                                        false,
                                        true,
                                        atomic::Ordering::SeqCst,
                                        atomic::Ordering::SeqCst,
                                    )
                                    .is_ok()
                                    || latency_updated
                                {
                                    consumer.appsrc.set_latency(latency, gst::ClockTime::NONE);
                                }
                            }

                            if is_discont {
                                // Whenever we have a discontinuity, we need a new keyframe
                                consumer.needs_keyframe.store(
                                    true,
                                    atomic::Ordering::SeqCst,
                                );
                            }

                            if !is_keyframe && consumer.needs_keyframe.load(atomic::Ordering::SeqCst)
                            {
                                // If we need a keyframe (and this one isn't) request a keyframe upstream
                                if !requested_keyframe {
                                    gst::debug!(CAT, obj: appsink, "Requesting keyframe for first buffer");

                                    appsink.send_event(
                                        gst_video::UpstreamForceKeyUnitEvent::builder()
                                            .all_headers(true)
                                            .build(),
                                    );
                                    requested_keyframe = true;
                                }

                                gst::debug!(CAT, obj: appsink, "Ignoring frame for {} while waiting for a keyframe",
                                    consumer.appsrc.name());
                                None
                            } else {
                                consumer.needs_keyframe.store(false, atomic::Ordering::SeqCst);

                                Some(consumer.appsrc.clone())
                            }
                        })
                        .collect::<Vec<_>>();
                    drop(consumers);

                    for consumer in current_consumers {
                        if let Err(err) = consumer.push_sample(&sample) {
                            gst::warning!(CAT, obj: appsink, "Failed to push sample: {}", err);
                        }
                    }

                    Ok(gst::FlowSuccess::Ok)
                }))
                .eos(glib::clone!(@strong consumers => move |_| {
                    let current_consumers = consumers
                        .lock()
                        .unwrap()
                        .consumers
                        .values()
                        .map(|c| c.appsrc.clone())
                        .collect::<Vec<_>>();

                    for consumer in current_consumers {
                        let _ = consumer.end_of_stream();
                    }
                }))
                .build(),
        );

        let sinkpad = appsink.static_pad("sink").unwrap();
        sinkpad.add_probe(
            gst::PadProbeType::EVENT_UPSTREAM,
            glib::clone!(@strong consumers => move |_pad, info| {
                if let Some(gst::PadProbeData::Event(ref ev)) = info.data {
                    if let gst::EventView::Latency(ev) = ev.view() {
                        let latency = ev.latency();
                        let mut consumers = consumers.lock().unwrap();
                        consumers.current_latency = Some(latency);
                        consumers.latency_updated = true;
                    }
                }
                gst::PadProbeReturn::Ok
            }),
        );

        StreamProducer {
            appsink: appsink.clone(),
            consumers,
        }
    }
}

/// Wrapper around a HashMap of consumers, exists for thread safety
/// and also protects some of the producer state
#[derive(Debug)]
struct StreamConsumers {
    /// The currently-observed latency
    current_latency: Option<gst::ClockTime>,
    /// Whether the consumers' appsrc latency needs updating
    latency_updated: bool,
    /// The consumers, AppSrc pointer value -> consumer
    consumers: HashMap<gst_app::AppSrc, StreamConsumer>,
    /// Whether appsrc samples should be forwarded to consumers yet
    discard: bool,
}

/// Wrapper around a consumer's `appsrc`
#[derive(Debug)]
struct StreamConsumer {
    /// The GStreamer `appsrc` of the consumer
    appsrc: gst_app::AppSrc,
    /// The id of a pad probe that intercepts force-key-unit events
    fku_probe_id: Option<gst::PadProbeId>,
    /// Whether an initial latency was forwarded to the `appsrc`
    forwarded_latency: atomic::AtomicBool,
    /// Whether a first buffer has made it through, used to determine
    /// whether a new key unit should be requested. Only useful for encoded
    /// streams.
    needs_keyframe: atomic::AtomicBool,
}

impl StreamConsumer {
    /// Create a new consumer
    fn new(appsrc: &gst_app::AppSrc, fku_probe_id: gst::PadProbeId) -> Self {
        appsrc.set_callbacks(
            gst_app::AppSrcCallbacks::builder()
                .enough_data(move |appsrc| {
                    gst::debug!(
                        CAT,
                        obj: appsrc,
                        "consumer {} ({:?}) is not consuming fast enough, old samples are getting dropped",
                        appsrc.name(),
                        appsrc,
                    );
                })
                .build(),
        );

        StreamConsumer {
            appsrc: appsrc.clone(),
            fku_probe_id: Some(fku_probe_id),
            forwarded_latency: atomic::AtomicBool::new(false),
            needs_keyframe: atomic::AtomicBool::new(true),
        }
    }
}

impl Drop for StreamConsumer {
    fn drop(&mut self) {
        if let Some(fku_probe_id) = self.fku_probe_id.take() {
            let srcpad = self.appsrc.static_pad("src").unwrap();
            srcpad.remove_probe(fku_probe_id);
        }
    }
}

impl PartialEq for StreamConsumer {
    fn eq(&self, other: &Self) -> bool {
        self.appsrc.eq(&other.appsrc)
    }
}

impl Eq for StreamConsumer {}

impl std::hash::Hash for StreamConsumer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.appsrc, state);
    }
}

impl std::borrow::Borrow<gst_app::AppSrc> for StreamConsumer {
    fn borrow(&self) -> &gst_app::AppSrc {
        &self.appsrc
    }
}

#[cfg(test)]
mod tests {
    use std::{
        str::FromStr,
        sync::{Arc, Mutex},
    };

    use futures::channel::mpsc;
    use futures::{channel::mpsc::Receiver, SinkExt, StreamExt};
    use gst::prelude::*;

    use crate::StreamProducer;

    fn create_producer() -> (
        gst::Pipeline,
        gst_app::AppSrc,
        gst_app::AppSink,
        StreamProducer,
    ) {
        let producer_pipe =
            gst::parse_launch("appsrc name=producer_src ! appsink name=producer_sink")
                .unwrap()
                .downcast::<gst::Pipeline>()
                .unwrap();
        let producer_sink = producer_pipe
            .by_name("producer_sink")
            .unwrap()
            .downcast::<gst_app::AppSink>()
            .unwrap();

        (
            producer_pipe.clone(),
            producer_pipe
                .by_name("producer_src")
                .unwrap()
                .downcast::<gst_app::AppSrc>()
                .unwrap(),
            producer_sink.clone(),
            StreamProducer::from(&producer_sink),
        )
    }

    struct Consumer {
        pipeline: gst::Pipeline,
        src: gst_app::AppSrc,
        sink: gst_app::AppSink,
        receiver: Mutex<Receiver<gst::Sample>>,
        connected: Mutex<bool>,
    }

    impl Consumer {
        fn new(id: &str) -> Self {
            let pipeline = gst::parse_launch(&format!("appsrc name={} ! appsink name=sink", id))
                .unwrap()
                .downcast::<gst::Pipeline>()
                .unwrap();

            let (sender, receiver) = mpsc::channel::<gst::Sample>(1000);
            let sender = Arc::new(Mutex::new(sender));
            let sink = pipeline
                .by_name("sink")
                .unwrap()
                .downcast::<gst_app::AppSink>()
                .unwrap();

            sink.set_callbacks(
                gst_app::AppSinkCallbacks::builder()
                    // Add a handler to the "new-sample" signal.
                    .new_sample(move |appsink| {
                        // Pull the sample in question out of the appsink's buffer.
                        let sender_clone = sender.clone();
                        futures::executor::block_on(
                            sender_clone
                                .lock()
                                .unwrap()
                                .send(appsink.pull_sample().unwrap()),
                        )
                        .unwrap();

                        Ok(gst::FlowSuccess::Ok)
                    })
                    .build(),
            );

            Self {
                pipeline: pipeline.clone(),
                src: pipeline
                    .by_name(id)
                    .unwrap()
                    .downcast::<gst_app::AppSrc>()
                    .unwrap(),
                sink,
                receiver: Mutex::new(receiver),
                connected: Mutex::new(false),
            }
        }

        fn connect(&self, producer: &StreamProducer) {
            {
                let mut connected = self.connected.lock().unwrap();
                *connected = true;
            }

            producer.add_consumer(&self.src);
        }

        fn disconnect(&self, producer: &StreamProducer) {
            {
                let mut connected = self.connected.lock().unwrap();
                *connected = false;
            }

            producer.remove_consumer(&self.src);
        }
    }

    #[test]
    fn simple() {
        gst::init().unwrap();

        let (producer_pipe, producer_src, _producer_sink, producer) = create_producer();
        producer.forward();
        producer_pipe
            .set_state(gst::State::Playing)
            .expect("Couldn't set producer pipeline state");

        let mut consumers: Vec<Consumer> = Vec::new();
        let consumer = Consumer::new("consumer1");
        consumer.connect(&producer);
        consumer
            .pipeline
            .set_state(gst::State::Playing)
            .expect("Couldn't set producer pipeline state");
        consumers.push(consumer);

        let consumer = Consumer::new("consumer2");
        consumer.connect(&producer);
        consumer
            .pipeline
            .set_state(gst::State::Playing)
            .expect("Couldn't set producer pipeline state");
        consumers.push(consumer);

        for i in 0..10 {
            let caps = gst::Caps::from_str(&format!("test,n={}", i)).unwrap();
            producer_src.set_caps(Some(&caps));
            producer_src.push_buffer(gst::Buffer::new()).unwrap();

            for consumer in &consumers {
                if *consumer.connected.lock().unwrap() {
                    let sample =
                        futures::executor::block_on(consumer.receiver.lock().unwrap().next())
                            .expect("Received an empty buffer?");
                    sample.buffer().expect("No buffer on the sample?");
                    assert_eq!(sample.caps(), Some(caps.as_ref()));
                } else {
                    debug_assert!(
                        consumer
                            .sink
                            .try_pull_sample(gst::ClockTime::from_nseconds(0))
                            .is_none(),
                        "Disconnected consumer got a new sample?!"
                    );
                }
            }

            if i == 5 {
                consumers.get(0).unwrap().disconnect(&producer);
            }
        }
    }
}
