<!-- file * -->
<!-- struct Player -->


# Implements

[`ObjectExt`](trait.ObjectExt.html)
<!-- impl Player::fn new -->
Creates a new `Player` instance that uses `signal_dispatcher` to dispatch
signals to some event loop system, or emits signals directly if NULL is
passed. See `PlayerGMainContextSignalDispatcher::new`.

Video is going to be rendered by `video_renderer`, or if `None` is provided
no special video set up will be done and some default handling will be
performed.
## `video_renderer`
GstPlayerVideoRenderer to use
## `signal_dispatcher`
GstPlayerSignalDispatcher to use

# Returns

a new `Player` instance
<!-- impl Player::fn config_get_position_update_interval -->
## `config`
a `Player` configuration

# Returns

current position update interval in milliseconds

Since 1.10
<!-- impl Player::fn config_get_seek_accurate -->
## `config`
a `Player` configuration

# Returns

`true` if accurate seeking is enabled

Since 1.12
<!-- impl Player::fn config_get_user_agent -->
Return the user agent which has been configured using
`Player::config_set_user_agent` if any.
## `config`
a `Player` configuration

# Returns

the configured agent, or `None`
Since 1.10
<!-- impl Player::fn config_set_position_update_interval -->
set interval in milliseconds between two position-updated signals.
pass 0 to stop updating the position.
Since 1.10
## `config`
a `Player` configuration
## `interval`
interval in ms
<!-- impl Player::fn config_set_user_agent -->
Set the user agent to pass to the server if `player` needs to connect
to a server during playback. This is typically used when playing HTTP
or RTSP streams.

Since 1.10
## `config`
a `Player` configuration
## `agent`
the string to use as user agent
<!-- impl Player::fn get_audio_streams -->
## `info`
a `PlayerMediaInfo`

# Returns

A `glib::List` of
matching `PlayerAudioInfo`.
<!-- impl Player::fn get_subtitle_streams -->
## `info`
a `PlayerMediaInfo`

# Returns

A `glib::List` of
matching `PlayerSubtitleInfo`.
<!-- impl Player::fn get_video_streams -->
## `info`
a `PlayerMediaInfo`

# Returns

A `glib::List` of
matching `PlayerVideoInfo`.
<!-- impl Player::fn visualizations_free -->
Frees a `None` terminated array of `PlayerVisualization`.
## `viss`
a `None` terminated array of `PlayerVisualization` to free
<!-- impl Player::fn visualizations_get -->

# Returns


 a `None` terminated array containing all available
 visualizations. Use `Player::visualizations_free` after
 usage.
<!-- impl Player::fn config_set_seek_accurate -->
Enable or disable accurate seeking. When enabled, elements will try harder
to seek as accurately as possible to the requested seek position. Generally
it will be slower especially for formats that don't have any indexes or
timestamp markers in the stream.

If accurate seeking is disabled, elements will seek as close as the request
position without slowing down seeking too much.

Accurate seeking is disabled by default.
## `accurate`
accurate seek or not
<!-- impl Player::fn get_audio_video_offset -->
Retrieve the current value of audio-video-offset property

# Returns

The current value of audio-video-offset in nanoseconds

Since 1.10
<!-- impl Player::fn get_color_balance -->
Retrieve the current value of the indicated `type_`.
## `type_`
`PlayerColorBalanceType`

# Returns

The current value of `type_`, between [0,1]. In case of
 error -1 is returned.
<!-- impl Player::fn get_config -->
Get a copy of the current configuration of the player. This configuration
can either be modified and used for the `Player::set_config` call
or it must be freed after usage.

# Returns

a copy of the current configuration of `self`. Use
`gst::Structure::free` after usage or `Player::set_config`.

Since 1.10
<!-- impl Player::fn get_current_audio_track -->
A Function to get current audio `PlayerAudioInfo` instance.

# Returns

current audio track.

The caller should free it with `gobject::ObjectExt::unref`
<!-- impl Player::fn get_current_subtitle_track -->
A Function to get current subtitle `PlayerSubtitleInfo` instance.

# Returns

current subtitle track.

The caller should free it with `gobject::ObjectExt::unref`
<!-- impl Player::fn get_current_video_track -->
A Function to get current video `PlayerVideoInfo` instance.

# Returns

current video track.

The caller should free it with `gobject::ObjectExt::unref`
<!-- impl Player::fn get_current_visualization -->

# Returns

Name of the currently enabled visualization.
 `g_free` after usage.
<!-- impl Player::fn get_duration -->
Retrieves the duration of the media stream that self represents.

# Returns

