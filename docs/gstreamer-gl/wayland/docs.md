<!-- file * -->
<!-- struct GLDisplayWayland -->
the contents of a `GLDisplayWayland` are private and should only be accessed
through the provided API

# Implements

[`gst_gl::GLDisplayExt`](../gst_gl/trait.GLDisplayExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl GLDisplayWayland::fn new -->
Create a new `GLDisplayWayland` from the wayland display name. See `wl_display_connect`()
for details on what is a valid name.
## `name`
a display name

# Returns

a new `GLDisplayWayland` or `None`
<!-- impl GLDisplayWayland::fn new_with_display -->
Creates a new display connection from a wl_display Display.
## `display`
an existing, wayland display

# Returns

a new `GLDisplayWayland`
