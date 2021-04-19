<!-- file * -->
<!-- struct GLAPI -->
<!-- struct GLAPI::const NONE -->
no API
<!-- struct GLAPI::const OPENGL -->
Desktop OpenGL up to and including 3.1. The
 compatibility profile when the OpenGL version is >= 3.2
<!-- struct GLAPI::const OPENGL3 -->
Desktop OpenGL >= 3.2 core profile
<!-- struct GLAPI::const GLES1 -->
OpenGL ES 1.x
<!-- struct GLAPI::const GLES2 -->
OpenGL ES 2.x and 3.x
<!-- struct GLAPI::const ANY -->
Any OpenGL API
<!-- struct GLAllocationParams -->
<!-- impl GLAllocationParams::fn copy -->

# Returns

a copy of the `GLAllocationParams` specified by
 `self` or `None` on failure
<!-- impl GLAllocationParams::fn copy_data -->
Copies the dynamically allocated data from `self` to `dest`. Direct subclasses
should call this function in their own overridden copy function.
## `dest`
the destination `GLAllocationParams`
<!-- impl GLAllocationParams::fn free -->
Frees the `GLAllocationParams` and all associated data.
<!-- impl GLAllocationParams::fn free_data -->
Frees the dynamically allocated data in `self`. Direct subclasses
should call this function in their own overridden free function.
<!-- impl GLAllocationParams::fn init -->
`notify` will be called once for each allocated memory using these `self`
when freeing the memory.
## `struct_size`
the struct size of the implementation
## `alloc_flags`
some alloc flags
## `copy`
a copy function
## `free`
a free function
## `context`
a `GLContext`
## `alloc_size`
the number of bytes to allocate.
## `alloc_params`
a `gst::AllocationParams` to apply
## `wrapped_data`
a sysmem data pointer to initialize the allocation with
## `gl_handle`
a GL handle to initialize the allocation with
## `user_data`
user data to call `notify` with
## `notify`
a `GDestroyNotify`

# Returns

whether the parameters could be initialized
<!-- struct GLBaseFilter -->
`GLBaseFilter` handles the nitty gritty details of retrieving an OpenGL
context. It also provided some wrappers around `gst_base::BaseTransform`'s
`start()`, `stop()` and `set_caps()` virtual methods that ensure an OpenGL
context is available and current in the calling thread.

# Implements

