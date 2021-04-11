<!-- file * -->
<!-- struct GLDisplayX11 -->
the contents of a `GLDisplayX11` are private and should only be accessed
through the provided API

# Implements

[`gst_gl::GLDisplayExt`](../gst_gl/trait.GLDisplayExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl GLDisplayX11::fn new -->
Create a new `GLDisplayX11` from the x11 display name. See `XOpenDisplay`()
for details on what is a valid name.
## `name`
a display name

# Returns

a new `GLDisplayX11` or `None`
<!-- impl GLDisplayX11::fn new_with_display -->
Creates a new display connection from a X11 Display.
## `display`
an existing, x11 display

# Returns

a new `GLDisplayX11`
