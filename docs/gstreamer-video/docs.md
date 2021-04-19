<!-- file * -->
<!-- enum VideoAFDSpec -->
Enumeration of the different standards that may apply to AFD data:

0) ETSI/DVB:
https://www.etsi.org/deliver/etsi_ts/101100_101199/101154/02.01.01_60/ts_101154v020101p.pdf

1) ATSC A/53:
https://www.atsc.org/wp-content/uploads/2015/03/a_53-Part-4-2009.pdf

2) SMPTE ST2016-1:
<!-- enum VideoAFDSpec::variant DvbEtsi -->
AFD value is from DVB/ETSI standard
<!-- enum VideoAFDSpec::variant AtscA53 -->
AFD value is from ATSC A/53 standard

Feature: `v1_18`

<!-- enum VideoAFDValue -->
Enumeration of the various values for Active Format Description (AFD)

AFD should be included in video user data whenever the rectangular
picture area containing useful information does not extend to the full height or width of the coded
frame. AFD data may also be included in user data when the rectangular picture area containing
useful information extends to the full height and width of the coded frame.

For details, see Table 6.14 Active Format in:

ATSC Digital Television Standard:
Part 4 – MPEG-2 Video System Characteristics

https://www.atsc.org/wp-content/uploads/2015/03/a_53-Part-4-2009.pdf

and Active Format Description in Complete list of AFD codes

https://en.wikipedia.org/wiki/Active_Format_Description`Complete_list_of_AFD_codes`

and SMPTE ST2016-1

Notes:

1) AFD 0 is undefined for ATSC and SMPTE ST2016-1, indicating that AFD data is not available:
If Bar Data is not present, AFD '0000' indicates that exact information
is not available and the active image should be assumed to be the same as the coded frame. AFD '0000'.
AFD '0000' accompanied by Bar Data signals that the active image’s aspect ratio is narrower than 16:9,
but is not 4:3 or 14:9. As the exact aspect ratio cannot be conveyed by AFD alone, wherever possible,
AFD ‘0000’ should be accompanied by Bar Data to define the exact vertical or horizontal extent
of the active image.
2) AFD 0 is reserved for DVB/ETSI
3) values 1, 5, 6, 7, and 12 are reserved for both ATSC and DVB/ETSI
4) values 2 and 3 are not recommended for ATSC, but are valid for DVB/ETSI
<!-- enum VideoAFDValue::variant Unavailable -->
Unavailable (see note 0 below).
<!-- enum VideoAFDValue::variant 169TopAligned -->
For 4:3 coded frame, letterbox 16:9 image,
 at top of the coded frame. For 16:9 coded frame, full frame 16:9 image,
 the same as the coded frame.
<!-- enum VideoAFDValue::variant 149TopAligned -->
For 4:3 coded frame, letterbox 14:9 image,
 at top of the coded frame. For 16:9 coded frame, pillarbox 14:9 image,
 horizontally centered in the coded frame.
<!-- enum VideoAFDValue::variant GreaterThan169 -->
For 4:3 coded frame, letterbox image with an aspect ratio
 greater than 16:9, vertically centered in the coded frame. For 16:9 coded frame,
 letterbox image with an aspect ratio greater than 16:9.
<!-- enum VideoAFDValue::variant 43Full169Full -->
For 4:3 coded frame, full frame 4:3 image,
 the same as the coded frame. For 16:9 coded frame, full frame 16:9 image, the same as
 the coded frame.
<!-- enum VideoAFDValue::variant 43Full43Pillar -->
For 4:3 coded frame, full frame 4:3 image, the same as
 the coded frame. For 16:9 coded frame, pillarbox 4:3 image, horizontally centered in the
 coded frame.
<!-- enum VideoAFDValue::variant 169Letter169Full -->
For 4:3 coded frame, letterbox 16:9 image, vertically centered in
 the coded frame with all image areas protected. For 16:9 coded frame, full frame 16:9 image,
 with all image areas protected.
<!-- enum VideoAFDValue::variant 149Letter149Pillar -->
For 4:3 coded frame, letterbox 14:9 image, vertically centered in
 the coded frame. For 16:9 coded frame, pillarbox 14:9 image, horizontally centered in the
 coded frame.
<!-- enum VideoAFDValue::variant 43Full149Center -->
For 4:3 coded frame, full frame 4:3 image, with alternative 14:9
 center. For 16:9 coded frame, pillarbox 4:3 image, with alternative 14:9 center.
<!-- enum VideoAFDValue::variant 169Letter149Center -->
For 4:3 coded frame, letterbox 16:9 image, with alternative 14:9
 center. For 16:9 coded frame, full frame 16:9 image, with alternative 14:9 center.
<!-- enum VideoAFDValue::variant 169Letter43Center -->
For 4:3 coded frame, letterbox 16:9 image, with alternative 4:3
 center. For 16:9 coded frame, full frame 16:9 image, with alternative 4:3 center.

Feature: `v1_18`

<!-- enum VideoAlphaMode -->
Different alpha modes.
<!-- enum VideoAlphaMode::variant Copy -->
When input and output have alpha, it will be copied.
 When the input has no alpha, alpha will be set to
 `GST_VIDEO_CONVERTER_OPT_ALPHA_VALUE`
<!-- enum VideoAlphaMode::variant Set -->
set all alpha to
 `GST_VIDEO_CONVERTER_OPT_ALPHA_VALUE`
<!-- enum VideoAlphaMode::variant Mult -->
multiply all alpha with
 `GST_VIDEO_CONVERTER_OPT_ALPHA_VALUE`.
 When the input format has no alpha but the output format has, the
 alpha value will be set to `GST_VIDEO_CONVERTER_OPT_ALPHA_VALUE`
<!-- struct VideoBufferFlags -->
Additional video buffer flags. These flags can potentially be used on any
buffers carrying closed caption data, or video data - even encoded data.

Note that these are only valid for `gst::Caps` of type: video/... and caption/...
They can conflict with other extended buffer flags.
<!-- struct VideoBufferFlags::const INTERLACED -->
If the `gst::Buffer` is interlaced. In mixed
 interlace-mode, this flags specifies if the frame is
 interlaced or progressive.
<!-- struct VideoBufferFlags::const TFF -->
If the `gst::Buffer` is interlaced, then the first field
 in the video frame is the top field. If unset, the
 bottom field is first.
<!-- struct VideoBufferFlags::const RFF -->
If the `gst::Buffer` is interlaced, then the first field
 (as defined by the `VideoBufferFlags::Tff` flag setting)
 is repeated.
<!-- struct VideoBufferFlags::const ONEFIELD -->
If the `gst::Buffer` is interlaced, then only the
 first field (as defined by the `VideoBufferFlags::Tff`
 flag setting) is to be displayed (Since: 1.16).
<!-- struct VideoBufferFlags::const MULTIPLE_VIEW -->
The `gst::Buffer` contains one or more specific views,
 such as left or right eye view. This flags is set on
 any buffer that contains non-mono content - even for
 streams that contain only a single viewpoint. In mixed
 mono / non-mono streams, the absence of the flag marks
 mono buffers.
<!-- struct VideoBufferFlags::const FIRST_IN_BUNDLE -->
When conveying stereo/multiview content with
 frame-by-frame methods, this flag marks the first buffer
 in a bundle of frames that belong together.
<!-- struct VideoBufferFlags::const TOP_FIELD -->
The video frame has the top field only. This is the
 same as GST_VIDEO_BUFFER_FLAG_TFF |
 GST_VIDEO_BUFFER_FLAG_ONEFIELD (Since: 1.16).
 Use GST_VIDEO_BUFFER_IS_TOP_FIELD() to check for this flag.
<!-- struct VideoBufferFlags::const BOTTOM_FIELD -->
The video frame has the bottom field only. This is
 the same as GST_VIDEO_BUFFER_FLAG_ONEFIELD
 (GST_VIDEO_BUFFER_FLAG_TFF flag unset) (Since: 1.16).
 Use GST_VIDEO_BUFFER_IS_BOTTOM_FIELD() to check for this flag.
<!-- struct VideoBufferFlags::const MARKER -->
The `gst::Buffer` contains the end of a video field or frame
 boundary such as the last subframe or packet (Since: 1.18).
<!-- struct VideoBufferFlags::const LAST -->
Offset to define more flags
<!-- struct VideoBufferPool -->


# Implements