[`GLBaseFilterExt`](trait@crate::GLBaseFilterExt), [`trait@gst_base::BaseTransformExt`], [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- trait GLBaseFilterExt -->
Trait containing all `GLBaseFilter` methods.

# Implementors

[`GLBaseFilter`](struct@crate::GLBaseFilter), [`GLFilter`](struct@crate::GLFilter)
<!-- trait GLBaseFilterExt::fn find_gl_context -->

Feature: `v1_16`


# Returns

Whether an OpenGL context could be retrieved or created successfully
<!-- trait GLBaseFilterExt::fn gl_context -->

Feature: `v1_18`


# Returns

the `GLContext` found by `self`
<!-- struct GLBaseMemory -->
GstGLBaseMemory is a `gst::Memory` subclass providing the basis of support
for the mapping of GL buffers.

Data is uploaded or downloaded from the GPU as is necessary.
<!-- impl GLBaseMemory::fn alloc_data -->
Note: only intended for subclass usage to allocate the system memory buffer
on demand. If there is already a non-NULL data pointer in `self`->data,
then this function imply returns TRUE.

# Returns

whether the system memory could be allocated
<!-- impl GLBaseMemory::fn init -->
Initializes `self` with the required parameters
## `allocator`
the `gst::Allocator` to initialize with
## `parent`
the parent `gst::Memory` to initialize with
## `context`
the `GLContext` to initialize with
## `params`
the [`crate::gst::AllocationParams`] (XXX: @-reference does not belong to GLBaseMemory!) to initialize with
## `size`
the number of bytes to be allocated
## `user_data`
user data to call `notify` with
## `notify`
a `GDestroyNotify`
<!-- impl GLBaseMemory::fn memcpy -->
## `dest`
the destination `GLBaseMemory`
## `offset`
the offset to start at
## `size`
the number of bytes to copy

# Returns

whether the copy succeeded.
<!-- impl GLBaseMemory::fn alloc -->
## `allocator`
a `GLBaseMemoryAllocator`
## `params`
the `GLAllocationParams` to allocate the memory with

# Returns

a new `GLBaseMemory` from `allocator` with the requested `params`.
<!-- impl GLBaseMemory::fn init_once -->
Initializes the GL Base Memory allocator. It is safe to call this function
multiple times. This must be called before any other GstGLBaseMemory operation.
<!-- struct GLBaseMemoryAllocator -->
Opaque `GLBaseMemoryAllocator` struct

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`trait@gst::AllocatorExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
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

[`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- impl GLColorConvert::fn new -->
## `context`
a `GLContext`

# Returns

a new `GLColorConvert` object
<!-- impl GLColorConvert::fn fixate_caps -->
Provides an implementation of `gst_base::BaseTransformClass.fixate_caps`()
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
Provides an implementation of `gst_base::BaseTransformClass.transform_caps`()
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
Provides an implementation of `gst_base::BaseTransformClass.decide_allocation`()
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

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`GLContextExt`](trait@crate::GLContextExt), [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`], [`GLContextExtManual`](trait@crate::GLContextExtManual)
<!-- trait GLContextExt -->
Trait containing all `GLContext` methods.

# Implementors

[`GLContext`](struct@crate::GLContext)
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

`context_type` must not be `GLPlatform::None` or `GLPlatform::Any`

`available_apis` must not be `GLAPI::None` or `GLAPI::Any`
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
to retrieve `name`.

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
<!-- trait GLContextExt::fn display -->

# Returns

the `GLDisplay` associated with this `self`
<!-- trait GLContextExt::fn gl_api -->
Get the currently enabled OpenGL api.

The currently available API may be limited by the `GLDisplay` in use and/or
the `GLWindow` chosen.

# Returns

the available OpenGL api
<!-- trait GLContextExt::fn gl_context -->
Gets the backing OpenGL context used by `self`.

# Returns

The platform specific backing OpenGL context
<!-- trait GLContextExt::fn gl_platform -->
Gets the OpenGL platform that used by `self`.

# Returns

The platform specific backing OpenGL context
<!-- trait GLContextExt::fn gl_platform_version -->
Get the version of the OpenGL platform (GLX, EGL, etc) used. Only valid
after a call to `GLContextExt::create`.
## `major`
return for the major version
## `minor`
return for the minor version
<!-- trait GLContextExt::fn gl_version -->
Returns the OpenGL version implemented by `self`. See
`GLContextExt::get_gl_api` for retrieving the OpenGL api implemented by
`self`.
## `maj`
resulting major version
## `min`
resulting minor version
<!-- trait GLContextExt::fn get_proc_address -->
Get a function pointer to a specified opengl function, `name`. If the the
specific function does not exist, NULL is returned instead.

Platform specific functions (names starting 'egl', 'glX', 'wgl', etc) can also
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
<!-- trait GLContextExt::fn window -->

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
<!-- trait GLContextExt::fn supports_precision -->

Feature: `v1_16`

## `version`
a `GLSLVersion`
## `profile`
a `GLSLProfile`

# Returns

whether `self` supports the 'precision' specifier in GLSL shaders
<!-- trait GLContextExt::fn supports_precision_highp -->

Feature: `v1_16`

## `version`
a `GLSLVersion`
## `profile`
a `GLSLProfile`

# Returns

whether `self` supports the 'precision highp' specifier in GLSL shaders
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
Elements are required to make use of `gst::Context` to share and propagate
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

[`GLDisplayExt`](trait@crate::GLDisplayExt), [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- trait GLDisplayExt -->
Trait containing all `GLDisplay` methods.

# Implementors

[`GLDisplay`](struct@crate::GLDisplay)
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

# Returns

a new `GLWindow` for `self` or `None`.
<!-- trait GLDisplayExt::fn filter_gl_api -->
limit the use of OpenGL to the requested `gl_api`. This is intended to allow
application and elements to request a specific set of OpenGL API's based on
what they support. See `GLContextExt::get_gl_api` for the retrieving the
API supported by a `GLContext`.
## `gl_api`
a `GLAPI` to filter with
<!-- trait GLDisplayExt::fn find_window -->
Deprecated for `GLDisplayExt::retrieve_window`.

Execute `compare_func` over the list of windows stored by `self`. The
first argument to `compare_func` is the `GLWindow` being checked and the
second argument is `data`.
## `data`
some data to pass to `compare_func`
## `compare_func`
a comparison function to run

# Returns

The first `GLWindow` that causes a match
 from `compare_func`
<!-- trait GLDisplayExt::fn gl_api -->
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
<!-- trait GLDisplayExt::fn handle_type -->

# Returns

the `GLDisplayType` of `self`
<!-- trait GLDisplayExt::fn remove_context -->
Must be called with the object lock held.

Feature: `v1_18`

## `context`
the `GLContext` to remove
<!-- trait GLDisplayExt::fn remove_window -->
## `window`
a `GLWindow` to remove

# Returns

if `window` could be removed from `self`
<!-- trait GLDisplayExt::fn retrieve_window -->
Execute `compare_func` over the list of windows stored by `self`. The
first argument to `compare_func` is the `GLWindow` being checked and the
second argument is `data`.

Feature: `v1_18`

## `data`
some data to pass to `compare_func`
## `compare_func`
a comparison function to run

# Returns

The first `GLWindow` that causes a match
 from `compare_func`
<!-- trait GLDisplayExt::fn connect_create_context -->
Overrides the [`crate::GLContext`] (XXX: @-reference does not belong to GLDisplayExt!) creation mechanism.
It can be called in any thread and it is emitted with
display's object lock held.
## `context`
other context to share resources with.

# Returns

the new context.
<!-- struct GLDisplayType -->
<!-- struct GLDisplayType::const NONE -->
no display type
<!-- struct GLDisplayType::const X11 -->
X11 display
<!-- struct GLDisplayType::const WAYLAND -->
Wayland display
<!-- struct GLDisplayType::const COCOA -->
Cocoa display
<!-- struct GLDisplayType::const WIN32 -->
Win32 display
<!-- struct GLDisplayType::const DISPMANX -->
Dispmanx display
<!-- struct GLDisplayType::const EGL -->
EGL display
<!-- struct GLDisplayType::const VIV_FB -->
Vivante Framebuffer display
<!-- struct GLDisplayType::const GBM -->
Mesa3D GBM display
<!-- struct GLDisplayType::const EGL_DEVICE -->
EGLDevice display (Since: 1.18)
<!-- struct GLDisplayType::const ANY -->
any display type
<!-- struct GLFilter -->
`GLFilter` helps to implement simple OpenGL filter elements taking a
single input and producing a single output with a `GLFramebuffer`

# Implements

[`GLFilterExt`](trait@crate::GLFilterExt), [`GLBaseFilterExt`](trait@crate::GLBaseFilterExt), [`trait@gst_base::BaseTransformExt`], [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- trait GLFilterExt -->
Trait containing all `GLFilter` methods.

# Implementors

[`GLFilter`](struct@crate::GLFilter)
<!-- trait GLFilterExt::fn draw_fullscreen_quad -->
Render a fullscreen quad using the current GL state. The only GL state this
modifies is the necessary vertex/index buffers and, if necessary, a
Vertex Array Object for drawing a fullscreen quad. Framebuffer state,
any shaders, viewport state, etc must be setup by the caller.
<!-- trait GLFilterExt::fn filter_texture -->
Calls filter_texture vfunc with correctly mapped `GstGLMemorys`
## `input`
an input buffer
## `output`
an output buffer

# Returns

whether the transformation succeeded
<!-- trait GLFilterExt::fn render_to_target -->
Transforms `input` into `output` using `func` on through FBO.
## `input`
the input texture
## `output`
the output texture
## `func`
the function to transform `input` into `output`. called with `data`
## `data`
the data associated with `func`

# Returns

the return value of `func`
<!-- trait GLFilterExt::fn render_to_target_with_shader -->
Transforms `input` into `output` using `shader` with a FBO.

See also: `GLFilterExt::render_to_target`
## `input`
the input texture
## `output`
the output texture
## `shader`
the shader to use.
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
<!-- enum GLFormat::variant Rgb16 -->
Three 16-bit components stored in the R, G, and B
 texture components
<!-- enum GLFormat::variant Rgba -->
Four components stored in the R, G, B, and A texture
 components respectively.
<!-- enum GLFormat::variant Rgba8 -->
Four 8-bit components stored in the R, G, B, and A texture
 components respectively.
<!-- enum GLFormat::variant Rgba16 -->
Four 16-bit components stored in the R, G, B, and A texture
 components respectively.
<!-- enum GLFormat::variant DepthComponent16 -->
A single 16-bit component for depth information.
<!-- enum GLFormat::variant Depth24Stencil8 -->
A 24-bit component for depth information and
 a 8-bit component for stencil informat.
<!-- enum GLFormat::variant R16 -->
Single 16-bit component stored in the R texture component
<!-- enum GLFormat::variant Rg16 -->
Two 16-bit components stored in the R and G texture components
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

[`GLFramebufferExt`](trait@crate::GLFramebufferExt), [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- trait GLFramebufferExt -->
Trait containing all `GLFramebuffer` methods.

# Implementors

[`GLFramebuffer`](struct@crate::GLFramebuffer)
<!-- impl GLFramebuffer::fn new -->
## `context`
a `GLContext`

# Returns

a new `GLFramebuffer`
<!-- impl GLFramebuffer::fn with_default_depth -->
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
<!-- trait GLFramebufferExt::fn effective_dimensions -->
Retrieve the effective dimensions from the current attachments attached to
`self`.
## `width`
output width
## `height`
output height
<!-- trait GLFramebufferExt::fn id -->

# Returns

the OpenGL id for `self`
<!-- struct GLMemory -->
GstGLMemory is a `GLBaseMemory` subclass providing support for the mapping of
OpenGL textures.

`GLMemory` is created or wrapped through `GLBaseMemory::alloc`
with `GLVideoAllocationParams`.

Data is uploaded or downloaded from the GPU as is necessary.

The `gst::Caps` that is used for `GLMemory` based buffers should contain
the `GST_CAPS_FEATURE_MEMORY_GL_MEMORY` as a `gst::CapsFeatures` and should
contain a 'texture-target' field with one of the `GLTextureTarget` values
as a string, i.e. some combination of 'texture-target=(string){2D,
rectangle, external-oes}'.
<!-- impl GLMemory::fn copy_into -->
Copies `self` into the texture specified by `tex_id`. The format of `tex_id`
is specified by `tex_format`, `width` and `height`.
## `tex_id`
OpenGL texture id
## `target`
the `GLTextureTarget`
## `tex_format`
the `GLFormat`
## `width`
width of `tex_id`
## `height`
height of `tex_id`

# Returns

Whether the copy succeeded
<!-- impl GLMemory::fn copy_teximage -->
Copies the texture in `GLMemory` into the texture specified by `tex_id`,
`out_target`, `out_tex_format`, `out_width` and `out_height`.
## `tex_id`
the destination texture id
## `out_target`
the destination `GLTextureTarget`
## `out_tex_format`
the destination `GLFormat`
## `out_width`
the destination width
## `out_height`
the destination height

# Returns

whether the copy succeeded.
<!-- impl GLMemory::fn get_texture_format -->

# Returns

the `GLFormat` of `self`
<!-- impl GLMemory::fn get_texture_height -->

# Returns

the texture height of `self`
<!-- impl GLMemory::fn get_texture_id -->

# Returns

the OpenGL texture handle of `self`
<!-- impl GLMemory::fn get_texture_target -->

# Returns

the `GLTextureTarget` of `self`
<!-- impl GLMemory::fn get_texture_width -->

# Returns

the texture width of `self`
<!-- impl GLMemory::fn init -->
Initializes `self` with the required parameters. `info` is assumed to have
already have been modified with `gst_video::VideoInfo::align`.
## `allocator`
the `gst::Allocator` to initialize with
## `parent`
the parent `gst::Memory` to initialize with
## `context`
the `GLContext` to initialize with
## `target`
the `GLTextureTarget` for this `GLMemory`
## `tex_format`
the `GLFormat` for this `GLMemory`
## `params`
the [`crate::gst::AllocationParams`] (XXX: @-reference does not belong to GLMemory!) to initialize with
## `info`
the `gst_video::VideoInfo` for this `GLMemory`
## `plane`
the plane number (starting from 0) for this `GLMemory`
## `valign`
optional `gst_video::VideoAlignment` parameters
## `user_data`
user data to call `notify` with
## `notify`
a `GDestroyNotify`
<!-- impl GLMemory::fn read_pixels -->
Reads the texture in `GLMemory` into `write_pointer` if no buffer is bound
to `GL_PIXEL_PACK_BUFFER`. Otherwise `write_pointer` is the byte offset into
the currently bound `GL_PIXEL_PACK_BUFFER` buffer to store the result of
glReadPixels. See the OpenGL specification for glReadPixels for more
details.
## `write_pointer`
the data pointer to pass to glReadPixels

# Returns

whether theread operation succeeded
<!-- impl GLMemory::fn texsubimage -->
Reads the texture in `read_pointer` into `self`.

See `GLMemory::read_pixels` for what `read_pointer` signifies.
## `read_pointer`
the data pointer to pass to glTexSubImage
<!-- impl GLMemory::fn init_once -->
Initializes the GL Base Texture allocator. It is safe to call this function
multiple times. This must be called before any other GstGLMemory operation.
<!-- impl GLMemory::fn setup_buffer -->
## `allocator`
the [`crate::GLMemoryAllocator`] (XXX: @-reference does not belong to GLMemory!) to allocate from
## `buffer`
a `gst::Buffer` to setup
## `params`
the `GLVideoAllocationParams` to allocate with
## `tex_formats`

 a list of `GLFormat`'s to allocate with.
## `wrapped_data`

 a list of wrapped data pointers
## `n_wrapped_pointers`
the number of elements in `tex_formats` and `wrapped_data`

# Returns

whether the buffer was correctly setup
<!-- struct GLMemoryAllocator -->
Opaque `GLMemoryAllocator` struct

# Implements

[`GLBaseMemoryAllocatorExt`](trait@crate::GLBaseMemoryAllocatorExt), [`trait@gst::AllocatorExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- impl GLMemoryAllocator::fn get_default -->
## `context`
a `GLContext`

# Returns

the default `GLMemoryAllocator` supported by
 `context`
<!-- struct GLOverlayCompositor -->
Opaque `GLOverlayCompositor` object

# Implements

[`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- struct GLPlatform -->
<!-- struct GLPlatform::const NONE -->
no platform
<!-- struct GLPlatform::const EGL -->
the EGL platform used primarily with the X11, wayland
 and android window systems as well as on embedded Linux
<!-- struct GLPlatform::const GLX -->
the GLX platform used primarily with the X11 window system
<!-- struct GLPlatform::const WGL -->
the WGL platform used primarily on Windows
<!-- struct GLPlatform::const CGL -->
the CGL platform used primarily on OS X
<!-- struct GLPlatform::const EAGL -->
the EAGL platform used primarily on iOS
<!-- struct GLPlatform::const ANY -->
any OpenGL platform
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
Compilation error occurred
<!-- enum GLSLError::variant Link -->
Link error occurred
<!-- enum GLSLError::variant Program -->
General program error occurred
<!-- struct GLSLProfile -->
GLSL profiles
<!-- struct GLSLProfile::const NONE -->
no profile supported/available
<!-- struct GLSLProfile::const ES -->
OpenGL|ES profile
<!-- struct GLSLProfile::const CORE -->
OpenGL core profile
<!-- struct GLSLProfile::const COMPATIBILITY -->
OpenGL compatibility profile
<!-- struct GLSLProfile::const ANY -->
any OpenGL/OpenGL|ES profile
<!-- struct GLSLStage -->
`GLSLStage` holds and represents a single OpenGL shader stage.

# Implements

[`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- impl GLSLStage::fn new -->
## `context`
a `GLContext`
## `type_`
the GL enum shader stage type

# Returns

a new `GLSLStage` of the specified `type_`
<!-- impl GLSLStage::fn with_string -->
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
<!-- impl GLSLStage::fn with_strings -->
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

whether the compilation succeeded
<!-- impl GLSLStage::fn handle -->

# Returns

The GL handle for this shader stage
<!-- impl GLSLStage::fn profile -->

# Returns

The GLSL profile for the current shader stage
<!-- impl GLSLStage::fn shader_type -->

# Returns

The GL shader type for this shader stage
<!-- impl GLSLStage::fn version -->

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

[`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
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
<!-- impl GLShader::fn with_stages -->
Each stage will attempt to be compiled and attached to `shader`. On error,
`None` will be returned and `error` will contain the details of the error.

Note: must be called in the GL thread
## `context`
a `GLContext`
## `error`
a `glib::Error`

# Returns

a new `shader` with the specified stages.
<!-- impl GLShader::fn string_fragment_external_oes_get_default -->

Feature: `v1_16`

## `context`
a `GLContext`
## `version`
a `GLSLVersion`
## `profile`
a `GLSLProfile`

# Returns

a passthrough shader string for copying an input external-oes
 texture to the output
<!-- impl GLShader::fn string_fragment_get_default -->

Feature: `v1_16`

## `context`
a `GLContext`
## `version`
a `GLSLVersion`
## `profile`
a `GLSLProfile`

# Returns

a passthrough shader string for copying an input texture to
 the output
<!-- impl GLShader::fn string_get_highest_precision -->
Generates a shader string that defines the precision of float types in
GLSL shaders. This is particularly needed for fragment shaders in a
GLSL ES context where there is no default precision specified.

Practically, this will return the string 'precision mediump float'
or 'precision highp float' depending on if high precision floats are
determined to be supported.

Feature: `v1_16`

## `context`
a `GLContext`
## `version`
a `GLSLVersion`
## `profile`
a `GLSLProfile`

# Returns

a shader string defining the precision of float types based on
 `context`, `version` and `profile`
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
`glBindAttributeLocation()`.
## `index`
attribute index to set
## `name`
name of the attribute
<!-- impl GLShader::fn bind_frag_data_location -->
Bind attribute `name` to the specified location `index` using
`glBindFragDataLocation()`.
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
<!-- impl GLShader::fn program_handle -->

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
Perform `glUniform1f()` for `name` on `self`
## `name`
name of the uniform
## `value`
value to set
<!-- impl GLShader::fn set_uniform_1fv -->
Perform `glUniform1fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_1i -->
Perform `glUniform1i()` for `name` on `self`
## `name`
name of the uniform
## `value`
value to set
<!-- impl GLShader::fn set_uniform_1iv -->
Perform `glUniform1iv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_2f -->
Perform `glUniform2f()` for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
<!-- impl GLShader::fn set_uniform_2fv -->
Perform `glUniform2fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_2i -->
Perform `glUniform2i()` for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
<!-- impl GLShader::fn set_uniform_2iv -->
Perform `glUniform2iv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_3f -->
Perform `glUniform3f()` for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
## `v2`
third value to set
<!-- impl GLShader::fn set_uniform_3fv -->
Perform `glUniform3fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_3i -->
Perform `glUniform3i()` for `name` on `self`
## `name`
name of the uniform
## `v0`
first value to set
## `v1`
second value to set
## `v2`
third value to set
<!-- impl GLShader::fn set_uniform_3iv -->
Perform `glUniform3iv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_4f -->
Perform `glUniform4f()` for `name` on `self`
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
Perform `glUniform4fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_4i -->
Perform `glUniform4i()` for `name` on `self`
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
Perform `glUniform4iv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of values to set
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_2fv -->
Perform `glUniformMatrix2fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of 2x2 matrices to set
## `transpose`
transpose the matrix
## `value`
matrix to set
<!-- impl GLShader::fn set_uniform_matrix_2x3fv -->
Perform `glUniformMatrix2x3fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of 2x3 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_2x4fv -->
Perform `glUniformMatrix2x4fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of 2x4 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_3fv -->
Perform `glUniformMatrix3fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of 3x3 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_3x2fv -->
Perform `glUniformMatrix3x2fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of 3x2 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_3x4fv -->
Perform `glUniformMatrix3x4fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of 3x4 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_4fv -->
Perform `glUniformMatrix4fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of 4x4 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_4x2fv -->
Perform `glUniformMatrix4x2fv()` for `name` on `self`
## `name`
name of the uniform
## `count`
number of 4x2 matrices to set
## `transpose`
transpose the matrix
## `value`
values to set
<!-- impl GLShader::fn set_uniform_matrix_4x3fv -->
Perform `glUniformMatrix4x3fv()` for `name` on `self`
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
<!-- enum GLStereoDownmix::variant GreenMagentaDubois -->
Dubois optimised Green-Magenta anaglyph
<!-- enum GLStereoDownmix::variant RedCyanDubois -->
Dubois optimised Red-Cyan anaglyph
<!-- enum GLStereoDownmix::variant AmberBlueDubois -->
Dubois optimised Amber-Blue anaglyph
<!-- enum GLTextureTarget -->
The OpenGL texture target that an OpenGL texture can be bound to. The
`gst_gl_value_set_texture_target_from_mask`,
`gst_gl_value_get_texture_target_mask`, and
`gst_gl_value_set_texture_target` functions can be used for handling texture
targets with `glib::object::Value`'s when e.g. dealing with `gst::Caps`.
<!-- enum GLTextureTarget::variant None -->
no texture target
<!-- enum GLTextureTarget::variant 2d -->
2D texture target (`GL_TEXTURE_2D`)
<!-- enum GLTextureTarget::variant Rectangle -->
rectangle texture target
 (`GL_TEXTURE_RECTANGLE`)
<!-- enum GLTextureTarget::variant ExternalOes -->
external oes texture target
 (`GL_TEXTURE_EXTERNAL_OES`)
<!-- struct GLUpload -->
`GLUpload` is an object that uploads data from system memory into GL textures.

A `GLUpload` can be created with `GLUpload::new`

# Implements

[`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- impl GLUpload::fn new -->
## `context`
a `GLContext`

# Returns

a new `GLUpload` object
<!-- impl GLUpload::fn caps -->
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
An unspecified error occurred
<!-- enum GLUploadReturn::variant Unsupported -->
The configuration is unsupported.
<!-- enum GLUploadReturn::variant Reconfigure -->
This element requires a reconfiguration.
<!-- enum GLUploadReturn::variant UnsharedGlContext -->
private return value.
<!-- struct GLVideoAllocationParams -->
<!-- impl GLVideoAllocationParams::fn new -->
## `context`
a `GLContext`
## `alloc_params`
the `gst::AllocationParams` for sysmem mappings of the texture
## `v_info`
the `gst_video::VideoInfo` for the texture
## `plane`
the video plane of `v_info` to allocate
## `valign`
any `gst_video::VideoAlignment` applied to symem mappings of the texture
## `target`
the `GLTextureTarget` for the created textures
## `tex_format`
the `GLFormat` for the created textures

# Returns

a new `GLVideoAllocationParams` for allocating `GLMemory`'s
<!-- impl GLVideoAllocationParams::fn new_wrapped_data -->
## `context`
a `GLContext`
## `alloc_params`
the `gst::AllocationParams` for `wrapped_data`
## `v_info`
the `gst_video::VideoInfo` for `wrapped_data`
## `plane`
the video plane `wrapped_data` represents
## `valign`
any `gst_video::VideoAlignment` applied to symem mappings of `wrapped_data`
## `target`
the `GLTextureTarget` for `wrapped_data`
## `tex_format`
the `GLFormat` for `wrapped_data`
## `wrapped_data`
the data pointer to wrap
## `user_data`
user data to call `notify` with
## `notify`
a `GDestroyNotify`

# Returns

a new `GLVideoAllocationParams` for wrapping `wrapped_data`
<!-- impl GLVideoAllocationParams::fn new_wrapped_gl_handle -->
`gl_handle` is defined by the specific OpenGL handle being wrapped
For `GLMemory` and `GLMemoryPBO` it is an OpenGL texture id.
Other memory types may define it to require a different type of parameter.
## `context`
a `GLContext`
## `alloc_params`
the `gst::AllocationParams` for `tex_id`
## `v_info`
the `gst_video::VideoInfo` for `tex_id`
## `plane`
the video plane `tex_id` represents
## `valign`
any `gst_video::VideoAlignment` applied to symem mappings of `tex_id`
## `target`
the `GLTextureTarget` for `tex_id`
## `tex_format`
the `GLFormat` for `tex_id`
## `gl_handle`
the GL handle to wrap
## `user_data`
user data to call `notify` with
## `notify`
a `GDestroyNotify`

# Returns

a new `GLVideoAllocationParams` for wrapping `gl_handle`
<!-- impl GLVideoAllocationParams::fn new_wrapped_texture -->
## `context`
a `GLContext`
## `alloc_params`
the `gst::AllocationParams` for `tex_id`
## `v_info`
the `gst_video::VideoInfo` for `tex_id`
## `plane`
the video plane `tex_id` represents
## `valign`
any `gst_video::VideoAlignment` applied to symem mappings of `tex_id`
## `target`
the `GLTextureTarget` for `tex_id`
## `tex_format`
the `GLFormat` for `tex_id`
## `tex_id`
the GL texture to wrap
## `user_data`
user data to call `notify` with
## `notify`
a `GDestroyNotify`

# Returns

a new `GLVideoAllocationParams` for wrapping `tex_id`
<!-- impl GLVideoAllocationParams::fn copy_data -->
Copy and set any dynamically allocated resources in `dest_vid`. Intended
for subclass usage only to chain up at the end of a subclass copy function.
## `dest_vid`
destination `GLVideoAllocationParams` to copy into
<!-- impl GLVideoAllocationParams::fn free_data -->
Unset and free any dynamically allocated resources. Intended for subclass
usage only to chain up at the end of a subclass free function.
<!-- impl GLVideoAllocationParams::fn init_full -->
Intended for subclass usage
## `struct_size`
the size of the struct in `self`
## `alloc_flags`
some allocation flags
## `copy`
a copy function
## `free`
a free function
## `context`
a `GLContext`
## `alloc_params`
the `gst::AllocationParams` for `wrapped_data`
## `v_info`
the `gst_video::VideoInfo` for `wrapped_data`
## `plane`
the video plane `wrapped_data` represents
## `valign`
any `gst_video::VideoAlignment` applied to symem mappings of `wrapped_data`
## `target`
the `GLTextureTarget`
## `tex_format`
the `GLFormat`
## `wrapped_data`
the optional data pointer to wrap
## `gl_handle`
the optional OpenGL handle to wrap or 0
## `user_data`
user data to call `notify` with
## `notify`
a `GDestroyNotify`

# Returns

initializes `self` with the parameters specified
<!-- struct GLViewConvert -->
Convert stereoscopic/multiview video using fragment shaders.

# Implements

[`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- impl GLViewConvert::fn new -->

# Returns

a new `GLViewConvert`
<!-- impl GLViewConvert::fn fixate_caps -->
Provides an implementation of `gst_base::BaseTransformClass.fixate_caps`()
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
Provides an implementation of `gst_base::BaseTransformClass.transform_caps`()
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

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`GLWindowExt`](trait@crate::GLWindowExt), [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- trait GLWindowExt -->
Trait containing all `GLWindow` methods.

# Implementors

[`GLWindow`](struct@crate::GLWindow)
<!-- impl GLWindow::fn new -->
## `display`
a `GLDisplay`

# Returns

a new `GLWindow` using `display`'s connection
<!-- trait GLWindowExt::fn controls_viewport -->
Checks if `self` controls the GL viewport.

Feature: `v1_16`


# Returns

`true` if `self` controls the GL viewport, otherwise `false`
<!-- trait GLWindowExt::fn draw -->
Redraw the window contents. Implementations should invoke the draw callback.
<!-- trait GLWindowExt::fn context -->

# Returns

the `GLContext` associated with this `self`
<!-- trait GLWindowExt::fn get_display -->

# Returns

the windowing system display handle for this `self`
<!-- trait GLWindowExt::fn surface_dimensions -->
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
<!-- trait GLWindowExt::fn has_output_surface -->
Query whether `self` has output surface or not

Feature: `v1_18`


# Returns

`true` if `self` has useable output surface
<!-- trait GLWindowExt::fn queue_resize -->
Queue resizing of `self`.
<!-- trait GLWindowExt::fn quit -->
Quit the runloop's execution.
<!-- trait GLWindowExt::fn resize -->
Resize `self` to the given `width` and `height`.
## `width`
new width
## `height`
new height
<!-- trait GLWindowExt::fn run -->
Start the execution of the runloop.
<!-- trait GLWindowExt::fn send_message -->
Invoke `callback` with data on the window thread. `callback` is guaranteed to
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
<!-- trait GLWindowExt::fn send_scroll_event -->
Notify a `self` about a scroll event. A scroll signal holding the event
coordinates will be emitted.

Feature: `v1_18`

## `posx`
x position of the mouse cursor
## `posy`
y position of the mouse cursor
## `delta_x`
the x offset of the scroll event
## `delta_y`
the y offset of the scroll event
<!-- trait GLWindowExt::fn set_close_callback -->
Sets the callback called when the window is about to close.
## `callback`
function to invoke
## `data`
data to invoke `callback` with
## `destroy_notify`
called when `data` is not needed any more
<!-- trait GLWindowExt::fn set_draw_callback -->
Sets the draw callback called every time `GLWindowExt::draw` is called
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
Sets the resize callback called every time a resize of the window occurs.
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
<!-- trait GLWindowExt::fn connect_scroll_event -->
Will be emitted when a mouse scroll event is received by the GstGLwindow.

Feature: `v1_18`

## `x`
the x coordinate of the mouse event
## `y`
the y coordinate of the mouse event
## `delta_x`
the x offset of the scroll event
## `delta_y`
the y offset of the scroll event
<!-- enum GLWindowError -->
<!-- enum GLWindowError::variant Failed -->
failed for a unspecified reason
<!-- enum GLWindowError::variant OldLibs -->
the implementation is too old
<!-- enum GLWindowError::variant ResourceUnavailable -->
no such resource was found
