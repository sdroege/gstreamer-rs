<!-- file * -->
<!-- struct Harness -->
`Harness` is meant to make writing unit test for GStreamer much easier.
It can be thought of as a way of treating a `gst::Element` as a black box,
deterministically feeding it data, and controlling what data it outputs.

The basic structure of `Harness` is two "floating" `GstPads` that connect
to the harnessed `gst::Element` src and sink `GstPads` like so:


```C
  #include <gst/gst.h>
  #include <gst/check/gstharness.h>
  GstHarness *h;
  GstBuffer *in_buf;
  GstBuffer *out_buf;

  // attach the harness to the src and sink pad of GstQueue
  h = gst_harness_new ("queue");

  // we must specify a caps before pushing buffers
  gst_harness_set_src_caps_str (h, "mycaps");

  // create a buffer of size 42
  in_buf = gst_harness_create_buffer (h, 42);

  // push the buffer into the queue
  gst_harness_push (h, in_buf);

  // pull the buffer from the queue
  out_buf = gst_harness_pull (h);

  // validate the buffer in is the same as buffer out
  fail_unless (in_buf == out_buf);

  // cleanup
  gst_buffer_unref (out_buf);
  gst_harness_teardown (h);

  ]|

Another main feature of the #GstHarness is its integration with the
#GstTestClock. Operating the #GstTestClock can be very challenging, but
#GstHarness simplifies some of the most desired actions a lot, like wanting
to manually advance the clock while at the same time releasing a #GstClockID
that is waiting, with functions like gst_harness_crank_single_clock_wait().

#GstHarness also supports sub-harnesses, as a way of generating and
validating data. A sub-harness is another #GstHarness that is managed by
the "parent" harness, and can either be created by using the standard
gst_harness_new type functions directly on the (GstHarness *)->src_harness,
or using the much more convenient gst_harness_add_src() or
gst_harness_add_sink_parse(). If you have a decoder-element you want to test,
(like vp8dec) it can be very useful to add a src-harness with both a
src-element (videotestsrc) and an encoder (vp8enc) to feed the decoder data
with different configurations, by simply doing:

|[<!-- language="C" -->
  GstHarness * h = gst_harness_new (h, "vp8dec");
  gst_harness_add_src_parse (h, "videotestsrc is-live=1 ! vp8enc", TRUE);
```

and then feeding it data with:


```C
gst_harness_push_from_src (h);
```
<!-- impl Harness::fn add_element_full -->
Adds a `gst::Element` to an empty `Harness`

MT safe.
## `element`
a `gst::Element` to add to the harness (transfer none)
## `hsrc`
a `gst::StaticPadTemplate` describing the harness srcpad.
`None` will not create a harness srcpad.
## `element_sinkpad_name`
a `gchar` with the name of the element
sinkpad that is then linked to the harness srcpad. Can be a static or request
or a sometimes pad that has been added. `None` will not get/request a sinkpad
from the element. (Like if the element is a src.)
## `hsink`
a `gst::StaticPadTemplate` describing the harness sinkpad.
`None` will not create a harness sinkpad.
## `element_srcpad_name`
a `gchar` with the name of the element
srcpad that is then linked to the harness sinkpad, similar to the
`element_sinkpad_name`.
<!-- impl Harness::fn add_element_sink_pad -->
Links the specified `gst::Pad` the `Harness` srcpad.

MT safe.
## `sinkpad`
a `gst::Pad` to link to the harness srcpad
<!-- impl Harness::fn add_element_src_pad -->
Links the specified `gst::Pad` the `Harness` sinkpad. This can be useful if
perhaps the srcpad did not exist at the time of creating the harness,
like a demuxer that provides a sometimes-pad after receiving data.

MT safe.
## `srcpad`
a `gst::Pad` to link to the harness sinkpad
<!-- impl Harness::fn add_parse -->
Parses the `launchline` and puts that in a `gst::Bin`,
and then attches the supplied `Harness` to the bin.

MT safe.
## `launchline`
a `gchar` describing a gst-launch type line
<!-- impl Harness::fn add_probe -->
A convenience function to allows you to call gst_pad_add_probe on a
`gst::Pad` of a `gst::Element` that are residing inside the `Harness`,
by using normal gst_pad_add_probe syntax

MT safe.
## `element_name`
a `gchar` with a `gst::ElementFactory` name
## `pad_name`
a `gchar` with the name of the pad to attach the probe to
## `mask`
a `gst::PadProbeType` (see gst_pad_add_probe)
## `callback`
a `GstPadProbeCallback` (see gst_pad_add_probe)
## `user_data`
a `gpointer` (see gst_pad_add_probe)
## `destroy_data`
a `GDestroyNotify` (see gst_pad_add_probe)
<!-- impl Harness::fn add_propose_allocation_meta -->
Add api with params as one of the supported metadata API to propose when
receiving an allocation query.

MT safe.

Feature: `v1_16`

## `api`
a metadata API
## `params`
API specific parameters
<!-- impl Harness::fn add_sink -->
Similar to gst_harness_add_sink_harness, this is a convenience to
directly create a sink-harness using the `sink_element_name` name specified.

MT safe.
## `sink_element_name`
a `gchar` with the name of a `gst::Element`
<!-- impl Harness::fn add_sink_harness -->
Similar to gst_harness_add_src, this allows you to send the data coming out
of your harnessed `gst::Element` to a sink-element, allowing to test different
responses the element output might create in sink elements. An example might
be an existing sink providing some analytical data on the input it receives that
can be useful to your testing. If the goal is to test a sink-element itself,
this is better achieved using gst_harness_new directly on the sink.