[`trait@gst::BufferPoolExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- impl VideoBufferPool::fn new -->
Create a new bufferpool that can allocate video frames. This bufferpool
supports all the video bufferpool options.

# Returns

a new `gst::BufferPool` to allocate video frames
<!-- enum VideoCaptionType -->
The various known types of Closed Caption (CC).
<!-- enum VideoCaptionType::variant Unknown -->
Unknown type of CC
<!-- enum VideoCaptionType::variant Cea608Raw -->
CEA-608 as byte pairs. Note that
 this format is not recommended since is does not specify to
 which field the caption comes from and therefore assumes
 it comes from the first field (and that there is no information
 on the second field). Use [`Cea708Raw`](Self::Cea708Raw)
 if you wish to store CEA-608 from two fields and prefix each byte pair
 with 0xFC for the first field and 0xFD for the second field.
<!-- enum VideoCaptionType::variant Cea608S3341a -->
CEA-608 as byte triplets as defined
 in SMPTE S334-1 Annex A. The second and third byte of the byte triplet
 is the raw CEA608 data, the first byte is a bitfield: The top/7th bit is
 0 for the second field, 1 for the first field, bit 6 and 5 are 0 and
 bits 4 to 0 are a 5 bit unsigned integer that represents the line
 offset relative to the base-line of the original image format (line 9
 for 525-line field 1, line 272 for 525-line field 2, line 5 for
 625-line field 1 and line 318 for 625-line field 2).
<!-- enum VideoCaptionType::variant Cea708Raw -->
CEA-708 as cc_data byte triplets. They
 can also contain 608-in-708 and the first byte of each triplet has to
 be inspected for detecting the type.
<!-- enum VideoCaptionType::variant Cea708Cdp -->
CEA-708 (and optionally CEA-608) in
 a CDP (Caption Distribution Packet) defined by SMPTE S-334-2.
 Contains the whole CDP (starting with 0x9669).

Feature: `v1_16`

<!-- enum VideoChromaMode -->
Different chroma downsampling and upsampling modes
<!-- enum VideoChromaMode::variant Full -->
do full chroma up and down sampling
<!-- enum VideoChromaMode::variant UpsampleOnly -->
only perform chroma upsampling
<!-- enum VideoChromaMode::variant DownsampleOnly -->
only perform chroma downsampling
<!-- enum VideoChromaMode::variant None -->
disable chroma resampling
<!-- struct VideoChromaSite -->
Various Chroma sitings.
<!-- struct VideoChromaSite::const UNKNOWN -->
unknown cositing
<!-- struct VideoChromaSite::const NONE -->
no cositing
<!-- struct VideoChromaSite::const H_COSITED -->
chroma is horizontally cosited
<!-- struct VideoChromaSite::const V_COSITED -->
chroma is vertically cosited
<!-- struct VideoChromaSite::const ALT_LINE -->
choma samples are sited on alternate lines
<!-- struct VideoChromaSite::const COSITED -->
chroma samples cosited with luma samples
<!-- struct VideoChromaSite::const JPEG -->
jpeg style cositing, also for mpeg1 and mjpeg
<!-- struct VideoChromaSite::const MPEG2 -->
mpeg2 style cositing
<!-- struct VideoChromaSite::const DV -->
DV style cositing
<!-- struct VideoCodecFrame -->
A `VideoCodecFrame` represents a video frame both in raw and
encoded form.
<!-- impl VideoCodecFrame::fn get_user_data -->
Gets private data set on the frame by the subclass via
`VideoCodecFrame::set_user_data` previously.

# Returns

The previously set user_data
<!-- impl VideoCodecFrame::fn ref -->
Increases the refcount of the given frame by one.

# Returns

`buf`
<!-- impl VideoCodecFrame::fn set_user_data -->
Sets `user_data` on the frame and the `GDestroyNotify` that will be called when
the frame is freed. Allows to attach private data by the subclass to frames.

If a `user_data` was previously set, then the previous set `notify` will be called
before the `user_data` is replaced.
## `user_data`
private data
## `notify`
a `GDestroyNotify`
<!-- impl VideoCodecFrame::fn unref -->
Decreases the refcount of the frame. If the refcount reaches 0, the frame
will be freed.
<!-- struct VideoCodecFrameFlags -->
Flags for `VideoCodecFrame`
<!-- struct VideoCodecFrameFlags::const DECODE_ONLY -->
is the frame only meant to be decoded
<!-- struct VideoCodecFrameFlags::const SYNC_POINT -->
is the frame a synchronization point (keyframe)
<!-- struct VideoCodecFrameFlags::const FORCE_KEYFRAME -->
should the output frame be made a keyframe
<!-- struct VideoCodecFrameFlags::const FORCE_KEYFRAME_HEADERS -->
should the encoder output stream headers
<!-- struct VideoCodecState -->
Structure representing the state of an incoming or outgoing video
stream for encoders and decoders.

Decoders and encoders will receive such a state through their
respective `set_format` vmethods.

Decoders and encoders can set the downstream state, by using the
[`crate::VideoDecoder::set_output_state`] (XXX: @-reference does not belong to VideoCodecState!)() or
[`crate::VideoEncoder::set_output_state`] (XXX: @-reference does not belong to VideoCodecState!)() methods.
<!-- impl VideoCodecState::fn ref -->
Increases the refcount of the given state by one.

# Returns

`buf`
<!-- impl VideoCodecState::fn unref -->
Decreases the refcount of the state. If the refcount reaches 0, the state
will be freed.
<!-- enum VideoColorMatrix -->
The color matrix is used to convert between Y'PbPr and
non-linear RGB (R'G'B')
<!-- enum VideoColorMatrix::variant Unknown -->
unknown matrix
<!-- enum VideoColorMatrix::variant Rgb -->
identity matrix. Order of coefficients is
actually GBR, also IEC 61966-2-1 (sRGB)
<!-- enum VideoColorMatrix::variant Fcc -->
FCC Title 47 Code of Federal Regulations 73.682 (a)(20)
<!-- enum VideoColorMatrix::variant Bt709 -->
ITU-R BT.709 color matrix, also ITU-R BT1361
/ IEC 61966-2-4 xvYCC709 / SMPTE RP177 Annex B
<!-- enum VideoColorMatrix::variant Bt601 -->
ITU-R BT.601 color matrix, also SMPTE170M / ITU-R BT1358 525 / ITU-R BT1700 NTSC
<!-- enum VideoColorMatrix::variant Smpte240m -->
SMPTE 240M color matrix
<!-- enum VideoColorMatrix::variant Bt2020 -->
ITU-R BT.2020 color matrix. Since: 1.6
<!-- enum VideoColorPrimaries -->
The color primaries define the how to transform linear RGB values to and from
the CIE XYZ colorspace.
<!-- enum VideoColorPrimaries::variant Unknown -->
unknown color primaries
<!-- enum VideoColorPrimaries::variant Bt709 -->
BT709 primaries, also ITU-R BT1361 / IEC
61966-2-4 / SMPTE RP177 Annex B
<!-- enum VideoColorPrimaries::variant Bt470m -->
BT470M primaries, also FCC Title 47 Code
of Federal Regulations 73.682 (a)(20)
<!-- enum VideoColorPrimaries::variant Bt470bg -->
BT470BG primaries, also ITU-R BT601-6
625 / ITU-R BT1358 625 / ITU-R BT1700 625 PAL & SECAM
<!-- enum VideoColorPrimaries::variant Smpte170m -->
SMPTE170M primaries, also ITU-R
BT601-6 525 / ITU-R BT1358 525 / ITU-R BT1700 NTSC
<!-- enum VideoColorPrimaries::variant Smpte240m -->
SMPTE240M primaries
<!-- enum VideoColorPrimaries::variant Film -->
Generic film (colour filters using
Illuminant C)
<!-- enum VideoColorPrimaries::variant Bt2020 -->
ITU-R BT2020 primaries. Since: 1.6
<!-- enum VideoColorPrimaries::variant Adobergb -->
Adobe RGB primaries. Since: 1.8
<!-- enum VideoColorPrimaries::variant Smptest428 -->
SMPTE ST 428 primaries (CIE 1931
XYZ). Since: 1.16
<!-- enum VideoColorPrimaries::variant Smpterp431 -->
SMPTE RP 431 primaries (ST 431-2
(2011) / DCI P3). Since: 1.16
<!-- enum VideoColorPrimaries::variant Smpteeg432 -->
SMPTE EG 432 primaries (ST 432-1
(2010) / P3 D65). Since: 1.16
<!-- enum VideoColorPrimaries::variant Ebu3213 -->
EBU 3213 primaries (JEDEC P22
phosphors). Since: 1.16
<!-- enum VideoColorRange -->
Possible color range values. These constants are defined for 8 bit color
values and can be scaled for other bit depths.
<!-- enum VideoColorRange::variant Unknown -->
unknown range
<!-- enum VideoColorRange::variant 0255 -->
[0..255] for 8 bit components
<!-- enum VideoColorRange::variant 16235 -->
[16..235] for 8 bit components. Chroma has
 [16..240] range.
<!-- struct VideoColorimetry -->
Structure describing the color info.
<!-- impl VideoColorimetry::fn from_string -->
Parse the colorimetry string and update `self` with the parsed
values.
## `color`
a colorimetry string

# Returns

`true` if `color` points to valid colorimetry info.
<!-- impl VideoColorimetry::fn is_equal -->
Compare the 2 colorimetry sets for equality
## `other`
another `VideoColorimetry`

# Returns

`true` if `self` and `other` are equal.
<!-- impl VideoColorimetry::fn matches -->
Check if the colorimetry information in `info` matches that of the
string `color`.
## `color`
a colorimetry string

# Returns

`true` if `color` conveys the same colorimetry info as the color
information in `info`.
<!-- impl VideoColorimetry::fn to_string -->
Make a string representation of `self`.

# Returns

a string representation of `self`
or `None` if all the entries of `self` are unknown values.
<!-- struct VideoDecoder -->
This base class is for video decoders turning encoded data into raw video
frames.

The GstVideoDecoder base class and derived subclasses should cooperate as
follows:

## Configuration

 * Initially, GstVideoDecoder calls `start` when the decoder element
 is activated, which allows the subclass to perform any global setup.

 * GstVideoDecoder calls `set_format` to inform the subclass of caps
 describing input video data that it is about to receive, including
 possibly configuration data.
 While unlikely, it might be called more than once, if changing input
 parameters require reconfiguration.

 * Incoming data buffers are processed as needed, described in Data
 Processing below.

 * GstVideoDecoder calls `stop` at end of all processing.

## Data processing

 * The base class gathers input data, and optionally allows subclass
 to parse this into subsequently manageable chunks, typically
 corresponding to and referred to as 'frames'.

 * Each input frame is provided in turn to the subclass' `handle_frame`
 callback.
 The ownership of the frame is given to the `handle_frame` callback.

 * If codec processing results in decoded data, the subclass should call
 [`finish_frame`](Self::finish_frame) to have decoded data pushed.
 downstream. Otherwise, the subclass must call
 [`drop_frame`](Self::drop_frame), to allow the base class to do timestamp
 and offset tracking, and possibly to requeue the frame for a later
 attempt in the case of reverse playback.

## Shutdown phase

 * The GstVideoDecoder class calls `stop` to inform the subclass that data
 parsing will be stopped.

## Additional Notes

 * Seeking/Flushing

 * When the pipeline is seeked or otherwise flushed, the subclass is
 informed via a call to its `reset` callback, with the hard parameter
 set to true. This indicates the subclass should drop any internal data
 queues and timestamps and prepare for a fresh set of buffers to arrive
 for parsing and decoding.

 * End Of Stream

 * At end-of-stream, the subclass `parse` function may be called some final
 times with the at_eos parameter set to true, indicating that the element
 should not expect any more data to be arriving, and it should parse and
 remaining frames and call `VideoDecoder::have_frame` if possible.

The subclass is responsible for providing pad template caps for
source and sink pads. The pads need to be named "sink" and "src". It also
needs to provide information about the output caps, when they are known.
This may be when the base class calls the subclass' `set_format` function,
though it might be during decoding, before calling
[`finish_frame`](Self::finish_frame). This is done via
[`set_output_state`](Self::set_output_state)

The subclass is also responsible for providing (presentation) timestamps
(likely based on corresponding input ones). If that is not applicable
or possible, the base class provides limited framerate based interpolation.

Similarly, the base class provides some limited (legacy) seeking support
if specifically requested by the subclass, as full-fledged support
should rather be left to upstream demuxer, parser or alike. This simple
approach caters for seeking and duration reporting using estimated input
bitrates. To enable it, a subclass should call
[`crate::VideoDecoderExt::set_estimate_rate`] (XXX: @-reference does not belong to VideoDecoder!) to enable handling of incoming
byte-streams.

The base class provides some support for reverse playback, in particular
in case incoming data is not packetized or upstream does not provide
fragments on keyframe boundaries. However, the subclass should then be
prepared for the parsing and frame processing stage to occur separately
(in normal forward processing, the latter immediately follows the former),
The subclass also needs to ensure the parsing stage properly marks
keyframes, unless it knows the upstream elements will do so properly for
incoming data.

The bare minimum that a functional subclass needs to implement is:

 * Provide pad templates
 * Inform the base class of output caps via
 [`set_output_state`](Self::set_output_state)

 * Parse input data, if it is not considered packetized from upstream
 Data will be provided to `parse` which should invoke
 [`crate::VideoDecoderExt::add_to_frame`] (XXX: @-reference does not belong to VideoDecoder!) and [`have_frame`](Self::have_frame) to
 separate the data belonging to each video frame.

 * Accept data in `handle_frame` and provide decoded results to
 [`finish_frame`](Self::finish_frame), or call [`drop_frame`](Self::drop_frame).

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`VideoDecoderExt`](trait@crate::VideoDecoderExt), [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`], [`VideoDecoderExtManual`](trait@crate::VideoDecoderExtManual)
<!-- trait VideoDecoderExt -->
Trait containing all `VideoDecoder` methods.

# Implementors

[`VideoDecoder`](struct@crate::VideoDecoder)
<!-- trait VideoDecoderExt::fn add_to_frame -->
Removes next `n_bytes` of input data and adds it to currently parsed frame.
## `n_bytes`
the number of bytes to add
<!-- trait VideoDecoderExt::fn allocate_output_buffer -->
Helper function that allocates a buffer to hold a video frame for `self`'s
current `VideoCodecState`.

You should use `VideoDecoder::allocate_output_frame` instead of this
function, if possible at all.

# Returns

allocated buffer, or NULL if no buffer could be
 allocated (e.g. when downstream is flushing or shutting down)
<!-- trait VideoDecoderExt::fn allocate_output_frame -->
Helper function that allocates a buffer to hold a video frame for `self`'s
current `VideoCodecState`. Subclass should already have configured video
state and set src pad caps.

The buffer allocated here is owned by the frame and you should only
keep references to the frame, not the buffer.
## `frame`
a `VideoCodecFrame`

# Returns

`gst::FlowReturn::Ok` if an output buffer could be allocated
<!-- trait VideoDecoderExt::fn allocate_output_frame_with_params -->
Same as `VideoDecoder::allocate_output_frame` except it allows passing
`gst::BufferPoolAcquireParams` to the sub call gst_buffer_pool_acquire_buffer.

Feature: `v1_12`

## `frame`
a `VideoCodecFrame`
## `params`
a `gst::BufferPoolAcquireParams`

# Returns

`gst::FlowReturn::Ok` if an output buffer could be allocated
<!-- trait VideoDecoderExt::fn drop_frame -->
Similar to `VideoDecoder::finish_frame`, but drops `frame` in any
case and posts a QoS message with the frame's details on the bus.
In any case, the frame is considered finished and released.
## `frame`
the `VideoCodecFrame` to drop

# Returns

a `gst::FlowReturn`, usually GST_FLOW_OK.
<!-- trait VideoDecoderExt::fn finish_frame -->
`frame` should have a valid decoded data buffer, whose metadata fields
are then appropriately set according to frame data and pushed downstream.
If no output data is provided, `frame` is considered skipped.
In any case, the frame is considered finished and released.

After calling this function the output buffer of the frame is to be
considered read-only. This function will also change the metadata
of the buffer.
## `frame`
a decoded `VideoCodecFrame`

# Returns

a `gst::FlowReturn` resulting from sending data downstream
<!-- trait VideoDecoderExt::fn allocator -->
Lets `VideoDecoder` sub-classes to know the memory `allocator`
used by the base class and its `params`.

Unref the `allocator` after use it.
## `allocator`
the `gst::Allocator`
used
## `params`
the
`gst::AllocationParams` of `allocator`
<!-- trait VideoDecoderExt::fn buffer_pool -->

# Returns

the instance of the `gst::BufferPool` used
by the decoder; free it after use it
<!-- trait VideoDecoderExt::fn estimate_rate -->

# Returns

currently configured byte to time conversion setting
<!-- trait VideoDecoderExt::fn frame -->
Get a pending unfinished `VideoCodecFrame`
## `frame_number`
system_frame_number of a frame

# Returns

pending unfinished `VideoCodecFrame` identified by `frame_number`.
<!-- trait VideoDecoderExt::fn frames -->
Get all pending unfinished `VideoCodecFrame`

# Returns

pending unfinished `VideoCodecFrame`.
<!-- trait VideoDecoderExt::fn latency -->
Query the configured decoder latency. Results will be returned via
`min_latency` and `max_latency`.
## `min_latency`
address of variable in which to store the
 configured minimum latency, or `None`
## `max_latency`
address of variable in which to store the
 configured mximum latency, or `None`
<!-- trait VideoDecoderExt::fn max_decode_time -->
Determines maximum possible decoding time for `frame` that will
allow it to decode and arrive in time (as determined by QoS events).
In particular, a negative result means decoding in time is no longer possible
and should therefore occur as soon/skippy as possible.
## `frame`
a `VideoCodecFrame`

# Returns

max decoding time.
<!-- trait VideoDecoderExt::fn max_errors -->

# Returns

currently configured decoder tolerated error count.
<!-- trait VideoDecoderExt::fn needs_format -->
Queries decoder required format handling.

# Returns

`true` if required format handling is enabled.
<!-- trait VideoDecoderExt::fn oldest_frame -->
Get the oldest pending unfinished `VideoCodecFrame`

# Returns

oldest pending unfinished `VideoCodecFrame`.
<!-- trait VideoDecoderExt::fn output_state -->
Get the `VideoCodecState` currently describing the output stream.

