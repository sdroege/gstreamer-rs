<!-- file * -->
<!-- struct AudioBaseSink -->
This is the base class for audio sinks. Subclasses need to implement the
::create_ringbuffer vmethod. This base class will then take care of
writing samples to the ringbuffer, synchronisation, clipping and flushing.

# Implements

[`AudioBaseSinkExt`](trait.AudioBaseSinkExt.html), [`gst_base::BaseSinkExt`](../gst_base/trait.BaseSinkExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AudioBaseSinkExt -->
Trait containing all `AudioBaseSink` methods.

# Implementors

[`AudioBaseSink`](struct.AudioBaseSink.html), [`AudioSink`](struct.AudioSink.html)
<!-- trait AudioBaseSinkExt::fn create_ringbuffer -->
Create and return the `AudioRingBuffer` for `self`. This function will
call the ::create_ringbuffer vmethod and will set `self` as the parent of
the returned buffer (see `gst::ObjectExt::set_parent`).

# Returns

The new ringbuffer of `self`.
<!-- trait AudioBaseSinkExt::fn get_alignment_threshold -->
Get the current alignment threshold, in nanoseconds, used by `self`.

# Returns

The current alignment threshold used by `self`.
<!-- trait AudioBaseSinkExt::fn get_discont_wait -->
Get the current discont wait, in nanoseconds, used by `self`.

# Returns

The current discont wait used by `self`.
<!-- trait AudioBaseSinkExt::fn get_drift_tolerance -->
Get the current drift tolerance, in microseconds, used by `self`.

# Returns

The current drift tolerance used by `self`.
<!-- trait AudioBaseSinkExt::fn get_provide_clock -->
Queries whether `self` will provide a clock or not. See also
gst_audio_base_sink_set_provide_clock.

# Returns

`true` if `self` will provide a clock.
<!-- trait AudioBaseSinkExt::fn get_slave_method -->
Get the current slave method used by `self`.

# Returns

The current slave method used by `self`.
<!-- trait AudioBaseSinkExt::fn report_device_failure -->
Informs this base class that the audio output device has failed for
some reason, causing a discontinuity (for example, because the device
recovered from the error, but lost all contents of its ring buffer).
This function is typically called by derived classes, and is useful
for the custom slave method.
<!-- trait AudioBaseSinkExt::fn set_alignment_threshold -->
Controls the sink's alignment threshold.
## `alignment_threshold`
the new alignment threshold in nanoseconds
<!-- trait AudioBaseSinkExt::fn set_custom_slaving_callback -->
Sets the custom slaving callback. This callback will
be invoked if the slave-method property is set to
GST_AUDIO_BASE_SINK_SLAVE_CUSTOM and the audio sink
receives and plays samples.

Setting the callback to NULL causes the sink to
behave as if the GST_AUDIO_BASE_SINK_SLAVE_NONE
method were used.
## `callback`
a `GstAudioBaseSinkCustomSlavingCallback`
## `user_data`
user data passed to the callback
## `notify`
called when user_data becomes unused
<!-- trait AudioBaseSinkExt::fn set_discont_wait -->
Controls how long the sink will wait before creating a discontinuity.
## `discont_wait`
the new discont wait in nanoseconds
<!-- trait AudioBaseSinkExt::fn set_drift_tolerance -->
Controls the sink's drift tolerance.
## `drift_tolerance`
the new drift tolerance in microseconds
<!-- trait AudioBaseSinkExt::fn set_provide_clock -->
Controls whether `self` will provide a clock or not. If `provide` is `true`,
`gst::ElementExt::provide_clock` will return a clock that reflects the datarate
of `self`. If `provide` is `false`, `gst::ElementExt::provide_clock` will return
NULL.
## `provide`
new state
<!-- trait AudioBaseSinkExt::fn set_slave_method -->
Controls how clock slaving will be performed in `self`.
## `method`
the new slave method
<!-- trait AudioBaseSinkExt::fn get_property_discont_wait -->
A window of time in nanoseconds to wait before creating a discontinuity as
a result of breaching the drift-tolerance.
<!-- trait AudioBaseSinkExt::fn set_property_discont_wait -->
A window of time in nanoseconds to wait before creating a discontinuity as
a result of breaching the drift-tolerance.
<!-- trait AudioBaseSinkExt::fn get_property_drift_tolerance -->
Controls the amount of time in microseconds that clocks are allowed
to drift before resynchronisation happens.
<!-- trait AudioBaseSinkExt::fn set_property_drift_tolerance -->
Controls the amount of time in microseconds that clocks are allowed
to drift before resynchronisation happens.
<!-- struct AudioBaseSrc -->
This is the base class for audio sources. Subclasses need to implement the
::create_ringbuffer vmethod. This base class will then take care of
reading samples from the ringbuffer, synchronisation and flushing.

# Implements

[`AudioBaseSrcExt`](trait.AudioBaseSrcExt.html), [`gst_base::BaseSrcExt`](../gst_base/trait.BaseSrcExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AudioBaseSrcExt -->
Trait containing all `AudioBaseSrc` methods.

# Implementors

[`AudioBaseSrc`](struct.AudioBaseSrc.html), [`AudioSrc`](struct.AudioSrc.html)
<!-- trait AudioBaseSrcExt::fn create_ringbuffer -->
Create and return the `AudioRingBuffer` for `self`. This function will call
the ::create_ringbuffer vmethod and will set `self` as the parent of the
returned buffer (see `gst::ObjectExt::set_parent`).

# Returns

The new ringbuffer of `self`.
<!-- trait AudioBaseSrcExt::fn get_provide_clock -->
Queries whether `self` will provide a clock or not. See also
gst_audio_base_src_set_provide_clock.

# Returns

`true` if `self` will provide a clock.
<!-- trait AudioBaseSrcExt::fn get_slave_method -->
Get the current slave method used by `self`.

# Returns

The current slave method used by `self`.
<!-- trait AudioBaseSrcExt::fn set_provide_clock -->
Controls whether `self` will provide a clock or not. If `provide` is `true`,
`gst::ElementExt::provide_clock` will return a clock that reflects the datarate
of `self`. If `provide` is `false`, `gst::ElementExt::provide_clock` will return NULL.
## `provide`
new state
<!-- trait AudioBaseSrcExt::fn set_slave_method -->
Controls how clock slaving will be performed in `self`.
## `method`
the new slave method
<!-- trait AudioBaseSrcExt::fn get_property_actual_buffer_time -->
Actual configured size of audio buffer in microseconds.
<!-- trait AudioBaseSrcExt::fn get_property_actual_latency_time -->
Actual configured audio latency in microseconds.
<!-- enum AudioChannelPosition -->
Audio channel positions.

These are the channels defined in SMPTE 2036-2-2008
Table 1 for 22.2 audio systems with the Surround and Wide channels from
DTS Coherent Acoustics (v.1.3.1) and 10.2 and 7.1 layouts. In the caps the
actual channel layout is expressed with a channel count and a channel mask,
which describes the existing channels. The positions in the bit mask correspond
to the enum values.
For negotiation it is allowed to have more bits set in the channel mask than
the number of channels to specify the allowed channel positions but this is
not allowed in negotiated caps. It is not allowed in any situation other
than the one mentioned below to have less bits set in the channel mask than
the number of channels.

`AudioChannelPosition::Mono` can only be used with a single mono channel that
has no direction information and would be mixed into all directional channels.
This is expressed in caps by having a single channel and no channel mask.

`AudioChannelPosition::None` can only be used if all channels have this position.
This is expressed in caps by having a channel mask with no bits set.

As another special case it is allowed to have two channels without a channel mask.
This implicitely means that this is a stereo stream with a front left and front right
channel.
<!-- enum AudioChannelPosition::variant None -->
used for position-less channels, e.g.
 from a sound card that records 1024 channels; mutually exclusive with
 any other channel position
<!-- enum AudioChannelPosition::variant Mono -->
Mono without direction;
 can only be used with 1 channel
<!-- enum AudioChannelPosition::variant Invalid -->
invalid position
<!-- enum AudioChannelPosition::variant FrontLeft -->
Front left
<!-- enum AudioChannelPosition::variant FrontRight -->
Front right
<!-- enum AudioChannelPosition::variant FrontCenter -->
Front center
<!-- enum AudioChannelPosition::variant Lfe1 -->
Low-frequency effects 1 (subwoofer)
<!-- enum AudioChannelPosition::variant RearLeft -->
Rear left
<!-- enum AudioChannelPosition::variant RearRight -->
Rear right
<!-- enum AudioChannelPosition::variant FrontLeftOfCenter -->
Front left of center
<!-- enum AudioChannelPosition::variant FrontRightOfCenter -->
Front right of center
<!-- enum AudioChannelPosition::variant RearCenter -->
Rear center
<!-- enum AudioChannelPosition::variant Lfe2 -->
Low-frequency effects 2 (subwoofer)
<!-- enum AudioChannelPosition::variant SideLeft -->
Side left
<!-- enum AudioChannelPosition::variant SideRight -->
Side right
<!-- enum AudioChannelPosition::variant TopFrontLeft -->
Top front left
<!-- enum AudioChannelPosition::variant TopFrontRight -->
Top front right
<!-- enum AudioChannelPosition::variant TopFrontCenter -->
Top front center
<!-- enum AudioChannelPosition::variant TopCenter -->
Top center
<!-- enum AudioChannelPosition::variant TopRearLeft -->
Top rear left
<!-- enum AudioChannelPosition::variant TopRearRight -->
Top rear right
<!-- enum AudioChannelPosition::variant TopSideLeft -->
Top side right
<!-- enum AudioChannelPosition::variant TopSideRight -->
Top rear right
<!-- enum AudioChannelPosition::variant TopRearCenter -->
Top rear center
<!-- enum AudioChannelPosition::variant BottomFrontCenter -->
Bottom front center
<!-- enum AudioChannelPosition::variant BottomFrontLeft -->
Bottom front left
<!-- enum AudioChannelPosition::variant BottomFrontRight -->
Bottom front right
<!-- enum AudioChannelPosition::variant WideLeft -->
Wide left (between front left and side left)
<!-- enum AudioChannelPosition::variant WideRight -->
Wide right (between front right and side right)
<!-- enum AudioChannelPosition::variant SurroundLeft -->
Surround left (between rear left and side left)
<!-- enum AudioChannelPosition::variant SurroundRight -->
Surround right (between rear right and side right)
<!-- struct AudioDecoder -->
This base class is for audio decoders turning encoded data into
raw audio samples.

GstAudioDecoder and subclass should cooperate as follows.

## Configuration

 * Initially, GstAudioDecoder calls `start` when the decoder element
 is activated, which allows subclass to perform any global setup.
 Base class (context) parameters can already be set according to subclass
 capabilities (or possibly upon receive more information in subsequent
 `set_format`).
 * GstAudioDecoder calls `set_format` to inform subclass of the format
 of input audio data that it is about to receive.
 While unlikely, it might be called more than once, if changing input
 parameters require reconfiguration.
 * GstAudioDecoder calls `stop` at end of all processing.

As of configuration stage, and throughout processing, GstAudioDecoder
provides various (context) parameters, e.g. describing the format of
output audio data (valid when output caps have been set) or current parsing state.
Conversely, subclass can and should configure context to inform
base class of its expectation w.r.t. buffer handling.

## Data processing
 * Base class gathers input data, and optionally allows subclass
 to parse this into subsequently manageable (as defined by subclass)
 chunks. Such chunks are subsequently referred to as 'frames',
 though they may or may not correspond to 1 (or more) audio format frame.
 * Input frame is provided to subclass' `handle_frame`.
 * If codec processing results in decoded data, subclass should call
 `AudioDecoder::finish_frame` to have decoded data pushed
 downstream.
 * Just prior to actually pushing a buffer downstream,
 it is passed to `pre_push`. Subclass should either use this callback
 to arrange for additional downstream pushing or otherwise ensure such
 custom pushing occurs after at least a method call has finished since
 setting src pad caps.
 * During the parsing process GstAudioDecoderClass will handle both
 srcpad and sinkpad events. Sink events will be passed to subclass
 if `event` callback has been provided.

## Shutdown phase

 * GstAudioDecoder class calls `stop` to inform the subclass that data
 parsing will be stopped.

Subclass is responsible for providing pad template caps for
source and sink pads. The pads need to be named "sink" and "src". It also
needs to set the fixed caps on srcpad, when the format is ensured. This
is typically when base class calls subclass' `set_format` function, though
it might be delayed until calling `AudioDecoder::finish_frame`.

In summary, above process should have subclass concentrating on
codec data processing while leaving other matters to base class,
such as most notably timestamp handling. While it may exert more control
in this area (see e.g. `pre_push`), it is very much not recommended.

In particular, base class will try to arrange for perfect output timestamps
as much as possible while tracking upstream timestamps.
To this end, if deviation between the next ideal expected perfect timestamp
and upstream exceeds `AudioDecoder:tolerance`, then resync to upstream
occurs (which would happen always if the tolerance mechanism is disabled).

In non-live pipelines, baseclass can also (configurably) arrange for
output buffer aggregation which may help to redue large(r) numbers of
small(er) buffers being pushed and processed downstream. Note that this
feature is only available if the buffer layout is interleaved. For planar
buffers, the decoder implementation is fully responsible for the output
buffer size.

On the other hand, it should be noted that baseclass only provides limited
seeking support (upon explicit subclass request), as full-fledged support
should rather be left to upstream demuxer, parser or alike. This simple
approach caters for seeking and duration reporting using estimated input
bitrates.

Things that subclass need to take care of:

 * Provide pad templates
 * Set source pad caps when appropriate
 * Set user-configurable properties to sane defaults for format and
 implementing codec at hand, and convey some subclass capabilities and
 expectations in context.

 * Accept data in `handle_frame` and provide encoded results to
 `AudioDecoder::finish_frame`. If it is prepared to perform
 PLC, it should also accept NULL data in `handle_frame` and provide for
 data for indicated duration.

# Implements

[`AudioDecoderExt`](trait.AudioDecoderExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AudioDecoderExt -->
Trait containing all `AudioDecoder` methods.

# Implementors

[`AudioDecoder`](struct.AudioDecoder.html)
<!-- trait AudioDecoderExt::fn allocate_output_buffer -->
Helper function that allocates a buffer to hold an audio frame
for `self`'s current output format.
## `size`
size of the buffer

# Returns

allocated buffer
<!-- trait AudioDecoderExt::fn finish_frame -->
Collects decoded data and pushes it downstream.

`buf` may be NULL in which case the indicated number of frames
are discarded and considered to have produced no output
(e.g. lead-in or setup frames).
Otherwise, source pad caps must be set when it is called with valid
data in `buf`.

Note that a frame received in `AudioDecoderClass.handle_frame`() may be
invalidated by a call to this function.
## `buf`
decoded data
## `frames`
number of decoded frames represented by decoded data

# Returns

a `gst::FlowReturn` that should be escalated to caller (of caller)
<!-- trait AudioDecoderExt::fn finish_subframe -->
Collects decoded data and pushes it downstream. This function may be called
multiple times for a given input frame.

`buf` may be NULL in which case it is assumed that the current input frame is
finished. This is equivalent to calling `AudioDecoder::finish_subframe`
with a NULL buffer and frames=1 after having pushed out all decoded audio
subframes using this function.

When called with valid data in `buf` the source pad caps must have been set
already.

Note that a frame received in `AudioDecoderClass.handle_frame`() may be
invalidated by a call to this function.

Feature: `v1_16`

## `buf`
decoded data

# Returns

a `gst::FlowReturn` that should be escalated to caller (of caller)
<!-- trait AudioDecoderExt::fn get_allocator -->
Lets `AudioDecoder` sub-classes to know the memory `allocator`
used by the base class and its `params`.

Unref the `allocator` after use it.
## `allocator`
the `gst::Allocator`
used
## `params`
the
`gst::AllocationParams` of `allocator`
<!-- trait AudioDecoderExt::fn get_audio_info -->

# Returns

a `AudioInfo` describing the input audio format
<!-- trait AudioDecoderExt::fn get_delay -->

# Returns

currently configured decoder delay
<!-- trait AudioDecoderExt::fn get_drainable -->
Queries decoder drain handling.

# Returns

TRUE if drainable handling is enabled.

MT safe.
<!-- trait AudioDecoderExt::fn get_estimate_rate -->

# Returns

currently configured byte to time conversion setting
<!-- trait AudioDecoderExt::fn get_latency -->
Sets the variables pointed to by `min` and `max` to the currently configured
latency.
## `min`
a pointer to storage to hold minimum latency
## `max`
a pointer to storage to hold maximum latency
<!-- trait AudioDecoderExt::fn get_max_errors -->

# Returns

currently configured decoder tolerated error count.
<!-- trait AudioDecoderExt::fn get_min_latency -->
Queries decoder's latency aggregation.

# Returns

aggregation latency.

MT safe.
<!-- trait AudioDecoderExt::fn get_needs_format -->
Queries decoder required format handling.

# Returns

TRUE if required format handling is enabled.

MT safe.
<!-- trait AudioDecoderExt::fn get_parse_state -->
Return current parsing (sync and eos) state.
## `sync`
a pointer to a variable to hold the current sync state
## `eos`
a pointer to a variable to hold the current eos state
<!-- trait AudioDecoderExt::fn get_plc -->
Queries decoder packet loss concealment handling.

# Returns

TRUE if packet loss concealment is enabled.

MT safe.
<!-- trait AudioDecoderExt::fn get_plc_aware -->

# Returns

currently configured plc handling
<!-- trait AudioDecoderExt::fn get_tolerance -->
Queries current audio jitter tolerance threshold.

# Returns

decoder audio jitter tolerance threshold.

MT safe.
<!-- trait AudioDecoderExt::fn merge_tags -->
Sets the audio decoder tags and how they should be merged with any
upstream stream tags. This will override any tags previously-set
with `AudioDecoderExt::merge_tags`.

Note that this is provided for convenience, and the subclass is
not required to use this and can still do tag handling on its own.
## `tags`
a `gst::TagList` to merge, or NULL
## `mode`
the `gst::TagMergeMode` to use, usually `gst::TagMergeMode::Replace`
<!-- trait AudioDecoderExt::fn negotiate -->
Negotiate with downstream elements to currently configured `AudioInfo`.
Unmark GST_PAD_FLAG_NEED_RECONFIGURE in any case. But mark it again if
negotiate fails.

# Returns

`true` if the negotiation succeeded, else `false`.
<!-- trait AudioDecoderExt::fn proxy_getcaps -->
Returns caps that express `caps` (or sink template caps if `caps` == NULL)
restricted to rate/channels/... combinations supported by downstream
elements.
## `caps`
initial caps
## `filter`
filter caps

# Returns

a `gst::Caps` owned by caller
<!-- trait AudioDecoderExt::fn set_allocation_caps -->
Sets a caps in allocation query which are different from the set
pad's caps. Use this function before calling
`AudioDecoder::negotiate`. Setting to `None` the allocation
query will use the caps from the pad.

Feature: `v1_10`

## `allocation_caps`
a `gst::Caps` or `None`
<!-- trait AudioDecoderExt::fn set_drainable -->
Configures decoder drain handling. If drainable, subclass might
be handed a NULL buffer to have it return any leftover decoded data.
Otherwise, it is not considered so capable and will only ever be passed
real data.

MT safe.
## `enabled`
new state
<!-- trait AudioDecoderExt::fn set_estimate_rate -->
Allows baseclass to perform byte to time estimated conversion.
## `enabled`
whether to enable byte to time conversion
<!-- trait AudioDecoderExt::fn set_latency -->
Sets decoder latency.
## `min`
minimum latency
## `max`
maximum latency
<!-- trait AudioDecoderExt::fn set_max_errors -->
Sets numbers of tolerated decoder errors, where a tolerated one is then only
warned about, but more than tolerated will lead to fatal error. You can set
-1 for never returning fatal errors. Default is set to
GST_AUDIO_DECODER_MAX_ERRORS.
## `num`
max tolerated errors
<!-- trait AudioDecoderExt::fn set_min_latency -->
Sets decoder minimum aggregation latency.

MT safe.
## `num`
new minimum latency
<!-- trait AudioDecoderExt::fn set_needs_format -->
Configures decoder format needs. If enabled, subclass needs to be
negotiated with format caps before it can process any data. It will then
never be handed any data before it has been configured.
Otherwise, it might be handed data without having been configured and
is then expected being able to do so either by default
or based on the input data.

MT safe.
## `enabled`
new state
<!-- trait AudioDecoderExt::fn set_output_caps -->
Configure output caps on the srcpad of `self`. Similar to
`AudioDecoder::set_output_format`, but allows subclasses to specify
output caps that can't be expressed via `AudioInfo` e.g. caps that have
caps features.

Feature: `v1_16`

## `caps`
(fixed) `gst::Caps`

# Returns

`true` on success.
<!-- trait AudioDecoderExt::fn set_output_format -->
Configure output info on the srcpad of `self`.
## `info`
`AudioInfo`

# Returns

`true` on success.
<!-- trait AudioDecoderExt::fn set_plc -->
Enable or disable decoder packet loss concealment, provided subclass
and codec are capable and allow handling plc.

MT safe.
## `enabled`
new state
<!-- trait AudioDecoderExt::fn set_plc_aware -->
Indicates whether or not subclass handles packet loss concealment (plc).
## `plc`
new plc state
<!-- trait AudioDecoderExt::fn set_tolerance -->
Configures decoder audio jitter tolerance threshold.

MT safe.
## `tolerance`
new tolerance
<!-- trait AudioDecoderExt::fn set_use_default_pad_acceptcaps -->
Lets `AudioDecoder` sub-classes decide if they want the sink pad
to use the default pad query handler to reply to accept-caps queries.

By setting this to true it is possible to further customize the default
handler with `GST_PAD_SET_ACCEPT_INTERSECT` and
`GST_PAD_SET_ACCEPT_TEMPLATE`
## `use_`
if the default pad accept-caps query handling should be used
<!-- struct AudioEncoder -->
This base class is for audio encoders turning raw audio samples into
encoded audio data.

GstAudioEncoder and subclass should cooperate as follows.

## Configuration

 * Initially, GstAudioEncoder calls `start` when the encoder element
 is activated, which allows subclass to perform any global setup.

 * GstAudioEncoder calls `set_format` to inform subclass of the format
 of input audio data that it is about to receive. Subclass should
 setup for encoding and configure various base class parameters
 appropriately, notably those directing desired input data handling.
 While unlikely, it might be called more than once, if changing input
 parameters require reconfiguration.

 * GstAudioEncoder calls `stop` at end of all processing.

As of configuration stage, and throughout processing, GstAudioEncoder
maintains various parameters that provide required context,
e.g. describing the format of input audio data.
Conversely, subclass can and should configure these context parameters
to inform base class of its expectation w.r.t. buffer handling.

## Data processing

 * Base class gathers input sample data (as directed by the context's
 frame_samples and frame_max) and provides this to subclass' `handle_frame`.
 * If codec processing results in encoded data, subclass should call
 `AudioEncoder::finish_frame` to have encoded data pushed
 downstream. Alternatively, it might also call
 `AudioEncoder::finish_frame` (with a NULL buffer and some number of
 dropped samples) to indicate dropped (non-encoded) samples.
 * Just prior to actually pushing a buffer downstream,
 it is passed to `pre_push`.
 * During the parsing process GstAudioEncoderClass will handle both
 srcpad and sinkpad events. Sink events will be passed to subclass
 if `event` callback has been provided.

## Shutdown phase

 * GstAudioEncoder class calls `stop` to inform the subclass that data
 parsing will be stopped.

Subclass is responsible for providing pad template caps for
source and sink pads. The pads need to be named "sink" and "src". It also
needs to set the fixed caps on srcpad, when the format is ensured. This
is typically when base class calls subclass' `set_format` function, though
it might be delayed until calling `AudioEncoder::finish_frame`.

In summary, above process should have subclass concentrating on
codec data processing while leaving other matters to base class,
such as most notably timestamp handling. While it may exert more control
in this area (see e.g. `pre_push`), it is very much not recommended.

In particular, base class will either favor tracking upstream timestamps
(at the possible expense of jitter) or aim to arrange for a perfect stream of
output timestamps, depending on `AudioEncoder:perfect-timestamp`.
However, in the latter case, the input may not be so perfect or ideal, which
is handled as follows. An input timestamp is compared with the expected
timestamp as dictated by input sample stream and if the deviation is less
than `AudioEncoder:tolerance`, the deviation is discarded.
Otherwise, it is considered a discontuinity and subsequent output timestamp
is resynced to the new position after performing configured discontinuity
processing. In the non-perfect-timestamp case, an upstream variation
exceeding tolerance only leads to marking DISCONT on subsequent outgoing
(while timestamps are adjusted to upstream regardless of variation).
While DISCONT is also marked in the perfect-timestamp case, this one
optionally (see `AudioEncoder:hard-resync`)
performs some additional steps, such as clipping of (early) input samples
or draining all currently remaining input data, depending on the direction
of the discontuinity.

If perfect timestamps are arranged, it is also possible to request baseclass
(usually set by subclass) to provide additional buffer metadata (in OFFSET
and OFFSET_END) fields according to granule defined semantics currently
needed by oggmux. Specifically, OFFSET is set to granulepos (= sample count
including buffer) and OFFSET_END to corresponding timestamp (as determined
by same sample count and sample rate).

Things that subclass need to take care of:

 * Provide pad templates
 * Set source pad caps when appropriate
 * Inform base class of buffer processing needs using context's
 frame_samples and frame_bytes.
 * Set user-configurable properties to sane defaults for format and
 implementing codec at hand, e.g. those controlling timestamp behaviour
 and discontinuity processing.
 * Accept data in `handle_frame` and provide encoded results to
 `AudioEncoder::finish_frame`.

# Implements

[`AudioEncoderExt`](trait.AudioEncoderExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AudioEncoderExt -->
Trait containing all `AudioEncoder` methods.

# Implementors

[`AudioEncoder`](struct.AudioEncoder.html)
<!-- trait AudioEncoderExt::fn allocate_output_buffer -->
Helper function that allocates a buffer to hold an encoded audio frame
for `self`'s current output format.
## `size`
size of the buffer

# Returns

allocated buffer
<!-- trait AudioEncoderExt::fn finish_frame -->
Collects encoded data and pushes encoded data downstream.
Source pad caps must be set when this is called.

If `samples` < 0, then best estimate is all samples provided to encoder
(subclass) so far. `buf` may be NULL, in which case next number of `samples`
are considered discarded, e.g. as a result of discontinuous transmission,
and a discontinuity is marked.

Note that samples received in `AudioEncoderClass.handle_frame`()
may be invalidated by a call to this function.
## `buffer`
encoded data
## `samples`
number of samples (per channel) represented by encoded data

# Returns

a `gst::FlowReturn` that should be escalated to caller (of caller)
<!-- trait AudioEncoderExt::fn get_allocator -->
Lets `AudioEncoder` sub-classes to know the memory `allocator`
used by the base class and its `params`.

Unref the `allocator` after use it.
## `allocator`
the `gst::Allocator`
used
## `params`
the
`gst::AllocationParams` of `allocator`
<!-- trait AudioEncoderExt::fn get_audio_info -->

# Returns

a `AudioInfo` describing the input audio format
<!-- trait AudioEncoderExt::fn get_drainable -->
Queries encoder drain handling.

# Returns

TRUE if drainable handling is enabled.

MT safe.
<!-- trait AudioEncoderExt::fn get_frame_max -->

# Returns

currently configured maximum handled frames
<!-- trait AudioEncoderExt::fn get_frame_samples_max -->

# Returns

currently maximum requested samples per frame
<!-- trait AudioEncoderExt::fn get_frame_samples_min -->

# Returns

currently minimum requested samples per frame
<!-- trait AudioEncoderExt::fn get_hard_min -->
Queries encoder hard minimum handling.

# Returns

TRUE if hard minimum handling is enabled.

MT safe.
<!-- trait AudioEncoderExt::fn get_latency -->
Sets the variables pointed to by `min` and `max` to the currently configured
latency.
## `min`
a pointer to storage to hold minimum latency
## `max`
a pointer to storage to hold maximum latency
<!-- trait AudioEncoderExt::fn get_lookahead -->

# Returns

currently configured encoder lookahead
<!-- trait AudioEncoderExt::fn get_mark_granule -->
Queries if the encoder will handle granule marking.

# Returns

TRUE if granule marking is enabled.

MT safe.
<!-- trait AudioEncoderExt::fn get_perfect_timestamp -->
Queries encoder perfect timestamp behaviour.

# Returns

TRUE if perfect timestamp setting enabled.

MT safe.
<!-- trait AudioEncoderExt::fn get_tolerance -->
Queries current audio jitter tolerance threshold.

# Returns

encoder audio jitter tolerance threshold.

MT safe.
<!-- trait AudioEncoderExt::fn merge_tags -->
Sets the audio encoder tags and how they should be merged with any
upstream stream tags. This will override any tags previously-set
with `AudioEncoderExt::merge_tags`.

Note that this is provided for convenience, and the subclass is
not required to use this and can still do tag handling on its own.

MT safe.
## `tags`
a `gst::TagList` to merge, or NULL to unset
 previously-set tags
## `mode`
the `gst::TagMergeMode` to use, usually `gst::TagMergeMode::Replace`
<!-- trait AudioEncoderExt::fn negotiate -->
Negotiate with downstream elements to currently configured `gst::Caps`.
Unmark GST_PAD_FLAG_NEED_RECONFIGURE in any case. But mark it again if
negotiate fails.

# Returns

`true` if the negotiation succeeded, else `false`.
<!-- trait AudioEncoderExt::fn proxy_getcaps -->
Returns caps that express `caps` (or sink template caps if `caps` == NULL)
restricted to channel/rate combinations supported by downstream elements
(e.g. muxers).
## `caps`
initial caps
## `filter`
filter caps

# Returns

a `gst::Caps` owned by caller
<!-- trait AudioEncoderExt::fn set_allocation_caps -->
Sets a caps in allocation query which are different from the set
pad's caps. Use this function before calling
`AudioEncoder::negotiate`. Setting to `None` the allocation
query will use the caps from the pad.

Feature: `v1_10`

## `allocation_caps`
a `gst::Caps` or `None`
<!-- trait AudioEncoderExt::fn set_drainable -->
Configures encoder drain handling. If drainable, subclass might
be handed a NULL buffer to have it return any leftover encoded data.
Otherwise, it is not considered so capable and will only ever be passed
real data.

MT safe.
## `enabled`
new state
<!-- trait AudioEncoderExt::fn set_frame_max -->
Sets max number of frames accepted at once (assumed minimally 1).
Requires `frame_samples_min` and `frame_samples_max` to be the equal.

Note: This value will be reset to 0 every time before
`AudioEncoderClass.set_format`() is called.
## `num`
number of frames
<!-- trait AudioEncoderExt::fn set_frame_samples_max -->
Sets number of samples (per channel) subclass needs to be handed,
at most or will be handed all available if 0.

If an exact number of samples is required, `AudioEncoderExt::set_frame_samples_min`
must be called with the same number.

Note: This value will be reset to 0 every time before
`AudioEncoderClass.set_format`() is called.
## `num`
number of samples per frame
<!-- trait AudioEncoderExt::fn set_frame_samples_min -->
Sets number of samples (per channel) subclass needs to be handed,
at least or will be handed all available if 0.

If an exact number of samples is required, `AudioEncoderExt::set_frame_samples_max`
must be called with the same number.

Note: This value will be reset to 0 every time before
`AudioEncoderClass.set_format`() is called.
## `num`
number of samples per frame
<!-- trait AudioEncoderExt::fn set_hard_min -->
Configures encoder hard minimum handling. If enabled, subclass
will never be handed less samples than it configured, which otherwise
might occur near end-of-data handling. Instead, the leftover samples
will simply be discarded.

MT safe.
## `enabled`
new state
<!-- trait AudioEncoderExt::fn set_headers -->
Set the codec headers to be sent downstream whenever requested.
## `headers`
a list of
 `gst::Buffer` containing the codec header
<!-- trait AudioEncoderExt::fn set_latency -->
Sets encoder latency.
## `min`
minimum latency
## `max`
maximum latency
<!-- trait AudioEncoderExt::fn set_lookahead -->
Sets encoder lookahead (in units of input rate samples)

Note: This value will be reset to 0 every time before
`AudioEncoderClass.set_format`() is called.
## `num`
lookahead
<!-- trait AudioEncoderExt::fn set_mark_granule -->
Enable or disable encoder granule handling.

MT safe.
## `enabled`
new state
<!-- trait AudioEncoderExt::fn set_output_format -->
Configure output caps on the srcpad of `self`.
## `caps`
`gst::Caps`

# Returns

`true` on success.
<!-- trait AudioEncoderExt::fn set_perfect_timestamp -->
Enable or disable encoder perfect output timestamp preference.

MT safe.
## `enabled`
new state
<!-- trait AudioEncoderExt::fn set_tolerance -->
Configures encoder audio jitter tolerance threshold.

MT safe.
## `tolerance`
new tolerance
<!-- enum AudioFormat -->
Enum value describing the most common audio formats.
<!-- enum AudioFormat::variant Unknown -->
unknown or unset audio format
<!-- enum AudioFormat::variant Encoded -->
encoded audio format
<!-- enum AudioFormat::variant S8 -->
8 bits in 8 bits, signed
<!-- enum AudioFormat::variant U8 -->
8 bits in 8 bits, unsigned
<!-- enum AudioFormat::variant S16le -->
16 bits in 16 bits, signed, little endian
<!-- enum AudioFormat::variant S16be -->
16 bits in 16 bits, signed, big endian
<!-- enum AudioFormat::variant U16le -->
16 bits in 16 bits, unsigned, little endian
<!-- enum AudioFormat::variant U16be -->
16 bits in 16 bits, unsigned, big endian
<!-- enum AudioFormat::variant S2432le -->
24 bits in 32 bits, signed, little endian
<!-- enum AudioFormat::variant S2432be -->
24 bits in 32 bits, signed, big endian
<!-- enum AudioFormat::variant U2432le -->
24 bits in 32 bits, unsigned, little endian
<!-- enum AudioFormat::variant U2432be -->
24 bits in 32 bits, unsigned, big endian
<!-- enum AudioFormat::variant S32le -->
32 bits in 32 bits, signed, little endian
<!-- enum AudioFormat::variant S32be -->
32 bits in 32 bits, signed, big endian
<!-- enum AudioFormat::variant U32le -->
32 bits in 32 bits, unsigned, little endian
<!-- enum AudioFormat::variant U32be -->
32 bits in 32 bits, unsigned, big endian
<!-- enum AudioFormat::variant S24le -->
24 bits in 24 bits, signed, little endian
<!-- enum AudioFormat::variant S24be -->
24 bits in 24 bits, signed, big endian
<!-- enum AudioFormat::variant U24le -->
24 bits in 24 bits, unsigned, little endian
<!-- enum AudioFormat::variant U24be -->
24 bits in 24 bits, unsigned, big endian
<!-- enum AudioFormat::variant S20le -->
20 bits in 24 bits, signed, little endian
<!-- enum AudioFormat::variant S20be -->
20 bits in 24 bits, signed, big endian
<!-- enum AudioFormat::variant U20le -->
20 bits in 24 bits, unsigned, little endian
<!-- enum AudioFormat::variant U20be -->
20 bits in 24 bits, unsigned, big endian
<!-- enum AudioFormat::variant S18le -->
18 bits in 24 bits, signed, little endian
<!-- enum AudioFormat::variant S18be -->
18 bits in 24 bits, signed, big endian
<!-- enum AudioFormat::variant U18le -->
18 bits in 24 bits, unsigned, little endian
<!-- enum AudioFormat::variant U18be -->
18 bits in 24 bits, unsigned, big endian
<!-- enum AudioFormat::variant F32le -->
32-bit floating point samples, little endian
<!-- enum AudioFormat::variant F32be -->
32-bit floating point samples, big endian
<!-- enum AudioFormat::variant F64le -->
64-bit floating point samples, little endian
<!-- enum AudioFormat::variant F64be -->
64-bit floating point samples, big endian
<!-- enum AudioFormat::variant S16 -->
16 bits in 16 bits, signed, native endianness
<!-- enum AudioFormat::variant U16 -->
16 bits in 16 bits, unsigned, native endianness
<!-- enum AudioFormat::variant S2432 -->
24 bits in 32 bits, signed, native endianness
<!-- enum AudioFormat::variant U2432 -->
24 bits in 32 bits, unsigned, native endianness
<!-- enum AudioFormat::variant S32 -->
32 bits in 32 bits, signed, native endianness
<!-- enum AudioFormat::variant U32 -->
32 bits in 32 bits, unsigned, native endianness
<!-- enum AudioFormat::variant S24 -->
24 bits in 24 bits, signed, native endianness
<!-- enum AudioFormat::variant U24 -->
24 bits in 24 bits, unsigned, native endianness
<!-- enum AudioFormat::variant S20 -->
20 bits in 24 bits, signed, native endianness
<!-- enum AudioFormat::variant U20 -->
20 bits in 24 bits, unsigned, native endianness
<!-- enum AudioFormat::variant S18 -->
18 bits in 24 bits, signed, native endianness
<!-- enum AudioFormat::variant U18 -->
18 bits in 24 bits, unsigned, native endianness
<!-- enum AudioFormat::variant F32 -->
32-bit floating point samples, native endianness
<!-- enum AudioFormat::variant F64 -->
64-bit floating point samples, native endianness
<!-- struct AudioFormatInfo -->
Information for an audio format.
<!-- struct AudioInfo -->
Information describing audio properties. This information can be filled
in from GstCaps with `AudioInfo::from_caps`.

Use the provided macros to access the info in this structure.
<!-- impl AudioInfo::fn new -->
Allocate a new `AudioInfo` that is also initialized with
`AudioInfo::init`.

# Returns

a new `AudioInfo`. free with `AudioInfo::free`.
<!-- impl AudioInfo::fn convert -->
Converts among various `gst::Format` types. This function handles
GST_FORMAT_BYTES, GST_FORMAT_TIME, and GST_FORMAT_DEFAULT. For
raw audio, GST_FORMAT_DEFAULT corresponds to audio frames. This
function can be used to handle pad queries of the type GST_QUERY_CONVERT.
## `src_fmt`
`gst::Format` of the `src_val`
## `src_val`
value to convert
## `dest_fmt`
`gst::Format` of the `dest_val`
## `dest_val`
pointer to destination value

# Returns

TRUE if the conversion was successful.
<!-- impl AudioInfo::fn copy -->
Copy a GstAudioInfo structure.

# Returns

a new `AudioInfo`. free with gst_audio_info_free.
<!-- impl AudioInfo::fn free -->
Free a GstAudioInfo structure previously allocated with `AudioInfo::new`
or `AudioInfo::copy`.
<!-- impl AudioInfo::fn from_caps -->
Parse `caps` and update `self`.
## `caps`
a `gst::Caps`

# Returns

TRUE if `caps` could be parsed
<!-- impl AudioInfo::fn init -->
Initialize `self` with default values.
<!-- impl AudioInfo::fn is_equal -->
Compares two `AudioInfo` and returns whether they are equal or not
## `other`
a `AudioInfo`

# Returns

`true` if `self` and `other` are equal, else `false`.
<!-- impl AudioInfo::fn set_format -->
Set the default info for the audio info of `format` and `rate` and `channels`.

Note: This initializes `self` first, no values are preserved.
## `format`
the format
## `rate`
the samplerate
## `channels`
the number of channels
## `position`
the channel positions
<!-- impl AudioInfo::fn to_caps -->
Convert the values of `self` into a `gst::Caps`.

# Returns

the new `gst::Caps` containing the
 info of `self`.
<!-- enum AudioLayout -->
Layout of the audio samples for the different channels.
<!-- enum AudioLayout::variant Interleaved -->
interleaved audio
<!-- enum AudioLayout::variant NonInterleaved -->
non-interleaved audio
<!-- enum AudioRingBufferFormatType -->
The format of the samples in the ringbuffer.
<!-- enum AudioRingBufferFormatType::variant Raw -->
samples in linear or float
<!-- enum AudioRingBufferFormatType::variant MuLaw -->
samples in mulaw
<!-- enum AudioRingBufferFormatType::variant ALaw -->
samples in alaw
<!-- enum AudioRingBufferFormatType::variant ImaAdpcm -->
samples in ima adpcm
<!-- enum AudioRingBufferFormatType::variant Mpeg -->
samples in mpeg audio (but not AAC) format
<!-- enum AudioRingBufferFormatType::variant Gsm -->
samples in gsm format
<!-- enum AudioRingBufferFormatType::variant Iec958 -->
samples in IEC958 frames (e.g. AC3)
<!-- enum AudioRingBufferFormatType::variant Ac3 -->
samples in AC3 format
<!-- enum AudioRingBufferFormatType::variant Eac3 -->
samples in EAC3 format
<!-- enum AudioRingBufferFormatType::variant Dts -->
samples in DTS format
<!-- enum AudioRingBufferFormatType::variant Mpeg2Aac -->
samples in MPEG-2 AAC ADTS format
<!-- enum AudioRingBufferFormatType::variant Mpeg4Aac -->
samples in MPEG-4 AAC ADTS format
<!-- enum AudioRingBufferFormatType::variant Mpeg2AacRaw -->
samples in MPEG-2 AAC raw format (Since: 1.12)
<!-- enum AudioRingBufferFormatType::variant Mpeg4AacRaw -->
samples in MPEG-4 AAC raw format (Since: 1.12)
<!-- enum AudioRingBufferFormatType::variant Flac -->
samples in FLAC format (Since: 1.12)
<!-- struct AudioSink -->
This is the most simple base class for audio sinks that only requires
subclasses to implement a set of simple functions:

* `open()` :Open the device.

* `prepare()` :Configure the device with the specified format.

* `write()` :Write samples to the device.

* `reset()` :Unblock writes and flush the device.

* `delay()` :Get the number of samples written but not yet played
by the device.

* `unprepare()` :Undo operations done by prepare.

* `close()` :Close the device.

All scheduling of samples and timestamps is done in this base class
together with `AudioBaseSink` using a default implementation of a
`AudioRingBuffer` that uses threads.

# Implements

[`AudioBaseSinkExt`](trait.AudioBaseSinkExt.html), [`gst_base::BaseSinkExt`](../gst_base/trait.BaseSinkExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- struct AudioSrc -->
This is the most simple base class for audio sources that only requires
subclasses to implement a set of simple functions:

* `open()` :Open the device.
* `prepare()` :Configure the device with the specified format.
* `read()` :Read samples from the device.
* `reset()` :Unblock reads and flush the device.
* `delay()` :Get the number of samples in the device but not yet read.
* `unprepare()` :Undo operations done by prepare.
* `close()` :Close the device.

All scheduling of samples and timestamps is done in this base class
together with `AudioBaseSrc` using a default implementation of a
`AudioRingBuffer` that uses threads.

# Implements

[`AudioBaseSrcExt`](trait.AudioBaseSrcExt.html), [`gst_base::BaseSrcExt`](../gst_base/trait.BaseSrcExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- struct AudioStreamAlign -->
`AudioStreamAlign` provides a helper object that helps tracking audio
stream alignment and discontinuities, and detects discontinuities if
possible.

See `AudioStreamAlign::new` for a description of its parameters and
`AudioStreamAlign::process` for the details of the processing.

Feature: `v1_14`
<!-- impl AudioStreamAlign::fn new -->
Allocate a new `AudioStreamAlign` with the given configuration. All
processing happens according to sample rate `rate`, until
`AudioStreamAlign::set_rate` is called with a new `rate`.
A negative rate can be used for reverse playback.

`alignment_threshold` gives the tolerance in nanoseconds after which a
timestamp difference is considered a discontinuity. Once detected,
`discont_wait` nanoseconds have to pass without going below the threshold
again until the output buffer is marked as a discontinuity. These can later
be re-configured with `AudioStreamAlign::set_alignment_threshold` and
`AudioStreamAlign::set_discont_wait`.

Feature: `v1_14`

## `rate`
a sample rate
## `alignment_threshold`
a alignment threshold in nanoseconds
## `discont_wait`
discont wait in nanoseconds

# Returns

a new `AudioStreamAlign`. free with `AudioStreamAlign::free`.
<!-- impl AudioStreamAlign::fn copy -->
Copy a GstAudioStreamAlign structure.

Feature: `v1_14`


# Returns

a new `AudioStreamAlign`. free with gst_audio_stream_align_free.
<!-- impl AudioStreamAlign::fn free -->
Free a GstAudioStreamAlign structure previously allocated with `AudioStreamAlign::new`
or `AudioStreamAlign::copy`.

Feature: `v1_14`

<!-- impl AudioStreamAlign::fn get_alignment_threshold -->
Gets the currently configured alignment threshold.

Feature: `v1_14`


# Returns

The currently configured alignment threshold
<!-- impl AudioStreamAlign::fn get_discont_wait -->
Gets the currently configured discont wait.

Feature: `v1_14`


# Returns

The currently configured discont wait
<!-- impl AudioStreamAlign::fn get_rate -->
Gets the currently configured sample rate.

Feature: `v1_14`


# Returns

The currently configured sample rate
<!-- impl AudioStreamAlign::fn get_samples_since_discont -->
Returns the number of samples that were processed since the last
discontinuity was detected.

Feature: `v1_14`


# Returns

The number of samples processed since the last discontinuity.
<!-- impl AudioStreamAlign::fn get_timestamp_at_discont -->
Timestamp that was passed when a discontinuity was detected, i.e. the first
timestamp after the discontinuity.

Feature: `v1_14`


# Returns

The last timestamp at when a discontinuity was detected
<!-- impl AudioStreamAlign::fn mark_discont -->
Marks the next buffer as discontinuous and resets timestamp tracking.

Feature: `v1_14`

<!-- impl AudioStreamAlign::fn process -->
Processes data with `timestamp` and `n_samples`, and returns the output
timestamp, duration and sample position together with a boolean to signal
whether a discontinuity was detected or not. All non-discontinuous data
will have perfect timestamps and durations.

A discontinuity is detected once the difference between the actual
timestamp and the timestamp calculated from the sample count since the last
discontinuity differs by more than the alignment threshold for a duration
longer than discont wait.

Note: In reverse playback, every buffer is considered discontinuous in the
context of buffer flags because the last sample of the previous buffer is
discontinuous with the first sample of the current one. However for this
function they are only considered discontinuous in reverse playback if the
first sample of the previous buffer is discontinuous with the last sample
of the current one.

Feature: `v1_14`

## `discont`
if this data is considered to be discontinuous
## `timestamp`
a `gst::ClockTime` of the start of the data
## `n_samples`
number of samples to process
## `out_timestamp`
output timestamp of the data
## `out_duration`
output duration of the data
## `out_sample_position`
output sample position of the start of the data

# Returns

`true` if a discontinuity was detected, `false` otherwise.
<!-- impl AudioStreamAlign::fn set_alignment_threshold -->
Sets `alignment_treshold` as new alignment threshold for the following processing.

Feature: `v1_14`

## `alignment_threshold`
a new alignment threshold
<!-- impl AudioStreamAlign::fn set_discont_wait -->
Sets `alignment_treshold` as new discont wait for the following processing.

Feature: `v1_14`

## `discont_wait`
a new discont wait
<!-- impl AudioStreamAlign::fn set_rate -->
Sets `rate` as new sample rate for the following processing. If the sample
rate differs this implicitely marks the next data as discontinuous.

Feature: `v1_14`

## `rate`
a new sample rate
<!-- struct StreamVolume -->
This interface is implemented by elements that provide a stream volume. Examples for
such elements are `volume` and `playbin`.

Applications can use this interface to get or set the current stream volume. For this
the "volume" `gobject::Object` property can be used or the helper functions `StreamVolume::set_volume`
and `StreamVolume::get_volume`. This volume is always a linear factor, i.e. 0.0 is muted
1.0 is 100%. For showing the volume in a GUI it might make sense to convert it to
a different format by using `StreamVolume::convert_volume`. Volume sliders should usually
use a cubic volume.

Separate from the volume the stream can also be muted by the "mute" `gobject::Object` property or
`StreamVolume::set_mute` and `StreamVolume::get_mute`.

Elements that provide some kind of stream volume should implement the "volume" and
"mute" `gobject::Object` properties and handle setting and getting of them properly.
The volume property is defined to be a linear volume factor.

# Implements

[`StreamVolumeExt`](trait.StreamVolumeExt.html)
<!-- trait StreamVolumeExt -->
Trait containing all `StreamVolume` methods.

# Implementors

[`StreamVolume`](struct.StreamVolume.html)
<!-- impl StreamVolume::fn convert_volume -->
## `from`
`StreamVolumeFormat` to convert from
## `to`
`StreamVolumeFormat` to convert to
## `val`
Volume in `from` format that should be converted

# Returns

the converted volume
<!-- trait StreamVolumeExt::fn get_mute -->

# Returns

Returns `true` if the stream is muted
<!-- trait StreamVolumeExt::fn get_volume -->
## `format`
`StreamVolumeFormat` which should be returned

# Returns

The current stream volume as linear factor
<!-- trait StreamVolumeExt::fn set_mute -->
## `mute`
Mute state that should be set
<!-- trait StreamVolumeExt::fn set_volume -->
## `format`
`StreamVolumeFormat` of `val`
## `val`
Linear volume factor that should be set
<!-- enum StreamVolumeFormat -->
Different representations of a stream volume. `StreamVolume::convert_volume`
allows to convert between the different representations.

Formulas to convert from a linear to a cubic or dB volume are
cbrt(val) and 20 * log10 (val).
<!-- enum StreamVolumeFormat::variant Linear -->
Linear scale factor, 1.0 = 100%
<!-- enum StreamVolumeFormat::variant Cubic -->
Cubic volume scale
<!-- enum StreamVolumeFormat::variant Db -->
Logarithmic volume scale (dB, amplitude not power)