If a sink-harness already exists it will be replaced.

MT safe.
## `sink_harness`
a `Harness` to be added as a sink-harness.
<!-- impl Harness::fn add_sink_parse -->
Similar to gst_harness_add_sink, this allows you to specify a launch-line
instead of just an element name. See gst_harness_add_src_parse for details.

MT safe.
## `launchline`
a `gchar` with the name of a `gst::Element`
<!-- impl Harness::fn add_src -->
Similar to gst_harness_add_src_harness, this is a convenience to
directly create a src-harness using the `src_element_name` name specified.

MT safe.
## `src_element_name`
a `gchar` with the name of a `gst::Element`
## `has_clock_wait`
a `gboolean` specifying if the `gst::Element` uses
gst_clock_wait_id internally.
<!-- impl Harness::fn add_src_harness -->
A src-harness is a great way of providing the `Harness` with data.
By adding a src-type `gst::Element`, it is then easy to use functions like
gst_harness_push_from_src or gst_harness_src_crank_and_push_many
to provide your harnessed element with input. The `has_clock_wait` variable
is a great way to control you src-element with, in that you can have it
produce a buffer for you by simply cranking the clock, and not have it
spin out of control producing buffers as fast as possible.

If a src-harness already exists it will be replaced.

MT safe.
## `src_harness`
a `Harness` to be added as a src-harness.
## `has_clock_wait`
a `gboolean` specifying if the `gst::Element` uses
gst_clock_wait_id internally.
<!-- impl Harness::fn add_src_parse -->
Similar to gst_harness_add_src, this allows you to specify a launch-line,
which can be useful for both having more then one `gst::Element` acting as your
src (Like a src producing raw buffers, and then an encoder, providing encoded
data), but also by allowing you to set properties like "is-live" directly on
the elements.

MT safe.
## `launchline`
a `gchar` describing a gst-launch type line
## `has_clock_wait`
a `gboolean` specifying if the `gst::Element` uses
gst_clock_wait_id internally.
<!-- impl Harness::fn buffers_in_queue -->
The number of `GstBuffers` currently in the `Harness` sinkpad `glib::AsyncQueue`

MT safe.

# Returns

a `guint` number of buffers in the queue
<!-- impl Harness::fn buffers_received -->
The total number of `GstBuffers` that has arrived on the `Harness` sinkpad.
This number includes buffers that have been dropped as well as buffers
that have already been pulled out.

MT safe.

# Returns

a `guint` number of buffers received
<!-- impl Harness::fn crank_multiple_clock_waits -->
Similar to `Harness::crank_single_clock_wait`, this is the function to use
if your harnessed element(s) are using more then one gst_clock_id_wait.
Failing to do so can (and will) make it racy which `gst::ClockID` you actually
are releasing, where as this function will process all the waits at the
same time, ensuring that one thread can't register another wait before
both are released.

MT safe.
## `waits`
a `guint` describing the number of `GstClockIDs` to crank

# Returns

a `gboolean` `true` if the "crank" was successful, `false` if not.
<!-- impl Harness::fn crank_single_clock_wait -->
A "crank" consists of three steps:
1: Wait for a `gst::ClockID` to be registered with the `TestClock`.
2: Advance the `TestClock` to the time the `gst::ClockID` is waiting for.
3: Release the `gst::ClockID` wait.
Together, this provides an easy way to not have to think about the details
around clocks and time, but still being able to write deterministic tests
that are dependent on this. A "crank" can be though of as the notion of
manually driving the clock forward to its next logical step.

MT safe.

# Returns

a `gboolean` `true` if the "crank" was successful, `false` if not.
<!-- impl Harness::fn create_buffer -->
Allocates a buffer using a `gst::BufferPool` if present, or else using the
configured `gst::Allocator` and `gst::AllocationParams`

MT safe.
## `size`
a `gsize` specifying the size of the buffer

# Returns

a `gst::Buffer` of size `size`
<!-- impl Harness::fn dump_to_file -->
Allows you to dump the `GstBuffers` the `Harness` sinkpad `glib::AsyncQueue`
to a file.

MT safe.
## `filename`
a `gchar` with a the name of a file
<!-- impl Harness::fn events_in_queue -->
The number of `GstEvents` currently in the `Harness` sinkpad `glib::AsyncQueue`

MT safe.

# Returns

a `guint` number of events in the queue
<!-- impl Harness::fn events_received -->
The total number of `GstEvents` that has arrived on the `Harness` sinkpad
This number includes events handled by the harness as well as events
that have already been pulled out.

MT safe.

# Returns

a `guint` number of events received
<!-- impl Harness::fn find_element -->
Most useful in conjunction with gst_harness_new_parse, this will scan the
`GstElements` inside the `Harness`, and check if any of them matches
`element_name`. Typical usecase being that you need to access one of the
harnessed elements for properties and/or signals.

MT safe.
## `element_name`
a `gchar` with a `gst::ElementFactory` name

# Returns

a `gst::Element` or `None` if not found
<!-- impl Harness::fn get -->
A convenience function to allows you to call g_object_get on a `gst::Element`
that are residing inside the `Harness`, by using normal g_object_get
syntax.

MT safe.
## `element_name`
a `gchar` with a `gst::ElementFactory` name
## `first_property_name`
a `gchar` with the first property name
<!-- impl Harness::fn get_allocator -->
Gets the `allocator` and its `params` that has been decided to use after an
allocation query.

