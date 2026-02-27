This crate provides a bridge between gstreamer and the tracing ecosystem.

The goal is to allow Rust applications utilizing GStreamer to better integrate into application
that otherwise use the [`tracing`] crate for their observability needs.

# Examples

## Events

To output `gstreamer` log messages as [`tracing`] events, call the [`integrate_events`]
function. Calling it before the call to any other `gstreamer` call (especially before the
`gstreamer::init`) is most likely to correctly forward all of the messages:

```rust
// Set up the tracing subscriber.
//
// e.g. tracing_subscriber::fmt::init();

tracing_gstreamer::integrate_events();
gstreamer::log::remove_default_log_function();
gstreamer::init();
```

Keep in mind that both `GST_DEBUG` and tracing filters are in effect. The `gstreamer` side of
filters can be relaxed from code via:

```
gstreamer::log::set_default_threshold(gstreamer::DebugLevel::Memdump);
```

Similarly you can use `tracing` APIs to adjust the filters on the `tracing` side.

## Spans

To provide `tracing` with more contextual information for some of the events, you can also enable
support for generating spans via `gstreamer`'s own [tracing infrastructure][gsttracing].

This functionality can be enabled by calling the [`integrate_spans`] function. It must be called
after `gstreamer::init`.

```rust
gstreamer::init();
tracing_gstreamer::integrate_spans();
```

## Subscriber showcase

This section demonstrates the results obtained with different kinds of subscribers.

### `tracing_subscriber::fmt`

This subscriber is a great replacement for the built-in gstreamer log handler. Here's an example of
what the output might look like when using this subscriber:

```text
$ env RUST_LOG=info cargo run --example videoenc
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/examples/videoenc`
Jan 01 00:00:00.000  INFO gstreamer::GST_INIT: Initializing GStreamer Core Library version 1.18.4
<snip>
Jan 01 00:00:00.000  INFO gstreamer::GST_INIT: initialized GStreamer successfully
Jan 01 00:00:00.000  INFO gstreamer::GST_PIPELINE: parsing pipeline description '
        videotestsrc num-buffers=120
        ! vp9enc
        ! webmmux name=mux
        ! fakesink sync=false

        audiotestsrc num-buffers=120
        ! opusenc
        ! mux.
    '
```

Certain messages may also provide more information than the built-in logger. While builtin logger
does present you with the type and address of the object being logged, `tracing-gstreamer` may
provide more readily useful information such as the element name:

```text
Jan 01 00:00:00.000  INFO gstreamer::GST_STATES: completed state change to READY gobject.address=94331150660528 gobject.type="GstAudioTestSrc" gstobject.name="audiotestsrc0" gstelement.state="ready" gstelement.pending_state="void-pending"
```

or provide additional context via spans, which may help to figure out which element is logging the
message when there is no other way to tell this otherwise, such as in this example:

```text
Jan 01 00:00:00.000  INFO pad_push{gstpad.state={NEED_PARENT} gstpad.parent.name="audiotestsrc0"}: gstreamer::structure: Expected field 'channel-mask' in structure: audio/x-raw, rate=(int)48000, channels=(int)1, format=(string)S16LE, layout=(string)interleaved;
```

### `tracing-tracy`

Tracy is a profiler focused primarily on game development workloads, but works fairly well for
general purpose code as well. Tracy features a sampling profiler, but works best with applications
that have manually instrumented points of interest.  `tracing` is a great source of such manual
instrumentation points and `tracing-tracy` is the bridge between the two. The following video
demonstrates the `videoenc` example from this repository adapted to utilize the `tracing-tracy`
subscriber.

<video src="https://user-images.githubusercontent.com/679122/131253926-63761e43-a804-44f4-ad8a-8b87cd274cf8.mp4" controls></video>

In this video there are a couple of highlights

* We can quickly see the amount of concurrency our pipeline enables (2 threads; perhaps adding some
  `queue`s would help?)
* Overall thread utilization (low for the audio portion and high for the video portion);
* Investigate the performance of the specific elements and quickly find out why some of them are
  slow. For example the `opusenc0` element sometimes takes an unusually long time because the
  downstream muxer already has a buffer queued at the time.

Similar results can be achieved with some other subscribers as well.

[gsttracing]: https://gstreamer.freedesktop.org/documentation/additional/design/tracing.html
[`tracing`]: tracing_core


### The GStreamer tracers

Several GStreamer tracers are also available so the integration of GStreamer
tracing and logging into the Rust tracing system is possible without modifying
the application that uses GStreamer.

For GStreamer to find the tracer you need to ensure that the
`libtracing_gstreamer.so` is installed as GStreamer plugin (you can also set
`GST_PLUGIN_PATH` for example with `export
GST_PLUGIN_PATH=$PWD/target/debug/:$GST_PLUGIN_PATH`).

Currently 2 tracers are available:

* `chrometracing`: This tracer will output the tracing events in the Chrome json
  tracing format. This will create a `trace-XXX.json` in the current directory
  that can be opened in [perfetto](https://ui.perfetto.dev/). This is useful to
  analyze GStreamer performance in a graphical way.

* `fmttracing`: Use the `tracing-subscriber::fmt` subscriber to format the
  tracing events. This is useful to get a human readable output. To actually get
  output you also need to set `RUST_LOG=<loglevel>`

> Note that only *one* of those tracer can be used at a time, and the application
> itself should never activate any other tracing_subscriber.

#### Tracers parameters

The tracer has the following parameters:

* `log-level`: String in the same form as the GST_DEBUG environment variable
  defining which GStreamer log level and category should be logged into the
  tracing system. This implies that the usual GStreamer log system will be
  disabled and the rust one will be used instead.

#### Examples

You can, for example, profile a GStreamer pipeline using [`gst-launch-1.0`]
with the following command:

##### Using the `chrometracing` tracer


``` sh
# Builds the tracer plugin and make sure GStreamer finds it.
# Enable the tracer with the chrome tracing output and activating GStreamer info logs
cargo build --features "tracing-chrome" && \
  GST_PLUGIN_PATH=$PWD/target/debug/:$GST_PLUGIN_PATH \
  GST_TRACERS="chrometracing(log-level=4)" \
  gst-launch-1.0 playbin3 uri="https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm"
```

A new `trace-XXX.json` file will be created in the current directory. You can
then open it in [perfetto](https://ui.perfetto.dev/) to analyze them.


##### Using the `fmttracing` tracer


``` sh
# Builds the tracer plugin and make sure GStreamer finds it.
# Enable the tracer with the fmt tracing output and activating GStreamer info logs
# Logs will be output on stderr in the `tracing-subscriber::fmt` format
cargo build --features "tracing-subscriber" && \
  GST_PLUGIN_PATH=$PWD/target/debug/:$GST_PLUGIN_PATH \
  RUST_LOG=debug GST_TRACERS="fmttracing(log-level=4)" \
  gst-launch-1.0 playbin3 uri="https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm"
```
