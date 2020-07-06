<!-- file * -->
<!-- struct Discoverer -->
The `Discoverer` is a utility object which allows to get as much
information as possible from one or many URIs.

It provides two APIs, allowing usage in blocking or non-blocking mode.

The blocking mode just requires calling `Discoverer::discover_uri`
with the URI one wishes to discover.

The non-blocking mode requires a running `glib::MainLoop` iterating a
`glib::MainContext`, where one connects to the various signals, appends the
URIs to be processed (through `Discoverer::discover_uri_async`) and then
asks for the discovery to begin (through `Discoverer::start`).
By default this will use the GLib default main context unless you have
set a custom context using `glib::MainContext::push_thread_default`.

All the information is returned in a `DiscovererInfo` structure.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl Discoverer::fn new -->
Creates a new `Discoverer` with the provided timeout.
## `timeout`
timeout per file, in nanoseconds. Allowed are values between
 one second (`GST_SECOND`) and one hour (3600 * `GST_SECOND`)

# Returns

The new `Discoverer`.
If an error occurred when creating the discoverer, `err` will be set
accordingly and `None` will be returned. If `err` is set, the caller must
free it when no longer needed using `glib::Error::free`.
<!-- impl Discoverer::fn discover_uri -->
Synchronously discovers the given `uri`.

A copy of `uri` will be made internally, so the caller can safely `g_free`
afterwards.
## `uri`
The URI to run on.

# Returns

the result of the scanning. Can be `None` if an
error occurred.
<!-- impl Discoverer::fn discover_uri_async -->
Appends the given `uri` to the list of URIs to discoverer. The actual
discovery of the `uri` will only take place if `Discoverer::start` has
been called.

A copy of `uri` will be made internally, so the caller can safely `g_free`
afterwards.
## `uri`
the URI to add.

# Returns

`true` if the `uri` was successfully appended to the list of pending
uris, else `false`
<!-- impl Discoverer::fn start -->
Allow asynchronous discovering of URIs to take place.
A `glib::MainLoop` must be available for `Discoverer` to properly work in
asynchronous mode.
<!-- impl Discoverer::fn stop -->
Stop the discovery of any pending URIs and clears the list of
pending URIS (if any).
<!-- impl Discoverer::fn connect_discovered -->
Will be emitted in async mode when all information on a URI could be
discovered, or an error occurred.

When an error occurs, `info` might still contain some partial information,
depending on the circumstances of the error.
## `info`
the results `DiscovererInfo`
## `error`
`glib::Error`, which will be non-NULL
 if an error occurred during
 discovery. You must not free
 this `glib::Error`, it will be freed by
 the discoverer.
<!-- impl Discoverer::fn connect_finished -->
Will be emitted in async mode when all pending URIs have been processed.
<!-- impl Discoverer::fn connect_source_setup -->
This signal is emitted after the source element has been created for, so
the URI being discovered, so it can be configured by setting additional
properties (e.g. set a proxy server for an http source, or set the device
and read speed for an audio cd source).

This signal is usually emitted from the context of a GStreamer streaming
thread.
## `source`
source element
<!-- impl Discoverer::fn connect_starting -->
Will be emitted when the discover starts analyzing the pending URIs
<!-- impl Discoverer::fn get_property_timeout -->
The duration (in nanoseconds) after which the discovery of an individual
URI will timeout.

If the discovery of a URI times out, the `DiscovererResult::Timeout` will be
set on the result flags.
<!-- impl Discoverer::fn set_property_timeout -->
The duration (in nanoseconds) after which the discovery of an individual
URI will timeout.

If the discovery of a URI times out, the `DiscovererResult::Timeout` will be
set on the result flags.
<!-- struct DiscovererAudioInfo -->
`DiscovererStreamInfo` specific to audio streams.

# Implements