MT safe.
## `allocator`
the `gst::Allocator` used
## `params`
the `gst::AllocationParams` of
 `allocator`
<!-- impl Harness::fn get_last_pushed_timestamp -->
Get the timestamp of the last `gst::Buffer` pushed on the `Harness` srcpad,
typically with gst_harness_push or gst_harness_push_from_src.

MT safe.

# Returns

a `gst::ClockTime` with the timestamp or `GST_CLOCK_TIME_NONE` if no
`gst::Buffer` has been pushed on the `Harness` srcpad
<!-- impl Harness::fn get_testclock -->
Get the `TestClock`. Useful if specific operations on the testclock is
needed.

MT safe.

# Returns

a `TestClock`, or `None` if the testclock is not
present.
<!-- impl Harness::fn play -->
This will set the harnessed `gst::Element` to `gst::State::Playing`.
`GstElements` without a sink-`gst::Pad` and with the `gst::ElementFlags::Source`
flag set is considered a src `gst::Element`
Non-src `GstElements` (like sinks and filters) are automatically set to
playing by the `Harness`, but src `GstElements` are not to avoid them
starting to produce buffers.
Hence, for src `gst::Element` you must call `Harness::play` explicitly.

MT safe.
<!-- impl Harness::fn pull -->
Pulls a `gst::Buffer` from the `glib::AsyncQueue` on the `Harness` sinkpad. The pull
will timeout in 60 seconds. This is the standard way of getting a buffer
from a harnessed `gst::Element`.

MT safe.

# Returns

a `gst::Buffer` or `None` if timed out.
<!-- impl Harness::fn pull_event -->
Pulls an `gst::Event` from the `glib::AsyncQueue` on the `Harness` sinkpad.
Timeouts after 60 seconds similar to gst_harness_pull.

MT safe.

# Returns

a `gst::Event` or `None` if timed out.
<!-- impl Harness::fn pull_upstream_event -->
Pulls an `gst::Event` from the `glib::AsyncQueue` on the `Harness` srcpad.
Timeouts after 60 seconds similar to gst_harness_pull.

MT safe.

# Returns

a `gst::Event` or `None` if timed out.
<!-- impl Harness::fn push -->
Pushes a `gst::Buffer` on the `Harness` srcpad. The standard way of
interacting with an harnessed element.

MT safe.
## `buffer`
a `gst::Buffer` to push

# Returns

a `gst::FlowReturn` with the result from the push
<!-- impl Harness::fn push_and_pull -->
Basically a gst_harness_push and a gst_harness_pull in one line. Reflects
the fact that you often want to do exactly this in your test: Push one buffer
in, and inspect the outcome.

MT safe.
## `buffer`
a `gst::Buffer` to push

# Returns

a `gst::Buffer` or `None` if timed out.
<!-- impl Harness::fn push_event -->
Pushes an `gst::Event` on the `Harness` srcpad.

MT safe.
## `event`
a `gst::Event` to push

# Returns

a `gboolean` with the result from the push
<!-- impl Harness::fn push_from_src -->
Transfer data from the src-`Harness` to the main-`Harness`. It consists
of 4 steps:
1: Make sure the src is started. (see: gst_harness_play)
2: Crank the clock (see: gst_harness_crank_single_clock_wait)
3: Pull a `gst::Buffer` from the src-`Harness` (see: gst_harness_pull)
4: Push the same `gst::Buffer` into the main-`Harness` (see: gst_harness_push)

MT safe.

# Returns

a `gst::FlowReturn` with the result of the push
<!-- impl Harness::fn push_to_sink -->
Transfer one `gst::Buffer` from the main-`Harness` to the sink-`Harness`.
See gst_harness_push_from_src for details.

MT safe.

# Returns

a `gst::FlowReturn` with the result of the push
<!-- impl Harness::fn push_upstream_event -->
Pushes an `gst::Event` on the `Harness` sinkpad.

MT safe.
## `event`
a `gst::Event` to push

# Returns

a `gboolean` with the result from the push
<!-- impl Harness::fn query_latency -->
Get the min latency reported by any harnessed `gst::Element`.

MT safe.

# Returns

a `gst::ClockTime` with min latency
<!-- impl Harness::fn set -->
A convenience function to allows you to call g_object_set on a `gst::Element`
that are residing inside the `Harness`, by using normal g_object_set
syntax.

MT safe.
## `element_name`
a `gchar` with a `gst::ElementFactory` name
## `first_property_name`
a `gchar` with the first property name
<!-- impl Harness::fn set_blocking_push_mode -->
Setting this will make the harness block in the chain-function, and
then release when `Harness::pull` or `Harness::try_pull` is called.
Can be useful when wanting to control a src-element that is not implementing
`gst::Clock::id_wait` so it can't be controlled by the `TestClock`, since
it otherwise would produce buffers as fast as possible.

MT safe.
<!-- impl Harness::fn set_caps -->
Sets the `Harness` srcpad and sinkpad caps.

MT safe.
## `in_`
a `gst::Caps` to set on the harness srcpad
## `out`
a `gst::Caps` to set on the harness sinkpad
<!-- impl Harness::fn set_caps_str -->
Sets the `Harness` srcpad and sinkpad caps using strings.

