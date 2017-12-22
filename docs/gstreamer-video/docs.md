<!-- file * -->
<!-- enum VideoColorMatrix -->
The color matrix is used to convert between Y'PbPr and
non-linear RGB (R'G'B')
<!-- enum VideoColorMatrix::variant Unknown -->
unknown matrix
<!-- enum VideoColorMatrix::variant Rgb -->
identity matrix
<!-- enum VideoColorMatrix::variant Fcc -->
FCC color matrix
<!-- enum VideoColorMatrix::variant Bt709 -->
ITU-R BT.709 color matrix
<!-- enum VideoColorMatrix::variant Bt601 -->
ITU-R BT.601 color matrix
<!-- enum VideoColorMatrix::variant Smpte240m -->
SMPTE 240M color matrix
<!-- enum VideoColorMatrix::variant Bt2020 -->
ITU-R BT.2020 color matrix. Since: 1.6.
<!-- enum VideoColorPrimaries -->
The color primaries define the how to transform linear RGB values to and from
the CIE XYZ colorspace.
<!-- enum VideoColorPrimaries::variant Unknown -->
unknown color primaries
<!-- enum VideoColorPrimaries::variant Bt709 -->
BT709 primaries
<!-- enum VideoColorPrimaries::variant Bt470m -->
BT470M primaries
<!-- enum VideoColorPrimaries::variant Bt470bg -->
BT470BG primaries
<!-- enum VideoColorPrimaries::variant Smpte170m -->
SMPTE170M primaries
<!-- enum VideoColorPrimaries::variant Smpte240m -->
SMPTE240M primaries
<!-- enum VideoColorPrimaries::variant Film -->
Generic film
<!-- enum VideoColorPrimaries::variant Bt2020 -->
BT2020 primaries. Since: 1.6.
<!-- enum VideoColorPrimaries::variant Adobergb -->
Adobe RGB primaries. Since: 1.8
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

a string representation of `self`.
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

<!-- enum VideoFormat -->
Enum value describing the most common video formats.
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
rgb
<!-- enum VideoFormat::variant Bgr -->
reverse rgb
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
packed 4:4:4 YUV (U-Y-V ...) (Since 1.10)
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
<!-- struct VideoFormatInfo -->
Information for a video format.
<!-- struct VideoFrame -->
A video frame obtained from `VideoFrame::map`
<!-- impl VideoFrame::fn copy -->
Copy the contents from `src` to `self`.
## `src`
a `VideoFrame`

# Returns

TRUE if the contents could be copied.
<!-- impl VideoFrame::fn copy_plane -->
Copy the plane with index `plane` from `src` to `self`.
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
uses `playbin` or `plasink` or knows exactly what particular element is used
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
GDK_WINDOW_XID() was just a simple structure access, so generally fine to do
within the bus sync handler; this macro was changed to a function call in
Gtk+ 2.18 and later, which is likely to cause problems when called from a
sync handler; see below for a better approach without GDK_WINDOW_XID()
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
#include &lt;glib.h&gt;
#include &lt;gst/gst.h&gt;
#include &lt;gst/video/videooverlay.h&gt;

#include &lt;QApplication&gt;
#include &lt;QTimer&gt;
#include &lt;QWidget&gt;

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

[`VideoOverlayExt`](trait.VideoOverlayExt.html)
<!-- trait VideoOverlayExt -->
Trait containing all `VideoOverlay` methods.

# Implementors

[`VideoOverlay`](struct.VideoOverlay.html)
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
<!-- enum VideoTileMode -->
Enum value describing the available tiling modes.
<!-- enum VideoTileMode::variant Unknown -->
Unknown or unset tile mode
<!-- enum VideoTileMode::variant Zflipz2x2 -->
Every four adjacent blocks - two
 horizontally and two vertically are grouped together and are located
 in memory in Z or flipped Z order. In case of odd rows, the last row
 of blocks is arranged in linear order.
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
 range
<!-- enum VideoTransferFunction::variant Smpte240m -->
Gamma 2.2 curve with a linear segment in the
 lower range
<!-- enum VideoTransferFunction::variant Srgb -->
Gamma 2.4 curve with a linear segment in the lower
 range
<!-- enum VideoTransferFunction::variant Gamma28 -->
Gamma 2.8 curve
<!-- enum VideoTransferFunction::variant Log100 -->
Logarithmic transfer characteristic
 100:1 range
<!-- enum VideoTransferFunction::variant Log316 -->
Logarithmic transfer characteristic
 316.22777:1 range
<!-- enum VideoTransferFunction::variant Bt202012 -->
Gamma 2.2 curve with a linear segment in the lower
 range. Used for BT.2020 with 12 bits per
 component. Since: 1.6.
<!-- enum VideoTransferFunction::variant Adobergb -->
Gamma 2.19921875. Since: 1.8
