<!-- file * -->
<!-- struct GLBaseFilter -->
`GLBaseFilter` handles the nitty gritty details of retrieving an OpenGL
context. It also provided some wrappers around `gst_base::BaseTransform`'s
`start`, `stop` and `set_caps` virtual methods that ensure an OpenGL context
is available and current in the calling thread.

# Implements

[`GLBaseFilterExt`](trait.GLBaseFilterExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait GLBaseFilterExt -->
Trait containing all `GLBaseFilter` methods.

# Implementors

[`GLBaseFilter`](struct.GLBaseFilter.html)
<!-- struct GLBuffer -->
GstGLBuffer is a `gst::Memory` subclass providing support for the mapping of
GL buffers.

Data is uploaded or downloaded from the GPU as is necessary.
<!-- impl GLBuffer::fn init_once -->
Initializes the GL Buffer allocator. It is safe to call this function
multiple times. This must be called before any other `GLBuffer` operation.
<!-- struct GLColorConvert -->
`GLColorConvert` is an object that converts between color spaces and/or
formats using OpenGL Shaders.

A `GLColorConvert` can be created with `GLColorConvert::new`, the
configuration negotiated with `GLColorConvert::transform_caps` and the
conversion performed with `GLColorConvert::perform`.

The glcolorconvertelement provides a GStreamer element that uses
`GLColorConvert` to convert between video formats and color spaces.

# Implements

[`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl GLColorConvert::fn new -->
## `context`
a `GLContext`

# Returns

a new `GLColorConvert` object
<!-- impl GLColorConvert::fn fixate_caps -->
Provides an implementation of `gst_base::BaseTransformClass::fixate_caps`()
## `context`
a `GLContext` to use for transforming `caps`
## `direction`
a `gst::PadDirection`
## `caps`
the `gst::Caps` of `direction`
## `other`
the `gst::Caps` to fixate

# Returns

the fixated `gst::Caps`
<!-- impl GLColorConvert::fn transform_caps -->
Provides an implementation of `gst_base::BaseTransformClass::transform_caps`()
## `context`
a `GLContext` to use for transforming `caps`
## `direction`
a `gst::PadDirection`
## `caps`
the `gst::Caps` to transform
## `filter`
a set of filter `gst::Caps`

# Returns

the converted `gst::Caps`
<!-- impl GLColorConvert::fn decide_allocation -->
Provides an implementation of `GstBaseTransfromClass::decide_allocation`()
## `query`
a completed ALLOCATION `gst::Query`

# Returns

whether the allocation parameters were successfully chosen
<!-- impl GLColorConvert::fn perform -->
Converts the data contained by `inbuf` using the formats specified by the
`gst::Caps` passed to `GLColorConvert::set_caps`
## `inbuf`
the `GLMemory` filled `gst::Buffer` to convert

# Returns

a converted `gst::Buffer` or `None`
<!-- impl GLColorConvert::fn set_caps -->
Initializes `self` with the information required for conversion.
## `in_caps`
input `gst::Caps`
## `out_caps`
output `gst::Caps`
<!-- struct GLContext -->
`GLContext` wraps an OpenGL context object in a uniform API. As a result
of the limitation on OpenGL context, this object is not thread safe unless
specified and must only be activated in a single thread.

# Implements

[`GLContextExt`](trait.GLContextExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait GLContextExt -->
Trait containing all `GLContext` methods.

# Implementors

[`GLContext`](struct.GLContext.html)
<!-- impl GLContext::fn new -->
Create a new `GLContext` with the specified `display`
## `display`
a `GLDisplay`

# Returns

a new `GLContext`
<!-- impl GLContext::fn new_wrapped -->
Wraps an existing OpenGL context into a `GLContext`.

Note: The caller is responsible for ensuring that the OpenGL context
represented by `handle` stays alive while the returned `GLContext` is
active.
## `display`
a `GLDisplay`
## `handle`
the OpenGL context to wrap
## `context_type`
a `GLPlatform` specifying the type of context in `handle`
## `available_apis`
a `GLAPI` containing the available OpenGL apis in `handle`

# Returns

a `GLContext` wrapping `handle`
<!-- impl GLContext::fn default_get_proc_address -->
A default implementation of the various GetProcAddress functions that looks
for `name` in the OpenGL shared libraries or in the current process.

See also: `GLContext::get_proc_address`
## `gl_api`
a `GLAPI`
## `name`
then function to get the address of

# Returns

an address pointing to `name` or `None`
<!-- impl GLContext::fn get_current -->
See also `GLContextExt::activate`.

# Returns

the `GLContext` active in the current thread or `None`
<!-- impl GLContext::fn get_current_gl_api -->
If an error occurs, `major` and `minor` are not modified and `GLAPI::None` is
returned.
## `platform`
the `GLPlatform` to retrieve the API for
## `major`
the major version
## `minor`
the minor version

# Returns

The version supported by the OpenGL context current in the calling
 thread or `GLAPI::None`
<!-- impl GLContext::fn get_current_gl_context -->
## `context_type`
a `GLPlatform` specifying the type of context to retrieve

# Returns

The OpenGL context handle current in the calling thread or `None`
<!-- impl GLContext::fn get_proc_address_with_platform -->
Attempts to use the `context_type` specific GetProcAddress implementations
to retreive `name`.

See also `GLContext::get_proc_address`.
## `context_type`
a `GLPlatform`
## `gl_api`
a `GLAPI`
## `name`
the name of the function to retrieve

# Returns

a function pointer for `name`, or `None`
<!-- trait GLContextExt::fn activate -->
(De)activate the OpenGL context represented by this `self`.

In OpenGL terms, calls eglMakeCurrent or similar with this context and the
currently set window. See `GLContextExt::set_window` for details.
## `activate`
`true` to activate, `false` to deactivate

# Returns

Whether the activation succeeded
<!-- trait GLContextExt::fn can_share -->
Note: This will always fail for two wrapped `GLContext`'s
## `other_context`
another `GLContext`

# Returns

whether `self` and `other_context` are able to share OpenGL
 resources.
<!-- trait GLContextExt::fn check_feature -->
Check for an OpenGL `feature` being supported.

Note: Most features require that the context be created before it is
possible to determine their existence and so will fail if that is not the
case.
## `feature`
a platform specific feature

# Returns

Whether `feature` is supported by `self`
<!-- trait GLContextExt::fn check_framebuffer_status -->
## `fbo_target`
the GL value of the framebuffer target, GL_FRAMEBUFFER,
 GL_READ_FRAMEBUFFER, GL_DRAW_FRAMEBUFFER

# Returns

whether whether the current framebuffer is complete
<!-- trait GLContextExt::fn check_gl_version -->
## `api`
api type required
## `maj`
major version required
## `min`
minor version required

# Returns

whether OpenGL context implements the required api and specified
version.
<!-- trait GLContextExt::fn clear_framebuffer -->
Unbind the current framebuffer
<!-- trait GLContextExt::fn clear_shader -->
Clear's the currently set shader from the GL state machine.

Note: must be called in the GL thread.
<!-- trait GLContextExt::fn create -->
Creates an OpenGL context with the specified `other_context` as a context
to share shareable OpenGL objects with. See the OpenGL specification for
what is shared between OpenGL contexts.

If an error occurs, and `error` is not `None`, then error will contain details
of the error and `false` will be returned.

Should only be called once.
## `other_context`
a `GLContext` to share OpenGL objects with

# Returns

whether the context could successfully be created
<!-- trait GLContextExt::fn destroy -->
Destroys an OpenGL context.

Should only be called after `GLContextExt::create` has been successfully
called for this context.
<!-- trait GLContextExt::fn fill_info -->
Fills `self`'s info (version, extensions, vtable, etc) from the GL
context in the current thread. Typically used with wrapped contexts to
allow wrapped contexts to be used as regular `GLContext`'s.
<!-- trait GLContextExt::fn get_display -->

# Returns

the `GLDisplay` associated with this `self`
<!-- trait GLContextExt::fn get_gl_api -->
Get the currently enabled OpenGL api.

The currently available API may be limited by the `GLDisplay` in use and/or
the `GLWindow` chosen.

# Returns

the available OpenGL api
<!-- trait GLContextExt::fn get_gl_context -->
Gets the backing OpenGL context used by `self`.

# Returns

The platform specific backing OpenGL context
<!-- trait GLContextExt::fn get_gl_platform -->
Gets the OpenGL platform that used by `self`.

# Returns

The platform specific backing OpenGL context
<!-- trait GLContextExt::fn get_gl_platform_version -->
Get the version of the OpenGL platform (GLX, EGL, etc) used. Only valid
after a call to `gst_gl_context_create_context`.
## `major`
return for the major version
## `minor`
return for the minor version
<!-- trait GLContextExt::fn get_gl_version -->
Returns the OpenGL version implemented by `self`. See
`GLContextExt::get_gl_api` for retreiving the OpenGL api implemented by
`self`.
## `maj`
resulting major version
## `min`
resulting minor version
<!-- trait GLContextExt::fn get_proc_address -->
Get a function pointer to a specified opengl function, `name`. If the the
specific function does not exist, NULL is returned instead.

Platform specfic functions (names starting 'egl', 'glX', 'wgl', etc) can also
be retrieved using this method.

Note: This function may return valid function pointers that may not be valid
to call in `self`. The caller is responsible for ensuring that the
returned function is a valid function to call in `self` by either checking
the OpenGL API and version or for an appropriate OpenGL extension.

Note: On success, you need to cast the returned function pointer to the
correct type to be able to call it correctly. On 32-bit Windows, this will
include the `GSTGLAPI` identifier to use the correct calling convention.
e.g.


```C
void (GSTGLAPI *PFN_glGetIntegerv) (GLenum name, GLint * ret)
```
## `name`
an opengl function name

# Returns

a function pointer or `None`
<!-- trait GLContextExt::fn get_thread -->

# Returns

The `glib::Thread`, `self` is current in or NULL
<!-- trait GLContextExt::fn get_window -->

# Returns

the currently set window
<!-- trait GLContextExt::fn is_shared -->

# Returns

Whether the `GLContext` has been shared with another `GLContext`
<!-- trait GLContextExt::fn set_shared_with -->
Will internally set `self` as shared with `share`
## `share`
another `GLContext`
<!-- trait GLContextExt::fn set_window -->
Set's the current window on `self` to `window`. The window can only be
changed before `GLContextExt::create` has been called and the `window` is not
already running.
## `window`
a `GLWindow`

# Returns

Whether the window was successfully updated
<!-- trait GLContextExt::fn supports_glsl_profile_version -->
## `version`
a `GLSLVersion`
## `profile`
a `GLSLProfile`

# Returns

Whether `self` supports the combination of `version` with `profile`
<!-- trait GLContextExt::fn swap_buffers -->
Swap the front and back buffers on the window attached to `self`.
This will display the frame on the next refresh cycle.
<!-- trait GLContextExt::fn thread_add -->
Execute `func` in the OpenGL thread of `self` with `data`

MT-safe
## `func`
a `GstGLContextThreadFunc`
## `data`
user data to call `func` with
<!-- enum GLContextError -->
OpenGL context errors.
<!-- enum GLContextError::variant Failed -->
Failed for an unspecified reason
<!-- enum GLContextError::variant WrongConfig -->
The configuration requested is not correct
<!-- enum GLContextError::variant WrongApi -->
The OpenGL API requested is not correct
<!-- enum GLContextError::variant OldLibs -->
The OpenGL libraries are too old
<!-- enum GLContextError::variant CreateContext -->
glXCreateContext (or similar) failed
<!-- enum GLContextError::variant ResourceUnavailable -->
A resource is not available
<!-- struct GLDisplay -->
`GLDisplay` represents a connection to the underlying windowing system.
Elements are required to make use of `gst::Context` to share and propogate
a `GLDisplay`.

There are a number of environment variables that influence the choice of
platform and window system specific functionality.
- GST_GL_WINDOW influences the window system to use. Common values are
 'x11', 'wayland', 'win32' or 'cocoa'.
- GST_GL_PLATFORM influences the OpenGL platform to use. Common values are
 'egl', 'glx', 'wgl' or 'cgl'.
- GST_GL_API influences the OpenGL API requested by the OpenGL platform.
 Common values are 'opengl', 'opengl3' and 'gles2'.

> Certain window systems require a special function to be called to
> initialize threading support. As this GStreamer GL library does not preclude
> concurrent access to the windowing system, it is strongly advised that
> applications ensure that threading support has been initialized before any
> other toolkit/library functionality is accessed. Failure to do so could
> result in sudden application abortion during execution. The most notably
> example of such a function is X11's XInitThreads\().

# Implements

[`GLDisplayExt`](trait.GLDisplayExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait GLDisplayExt -->
Trait containing all `GLDisplay` methods.

# Implementors

[`GLDisplayEGL`](struct.GLDisplayEGL.html), [`GLDisplay`](struct.GLDisplay.html)
<!-- impl GLDisplay::fn new -->

# Returns

a new `GLDisplay`
<!-- trait GLDisplayExt::fn add_context -->
## `context`
a `GLContext`

# Returns

whether `context` was successfully added. `false` may be returned
if there already exists another context for `context`'s active thread.

Must be called with the object lock held.
<!-- trait GLDisplayExt::fn create_context -->
It requires the display's object lock to be held.
## `other_context`
other `GLContext` to share resources with.
## `p_context`
resulting `GLContext`

# Returns

whether a new context could be created.
<!-- trait GLDisplayExt::fn create_window -->
It requires the display's object lock to be held.

# Returns

a new `GLWindow` for `self` or `None`.
<!-- trait GLDisplayExt::fn filter_gl_api -->
limit the use of OpenGL to the requested `gl_api`. This is intended to allow
application and elements to request a specific set of OpenGL API's based on
what they support. See `GLContextExt::get_gl_api` for the retreiving the
API supported by a `GLContext`.
## `gl_api`
a `GLAPI` to filter with
<!-- trait GLDisplayExt::fn find_window -->
Execute `compare_func` over the list of windows stored by `self`. The
first argment to `compare_func` is the `GLWindow` being checked and the
second argument is `data`.
## `data`
some data to pass to `compare_func`
## `compare_func`
a comparison function to run

# Returns

The first `GLWindow` that causes a match
 from `compare_func`
<!-- trait GLDisplayExt::fn get_gl_api -->
see `GLDisplayExt::filter_gl_api` for what the returned value represents

# Returns

the `GLAPI` configured for `self`
<!-- trait GLDisplayExt::fn get_gl_context_for_thread -->
## `thread`
a `glib::Thread`

# Returns

the `GLContext` current on `thread` or `None`

Must be called with the object lock held.
<!-- trait GLDisplayExt::fn get_handle -->

# Returns

the native handle for the display
<!-- trait GLDisplayExt::fn get_handle_type -->

# Returns

the `GLDisplayType` of `self`
<!-- trait GLDisplayExt::fn remove_window -->
## `window`
a `GLWindow` to remove

# Returns

if `window` could be removed from `self`
<!-- trait GLDisplayExt::fn connect_create_context -->
Overrides the `GLContext` creation mechanism.
It can be called in any thread and it is emitted with
display's object lock held.
## `context`
other context to share resources with.

# Returns

the new context.
<!-- struct GLDisplayEGL -->
the contents of a `GLDisplayEGL` are private and should only be accessed
through the provided API

# Implements

[`GLDisplayExt`](trait.GLDisplayExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl GLDisplayEGL::fn new -->
Create a new `GLDisplayEGL` using the default EGL_DEFAULT_DISPLAY.

# Returns

a new `GLDisplayEGL` or `None`
<!-- impl GLDisplayEGL::fn get_from_native -->
Attempts to create a new `EGLDisplay` from `display`. If `type_` is
`GLDisplayType::Any`, then `display` must be 0. `type_` must not be
`GLDisplayType::None`.
## `type_`
a `GLDisplayType`
## `display`
pointer to a display (or 0)

# Returns

A `EGLDisplay` or `EGL_NO_DISPLAY`
<!-- enum GLFormat -->
<!-- enum GLFormat::variant Luminance -->
Single component replicated across R, G, and B textures
 components
<!-- enum GLFormat::variant Alpha -->
Single component stored in the A texture component
<!-- enum GLFormat::variant LuminanceAlpha -->
Combination of `GLFormat::Luminance` and `GLFormat::Alpha`
<!-- enum GLFormat::variant Red -->
Single component stored in the R texture component
<!-- enum GLFormat::variant R8 -->
Single 8-bit component stored in the R texture component
<!-- enum GLFormat::variant Rg -->
Two components stored in the R and G texture components
<!-- enum GLFormat::variant Rg8 -->
Two 8-bit components stored in the R and G texture components
<!-- enum GLFormat::variant Rgb -->
Three components stored in the R, G, and B texture components
<!-- enum GLFormat::variant Rgb8 -->
Three 8-bit components stored in the R, G, and B
 texture components
<!-- enum GLFormat::variant Rgb565 -->
Three components of bit depth 5, 6 and 5 stored in the R, G,
 and B texture components respectively.
<!-- enum GLFormat::variant Rgba -->
Four components stored in the R, G, B, and A texture
 components respectively.
<!-- enum GLFormat::variant Rgba8 -->
Four 8-bit components stored in the R, G, B, and A texture
 components respectively.
<!-- enum GLFormat::variant DepthComponent16 -->
A single 16-bit component for depth information.
<!-- enum GLFormat::variant Depth24Stencil8 -->
A 24-bit component for depth information and
 a 8-bit component for stencil informat.
<!-- struct GLFramebuffer -->
A `GLFramebuffer` represents and holds an OpenGL framebuffer object with
it's associated attachments.

A `GLFramebuffer` can be created with `GLFramebuffer::new` or
`GLFramebuffer::new_with_default_depth` and bound with
`GLFramebufferExt::bind`. Other resources can be bound with
`GLFramebufferExt::attach`

Note: OpenGL framebuffers are not shareable resources so cannot be used
between multiple OpenGL contexts.

# Implements

[`GLFramebufferExt`](trait.GLFramebufferExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait GLFramebufferExt -->
Trait containing all `GLFramebuffer` methods.

# Implementors

[`GLFramebuffer`](struct.GLFramebuffer.html)
<!-- impl GLFramebuffer::fn new -->
## `context`
a `GLContext`

# Returns

a new `GLFramebuffer`
<!-- impl GLFramebuffer::fn new_with_default_depth -->
## `context`
a `GLContext`
## `width`
width for the depth buffer
## `height`
for the depth buffer

# Returns

a new `GLFramebuffer` with a depth buffer of `width` and `height`
<!-- trait GLFramebufferExt::fn attach -->
attach `mem` to `attachment_point`
## `attachment_point`
the OpenGL attachment point to bind `mem` to
## `mem`
the memory object to bind to `attachment_point`
<!-- trait GLFramebufferExt::fn bind -->
Bind `self` into the current thread
<!-- trait GLFramebufferExt::fn draw_to_texture -->
Perform the steps necessary to have the output of a glDraw* command in
`func` update the contents of `mem`.
## `mem`
the `GLMemory` to draw to
## `func`
the function to run
## `user_data`
data to pass to `func`

# Returns

the result of executing `func`
<!-- trait GLFramebufferExt::fn get_effective_dimensions -->
Retreive the effective dimensions from the current attachments attached to
`self`.
## `width`
output width
## `height`
output height
<!-- trait GLFramebufferExt::fn get_id -->

# Returns

the OpenGL id for `self`
<!-- struct GLOverlayCompositor -->
Opaque `GLOverlayCompositor` object

# Implements

[`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- struct GLQuery -->
A `GLQuery` represents and holds an OpenGL query object. Various types of
queries can be run or counters retrieved.
<!-- impl GLQuery::fn counter -->
Record the result of a counter
<!-- impl GLQuery::fn end -->
End counting the query
<!-- impl GLQuery::fn free -->
Frees a `GLQuery`
<!-- impl GLQuery::fn init -->
## `context`
a `GLContext`
## `query_type`
the `GLQueryType`
<!-- impl GLQuery::fn result -->

# Returns

the result of the query
<!-- impl GLQuery::fn start -->
Start counting the query
<!-- impl GLQuery::fn unset -->
Free any dynamically allocated resources
<!-- impl GLQuery::fn local_gl_context -->
Performs a GST_QUERY_CONTEXT query of type "gst.gl.local_context" on all
`GstPads` in `element` of `direction` for the local OpenGL context used by
GStreamer elements.
## `element`
a `gst::Element` to query from
## `direction`
the `gst::PadDirection` to query
## `context_ptr`
location containing the current and/or resulting
 `GLContext`

# Returns

whether `context_ptr` contains a `GLContext`
<!-- impl GLQuery::fn new -->
Free with `GLQuery::free`
## `context`
a `GLContext`
## `query_type`
the `GLQueryType` to create

# Returns

a new `GLQuery`
<!-- enum GLQueryType -->
<!-- enum GLQueryType::variant None -->
no query
<!-- enum GLQueryType::variant TimeElapsed -->
query the time elapsed
<!-- enum GLQueryType::variant Timestamp -->
query the current time
<!-- enum GLSLError -->
Compilation stage that caused an error
<!-- enum GLSLError::variant Compile -->
Compilation error occured
<!-- enum GLSLError::variant Link -->
Link error occured
<!-- enum GLSLError::variant Program -->
General program error occured
<!-- struct GLSLStage -->
`GLSLStage` holds and represents a single OpenGL shader stage.

# Implements

[`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl GLSLStage::fn new -->
## `context`
a `GLContext`
## `type_`
the GL enum shader stage type

# Returns

a new `GLSLStage` of the specified `type_`
<!-- impl GLSLStage::fn new_with_string -->
## `context`
a `GLContext`
## `type_`
the GL enum shader stage type
## `version`
the `GLSLVersion`
## `profile`
the `GLSLProfile`
## `str`
a shader string

# Returns

a new `GLSLStage` of the specified `type_`
<!-- impl GLSLStage::fn new_with_strings -->
## `context`
a `GLContext`
## `type_`
the GL enum shader stage type
## `version`
the `GLSLVersion`
## `profile`
the `GLSLProfile`
## `n_strings`
the number of strings in `str`
## `str`

 an array of strings concatted together to produce a shader

# Returns

a new `GLSLStage` of the specified `type_`
<!-- impl GLSLStage::fn compile -->

# Returns

whether the compilation suceeded
<!-- impl GLSLStage::fn get_handle -->

# Returns

The GL handle for this shader stage
<!-- impl GLSLStage::fn get_profile -->

# Returns

The GLSL profile for the current shader stage
<!-- impl GLSLStage::fn get_shader_type -->

# Returns

The GL shader type for this shader stage
<!-- impl GLSLStage::fn get_version -->

# Returns

The GLSL version for the current shader stage
<!-- impl GLSLStage::fn set_strings -->
Replaces the current shader string with `str`.
## `version`
a `GLSLVersion`
## `profile`
a `GLSLProfile`
## `n_strings`
number of strings in `str`
## `str`
a GLSL shader string
<!-- enum GLSLVersion -->
GLSL version list
<!-- enum GLSLVersion::variant None -->
no version
<!-- enum GLSLVersion::variant 100 -->
version 100 (only valid for ES)
<!-- enum GLSLVersion::variant 110 -->
version 110 (only valid for compatibility desktop GL)
<!-- enum GLSLVersion::variant 120 -->
version 120 (only valid for compatibility desktop GL)
<!-- enum GLSLVersion::variant 130 -->
version 130 (only valid for compatibility desktop GL)
<!-- enum GLSLVersion::variant 140 -->
version 140 (only valid for compatibility desktop GL)
<!-- enum GLSLVersion::variant 150 -->
version 150 (valid for compatibility/core desktop GL)
<!-- enum GLSLVersion::variant 300 -->
version 300 (only valid for ES)
<!-- enum GLSLVersion::variant 310 -->
version 310 (only valid for ES)
<!-- enum GLSLVersion::variant 320 -->
version 320 (only valid for ES)
<!-- enum GLSLVersion::variant 330 -->
version 330 (valid for compatibility/core desktop GL)
<!-- enum GLSLVersion::variant 400 -->
version 400 (valid for compatibility/core desktop GL)
<!-- enum GLSLVersion::variant 410 -->
version 410 (valid for compatibility/core desktop GL)
<!-- enum GLSLVersion::variant 420 -->
version 420 (valid for compatibility/core desktop GL)
<!-- enum GLSLVersion::variant 430 -->
version 430 (valid for compatibility/core desktop GL)
<!-- enum GLSLVersion::variant 440 -->
version 440 (valid for compatibility/core desktop GL)
<!-- enum GLSLVersion::variant 450 -->
version 450 (valid for compatibility/core desktop GL)
<!-- struct GLShader -->


# Implements

[`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl GLShader::fn new -->
Note: must be called in the GL thread
## `context`
a `GLContext`

# Returns

a new empty `shader`
<!-- impl GLShader::fn new_default -->
Note: must be called in the GL thread
## `context`
a `GLContext`

# Returns

a default `shader` or `None` on failure
<!-- impl GLShader::fn new_link_with_stages -->
Each stage will attempt to be compiled and attached to `shader`. Then
the shader will be linked. On error, `None` will be returned and `error` will
contain the details of the error.

Note: must be called in the GL thread
## `context`
a `GLContext`
## `error`
a `glib::Error`

# Returns

a new `shader` with the specified stages.
<!-- impl GLShader::fn new_with_stages -->
Each stage will attempt to be compiled and attached to `shader`. On error,
`None` will be returned and `error` will contain the details of the error.

Note: must be called in the GL thread
## `context`
a `GLContext`
## `error`
a `glib::Error`

# Returns

a new `shader` with the specified stages.
<!-- impl GLShader::fn attach -->
Attaches `stage` to `self`. `stage` must have been successfully compiled
with `GLSLStage::compile`.

Note: must be called in the GL thread
## `stage`
a `GLSLStage` to attach

# Returns

whether `stage` could be attached to `self`
<!-- impl GLShader::fn attach_unlocked -->
Attaches `stage` to `self`. `stage` must have been successfully compiled
with `GLSLStage::compile`.

Note: must be called in the GL thread
## `stage`
a `GLSLStage` to attach

# Returns

whether `stage` could be attached to `self`
<!-- impl GLShader::fn bind_attribute_location -->
Bind attribute `name` to the specified location `index` using
glBindAttributeLocation().
## `index`
attribute index to set
## `name`
name of the attribute
<!-- impl GLShader::fn bind_frag_data_location -->
Bind attribute `name` to the specified location `index` using
glBindFragDataLocation().
## `index`
attribute index to set
## `name`
name of the attribute
<!-- impl GLShader::fn compile_attach_stage -->
Compiles `stage` and attaches it to `self`.

Note: must be called in the GL thread
## `stage`
a `GLSLStage` to attach

# Returns

whether `stage` could be compiled and attached to `self`
<!-- impl GLShader::fn detach -->
Detaches `stage` from `self`. `stage` must have been successfully attached
to `self` with `GLShader::attach` or `GLShader::attach_unlocked`.

Note: must be called in the GL thread
## `stage`
a `GLSLStage` to attach
<!-- impl GLShader::fn detach_unlocked -->
Detaches `stage` from `self`. `stage` must have been successfully attached
to `self` with `GLShader::attach` or `GLShader::attach_unlocked`.

Note: must be called in the GL thread
## `stage`
a `GLSLStage` to attach
<!-- impl GLShader::fn get_attribute_location -->
## `name`
name of the attribute

# Returns

the attribute index for `name` in `self` or -1 on failure
<!-- impl GLShader::fn get_program_handle -->

# Returns

the GL program handle for this shader
<!-- impl GLShader::fn is_linked -->
Note: must be called in the GL thread

# Returns

whether `self` has been successfully linked
<!-- impl GLShader::fn link -->
Links the current list of `GLSLStage`'s in `self`.

Note: must be called in the GL thread

# Returns

whether `self` could be linked together.
<!-- impl GLShader::fn release -->
Releases the shader and stages.

Note: must be called in the GL thread
<!-- impl GLShader::fn release_unlocked -->
Releases the shader and stages.

Note: must be called in the GL thread
<!-- impl GLShader::fn set_uniform_1f -->
Perform glUniform1f() for `name` on `self`
## `name`
name of the uniform
## `value`
value to set
<!-- impl GLShader::fn set_uniform_1fv -->
Perform glUniform1fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_1i -->
Perform glUniform1i() for `name` on `self`
## `name`
name of the uniform
## `value`
value to set
<!-- impl GLShader::fn set_uniform_1iv -->
Perform glUniform1iv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_2f -->
Perform glUniform2f() for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
<!-- impl GLShader::fn set_uniform_2fv -->
Perform glUniform2fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_2i -->
Perform glUniform2i() for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
<!-- impl GLShader::fn set_uniform_2iv -->
Perform glUniform2iv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_3f -->
Perform glUniform3f() for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
## `v2`
third value to set
<!-- impl GLShader::fn set_uniform_3fv -->
Perform glUniform3fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_3i -->
Perform glUniform3i() for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
## `v2`
third value to set
<!-- impl GLShader::fn set_uniform_3iv -->
Perform glUniform3iv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_4f -->
Perform glUniform4f() for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
## `v2`
third value to set
## `v3`
fourth value to set
<!-- impl GLShader::fn set_uniform_4fv -->
Perform glUniform4fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_4i -->
Perform glUniform4i() for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
## `v2`
third value to set
## `v3`
fourth value to set
<!-- impl GLShader::fn set_uniform_4iv -->
Perform glUniform4iv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_2fv -->
Perform glUniformMatrix2fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 2x2 matrices to set
## `transpose`
transpose the matrix
## `value`
matrix to set
<!-- impl GLShader::fn set_uniform_matrix_2x3fv -->
Perform glUniformMatrix2x3fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 2x3 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_2x4fv -->
Perform glUniformMatrix2x4fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 2x4 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_3fv -->
Perform glUniformMatrix3fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 3x3 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_3x2fv -->
Perform glUniformMatrix3x2fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 3x2 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_3x4fv -->
Perform glUniformMatrix3x4fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 3x4 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_4fv -->
Perform glUniformMatrix4fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 4x4 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_4x2fv -->
Perform glUniformMatrix4x2fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 4x2 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_4x3fv -->
Perform glUniformMatrix4x3fv() for `name` on `self`
## `name`
name of the uniform
## `count`
number of 4x3 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn use -->
Mark's `self` as being used for the next GL draw command.

Note: must be called in the GL thread and `self` must have been linked.
<!-- enum GLStereoDownmix -->
Output anaglyph type to generate when downmixing to mono
<!-- enum GLTextureTarget -->
<!-- enum GLTextureTarget::variant None -->
no texture target
<!-- enum GLTextureTarget::variant 2d -->
2D texture target
<!-- enum GLTextureTarget::variant Rectangle -->
rectangle texture target
<!-- enum GLTextureTarget::variant ExternalOes -->
external oes texture target
<!-- struct GLUpload -->
`GLUpload` is an object that uploads data from system memory into GL textures.

A `GLUpload` can be created with `GLUpload::new`

# Implements

[`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl GLUpload::fn new -->
## `context`
a `GLContext`

# Returns

a new `GLUpload` object
<!-- impl GLUpload::fn get_caps -->
## `in_caps`
the input `gst::Caps`
## `out_caps`
the output `gst::Caps`
<!-- impl GLUpload::fn perform_with_buffer -->
Uploads `buffer` using the transformation specified by
`GLUpload::set_caps` creating a new `gst::Buffer` in `outbuf_ptr`.
## `buffer`
input `gst::Buffer`
## `outbuf_ptr`
resulting `gst::Buffer`

# Returns

whether the upload was successful
<!-- impl GLUpload::fn propose_allocation -->
Adds the required allocation parameters to support uploading.
## `decide_query`
a `gst::Query` from a decide allocation
## `query`
the proposed allocation query
<!-- impl GLUpload::fn set_caps -->
Initializes `self` with the information required for upload.
## `in_caps`
input `gst::Caps`
## `out_caps`
output `gst::Caps`

# Returns

whether `in_caps` and `out_caps` could be set on `self`
<!-- enum GLUploadReturn -->
<!-- enum GLUploadReturn::variant Done -->
No further processing required
<!-- enum GLUploadReturn::variant Error -->
An unspecified error occured
<!-- enum GLUploadReturn::variant Unsupported -->
The configuration is unsupported.
<!-- enum GLUploadReturn::variant Reconfigure -->
This element requires a reconfiguration.
<!-- struct GLViewConvert -->
Convert stereoscopic/multiview video using fragment shaders.

# Implements

[`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl GLViewConvert::fn new -->

# Returns

a new `GLViewConvert`
<!-- impl GLViewConvert::fn fixate_caps -->
Provides an implementation of `gst_base::BaseTransformClass::fixate_caps`()
## `direction`
a `gst::PadDirection`
## `caps`
the `gst::Caps` of `direction`
## `othercaps`
the `gst::Caps` to fixate

# Returns

the fixated `gst::Caps`
<!-- impl GLViewConvert::fn get_output -->
Retrieve the processed output buffer placing the output in `outbuf_ptr`.
## `outbuf_ptr`
a `gst::Buffer`

# Returns

a `gst::FlowReturn`
<!-- impl GLViewConvert::fn perform -->
Converts the data contained by `inbuf` using the formats specified by the
`gst::Caps` passed to `GLViewConvert::set_caps`
## `inbuf`
the `GLMemory` filled `gst::Buffer` to convert

# Returns

a converted `gst::Buffer` or `None`
<!-- impl GLViewConvert::fn reset -->
Reset `self` to the default state. Further operation will require
setting the caps with `GLViewConvert::set_caps`.
<!-- impl GLViewConvert::fn set_caps -->
Initializes `self` with the information required for conversion.
## `in_caps`
input `gst::Caps`
## `out_caps`
output `gst::Caps`
<!-- impl GLViewConvert::fn set_context -->
Set `context` on `self`
## `context`
the `GLContext` to set
<!-- impl GLViewConvert::fn submit_input_buffer -->
Submit `input` to be processed by `self`
## `is_discont`
true if we have a discontinuity
## `input`
a `gst::Buffer`

# Returns

a `gst::FlowReturn`
<!-- impl GLViewConvert::fn transform_caps -->
Provides an implementation of `gst_base::BaseTransformClass::transform_caps`()
## `direction`
a `gst::PadDirection`
## `caps`
the `gst::Caps` to transform
## `filter`
a set of filter `gst::Caps`

# Returns

the converted `gst::Caps`
<!-- struct GLWindow -->
GstGLWindow represents a window that elements can render into. A window can
either be a user visible window (onscreen) or hidden (offscreen).

# Implements

[`GLWindowExt`](trait.GLWindowExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait GLWindowExt -->
Trait containing all `GLWindow` methods.

# Implementors

[`GLWindow`](struct.GLWindow.html)
<!-- impl GLWindow::fn new -->
## `display`
a `GLDisplay`

# Returns

a new `GLWindow` using `display`'s connection
<!-- trait GLWindowExt::fn draw -->
Redraw the window contents. Implementations should invoke the draw callback.
<!-- trait GLWindowExt::fn get_context -->

# Returns

the `GLContext` associated with this `self`
<!-- trait GLWindowExt::fn get_display -->

# Returns

the windowing system display handle for this `self`
<!-- trait GLWindowExt::fn get_surface_dimensions -->
## `width`
resulting surface width
## `height`
resulting surface height
<!-- trait GLWindowExt::fn get_window_handle -->

# Returns

the window handle we are currently rendering into
<!-- trait GLWindowExt::fn handle_events -->
Tell a `self` that it should handle events from the window system. These
events are forwarded upstream as navigation events. In some window systems
events are not propagated in the window hierarchy if a client is listening
for them. This method allows you to disable events handling completely
from the `self`.
## `handle_events`
a `gboolean` indicating if events should be handled or not.
<!-- trait GLWindowExt::fn quit -->
Quit the runloop's execution.
<!-- trait GLWindowExt::fn run -->
Start the execution of the runloop.
<!-- trait GLWindowExt::fn send_message -->
Invoke `callback` with data on the window thread. `callback` is guarenteed to
have executed when this function returns.
## `callback`
function to invoke
## `data`
data to invoke `callback` with
<!-- trait GLWindowExt::fn send_message_async -->
Invoke `callback` with `data` on the window thread. The callback may not
have been executed when this function returns.
## `callback`
function to invoke
## `data`
data to invoke `callback` with
## `destroy`
called when `data` is not needed anymore
<!-- trait GLWindowExt::fn set_close_callback -->
Sets the callback called when the window is about to close.
## `callback`
function to invoke
## `data`
data to invoke `callback` with
## `destroy_notify`
called when `data` is not needed any more
<!-- trait GLWindowExt::fn set_draw_callback -->
Sets the draw callback called everytime `GLWindowExt::draw` is called
## `callback`
function to invoke
## `data`
data to invoke `callback` with
## `destroy_notify`
called when `data` is not needed any more
<!-- trait GLWindowExt::fn set_preferred_size -->
Set the preferred width and height of the window. Implementations are free
to ignore this information.
## `width`
new preferred width
## `height`
new preferred height
<!-- trait GLWindowExt::fn set_render_rectangle -->
Tell a `self` that it should render into a specific region of the window
according to the `gst_video::VideoOverlay` interface.
## `x`
x position
## `y`
y position
## `width`
width
## `height`
height

# Returns

whether the specified region could be set
<!-- trait GLWindowExt::fn set_resize_callback -->
Sets the resize callback called everytime a resize of the window occurs.
## `callback`
function to invoke
## `data`
data to invoke `callback` with
## `destroy_notify`
called when `data` is not needed any more
<!-- trait GLWindowExt::fn set_window_handle -->
Sets the window that this `self` should render into. Some implementations
require this to be called with a valid handle before drawing can commence.
## `handle`
handle to the window
<!-- trait GLWindowExt::fn show -->
Present the window to the screen.
<!-- trait GLWindowExt::fn connect_key_event -->
Will be emitted when a key event is received by the GstGLwindow.
## `id`
the name of the event
## `key`
the id of the key pressed
<!-- trait GLWindowExt::fn connect_mouse_event -->
Will be emitted when a mouse event is received by the GstGLwindow.
## `id`
the name of the event
## `button`
the id of the button
## `x`
the x coordinate of the mouse event
## `y`
the y coordinate of the mouse event
<!-- enum GLWindowError -->
<!-- enum GLWindowError::variant Failed -->
failed for a unspecified reason
<!-- enum GLWindowError::variant OldLibs -->
the implementation is too old
<!-- enum GLWindowError::variant ResourceUnavailable -->
no such resource was found