MT safe.
## `in_`
a `gchar` describing a `gst::Caps` to set on the harness srcpad
## `out`
a `gchar` describing a `gst::Caps` to set on the harness sinkpad
<!-- impl Harness::fn set_drop_buffers -->
When set to `true`, instead of placing the buffers arriving from the harnessed
`gst::Element` inside the sinkpads `glib::AsyncQueue`, they are instead unreffed.

MT safe.
## `drop_buffers`
a `gboolean` specifying to drop outgoing buffers or not
<!-- impl Harness::fn set_forwarding -->
As a convenience, a src-harness will forward `gst::EventType::StreamStart`,
`gst::EventType::Caps` and `gst::EventType::Segment` to the main-harness if forwarding
is enabled, and forward any sticky-events from the main-harness to
the sink-harness. It will also forward the `gst::QueryType::Allocation`.

If forwarding is disabled, the user will have to either manually push
these events from the src-harness using `Harness::src_push_event`, or
create and push them manually. While this will allow full control and
inspection of these events, for the most cases having forwarding enabled
will be sufficient when writing a test where the src-harness' main function
is providing data for the main-harness.

Forwarding is enabled by default.

MT safe.
## `forwarding`
a `gboolean` to enable/disable forwarding
<!-- impl Harness::fn set_propose_allocator -->
Sets the `allocator` and `params` to propose when receiving an allocation
query.

MT safe.
## `allocator`
a `gst::Allocator`
## `params`
a `gst::AllocationParams`
<!-- impl Harness::fn set_sink_caps -->
Sets the `Harness` sinkpad caps.

MT safe.
## `caps`
a `gst::Caps` to set on the harness sinkpad
<!-- impl Harness::fn set_sink_caps_str -->
Sets the `Harness` sinkpad caps using a string.

MT safe.
## `str`
a `gchar` describing a `gst::Caps` to set on the harness sinkpad
<!-- impl Harness::fn set_src_caps -->
Sets the `Harness` srcpad caps. This must be done before any buffers
can legally be pushed from the harness to the element.

MT safe.
## `caps`
a `gst::Caps` to set on the harness srcpad
<!-- impl Harness::fn set_src_caps_str -->
Sets the `Harness` srcpad caps using a string. This must be done before
any buffers can legally be pushed from the harness to the element.

MT safe.
## `str`
a `gchar` describing a `gst::Caps` to set on the harness srcpad
<!-- impl Harness::fn set_time -->
Advance the `TestClock` to a specific time.

MT safe.
## `time`
a `gst::ClockTime` to advance the clock to

# Returns

a `gboolean` `true` if the time could be set. `false` if not.
<!-- impl Harness::fn set_upstream_latency -->
Sets the min latency reported by `Harness` when receiving a latency-query
## `latency`
a `gst::ClockTime` specifying the latency
<!-- impl Harness::fn sink_push_many -->
Convenience that calls gst_harness_push_to_sink `pushes` number of times.
Will abort the pushing if any one push fails.

MT safe.
## `pushes`
a `gint` with the number of calls to gst_harness_push_to_sink

# Returns

a `gst::FlowReturn` with the result of the push
<!-- impl Harness::fn src_crank_and_push_many -->
Transfer data from the src-`Harness` to the main-`Harness`. Similar to
gst_harness_push_from_src, this variant allows you to specify how many cranks
and how many pushes to perform. This can be useful for both moving a lot
of data at the same time, as well as cases when one crank does not equal one
buffer to push and v.v.

MT safe.
## `cranks`
a `gint` with the number of calls to gst_harness_crank_single_clock_wait
## `pushes`
a `gint` with the number of calls to gst_harness_push

# Returns

a `gst::FlowReturn` with the result of the push
<!-- impl Harness::fn src_push_event -->
Similar to what gst_harness_src_push does with `GstBuffers`, this transfers
a `gst::Event` from the src-`Harness` to the main-`Harness`. Note that
some `GstEvents` are being transferred automagically. Look at sink_forward_pad
for details.

MT safe.

# Returns

a `gboolean` with the result of the push
<!-- impl Harness::fn stress_custom_start -->
Start a custom stress-thread that will call your `callback` for every
iteration allowing you to do something nasty.

MT safe.
## `init`
a `GFunc` that is called initially and only once
## `callback`
a `GFunc` that is called as often as possible
## `data`
a `gpointer` with custom data to pass to the `callback` function
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each call to the `callback`

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_property_start_full -->
Call g_object_set with `name` and `value` in intervals of `sleep` microseconds

MT safe.
## `name`
a `gchar` specifying a property name
## `value`
a `gobject::Value` to set the property to
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each g_object_set with `name` and `value`

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_push_buffer_start_full -->
Push a `gst::Buffer` in intervals of `sleep` microseconds.

MT safe.
## `caps`
a `gst::Caps` for the `gst::Buffer`
## `segment`
a `gst::Segment`
## `buf`
a `gst::Buffer` to push
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each call to gst_pad_push

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_push_buffer_with_cb_start_full -->
Push a `gst::Buffer` returned by `func` in intervals of `sleep` microseconds.

MT safe.
## `caps`
a `gst::Caps` for the `gst::Buffer`
## `segment`
a `gst::Segment`
## `func`
a `GstHarnessPrepareBufferFunc` function called before every iteration
to prepare / create a `gst::Buffer` for pushing
## `data`
a `gpointer` with data to the `GstHarnessPrepareBufferFunc` function
## `notify`
a `GDestroyNotify` that is called when thread is stopped
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each call to gst_pad_push

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_push_event_start_full -->
Push the `event` onto the harnessed `gst::Element` sinkpad in intervals of
`sleep` microseconds

