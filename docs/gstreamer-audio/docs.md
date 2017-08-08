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