# Returns

`VideoCodecState` describing format of video data.
<!-- trait VideoDecoderExt::fn is_packetized -->
Queries whether input data is considered packetized or not by the
base class.

# Returns

TRUE if input data is considered packetized.
<!-- trait VideoDecoderExt::fn pending_frame_size -->
Returns the number of bytes previously added to the current frame
by calling `VideoDecoderExt::add_to_frame`.

# Returns

The number of bytes pending for the current frame
<!-- trait VideoDecoderExt::fn qos_proportion -->

# Returns

The current QoS proportion.
<!-- trait VideoDecoderExt::fn have_frame -->
Gathers all data collected for currently parsed frame, gathers corresponding
metadata and passes it along for further processing, i.e. `handle_frame`.

# Returns

a `gst::FlowReturn`
<!-- trait VideoDecoderExt::fn merge_tags -->
Sets the audio decoder tags and how they should be merged with any
upstream stream tags. This will override any tags previously-set
with `gst_audio_decoder_merge_tags`.

Note that this is provided for convenience, and the subclass is
not required to use this and can still do tag handling on its own.

MT safe.
## `tags`
a `gst::TagList` to merge, or NULL to unset
 previously-set tags
## `mode`
the `gst::TagMergeMode` to use, usually `gst::TagMergeMode::Replace`
<!-- trait VideoDecoderExt::fn negotiate -->
Negotiate with downstream elements to currently configured `VideoCodecState`.
Unmark GST_PAD_FLAG_NEED_RECONFIGURE in any case. But mark it again if
negotiate fails.

# Returns

`true` if the negotiation succeeded, else `false`.
<!-- trait VideoDecoderExt::fn proxy_getcaps -->
Returns caps that express `caps` (or sink template caps if `caps` == NULL)
restricted to resolution/format/... combinations supported by downstream
elements.
## `caps`
initial caps
## `filter`
filter caps

# Returns

a `gst::Caps` owned by caller
<!-- trait VideoDecoderExt::fn release_frame -->
Similar to `VideoDecoder::drop_frame`, but simply releases `frame`
without any processing other than removing it from list of pending frames,
after which it is considered finished and released.
## `frame`
the `VideoCodecFrame` to release
<!-- trait VideoDecoderExt::fn set_estimate_rate -->
Allows baseclass to perform byte to time estimated conversion.
## `enabled`
whether to enable byte to time conversion
<!-- trait VideoDecoderExt::fn set_interlaced_output_state -->
Same as `VideoDecoder::set_output_state`() but also allows you to also set
the interlacing mode.

Feature: `v1_16`

## `fmt`
a `VideoFormat`
## `interlace_mode`
A `VideoInterlaceMode`
## `width`
The width in pixels
## `height`
The height in pixels
## `reference`
An optional reference `VideoCodecState`

# Returns

the newly configured output state.
<!-- trait VideoDecoderExt::fn set_latency -->
Lets `VideoDecoder` sub-classes tell the baseclass what the decoder
latency is. Will also post a LATENCY message on the bus so the pipeline
can reconfigure its global latency.
## `min_latency`
minimum latency
## `max_latency`
maximum latency
<!-- trait VideoDecoderExt::fn set_max_errors -->
Sets numbers of tolerated decoder errors, where a tolerated one is then only
warned about, but more than tolerated will lead to fatal error. You can set
-1 for never returning fatal errors. Default is set to
GST_VIDEO_DECODER_MAX_ERRORS.

The '-1' option was added in 1.4
## `num`
max tolerated errors
<!-- trait VideoDecoderExt::fn set_needs_format -->
Configures decoder format needs. If enabled, subclass needs to be
negotiated with format caps before it can process any data. It will then
never be handed any data before it has been configured.
Otherwise, it might be handed data without having been configured and
is then expected being able to do so either by default
or based on the input data.
## `enabled`
new state
<!-- trait VideoDecoderExt::fn set_output_state -->
Creates a new `VideoCodecState` with the specified `fmt`, `width` and `height`
as the output state for the decoder.
Any previously set output state on `self` will be replaced by the newly
created one.

If the subclass wishes to copy over existing fields (like pixel aspec ratio,
or framerate) from an existing `VideoCodecState`, it can be provided as a
`reference`.

If the subclass wishes to override some fields from the output state (like
pixel-aspect-ratio or framerate) it can do so on the returned `VideoCodecState`.

The new output state will only take effect (set on pads and buffers) starting
from the next call to `VideoDecoder::finish_frame`().
## `fmt`
a `VideoFormat`
## `width`
The width in pixels
## `height`
The height in pixels
## `reference`
An optional reference `VideoCodecState`

# Returns

the newly configured output state.
<!-- trait VideoDecoderExt::fn set_packetized -->
Allows baseclass to consider input data as packetized or not. If the
input is packetized, then the `parse` method will not be called.
## `packetized`
whether the input data should be considered as packetized.
<!-- trait VideoDecoderExt::fn set_use_default_pad_acceptcaps -->
Lets `VideoDecoder` sub-classes decide if they want the sink pad
to use the default pad query handler to reply to accept-caps queries.

By setting this to true it is possible to further customize the default
handler with `GST_PAD_SET_ACCEPT_INTERSECT` and
`GST_PAD_SET_ACCEPT_TEMPLATE`
## `use_`
if the default pad accept-caps query handling should be used
<!-- trait VideoDecoderExt::fn get_property_max_errors -->
Maximum number of tolerated consecutive decode errors. See
`VideoDecoderExt::set_max_errors` for more details.

Feature: `v1_18`

<!-- trait VideoDecoderExt::fn set_property_max_errors -->
Maximum number of tolerated consecutive decode errors. See
`VideoDecoderExt::set_max_errors` for more details.

Feature: `v1_18`

<!-- trait VideoDecoderExt::fn get_property_qos -->
If set to `true` the decoder will handle QoS events received
from downstream elements.
This includes dropping output frames which are detected as late
using the metrics reported by those events.

Feature: `v1_18`

<!-- trait VideoDecoderExt::fn set_property_qos -->
If set to `true` the decoder will handle QoS events received
from downstream elements.
This includes dropping output frames which are detected as late
using the metrics reported by those events.

Feature: `v1_18`

<!-- enum VideoDitherMethod -->
Different dithering methods to use.
<!-- enum VideoDitherMethod::variant None -->
no dithering
<!-- enum VideoDitherMethod::variant Verterr -->
propagate rounding errors downwards
<!-- enum VideoDitherMethod::variant FloydSteinberg -->
Dither with floyd-steinberg error diffusion
<!-- enum VideoDitherMethod::variant SierraLite -->
Dither with Sierra Lite error diffusion
<!-- enum VideoDitherMethod::variant Bayer -->
ordered dither using a bayer pattern
<!-- struct VideoEncoder -->
This base class is for video encoders turning raw video into
encoded video data.

GstVideoEncoder and subclass should cooperate as follows.

## Configuration

 * Initially, GstVideoEncoder calls `start` when the encoder element
 is activated, which allows subclass to perform any global setup.
 * GstVideoEncoder calls `set_format` to inform subclass of the format
 of input video data that it is about to receive. Subclass should
 setup for encoding and configure base class as appropriate
 (e.g. latency). While unlikely, it might be called more than once,
 if changing input parameters require reconfiguration. Baseclass
 will ensure that processing of current configuration is finished.
 * GstVideoEncoder calls `stop` at end of all processing.

## Data processing

 * Base class collects input data and metadata into a frame and hands
 this to subclass' `handle_frame`.

 * If codec processing results in encoded data, subclass should call
 [`finish_frame`](Self::finish_frame) to have encoded data pushed
 downstream.

 * If implemented, baseclass calls subclass `pre_push` just prior to
 pushing to allow subclasses to modify some metadata on the buffer.
 If it returns GST_FLOW_OK, the buffer is pushed downstream.

 * GstVideoEncoderClass will handle both srcpad and sinkpad events.
 Sink events will be passed to subclass if `event` callback has been
 provided.

## Shutdown phase

 * GstVideoEncoder class calls `stop` to inform the subclass that data
 parsing will be stopped.

Subclass is responsible for providing pad template caps for
source and sink pads. The pads need to be named "sink" and "src". It should
also be able to provide fixed src pad caps in `getcaps` by the time it calls
[`finish_frame`](Self::finish_frame).

Things that subclass need to take care of:

 * Provide pad templates
 * Provide source pad caps before pushing the first buffer
 * Accept data in `handle_frame` and provide encoded results to
 [`finish_frame`](Self::finish_frame).