MT safe.
## `event`
a `gst::Event` to push
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each gst_event_push with `event`

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_push_event_with_cb_start_full -->
Push a `gst::Event` returned by `func` onto the harnessed `gst::Element` sinkpad
in intervals of `sleep` microseconds.

MT safe.
## `func`
a `GstHarnessPrepareEventFunc` function called before every iteration
to prepare / create a `gst::Event` for pushing
## `data`
a `gpointer` with data to the `GstHarnessPrepareEventFunc` function
## `notify`
a `GDestroyNotify` that is called when thread is stopped
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each call to gst_pad_push

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_push_upstream_event_start_full -->
Push the `event` onto the harnessed `gst::Element` srcpad in intervals of
`sleep` microseconds.

MT safe.
## `event`
a `gst::Event` to push
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each gst_event_push with `event`

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_push_upstream_event_with_cb_start_full -->
Push a `gst::Event` returned by `func` onto the harnessed `gst::Element` srcpad
in intervals of `sleep` microseconds.

MT safe.
## `func`
a `GstHarnessPrepareEventFunc` function called before every iteration
to prepare / create a `gst::Event` for pushing
## `data`
a `gpointer` with data to the `GstHarnessPrepareEventFunc` function
## `notify`
a `GDestroyNotify` that is called when thread is stopped
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each call to gst_pad_push

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_requestpad_start_full -->
Call gst_element_request_pad in intervals of `sleep` microseconds

MT safe.
## `templ`
a `gst::PadTemplate`
## `name`
a `gchar`
## `caps`
a `gst::Caps`
## `release`
a `gboolean`
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each gst_element_request_pad

# Returns

a `HarnessThread`
<!-- impl Harness::fn stress_statechange_start_full -->
Change the state of your harnessed `gst::Element` from NULL to PLAYING and
back again, only pausing for `sleep` microseconds every time.

MT safe.
## `sleep`
a `gulong` specifying how long to sleep in (microseconds) for
each state-change

# Returns

a `HarnessThread`
<!-- impl Harness::fn take_all_data -->
Pulls all pending data from the harness and returns it as a single
data slice.

Feature: `v1_14`

## `size`
the size of the data in bytes

# Returns

a pointer to the data, newly allocated. Free
 with `g_free` when no longer needed. Will return `None` if there is no
 data.
<!-- impl Harness::fn take_all_data_as_buffer -->
Pulls all pending data from the harness and returns it as a single buffer.

Feature: `v1_14`


# Returns

the data as a buffer. Unref with `gst_buffer_unref`
 when no longer needed.
<!-- impl Harness::fn take_all_data_as_bytes -->
Pulls all pending data from the harness and returns it as a single `glib::Bytes`.

Feature: `v1_14`


# Returns

a pointer to the data, newly allocated. Free
 with `g_free` when no longer needed.
<!-- impl Harness::fn teardown -->
Tears down a `Harness`, freeing all resources allocated using it.

MT safe.
<!-- impl Harness::fn try_pull -->
Pulls a `gst::Buffer` from the `glib::AsyncQueue` on the `Harness` sinkpad. Unlike
gst_harness_pull this will not wait for any buffers if not any are present,
and return `None` straight away.

MT safe.

# Returns

a `gst::Buffer` or `None` if no buffers are present in the `glib::AsyncQueue`
<!-- impl Harness::fn try_pull_event -->
Pulls an `gst::Event` from the `glib::AsyncQueue` on the `Harness` sinkpad.
See gst_harness_try_pull for details.

MT safe.

# Returns

a `gst::Event` or `None` if no buffers are present in the `glib::AsyncQueue`
<!-- impl Harness::fn try_pull_upstream_event -->
Pulls an `gst::Event` from the `glib::AsyncQueue` on the `Harness` srcpad.
See gst_harness_try_pull for details.

MT safe.

# Returns

a `gst::Event` or `None` if no buffers are present in the `glib::AsyncQueue`
<!-- impl Harness::fn upstream_events_in_queue -->
The number of `GstEvents` currently in the `Harness` srcpad `glib::AsyncQueue`

MT safe.

# Returns

a `guint` number of events in the queue
<!-- impl Harness::fn upstream_events_received -->
The total number of `GstEvents` that has arrived on the `Harness` srcpad
This number includes events handled by the harness as well as events
that have already been pulled out.

MT safe.

# Returns

a `guint` number of events received
<!-- impl Harness::fn use_systemclock -->
Sets the system `gst::Clock` on the `Harness` `gst::Element`

MT safe.
<!-- impl Harness::fn use_testclock -->
Sets the `TestClock` on the `Harness` `gst::Element`

MT safe.
<!-- impl Harness::fn wait_for_clock_id_waits -->
Waits for `timeout` seconds until `waits` number of `gst::ClockID` waits is
registered with the `TestClock`. Useful for writing deterministic tests,
where you want to make sure that an expected number of waits have been
reached.

MT safe.
## `waits`
a `guint` describing the numbers of `gst::ClockID` registered with
the `TestClock`
## `timeout`
a `guint` describing how many seconds to wait for `waits` to be true

# Returns

a `gboolean` `true` if the waits have been registered, `false` if not.
(Could be that it timed out waiting or that more waits than waits was found)
<!-- impl Harness::fn new -->
Creates a new harness. Works like `Harness::new_with_padnames`, except it
assumes the `gst::Element` sinkpad is named "sink" and srcpad is named "src"

