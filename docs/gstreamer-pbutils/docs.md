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
<!-- trait DiscovererExt::fn connect_discovered -->
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
<!-- trait DiscovererExt::fn connect_finished -->
Will be emitted in async mode when all pending URIs have been processed.
<!-- trait DiscovererExt::fn connect_source_setup -->
This signal is emitted after the source element has been created for, so
the URI being discovered, so it can be configured by setting additional
properties (e.g. set a proxy server for an http source, or set the device
and read speed for an audio cd source).

This signal is usually emitted from the context of a GStreamer streaming
thread.
## `source`
source element
<!-- trait DiscovererExt::fn connect_starting -->
Will be emitted when the discover starts analyzing the pending URIs
<!-- trait DiscovererExt::fn get_property_timeout -->
The duration (in nanoseconds) after which the discovery of an individual
URI will timeout.

If the discovery of a URI times out, the `DiscovererResult::Timeout` will be
set on the result flags.
<!-- trait DiscovererExt::fn set_property_timeout -->
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

[`DiscovererInfoExt`](trait.DiscovererInfoExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait DiscovererInfoExt -->
Trait containing all `DiscovererInfo` methods.

# Implementors

[`DiscovererInfo`](struct.DiscovererInfo.html)
<!-- impl DiscovererInfo::fn from_variant -->
Parses a `glib::Variant` as produced by `DiscovererInfoExt::to_variant`
back to a `DiscovererInfo`.
## `variant`
A `glib::Variant` to deserialize into a `DiscovererInfo`.

# Returns

A newly-allocated `DiscovererInfo`.
<!-- trait DiscovererInfoExt::fn copy -->

# Returns

A copy of the `DiscovererInfo`
<!-- trait DiscovererInfoExt::fn get_audio_streams -->
Finds all the `DiscovererAudioInfo` contained in `self`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- trait DiscovererInfoExt::fn get_container_streams -->
Finds all the `DiscovererContainerInfo` contained in `self`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- trait DiscovererInfoExt::fn get_duration -->

# Returns

the duration of the URI in `gst::ClockTime` (nanoseconds).
<!-- trait DiscovererInfoExt::fn get_live -->

Feature: `v1_14`


# Returns

whether the URI is live.
<!-- trait DiscovererInfoExt::fn get_misc -->

# Deprecated

This functions is deprecated since version 1.4, use
`DiscovererInfoExt::get_missing_elements_installer_details`

# Returns

Miscellaneous information stored as a `gst::Structure`
(for example: information about missing plugins). If you wish to use the
`gst::Structure` after the life-time of `self`, you will need to copy it.
<!-- trait DiscovererInfoExt::fn get_missing_elements_installer_details -->
Get the installer details for missing elements

# Returns

An array of strings
containing informations about how to install the various missing elements
for `self` to be usable. If you wish to use the strings after the life-time
of `self`, you will need to copy them.
<!-- trait DiscovererInfoExt::fn get_result -->

# Returns

the result of the discovery as a `DiscovererResult`.
<!-- trait DiscovererInfoExt::fn get_seekable -->

# Returns

the whether the URI is seekable.
<!-- trait DiscovererInfoExt::fn get_stream_info -->

# Returns

the structure (or topology) of the URI as a
`DiscovererStreamInfo`.
This structure can be traversed to see the original hierarchy. Unref with
`gst_discoverer_stream_info_unref` after usage.
<!-- trait DiscovererInfoExt::fn get_stream_list -->

# Returns

the list of
all streams contained in the `info`. Free after usage
with `DiscovererStreamInfo::list_free`.
<!-- trait DiscovererInfoExt::fn get_streams -->
Finds the `DiscovererStreamInfo` contained in `self` that match the
given `streamtype`.
## `streamtype`
a `glib::Type` derived from `DiscovererStreamInfo`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- trait DiscovererInfoExt::fn get_subtitle_streams -->
Finds all the `DiscovererSubtitleInfo` contained in `self`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- trait DiscovererInfoExt::fn get_tags -->

# Returns

all tags contained in the URI. If you wish to use
the tags after the life-time of `self`, you will need to copy them.
<!-- trait DiscovererInfoExt::fn get_toc -->

# Returns

TOC contained in the URI. If you wish to use
the TOC after the life-time of `self`, you will need to copy it.
<!-- trait DiscovererInfoExt::fn get_uri -->

# Returns

the URI to which this information corresponds to.
Copy it if you wish to use it after the life-time of `self`.
<!-- trait DiscovererInfoExt::fn get_video_streams -->
Finds all the `DiscovererVideoInfo` contained in `self`

# Returns

A `glib::List` of
matching `DiscovererStreamInfo`. The caller should free it with
`DiscovererStreamInfo::list_free`.
<!-- trait DiscovererInfoExt::fn to_variant -->
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
`gst_caps_unref` after usage.
<!-- trait DiscovererStreamInfoExt::fn get_misc -->

# Deprecated

This functions is deprecated since version 1.4, use
`DiscovererInfoExt::get_missing_elements_installer_details`

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
