use libc::c_char;

// See https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/issues/497
#[cfg(any(feature = "egl", feature = "dox"))]
pub const GST_GL_DISPLAY_EGL_NAME: *const c_char =
    b"gst.gl.display.egl\0" as *const u8 as *const c_char;

// See https://gitlab.gnome.org/GNOME/gobject-introspection/issues/238
pub const GST_GL_COLOR_CONVERT_VIDEO_CAPS: *const c_char = b"video/x-raw(memory:GLMemory), format = (string) { RGBA, RGB, RGBx, BGR, BGRx, BGRA, xRGB, xBGR, ARGB, ABGR, Y444, I420, YV12, Y42B, Y41B, NV12, NV21, YUY2, UYVY, AYUV, GRAY8, GRAY16_LE, GRAY16_BE, RGB16, BGR16 }, width = (int) [ 1, max ], height = (int) [ 1, max ], framerate = (fraction) [ 0, max ], texture-target = (string) { 2D, rectangle, external-oes }  ; video/x-raw(memory:GLMemory,meta:GstVideoOverlayComposition), format = (string) { RGBA, RGB, RGBx, BGR, BGRx, BGRA, xRGB, xBGR, ARGB, ABGR, Y444, I420, YV12, Y42B, Y41B, NV12, NV21, YUY2, UYVY, AYUV, GRAY8, GRAY16_LE, GRAY16_BE, RGB16, BGR16 }, width = (int) [ 1, max ], height = (int) [ 1, max ], framerate = (fraction) [ 0, max ], texture-target = (string) { 2D, rectangle, external-oes }\0" as *const u8 as *const c_char;