MT safe.
## `element_name`
a `gchar` describing the `gst::Element` name

# Returns

a `Harness`, or `None` if the harness could
not be created
<!-- impl Harness::fn new_empty -->
Creates a new empty harness. Use `Harness::add_element_full` to add
an `gst::Element` to it.

MT safe.

# Returns

a `Harness`, or `None` if the harness could
not be created
<!-- impl Harness::fn new_full -->
Creates a new harness.

MT safe.
## `element`
a `gst::Element` to attach the harness to (transfer none)
## `hsrc`
a `gst::StaticPadTemplate` describing the harness srcpad.
`None` will not create a harness srcpad.
## `element_sinkpad_name`
a `gchar` with the name of the element
sinkpad that is then linked to the harness srcpad. Can be a static or request
or a sometimes pad that has been added. `None` will not get/request a sinkpad
from the element. (Like if the element is a src.)
## `hsink`
a `gst::StaticPadTemplate` describing the harness sinkpad.
`None` will not create a harness sinkpad.
## `element_srcpad_name`
a `gchar` with the name of the element
srcpad that is then linked to the harness sinkpad, similar to the
`element_sinkpad_name`.

# Returns

a `Harness`, or `None` if the harness could
not be created
<!-- impl Harness::fn new_parse -->
Creates a new harness, parsing the `launchline` and putting that in a `gst::Bin`,
and then attches the harness to the bin.

MT safe.
## `launchline`
a `gchar` describing a gst-launch type line

# Returns

a `Harness`, or `None` if the harness could
not be created
<!-- impl Harness::fn new_with_element -->
Creates a new harness. Works in the same way as `Harness::new_full`, only
that generic padtemplates are used for the harness src and sinkpads, which
will be sufficient in most usecases.

MT safe.
## `element`
a `gst::Element` to attach the harness to (transfer none)
## `element_sinkpad_name`
a `gchar` with the name of the element
sinkpad that is then linked to the harness srcpad. `None` does not attach a
sinkpad
## `element_srcpad_name`
a `gchar` with the name of the element
srcpad that is then linked to the harness sinkpad. `None` does not attach a
srcpad

# Returns

a `Harness`, or `None` if the harness could
not be created
<!-- impl Harness::fn new_with_padnames -->
Creates a new harness. Works like `Harness::new_with_element`,
except you specify the factoryname of the `gst::Element`

MT safe.
## `element_name`
a `gchar` describing the `gst::Element` name
## `element_sinkpad_name`
a `gchar` with the name of the element
sinkpad that is then linked to the harness srcpad. `None` does not attach a
sinkpad
## `element_srcpad_name`
a `gchar` with the name of the element
srcpad that is then linked to the harness sinkpad. `None` does not attach a
srcpad

# Returns

a `Harness`, or `None` if the harness could
not be created
<!-- impl Harness::fn new_with_templates -->
Creates a new harness, like `Harness::new_full`, except it
assumes the `gst::Element` sinkpad is named "sink" and srcpad is named "src"

MT safe.
## `element_name`
a `gchar` describing the `gst::Element` name
## `hsrc`
a `gst::StaticPadTemplate` describing the harness srcpad.
`None` will not create a harness srcpad.
## `hsink`
a `gst::StaticPadTemplate` describing the harness sinkpad.
`None` will not create a harness sinkpad.

# Returns

a `Harness`, or `None` if the harness could
not be created
<!-- impl Harness::fn stress_thread_stop -->
Stop the running `HarnessThread`

MT safe.
## `t`
a `HarnessThread`
<!-- struct TestClock -->
GstTestClock is an implementation of `gst::Clock` which has different
behaviour compared to `gst::SystemClock`. Time for `gst::SystemClock` advances
according to the system time, while time for `TestClock` changes only
when `TestClock::set_time` or `TestClock::advance_time` are
called. `TestClock` provides unit tests with the possibility to
precisely advance the time in a deterministic manner, independent of the
system time or any other external factors.

## Advancing the time of a `TestClock`


```C
  #include <gst/gst.h>
  #include <gst/check/gsttestclock.h>

  GstClock *clock;
  GstTestClock *test_clock;

  clock = gst_test_clock_new ();
  test_clock = GST_TEST_CLOCK (clock);
  GST_INFO ("Time: %" GST_TIME_FORMAT, GST_TIME_ARGS (gst_clock_get_time (clock)));
  gst_test_clock_advance_time ( test_clock, 1 * GST_SECOND);
  GST_INFO ("Time: %" GST_TIME_FORMAT, GST_TIME_ARGS (gst_clock_get_time (clock)));
  g_usleep (10 * G_USEC_PER_SEC);
  GST_INFO ("Time: %" GST_TIME_FORMAT, GST_TIME_ARGS (gst_clock_get_time (clock)));
  gst_test_clock_set_time (test_clock, 42 * GST_SECOND);
  GST_INFO ("Time: %" GST_TIME_FORMAT, GST_TIME_ARGS (gst_clock_get_time (clock)));
  ...
```

`gst::Clock` allows for setting up single shot or periodic clock notifications
as well as waiting for these notifications synchronously (using
`gst::Clock::id_wait`) or asynchronously (using `gst::Clock::id_wait_async` or
`gst::Clock::id_wait_async`). This is used by many GStreamer elements,
among them `GstBaseSrc` and `GstBaseSink`.

