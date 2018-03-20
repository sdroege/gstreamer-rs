<!-- file * -->
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
`gst_audio_discont_wait_set_rate` is called with a new `rate`.
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
