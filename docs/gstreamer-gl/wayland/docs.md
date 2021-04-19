<!-- file * -->
<!-- struct GLDisplayWayland -->
the contents of a `GLDisplayWayland` are private and should only be accessed
through the provided API

# Implements

[`trait@gst_gl::GLDisplayExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- impl GLDisplayWayland::fn new -->
Create a new `GLDisplayWayland` from the wayland display name. See `wl_display_connect`()
for details on what is a valid name.
## `name`
a display name

# Returns

a new `GLDisplayWayland` or `None`
<!-- impl GLDisplayWayland::fn with_display -->
Creates a new display connection from a wl_display Display.
## `display`
an existing, wayland display

# Returns

a new `GLDisplayWayland`