`TestClock` keeps track of these clock notifications. By calling
`TestClock::wait_for_next_pending_id` or
`TestClock::wait_for_multiple_pending_ids` a unit tests may wait for the
next one or several clock notifications to be requested. Additionally unit
tests may release blocked waits in a controlled fashion by calling
`TestClock::process_next_clock_id`. This way a unit test can control the
inaccuracy (jitter) of clock notifications, since the test can decide to
release blocked waits when the clock time has advanced exactly to, or past,
the requested clock notification time.

There are also interfaces for determining if a notification belongs to a
`TestClock` or not, as well as getting the number of requested clock
notifications so far.

N.B.: When a unit test waits for a certain amount of clock notifications to
be requested in `TestClock::wait_for_next_pending_id` or
`TestClock::wait_for_multiple_pending_ids` then these functions may block
for a long time. If they block forever then the expected clock notifications
were never requested from `TestClock`, and so the assumptions in the code
of the unit test are wrong. The unit test case runner in gstcheck is
expected to catch these cases either by the default test case timeout or the
one set for the unit test by calling tcase_set_timeout\(\).

The sample code below assumes that the element under test will delay a
buffer pushed on the source pad by some latency until it arrives on the sink
pad. Moreover it is assumed that the element will at some point call
`gst::Clock::id_wait` to synchronously wait for a specific time. The first
buffer sent will arrive exactly on time only delayed by the latency. The
second buffer will arrive a little late (7ms) due to simulated jitter in the
clock notification.

## Demonstration of how to work with clock notifications and `TestClock`


```C
  #include <gst/gst.h>
  #include <gst/check/gstcheck.h>
  #include <gst/check/gsttestclock.h>

  GstClockTime latency;
  GstElement *element;
  GstPad *srcpad;
  GstClock *clock;
  GstTestClock *test_clock;
  GstBuffer buf;
  GstClockID pending_id;
  GstClockID processed_id;

  latency = 42 * GST_MSECOND;
  element = create_element (latency, ...);
  srcpad = get_source_pad (element);

  clock = gst_test_clock_new ();
  test_clock = GST_TEST_CLOCK (clock);
  gst_element_set_clock (element, clock);

  GST_INFO ("Set time, create and push the first buffer\n");
  gst_test_clock_set_time (test_clock, 0);
  buf = create_test_buffer (gst_clock_get_time (clock), ...);
  gst_assert_cmpint (gst_pad_push (srcpad, buf), ==, GST_FLOW_OK);

  GST_INFO ("Block until element is waiting for a clock notification\n");
  gst_test_clock_wait_for_next_pending_id (test_clock, &pending_id);
  GST_INFO ("Advance to the requested time of the clock notification\n");
  gst_test_clock_advance_time (test_clock, latency);
  GST_INFO ("Release the next blocking wait and make sure it is the one from element\n");
  processed_id = gst_test_clock_process_next_clock_id (test_clock);
  g_assert (processed_id == pending_id);
  g_assert_cmpint (GST_CLOCK_ENTRY_STATUS (processed_id), ==, GST_CLOCK_OK);
  gst_clock_id_unref (pending_id);
  gst_clock_id_unref (processed_id);

  GST_INFO ("Validate that element produced an output buffer and check its timestamp\n");
  g_assert_cmpint (get_number_of_output_buffer (...), ==, 1);
  buf = get_buffer_pushed_by_element (element, ...);
  g_assert_cmpint (GST_BUFFER_TIMESTAMP (buf), ==, latency);
  gst_buffer_unref (buf);
  GST_INFO ("Check that element does not wait for any clock notification\n");
  g_assert (!gst_test_clock_peek_next_pending_id (test_clock, NULL));

  GST_INFO ("Set time, create and push the second buffer\n");
  gst_test_clock_advance_time (test_clock, 10 * GST_SECOND);
  buf = create_test_buffer (gst_clock_get_time (clock), ...);
  gst_assert_cmpint (gst_pad_push (srcpad, buf), ==, GST_FLOW_OK);

  GST_INFO ("Block until element is waiting for a new clock notification\n");
  (gst_test_clock_wait_for_next_pending_id (test_clock, &pending_id);
  GST_INFO ("Advance past 7ms beyond the requested time of the clock notification\n");
  gst_test_clock_advance_time (test_clock, latency + 7 * GST_MSECOND);
  GST_INFO ("Release the next blocking wait and make sure it is the one from element\n");
  processed_id = gst_test_clock_process_next_clock_id (test_clock);
  g_assert (processed_id == pending_id);
  g_assert_cmpint (GST_CLOCK_ENTRY_STATUS (processed_id), ==, GST_CLOCK_OK);
  gst_clock_id_unref (pending_id);
  gst_clock_id_unref (processed_id);

  GST_INFO ("Validate that element produced an output buffer and check its timestamp\n");
  g_assert_cmpint (get_number_of_output_buffer (...), ==, 1);
  buf = get_buffer_pushed_by_element (element, ...);
  g_assert_cmpint (GST_BUFFER_TIMESTAMP (buf), ==,
      10 * GST_SECOND + latency + 7 * GST_MSECOND);
  gst_buffer_unref (buf);
  GST_INFO ("Check that element does not wait for any clock notification\n");
  g_assert (!gst_test_clock_peek_next_pending_id (test_clock, NULL));
  ...
```

Since `TestClock` is only supposed to be used in unit tests it calls
`g_assert`, `g_assert_cmpint` or `g_assert_cmpuint` to validate all function
arguments. This will highlight any issues with the unit test code itself.