The `VideoEncoder:qos` property will enable the Quality-of-Service
features of the encoder which gather statistics about the real-time
performance of the downstream elements. If enabled, subclasses can
use `VideoEncoderExt::get_max_encode_time` to check if input frames
are already late and drop them right away to give a chance to the
pipeline to catch up.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`VideoEncoderExt`](trait@crate::VideoEncoderExt), [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`], [`VideoEncoderExtManual`](trait@crate::VideoEncoderExtManual)
<!-- trait VideoEncoderExt -->
Trait containing all `VideoEncoder` methods.

# Implementors

[`VideoEncoder`](struct@crate::VideoEncoder)
<!-- trait VideoEncoderExt::fn allocate_output_buffer -->
Helper function that allocates a buffer to hold an encoded video frame
for `self`'s current `VideoCodecState`.
## `size`
size of the buffer

# Returns

allocated buffer
<!-- trait VideoEncoderExt::fn allocate_output_frame -->
Helper function that allocates a buffer to hold an encoded video frame for `self`'s
current `VideoCodecState`. Subclass should already have configured video
state and set src pad caps.

The buffer allocated here is owned by the frame and you should only
keep references to the frame, not the buffer.
## `frame`
a `VideoCodecFrame`
## `size`
size of the buffer

# Returns

`gst::FlowReturn::Ok` if an output buffer could be allocated
<!-- trait VideoEncoderExt::fn finish_frame -->
`frame` must have a valid encoded data buffer, whose metadata fields
are then appropriately set according to frame data or no buffer at
all if the frame should be dropped.
It is subsequently pushed downstream or provided to `pre_push`.
In any case, the frame is considered finished and released.

After calling this function the output buffer of the frame is to be
considered read-only. This function will also change the metadata
of the buffer.
## `frame`
an encoded `VideoCodecFrame`

# Returns

a `gst::FlowReturn` resulting from sending data downstream
<!-- trait VideoEncoderExt::fn finish_subframe -->
If multiple subframes are produced for one input frame then use this method
for each subframe, except for the last one. Before calling this function,
you need to fill frame->output_buffer with the encoded buffer to push.

You must call `VideoEncoder::finish_frame`() for the last sub-frame
to tell the encoder that the frame has been fully encoded.

This function will change the metadata of `frame` and frame->output_buffer
will be pushed downstream.

Feature: `v1_18`

## `frame`
a `VideoCodecFrame` being encoded

# Returns

a `gst::FlowReturn` resulting from pushing the buffer downstream.
<!-- trait VideoEncoderExt::fn allocator -->
Lets `VideoEncoder` sub-classes to know the memory `allocator`
used by the base class and its `params`.

Unref the `allocator` after use it.
## `allocator`
the `gst::Allocator`
used
## `params`
the
`gst::AllocationParams` of `allocator`
<!-- trait VideoEncoderExt::fn frame -->
Get a pending unfinished `VideoCodecFrame`
## `frame_number`
system_frame_number of a frame

# Returns

pending unfinished `VideoCodecFrame` identified by `frame_number`.
<!-- trait VideoEncoderExt::fn frames -->
Get all pending unfinished `VideoCodecFrame`

# Returns

pending unfinished `VideoCodecFrame`.
<!-- trait VideoEncoderExt::fn latency -->
Query the configured encoding latency. Results will be returned via
`min_latency` and `max_latency`.
## `min_latency`
address of variable in which to store the
 configured minimum latency, or `None`
## `max_latency`
address of variable in which to store the
 configured maximum latency, or `None`
<!-- trait VideoEncoderExt::fn max_encode_time -->
Determines maximum possible encoding time for `frame` that will
allow it to encode and arrive in time (as determined by QoS events).
In particular, a negative result means encoding in time is no longer possible
and should therefore occur as soon/skippy as possible.

If no QoS events have been received from downstream, or if
`VideoEncoder:qos` is disabled this function returns `G_MAXINT64`.

Feature: `v1_14`

## `frame`
a `VideoCodecFrame`

# Returns

max decoding time.
<!-- trait VideoEncoderExt::fn min_force_key_unit_interval -->
Returns the minimum force-keyunit interval, see `VideoEncoderExt::set_min_force_key_unit_interval`
for more details.

Feature: `v1_18`


# Returns

the minimum force-keyunit interval
<!-- trait VideoEncoderExt::fn oldest_frame -->
Get the oldest unfinished pending `VideoCodecFrame`

# Returns

oldest unfinished pending `VideoCodecFrame`
<!-- trait VideoEncoderExt::fn output_state -->
Get the current `VideoCodecState`

# Returns

`VideoCodecState` describing format of video data.
<!-- trait VideoEncoderExt::fn is_qos_enabled -->
Checks if `self` is currently configured to handle Quality-of-Service
events from downstream.

Feature: `v1_14`


# Returns

`true` if the encoder is configured to perform Quality-of-Service.
<!-- trait VideoEncoderExt::fn merge_tags -->
Sets the video encoder tags and how they should be merged with any
upstream stream tags. This will override any tags previously-set
with `VideoEncoderExt::merge_tags`.

Note that this is provided for convenience, and the subclass is
not required to use this and can still do tag handling on its own.

MT safe.
## `tags`
a `gst::TagList` to merge, or NULL to unset
 previously-set tags
## `mode`
the `gst::TagMergeMode` to use, usually `gst::TagMergeMode::Replace`
<!-- trait VideoEncoderExt::fn negotiate -->
Negotiate with downstream elements to currently configured `VideoCodecState`.
Unmark GST_PAD_FLAG_NEED_RECONFIGURE in any case. But mark it again if
negotiate fails.

# Returns

`true` if the negotiation succeeded, else `false`.
<!-- trait VideoEncoderExt::fn proxy_getcaps -->
Returns caps that express `caps` (or sink template caps if `caps` == NULL)
restricted to resolution/format/... combinations supported by downstream
elements (e.g. muxers).
## `caps`
initial caps
## `filter`
filter caps

# Returns

a `gst::Caps` owned by caller
<!-- trait VideoEncoderExt::fn set_headers -->
Set the codec headers to be sent downstream whenever requested.
## `headers`
a list of `gst::Buffer` containing the codec header
<!-- trait VideoEncoderExt::fn set_latency -->
Informs baseclass of encoding latency.
## `min_latency`
minimum latency
## `max_latency`
maximum latency
<!-- trait VideoEncoderExt::fn set_min_force_key_unit_interval -->
Sets the minimum interval for requesting keyframes based on force-keyunit
events. Setting this to 0 will allow to handle every event, setting this to
`GST_CLOCK_TIME_NONE` causes force-keyunit events to be ignored.

Feature: `v1_18`

## `interval`
minimum interval
<!-- trait VideoEncoderExt::fn set_min_pts -->
Request minimal value for PTS passed to handle_frame.

For streams with reordered frames this can be used to ensure that there
is enough time to accommodate first DTS, which may be less than first PTS
## `min_pts`
minimal PTS that will be passed to handle_frame
<!-- trait VideoEncoderExt::fn set_output_state -->
Creates a new `VideoCodecState` with the specified caps as the output state
for the encoder.
Any previously set output state on `self` will be replaced by the newly
created one.

The specified `caps` should not contain any resolution, pixel-aspect-ratio,
framerate, codec-data, .... Those should be specified instead in the returned
`VideoCodecState`.

If the subclass wishes to copy over existing fields (like pixel aspect ratio,
or framerate) from an existing `VideoCodecState`, it can be provided as a
`reference`.

If the subclass wishes to override some fields from the output state (like
pixel-aspect-ratio or framerate) it can do so on the returned `VideoCodecState`.

The new output state will only take effect (set on pads and buffers) starting
from the next call to `VideoEncoder::finish_frame`().
## `caps`
the `gst::Caps` to use for the output
## `reference`
An optional reference [`crate::VideoCodecState`] (XXX: @-reference does not belong to VideoEncoderExt!)

# Returns

the newly configured output state.
<!-- trait VideoEncoderExt::fn set_qos_enabled -->
Configures `self` to handle Quality-of-Service events from downstream.

Feature: `v1_14`

## `enabled`
the new qos value.
<!-- trait VideoEncoderExt::fn get_property_min_force_key_unit_interval -->
Minimum interval between force-keyunit requests in nanoseconds. See
`VideoEncoderExt::set_min_force_key_unit_interval` for more details.

Feature: `v1_18`

<!-- trait VideoEncoderExt::fn set_property_min_force_key_unit_interval -->
Minimum interval between force-keyunit requests in nanoseconds. See
`VideoEncoderExt::set_min_force_key_unit_interval` for more details.

Feature: `v1_18`

<!-- enum VideoFieldOrder -->
Field order of interlaced content. This is only valid for
interlace-mode=interleaved and not interlace-mode=mixed. In the case of
mixed or GST_VIDEO_FIELD_ORDER_UNKOWN, the field order is signalled via
buffer flags.
<!-- enum VideoFieldOrder::variant Unknown -->
unknown field order for interlaced content.
 The actual field order is signalled via buffer flags.
<!-- enum VideoFieldOrder::variant TopFieldFirst -->
top field is first
<!-- enum VideoFieldOrder::variant BottomFieldFirst -->
bottom field is first

Feature: `v1_12`

<!-- struct VideoFilter -->
Provides useful functions and a base class for video filters.

The videofilter will by default enable QoS on the parent GstBaseTransform
to implement frame dropping.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`trait@gst_base::BaseTransformExt`], [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- struct VideoFlags -->
Extra video flags
<!-- struct VideoFlags::const NONE -->
no flags
<!-- struct VideoFlags::const VARIABLE_FPS -->
a variable fps is selected, fps_n and fps_d
 denote the maximum fps of the video
<!-- struct VideoFlags::const PREMULTIPLIED_ALPHA -->
Each color has been scaled by the alpha
 value.
<!-- enum VideoFormat -->
Enum value describing the most common video formats.

See the [GStreamer raw video format design document](https://gstreamer.freedesktop.org/documentation/additional/design/mediatype-video-raw.html`formats`)
for details about the layout and packing of these formats in memory.
<!-- enum VideoFormat::variant Unknown -->
Unknown or unset video format id
<!-- enum VideoFormat::variant Encoded -->
Encoded video format. Only ever use that in caps for
 special video formats in combination with non-system
 memory GstCapsFeatures where it does not make sense
 to specify a real video format.
<!-- enum VideoFormat::variant I420 -->
planar 4:2:0 YUV
<!-- enum VideoFormat::variant Yv12 -->
planar 4:2:0 YVU (like I420 but UV planes swapped)
<!-- enum VideoFormat::variant Yuy2 -->
packed 4:2:2 YUV (Y0-U0-Y1-V0 Y2-U2-Y3-V2 Y4 ...)
<!-- enum VideoFormat::variant Uyvy -->
packed 4:2:2 YUV (U0-Y0-V0-Y1 U2-Y2-V2-Y3 U4 ...)
<!-- enum VideoFormat::variant Ayuv -->
packed 4:4:4 YUV with alpha channel (A0-Y0-U0-V0 ...)
<!-- enum VideoFormat::variant Rgbx -->
sparse rgb packed into 32 bit, space last
<!-- enum VideoFormat::variant Bgrx -->
sparse reverse rgb packed into 32 bit, space last
<!-- enum VideoFormat::variant Xrgb -->
sparse rgb packed into 32 bit, space first
<!-- enum VideoFormat::variant Xbgr -->
sparse reverse rgb packed into 32 bit, space first
<!-- enum VideoFormat::variant Rgba -->
rgb with alpha channel last
<!-- enum VideoFormat::variant Bgra -->
reverse rgb with alpha channel last
<!-- enum VideoFormat::variant Argb -->
rgb with alpha channel first
<!-- enum VideoFormat::variant Abgr -->
reverse rgb with alpha channel first
<!-- enum VideoFormat::variant Rgb -->
RGB packed into 24 bits without padding (`R-G-B-R-G-B`)
<!-- enum VideoFormat::variant Bgr -->
reverse RGB packed into 24 bits without padding (`B-G-R-B-G-R`)
<!-- enum VideoFormat::variant Y41b -->
planar 4:1:1 YUV
<!-- enum VideoFormat::variant Y42b -->
planar 4:2:2 YUV
<!-- enum VideoFormat::variant Yvyu -->
packed 4:2:2 YUV (Y0-V0-Y1-U0 Y2-V2-Y3-U2 Y4 ...)
<!-- enum VideoFormat::variant Y444 -->
planar 4:4:4 YUV
<!-- enum VideoFormat::variant V210 -->
packed 4:2:2 10-bit YUV, complex format
<!-- enum VideoFormat::variant V216 -->
packed 4:2:2 16-bit YUV, Y0-U0-Y1-V1 order
<!-- enum VideoFormat::variant Nv12 -->
planar 4:2:0 YUV with interleaved UV plane
<!-- enum VideoFormat::variant Nv21 -->
planar 4:2:0 YUV with interleaved VU plane
<!-- enum VideoFormat::variant Gray8 -->
8-bit grayscale
<!-- enum VideoFormat::variant Gray16Be -->
16-bit grayscale, most significant byte first
<!-- enum VideoFormat::variant Gray16Le -->
16-bit grayscale, least significant byte first
<!-- enum VideoFormat::variant V308 -->
packed 4:4:4 YUV (Y-U-V ...)
<!-- enum VideoFormat::variant Rgb16 -->
rgb 5-6-5 bits per component
<!-- enum VideoFormat::variant Bgr16 -->
reverse rgb 5-6-5 bits per component
<!-- enum VideoFormat::variant Rgb15 -->
rgb 5-5-5 bits per component
<!-- enum VideoFormat::variant Bgr15 -->
reverse rgb 5-5-5 bits per component
<!-- enum VideoFormat::variant Uyvp -->
packed 10-bit 4:2:2 YUV (U0-Y0-V0-Y1 U2-Y2-V2-Y3 U4 ...)
<!-- enum VideoFormat::variant A420 -->
planar 4:4:2:0 AYUV
<!-- enum VideoFormat::variant Rgb8p -->
8-bit paletted RGB
<!-- enum VideoFormat::variant Yuv9 -->
planar 4:1:0 YUV
<!-- enum VideoFormat::variant Yvu9 -->
planar 4:1:0 YUV (like YUV9 but UV planes swapped)
<!-- enum VideoFormat::variant Iyu1 -->
packed 4:1:1 YUV (Cb-Y0-Y1-Cr-Y2-Y3 ...)
<!-- enum VideoFormat::variant Argb64 -->
rgb with alpha channel first, 16 bits per channel
<!-- enum VideoFormat::variant Ayuv64 -->
packed 4:4:4 YUV with alpha channel, 16 bits per channel (A0-Y0-U0-V0 ...)
<!-- enum VideoFormat::variant R210 -->
packed 4:4:4 RGB, 10 bits per channel
<!-- enum VideoFormat::variant I42010be -->
planar 4:2:0 YUV, 10 bits per channel
<!-- enum VideoFormat::variant I42010le -->
planar 4:2:0 YUV, 10 bits per channel
<!-- enum VideoFormat::variant I42210be -->
planar 4:2:2 YUV, 10 bits per channel
<!-- enum VideoFormat::variant I42210le -->
planar 4:2:2 YUV, 10 bits per channel
<!-- enum VideoFormat::variant Y44410be -->
planar 4:4:4 YUV, 10 bits per channel (Since: 1.2)
<!-- enum VideoFormat::variant Y44410le -->
planar 4:4:4 YUV, 10 bits per channel (Since: 1.2)
<!-- enum VideoFormat::variant Gbr -->
planar 4:4:4 RGB, 8 bits per channel (Since: 1.2)
<!-- enum VideoFormat::variant Gbr10be -->
planar 4:4:4 RGB, 10 bits per channel (Since: 1.2)
<!-- enum VideoFormat::variant Gbr10le -->
planar 4:4:4 RGB, 10 bits per channel (Since: 1.2)
<!-- enum VideoFormat::variant Nv16 -->
planar 4:2:2 YUV with interleaved UV plane (Since: 1.2)
<!-- enum VideoFormat::variant Nv24 -->
planar 4:4:4 YUV with interleaved UV plane (Since: 1.2)
<!-- enum VideoFormat::variant Nv1264z32 -->
NV12 with 64x32 tiling in zigzag pattern (Since: 1.4)
<!-- enum VideoFormat::variant A42010be -->
planar 4:4:2:0 YUV, 10 bits per channel (Since: 1.6)
<!-- enum VideoFormat::variant A42010le -->
planar 4:4:2:0 YUV, 10 bits per channel (Since: 1.6)
<!-- enum VideoFormat::variant A42210be -->
planar 4:4:2:2 YUV, 10 bits per channel (Since: 1.6)
<!-- enum VideoFormat::variant A42210le -->
planar 4:4:2:2 YUV, 10 bits per channel (Since: 1.6)
<!-- enum VideoFormat::variant A44410be -->
planar 4:4:4:4 YUV, 10 bits per channel (Since: 1.6)
<!-- enum VideoFormat::variant A44410le -->
planar 4:4:4:4 YUV, 10 bits per channel (Since: 1.6)
<!-- enum VideoFormat::variant Nv61 -->
planar 4:2:2 YUV with interleaved VU plane (Since: 1.6)
<!-- enum VideoFormat::variant P01010be -->
planar 4:2:0 YUV with interleaved UV plane, 10 bits per channel (Since: 1.10)
<!-- enum VideoFormat::variant P01010le -->
planar 4:2:0 YUV with interleaved UV plane, 10 bits per channel (Since: 1.10)
<!-- enum VideoFormat::variant Iyu2 -->
packed 4:4:4 YUV (U-Y-V ...) (Since: 1.10)
<!-- enum VideoFormat::variant Vyuy -->
packed 4:2:2 YUV (V0-Y0-U0-Y1 V2-Y2-U2-Y3 V4 ...)
<!-- enum VideoFormat::variant Gbra -->
planar 4:4:4:4 ARGB, 8 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Gbra10be -->
planar 4:4:4:4 ARGB, 10 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Gbra10le -->
planar 4:4:4:4 ARGB, 10 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Gbr12be -->
planar 4:4:4 RGB, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Gbr12le -->
planar 4:4:4 RGB, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Gbra12be -->
planar 4:4:4:4 ARGB, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Gbra12le -->
planar 4:4:4:4 ARGB, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant I42012be -->
planar 4:2:0 YUV, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant I42012le -->
planar 4:2:0 YUV, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant I42212be -->
planar 4:2:2 YUV, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant I42212le -->
planar 4:2:2 YUV, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Y44412be -->
planar 4:4:4 YUV, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Y44412le -->
planar 4:4:4 YUV, 12 bits per channel (Since: 1.12)
<!-- enum VideoFormat::variant Gray10Le32 -->
10-bit grayscale, packed into 32bit words (2 bits padding) (Since: 1.14)
<!-- enum VideoFormat::variant Nv1210le32 -->
10-bit variant of [`Nv12`](Self::Nv12), packed into 32bit words (MSB 2 bits padding) (Since: 1.14)
<!-- enum VideoFormat::variant Nv1610le32 -->
10-bit variant of [`Nv16`](Self::Nv16), packed into 32bit words (MSB 2 bits padding) (Since: 1.14)
<!-- enum VideoFormat::variant Nv1210le40 -->
Fully packed variant of NV12_10LE32 (Since: 1.16)
<!-- enum VideoFormat::variant Y210 -->
packed 4:2:2 YUV, 10 bits per channel (Since: 1.16)
<!-- enum VideoFormat::variant Y410 -->
packed 4:4:4 YUV, 10 bits per channel(A-V-Y-U...) (Since: 1.16)
<!-- enum VideoFormat::variant Vuya -->
packed 4:4:4 YUV with alpha channel (V0-U0-Y0-A0...) (Since: 1.16)
<!-- enum VideoFormat::variant Bgr10a2Le -->
packed 4:4:4 RGB with alpha channel(B-G-R-A), 10 bits for R/G/B channel and MSB 2 bits for alpha channel (Since: 1.16)
<!-- enum VideoFormat::variant Rgb10a2Le -->
packed 4:4:4 RGB with alpha channel(R-G-B-A), 10 bits for R/G/B channel and MSB 2 bits for alpha channel (Since: 1.18)
<!-- enum VideoFormat::variant Y44416be -->
planar 4:4:4 YUV, 16 bits per channel (Since: 1.18)
<!-- enum VideoFormat::variant Y44416le -->
planar 4:4:4 YUV, 16 bits per channel (Since: 1.18)
<!-- enum VideoFormat::variant P016Be -->
planar 4:2:0 YUV with interleaved UV plane, 16 bits per channel (Since: 1.18)
<!-- enum VideoFormat::variant P016Le -->
planar 4:2:0 YUV with interleaved UV plane, 16 bits per channel (Since: 1.18)
<!-- enum VideoFormat::variant P012Be -->
planar 4:2:0 YUV with interleaved UV plane, 12 bits per channel (Since: 1.18)
<!-- enum VideoFormat::variant P012Le -->
planar 4:2:0 YUV with interleaved UV plane, 12 bits per channel (Since: 1.18)
<!-- enum VideoFormat::variant Y212Be -->
packed 4:2:2 YUV, 12 bits per channel (Y-U-Y-V) (Since: 1.18)
<!-- enum VideoFormat::variant Y212Le -->
packed 4:2:2 YUV, 12 bits per channel (Y-U-Y-V) (Since: 1.18)
<!-- enum VideoFormat::variant Y412Be -->
packed 4:4:4:4 YUV, 12 bits per channel(U-Y-V-A...) (Since: 1.18)
<!-- enum VideoFormat::variant Y412Le -->
packed 4:4:4:4 YUV, 12 bits per channel(U-Y-V-A...) (Since: 1.18)
<!-- enum VideoFormat::variant Nv124l4 -->
NV12 with 4x4 tiles in linear order.
<!-- enum VideoFormat::variant Nv1232l32 -->
NV12 with 32x32 tiles in linear order.
<!-- struct VideoFormatFlags -->
The different video flags that a format info can have.
<!-- struct VideoFormatFlags::const YUV -->
The video format is YUV, components are numbered
 0=Y, 1=U, 2=V.
<!-- struct VideoFormatFlags::const RGB -->
The video format is RGB, components are numbered
 0=R, 1=G, 2=B.
<!-- struct VideoFormatFlags::const GRAY -->
The video is gray, there is one gray component
 with index 0.
<!-- struct VideoFormatFlags::const ALPHA -->
The video format has an alpha components with
 the number 3.
<!-- struct VideoFormatFlags::const LE -->
The video format has data stored in little
 endianness.
<!-- struct VideoFormatFlags::const PALETTE -->
The video format has a palette. The palette
 is stored in the second plane and indexes are stored in the first plane.
<!-- struct VideoFormatFlags::const COMPLEX -->
The video format has a complex layout that
 can't be described with the usual information in the `VideoFormatInfo`.
<!-- struct VideoFormatFlags::const UNPACK -->
This format can be used in a
 `GstVideoFormatUnpack` and `GstVideoFormatPack` function.
<!-- struct VideoFormatFlags::const TILED -->
The format is tiled, there is tiling information
 in the last plane.
<!-- struct VideoFormatInfo -->
Information for a video format.

Feature: `v1_18`
<!-- impl VideoFormatInfo::fn component -->
Fill `components` with the number of all the components packed in plane `p`
for the format `self`. A value of -1 in `components` indicates that no more
components are packed in the plane.

Feature: `v1_18`

## `plane`
a plane number
## `components`
array used to store component numbers
<!-- struct VideoFrame -->
A video frame obtained from `VideoFrame::map`
<!-- impl VideoFrame::fn copy -->
Copy the contents from `src` to `self`.

Note: Since: 1.18, `self` dimensions are allowed to be
smaller than `src` dimensions.
## `src`
a `VideoFrame`

# Returns

TRUE if the contents could be copied.
<!-- impl VideoFrame::fn copy_plane -->
Copy the plane with index `plane` from `src` to `self`.

Note: Since: 1.18, `self` dimensions are allowed to be
smaller than `src` dimensions.
## `src`
a `VideoFrame`
## `plane`
a plane

# Returns

TRUE if the contents could be copied.
<!-- impl VideoFrame::fn map -->
Use `info` and `buffer` to fill in the values of `self`. `self` is usually
allocated on the stack, and you will pass the address to the `VideoFrame`
structure allocated on the stack; `VideoFrame::map` will then fill in
the structures with the various video-specific information you need to access
the pixels of the video buffer. You can then use accessor macros such as
GST_VIDEO_FRAME_COMP_DATA(), GST_VIDEO_FRAME_PLANE_DATA(),
GST_VIDEO_FRAME_COMP_STRIDE(), GST_VIDEO_FRAME_PLANE_STRIDE() etc.
to get to the pixels.


```C
  GstVideoFrame vframe;
  ...
  // set RGB pixels to black one at a time
  if (gst_video_frame_map (&amp;vframe, video_info, video_buffer, GST_MAP_WRITE)) {
    guint8 *pixels = GST_VIDEO_FRAME_PLANE_DATA (vframe, 0);
    guint stride = GST_VIDEO_FRAME_PLANE_STRIDE (vframe, 0);
    guint pixel_stride = GST_VIDEO_FRAME_COMP_PSTRIDE (vframe, 0);

    for (h = 0; h < height; ++h) {
      for (w = 0; w < width; ++w) {
        guint8 *pixel = pixels + h * stride + w * pixel_stride;

        memset (pixel, 0, pixel_stride);
      }
    }

    gst_video_frame_unmap (&amp;vframe);
  }
  ...
```

All video planes of `buffer` will be mapped and the pointers will be set in
`self`->data.

The purpose of this function is to make it easy for you to get to the video
pixels in a generic way, without you having to worry too much about details
such as whether the video data is allocated in one contiguous memory chunk
or multiple memory chunks (e.g. one for each plane); or if custom strides
and custom plane offsets are used or not (as signalled by GstVideoMeta on
each buffer). This function will just fill the `VideoFrame` structure
with the right values and if you use the accessor macros everything will
just work and you can access the data easily. It also maps the underlying
memory chunks for you.
## `info`
a `VideoInfo`
## `buffer`
the buffer to map
## `flags`
`gst::MapFlags`

# Returns

`true` on success.
<!-- impl VideoFrame::fn map_id -->
Use `info` and `buffer` to fill in the values of `self` with the video frame
information of frame `id`.

When `id` is -1, the default frame is mapped. When `id` != -1, this function
will return `false` when there is no GstVideoMeta with that id.

All video planes of `buffer` will be mapped and the pointers will be set in
`self`->data.
## `info`
a `VideoInfo`
## `buffer`
the buffer to map
## `id`
the frame id to map
## `flags`
`gst::MapFlags`

# Returns

`true` on success.
<!-- impl VideoFrame::fn unmap -->
Unmap the memory previously mapped with gst_video_frame_map.
<!-- struct VideoFrameFlags -->
Extra video frame flags
<!-- struct VideoFrameFlags::const NONE -->
no flags
<!-- struct VideoFrameFlags::const INTERLACED -->
The video frame is interlaced. In mixed
 interlace-mode, this flag specifies if the frame is interlaced or
 progressive.
<!-- struct VideoFrameFlags::const TFF -->
The video frame has the top field first
<!-- struct VideoFrameFlags::const RFF -->
The video frame has the repeat flag
<!-- struct VideoFrameFlags::const ONEFIELD -->
The video frame has one field
<!-- struct VideoFrameFlags::const MULTIPLE_VIEW -->
The video contains one or
 more non-mono views
<!-- struct VideoFrameFlags::const FIRST_IN_BUNDLE -->
The video frame is the first
 in a set of corresponding views provided as sequential frames.
<!-- struct VideoFrameFlags::const TOP_FIELD -->
The video frame has the top field only. This
 is the same as GST_VIDEO_FRAME_FLAG_TFF | GST_VIDEO_FRAME_FLAG_ONEFIELD
 (Since: 1.16).
<!-- struct VideoFrameFlags::const BOTTOM_FIELD -->
The video frame has the bottom field
 only. This is the same as GST_VIDEO_FRAME_FLAG_ONEFIELD
 (GST_VIDEO_FRAME_FLAG_TFF flag unset) (Since: 1.16).
<!-- enum VideoGammaMode -->
<!-- enum VideoGammaMode::variant None -->
disable gamma handling
<!-- enum VideoGammaMode::variant Remap -->
convert between input and output gamma
Different gamma conversion modes
<!-- struct VideoInfo -->
Information describing image properties. This information can be filled
in from GstCaps with `VideoInfo::from_caps`. The information is also used
to store the specific video info when mapping a video frame with
`VideoFrame::map`.

Use the provided macros to access the info in this structure.
<!-- impl VideoInfo::fn new -->
Allocate a new `VideoInfo` that is also initialized with
`VideoInfo::init`.

# Returns

a new `VideoInfo`. free with `VideoInfo::free`.
<!-- impl VideoInfo::fn align -->
Adjust the offset and stride fields in `self` so that the padding and
stride alignment in `align` is respected.

Extra padding will be added to the right side when stride alignment padding
is required and `align` will be updated with the new padding values.
## `align`
alignment parameters

# Returns

`false` if alignment could not be applied, e.g. because the
 size of a frame can't be represented as a 32 bit integer (Since: 1.12)
<!-- impl VideoInfo::fn align_full -->
This variant of `VideoInfo::align` provides the updated size, in bytes,
of each video plane after the alignment, including all horizontal and vertical
paddings.

In case of GST_VIDEO_INTERLACE_MODE_ALTERNATE info, the returned sizes are the
ones used to hold a single field, not the full frame.

Feature: `v1_18`

## `align`
alignment parameters
## `plane_size`
array used to store the plane sizes

# Returns

`false` if alignment could not be applied, e.g. because the
 size of a frame can't be represented as a 32 bit integer
<!-- impl VideoInfo::fn convert -->
Converts among various `gst::Format` types. This function handles
GST_FORMAT_BYTES, GST_FORMAT_TIME, and GST_FORMAT_DEFAULT. For
raw video, GST_FORMAT_DEFAULT corresponds to video frames. This
function can be used to handle pad queries of the type GST_QUERY_CONVERT.
## `src_format`
`gst::Format` of the `src_value`
## `src_value`
value to convert
## `dest_format`
`gst::Format` of the `dest_value`
## `dest_value`
pointer to destination value

# Returns

TRUE if the conversion was successful.
<!-- impl VideoInfo::fn copy -->
Copy a GstVideoInfo structure.

# Returns

a new `VideoInfo`. free with gst_video_info_free.
<!-- impl VideoInfo::fn free -->
Free a GstVideoInfo structure previously allocated with `VideoInfo::new`
or `VideoInfo::copy`.
<!-- impl VideoInfo::fn from_caps -->
Parse `caps` and update `self`.
## `caps`
a `gst::Caps`

# Returns

TRUE if `caps` could be parsed
<!-- impl VideoInfo::fn init -->
Initialize `self` with default values.
<!-- impl VideoInfo::fn is_equal -->
Compares two `VideoInfo` and returns whether they are equal or not
## `other`
a `VideoInfo`

# Returns

`true` if `self` and `other` are equal, else `false`.
<!-- impl VideoInfo::fn set_format -->
Set the default info for a video frame of `format` and `width` and `height`.

Note: This initializes `self` first, no values are preserved. This function
does not set the offsets correctly for interlaced vertically
subsampled formats.
## `format`
the format
## `width`
a width
## `height`
a height

# Returns

`false` if the returned video info is invalid, e.g. because the
 size of a frame can't be represented as a 32 bit integer (Since: 1.12)
<!-- impl VideoInfo::fn set_interlaced_format -->
Same as `VideoInfo::set_format` but also allowing to set the interlaced
mode.

Feature: `v1_16`

## `format`
the format
## `mode`
a `VideoInterlaceMode`
## `width`
a width
## `height`
a height

# Returns

`false` if the returned video info is invalid, e.g. because the
 size of a frame can't be represented as a 32 bit integer.
<!-- impl VideoInfo::fn to_caps -->
Convert the values of `self` into a `gst::Caps`.

# Returns

a new `gst::Caps` containing the info of `self`.
<!-- enum VideoInterlaceMode -->
The possible values of the `VideoInterlaceMode` describing the interlace
mode of the stream.
<!-- enum VideoInterlaceMode::variant Progressive -->
all frames are progressive
<!-- enum VideoInterlaceMode::variant Interleaved -->
2 fields are interleaved in one video
 frame. Extra buffer flags describe the field order.
<!-- enum VideoInterlaceMode::variant Mixed -->
frames contains both interlaced and
 progressive video, the buffer flags describe the frame and fields.
<!-- enum VideoInterlaceMode::variant Fields -->
2 fields are stored in one buffer, use the
 frame ID to get access to the required field. For multiview (the
 'views' property > 1) the fields of view N can be found at frame ID
 (N * 2) and (N * 2) + 1.
 Each field has only half the amount of lines as noted in the
 height property. This mode requires multiple GstVideoMeta metadata
 to describe the fields.
<!-- enum VideoInterlaceMode::variant Alternate -->
1 field is stored in one buffer,
 `GST_VIDEO_BUFFER_FLAG_TF` or `GST_VIDEO_BUFFER_FLAG_BF` indicates if
 the buffer is carrying the top or bottom field, respectively. The top and
 bottom buffers must alternate in the pipeline, with this mode
 (Since: 1.16).
<!-- enum VideoMatrixMode -->
Different color matrix conversion modes
<!-- enum VideoMatrixMode::variant Full -->
do conversion between color matrices
<!-- enum VideoMatrixMode::variant InputOnly -->
use the input color matrix to convert
 to and from R'G'B
<!-- enum VideoMatrixMode::variant OutputOnly -->
use the output color matrix to convert
 to and from R'G'B
<!-- enum VideoMatrixMode::variant None -->
disable color matrix conversion.
<!-- struct VideoMultiviewFlags -->
GstVideoMultiviewFlags are used to indicate extra properties of a
stereo/multiview stream beyond the frame layout and buffer mapping
that is conveyed in the `VideoMultiviewMode`.
<!-- struct VideoMultiviewFlags::const NONE -->
No flags
<!-- struct VideoMultiviewFlags::const RIGHT_VIEW_FIRST -->
For stereo streams, the
 normal arrangement of left and right views is reversed.
<!-- struct VideoMultiviewFlags::const LEFT_FLIPPED -->
The left view is vertically
 mirrored.
<!-- struct VideoMultiviewFlags::const LEFT_FLOPPED -->
The left view is horizontally
 mirrored.
<!-- struct VideoMultiviewFlags::const RIGHT_FLIPPED -->
The right view is
 vertically mirrored.
<!-- struct VideoMultiviewFlags::const RIGHT_FLOPPED -->
The right view is
 horizontally mirrored.
<!-- struct VideoMultiviewFlags::const HALF_ASPECT -->
For frame-packed
 multiview modes, indicates that the individual
 views have been encoded with half the true width or height
 and should be scaled back up for display. This flag
 is used for overriding input layout interpretation
 by adjusting pixel-aspect-ratio.
 For side-by-side, column interleaved or checkerboard packings, the
 pixel width will be doubled. For row interleaved and top-bottom
 encodings, pixel height will be doubled.
<!-- struct VideoMultiviewFlags::const MIXED_MONO -->
The video stream contains both
 mono and multiview portions, signalled on each buffer by the
 absence or presence of the [`crate::VideoBufferFlags::MultipleView`] (XXX: @-reference does not belong to VideoMultiviewFlags!)
 buffer flag.
<!-- enum VideoMultiviewFramePacking -->
`VideoMultiviewFramePacking` represents the subset of `VideoMultiviewMode`
values that can be applied to any video frame without needing extra metadata.
It can be used by elements that provide a property to override the
multiview interpretation of a video stream when the video doesn't contain
any markers.

This enum is used (for example) on playbin, to re-interpret a played
video stream as a stereoscopic video. The individual enum values are
equivalent to and have the same value as the matching `VideoMultiviewMode`.
<!-- enum VideoMultiviewFramePacking::variant None -->
A special value indicating
no frame packing info.
<!-- enum VideoMultiviewFramePacking::variant Mono -->
All frames are monoscopic.
<!-- enum VideoMultiviewFramePacking::variant Left -->
All frames represent a left-eye view.
<!-- enum VideoMultiviewFramePacking::variant Right -->
All frames represent a right-eye view.
<!-- enum VideoMultiviewFramePacking::variant SideBySide -->
Left and right eye views are
provided in the left and right half of the frame respectively.
<!-- enum VideoMultiviewFramePacking::variant SideBySideQuincunx -->
Left and right eye
views are provided in the left and right half of the frame, but
have been sampled using quincunx method, with half-pixel offset
between the 2 views.
<!-- enum VideoMultiviewFramePacking::variant ColumnInterleaved -->
Alternating vertical
columns of pixels represent the left and right eye view respectively.
<!-- enum VideoMultiviewFramePacking::variant RowInterleaved -->
Alternating horizontal
rows of pixels represent the left and right eye view respectively.
<!-- enum VideoMultiviewFramePacking::variant TopBottom -->
The top half of the frame
contains the left eye, and the bottom half the right eye.
<!-- enum VideoMultiviewFramePacking::variant Checkerboard -->
Pixels are arranged with
alternating pixels representing left and right eye views in a
checkerboard fashion.
<!-- enum VideoMultiviewMode -->
All possible stereoscopic 3D and multiview representations.
In conjunction with `VideoMultiviewFlags`, describes how
multiview content is being transported in the stream.
<!-- enum VideoMultiviewMode::variant None -->
A special value indicating
no multiview information. Used in GstVideoInfo and other places to
indicate that no specific multiview handling has been requested or
provided. This value is never carried on caps.
<!-- enum VideoMultiviewMode::variant Mono -->
All frames are monoscopic.
<!-- enum VideoMultiviewMode::variant Left -->
All frames represent a left-eye view.
<!-- enum VideoMultiviewMode::variant Right -->
All frames represent a right-eye view.
<!-- enum VideoMultiviewMode::variant SideBySide -->
Left and right eye views are
provided in the left and right half of the frame respectively.
<!-- enum VideoMultiviewMode::variant SideBySideQuincunx -->
Left and right eye
views are provided in the left and right half of the frame, but
have been sampled using quincunx method, with half-pixel offset
between the 2 views.
<!-- enum VideoMultiviewMode::variant ColumnInterleaved -->
Alternating vertical
columns of pixels represent the left and right eye view respectively.
<!-- enum VideoMultiviewMode::variant RowInterleaved -->
Alternating horizontal
rows of pixels represent the left and right eye view respectively.
<!-- enum VideoMultiviewMode::variant TopBottom -->
The top half of the frame
contains the left eye, and the bottom half the right eye.
<!-- enum VideoMultiviewMode::variant Checkerboard -->
Pixels are arranged with
alternating pixels representing left and right eye views in a
checkerboard fashion.
<!-- enum VideoMultiviewMode::variant FrameByFrame -->
Left and right eye views
are provided in separate frames alternately.
<!-- enum VideoMultiviewMode::variant MultiviewFrameByFrame -->
Multiple
independent views are provided in separate frames in sequence.
This method only applies to raw video buffers at the moment.
Specific view identification is via the `GstVideoMultiviewMeta`
and `VideoMeta`(s) on raw video buffers.
<!-- enum VideoMultiviewMode::variant Separated -->
Multiple views are
provided as separate `gst::Memory` framebuffers attached to each
`gst::Buffer`, described by the `GstVideoMultiviewMeta`
and `VideoMeta`(s)
<!-- struct VideoOrientation -->
The interface allows unified access to control flipping and autocenter
operation of video-sources or operators.

# Implements

[`VideoOrientationExt`](trait@crate::VideoOrientationExt)
<!-- trait VideoOrientationExt -->
Trait containing all `VideoOrientation` methods.

# Implementors

[`VideoOrientation`](struct@crate::VideoOrientation)
<!-- trait VideoOrientationExt::fn hcenter -->
Get the horizontal centering offset from the given object.
## `center`
return location for the result

# Returns

`true` in case the element supports centering
<!-- trait VideoOrientationExt::fn hflip -->
Get the horizontal flipping state (`true` for flipped) from the given object.
## `flip`
return location for the result

# Returns

`true` in case the element supports flipping
<!-- trait VideoOrientationExt::fn vcenter -->
Get the vertical centering offset from the given object.
## `center`
return location for the result

# Returns

`true` in case the element supports centering
<!-- trait VideoOrientationExt::fn vflip -->
Get the vertical flipping state (`true` for flipped) from the given object.
## `flip`
return location for the result

# Returns

`true` in case the element supports flipping
<!-- trait VideoOrientationExt::fn set_hcenter -->
Set the horizontal centering offset for the given object.
## `center`
centering offset

# Returns

`true` in case the element supports centering
<!-- trait VideoOrientationExt::fn set_hflip -->
Set the horizontal flipping state (`true` for flipped) for the given object.
## `flip`
use flipping

# Returns

`true` in case the element supports flipping
<!-- trait VideoOrientationExt::fn set_vcenter -->
Set the vertical centering offset for the given object.
## `center`
centering offset

# Returns

`true` in case the element supports centering
<!-- trait VideoOrientationExt::fn set_vflip -->
Set the vertical flipping state (`true` for flipped) for the given object.
## `flip`
use flipping

# Returns

`true` in case the element supports flipping
<!-- enum VideoOrientationMethod -->
The different video orientation methods.
<!-- enum VideoOrientationMethod::variant Identity -->
Identity (no rotation)
<!-- enum VideoOrientationMethod::variant 90r -->
Rotate clockwise 90 degrees
<!-- enum VideoOrientationMethod::variant 180 -->
Rotate 180 degrees
<!-- enum VideoOrientationMethod::variant 90l -->
Rotate counter-clockwise 90 degrees
<!-- enum VideoOrientationMethod::variant Horiz -->
Flip horizontally
<!-- enum VideoOrientationMethod::variant Vert -->
Flip vertically
<!-- enum VideoOrientationMethod::variant UlLr -->
Flip across upper left/lower right diagonal
<!-- enum VideoOrientationMethod::variant UrLl -->
Flip across upper right/lower left diagonal
<!-- enum VideoOrientationMethod::variant Auto -->
Select flip method based on image-orientation tag
<!-- enum VideoOrientationMethod::variant Custom -->
Current status depends on plugin internal setup

Feature: `v1_10`

<!-- struct VideoOverlay -->
The `VideoOverlay` interface is used for 2 main purposes :

* To get a grab on the Window where the video sink element is going to render.
 This is achieved by either being informed about the Window identifier that
 the video sink element generated, or by forcing the video sink element to use
 a specific Window identifier for rendering.
* To force a redrawing of the latest video frame the video sink element
 displayed on the Window. Indeed if the `gst::Pipeline` is in `gst::State::Paused`
 state, moving the Window around will damage its content. Application
 developers will want to handle the Expose events themselves and force the
 video sink element to refresh the Window's content.

Using the Window created by the video sink is probably the simplest scenario,
in some cases, though, it might not be flexible enough for application
developers if they need to catch events such as mouse moves and button
clicks.

Setting a specific Window identifier on the video sink element is the most
flexible solution but it has some issues. Indeed the application needs to set
its Window identifier at the right time to avoid internal Window creation
from the video sink element. To solve this issue a `gst::Message` is posted on
the bus to inform the application that it should set the Window identifier
immediately. Here is an example on how to do that correctly:

```text
static GstBusSyncReply
create_window (GstBus * bus, GstMessage * message, GstPipeline * pipeline)
{
 // ignore anything but 'prepare-window-handle' element messages
 if (!gst_is_video_overlay_prepare_window_handle_message (message))
   return GST_BUS_PASS;

 win = XCreateSimpleWindow (disp, root, 0, 0, 320, 240, 0, 0, 0);

 XSetWindowBackgroundPixmap (disp, win, None);

 XMapRaised (disp, win);

 XSync (disp, FALSE);

 gst_video_overlay_set_window_handle (GST_VIDEO_OVERLAY (GST_MESSAGE_SRC (message)),
     win);

 gst_message_unref (message);

 return GST_BUS_DROP;
}
...
int
main (int argc, char **argv)
{
...
 bus = gst_pipeline_get_bus (GST_PIPELINE (pipeline));
 gst_bus_set_sync_handler (bus, (GstBusSyncHandler) create_window, pipeline,
        NULL);
...
}
```

## Two basic usage scenarios

There are two basic usage scenarios: in the simplest case, the application
uses `playbin` or `playsink` or knows exactly what particular element is used
for video output, which is usually the case when the application creates
the videosink to use (e.g. `xvimagesink`, `ximagesink`, etc.) itself; in this
case, the application can just create the videosink element, create and
realize the window to render the video on and then
call `VideoOverlay::set_window_handle` directly with the XID or native
window handle, before starting up the pipeline.
As `playbin` and `playsink` implement the video overlay interface and proxy
it transparently to the actual video sink even if it is created later, this
case also applies when using these elements.

In the other and more common case, the application does not know in advance
what GStreamer video sink element will be used for video output. This is
usually the case when an element such as `autovideosink` is used.
In this case, the video sink element itself is created
asynchronously from a GStreamer streaming thread some time after the
pipeline has been started up. When that happens, however, the video sink
will need to know right then whether to render onto an already existing
application window or whether to create its own window. This is when it
posts a prepare-window-handle message, and that is also why this message needs
to be handled in a sync bus handler which will be called from the streaming
thread directly (because the video sink will need an answer right then).

As response to the prepare-window-handle element message in the bus sync
handler, the application may use `VideoOverlay::set_window_handle` to tell
the video sink to render onto an existing window surface. At this point the
application should already have obtained the window handle / XID, so it
just needs to set it. It is generally not advisable to call any GUI toolkit
functions or window system functions from the streaming thread in which the
prepare-window-handle message is handled, because most GUI toolkits and
windowing systems are not thread-safe at all and a lot of care would be
required to co-ordinate the toolkit and window system calls of the
different threads (Gtk+ users please note: prior to Gtk+ 2.18
`GDK_WINDOW_XID` was just a simple structure access, so generally fine to do
within the bus sync handler; this macro was changed to a function call in
Gtk+ 2.18 and later, which is likely to cause problems when called from a
sync handler; see below for a better approach without `GDK_WINDOW_XID`
used in the callback).

## GstVideoOverlay and Gtk+


```text
#include &lt;gst/video/videooverlay.h&gt;
#include &lt;gtk/gtk.h&gt;
#ifdef GDK_WINDOWING_X11
#include &lt;gdk/gdkx.h&gt;  // for GDK_WINDOW_XID
#endif
#ifdef GDK_WINDOWING_WIN32
#include &lt;gdk/gdkwin32.h&gt;  // for GDK_WINDOW_HWND
#endif
...
static guintptr video_window_handle = 0;
...
static GstBusSyncReply
bus_sync_handler (GstBus * bus, GstMessage * message, gpointer user_data)
{
 // ignore anything but 'prepare-window-handle' element messages
 if (!gst_is_video_overlay_prepare_window_handle_message (message))
   return GST_BUS_PASS;

 if (video_window_handle != 0) {
   GstVideoOverlay *overlay;

   // GST_MESSAGE_SRC (message) will be the video sink element
   overlay = GST_VIDEO_OVERLAY (GST_MESSAGE_SRC (message));
   gst_video_overlay_set_window_handle (overlay, video_window_handle);
 } else {
   g_warning ("Should have obtained video_window_handle by now!");
 }

 gst_message_unref (message);
 return GST_BUS_DROP;
}
...
static void
video_widget_realize_cb (GtkWidget * widget, gpointer data)
{
#if GTK_CHECK_VERSION(2,18,0)
  // Tell Gtk+/Gdk to create a native window for this widget instead of
  // drawing onto the parent widget.
  // This is here just for pedagogical purposes, GDK_WINDOW_XID will call
  // it as well in newer Gtk versions
  if (!gdk_window_ensure_native (widget->window))
    g_error ("Couldn't create native window needed for GstVideoOverlay!");
#endif

#ifdef GDK_WINDOWING_X11
  {
    gulong xid = GDK_WINDOW_XID (gtk_widget_get_window (video_window));
    video_window_handle = xid;
  }
#endif
#ifdef GDK_WINDOWING_WIN32
  {
    HWND wnd = GDK_WINDOW_HWND (gtk_widget_get_window (video_window));
    video_window_handle = (guintptr) wnd;
  }
#endif
}
...
int
main (int argc, char **argv)
{
  GtkWidget *video_window;
  GtkWidget *app_window;
  ...
  app_window = gtk_window_new (GTK_WINDOW_TOPLEVEL);
  ...
  video_window = gtk_drawing_area_new ();
  g_signal_connect (video_window, "realize",
      G_CALLBACK (video_widget_realize_cb), NULL);
  gtk_widget_set_double_buffered (video_window, FALSE);
  ...
  // usually the video_window will not be directly embedded into the
  // application window like this, but there will be many other widgets
  // and the video window will be embedded in one of them instead
  gtk_container_add (GTK_CONTAINER (ap_window), video_window);
  ...
  // show the GUI
  gtk_widget_show_all (app_window);

  // realize window now so that the video window gets created and we can
  // obtain its XID/HWND before the pipeline is started up and the videosink
  // asks for the XID/HWND of the window to render onto
  gtk_widget_realize (video_window);

  // we should have the XID/HWND now
  g_assert (video_window_handle != 0);
  ...
  // set up sync handler for setting the xid once the pipeline is started
  bus = gst_pipeline_get_bus (GST_PIPELINE (pipeline));
  gst_bus_set_sync_handler (bus, (GstBusSyncHandler) bus_sync_handler, NULL,
      NULL);
  gst_object_unref (bus);
  ...
  gst_element_set_state (pipeline, GST_STATE_PLAYING);
  ...
}
```

## GstVideoOverlay and Qt


```text
#include <glib.h>;
#include <gst/gst.h>;
#include <gst/video/videooverlay.h>;

#include <QApplication>;
#include <QTimer>;
#include <QWidget>;

int main(int argc, char *argv[])
{
  if (!g_thread_supported ())
    g_thread_init (NULL);

  gst_init (&argc, &argv);
  QApplication app(argc, argv);
  app.connect(&app, SIGNAL(lastWindowClosed()), &app, SLOT(quit ()));

  // prepare the pipeline

  GstElement *pipeline = gst_pipeline_new ("xvoverlay");
  GstElement *src = gst_element_factory_make ("videotestsrc", NULL);
  GstElement *sink = gst_element_factory_make ("xvimagesink", NULL);
  gst_bin_add_many (GST_BIN (pipeline), src, sink, NULL);
  gst_element_link (src, sink);

  // prepare the ui

  QWidget window;
  window.resize(320, 240);
  window.show();

  WId xwinid = window.winId();
  gst_video_overlay_set_window_handle (GST_VIDEO_OVERLAY (sink), xwinid);

  // run the pipeline

  GstStateChangeReturn sret = gst_element_set_state (pipeline,
      GST_STATE_PLAYING);
  if (sret == GST_STATE_CHANGE_FAILURE) {
    gst_element_set_state (pipeline, GST_STATE_NULL);
    gst_object_unref (pipeline);
    // Exit application
    QTimer::singleShot(0, QApplication::activeWindow(), SLOT(quit()));
  }

  int ret = app.exec();

  window.hide();
  gst_element_set_state (pipeline, GST_STATE_NULL);
  gst_object_unref (pipeline);

  return ret;
}
```

# Implements

[`VideoOverlayExt`](trait@crate::VideoOverlayExt), [`VideoOverlayExtManual`](trait@crate::VideoOverlayExtManual)
<!-- trait VideoOverlayExt -->
Trait containing all `VideoOverlay` methods.

# Implementors

[`VideoOverlay`](struct@crate::VideoOverlay)
<!-- impl VideoOverlay::fn install_properties -->
This helper shall be used by classes implementing the `VideoOverlay`
interface that want the render rectangle to be controllable using
properties. This helper will install "render-rectangle" property into the
class.

Feature: `v1_14`

## `oclass`
The class on which the properties will be installed
## `last_prop_id`
The first free property ID to use
<!-- impl VideoOverlay::fn set_property -->
This helper shall be used by classes implementing the `VideoOverlay`
interface that want the render rectangle to be controllable using
properties. This helper will parse and set the render rectangle calling
`VideoOverlay::set_render_rectangle`.

Feature: `v1_14`

## `object`
The instance on which the property is set
## `last_prop_id`
The highest property ID.
## `property_id`
The property ID
## `value`
The `glib::object::Value` to be set

# Returns

`true` if the `property_id` matches the GstVideoOverlay property
<!-- trait VideoOverlayExt::fn expose -->
Tell an overlay that it has been exposed. This will redraw the current frame
in the drawable even if the pipeline is PAUSED.
<!-- trait VideoOverlayExt::fn got_window_handle -->
This will post a "have-window-handle" element message on the bus.

This function should only be used by video overlay plugin developers.
## `handle`
a platform-specific handle referencing the window
<!-- trait VideoOverlayExt::fn handle_events -->
Tell an overlay that it should handle events from the window system. These
events are forwarded upstream as navigation events. In some window system,
events are not propagated in the window hierarchy if a client is listening
for them. This method allows you to disable events handling completely
from the `VideoOverlay`.
## `handle_events`
a `gboolean` indicating if events should be handled or not.
<!-- trait VideoOverlayExt::fn prepare_window_handle -->
This will post a "prepare-window-handle" element message on the bus
to give applications an opportunity to call
`VideoOverlay::set_window_handle` before a plugin creates its own
window.

This function should only be used by video overlay plugin developers.
<!-- trait VideoOverlayExt::fn set_render_rectangle -->
Configure a subregion as a video target within the window set by
`VideoOverlay::set_window_handle`. If this is not used or not supported
the video will fill the area of the window set as the overlay to 100%.
By specifying the rectangle, the video can be overlayed to a specific region
of that window only. After setting the new rectangle one should call
`VideoOverlay::expose` to force a redraw. To unset the region pass -1 for
the `width` and `height` parameters.

This method is needed for non fullscreen video overlay in UI toolkits that
do not support subwindows.
## `x`
the horizontal offset of the render area inside the window
## `y`
the vertical offset of the render area inside the window
## `width`
the width of the render area inside the window
## `height`
the height of the render area inside the window

# Returns

`false` if not supported by the sink.
<!-- trait VideoOverlayExt::fn set_window_handle -->
This will call the video overlay's set_window_handle method. You
should use this method to tell to an overlay to display video output to a
specific window (e.g. an XWindow on X11). Passing 0 as the `handle` will
tell the overlay to stop using that window and create an internal one.
## `handle`
a handle referencing the window.
<!-- struct VideoOverlayFormatFlags -->
Overlay format flags.
<!-- struct VideoOverlayFormatFlags::const NONE -->
no flags
<!-- struct VideoOverlayFormatFlags::const PREMULTIPLIED_ALPHA -->
RGB are premultiplied by A/255.
<!-- struct VideoOverlayFormatFlags::const GLOBAL_ALPHA -->
a global-alpha value != 1 is set.
<!-- struct VideoPackFlags -->
The different flags that can be used when packing and unpacking.
<!-- struct VideoPackFlags::const NONE -->
No flag
<!-- struct VideoPackFlags::const TRUNCATE_RANGE -->
When the source has a smaller depth
 than the target format, set the least significant bits of the target
 to 0. This is likely slightly faster but less accurate. When this flag
 is not specified, the most significant bits of the source are duplicated
 in the least significant bits of the destination.
<!-- struct VideoPackFlags::const INTERLACED -->
The source is interlaced. The unpacked
 format will be interlaced as well with each line containing
 information from alternating fields. (Since: 1.2)
<!-- enum VideoPrimariesMode -->
Different primaries conversion modes
<!-- enum VideoPrimariesMode::variant None -->
disable conversion between primaries
<!-- enum VideoPrimariesMode::variant MergeOnly -->
do conversion between primaries only
 when it can be merged with color matrix conversion.
<!-- enum VideoPrimariesMode::variant Fast -->
fast conversion between primaries
<!-- enum VideoResamplerMethod -->
Different subsampling and upsampling methods
<!-- enum VideoResamplerMethod::variant Nearest -->
Duplicates the samples when
 upsampling and drops when downsampling
<!-- enum VideoResamplerMethod::variant Linear -->
Uses linear interpolation to reconstruct
 missing samples and averaging to downsample
<!-- enum VideoResamplerMethod::variant Cubic -->
Uses cubic interpolation
<!-- enum VideoResamplerMethod::variant Sinc -->
Uses sinc interpolation
<!-- enum VideoResamplerMethod::variant Lanczos -->
Uses lanczos interpolation
<!-- struct VideoSink -->
Provides useful functions and a base class for video sinks.

GstVideoSink will configure the default base sink to drop frames that
arrive later than 20ms as this is considered the default threshold for
observing out-of-sync frames.

# Implements

[`VideoSinkExt`](trait@crate::VideoSinkExt), [`trait@gst_base::BaseSinkExt`], [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- trait VideoSinkExt -->
Trait containing all `VideoSink` methods.

# Implementors

[`VideoSink`](struct@crate::VideoSink)
<!-- impl VideoSink::fn center_rect -->
Takes `src` rectangle and position it at the center of `dst` rectangle with or
without `scaling`. It handles clipping if the `src` rectangle is bigger than
the `dst` one and `scaling` is set to FALSE.
## `src`
the `VideoRectangle` describing the source area
## `dst`
the `VideoRectangle` describing the destination area
## `result`
a pointer to a `VideoRectangle` which will receive the result area
## `scaling`
a `gboolean` indicating if scaling should be applied or not
<!-- trait VideoSinkExt::fn get_property_show_preroll_frame -->
Whether to show video frames during preroll. If set to `false`, video
frames will only be rendered in PLAYING state.
<!-- trait VideoSinkExt::fn set_property_show_preroll_frame -->
Whether to show video frames during preroll. If set to `false`, video
frames will only be rendered in PLAYING state.
<!-- enum VideoTileMode -->
Enum value describing the available tiling modes.
<!-- enum VideoTileMode::variant Unknown -->
Unknown or unset tile mode
<!-- enum VideoTileMode::variant Zflipz2x2 -->
Every four adjacent blocks - two
 horizontally and two vertically are grouped together and are located
 in memory in Z or flipped Z order. In case of odd rows, the last row
 of blocks is arranged in linear order.
<!-- enum VideoTileMode::variant Linear -->
Tiles are in row order.
<!-- struct VideoTimeCode -->
`field_count` must be 0 for progressive video and 1 or 2 for interlaced.

A representation of a SMPTE time code.

`hours` must be positive and less than 24. Will wrap around otherwise.
`minutes` and `seconds` must be positive and less than 60.
`frames` must be less than or equal to `config.fps_n` / `config.fps_d`
These values are *NOT* automatically normalized.

Feature: `v1_10`
<!-- impl VideoTimeCode::fn new -->
`field_count` is 0 for progressive, 1 or 2 for interlaced.
`latest_daiy_jam` reference is stolen from caller.

Feature: `v1_10`

## `fps_n`
Numerator of the frame rate
## `fps_d`
Denominator of the frame rate
## `latest_daily_jam`
The latest daily jam of the `VideoTimeCode`
## `flags`
`VideoTimeCodeFlags`
## `hours`
the hours field of `VideoTimeCode`
## `minutes`
the minutes field of `VideoTimeCode`
## `seconds`
the seconds field of `VideoTimeCode`
## `frames`
the frames field of `VideoTimeCode`
## `field_count`
Interlaced video field count

# Returns

a new `VideoTimeCode` with the given values.
The values are not checked for being in a valid range. To see if your
timecode actually has valid content, use `VideoTimeCode::is_valid`.
<!-- impl VideoTimeCode::fn new_empty -->

Feature: `v1_10`


# Returns

a new empty, invalid `VideoTimeCode`
<!-- impl VideoTimeCode::fn new_from_date_time -->
The resulting config->latest_daily_jam is set to
midnight, and timecode is set to the given time.

This might return a completely invalid timecode, use
`VideoTimeCode::new_from_date_time_full` to ensure
that you would get `None` instead in that case.

Feature: `v1_12`

## `fps_n`
Numerator of the frame rate
## `fps_d`
Denominator of the frame rate
## `dt`
`glib::DateTime` to convert
## `flags`
`VideoTimeCodeFlags`
## `field_count`
Interlaced video field count

# Returns

the `VideoTimeCode` representation of `dt`.
<!-- impl VideoTimeCode::fn new_from_date_time_full -->
The resulting config->latest_daily_jam is set to
midnight, and timecode is set to the given time.

Feature: `v1_16`

## `fps_n`
Numerator of the frame rate
## `fps_d`
Denominator of the frame rate
## `dt`
`glib::DateTime` to convert
## `flags`
`VideoTimeCodeFlags`
## `field_count`
Interlaced video field count

# Returns

the `VideoTimeCode` representation of `dt`, or `None` if
 no valid timecode could be created.
<!-- impl VideoTimeCode::fn new_from_string -->

Feature: `v1_12`

## `tc_str`
The string that represents the `VideoTimeCode`

# Returns

a new `VideoTimeCode` from the given string or `None`
 if the string could not be passed.
<!-- impl VideoTimeCode::fn add_frames -->
Adds or subtracts `frames` amount of frames to `self`. tc needs to
contain valid data, as verified by `VideoTimeCode::is_valid`.

Feature: `v1_10`

## `frames`
How many frames to add or subtract
<!-- impl VideoTimeCode::fn add_interval -->
This makes a component-wise addition of `tc_inter` to `self`. For example,
adding ("01:02:03:04", "00:01:00:00") will return "01:03:03:04".
When it comes to drop-frame timecodes,
adding ("00:00:00;00", "00:01:00:00") will return "00:01:00;02"
because of drop-frame oddities. However,
adding ("00:09:00;02", "00:01:00:00") will return "00:10:00;00"
because this time we can have an exact minute.

Feature: `v1_12`

## `tc_inter`
The `VideoTimeCodeInterval` to add to `self`.
The interval must contain valid values, except that for drop-frame
timecode, it may also contain timecodes which would normally
be dropped. These are then corrected to the next reasonable timecode.

# Returns

A new `VideoTimeCode` with `tc_inter` added or `None`
 if the interval can't be added.
<!-- impl VideoTimeCode::fn clear -->
Initializes `self` with empty/zero/NULL values and frees any memory
it might currently use.

Feature: `v1_10`

<!-- impl VideoTimeCode::fn compare -->
Compares `self` and `tc2`. If both have latest daily jam information, it is
taken into account. Otherwise, it is assumed that the daily jam of both
`self` and `tc2` was at the same time. Both time codes must be valid.

Feature: `v1_10`

## `tc2`
another valid `VideoTimeCode`

# Returns

1 if `self` is after `tc2`, -1 if `self` is before `tc2`, 0 otherwise.
<!-- impl VideoTimeCode::fn copy -->

Feature: `v1_10`


# Returns

a new `VideoTimeCode` with the same values as `self`.
<!-- impl VideoTimeCode::fn frames_since_daily_jam -->

Feature: `v1_10`


# Returns

how many frames have passed since the daily jam of `self`.
<!-- impl VideoTimeCode::fn free -->
Frees `self`.

Feature: `v1_10`

<!-- impl VideoTimeCode::fn increment_frame -->
Adds one frame to `self`.

Feature: `v1_10`

<!-- impl VideoTimeCode::fn init -->
`field_count` is 0 for progressive, 1 or 2 for interlaced.
`latest_daiy_jam` reference is stolen from caller.

Initializes `self` with the given values.
The values are not checked for being in a valid range. To see if your
timecode actually has valid content, use `VideoTimeCode::is_valid`.

Feature: `v1_10`

## `fps_n`
Numerator of the frame rate
## `fps_d`
Denominator of the frame rate
## `latest_daily_jam`
The latest daily jam of the `VideoTimeCode`
## `flags`
`VideoTimeCodeFlags`
## `hours`
the hours field of `VideoTimeCode`
## `minutes`
the minutes field of `VideoTimeCode`
## `seconds`
the seconds field of `VideoTimeCode`
## `frames`
the frames field of `VideoTimeCode`
## `field_count`
Interlaced video field count
<!-- impl VideoTimeCode::fn init_from_date_time -->
The resulting config->latest_daily_jam is set to midnight, and timecode is
set to the given time.

Will assert on invalid parameters, use `VideoTimeCode::init_from_date_time_full`
for being able to handle invalid parameters.

Feature: `v1_12`

## `fps_n`
Numerator of the frame rate
## `fps_d`
Denominator of the frame rate
## `dt`
`glib::DateTime` to convert
## `flags`
`VideoTimeCodeFlags`
## `field_count`
Interlaced video field count
<!-- impl VideoTimeCode::fn init_from_date_time_full -->
The resulting config->latest_daily_jam is set to
midnight, and timecode is set to the given time.

Feature: `v1_16`

## `fps_n`
Numerator of the frame rate
## `fps_d`
Denominator of the frame rate
## `dt`
`glib::DateTime` to convert
## `flags`
`VideoTimeCodeFlags`
## `field_count`
Interlaced video field count

# Returns

`true` if `self` could be correctly initialized to a valid timecode
<!-- impl VideoTimeCode::fn is_valid -->

Feature: `v1_10`


# Returns

whether `self` is a valid timecode (supported frame rate,
hours/minutes/seconds/frames not overflowing)
<!-- impl VideoTimeCode::fn nsec_since_daily_jam -->

Feature: `v1_10`


# Returns

how many nsec have passed since the daily jam of `self`.
<!-- impl VideoTimeCode::fn to_date_time -->
The `self.config`->latest_daily_jam is required to be non-NULL.

Feature: `v1_10`


# Returns

the `glib::DateTime` representation of `self` or `None` if `self`
 has no daily jam.
<!-- impl VideoTimeCode::fn to_string -->

Feature: `v1_10`


# Returns

the SMPTE ST 2059-1:2015 string representation of `self`. That will
take the form hh:mm:ss:ff. The last separator (between seconds and frames)
may vary:

';' for drop-frame, non-interlaced content and for drop-frame interlaced
field 2
',' for drop-frame interlaced field 1
':' for non-drop-frame, non-interlaced content and for non-drop-frame
interlaced field 2
'.' for non-drop-frame interlaced field 1
<!-- struct VideoTimeCodeFlags -->
Flags related to the time code information.
For drop frame, only 30000/1001 and 60000/1001 frame rates are supported.
<!-- struct VideoTimeCodeFlags::const NONE -->
No flags
<!-- struct VideoTimeCodeFlags::const DROP_FRAME -->
Whether we have drop frame rate
<!-- struct VideoTimeCodeFlags::const INTERLACED -->
Whether we have interlaced video

Feature: `v1_10`

<!-- struct VideoTimeCodeInterval -->
A representation of a difference between two `VideoTimeCode` instances.
Will not necessarily correspond to a real timecode (e.g. 00:00:10;00)

Feature: `v1_12`
<!-- impl VideoTimeCodeInterval::fn new -->

Feature: `v1_12`

## `hours`
the hours field of `VideoTimeCodeInterval`
## `minutes`
the minutes field of `VideoTimeCodeInterval`
## `seconds`
the seconds field of `VideoTimeCodeInterval`
## `frames`
the frames field of `VideoTimeCodeInterval`

# Returns

a new `VideoTimeCodeInterval` with the given values.
<!-- impl VideoTimeCodeInterval::fn new_from_string -->
`tc_inter_str` must only have ":" as separators.

Feature: `v1_12`

## `tc_inter_str`
The string that represents the `VideoTimeCodeInterval`

# Returns

a new `VideoTimeCodeInterval` from the given string
 or `None` if the string could not be passed.
<!-- impl VideoTimeCodeInterval::fn clear -->
Initializes `self` with empty/zero/NULL values.

Feature: `v1_12`

<!-- impl VideoTimeCodeInterval::fn copy -->

Feature: `v1_12`


# Returns

a new `VideoTimeCodeInterval` with the same values as `self`.
<!-- impl VideoTimeCodeInterval::fn free -->
Frees `self`.

Feature: `v1_12`

<!-- impl VideoTimeCodeInterval::fn init -->
Initializes `self` with the given values.

Feature: `v1_12`

## `hours`
the hours field of `VideoTimeCodeInterval`
## `minutes`
the minutes field of `VideoTimeCodeInterval`
## `seconds`
the seconds field of `VideoTimeCodeInterval`
## `frames`
the frames field of `VideoTimeCodeInterval`
<!-- enum VideoTransferFunction -->
The video transfer function defines the formula for converting between
non-linear RGB (R'G'B') and linear RGB
<!-- enum VideoTransferFunction::variant Unknown -->
unknown transfer function
<!-- enum VideoTransferFunction::variant Gamma10 -->
linear RGB, gamma 1.0 curve
<!-- enum VideoTransferFunction::variant Gamma18 -->
Gamma 1.8 curve
<!-- enum VideoTransferFunction::variant Gamma20 -->
Gamma 2.0 curve
<!-- enum VideoTransferFunction::variant Gamma22 -->
Gamma 2.2 curve
<!-- enum VideoTransferFunction::variant Bt709 -->
Gamma 2.2 curve with a linear segment in the lower
 range, also ITU-R BT470M / ITU-R BT1700 625 PAL &
 SECAM / ITU-R BT1361
<!-- enum VideoTransferFunction::variant Smpte240m -->
Gamma 2.2 curve with a linear segment in the
 lower range
<!-- enum VideoTransferFunction::variant Srgb -->
Gamma 2.4 curve with a linear segment in the lower
 range. IEC 61966-2-1 (sRGB or sYCC)
<!-- enum VideoTransferFunction::variant Gamma28 -->
Gamma 2.8 curve, also ITU-R BT470BG
<!-- enum VideoTransferFunction::variant Log100 -->
Logarithmic transfer characteristic
 100:1 range
<!-- enum VideoTransferFunction::variant Log316 -->
Logarithmic transfer characteristic
 316.22777:1 range (100 * sqrt(10) : 1)
<!-- enum VideoTransferFunction::variant Bt202012 -->
Gamma 2.2 curve with a linear segment in the lower
 range. Used for BT.2020 with 12 bits per
 component. Since: 1.6
<!-- enum VideoTransferFunction::variant Adobergb -->
Gamma 2.19921875. Since: 1.8
<!-- enum VideoTransferFunction::variant Bt202010 -->
Rec. ITU-R BT.2020-2 with 10 bits per component.
 (functionally the same as the values
 GST_VIDEO_TRANSFER_BT709 and GST_VIDEO_TRANSFER_BT601).
 Since: 1.18
<!-- enum VideoTransferFunction::variant Smpte2084 -->
SMPTE ST 2084 for 10, 12, 14, and 16-bit systems.
 Known as perceptual quantization (PQ)
 Since: 1.18
<!-- enum VideoTransferFunction::variant AribStdB67 -->
Association of Radio Industries and Businesses (ARIB)
 STD-B67 and Rec. ITU-R BT.2100-1 hybrid loggamma (HLG) system
 Since: 1.18
<!-- enum VideoTransferFunction::variant Bt601 -->
also known as SMPTE170M / ITU-R BT1358 525 or 625 / ITU-R BT1700 NTSC