the duration of the currently-playing media stream, in
nanoseconds.
<!-- impl Player::fn get_media_info -->
A Function to get the current media info `PlayerMediaInfo` instance.

# Returns

media info instance.

The caller should free it with `gobject::ObjectExt::unref`
<!-- impl Player::fn get_multiview_flags -->
Retrieve the current value of the indicated `type_`.

# Returns

The current value of `type_`, Default: 0x00000000 "none
<!-- impl Player::fn get_multiview_mode -->
Retrieve the current value of the indicated `type_`.

# Returns

The current value of `type_`, Default: -1 "none"
<!-- impl Player::fn get_mute -->

# Returns

`true` if the currently-playing stream is muted.
<!-- impl Player::fn get_pipeline -->

# Returns

The internal playbin instance
<!-- impl Player::fn get_position -->

# Returns

the absolute position time, in nanoseconds, of the
currently-playing stream.
<!-- impl Player::fn get_rate -->

# Returns

current playback rate
<!-- impl Player::fn get_subtitle_uri -->
current subtitle URI

# Returns

URI of the current external subtitle.
 `g_free` after usage.
<!-- impl Player::fn get_uri -->
Gets the URI of the currently-playing stream.

# Returns

a string containing the URI of the
currently-playing stream. `g_free` after usage.
<!-- impl Player::fn get_video_snapshot -->
Get a snapshot of the currently selected video stream, if any. The format can be
selected with `format` and optional configuration is possible with `config`
Currently supported settings are:
- width, height of type G_TYPE_INT
- pixel-aspect-ratio of type GST_TYPE_FRACTION
 Except for GST_PLAYER_THUMBNAIL_RAW_NATIVE format, if no config is set, pixel-aspect-ratio would be 1/1
## `format`
output format of the video snapshot
## `config`
Additional configuration

# Returns

Current video snapshot sample or `None` on failure

Since 1.12
<!-- impl Player::fn get_volume -->
Returns the current volume level, as a percentage between 0 and 1.

# Returns

the volume as percentage between 0 and 1.
<!-- impl Player::fn has_color_balance -->
Checks whether the `self` has color balance support available.

# Returns

`true` if `self` has color balance support. Otherwise,
 `false`.
<!-- impl Player::fn pause -->
Pauses the current stream.
<!-- impl Player::fn play -->
Request to play the loaded stream.
<!-- impl Player::fn seek -->
Seeks the currently-playing stream to the absolute `position` time
in nanoseconds.
## `position`
position to seek in nanoseconds
<!-- impl Player::fn set_audio_track -->
## `stream_index`
stream index

# Returns

`true` or `false`

Sets the audio track `stream_idex`.
<!-- impl Player::fn set_audio_track_enabled -->
Enable or disable the current audio track.
## `enabled`
TRUE or FALSE
<!-- impl Player::fn set_audio_video_offset -->
Sets audio-video-offset property by value of `offset`

Since 1.10
## `offset`
`gint64` in nanoseconds
<!-- impl Player::fn set_color_balance -->
Sets the current value of the indicated channel `type_` to the passed
value.
## `type_`
`PlayerColorBalanceType`
## `value`
The new value for the `type_`, ranged [0,1]
<!-- impl Player::fn set_config -->
Set the configuration of the player. If the player is already configured, and
the configuration haven't change, this function will return `true`. If the
player is not in the GST_PLAYER_STATE_STOPPED, this method will return `false`
and active configuration will remain.

`config` is a `gst::Structure` that contains the configuration parameters for
the player.

This function takes ownership of `config`.
## `config`
a `gst::Structure`

# Returns

`true` when the configuration could be set.
Since 1.10
<!-- impl Player::fn set_multiview_flags -->
Sets the current value of the indicated mode `type_` to the passed
value.
## `flags`
The new value for the `type_`
<!-- impl Player::fn set_multiview_mode -->
Sets the current value of the indicated mode `type_` to the passed
value.
## `mode`
The new value for the `type_`
<!-- impl Player::fn set_mute -->
`true` if the currently-playing stream should be muted.
## `val`
Mute state the should be set
<!-- impl Player::fn set_rate -->
Playback at specified rate
## `rate`
playback rate
<!-- impl Player::fn set_subtitle_track -->
## `stream_index`
stream index

# Returns

`true` or `false`

Sets the subtitle strack `stream_index`.
<!-- impl Player::fn set_subtitle_track_enabled -->
Enable or disable the current subtitle track.
## `enabled`
TRUE or FALSE
<!-- impl Player::fn set_subtitle_uri -->
Sets the external subtitle URI.
## `uri`
subtitle URI
<!-- impl Player::fn set_uri -->
Sets the next URI to play.
## `uri`
next URI to play.
<!-- impl Player::fn set_video_track -->
## `stream_index`
stream index