# Implements

[`gst::ClockExt`](../gst/trait.ClockExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl TestClock::fn new -->
Creates a new test clock with its time set to zero.

MT safe.

# Returns

a `TestClock` cast to `gst::Clock`.
<!-- impl TestClock::fn new_with_start_time -->
Creates a new test clock with its time set to the specified time.

MT safe.
## `start_time`
a `gst::ClockTime` set to the desired start time of the clock.

# Returns

a `TestClock` cast to `gst::Clock`.
<!-- impl TestClock::fn id_list_get_latest_time -->
Finds the latest time inside the list.

MT safe.
## `pending_list`
List
 of of pending `GstClockIDs`
<!-- impl TestClock::fn advance_time -->
Advances the time of the `self` by the amount given by `delta`. The
time of `self` is monotonically increasing, therefore providing a
`delta` which is negative or zero is a programming error.

MT safe.
## `delta`
a positive `gst::ClockTimeDiff` to be added to the time of the clock
<!-- impl TestClock::fn crank -->
A "crank" consists of three steps:
1: Wait for a `gst::ClockID` to be registered with the `TestClock`.
2: Advance the `TestClock` to the time the `gst::ClockID` is waiting for.
3: Release the `gst::ClockID` wait.
A "crank" can be though of as the notion of
manually driving the clock forward to its next logical step.

# Returns

`true` if the crank was successful, `false` otherwise.

MT safe.
<!-- impl TestClock::fn get_next_entry_time -->
Retrieve the requested time for the next pending clock notification.

MT safe.

# Returns

a `gst::ClockTime` set to the time of the next pending clock
notification. If no clock notifications have been requested
`GST_CLOCK_TIME_NONE` will be returned.
<!-- impl TestClock::fn has_id -->
Checks whether `self` was requested to provide the clock notification
given by `id`.

MT safe.
## `id`
a `gst::ClockID` clock notification

# Returns

`true` if the clock has been asked to provide the given clock
notification, `false` otherwise.
<!-- impl TestClock::fn peek_id_count -->
Determine the number of pending clock notifications that have been
requested from the `self`.

MT safe.

# Returns

the number of pending clock notifications.
<!-- impl TestClock::fn peek_next_pending_id -->
Determines if the `pending_id` is the next clock notification scheduled to
be triggered given the current time of the `self`.

MT safe.
## `pending_id`
a `gst::ClockID` clock
notification to look for

# Returns

`true` if `pending_id` is the next clock notification to be
triggered, `false` otherwise.
<!-- impl TestClock::fn process_id_list -->
Processes and releases the pending IDs in the list.

MT safe.
## `pending_list`
List
 of pending `GstClockIDs`
<!-- impl TestClock::fn process_next_clock_id -->
MT safe.

# Returns

a `gst::ClockID` containing the next pending clock
notification.
<!-- impl TestClock::fn set_time -->
Sets the time of `self` to the time given by `new_time`. The time of
`self` is monotonically increasing, therefore providing a `new_time`
which is earlier or equal to the time of the clock as given by
`gst::ClockExt::get_time` is a programming error.

MT safe.
## `new_time`
a `gst::ClockTime` later than that returned by `gst::ClockExt::get_time`
<!-- impl TestClock::fn timed_wait_for_multiple_pending_ids -->
Blocks until at least `count` clock notifications have been requested from
`self`, or the timeout expires.

MT safe.

Feature: `v1_16`

## `count`
the number of pending clock notifications to wait for
## `timeout_ms`
the timeout in milliseconds
## `pending_list`
Address
 of a `glib::List` pointer variable to store the list of pending `GstClockIDs`
 that expired, or `None`

# Returns

a `gboolean` `true` if the waits have been registered, `false` if not.
(Could be that it timed out waiting or that more waits than waits was found)
<!-- impl TestClock::fn wait_for_multiple_pending_ids -->
Blocks until at least `count` clock notifications have been requested from
`self`. There is no timeout for this wait, see the main description of
`TestClock`.

MT safe.
## `count`
the number of pending clock notifications to wait for
## `pending_list`
Address
 of a `glib::List` pointer variable to store the list of pending `GstClockIDs`
 that expired, or `None`
<!-- impl TestClock::fn wait_for_next_pending_id -->
Waits until a clock notification is requested from `self`. There is no
timeout for this wait, see the main description of `TestClock`. A reference
to the pending clock notification is stored in `pending_id`.

MT safe.
## `pending_id`
`gst::ClockID`
with information about the pending clock notification
<!-- impl TestClock::fn wait_for_pending_id_count -->
Blocks until at least `count` clock notifications have been requested from
`self`. There is no timeout for this wait, see the main description of
`TestClock`.

# Deprecated

use `TestClock::wait_for_multiple_pending_ids` instead.
## `count`
the number of pending clock notifications to wait for
<!-- trait TestClockExt::fn get_property_start-time -->
When a `TestClock` is constructed it will have a certain start time set.
If the clock was created using `TestClock::new_with_start_time` then
this property contains the value of the `start_time` argument. If
`TestClock::new` was called the clock started at time zero, and thus
this property contains the value 0.
<!-- trait TestClockExt::fn set_property_start-time -->
When a `TestClock` is constructed it will have a certain start time set.
If the clock was created using `TestClock::new_with_start_time` then
this property contains the value of the `start_time` argument. If
`TestClock::new` was called the clock started at time zero, and thus
this property contains the value 0.
