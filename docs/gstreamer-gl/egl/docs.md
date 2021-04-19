<!-- file * -->
<!-- struct GLDisplayEGL -->
the contents of a `GLDisplayEGL` are private and should only be accessed
through the provided API

# Implements

[`trait@gst_gl::GLDisplayExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- impl GLDisplayEGL::fn new -->
Create a new `GLDisplayEGL` using the default EGL_DEFAULT_DISPLAY.

# Returns

a new `GLDisplayEGL` or `None`
<!-- impl GLDisplayEGL::fn from_gl_display -->
Creates a EGL display connection from a native Display.

This function will return the same value for multiple calls with the same
`display`.
## `display`
an existing `gst_gl::GLDisplay`

# Returns

a new `GLDisplayEGL`
<!-- impl GLDisplayEGL::fn get_from_native -->
Attempts to create a new `EGLDisplay` from `display`. If `type_` is
`gst_gl::GLDisplayType::Any`, then `display` must be 0. `type_` must not be
`gst_gl::GLDisplayType::None`.
## `type_`
a `gst_gl::GLDisplayType`
## `display`
pointer to a display (or 0)

# Returns

A `EGLDisplay` or `EGL_NO_DISPLAY`