# Returns

`true` or `false`

Sets the video track `stream_index`.
<!-- impl Player::fn set_video_track_enabled -->
Enable or disable the current video track.
## `enabled`
TRUE or FALSE
<!-- impl Player::fn set_visualization -->
## `name`
visualization element obtained from
`Player::visualizations_get`()

# Returns

`true` if the visualizations was set correctly. Otherwise,
`false`.
<!-- impl Player::fn set_visualization_enabled -->
Enable or disable the visualization.
## `enabled`
TRUE or FALSE
<!-- impl Player::fn set_volume -->
Sets the volume level of the stream as a percentage between 0 and 1.
## `val`
the new volume level, as a percentage between 0 and 1
<!-- impl Player::fn stop -->
Stops playing the current stream and resets to the first position
in the stream.
<!-- struct PlayerAudioInfo -->
`PlayerStreamInfo` specific to audio streams.

# Implements

[`PlayerAudioInfoExt`](trait.PlayerAudioInfoExt.html), [`PlayerStreamInfoExt`](trait.PlayerStreamInfoExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait PlayerAudioInfoExt -->
Trait containing all `PlayerAudioInfo` methods.

# Implementors

[`PlayerAudioInfo`](struct.PlayerAudioInfo.html)
<!-- trait PlayerAudioInfoExt::fn get_bitrate -->

# Returns

the audio bitrate in `PlayerAudioInfo`.
<!-- trait PlayerAudioInfoExt::fn get_channels -->

# Returns

the number of audio channels in `PlayerAudioInfo`.
<!-- trait PlayerAudioInfoExt::fn get_language -->

# Returns

the language of the stream, or NULL if unknown.
<!-- trait PlayerAudioInfoExt::fn get_max_bitrate -->

# Returns

the audio maximum bitrate in `PlayerAudioInfo`.
<!-- trait PlayerAudioInfoExt::fn get_sample_rate -->

# Returns

the audio sample rate in `PlayerAudioInfo`.
<!-- enum PlayerColorBalanceType -->
<!-- enum PlayerColorBalanceType::variant Hue -->
hue or color balance.
<!-- enum PlayerColorBalanceType::variant Brightness -->
brightness or black level.
<!-- enum PlayerColorBalanceType::variant Saturation -->
color saturation or chroma
gain.
<!-- enum PlayerColorBalanceType::variant Contrast -->
contrast or luma gain.
<!-- enum PlayerError -->
<!-- enum PlayerError::variant Failed -->
generic error.
<!-- struct PlayerGMainContextSignalDispatcher -->


# Implements

[`PlayerGMainContextSignalDispatcherExt`](trait.PlayerGMainContextSignalDispatcherExt.html), [`ObjectExt`](trait.ObjectExt.html), [`PlayerSignalDispatcherExt`](trait.PlayerSignalDispatcherExt.html)
<!-- trait PlayerGMainContextSignalDispatcherExt -->
Trait containing all `PlayerGMainContextSignalDispatcher` methods.

# Implementors

[`PlayerGMainContextSignalDispatcher`](struct.PlayerGMainContextSignalDispatcher.html)
<!-- impl PlayerGMainContextSignalDispatcher::fn new -->
Creates a new GstPlayerSignalDispatcher that uses `application_context`,
or the thread default one if `None` is used. See `gst_player_new_full`.
## `application_context`
GMainContext to use or `None`

# Returns

the new GstPlayerSignalDispatcher
<!-- struct PlayerMediaInfo -->
Structure containing the media information of a URI.

# Implements

[`PlayerMediaInfoExt`](trait.PlayerMediaInfoExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait PlayerMediaInfoExt -->
Trait containing all `PlayerMediaInfo` methods.

# Implementors

[`PlayerMediaInfo`](struct.PlayerMediaInfo.html)
<!-- trait PlayerMediaInfoExt::fn get_audio_streams -->

# Returns

A `glib::List` of
matching `PlayerAudioInfo`.
<!-- trait PlayerMediaInfoExt::fn get_container_format -->

# Returns

the container format.
<!-- trait PlayerMediaInfoExt::fn get_duration -->

# Returns

duration of the media.
<!-- trait PlayerMediaInfoExt::fn get_image_sample -->
Function to get the image (or preview-image) stored in taglist.
Application can use gst_sample_*`_` API's to get caps, buffer etc.

# Returns

GstSample or NULL.
<!-- trait PlayerMediaInfoExt::fn get_number_of_audio_streams -->

# Returns

number of audio streams.
<!-- trait PlayerMediaInfoExt::fn get_number_of_streams -->

# Returns

number of total streams.
<!-- trait PlayerMediaInfoExt::fn get_number_of_subtitle_streams -->

# Returns

number of subtitle streams.
<!-- trait PlayerMediaInfoExt::fn get_number_of_video_streams -->

# Returns

number of video streams.
<!-- trait PlayerMediaInfoExt::fn get_stream_list -->

# Returns

A `glib::List` of
matching `PlayerStreamInfo`.
<!-- trait PlayerMediaInfoExt::fn get_subtitle_streams -->

# Returns

A `glib::List` of
matching `PlayerSubtitleInfo`.
<!-- trait PlayerMediaInfoExt::fn get_tags -->

# Returns

the tags contained in media info.
<!-- trait PlayerMediaInfoExt::fn get_title -->

# Returns

the media title.
<!-- trait PlayerMediaInfoExt::fn get_uri -->

# Returns

the URI associated with `PlayerMediaInfo`.
<!-- trait PlayerMediaInfoExt::fn get_video_streams -->

# Returns

A `glib::List` of
matching `PlayerVideoInfo`.
<!-- trait PlayerMediaInfoExt::fn is_live -->

# Returns

`true` if the media is live.
<!-- trait PlayerMediaInfoExt::fn is_seekable -->

# Returns

`true` if the media is seekable.
<!-- struct PlayerSignalDispatcher -->


# Implements

[`PlayerSignalDispatcherExt`](trait.PlayerSignalDispatcherExt.html)
<!-- trait PlayerSignalDispatcherExt -->
Trait containing all `PlayerSignalDispatcher` methods.

# Implementors

[`PlayerGMainContextSignalDispatcher`](struct.PlayerGMainContextSignalDispatcher.html), [`PlayerSignalDispatcher`](struct.PlayerSignalDispatcher.html)
<!-- enum PlayerSnapshotFormat -->
<!-- enum PlayerState -->
<!-- enum PlayerState::variant Stopped -->
the player is stopped.
<!-- enum PlayerState::variant Buffering -->
the player is buffering.
<!-- enum PlayerState::variant Paused -->
the player is paused.
<!-- enum PlayerState::variant Playing -->
the player is currently playing a
stream.
<!-- struct PlayerStreamInfo -->
Base structure for information concering a media stream. Depending on
the stream type, one can find more media-specific information in
`PlayerVideoInfo`, `PlayerAudioInfo`, `PlayerSubtitleInfo`.

# Implements

[`PlayerStreamInfoExt`](trait.PlayerStreamInfoExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait PlayerStreamInfoExt -->
Trait containing all `PlayerStreamInfo` methods.

# Implementors

[`PlayerAudioInfo`](struct.PlayerAudioInfo.html), [`PlayerStreamInfo`](struct.PlayerStreamInfo.html), [`PlayerSubtitleInfo`](struct.PlayerSubtitleInfo.html), [`PlayerVideoInfo`](struct.PlayerVideoInfo.html)
<!-- trait PlayerStreamInfoExt::fn get_caps -->

# Returns

the `gst::Caps` of the stream.
<!-- trait PlayerStreamInfoExt::fn get_codec -->
A string describing codec used in `PlayerStreamInfo`.

# Returns

codec string or NULL on unknown.
<!-- trait PlayerStreamInfoExt::fn get_index -->
Function to get stream index from `PlayerStreamInfo` instance.

# Returns

the stream index of this stream.
<!-- trait PlayerStreamInfoExt::fn get_stream_type -->
Function to return human readable name for the stream type
of the given `self` (ex: "audio", "video", "subtitle")

# Returns

a human readable name
<!-- trait PlayerStreamInfoExt::fn get_tags -->

# Returns

the tags contained in this stream.
<!-- struct PlayerSubtitleInfo -->
`PlayerStreamInfo` specific to subtitle streams.

# Implements

[`PlayerSubtitleInfoExt`](trait.PlayerSubtitleInfoExt.html), [`PlayerStreamInfoExt`](trait.PlayerStreamInfoExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait PlayerSubtitleInfoExt -->
Trait containing all `PlayerSubtitleInfo` methods.

# Implementors

[`PlayerSubtitleInfo`](struct.PlayerSubtitleInfo.html)
<!-- trait PlayerSubtitleInfoExt::fn get_language -->

# Returns

the language of the stream, or NULL if unknown.
<!-- struct PlayerVideoInfo -->
`PlayerStreamInfo` specific to video streams.

# Implements

[`PlayerVideoInfoExt`](trait.PlayerVideoInfoExt.html), [`PlayerStreamInfoExt`](trait.PlayerStreamInfoExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait PlayerVideoInfoExt -->
Trait containing all `PlayerVideoInfo` methods.

# Implementors

[`PlayerVideoInfo`](struct.PlayerVideoInfo.html)
<!-- trait PlayerVideoInfoExt::fn get_bitrate -->

# Returns

the current bitrate of video in `PlayerVideoInfo`.
<!-- trait PlayerVideoInfoExt::fn get_framerate -->
## `fps_n`
Numerator of frame rate
## `fps_d`
Denominator of frame rate
<!-- trait PlayerVideoInfoExt::fn get_height -->

# Returns

the height of video in `PlayerVideoInfo`.
<!-- trait PlayerVideoInfoExt::fn get_max_bitrate -->

# Returns

the maximum bitrate of video in `PlayerVideoInfo`.
<!-- trait PlayerVideoInfoExt::fn get_pixel_aspect_ratio -->
Returns the pixel aspect ratio in `par_n` and `par_d`
## `par_n`
numerator
## `par_d`
denominator
<!-- trait PlayerVideoInfoExt::fn get_width -->

# Returns

the width of video in `PlayerVideoInfo`.
<!-- struct PlayerVideoOverlayVideoRenderer -->


# Implements

[`PlayerVideoOverlayVideoRendererExt`](trait.PlayerVideoOverlayVideoRendererExt.html), [`ObjectExt`](trait.ObjectExt.html), [`PlayerVideoRendererExt`](trait.PlayerVideoRendererExt.html)
<!-- trait PlayerVideoOverlayVideoRendererExt -->
Trait containing all `PlayerVideoOverlayVideoRenderer` methods.

# Implementors

[`PlayerVideoOverlayVideoRenderer`](struct.PlayerVideoOverlayVideoRenderer.html)
<!-- impl PlayerVideoOverlayVideoRenderer::fn new -->
## `window_handle`
Window handle to use or `None`
<!-- impl PlayerVideoOverlayVideoRenderer::fn new_with_sink -->
## `window_handle`
Window handle to use or `None`
## `video_sink`
the custom video_sink element to be set for the video renderer

# Returns



Since 1.12
<!-- trait PlayerVideoOverlayVideoRendererExt::fn expose -->
Tell an overlay that it has been exposed. This will redraw the current frame
in the drawable even if the pipeline is PAUSED.
<!-- trait PlayerVideoOverlayVideoRendererExt::fn get_render_rectangle -->
Return the currently configured render rectangle. See `PlayerVideoOverlayVideoRendererExt::set_render_rectangle`
for details.
## `x`
the horizontal offset of the render area inside the window
## `y`
the vertical offset of the render area inside the window
## `width`
the width of the render area inside the window
## `height`
the height of the render area inside the window
<!-- trait PlayerVideoOverlayVideoRendererExt::fn get_window_handle -->

# Returns

The currently set, platform specific window
handle
<!-- trait PlayerVideoOverlayVideoRendererExt::fn set_render_rectangle -->
Configure a subregion as a video target within the window set by
`PlayerVideoOverlayVideoRendererExt::set_window_handle`. If this is not
used or not supported the video will fill the area of the window set as the
overlay to 100%. By specifying the rectangle, the video can be overlaid to
a specific region of that window only. After setting the new rectangle one
should call `PlayerVideoOverlayVideoRendererExt::expose` to force a
redraw. To unset the region pass -1 for the `width` and `height` parameters.

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
<!-- trait PlayerVideoOverlayVideoRendererExt::fn set_window_handle -->
Sets the platform specific window handle into which the video
should be rendered
## `window_handle`
handle referencing to the platform specific window
<!-- struct PlayerVideoRenderer -->


# Implements

[`PlayerVideoRendererExt`](trait.PlayerVideoRendererExt.html)
<!-- trait PlayerVideoRendererExt -->
Trait containing all `PlayerVideoRenderer` methods.

# Implementors

[`PlayerVideoOverlayVideoRenderer`](struct.PlayerVideoOverlayVideoRenderer.html), [`PlayerVideoRenderer`](struct.PlayerVideoRenderer.html)
<!-- struct PlayerVisualization -->
A `PlayerVisualization` descriptor.
<!-- impl PlayerVisualization::fn copy -->
Makes a copy of the `PlayerVisualization`. The result must be
freed using `PlayerVisualization::free`.

# Returns

an allocated copy of `self`.
<!-- impl PlayerVisualization::fn free -->
Frees a `PlayerVisualization`.