[`DiscovererStreamInfoExt`](trait.DiscovererStreamInfoExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl DiscovererAudioInfo::fn get_bitrate -->

# Returns

the average or nominal bitrate of the stream in bits/second.
<!-- impl DiscovererAudioInfo::fn get_channel_mask -->

Feature: `v1_14`


# Returns

the channel-mask of the stream, refer to
`gst_audio_channel_positions_from_mask` for more
information.
<!-- impl DiscovererAudioInfo::fn get_channels -->

# Returns

the number of channels in the stream.
<!-- impl DiscovererAudioInfo::fn get_depth -->

# Returns

the number of bits used per sample in each channel.
<!-- impl DiscovererAudioInfo::fn get_language -->

# Returns

the language of the stream, or NULL if unknown.
<!-- impl DiscovererAudioInfo::fn get_max_bitrate -->

# Returns

the maximum bitrate of the stream in bits/second.
<!-- impl DiscovererAudioInfo::fn get_sample_rate -->

# Returns

the sample rate of the stream in Hertz.
<!-- struct DiscovererContainerInfo -->
`DiscovererStreamInfo` specific to container streams.

# Implements

[`DiscovererStreamInfoExt`](trait.DiscovererStreamInfoExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl DiscovererContainerInfo::fn get_streams -->

# Returns

the list of
`DiscovererStreamInfo` this container stream offers.
Free with `DiscovererStreamInfo::list_free` after usage.
<!-- struct DiscovererInfo -->
Structure containing the information of a URI analyzed by `Discoverer`.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl DiscovererInfo::fn from_variant -->
Parses a `glib::Variant` as produced by `DiscovererInfo::to_variant`
back to a `DiscovererInfo`.
## `variant`
A `glib::Variant` to deserialize into a `DiscovererInfo`.

# Returns

A newly-allocated `DiscovererInfo`.
<!-- impl DiscovererInfo::fn copy -->

# Returns

A copy of the `DiscovererInfo`
<!-- impl DiscovererInfo::fn get_audio_streams -->
Finds all the `DiscovererAudioInfo` contained in `self`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- impl DiscovererInfo::fn get_container_streams -->
Finds all the `DiscovererContainerInfo` contained in `self`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- impl DiscovererInfo::fn get_duration -->

# Returns

the duration of the URI in `gst::ClockTime` (nanoseconds).
<!-- impl DiscovererInfo::fn get_live -->

Feature: `v1_14`


# Returns

whether the URI is live.
<!-- impl DiscovererInfo::fn get_misc -->

# Deprecated

This functions is deprecated since version 1.4, use
`DiscovererInfo::get_missing_elements_installer_details`

# Returns

Miscellaneous information stored as a `gst::Structure`
(for example: information about missing plugins). If you wish to use the
`gst::Structure` after the life-time of `self`, you will need to copy it.
<!-- impl DiscovererInfo::fn get_missing_elements_installer_details -->
Get the installer details for missing elements

# Returns

An array of strings
containing information about how to install the various missing elements
for `self` to be usable. If you wish to use the strings after the life-time
of `self`, you will need to copy them.
<!-- impl DiscovererInfo::fn get_result -->

# Returns

the result of the discovery as a `DiscovererResult`.
<!-- impl DiscovererInfo::fn get_seekable -->

# Returns

the whether the URI is seekable.
<!-- impl DiscovererInfo::fn get_stream_info -->

# Returns

the structure (or topology) of the URI as a
`DiscovererStreamInfo`.
This structure can be traversed to see the original hierarchy. Unref with
`gst_discoverer_stream_info_unref` after usage.
<!-- impl DiscovererInfo::fn get_stream_list -->

# Returns

the list of
all streams contained in the `info`. Free after usage
with `DiscovererStreamInfo::list_free`.
<!-- impl DiscovererInfo::fn get_streams -->
Finds the `DiscovererStreamInfo` contained in `self` that match the
given `streamtype`.
## `streamtype`
a `glib::Type` derived from `DiscovererStreamInfo`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- impl DiscovererInfo::fn get_subtitle_streams -->
Finds all the `DiscovererSubtitleInfo` contained in `self`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- impl DiscovererInfo::fn get_tags -->

# Returns

all tags contained in the URI. If you wish to use
the tags after the life-time of `self`, you will need to copy them.
<!-- impl DiscovererInfo::fn get_toc -->

# Returns

TOC contained in the URI. If you wish to use
the TOC after the life-time of `self`, you will need to copy it.
<!-- impl DiscovererInfo::fn get_uri -->

# Returns

the URI to which this information corresponds to.
Copy it if you wish to use it after the life-time of `self`.
<!-- impl DiscovererInfo::fn get_video_streams -->
Finds all the `DiscovererVideoInfo` contained in `self`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- impl DiscovererInfo::fn to_variant -->
Serializes `self` to a `glib::Variant` that can be parsed again
through `DiscovererInfo::from_variant`.

Note that any `gst::Toc` (s) that might have been discovered will not be serialized
for now.
## `flags`
A combination of `DiscovererSerializeFlags` to specify
what needs to be serialized.

# Returns

A newly-allocated `glib::Variant` representing `self`.
<!-- enum DiscovererResult -->
Result values for the discovery process.
<!-- enum DiscovererResult::variant Ok -->
The discovery was successful
<!-- enum DiscovererResult::variant UriInvalid -->
the URI is invalid
<!-- enum DiscovererResult::variant Error -->
an error happened and the GError is set
<!-- enum DiscovererResult::variant Timeout -->
the discovery timed-out
<!-- enum DiscovererResult::variant Busy -->
the discoverer was already discovering a file
<!-- enum DiscovererResult::variant MissingPlugins -->
Some plugins are missing for full discovery
<!-- struct DiscovererSerializeFlags -->
You can use these flags to control what is serialized by
`DiscovererInfo::to_variant`
<!-- struct DiscovererSerializeFlags::const BASIC -->
Serialize only basic information, excluding
caps, tags and miscellaneous information
<!-- struct DiscovererSerializeFlags::const CAPS -->
Serialize the caps for each stream
<!-- struct DiscovererSerializeFlags::const TAGS -->
Serialize the tags for each stream
<!-- struct DiscovererSerializeFlags::const MISC -->
Serialize miscellaneous information for each stream
<!-- struct DiscovererSerializeFlags::const ALL -->
Serialize all the available info, including
caps, tags and miscellaneous information
<!-- struct DiscovererStreamInfo -->
Base structure for information concerning a media stream. Depending on the
stream type, one can find more media-specific information in
`DiscovererAudioInfo`, `DiscovererVideoInfo`, and
`DiscovererContainerInfo`.

The `DiscovererStreamInfo` represents the topology of the stream. Siblings
can be iterated over with `DiscovererStreamInfoExt::get_next` and
`DiscovererStreamInfoExt::get_previous`. Children (sub-streams) of a
stream can be accessed using the `DiscovererContainerInfo` API.

As a simple example, if you run `Discoverer` on an AVI file with one audio
and one video stream, you will get a `DiscovererContainerInfo`
corresponding to the AVI container, which in turn will have a
`DiscovererAudioInfo` sub-stream and a `DiscovererVideoInfo` sub-stream
for the audio and video streams respectively.

# Implements

[`DiscovererStreamInfoExt`](trait.DiscovererStreamInfoExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait DiscovererStreamInfoExt -->
Trait containing all `DiscovererStreamInfo` methods.

# Implementors

[`DiscovererAudioInfo`](struct.DiscovererAudioInfo.html), [`DiscovererContainerInfo`](struct.DiscovererContainerInfo.html), [`DiscovererStreamInfo`](struct.DiscovererStreamInfo.html), [`DiscovererSubtitleInfo`](struct.DiscovererSubtitleInfo.html), [`DiscovererVideoInfo`](struct.DiscovererVideoInfo.html)
<!-- impl DiscovererStreamInfo::fn list_free -->
Decrements the reference count of all contained `DiscovererStreamInfo`
and fress the `glib::List`.
## `infos`
a `glib::List` of `DiscovererStreamInfo`
<!-- trait DiscovererStreamInfoExt::fn get_caps -->

# Returns

the `gst::Caps` of the stream. Unref with
`gst::Caps::unref` after usage.
<!-- trait DiscovererStreamInfoExt::fn get_misc -->

# Deprecated

This functions is deprecated since version 1.4, use
`DiscovererInfo::get_missing_elements_installer_details`

# Returns

additional information regarding the stream (for
example codec version, profile, etc..). If you wish to use the `gst::Structure`
after the life-time of `self` you will need to copy it.
<!-- trait DiscovererStreamInfoExt::fn get_next -->

# Returns

the next `DiscovererStreamInfo` in a chain. `None`
for final streams.
Unref with `gst_discoverer_stream_info_unref` after usage.
<!-- trait DiscovererStreamInfoExt::fn get_previous -->

# Returns

the previous `DiscovererStreamInfo` in a chain.
`None` for starting points. Unref with `gst_discoverer_stream_info_unref`
after usage.
<!-- trait DiscovererStreamInfoExt::fn get_stream_id -->

# Returns

the stream ID of this stream. If you wish to
use the stream ID after the life-time of `self` you will need to copy it.
<!-- trait DiscovererStreamInfoExt::fn get_stream_type_nick -->

# Returns

a human readable name for the stream type of the given `self` (ex : "audio",
"container",...).
<!-- trait DiscovererStreamInfoExt::fn get_tags -->

# Returns

the tags contained in this stream. If you wish to
use the tags after the life-time of `self` you will need to copy them.
<!-- trait DiscovererStreamInfoExt::fn get_toc -->

# Returns

the TOC contained in this stream. If you wish to
use the TOC after the life-time of `self` you will need to copy it.
<!-- struct DiscovererSubtitleInfo -->
`DiscovererStreamInfo` specific to subtitle streams (this includes text and
image based ones).

# Implements

[`DiscovererStreamInfoExt`](trait.DiscovererStreamInfoExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl DiscovererSubtitleInfo::fn get_language -->

# Returns

the language of the stream, or NULL if unknown.
<!-- struct DiscovererVideoInfo -->
`DiscovererStreamInfo` specific to video streams (this includes images).

# Implements

[`DiscovererStreamInfoExt`](trait.DiscovererStreamInfoExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl DiscovererVideoInfo::fn get_bitrate -->

# Returns

the average or nominal bitrate of the video stream in bits/second.
<!-- impl DiscovererVideoInfo::fn get_depth -->

# Returns

the depth in bits of the video stream.
<!-- impl DiscovererVideoInfo::fn get_framerate_denom -->

# Returns

the framerate of the video stream (denominator).
<!-- impl DiscovererVideoInfo::fn get_framerate_num -->

# Returns

the framerate of the video stream (numerator).
<!-- impl DiscovererVideoInfo::fn get_height -->

# Returns

the height of the video stream in pixels.
<!-- impl DiscovererVideoInfo::fn get_max_bitrate -->

# Returns

the maximum bitrate of the video stream in bits/second.
<!-- impl DiscovererVideoInfo::fn get_par_denom -->

# Returns

the Pixel Aspect Ratio (PAR) of the video stream (denominator).
<!-- impl DiscovererVideoInfo::fn get_par_num -->

# Returns

the Pixel Aspect Ratio (PAR) of the video stream (numerator).
<!-- impl DiscovererVideoInfo::fn get_width -->

# Returns

the width of the video stream in pixels.
<!-- impl DiscovererVideoInfo::fn is_image -->

# Returns

`true` if the video stream corresponds to an image (i.e. only contains
one frame).
<!-- impl DiscovererVideoInfo::fn is_interlaced -->

# Returns

`true` if the stream is interlaced, else `false`.
<!-- struct EncodingAudioProfile -->
Variant of `EncodingProfile` for audio streams.

# Implements

[`EncodingProfileExt`](trait.EncodingProfileExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl EncodingAudioProfile::fn new -->
Creates a new `EncodingAudioProfile`

All provided allocatable arguments will be internally copied, so can be
safely freed/unreferenced after calling this method.
## `format`
the `gst::Caps`
## `preset`
the preset(s) to use on the encoder, can be `None`
## `restriction`
the `gst::Caps` used to restrict the input to the encoder, can be
NULL. See `EncodingProfile::get_restriction` for more details.
## `presence`
the number of time this stream must be used. 0 means any number of
 times (including never)

# Returns

the newly created `EncodingAudioProfile`.
<!-- struct EncodingContainerProfile -->
Encoding profiles for containers. Keeps track of a list of `EncodingProfile`

# Implements

[`EncodingProfileExt`](trait.EncodingProfileExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl EncodingContainerProfile::fn new -->
Creates a new `EncodingContainerProfile`.
## `name`
The name of the container profile, can be `None`
## `description`
The description of the container profile,
 can be `None`
## `format`
The format to use for this profile
## `preset`
The preset to use for this profile.

# Returns

The newly created `EncodingContainerProfile`.
<!-- impl EncodingContainerProfile::fn add_profile -->
Add a `EncodingProfile` to the list of profiles handled by `self`.

No copy of `profile` will be made, if you wish to use it elsewhere after this
method you should increment its reference count.
## `profile`
the `EncodingProfile` to add.

# Returns

`true` if the `stream` was properly added, else `false`.
<!-- impl EncodingContainerProfile::fn contains_profile -->
Checks if `self` contains a `EncodingProfile` identical to
`profile`.
## `profile`
a `EncodingProfile`

# Returns

`true` if `self` contains a `EncodingProfile` identical
to `profile`, else `false`.
<!-- impl EncodingContainerProfile::fn get_profiles -->

# Returns


the list of contained `EncodingProfile`.
<!-- struct EncodingProfile -->
The opaque base class object for all encoding profiles. This contains generic
information like name, description, format and preset.

# Implements

[`EncodingProfileExt`](trait.EncodingProfileExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait EncodingProfileExt -->
Trait containing all `EncodingProfile` methods.

# Implementors

[`EncodingAudioProfile`](struct.EncodingAudioProfile.html), [`EncodingContainerProfile`](struct.EncodingContainerProfile.html), [`EncodingProfile`](struct.EncodingProfile.html), [`EncodingVideoProfile`](struct.EncodingVideoProfile.html)
<!-- impl EncodingProfile::fn find -->
Find the `EncodingProfile` with the specified name and category.
## `targetname`
The name of the target
## `profilename`
The name of the profile, if `None`
provided, it will default to the encoding profile called `default`.
## `category`
The target category. Can be `None`

# Returns

The matching `EncodingProfile` or `None`.
<!-- impl EncodingProfile::fn from_discoverer -->
Creates a `EncodingProfile` matching the formats from the given
`DiscovererInfo`. Streams other than audio or video (eg,
subtitles), are currently ignored.
## `info`
The `DiscovererInfo` to read from

# Returns

The new `EncodingProfile` or `None`.
<!-- trait EncodingProfileExt::fn copy -->
Makes a deep copy of `self`

Feature: `v1_12`


# Returns

The copy of `self`
<!-- trait EncodingProfileExt::fn get_allow_dynamic_output -->
Get whether the format that has been negotiated in at some point can be renegotiated
later during the encoding.
<!-- trait EncodingProfileExt::fn get_description -->

# Returns

the description of the profile, can be `None`.
<!-- trait EncodingProfileExt::fn get_file_extension -->

# Returns

a suitable file extension for `self`, or NULL.
<!-- trait EncodingProfileExt::fn get_format -->

# Returns

the `gst::Caps` corresponding to the media format used
in the profile. Unref after usage.
<!-- trait EncodingProfileExt::fn get_input_caps -->
Computes the full output caps that this `self` will be able to consume.

# Returns

The full caps the given `self` can consume. Call
`gst::Caps::unref` when you are done with the caps.
<!-- trait EncodingProfileExt::fn get_name -->

# Returns

the name of the profile, can be `None`.
<!-- trait EncodingProfileExt::fn get_presence -->

# Returns

The number of times the profile is used in its parent
container profile. If 0, it is not a mandatory stream.
<!-- trait EncodingProfileExt::fn get_preset -->

# Returns

the name of the `gst::Preset` to be used in the profile.
This is the name that has been set when saving the preset.
<!-- trait EncodingProfileExt::fn get_preset_name -->

# Returns

the name of the `gst::Preset` factory to be used in the profile.
<!-- trait EncodingProfileExt::fn get_restriction -->

# Returns

The restriction `gst::Caps` to apply before the encoder
that will be used in the profile. The fields present in restriction caps are
properties of the raw stream (that is before encoding), such as height and
width for video and depth and sampling rate for audio. Does not apply to
`EncodingContainerProfile` (since there is no corresponding raw stream).
Can be `None`. Unref after usage.
<!-- trait EncodingProfileExt::fn get_single_segment -->

Feature: `v1_18`


# Returns

`true` if the stream represented by `self` should use a single
segment before the encoder, `false` otherwise. This means that buffers will be retimestamped
and segments will be eat so as to appear as one segment.
<!-- trait EncodingProfileExt::fn get_type_nick -->

# Returns

the human-readable name of the type of `self`.
<!-- trait EncodingProfileExt::fn is_equal -->
Checks whether the two `EncodingProfile` are equal
## `b`
a `EncodingProfile`

# Returns

`true` if `self` and `b` are equal, else `false`.
<!-- trait EncodingProfileExt::fn set_allow_dynamic_output -->
Sets whether the format that has been negotiated in at some point can be renegotiated
later during the encoding.
## `allow_dynamic_output`
Whether the format that has been negotiated first can be renegotiated
during the encoding
<!-- trait EncodingProfileExt::fn set_description -->
Set `description` as the given description for the `self`. A copy of
`description` will be made internally.
## `description`
the description to set on the profile
<!-- trait EncodingProfileExt::fn set_enabled -->
Set whether the profile should be used or not.
## `enabled`
`false` to disable `self`, `true` to enable it
<!-- trait EncodingProfileExt::fn set_format -->
Sets the media format used in the profile.
## `format`
the media format to use in the profile.
<!-- trait EncodingProfileExt::fn set_name -->
Set `name` as the given name for the `self`. A copy of `name` will be made
internally.
## `name`
the name to set on the profile
<!-- trait EncodingProfileExt::fn set_presence -->
Set the number of time the profile is used in its parent
container profile. If 0, it is not a mandatory stream
## `presence`
the number of time the profile can be used
<!-- trait EncodingProfileExt::fn set_preset -->
Sets the name of the `gst::Element` that implements the `gst::Preset` interface
to use for the profile.
This is the name that has been set when saving the preset.
## `preset`
the element preset to use
<!-- trait EncodingProfileExt::fn set_preset_name -->
Sets the name of the `gst::Preset`'s factory to be used in the profile.
## `preset_name`
The name of the preset to use in this `self`.
<!-- trait EncodingProfileExt::fn set_restriction -->
Set the restriction `gst::Caps` to apply before the encoder
that will be used in the profile. See `EncodingProfile::get_restriction`
for more about restrictions. Does not apply to `EncodingContainerProfile`.
## `restriction`
the restriction to apply
<!-- trait EncodingProfileExt::fn set_single_segment -->
If using a single segment, buffers will be retimestamped
and segments will be eat so as to appear as one segment.

Feature: `v1_18`

## `single_segment`
`true` if the stream represented by `self` should use a single
segment before the encoder `false` otherwise.
<!-- struct EncodingTarget -->
Collection of `EncodingProfile` for a specific target or use-case.

When being stored/loaded, targets come from a specific category, like
`GST_ENCODING_CATEGORY_DEVICE`.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl EncodingTarget::fn new -->
Creates a new `EncodingTarget`.

The name and category can only consist of lowercase ASCII letters for the
first character, followed by either lowercase ASCII letters, digits or
hyphens ('-').

The `category` *should* be one of the existing
well-defined categories, like `GST_ENCODING_CATEGORY_DEVICE`, but it
*can* be a application or user specific category if
needed.
## `name`
The name of the target.
## `category`
The name of the category to which this `target`
belongs. For example: `GST_ENCODING_CATEGORY_DEVICE`.
## `description`
A description of `EncodingTarget` in the
current locale.
## `profiles`
A `glib::List` of
`EncodingProfile`.

# Returns

The newly created `EncodingTarget` or `None` if
there was an error.
<!-- impl EncodingTarget::fn load -->
Searches for the `EncodingTarget` with the given name, loads it
and returns it.

If the category name is specified only targets from that category will be
searched for.
## `name`
the name of the `EncodingTarget` to load (automatically
converted to lower case internally as capital letters are not
valid for target names).
## `category`
the name of the target category, like
`GST_ENCODING_CATEGORY_DEVICE`. Can be `None`

# Returns

The `EncodingTarget` if available, else `None`.
<!-- impl EncodingTarget::fn load_from_file -->
Opens the provided file and returns the contained `EncodingTarget`.
## `filepath`
The file location to load the `EncodingTarget` from

# Returns

The `EncodingTarget` contained in the file, else
`None`
<!-- impl EncodingTarget::fn add_profile -->
Adds the given `profile` to the `self`. Each added profile must have
a unique name within the profile.

The `self` will steal a reference to the `profile`. If you wish to use
the profile after calling this method, you should increase its reference
count.
## `profile`
the `EncodingProfile` to add

# Returns

`true` if the profile was added, else `false`.
<!-- impl EncodingTarget::fn get_category -->

# Returns

The category of the `self`. For example:
`GST_ENCODING_CATEGORY_DEVICE`.
<!-- impl EncodingTarget::fn get_description -->

# Returns

The description of the `self`.
<!-- impl EncodingTarget::fn get_name -->

# Returns

The name of the `self`.
<!-- impl EncodingTarget::fn get_path -->

Feature: `v1_18`


# Returns

The path to the `self` file.
<!-- impl EncodingTarget::fn get_profile -->
## `name`
the name of the profile to retrieve

# Returns

The matching `EncodingProfile`, or `None`.
<!-- impl EncodingTarget::fn get_profiles -->

# Returns

A list of
`EncodingProfile`(s) this `self` handles.
<!-- impl EncodingTarget::fn save -->
Saves the `self` to a default user-local directory.

# Returns

`true` if the target was correctly saved, else `false`.
<!-- impl EncodingTarget::fn save_to_file -->
Saves the `self` to the provided file location.
## `filepath`
the location to store the `self` at.

# Returns

`true` if the target was correctly saved, else `false`.
<!-- struct EncodingVideoProfile -->
Variant of `EncodingProfile` for video streams, allows specifying the `pass`.

# Implements

[`EncodingProfileExt`](trait.EncodingProfileExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl EncodingVideoProfile::fn new -->
Creates a new `EncodingVideoProfile`

All provided allocatable arguments will be internally copied, so can be
safely freed/unreferenced after calling this method.

If you wish to control the pass number (in case of multi-pass scenarios),
please refer to the `EncodingVideoProfile::set_pass` documentation.

If you wish to use/force a constant framerate please refer to the
`EncodingVideoProfile::set_variableframerate` documentation.
## `format`
the `gst::Caps`
## `preset`
the preset(s) to use on the encoder, can be `None`
## `restriction`
the `gst::Caps` used to restrict the input to the encoder, can be
NULL. See `EncodingProfile::get_restriction` for more details.
## `presence`
the number of time this stream must be used. 0 means any number of
 times (including never)

# Returns

the newly created `EncodingVideoProfile`.
<!-- impl EncodingVideoProfile::fn get_pass -->
Get the pass number if this is part of a multi-pass profile.

# Returns

The pass number. Starts at 1 for multi-pass. 0 if this is
not a multi-pass profile
<!-- impl EncodingVideoProfile::fn get_variableframerate -->

# Returns

Whether non-constant video framerate is allowed for encoding.
<!-- impl EncodingVideoProfile::fn set_pass -->
Sets the pass number of this video profile. The first pass profile should have
this value set to 1. If this video profile isn't part of a multi-pass profile,
you may set it to 0 (the default value).
## `pass`
the pass number for this profile
<!-- impl EncodingVideoProfile::fn set_variableframerate -->
If set to `true`, then the incoming stream will be allowed to have non-constant
framerate. If set to `false` (default value), then the incoming stream will
be normalized by dropping/duplicating frames in order to produce a
constance framerate.
## `variableframerate`
a boolean
