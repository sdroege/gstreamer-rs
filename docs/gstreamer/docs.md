<!-- file * -->
<!-- struct Bin -->
`Bin` is an element that can contain other `Element`, allowing them to be
managed as a group.
Pads from the child elements can be ghosted to the bin, see `GhostPad`.
This makes the bin look like any other elements and enables creation of
higher-level abstraction elements.

A new `Bin` is created with `Bin::new`. Use a `Pipeline` instead if you
want to create a toplevel bin because a normal bin doesn't have a bus or
handle clock distribution of its own.

After the bin has been created you will typically add elements to it with
`BinExt::add`. You can remove elements with `BinExt::remove`.

An element can be retrieved from a bin with `BinExt::get_by_name`, using the
elements name. `BinExt::get_by_name_recurse_up` is mainly used for internal
purposes and will query the parent bins when the element is not found in the
current bin.

An iterator of elements in a bin can be retrieved with
`BinExt::iterate_elements`. Various other iterators exist to retrieve the
elements in a bin.

`GstObjectExt::unref` is used to drop your reference to the bin.

The `Bin::element-added` signal is fired whenever a new element is added to
the bin. Likewise the `Bin::element-removed` signal is fired whenever an
element is removed from the bin.

## Notes

A `Bin` internally intercepts every `Message` posted by its children and
implements the following default behaviour for each of them:

* GST_MESSAGE_EOS: This message is only posted by sinks in the PLAYING
state. If all sinks posted the EOS message, this bin will post and EOS
message upwards.

* GST_MESSAGE_SEGMENT_START: Just collected and never forwarded upwards.
The messages are used to decide when all elements have completed playback
of their segment.

* GST_MESSAGE_SEGMENT_DONE: Is posted by `Bin` when all elements that posted
a SEGMENT_START have posted a SEGMENT_DONE.

* GST_MESSAGE_DURATION_CHANGED: Is posted by an element that detected a change
in the stream duration. The default bin behaviour is to clear any
cached duration values so that the next duration query will perform
a full duration recalculation. The duration change is posted to the
application so that it can refetch the new duration with a duration
query. Note that these messages can be posted before the bin is
prerolled, in which case the duration query might fail.

* GST_MESSAGE_CLOCK_LOST: This message is posted by an element when it
can no longer provide a clock. The default bin behaviour is to
check if the lost clock was the one provided by the bin. If so and
the bin is currently in the PLAYING state, the message is forwarded to
the bin parent.
This message is also generated when a clock provider is removed from
the bin. If this message is received by the application, it should
PAUSE the pipeline and set it back to PLAYING to force a new clock
distribution.

* GST_MESSAGE_CLOCK_PROVIDE: This message is generated when an element
can provide a clock. This mostly happens when a new clock
provider is added to the bin. The default behaviour of the bin is to
mark the currently selected clock as dirty, which will perform a clock
recalculation the next time the bin is asked to provide a clock.
This message is never sent tot the application but is forwarded to
the parent of the bin.

* OTHERS: posted upwards.

A `Bin` implements the following default behaviour for answering to a
`Query`:

* GST_QUERY_DURATION:If the query has been asked before with the same format
and the bin is a toplevel bin (ie. has no parent),
use the cached previous value. If no previous value was cached, the
query is sent to all sink elements in the bin and the MAXIMUM of all
values is returned. If the bin is a toplevel bin the value is cached.
If no sinks are available in the bin, the query fails.

* GST_QUERY_POSITION:The query is sent to all sink elements in the bin and the
MAXIMUM of all values is returned. If no sinks are available in the bin,
the query fails.

* OTHERS:the query is forwarded to all sink elements, the result
of the first sink that answers the query successfully is returned. If no
sink is in the bin, the query fails.

A `Bin` will by default forward any event sent to it to all sink
(`EventTypeFlags::Downstream`) or source (`EventTypeFlags::Upstream`) elements
depending on the event type.
If all the elements return `true`, the bin will also return `true`, else `false`
is returned. If no elements of the required type are in the bin, the event
handler will return `true`.

# Implements

[`BinExt`](trait.BinExt.html), [`ElementExt`](trait.ElementExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ChildProxyExt`](trait.ChildProxyExt.html)
<!-- trait BinExt -->
Trait containing all `Bin` methods.

# Implementors

[`Bin`](struct.Bin.html), [`Pipeline`](struct.Pipeline.html)
<!-- impl Bin::fn new -->
Creates a new bin with the given name.
## `name`
the name of the new bin

# Returns

a new `Bin`
<!-- trait BinExt::fn add -->
Adds the given element to the bin. Sets the element's parent, and thus
takes ownership of the element. An element can only be added to one bin.

If the element's pads are linked to other pads, the pads will be unlinked
before the element is added to the bin.

> When you add an element to an already-running pipeline, you will have to
> take care to set the state of the newly-added element to the desired
> state (usually PLAYING or PAUSED, same you set the pipeline to originally)
> with `ElementExt::set_state`, or use `ElementExt::sync_state_with_parent`.
> The bin or pipeline will not take care of this for you.

MT safe.
## `element`
the `Element` to add

# Returns

`true` if the element could be added, `false` if
the bin does not want to accept the element.
<!-- trait BinExt::fn add_many -->
Adds a `None`-terminated list of elements to a bin. This function is
equivalent to calling `BinExt::add` for each member of the list. The return
value of each `BinExt::add` is ignored.
## `element_1`
the `Element` element to add to the bin
<!-- trait BinExt::fn find_unlinked_pad -->
Recursively looks for elements with an unlinked pad of the given
direction within the specified bin and returns an unlinked pad
if one is found, or `None` otherwise. If a pad is found, the caller
owns a reference to it and should use `GstObjectExt::unref` on the
pad when it is not needed any longer.
## `direction`
whether to look for an unlinked source or sink pad

# Returns

unlinked pad of the given
direction, `None`.
<!-- trait BinExt::fn get_by_interface -->
Looks for an element inside the bin that implements the given
interface. If such an element is found, it returns the element.
You can cast this element to the given interface afterwards. If you want
all elements that implement the interface, use
`BinExt::iterate_all_by_interface`. This function recurses into child bins.

MT safe. Caller owns returned reference.
## `iface`
the `glib::Type` of an interface

# Returns

A `Element` inside the bin implementing the interface
<!-- trait BinExt::fn get_by_name -->
Gets the element with the given name from a bin. This
function recurses into child bins.

Returns `None` if no element with the given name is found in the bin.

MT safe. Caller owns returned reference.
## `name`
the element name to search for

# Returns

the `Element` with the given
name, or `None`
<!-- trait BinExt::fn get_by_name_recurse_up -->
Gets the element with the given name from this bin. If the
element is not found, a recursion is performed on the parent bin.

Returns `None` if:
- no element with the given name is found in the bin

MT safe. Caller owns returned reference.
## `name`
the element name to search for

# Returns

the `Element` with the given
name, or `None`
<!-- trait BinExt::fn get_suppressed_flags -->
Return the suppressed flags of the bin.

MT safe.

Feature: `v1_10`


# Returns

the bin's suppressed `ElementFlags`.
<!-- trait BinExt::fn iterate_all_by_interface -->
Looks for all elements inside the bin that implements the given
interface. You can safely cast all returned elements to the given interface.
The function recurses inside child bins. The iterator will yield a series
of `Element` that should be unreffed after use.

MT safe. Caller owns returned value.
## `iface`
the `glib::Type` of an interface

# Returns

a `Iterator` of `Element`
 for all elements in the bin implementing the given interface,
 or `None`
<!-- trait BinExt::fn iterate_elements -->
Gets an iterator for the elements in this bin.

MT safe. Caller owns returned value.

# Returns

a `Iterator` of `Element`,
or `None`
<!-- trait BinExt::fn iterate_recurse -->
Gets an iterator for the elements in this bin.
This iterator recurses into GstBin children.

MT safe. Caller owns returned value.

# Returns

a `Iterator` of `Element`,
or `None`
<!-- trait BinExt::fn iterate_sinks -->
Gets an iterator for all elements in the bin that have the
`ElementFlags::Sink` flag set.

MT safe. Caller owns returned value.

# Returns

a `Iterator` of `Element`,
or `None`
<!-- trait BinExt::fn iterate_sorted -->
Gets an iterator for the elements in this bin in topologically
sorted order. This means that the elements are returned from
the most downstream elements (sinks) to the sources.

This function is used internally to perform the state changes
of the bin elements and for clock selection.

MT safe. Caller owns returned value.

# Returns

a `Iterator` of `Element`,
or `None`
<!-- trait BinExt::fn iterate_sources -->
Gets an iterator for all elements in the bin that have the
`ElementFlags::Source` flag set.

MT safe. Caller owns returned value.

# Returns

a `Iterator` of `Element`,
or `None`
<!-- trait BinExt::fn recalculate_latency -->
Query `self` for the current latency using and reconfigures this latency to all the
elements with a LATENCY event.

This method is typically called on the pipeline when a `MessageType::Latency`
is posted on the bus.

This function simply emits the 'do-latency' signal so any custom latency
calculations will be performed.

# Returns

`true` if the latency could be queried and reconfigured.
<!-- trait BinExt::fn remove -->
Removes the element from the bin, unparenting it as well.
Unparenting the element means that the element will be dereferenced,
so if the bin holds the only reference to the element, the element
will be freed in the process of removing it from the bin. If you
want the element to still exist after removing, you need to call
`GstObjectExt::ref` before removing it from the bin.

If the element's pads are linked to other pads, the pads will be unlinked
before the element is removed from the bin.

MT safe.
## `element`
the `Element` to remove

# Returns

`true` if the element could be removed, `false` if
the bin does not want to remove the element.
<!-- trait BinExt::fn remove_many -->
Remove a list of elements from a bin. This function is equivalent
to calling `BinExt::remove` with each member of the list.
## `element_1`
the first `Element` to remove from the bin
<!-- trait BinExt::fn set_suppressed_flags -->
Suppress the given flags on the bin. `ElementFlags` of a
child element are propagated when it is added to the bin.
When suppressed flags are set, those specified flags will
not be propagated to the bin.

MT safe.

Feature: `v1_10`

## `flags`
the `ElementFlags` to suppress
<!-- trait BinExt::fn sync_children_states -->
Synchronizes the state of every child of `self` with the state
of `self`. See also `ElementExt::sync_state_with_parent`.

# Returns

`true` if syncing the state was successful for all children,
 otherwise `false`.
<!-- struct Buffer -->
Buffers are the basic unit of data transfer in GStreamer. They contain the
timing and offset along with other arbitrary metadata that is associated
with the `Memory` blocks that the buffer contains.

Buffers are usually created with `Buffer::new`. After a buffer has been
created one will typically allocate memory for it and add it to the buffer.
The following example creates a buffer that can hold a given video frame
with a given width, height and bits per plane.

```C
  GstBuffer *buffer;
  GstMemory *memory;
  gint size, width, height, bpp;
  ...
  size = width * height * bpp;
  buffer = gst_buffer_new ();
  memory = gst_allocator_alloc (NULL, size, NULL);
  gst_buffer_insert_memory (buffer, -1, memory);
  ...
```

Alternatively, use `Buffer::new_allocate` to create a buffer with
preallocated data of a given size.

Buffers can contain a list of `Memory` objects. You can retrieve how many
memory objects with `Buffer::n_memory` and you can get a pointer
to memory with `Buffer::peek_memory`

A buffer will usually have timestamps, and a duration, but neither of these
are guaranteed (they may be set to `GST_CLOCK_TIME_NONE`). Whenever a
meaningful value can be given for these, they should be set. The timestamps
and duration are measured in nanoseconds (they are `ClockTime` values).

The buffer DTS refers to the timestamp when the buffer should be decoded and
is usually monotonically increasing. The buffer PTS refers to the timestamp when
the buffer content should be presented to the user and is not always
monotonically increasing.

A buffer can also have one or both of a start and an end offset. These are
media-type specific. For video buffers, the start offset will generally be
the frame number. For audio buffers, it will be the number of samples
produced so far. For compressed data, it could be the byte offset in a
source or destination file. Likewise, the end offset will be the offset of
the end of the buffer. These can only be meaningfully interpreted if you
know the media type of the buffer (the preceding CAPS event). Either or both
can be set to `GST_BUFFER_OFFSET_NONE`.

`gst_buffer_ref` is used to increase the refcount of a buffer. This must be
done when you want to keep a handle to the buffer after pushing it to the
next element. The buffer refcount determines the writability of the buffer, a
buffer is only writable when the refcount is exactly 1, i.e. when the caller
has the only reference to the buffer.

To efficiently create a smaller buffer out of an existing one, you can
use `Buffer::copy_region`. This method tries to share the memory objects
between the two buffers.

If a plug-in wants to modify the buffer data or metadata in-place, it should
first obtain a buffer that is safe to modify by using
`gst_buffer_make_writable`. This function is optimized so that a copy will
only be made when it is necessary.

Several flags of the buffer can be set and unset with the
GST_BUFFER_FLAG_SET() and GST_BUFFER_FLAG_UNSET() macros. Use
GST_BUFFER_FLAG_IS_SET() to test if a certain `BufferFlags` flag is set.

Buffers can be efficiently merged into a larger buffer with
`Buffer::append`. Copying of memory will only be done when absolutely
needed.

Arbitrary extra metadata can be set on a buffer with `Buffer::add_meta`.
Metadata can be retrieved with `Buffer::get_meta`. See also `Meta`

An element should either unref the buffer or push it out on a src pad
using `Pad::push` (see `Pad`).

Buffers are usually freed by unreffing them with `gst_buffer_unref`. When
the refcount drops to 0, any memory and metadata pointed to by the buffer is
unreffed as well. Buffers allocated from a `BufferPool` will be returned to
the pool when the refcount drops to 0.

The `ParentBufferMeta` is a meta which can be attached to a `Buffer`
to hold a reference to another buffer that is only released when the child
`Buffer` is released.

Typically, `ParentBufferMeta` is used when the child buffer is directly
using the `Memory` of the parent buffer, and wants to prevent the parent
buffer from being returned to a buffer pool until the `Memory` is available
for re-use. (Since 1.6)
<!-- impl Buffer::fn new -->
Creates a newly allocated buffer without any data.

MT safe.

# Returns

the new `Buffer`.
<!-- impl Buffer::fn new_allocate -->
Tries to create a newly allocated buffer with data of the given size and
extra parameters from `allocator`. If the requested amount of memory can't be
allocated, `None` will be returned. The allocated buffer memory is not cleared.

When `allocator` is `None`, the default memory allocator will be used.

Note that when `size` == 0, the buffer will not have memory associated with it.

MT safe.
## `allocator`
the `Allocator` to use, or `None` to use the
 default allocator
## `size`
the size in bytes of the new buffer's data.
## `params`
optional parameters

# Returns

a new `Buffer`, or `None` if
 the memory couldn't be allocated.
<!-- impl Buffer::fn new_wrapped -->
Creates a new buffer that wraps the given `data`. The memory will be freed
with g_free and will be marked writable.

MT safe.
## `data`
data to wrap
## `size`
allocated size of `data`

# Returns

a new `Buffer`
<!-- impl Buffer::fn new_wrapped_full -->
Allocate a new buffer that wraps the given memory. `data` must point to
`maxsize` of memory, the wrapped buffer will have the region from `offset` and
`size` visible.

When the buffer is destroyed, `notify` will be called with `user_data`.

The prefix/padding must be filled with 0 if `flags` contains
`MemoryFlags::ZeroPrefixed` and `MemoryFlags::ZeroPadded` respectively.
## `flags`
`MemoryFlags`
## `data`
data to wrap
## `maxsize`
allocated size of `data`
## `offset`
offset in `data`
## `size`
size of valid data
## `user_data`
user_data
## `notify`
called with `user_data` when the memory is freed

# Returns

a new `Buffer`
<!-- impl Buffer::fn add_meta -->
Add metadata for `info` to `self` using the parameters in `params`.
## `info`
a `MetaInfo`
## `params`
params for `info`

# Returns

the metadata for the api in `info` on `self`.
<!-- impl Buffer::fn add_parent_buffer_meta -->
Add a `ParentBufferMeta` to `self` that holds a reference on
`ref_` until the buffer is freed.
## `ref_`
a `Buffer` to ref

# Returns

The `ParentBufferMeta` that was added to the buffer
<!-- impl Buffer::fn add_protection_meta -->
Attaches protection metadata to a `Buffer`.
## `info`
a `Structure` holding cryptographic
 information relating to the sample contained in `self`. This
 function takes ownership of `info`.

# Returns

a pointer to the added `ProtectionMeta` if successful; `None` if
unsuccessful.
<!-- impl Buffer::fn append -->
Append all the memory from `buf2` to `self`. The result buffer will contain a
concatenation of the memory of `self` and `buf2`.
## `buf2`
the second source `Buffer` to append.

# Returns

the new `Buffer` that contains the memory
 of the two source buffers.
<!-- impl Buffer::fn append_memory -->
Append the memory block `mem` to `self`. This function takes
ownership of `mem` and thus doesn't increase its refcount.

This function is identical to `Buffer::insert_memory` with an index of -1.
See `Buffer::insert_memory` for more details.
## `mem`
a `Memory`.
<!-- impl Buffer::fn append_region -->
Append `size` bytes at `offset` from `buf2` to `self`. The result buffer will
contain a concatenation of the memory of `self` and the requested region of
`buf2`.
## `buf2`
the second source `Buffer` to append.
## `offset`
the offset in `buf2`
## `size`
the size or -1 of `buf2`

# Returns

the new `Buffer` that contains the memory
 of the two source buffers.
<!-- impl Buffer::fn copy_deep -->
Create a copy of the given buffer. This will make a newly allocated
copy of the data the source buffer contains.

# Returns

a new copy of `self`.
<!-- impl Buffer::fn copy_into -->
Copies the information from `src` into `self`.

If `self` already contains memory and `flags` contains GST_BUFFER_COPY_MEMORY,
the memory from `src` will be appended to `self`.

`flags` indicate which fields will be copied.
## `src`
a source `Buffer`
## `flags`
flags indicating what metadata fields should be copied.
## `offset`
offset to copy from
## `size`
total size to copy. If -1, all data is copied.

# Returns

`true` if the copying succeeded, `false` otherwise.
<!-- impl Buffer::fn copy_region -->
Creates a sub-buffer from `self` at `offset` and `size`.
This sub-buffer uses the actual memory space of the parent buffer.
This function will copy the offset and timestamp fields when the
offset is 0. If not, they will be set to `GST_CLOCK_TIME_NONE` and
`GST_BUFFER_OFFSET_NONE`.
If `offset` equals 0 and `size` equals the total size of `buffer`, the
duration and offset end fields are also copied. If not they will be set
to `GST_CLOCK_TIME_NONE` and `GST_BUFFER_OFFSET_NONE`.

MT safe.
## `flags`
the `BufferCopyFlags`
## `offset`
the offset into parent `Buffer` at which the new sub-buffer
 begins.
## `size`
the size of the new `Buffer` sub-buffer, in bytes. If -1, all
 data is copied.

# Returns

the new `Buffer` or `None` if the arguments were
 invalid.
<!-- impl Buffer::fn extract -->
Copy `size` bytes starting from `offset` in `self` to `dest`.
## `offset`
the offset to extract
## `dest`
the destination address
## `size`
the size to extract

# Returns

The amount of bytes extracted. This value can be lower than `size`
 when `self` did not contain enough data.
<!-- impl Buffer::fn extract_dup -->
Extracts a copy of at most `size` bytes the data at `offset` into
newly-allocated memory. `dest` must be freed using `g_free` when done.
## `offset`
the offset to extract
## `size`
the size to extract
## `dest`
A pointer where
 the destination array will be written.
## `dest_size`
A location where the size of `dest` can be written
<!-- impl Buffer::fn fill -->
Copy `size` bytes from `src` to `self` at `offset`.
## `offset`
the offset to fill
## `src`
the source address
## `size`
the size to fill

# Returns

The amount of bytes copied. This value can be lower than `size`
 when `self` did not contain enough data.
<!-- impl Buffer::fn find_memory -->
Find the memory blocks that span `size` bytes starting from `offset`
in `self`.

When this function returns `true`, `idx` will contain the index of the first
memory block where the byte for `offset` can be found and `length` contains the
number of memory blocks containing the `size` remaining bytes. `skip` contains
the number of bytes to skip in the memory block at `idx` to get to the byte
for `offset`.

`size` can be -1 to get all the memory blocks after `idx`.
## `offset`
an offset
## `size`
a size
## `idx`
pointer to index
## `length`
pointer to length
## `skip`
pointer to skip

# Returns

`true` when `size` bytes starting from `offset` could be found in
`self` and `idx`, `length` and `skip` will be filled.
<!-- impl Buffer::fn foreach_meta -->
Call `func` with `user_data` for each meta in `self`.

`func` can modify the passed meta pointer or its contents. The return value
of `func` define if this function returns or if the remaining metadata items
in the buffer should be skipped.
## `func`
a `GstBufferForeachMetaFunc` to call
## `user_data`
user data passed to `func`

# Returns

`false` when `func` returned `false` for one of the metadata.
<!-- impl Buffer::fn get_all_memory -->
Get all the memory block in `self`. The memory blocks will be merged
into one large `Memory`.

# Returns

a `Memory` that contains the merged memory.
Use gst_memory_unref () after usage.
<!-- impl Buffer::fn get_flags -->
Get the `BufferFlags` flags set on this buffer.

Feature: `v1_10`


# Returns

the flags set on this buffer.
<!-- impl Buffer::fn get_memory -->
Get the memory block at index `idx` in `self`.
## `idx`
an index

# Returns

a `Memory` that contains the data of the
memory block at `idx`. Use gst_memory_unref () after usage.
<!-- impl Buffer::fn get_memory_range -->
Get `length` memory blocks in `self` starting at `idx`. The memory blocks will
be merged into one large `Memory`.

If `length` is -1, all memory starting from `idx` is merged.
## `idx`
an index
## `length`
a length

# Returns

a `Memory` that contains the merged data of `length`
 blocks starting at `idx`. Use gst_memory_unref () after usage.
<!-- impl Buffer::fn get_meta -->
Get the metadata for `api` on buffer. When there is no such metadata, `None` is
returned. If multiple metadata with the given `api` are attached to this
buffer only the first one is returned. To handle multiple metadata with a
given API use `Buffer::iterate_meta` or `Buffer::foreach_meta` instead
and check the meta->info.api member for the API type.
## `api`
the `glib::Type` of an API

# Returns

the metadata for `api` on
`self`.
<!-- impl Buffer::fn get_size -->
Get the total size of the memory blocks in `self`.

# Returns

total size of the memory blocks in `self`.
<!-- impl Buffer::fn get_sizes -->
Get the total size of the memory blocks in `b`.

When not `None`, `offset` will contain the offset of the data in the
first memory block in `self` and `maxsize` will contain the sum of
the size and `offset` and the amount of extra padding on the last
memory block. `offset` and `maxsize` can be used to resize the
buffer memory blocks with `Buffer::resize`.
## `offset`
a pointer to the offset
## `maxsize`
a pointer to the maxsize

# Returns

total size of the memory blocks in `self`.
<!-- impl Buffer::fn get_sizes_range -->
Get the total size of `length` memory blocks stating from `idx` in `self`.

When not `None`, `offset` will contain the offset of the data in the
memory block in `self` at `idx` and `maxsize` will contain the sum of the size
and `offset` and the amount of extra padding on the memory block at `idx` +
`length` -1.
`offset` and `maxsize` can be used to resize the buffer memory blocks with
`Buffer::resize_range`.
## `idx`
an index
## `length`
a length
## `offset`
a pointer to the offset
## `maxsize`
a pointer to the maxsize

# Returns

total size of `length` memory blocks starting at `idx` in `self`.
<!-- impl Buffer::fn insert_memory -->
Insert the memory block `mem` to `self` at `idx`. This function takes ownership
of `mem` and thus doesn't increase its refcount.

Only `Buffer::get_max_memory` can be added to a buffer. If more memory is
added, existing memory blocks will automatically be merged to make room for
the new memory.
## `idx`
the index to add the memory at, or -1 to append it to the end
## `mem`
a `Memory`.
<!-- impl Buffer::fn is_all_memory_writable -->
Check if all memory blocks in `self` are writable.

Note that this function does not check if `self` is writable, use
`gst_buffer_is_writable` to check that if needed.

# Returns

`true` if all memory blocks in `self` are writable
<!-- impl Buffer::fn is_memory_range_writable -->
Check if `length` memory blocks in `self` starting from `idx` are writable.

`length` can be -1 to check all the memory blocks after `idx`.

Note that this function does not check if `self` is writable, use
`gst_buffer_is_writable` to check that if needed.
## `idx`
an index
## `length`
a length should not be 0

# Returns

`true` if the memory range is writable
<!-- impl Buffer::fn iterate_meta -->
Retrieve the next `Meta` after `current`. If `state` points
to `None`, the first metadata is returned.

`state` will be updated with an opaque state pointer
## `state`
an opaque state pointer

# Returns

The next `Meta` or `None`
when there are no more items.
<!-- impl Buffer::fn iterate_meta_filtered -->
Retrieve the next `Meta` of type `meta_api_type` after the current one
according to `state`. If `state` points to `None`, the first metadata of
type `meta_api_type` is returned.

`state` will be updated with an opaque state pointer

Feature: `v1_12`

## `state`
an opaque state pointer
## `meta_api_type`
only return `Meta` of this type

# Returns

The next `Meta` of type
`meta_api_type` or `None` when there are no more items.
<!-- impl Buffer::fn map -->
This function fills `info` with the `MapInfo` of all merged memory
blocks in `self`.

`flags` describe the desired access of the memory. When `flags` is
`MapFlags::Write`, `self` should be writable (as returned from
`gst_buffer_is_writable`).

When `self` is writable but the memory isn't, a writable copy will
automatically be created and returned. The readonly copy of the
buffer memory will then also be replaced with this writable copy.

The memory in `info` should be unmapped with `Buffer::unmap` after
usage.
## `info`
info about the mapping
## `flags`
flags for the mapping

# Returns

`true` if the map succeeded and `info` contains valid data.
<!-- impl Buffer::fn map_range -->
This function fills `info` with the `MapInfo` of `length` merged memory blocks
starting at `idx` in `self`. When `length` is -1, all memory blocks starting
from `idx` are merged and mapped.

`flags` describe the desired access of the memory. When `flags` is
`MapFlags::Write`, `self` should be writable (as returned from
`gst_buffer_is_writable`).

When `self` is writable but the memory isn't, a writable copy will
automatically be created and returned. The readonly copy of the buffer memory
will then also be replaced with this writable copy.

The memory in `info` should be unmapped with `Buffer::unmap` after usage.
## `idx`
an index
## `length`
a length
## `info`
info about the mapping
## `flags`
flags for the mapping

# Returns

`true` if the map succeeded and `info` contains valid
data.
<!-- impl Buffer::fn memcmp -->
Compare `size` bytes starting from `offset` in `self` with the memory in `mem`.
## `offset`
the offset in `self`
## `mem`
the memory to compare
## `size`
the size to compare

# Returns

0 if the memory is equal.
<!-- impl Buffer::fn memset -->
Fill `buf` with `size` bytes with `val` starting from `offset`.
## `offset`
the offset in `self`
## `val`
the value to set
## `size`
the size to set

# Returns

The amount of bytes filled. This value can be lower than `size`
 when `self` did not contain enough data.
<!-- impl Buffer::fn n_memory -->
Get the amount of memory blocks that this buffer has. This amount is never
larger than what `Buffer::get_max_memory` returns.

# Returns

the number of memory blocks this buffer is made of.
<!-- impl Buffer::fn peek_memory -->
Get the memory block at `idx` in `self`. The memory block stays valid until
the memory block in `self` is removed, replaced or merged, typically with
any call that modifies the memory in `self`.
## `idx`
an index

# Returns

the `Memory` at `idx`.
<!-- impl Buffer::fn prepend_memory -->
Prepend the memory block `mem` to `self`. This function takes
ownership of `mem` and thus doesn't increase its refcount.

This function is identical to `Buffer::insert_memory` with an index of 0.
See `Buffer::insert_memory` for more details.
## `mem`
a `Memory`.
<!-- impl Buffer::fn remove_all_memory -->
Remove all the memory blocks in `self`.
<!-- impl Buffer::fn remove_memory -->
Remove the memory block in `b` at index `i`.
## `idx`
an index
<!-- impl Buffer::fn remove_memory_range -->
Remove `length` memory blocks in `self` starting from `idx`.

`length` can be -1, in which case all memory starting from `idx` is removed.
## `idx`
an index
## `length`
a length
<!-- impl Buffer::fn remove_meta -->
Remove the metadata for `meta` on `self`.
## `meta`
a `Meta`

# Returns

`true` if the metadata existed and was removed, `false` if no such
metadata was on `self`.
<!-- impl Buffer::fn replace_all_memory -->
Replaces all memory in `self` with `mem`.
## `mem`
a `Memory`
<!-- impl Buffer::fn replace_memory -->
Replaces the memory block at index `idx` in `self` with `mem`.
## `idx`
an index
## `mem`
a `Memory`
<!-- impl Buffer::fn replace_memory_range -->
Replaces `length` memory blocks in `self` starting at `idx` with `mem`.

If `length` is -1, all memory starting from `idx` will be removed and
replaced with `mem`.

`self` should be writable.
## `idx`
an index
## `length`
a length should not be 0
## `mem`
a `Memory`
<!-- impl Buffer::fn resize -->
Set the offset and total size of the memory blocks in `self`.
## `offset`
the offset adjustment
## `size`
the new size or -1 to just adjust the offset
<!-- impl Buffer::fn resize_range -->
Set the total size of the `length` memory blocks starting at `idx` in
`self`
## `idx`
an index
## `length`
a length
## `offset`
the offset adjustment
## `size`
the new size or -1 to just adjust the offset

# Returns

`true` if resizing succeeded, `false` otherwise.
<!-- impl Buffer::fn set_flags -->
Sets one or more buffer flags on a buffer.

Feature: `v1_10`

## `flags`
the `BufferFlags` to set.

# Returns

`true` if `flags` were successfully set on buffer.
<!-- impl Buffer::fn set_size -->
Set the total size of the memory blocks in `self`.
## `size`
the new size
<!-- impl Buffer::fn unmap -->
Release the memory previously mapped with `Buffer::map`.
## `info`
a `MapInfo`
<!-- impl Buffer::fn unset_flags -->
Clears one or more buffer flags.

Feature: `v1_10`

## `flags`
the `BufferFlags` to clear

# Returns

true if `flags` is successfully cleared from buffer.
<!-- impl Buffer::fn get_max_memory -->
Get the maximum amount of memory blocks that a buffer can hold. This is a
compile time constant that can be queried with the function.

When more memory blocks are added, existing memory blocks will be merged
together to make room for the new block.

# Returns

the maximum amount of memory blocks that a buffer can hold.
<!-- enum BufferingMode -->
The different types of buffering methods.
<!-- enum BufferingMode::variant Stream -->
a small amount of data is buffered
<!-- enum BufferingMode::variant Download -->
the stream is being downloaded
<!-- enum BufferingMode::variant Timeshift -->
the stream is being downloaded in a ringbuffer
<!-- enum BufferingMode::variant Live -->
the stream is a live stream
<!-- struct Bus -->
The `Bus` is an object responsible for delivering `Message` packets in
a first-in first-out way from the streaming threads (see `Task`) to the
application.

Since the application typically only wants to deal with delivery of these
messages from one thread, the GstBus will marshall the messages between
different threads. This is important since the actual streaming of media
is done in another thread than the application.

The GstBus provides support for `glib::Source` based notifications. This makes it
possible to handle the delivery in the glib mainloop.

The `glib::Source` callback function `Bus::async_signal_func` can be used to
convert all bus messages into signal emissions.

A message is posted on the bus with the `Bus::post` method. With the
`Bus::peek` and `Bus::pop` methods one can look at or retrieve a
previously posted message.

The bus can be polled with the `Bus::poll` method. This methods blocks
up to the specified timeout value until one of the specified messages types
is posted on the bus. The application can then `Bus::pop` the messages
from the bus to handle them.
Alternatively the application can register an asynchronous bus function
using `Bus::add_watch_full` or `Bus::add_watch`. This function will
install a `glib::Source` in the default glib main loop and will deliver messages
a short while after they have been posted. Note that the main loop should
be running for the asynchronous callbacks.

It is also possible to get messages from the bus without any thread
marshalling with the `Bus::set_sync_handler` method. This makes it
possible to react to a message in the same thread that posted the
message on the bus. This should only be used if the application is able
to deal with messages from different threads.

Every `Pipeline` has one bus.

Note that a `Pipeline` will set its bus into flushing state when changing
from READY to NULL state.

# Implements

[`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- impl Bus::fn new -->
Creates a new `Bus` instance.

# Returns

a new `Bus` instance
<!-- impl Bus::fn add_signal_watch -->
Adds a bus signal watch to the default main context with the default priority
(`G_PRIORITY_DEFAULT`). It is also possible to use a non-default
main context set up using `glib::MainContext::push_thread_default` (before
one had to create a bus watch source and attach it to the desired main
context 'manually').

After calling this statement, the bus will emit the "message" signal for each
message posted on the bus.

This function may be called multiple times. To clean up, the caller is
responsible for calling `Bus::remove_signal_watch` as many times as this
function is called.

MT safe.
<!-- impl Bus::fn add_signal_watch_full -->
Adds a bus signal watch to the default main context with the given `priority`
(e.g. `G_PRIORITY_DEFAULT`). It is also possible to use a non-default main
context set up using `glib::MainContext::push_thread_default`
(before one had to create a bus watch source and attach it to the desired
main context 'manually').

After calling this statement, the bus will emit the "message" signal for each
message posted on the bus when the main loop is running.

This function may be called multiple times. To clean up, the caller is
responsible for calling `Bus::remove_signal_watch` as many times as this
function is called.

There can only be a single bus watch per bus, you must remove any signal
watch before you can set another type of watch.

MT safe.
## `priority`
The priority of the watch.
<!-- impl Bus::fn add_watch -->
Adds a bus watch to the default main context with the default priority
(`G_PRIORITY_DEFAULT`). It is also possible to use a non-default main
context set up using `glib::MainContext::push_thread_default` (before
one had to create a bus watch source and attach it to the desired main
context 'manually').

This function is used to receive asynchronous messages in the main loop.
There can only be a single bus watch per bus, you must remove it before you
can set a new one.

The bus watch will only work if a GLib main loop is being run.

The watch can be removed using `Bus::remove_watch` or by returning `false`
from `func`. If the watch was added to the default main context it is also
possible to remove the watch using `glib::Source::remove`.
## `func`
A function to call when a message is received.
## `user_data`
user data passed to `func`.

# Returns

The event source id or 0 if `self` already got an event source.

MT safe.
<!-- impl Bus::fn add_watch_full -->
Adds a bus watch to the default main context with the given `priority` (e.g.
`G_PRIORITY_DEFAULT`). It is also possible to use a non-default main
context set up using `glib::MainContext::push_thread_default` (before
one had to create a bus watch source and attach it to the desired main
context 'manually').

This function is used to receive asynchronous messages in the main loop.
There can only be a single bus watch per bus, you must remove it before you
can set a new one.

The bus watch will only work if a GLib main loop is being run.

When `func` is called, the message belongs to the caller; if you want to
keep a copy of it, call `gst_message_ref` before leaving `func`.

The watch can be removed using `Bus::remove_watch` or by returning `false`
from `func`. If the watch was added to the default main context it is also
possible to remove the watch using `glib::Source::remove`.

MT safe.
## `priority`
The priority of the watch.
## `func`
A function to call when a message is received.
## `user_data`
user data passed to `func`.
## `notify`
the function to call when the source is removed.

# Returns

The event source id or 0 if `self` already got an event source.
<!-- impl Bus::fn async_signal_func -->
A helper `GstBusFunc` that can be used to convert all asynchronous messages
into signals.
## `message`
the `Message` received
## `data`
user data

# Returns

`true`
<!-- impl Bus::fn create_watch -->
Create watch for this bus. The GSource will be dispatched whenever
a message is on the bus. After the GSource is dispatched, the
message is popped off the bus and unreffed.

# Returns

a `glib::Source` that can be added to a mainloop.
<!-- impl Bus::fn disable_sync_message_emission -->
Instructs GStreamer to stop emitting the "sync-message" signal for this bus.
See `Bus::enable_sync_message_emission` for more information.

In the event that multiple pieces of code have called
`Bus::enable_sync_message_emission`, the sync-message emissions will only
be stopped after all calls to `Bus::enable_sync_message_emission` were
"cancelled" by calling this function. In this way the semantics are exactly
the same as `GstObjectExt::ref` that which calls enable should also call
disable.

MT safe.
<!-- impl Bus::fn enable_sync_message_emission -->
Instructs GStreamer to emit the "sync-message" signal after running the bus's
sync handler. This function is here so that code can ensure that they can
synchronously receive messages without having to affect what the bin's sync
handler is.

This function may be called multiple times. To clean up, the caller is
responsible for calling `Bus::disable_sync_message_emission` as many times
as this function is called.

While this function looks similar to `Bus::add_signal_watch`, it is not
exactly the same -- this function enables `<emphasis>`synchronous`</emphasis>` emission of
signals when messages arrive; `Bus::add_signal_watch` adds an idle callback
to pop messages off the bus `<emphasis>`asynchronously`</emphasis>`. The sync-message signal
comes from the thread of whatever object posted the message; the "message"
signal is marshalled to the main thread via the main loop.

MT safe.
<!-- impl Bus::fn have_pending -->
Check if there are pending messages on the bus that
should be handled.

# Returns

`true` if there are messages on the bus to be handled, `false`
otherwise.

MT safe.
<!-- impl Bus::fn peek -->
Peek the message on the top of the bus' queue. The message will remain
on the bus' message queue. A reference is returned, and needs to be unreffed
by the caller.

# Returns

the `Message` that is on the
 bus, or `None` if the bus is empty.

MT safe.
<!-- impl Bus::fn poll -->
Poll the bus for messages. Will block while waiting for messages to come.
You can specify a maximum time to poll with the `timeout` parameter. If
`timeout` is negative, this function will block indefinitely.

All messages not in `events` will be popped off the bus and will be ignored.
It is not possible to use message enums beyond `MessageType::Extended` in the
`events` mask

Because poll is implemented using the "message" signal enabled by
`Bus::add_signal_watch`, calling `Bus::poll` will cause the "message"
signal to be emitted for every message that poll sees. Thus a "message"
signal handler will see the same messages that this function sees -- neither
will steal messages from the other.

This function will run a main loop from the default main context when
polling.

You should never use this function, since it is pure evil. This is
especially true for GUI applications based on Gtk+ or Qt, but also for any
other non-trivial application that uses the GLib main loop. As this function
runs a GLib main loop, any callback attached to the default GLib main
context may be invoked. This could be timeouts, GUI events, I/O events etc.;
even if `Bus::poll` is called with a 0 timeout. Any of these callbacks
may do things you do not expect, e.g. destroy the main application window or
some other resource; change other application state; display a dialog and
run another main loop until the user clicks it away. In short, using this
function may add a lot of complexity to your code through unexpected
re-entrancy and unexpected changes to your application's state.

For 0 timeouts use `Bus::pop_filtered` instead of this function; for
other short timeouts use `Bus::timed_pop_filtered`; everything else is
better handled by setting up an asynchronous bus watch and doing things
from there.
## `events`
a mask of `MessageType`, representing the set of message types to
poll for (note special handling of extended message types below)
## `timeout`
the poll timeout, as a `ClockTime`, or `GST_CLOCK_TIME_NONE` to poll
indefinitely.

# Returns

the message that was received,
 or `None` if the poll timed out. The message is taken from the
 bus and needs to be unreffed with `gst_message_unref` after
 usage.
<!-- impl Bus::fn pop -->
Get a message from the bus.

# Returns

the `Message` that is on the
 bus, or `None` if the bus is empty. The message is taken from
 the bus and needs to be unreffed with `gst_message_unref` after
 usage.

MT safe.
<!-- impl Bus::fn pop_filtered -->
Get a message matching `type_` from the bus. Will discard all messages on
the bus that do not match `type_` and that have been posted before the first
message that does match `type_`. If there is no message matching `type_` on
the bus, all messages will be discarded. It is not possible to use message
enums beyond `MessageType::Extended` in the `events` mask.
## `types`
message types to take into account

# Returns

the next `Message` matching
 `type_` that is on the bus, or `None` if the bus is empty or there
 is no message matching `type_`. The message is taken from the bus
 and needs to be unreffed with `gst_message_unref` after usage.

MT safe.
<!-- impl Bus::fn post -->
Post a message on the given bus. Ownership of the message
is taken by the bus.
## `message`
the `Message` to post

# Returns

`true` if the message could be posted, `false` if the bus is flushing.

MT safe.
<!-- impl Bus::fn remove_signal_watch -->
Removes a signal watch previously added with `Bus::add_signal_watch`.

MT safe.
<!-- impl Bus::fn remove_watch -->
Removes an installed bus watch from `self`.

# Returns

`true` on success or `false` if `self` has no event source.
<!-- impl Bus::fn set_flushing -->
If `flushing`, flush out and unref any messages queued in the bus. Releases
references to the message origin objects. Will flush future messages until
`Bus::set_flushing` sets `flushing` to `false`.

MT safe.
## `flushing`
whether or not to flush the bus
<!-- impl Bus::fn set_sync_handler -->
Sets the synchronous handler on the bus. The function will be called
every time a new message is posted on the bus. Note that the function
will be called in the same thread context as the posting object. This
function is usually only called by the creator of the bus. Applications
should handle messages asynchronously using the gst_bus watch and poll
functions.

You cannot replace an existing sync_handler. You can pass `None` to this
function, which will clear the existing handler.
## `func`
The handler function to install
## `user_data`
User data that will be sent to the handler function.
## `notify`
called when `user_data` becomes unused
<!-- impl Bus::fn sync_signal_handler -->
A helper GstBusSyncHandler that can be used to convert all synchronous
messages into signals.
## `message`
the `Message` received
## `data`
user data

# Returns

GST_BUS_PASS
<!-- impl Bus::fn timed_pop -->
Get a message from the bus, waiting up to the specified timeout.

If `timeout` is 0, this function behaves like `Bus::pop`. If `timeout` is
`GST_CLOCK_TIME_NONE`, this function will block forever until a message was
posted on the bus.
## `timeout`
a timeout

# Returns

the `Message` that is on the
 bus after the specified timeout or `None` if the bus is empty
 after the timeout expired. The message is taken from the bus
 and needs to be unreffed with `gst_message_unref` after usage.

MT safe.
<!-- impl Bus::fn timed_pop_filtered -->
Get a message from the bus whose type matches the message type mask `types`,
waiting up to the specified timeout (and discarding any messages that do not
match the mask provided).

If `timeout` is 0, this function behaves like `Bus::pop_filtered`. If
`timeout` is `GST_CLOCK_TIME_NONE`, this function will block forever until a
matching message was posted on the bus.
## `timeout`
a timeout in nanoseconds, or GST_CLOCK_TIME_NONE to wait forever
## `types`
message types to take into account, GST_MESSAGE_ANY for any type

# Returns

a `Message` matching the
 filter in `types`, or `None` if no matching message was found on
 the bus until the timeout expired. The message is taken from
 the bus and needs to be unreffed with `gst_message_unref` after
 usage.

MT safe.
<!-- enum BusSyncReply -->
The result values for a GstBusSyncHandler.
<!-- enum BusSyncReply::variant Drop -->
drop the message
<!-- enum BusSyncReply::variant Pass -->
pass the message to the async queue
<!-- enum BusSyncReply::variant Async -->
pass message to async queue, continue if message is handled
<!-- struct Caps -->
Caps (capabilities) are lightweight refcounted objects describing media types.
They are composed of an array of `Structure`.

Caps are exposed on `PadTemplate` to describe all possible types a
given pad can handle. They are also stored in the `Registry` along with
a description of the `Element`.

Caps are exposed on the element pads using the `PadExt::query_caps` pad
function. This function describes the possible types that the pad can
handle or produce at runtime.

A `Caps` can be constructed with the following code fragment:

```C
  GstCaps *caps = gst_caps_new_simple ("video/x-raw",
     "format", G_TYPE_STRING, "I420",
     "framerate", GST_TYPE_FRACTION, 25, 1,
     "pixel-aspect-ratio", GST_TYPE_FRACTION, 1, 1,
     "width", G_TYPE_INT, 320,
     "height", G_TYPE_INT, 240,
     NULL);
```

A `Caps` is fixed when it has no properties with ranges or lists. Use
`Caps::is_fixed` to test for fixed caps. Fixed caps can be used in a
caps event to notify downstream elements of the current media type.

Various methods exist to work with the media types such as subtracting
or intersecting.

Be aware that the current `Caps` / `Structure` serialization into string
has limited support for nested `Caps` / `Structure` fields. It can only
support one level of nesting. Using more levels will lead to unexpected
behavior when using serialization features, such as `Caps::to_string` or
`gst_value_serialize` and their counterparts.
<!-- impl Caps::fn new_any -->
Creates a new `Caps` that indicates that it is compatible with
any media format.

# Returns

the new `Caps`
<!-- impl Caps::fn new_empty -->
Creates a new `Caps` that is empty. That is, the returned
`Caps` contains no media formats.
The `Caps` is guaranteed to be writable.
Caller is responsible for unreffing the returned caps.

# Returns

the new `Caps`
<!-- impl Caps::fn new_empty_simple -->
Creates a new `Caps` that contains one `Structure` with name
`media_type`.
Caller is responsible for unreffing the returned caps.
## `media_type`
the media type of the structure

# Returns

the new `Caps`
<!-- impl Caps::fn new_full -->
Creates a new `Caps` and adds all the structures listed as
arguments. The list must be `None`-terminated. The structures
are not copied; the returned `Caps` owns the structures.
## `struct1`
the first structure to add

# Returns

the new `Caps`
<!-- impl Caps::fn new_full_valist -->
Creates a new `Caps` and adds all the structures listed as
arguments. The list must be `None`-terminated. The structures
are not copied; the returned `Caps` owns the structures.
## `structure`
the first structure to add
## `var_args`
additional structures to add

# Returns

the new `Caps`
<!-- impl Caps::fn new_simple -->
Creates a new `Caps` that contains one `Structure`. The
structure is defined by the arguments, which have the same format
as `Structure::new`.
Caller is responsible for unreffing the returned caps.
## `media_type`
the media type of the structure
## `fieldname`
first field to set

# Returns

the new `Caps`
<!-- impl Caps::fn append -->
Appends the structures contained in `caps2` to `self`. The structures in
`caps2` are not copied -- they are transferred to `self`, and then `caps2` is
freed. If either caps is ANY, the resulting caps will be ANY.
## `caps2`
the `Caps` to append
<!-- impl Caps::fn append_structure -->
Appends `structure` to `self`. The structure is not copied; `self`
becomes the owner of `structure`.
## `structure`
the `Structure` to append
<!-- impl Caps::fn append_structure_full -->
Appends `structure` with `features` to `self`. The structure is not copied; `self`
becomes the owner of `structure`.
## `structure`
the `Structure` to append
## `features`
the `CapsFeatures` to append
<!-- impl Caps::fn can_intersect -->
Tries intersecting `self` and `caps2` and reports whether the result would not
be empty
## `caps2`
a `Caps` to intersect

# Returns

`true` if intersection would be not empty
<!-- impl Caps::fn copy_nth -->
Creates a new `Caps` and appends a copy of the nth structure
contained in `self`.
## `nth`
the nth structure to copy

# Returns

the new `Caps`
<!-- impl Caps::fn filter_and_map_in_place -->
Calls the provided function once for each structure and caps feature in the
`Caps`. In contrast to `Caps::foreach`, the function may modify the
structure and features. In contrast to `Caps::filter_and_map_in_place`,
the structure and features are removed from the caps if `false` is returned
from the function.
The caps must be mutable.
## `func`
a function to call for each field
## `user_data`
private data
<!-- impl Caps::fn fixate -->
Modifies the given `self` into a representation with only fixed
values. First the caps will be truncated and then the first structure will be
fixated with `Structure::fixate`.

This function takes ownership of `self` and will call `gst_caps_make_writable`
on it so you must not use `self` afterwards unless you keep an additional
reference to it with `gst_caps_ref`.

# Returns

the fixated caps
<!-- impl Caps::fn foreach -->
Calls the provided function once for each structure and caps feature in the
`Caps`. The function must not modify the fields.
Also see `Caps::map_in_place` and `Caps::filter_and_map_in_place`.
## `func`
a function to call for each field
## `user_data`
private data

# Returns

`true` if the supplied function returns `true` for each call,
`false` otherwise.
<!-- impl Caps::fn get_features -->
Finds the features in `self` that has the index `index`, and
returns it.

WARNING: This function takes a const GstCaps *, but returns a
non-const GstCapsFeatures *. This is for programming convenience --
the caller should be aware that structures inside a constant
`Caps` should not be modified. However, if you know the caps
are writable, either because you have just copied them or made
them writable with `gst_caps_make_writable`, you may modify the
features returned in the usual way, e.g. with functions like
`CapsFeatures::add`.

You do not need to free or unref the structure returned, it
belongs to the `Caps`.
## `index`
the index of the structure

# Returns

a pointer to the `CapsFeatures` corresponding
 to `index`
<!-- impl Caps::fn get_size -->
Gets the number of structures contained in `self`.

# Returns

the number of structures that `self` contains
<!-- impl Caps::fn get_structure -->
Finds the structure in `self` that has the index `index`, and
returns it.

WARNING: This function takes a const GstCaps *, but returns a
non-const GstStructure *. This is for programming convenience --
the caller should be aware that structures inside a constant
`Caps` should not be modified. However, if you know the caps
are writable, either because you have just copied them or made
them writable with `gst_caps_make_writable`, you may modify the
structure returned in the usual way, e.g. with functions like
`Structure::set`.

You do not need to free or unref the structure returned, it
belongs to the `Caps`.
## `index`
the index of the structure

# Returns

a pointer to the `Structure` corresponding
 to `index`
<!-- impl Caps::fn intersect -->
Creates a new `Caps` that contains all the formats that are common
to both `self` and `caps2`. Defaults to `CapsIntersectMode::ZigZag` mode.
## `caps2`
a `Caps` to intersect

# Returns

the new `Caps`
<!-- impl Caps::fn intersect_full -->
Creates a new `Caps` that contains all the formats that are common
to both `self` and `caps2`, the order is defined by the `CapsIntersectMode`
used.
## `caps2`
a `Caps` to intersect
## `mode`
The intersection algorithm/mode to use

# Returns

the new `Caps`
<!-- impl Caps::fn is_always_compatible -->
A given `Caps` structure is always compatible with another if
every media format that is in the first is also contained in the
second. That is, `self` is a subset of `caps2`.
## `caps2`
the `Caps` to test

# Returns

`true` if `self` is a subset of `caps2`.
<!-- impl Caps::fn is_any -->
Determines if `self` represents any media format.

# Returns

`true` if `self` represents any format.
<!-- impl Caps::fn is_empty -->
Determines if `self` represents no media formats.

# Returns

`true` if `self` represents no formats.
<!-- impl Caps::fn is_equal -->
Checks if the given caps represent the same set of caps.
## `caps2`
another `Caps`

# Returns

`true` if both caps are equal.
<!-- impl Caps::fn is_equal_fixed -->
Tests if two `Caps` are equal. This function only works on fixed
`Caps`.
## `caps2`
the `Caps` to test

# Returns

`true` if the arguments represent the same format
<!-- impl Caps::fn is_fixed -->
Fixed `Caps` describe exactly one format, that is, they have exactly
one structure, and each field in the structure describes a fixed type.
Examples of non-fixed types are GST_TYPE_INT_RANGE and GST_TYPE_LIST.

# Returns

`true` if `self` is fixed
<!-- impl Caps::fn is_strictly_equal -->
Checks if the given caps are exactly the same set of caps.
## `caps2`
another `Caps`

# Returns

`true` if both caps are strictly equal.
<!-- impl Caps::fn is_subset -->
Checks if all caps represented by `self` are also represented by `superset`.
## `superset`
a potentially greater `Caps`

# Returns

`true` if `self` is a subset of `superset`
<!-- impl Caps::fn is_subset_structure -->
Checks if `structure` is a subset of `self`. See `Caps::is_subset`
for more information.
## `structure`
a potential `Structure` subset of `self`

# Returns

`true` if `structure` is a subset of `self`
<!-- impl Caps::fn is_subset_structure_full -->
Checks if `structure` is a subset of `self`. See `Caps::is_subset`
for more information.
## `structure`
a potential `Structure` subset of `self`
## `features`
a `CapsFeatures` for `structure`

# Returns

`true` if `structure` is a subset of `self`
<!-- impl Caps::fn map_in_place -->
Calls the provided function once for each structure and caps feature in the
`Caps`. In contrast to `Caps::foreach`, the function may modify but not
delete the structures and features. The caps must be mutable.
## `func`
a function to call for each field
## `user_data`
private data

# Returns

`true` if the supplied function returns `true` for each call,
`false` otherwise.
<!-- impl Caps::fn merge -->
Appends the structures contained in `caps2` to `self` if they are not yet
expressed by `self`. The structures in `caps2` are not copied -- they are
transferred to a writable copy of `self`, and then `caps2` is freed.
If either caps is ANY, the resulting caps will be ANY.
## `caps2`
the `Caps` to merge in

# Returns

the merged caps.
<!-- impl Caps::fn merge_structure -->
Appends `structure` to `self` if its not already expressed by `self`.
## `structure`
the `Structure` to merge

# Returns

the merged caps.
<!-- impl Caps::fn merge_structure_full -->
Appends `structure` with `features` to `self` if its not already expressed by `self`.
## `structure`
the `Structure` to merge
## `features`
the `CapsFeatures` to merge

# Returns

the merged caps.
<!-- impl Caps::fn normalize -->
Returns a `Caps` that represents the same set of formats as
`self`, but contains no lists. Each list is expanded into separate
`GstStructures`.

This function takes ownership of `self` and will call `gst_caps_make_writable`
on it so you must not use `self` afterwards unless you keep an additional
reference to it with `gst_caps_ref`.

# Returns

the normalized `Caps`
<!-- impl Caps::fn remove_structure -->
removes the structure with the given index from the list of structures
contained in `self`.
## `idx`
Index of the structure to remove
<!-- impl Caps::fn set_features -->
Sets the `CapsFeatures` `features` for the structure at `index`.
## `index`
the index of the structure
## `features`
the `CapsFeatures` to set
<!-- impl Caps::fn set_simple -->
Sets fields in a `Caps`. The arguments must be passed in the same
manner as `Structure::set`, and be `None`-terminated.
## `field`
first field to set
<!-- impl Caps::fn set_simple_valist -->
Sets fields in a `Caps`. The arguments must be passed in the same
manner as `Structure::set`, and be `None`-terminated.
## `field`
first field to set
## `varargs`
additional parameters
<!-- impl Caps::fn set_value -->
Sets the given `field` on all structures of `self` to the given `value`.
This is a convenience function for calling `Structure::set_value` on
all structures of `self`.
## `field`
name of the field to set
## `value`
value to set the field to
<!-- impl Caps::fn simplify -->
Converts the given `self` into a representation that represents the
same set of formats, but in a simpler form. Component structures that are
identical are merged. Component structures that have values that can be
merged are also merged.

This function takes ownership of `self` and will call `gst_caps_make_writable`
on it if necessary, so you must not use `self` afterwards unless you keep an
additional reference to it with `gst_caps_ref`.

This method does not preserve the original order of `self`.

# Returns

The simplified caps.
<!-- impl Caps::fn steal_structure -->
Retrieves the structure with the given index from the list of structures
contained in `self`. The caller becomes the owner of the returned structure.
## `index`
Index of the structure to retrieve

# Returns

a pointer to the `Structure` corresponding
 to `index`.
<!-- impl Caps::fn subtract -->
Subtracts the `subtrahend` from the `self`.
> This function does not work reliably if optional properties for caps
> are included on one caps and omitted on the other.
## `subtrahend`
`Caps` to subtract

# Returns

the resulting caps
<!-- impl Caps::fn to_string -->
Converts `self` to a string representation. This string representation
can be converted back to a `Caps` by `Caps::from_string`.

For debugging purposes its easier to do something like this:

```C
GST_LOG ("caps are %" GST_PTR_FORMAT, caps);
```
This prints the caps in human readable form.

The current implementation of serialization will lead to unexpected results
when there are nested `Caps` / `Structure` deeper than one level.

# Returns

a newly allocated string representing `self`.
<!-- impl Caps::fn truncate -->
Discard all but the first structure from `self`. Useful when
fixating.

This function takes ownership of `self` and will call `gst_caps_make_writable`
on it if necessary, so you must not use `self` afterwards unless you keep an
additional reference to it with `gst_caps_ref`.

# Returns

truncated caps
<!-- impl Caps::fn from_string -->
Converts `caps` from a string representation.

The current implementation of serialization will lead to unexpected results
when there are nested `Caps` / `Structure` deeper than one level.
## `string`
a string to convert to `Caps`

# Returns

a newly allocated `Caps`
<!-- enum CapsIntersectMode -->
Modes of caps intersection

`CapsIntersectMode::ZigZag` tries to preserve overall order of both caps
by iterating on the caps' structures as the following matrix shows:

```text
         caps1
      +-------------
      | 1  2  4  7
caps2 | 3  5  8 10
      | 6  9 11 12
```
Used when there is no explicit precedence of one caps over the other. e.g.
tee's sink pad getcaps function, it will probe its src pad peers' for their
caps and intersect them with this mode.

`CapsIntersectMode::First` is useful when an element wants to preserve
another element's caps priority order when intersecting with its own caps.
Example: If caps1 is [A, B, C] and caps2 is [E, B, D, A], the result
would be [A, B], maintaining the first caps priority on the intersection.
<!-- enum CapsIntersectMode::variant ZigZag -->
Zig-zags over both caps.
<!-- enum CapsIntersectMode::variant First -->
Keeps the first caps order.
<!-- struct ChildProxy -->
This interface abstracts handling of property sets for elements with
children. Imagine elements such as mixers or polyphonic generators. They all
have multiple `Pad` or some kind of voice objects. Another use case are
container elements like `Bin`.
The element implementing the interface acts as a parent for those child
objects.

By implementing this interface the child properties can be accessed from the
parent element by using `ChildProxy::get` and `ChildProxy::set`.

Property names are written as "child-name::property-name". The whole naming
scheme is recursive. Thus "child1::child2::property" is valid too, if
"child1" and "child2" implement the `ChildProxy` interface.

# Implements

[`ChildProxyExt`](trait.ChildProxyExt.html)
<!-- trait ChildProxyExt -->
Trait containing all `ChildProxy` methods.

# Implementors

[`Bin`](struct.Bin.html), [`ChildProxy`](struct.ChildProxy.html), [`Pipeline`](struct.Pipeline.html)
<!-- trait ChildProxyExt::fn child_added -->
Emits the "child-added" signal.
## `child`
the newly added child
## `name`
the name of the new child
<!-- trait ChildProxyExt::fn child_removed -->
Emits the "child-removed" signal.
## `child`
the removed child
## `name`
the name of the old child
<!-- trait ChildProxyExt::fn get -->
Gets properties of the parent object and its children.
## `first_property_name`
name of the first property to get
<!-- trait ChildProxyExt::fn get_child_by_index -->
Fetches a child by its number.
## `index`
the child's position in the child list

# Returns

the child object or `None` if
 not found (index too high). Unref after usage.

MT safe.
<!-- trait ChildProxyExt::fn get_child_by_name -->
Looks up a child element by the given name.

This virtual method has a default implementation that uses `Object`
together with `GstObjectExt::get_name`. If the interface is to be used with
`GObjects`, this methods needs to be overridden.
## `name`
the child's name

# Returns

the child object or `None` if
 not found. Unref after usage.

MT safe.
<!-- trait ChildProxyExt::fn get_children_count -->
Gets the number of child objects this parent contains.

# Returns

the number of child objects

MT safe.
<!-- trait ChildProxyExt::fn get_property -->
Gets a single property using the GstChildProxy mechanism.
You are responsible for freeing it by calling `gobject::Value::unset`
## `name`
name of the property
## `value`
a `gobject::Value` that should take the result.
<!-- trait ChildProxyExt::fn get_valist -->
Gets properties of the parent object and its children.
## `first_property_name`
name of the first property to get
## `var_args`
return location for the first property, followed optionally by more name/return location pairs, followed by `None`
<!-- trait ChildProxyExt::fn lookup -->
Looks up which object and `gobject::ParamSpec` would be effected by the given `name`.

MT safe.
## `name`
name of the property to look up
## `target`
pointer to a `gobject::Object` that
 takes the real object to set property on
## `pspec`
pointer to take the `gobject::ParamSpec`
 describing the property

# Returns

`true` if `target` and `pspec` could be found. `false` otherwise. In that
case the values for `pspec` and `target` are not modified. Unref `target` after
usage. For plain GObjects `target` is the same as `self`.
<!-- trait ChildProxyExt::fn set -->
Sets properties of the parent object and its children.
## `first_property_name`
name of the first property to set
<!-- trait ChildProxyExt::fn set_property -->
Sets a single property using the GstChildProxy mechanism.
## `name`
name of the property to set
## `value`
new `gobject::Value` for the property
<!-- trait ChildProxyExt::fn set_valist -->
Sets properties of the parent object and its children.
## `first_property_name`
name of the first property to set
## `var_args`
value for the first property, followed optionally by more name/value pairs, followed by `None`
<!-- struct Clock -->
GStreamer uses a global clock to synchronize the plugins in a pipeline.
Different clock implementations are possible by implementing this abstract
base class or, more conveniently, by subclassing `SystemClock`.

The `Clock` returns a monotonically increasing time with the method
`ClockExt::get_time`. Its accuracy and base time depend on the specific
clock implementation but time is always expressed in nanoseconds. Since the
baseline of the clock is undefined, the clock time returned is not
meaningful in itself, what matters are the deltas between two clock times.
The time returned by a clock is called the absolute time.

The pipeline uses the clock to calculate the running time. Usually all
renderers synchronize to the global clock using the buffer timestamps, the
newsegment events and the element's base time, see `Pipeline`.

A clock implementation can support periodic and single shot clock
notifications both synchronous and asynchronous.

One first needs to create a `ClockID` for the periodic or single shot
notification using `ClockExt::new_single_shot_id` or
`ClockExt::new_periodic_id`.

To perform a blocking wait for the specific time of the `ClockID` use the
`Clock::id_wait`. To receive a callback when the specific time is reached
in the clock use `Clock::id_wait_async`. Both these calls can be
interrupted with the `Clock::id_unschedule` call. If the blocking wait is
unscheduled a return value of `ClockReturn::Unscheduled` is returned.

Periodic callbacks scheduled async will be repeatedly called automatically
until it is unscheduled. To schedule a sync periodic callback,
`Clock::id_wait` should be called repeatedly.

The async callbacks can happen from any thread, either provided by the core
or from a streaming thread. The application should be prepared for this.

A `ClockID` that has been unscheduled cannot be used again for any wait
operation, a new `ClockID` should be created and the old unscheduled one
should be destroyed with `Clock::id_unref`.

It is possible to perform a blocking wait on the same `ClockID` from
multiple threads. However, registering the same `ClockID` for multiple
async notifications is not possible, the callback will only be called for
the thread registering the entry last.

None of the wait operations unref the `ClockID`, the owner is responsible
for unreffing the ids itself. This holds for both periodic and single shot
notifications. The reason being that the owner of the `ClockID` has to
keep a handle to the `ClockID` to unblock the wait on FLUSHING events or
state changes and if the entry would be unreffed automatically, the handle
might become invalid without any notification.

These clock operations do not operate on the running time, so the callbacks
will also occur when not in PLAYING state as if the clock just keeps on
running. Some clocks however do not progress when the element that provided
the clock is not PLAYING.

When a clock has the `ClockFlags::CanSetMaster` flag set, it can be
slaved to another `Clock` with the `ClockExt::set_master`. The clock will
then automatically be synchronized to this master clock by repeatedly
sampling the master clock and the slave clock and recalibrating the slave
clock with `ClockExt::set_calibration`. This feature is mostly useful for
plugins that have an internal clock but must operate with another clock
selected by the `Pipeline`. They can track the offset and rate difference
of their internal clock relative to the master clock by using the
`ClockExt::get_calibration` function.

The master/slave synchronisation can be tuned with the `Clock:timeout`,
`Clock:window-size` and `Clock:window-threshold` properties.
The `Clock:timeout` property defines the interval to sample the master
clock and run the calibration functions. `Clock:window-size` defines the
number of samples to use when calibrating and `Clock:window-threshold`
defines the minimum number of samples before the calibration is performed.

# Implements

[`ClockExt`](trait.ClockExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait ClockExt -->
Trait containing all `Clock` methods.

# Implementors

[`Clock`](struct.Clock.html)
<!-- impl Clock::fn id_compare_func -->
Compares the two `ClockID` instances. This function can be used
as a GCompareFunc when sorting ids.
## `id1`
A `ClockID`
## `id2`
A `ClockID` to compare with

# Returns

negative value if a < b; zero if a = b; positive value if a > b

MT safe.
<!-- impl Clock::fn id_get_time -->
Get the time of the clock ID
## `id`
The `ClockID` to query

# Returns

the time of the given clock id.

MT safe.
<!-- impl Clock::fn id_ref -->
Increase the refcount of given `id`.
## `id`
The `ClockID` to ref

# Returns

The same `ClockID` with increased refcount.

MT safe.
<!-- impl Clock::fn id_unref -->
Unref given `id`. When the refcount reaches 0 the
`ClockID` will be freed.

MT safe.
## `id`
The `ClockID` to unref
<!-- impl Clock::fn id_unschedule -->
Cancel an outstanding request with `id`. This can either
be an outstanding async notification or a pending sync notification.
After this call, `id` cannot be used anymore to receive sync or
async notifications, you need to create a new `ClockID`.

MT safe.
## `id`
The id to unschedule
<!-- impl Clock::fn id_wait -->
Perform a blocking wait on `id`.
`id` should have been created with `ClockExt::new_single_shot_id`
or `ClockExt::new_periodic_id` and should not have been unscheduled
with a call to `Clock::id_unschedule`.

If the `jitter` argument is not `None` and this function returns `ClockReturn::Ok`
or `ClockReturn::Early`, it will contain the difference
against the clock and the time of `id` when this method was
called.
Positive values indicate how late `id` was relative to the clock
(in which case this function will return `ClockReturn::Early`).
Negative values indicate how much time was spent waiting on the clock
before this function returned.
## `id`
The `ClockID` to wait on
## `jitter`
a pointer that will contain the jitter,
 can be `None`.

# Returns

the result of the blocking wait. `ClockReturn::Early` will be returned
if the current clock time is past the time of `id`, `ClockReturn::Ok` if
`id` was scheduled in time. `ClockReturn::Unscheduled` if `id` was
unscheduled with `Clock::id_unschedule`.

MT safe.
<!-- impl Clock::fn id_wait_async -->
Register a callback on the given `ClockID` `id` with the given
function and user_data. When passing a `ClockID` with an invalid
time to this function, the callback will be called immediately
with a time set to GST_CLOCK_TIME_NONE. The callback will
be called when the time of `id` has been reached.

The callback `func` can be invoked from any thread, either provided by the
core or from a streaming thread. The application should be prepared for this.
## `id`
a `ClockID` to wait on
## `func`
The callback function
## `user_data`
User data passed in the callback
## `destroy_data`
`GDestroyNotify` for user_data

# Returns

the result of the non blocking wait.

MT safe.
<!-- trait ClockExt::fn add_observation -->
The time `master` of the master clock and the time `slave` of the slave
clock are added to the list of observations. If enough observations
are available, a linear regression algorithm is run on the
observations and `self` is recalibrated.

If this functions returns `true`, `r_squared` will contain the
correlation coefficient of the interpolation. A value of 1.0
means a perfect regression was performed. This value can
be used to control the sampling frequency of the master and slave
clocks.
## `slave`
a time on the slave
## `master`
a time on the master
## `r_squared`
a pointer to hold the result

# Returns

`true` if enough observations were added to run the
regression algorithm.

MT safe.
<!-- trait ClockExt::fn add_observation_unapplied -->
Add a clock observation to the internal slaving algorithm the same as
`ClockExt::add_observation`, and return the result of the master clock
estimation, without updating the internal calibration.

The caller can then take the results and call `ClockExt::set_calibration`
with the values, or some modified version of them.
## `slave`
a time on the slave
## `master`
a time on the master
## `r_squared`
a pointer to hold the result
## `internal`
a location to store the internal time
## `external`
a location to store the external time
## `rate_num`
a location to store the rate numerator
## `rate_denom`
a location to store the rate denominator
<!-- trait ClockExt::fn adjust_unlocked -->
Converts the given `internal` clock time to the external time, adjusting for the
rate and reference time set with `ClockExt::set_calibration` and making sure
that the returned time is increasing. This function should be called with the
clock's OBJECT_LOCK held and is mainly used by clock subclasses.

This function is the reverse of `ClockExt::unadjust_unlocked`.
## `internal`
a clock time

# Returns

the converted time of the clock.
<!-- trait ClockExt::fn adjust_with_calibration -->
Converts the given `internal_target` clock time to the external time,
using the passed calibration parameters. This function performs the
same calculation as `ClockExt::adjust_unlocked` when called using the
current calibration parameters, but doesn't ensure a monotonically
increasing result as `ClockExt::adjust_unlocked` does.

Note: The `self` parameter is unused and can be NULL
## `internal_target`
a clock time
## `cinternal`
a reference internal time
## `cexternal`
a reference external time
## `cnum`
the numerator of the rate of the clock relative to its
 internal time
## `cdenom`
the denominator of the rate of the clock

# Returns

the converted time of the clock.
<!-- trait ClockExt::fn get_calibration -->
Gets the internal rate and reference time of `self`. See
`ClockExt::set_calibration` for more information.

`internal`, `external`, `rate_num`, and `rate_denom` can be left `None` if the
caller is not interested in the values.

MT safe.
## `internal`
a location to store the internal time
## `external`
a location to store the external time
## `rate_num`
a location to store the rate numerator
## `rate_denom`
a location to store the rate denominator
<!-- trait ClockExt::fn get_internal_time -->
Gets the current internal time of the given clock. The time is returned
unadjusted for the offset and the rate.

# Returns

the internal time of the clock. Or GST_CLOCK_TIME_NONE when
given invalid input.

MT safe.
<!-- trait ClockExt::fn get_master -->
Get the master clock that `self` is slaved to or `None` when the clock is
not slaved to any master clock.

# Returns

a master `Clock` or `None`
 when this clock is not slaved to a master clock. Unref after
 usage.

MT safe.
<!-- trait ClockExt::fn get_resolution -->
Get the accuracy of the clock. The accuracy of the clock is the granularity
of the values returned by `ClockExt::get_time`.

# Returns

the resolution of the clock in units of `ClockTime`.

MT safe.
<!-- trait ClockExt::fn get_time -->
Gets the current time of the given clock. The time is always
monotonically increasing and adjusted according to the current
offset and rate.

# Returns

the time of the clock. Or GST_CLOCK_TIME_NONE when
given invalid input.

MT safe.
<!-- trait ClockExt::fn get_timeout -->
Get the amount of time that master and slave clocks are sampled.

# Returns

the interval between samples.
<!-- trait ClockExt::fn is_synced -->
Checks if the clock is currently synced.

This returns if GST_CLOCK_FLAG_NEEDS_STARTUP_SYNC is not set on the clock.

# Returns

`true` if the clock is currently synced
<!-- trait ClockExt::fn new_periodic_id -->
Get an ID from `self` to trigger a periodic notification.
The periodic notifications will start at time `start_time` and
will then be fired with the given `interval`. `id` should be unreffed
after usage.

Free-function: gst_clock_id_unref
## `start_time`
the requested start time
## `interval`
the requested interval

# Returns

a `ClockID` that can be used to request the
 time notification.

MT safe.
<!-- trait ClockExt::fn new_single_shot_id -->
Get a `ClockID` from `self` to trigger a single shot
notification at the requested time. The single shot id should be
unreffed after usage.

Free-function: gst_clock_id_unref
## `time`
the requested time

# Returns

a `ClockID` that can be used to request the
 time notification.

MT safe.
<!-- trait ClockExt::fn periodic_id_reinit -->
Reinitializes the provided periodic `id` to the provided start time and
interval. Does not modify the reference count.
## `id`
a `ClockID`
## `start_time`
the requested start time
## `interval`
the requested interval

# Returns

`true` if the GstClockID could be reinitialized to the provided
`time`, else `false`.
<!-- trait ClockExt::fn set_calibration -->
Adjusts the rate and time of `self`. A rate of 1/1 is the normal speed of
the clock. Values bigger than 1/1 make the clock go faster.

`internal` and `external` are calibration parameters that arrange that
`ClockExt::get_time` should have been `external` at internal time `internal`.
This internal time should not be in the future; that is, it should be less
than the value of `ClockExt::get_internal_time` when this function is called.

Subsequent calls to `ClockExt::get_time` will return clock times computed as
follows:


```text
  time = (internal_time - internal) * rate_num / rate_denom + external
```

This formula is implemented in `ClockExt::adjust_unlocked`. Of course, it
tries to do the integer arithmetic as precisely as possible.

Note that `ClockExt::get_time` always returns increasing values so when you
move the clock backwards, `ClockExt::get_time` will report the previous value
until the clock catches up.

MT safe.
## `internal`
a reference internal time
## `external`
a reference external time
## `rate_num`
the numerator of the rate of the clock relative to its
 internal time
## `rate_denom`
the denominator of the rate of the clock
<!-- trait ClockExt::fn set_master -->
Set `master` as the master clock for `self`. `self` will be automatically
calibrated so that `ClockExt::get_time` reports the same time as the
master clock.

A clock provider that slaves its clock to a master can get the current
calibration values with `ClockExt::get_calibration`.

`master` can be `None` in which case `self` will not be slaved anymore. It will
however keep reporting its time adjusted with the last configured rate
and time offsets.
## `master`
a master `Clock`

# Returns

`true` if the clock is capable of being slaved to a master clock.
Trying to set a master on a clock without the
`ClockFlags::CanSetMaster` flag will make this function return `false`.

MT safe.
<!-- trait ClockExt::fn set_resolution -->
Set the accuracy of the clock. Some clocks have the possibility to operate
with different accuracy at the expense of more resource usage. There is
normally no need to change the default resolution of a clock. The resolution
of a clock can only be changed if the clock has the
`ClockFlags::CanSetResolution` flag set.
## `resolution`
The resolution to set

# Returns

the new resolution of the clock.
<!-- trait ClockExt::fn set_synced -->
Sets `self` to synced and emits the GstClock::synced signal, and wakes up any
thread waiting in `ClockExt::wait_for_sync`.

This function must only be called if GST_CLOCK_FLAG_NEEDS_STARTUP_SYNC
is set on the clock, and is intended to be called by subclasses only.
## `synced`
if the clock is synced
<!-- trait ClockExt::fn set_timeout -->
Set the amount of time, in nanoseconds, to sample master and slave
clocks
## `timeout`
a timeout
<!-- trait ClockExt::fn single_shot_id_reinit -->
Reinitializes the provided single shot `id` to the provided time. Does not
modify the reference count.
## `id`
a `ClockID`
## `time`
The requested time.

# Returns

`true` if the GstClockID could be reinitialized to the provided
`time`, else `false`.
<!-- trait ClockExt::fn unadjust_unlocked -->
Converts the given `external` clock time to the internal time of `self`,
using the rate and reference time set with `ClockExt::set_calibration`.
This function should be called with the clock's OBJECT_LOCK held and
is mainly used by clock subclasses.

This function is the reverse of `ClockExt::adjust_unlocked`.
## `external`
an external clock time

# Returns

the internal time of the clock corresponding to `external`.
<!-- trait ClockExt::fn unadjust_with_calibration -->
Converts the given `external_target` clock time to the internal time,
using the passed calibration parameters. This function performs the
same calculation as `ClockExt::unadjust_unlocked` when called using the
current calibration parameters.

Note: The `self` parameter is unused and can be NULL
## `external_target`
a clock time
## `cinternal`
a reference internal time
## `cexternal`
a reference external time
## `cnum`
the numerator of the rate of the clock relative to its
 internal time
## `cdenom`
the denominator of the rate of the clock

# Returns

the converted time of the clock.
<!-- trait ClockExt::fn wait_for_sync -->
Waits until `self` is synced for reporting the current time. If `timeout`
is `GST_CLOCK_TIME_NONE` it will wait forever, otherwise it will time out
after `timeout` nanoseconds.

For asynchronous waiting, the GstClock::synced signal can be used.

This returns immediately with TRUE if GST_CLOCK_FLAG_NEEDS_STARTUP_SYNC
is not set on the clock, or if the clock is already synced.
## `timeout`
timeout for waiting or `GST_CLOCK_TIME_NONE`

# Returns

`true` if waiting was successful, or `false` on timeout
<!-- struct Context -->
`Context` is a container object used to store contexts like a device
context, a display server connection and similar concepts that should
be shared between multiple elements.

Applications can set a context on a complete pipeline by using
`ElementExt::set_context`, which will then be propagated to all
child elements. Elements can handle these in `ElementClass.set_context`()
and merge them with the context information they already have.

When an element needs a context it will do the following actions in this
order until one step succeeds:
1. Check if the element already has a context
2. Query downstream with GST_QUERY_CONTEXT for the context
3. Query upstream with GST_QUERY_CONTEXT for the context
4. Post a GST_MESSAGE_NEED_CONTEXT message on the bus with the required
 context types and afterwards check if a usable context was set now
5. Create a context by itself and post a GST_MESSAGE_HAVE_CONTEXT message
 on the bus.

Bins will catch GST_MESSAGE_NEED_CONTEXT messages and will set any previously
known context on the element that asks for it if possible. Otherwise the
application should provide one if it can.

`Context`<!-- -->s can be persistent.
A persistent `Context` is kept in elements when they reach
`State::Null`, non-persistent ones will be removed.
Also, a non-persistent context won't override a previous persistent
context set to an element.
<!-- impl Context::fn new -->
Create a new context.
## `context_type`
Context type
## `persistent`
Persistent context

# Returns

The new context.
<!-- impl Context::fn get_context_type -->
Get the type of `self`.

# Returns

The type of the context.
<!-- impl Context::fn get_structure -->
Access the structure of the context.

# Returns

The structure of the context. The structure is
still owned by the context, which means that you should not modify it,
free it and that the pointer becomes invalid when you free the context.
<!-- impl Context::fn has_context_type -->
Checks if `self` has `context_type`.
## `context_type`
Context type to check.

# Returns

`true` if `self` has `context_type`.
<!-- impl Context::fn is_persistent -->
Check if `self` is persistent.

# Returns

`true` if the context is persistent.
<!-- impl Context::fn writable_structure -->
Get a writable version of the structure.

# Returns

The structure of the context. The structure is still
owned by the context, which means that you should not free it and
that the pointer becomes invalid when you free the context.
This function checks if `self` is writable.
<!-- enum CoreError -->
Core errors are errors inside the core GStreamer library.
<!-- enum CoreError::variant Failed -->
a general error which doesn't fit in any other
category. Make sure you add a custom message to the error call.
<!-- enum CoreError::variant TooLazy -->
do not use this except as a placeholder for
deciding where to go while developing code.
<!-- enum CoreError::variant NotImplemented -->
use this when you do not want to implement
this functionality yet.
<!-- enum CoreError::variant StateChange -->
used for state change errors.
<!-- enum CoreError::variant Pad -->
used for pad-related errors.
<!-- enum CoreError::variant Thread -->
used for thread-related errors.
<!-- enum CoreError::variant Negotiation -->
used for negotiation-related errors.
<!-- enum CoreError::variant Event -->
used for event-related errors.
<!-- enum CoreError::variant Seek -->
used for seek-related errors.
<!-- enum CoreError::variant Caps -->
used for caps-related errors.
<!-- enum CoreError::variant Tag -->
used for negotiation-related errors.
<!-- enum CoreError::variant MissingPlugin -->
used if a plugin is missing.
<!-- enum CoreError::variant Clock -->
used for clock related errors.
<!-- enum CoreError::variant Disabled -->
used if functionality has been disabled at
 compile time.
<!-- enum CoreError::variant NumErrors -->
the number of core error types.
<!-- struct DateTime -->
Struct to store date, time and timezone information altogether.
`DateTime` is refcounted and immutable.

Date information is handled using the proleptic Gregorian calendar.

Provides basic creation functions and accessor functions to its fields.
<!-- impl DateTime::fn new -->
Creates a new `DateTime` using the date and times in the gregorian calendar
in the supplied timezone.

`year` should be from 1 to 9999, `month` should be from 1 to 12, `day` from
1 to 31, `hour` from 0 to 23, `minutes` and `seconds` from 0 to 59.

Note that `tzoffset` is a float and was chosen so for being able to handle
some fractional timezones, while it still keeps the readability of
representing it in hours for most timezones.

If value is -1 then all over value will be ignored. For example
if `month` == -1, then `DateTime` will created only for `year`. If
`day` == -1, then `DateTime` will created for `year` and `month` and
so on.

Free-function: gst_date_time_unref
## `tzoffset`
Offset from UTC in hours.
## `year`
the gregorian year
## `month`
the gregorian month
## `day`
the day of the gregorian month
## `hour`
the hour of the day
## `minute`
the minute of the hour
## `seconds`
the second of the minute

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn new_from_g_date_time -->
Creates a new `DateTime` from a `glib::DateTime` object.

Free-function: gst_date_time_unref
## `dt`
the `glib::DateTime`. The new `DateTime` takes ownership.

# Returns

a newly created `DateTime`,
or `None` on error
<!-- impl DateTime::fn new_from_iso8601_string -->
Tries to parse common variants of ISO-8601 datetime strings into a
`DateTime`. Possible input formats are (for example):
2012-06-30T22:46:43Z, 2012, 2012-06, 2012-06-30, 2012-06-30T22:46:43-0430,
2012-06-30T22:46Z, 2012-06-30T22:46-0430, 2012-06-30 22:46,
2012-06-30 22:46:43, 2012-06-00, 2012-00-00, 2012-00-30, 22:46:43Z, 22:46Z,
22:46:43-0430, 22:46-0430, 22:46:30, 22:46
If no date is provided, it is assumed to be "today" in the timezone
provided (if any), otherwise UTC.

Free-function: gst_date_time_unref
## `string`
ISO 8601-formatted datetime string.

# Returns

a newly created `DateTime`,
or `None` on error
<!-- impl DateTime::fn new_from_unix_epoch_local_time -->
Creates a new `DateTime` using the time since Jan 1, 1970 specified by
`secs`. The `DateTime` is in the local timezone.

Free-function: gst_date_time_unref
## `secs`
seconds from the Unix epoch

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn new_from_unix_epoch_utc -->
Creates a new `DateTime` using the time since Jan 1, 1970 specified by
`secs`. The `DateTime` is in the UTC timezone.

Free-function: gst_date_time_unref
## `secs`
seconds from the Unix epoch

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn new_local_time -->
Creates a new `DateTime` using the date and times in the gregorian calendar
in the local timezone.

`year` should be from 1 to 9999, `month` should be from 1 to 12, `day` from
1 to 31, `hour` from 0 to 23, `minutes` and `seconds` from 0 to 59.

If `month` is -1, then the `DateTime` created will only contain `year`,
and all other fields will be considered not set.

If `day` is -1, then the `DateTime` created will only contain `year` and
`month` and all other fields will be considered not set.

If `hour` is -1, then the `DateTime` created will only contain `year` and
`month` and `day`, and the time fields will be considered not set. In this
case `minute` and `seconds` should also be -1.

Free-function: gst_date_time_unref
## `year`
the gregorian year
## `month`
the gregorian month, or -1
## `day`
the day of the gregorian month, or -1
## `hour`
the hour of the day, or -1
## `minute`
the minute of the hour, or -1
## `seconds`
the second of the minute, or -1

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn new_now_local_time -->
Creates a new `DateTime` representing the current date and time.

Free-function: gst_date_time_unref

# Returns

the newly created `DateTime` which should
 be freed with `DateTime::unref`.
<!-- impl DateTime::fn new_now_utc -->
Creates a new `DateTime` that represents the current instant at Universal
coordinated time.

Free-function: gst_date_time_unref

# Returns

the newly created `DateTime` which should
 be freed with `DateTime::unref`.
<!-- impl DateTime::fn new_y -->
Creates a new `DateTime` using the date and times in the gregorian calendar
in the local timezone.

`year` should be from 1 to 9999.

Free-function: gst_date_time_unref
## `year`
the gregorian year

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn new_ym -->
Creates a new `DateTime` using the date and times in the gregorian calendar
in the local timezone.

`year` should be from 1 to 9999, `month` should be from 1 to 12.

If value is -1 then all over value will be ignored. For example
if `month` == -1, then `DateTime` will created only for `year`.

Free-function: gst_date_time_unref
## `year`
the gregorian year
## `month`
the gregorian month

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn new_ymd -->
Creates a new `DateTime` using the date and times in the gregorian calendar
in the local timezone.

`year` should be from 1 to 9999, `month` should be from 1 to 12, `day` from
1 to 31.

If value is -1 then all over value will be ignored. For example
if `month` == -1, then `DateTime` will created only for `year`. If
`day` == -1, then `DateTime` will created for `year` and `month` and
so on.

Free-function: gst_date_time_unref
## `year`
the gregorian year
## `month`
the gregorian month
## `day`
the day of the gregorian month

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn get_day -->
Returns the day of the month of this `DateTime`.
Call gst_date_time_has_day before, to avoid warnings.

# Returns

The day of this `DateTime`
<!-- impl DateTime::fn get_hour -->
Retrieves the hour of the day represented by `self` in the gregorian
calendar. The return is in the range of 0 to 23.
Call gst_date_time_has_haur before, to avoid warnings.

# Returns

the hour of the day
<!-- impl DateTime::fn get_microsecond -->
Retrieves the fractional part of the seconds in microseconds represented by
`self` in the gregorian calendar.

# Returns

the microsecond of the second
<!-- impl DateTime::fn get_minute -->
Retrieves the minute of the hour represented by `self` in the gregorian
calendar.
Call gst_date_time_has_minute before, to avoid warnings.

# Returns

the minute of the hour
<!-- impl DateTime::fn get_month -->
Returns the month of this `DateTime`. January is 1, February is 2, etc..
Call gst_date_time_has_month before, to avoid warnings.

# Returns

The month of this `DateTime`
<!-- impl DateTime::fn get_second -->
Retrieves the second of the minute represented by `self` in the gregorian
calendar.
Call gst_date_time_has_second before, to avoid warnings.

# Returns

the second represented by `self`
<!-- impl DateTime::fn get_time_zone_offset -->
Retrieves the offset from UTC in hours that the timezone specified
by `self` represents. Timezones ahead (to the east) of UTC have positive
values, timezones before (to the west) of UTC have negative values.
If `self` represents UTC time, then the offset is zero.

# Returns

the offset from UTC in hours
<!-- impl DateTime::fn get_year -->
Returns the year of this `DateTime`
Call gst_date_time_has_year before, to avoid warnings.

# Returns

The year of this `DateTime`
<!-- impl DateTime::fn has_day -->

# Returns

`true` if `self`<!-- -->'s day field is set, otherwise `false`
<!-- impl DateTime::fn has_month -->

# Returns

`true` if `self`<!-- -->'s month field is set, otherwise `false`
<!-- impl DateTime::fn has_second -->

# Returns

`true` if `self`<!-- -->'s second field is set, otherwise `false`
<!-- impl DateTime::fn has_time -->

# Returns

`true` if `self`<!-- -->'s hour and minute fields are set,
 otherwise `false`
<!-- impl DateTime::fn has_year -->

# Returns

`true` if `self`<!-- -->'s year field is set (which should always
 be the case), otherwise `false`
<!-- impl DateTime::fn ref -->
Atomically increments the reference count of `self` by one.

# Returns

the reference `self`
<!-- impl DateTime::fn to_g_date_time -->
Creates a new `glib::DateTime` from a fully defined `DateTime` object.

Free-function: g_date_time_unref

# Returns

a newly created `glib::DateTime`, or
`None` on error
<!-- impl DateTime::fn to_iso8601_string -->
Create a minimal string compatible with ISO-8601. Possible output formats
are (for example): 2012, 2012-06, 2012-06-23, 2012-06-23T23:30Z,
2012-06-23T23:30+0100, 2012-06-23T23:30:59Z, 2012-06-23T23:30:59+0100

# Returns

a newly allocated string formatted according
 to ISO 8601 and only including the datetime fields that are
 valid, or `None` in case there was an error. The string should
 be freed with `g_free`.
<!-- impl DateTime::fn unref -->
Atomically decrements the reference count of `self` by one. When the
reference count reaches zero, the structure is freed.
<!-- struct Device -->
`Device` are objects representing a device, they contain
relevant metadata about the device, such as its class and the `Caps`
representing the media types it can produce or handle.

`Device` are created by `DeviceProvider` objects which can be
aggregated by `DeviceMonitor` objects.

# Implements

[`DeviceExt`](trait.DeviceExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait DeviceExt -->
Trait containing all `Device` methods.

# Implementors

[`Device`](struct.Device.html)
<!-- trait DeviceExt::fn create_element -->
Creates the element with all of the required parameters set to use
this device.
## `name`
name of new element, or `None` to automatically
create a unique name.

# Returns

a new `Element` configured to use this device
<!-- trait DeviceExt::fn get_caps -->
Getter for the `Caps` that this device supports.

# Returns

The `Caps` supported by this device. Unref with
`gst_caps_unref` when done.
<!-- trait DeviceExt::fn get_device_class -->
Gets the "class" of a device. This is a "/" separated list of
classes that represent this device. They are a subset of the
classes of the `DeviceProvider` that produced this device.

# Returns

The device class. Free with `g_free` after use.
<!-- trait DeviceExt::fn get_display_name -->
Gets the user-friendly name of the device.

# Returns

The device name. Free with `g_free` after use.
<!-- trait DeviceExt::fn get_properties -->
Gets the extra properties of a device.

# Returns

The extra properties or `None` when there are none.
 Free with `Structure::free` after use.
<!-- trait DeviceExt::fn has_classes -->
Check if `self` matches all of the given classes
## `classes`
a "/"-separated list of device classes to match, only match if
 all classes are matched

# Returns

`true` if `self` matches.
<!-- trait DeviceExt::fn has_classesv -->
Check if `factory` matches all of the given classes
## `classes`
a `None` terminated array of classes
 to match, only match if all classes are matched

# Returns

`true` if `self` matches.
<!-- trait DeviceExt::fn reconfigure_element -->
Tries to reconfigure an existing element to use the device. If this
function fails, then one must destroy the element and create a new one
using `DeviceExt::create_element`.

Note: This should only be implemented for elements can change their
device in the PLAYING state.
## `element`
a `Element`

# Returns

`true` if the element could be reconfigured to use this device,
`false` otherwise.
<!-- struct DeviceMonitor -->
Applications should create a `DeviceMonitor` when they want
to probe, list and monitor devices of a specific type. The
`DeviceMonitor` will create the appropriate
`DeviceProvider` objects and manage them. It will then post
messages on its `Bus` for devices that have been added and
removed.

The device monitor will monitor all devices matching the filters that
the application has set.

The basic use pattern of a device monitor is as follows:

```text
  static gboolean
  my_bus_func (GstBus * bus, GstMessage * message, gpointer user_data)
  {
     GstDevice *device;
     gchar *name;

     switch (GST_MESSAGE_TYPE (message)) {
       case GST_MESSAGE_DEVICE_ADDED:
         gst_message_parse_device_added (message, &device);
         name = gst_device_get_display_name (device);
         g_print("Device added: %s\n", name);
         g_free (name);
         gst_object_unref (device);
         break;
       case GST_MESSAGE_DEVICE_REMOVED:
         gst_message_parse_device_removed (message, &device);
         name = gst_device_get_display_name (device);
         g_print("Device removed: %s\n", name);
         g_free (name);
         gst_object_unref (device);
         break;
       default:
         break;
     }

     return G_SOURCE_CONTINUE;
  }

  GstDeviceMonitor *
  setup_raw_video_source_device_monitor (void) {
     GstDeviceMonitor *monitor;
     GstBus *bus;
     GstCaps *caps;

     monitor = gst_device_monitor_new ();

     bus = gst_device_monitor_get_bus (monitor);
     gst_bus_add_watch (bus, my_bus_func, NULL);
     gst_object_unref (bus);

     caps = gst_caps_new_empty_simple ("video/x-raw");
     gst_device_monitor_add_filter (monitor, "Video/Source", caps);
     gst_caps_unref (caps);

     gst_device_monitor_start (monitor);

     return monitor;
  }
```

# Implements

[`DeviceMonitorExt`](trait.DeviceMonitorExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait DeviceMonitorExt -->
Trait containing all `DeviceMonitor` methods.

# Implementors

[`DeviceMonitor`](struct.DeviceMonitor.html)
<!-- impl DeviceMonitor::fn new -->
Create a new `DeviceMonitor`

# Returns

a new device monitor.
<!-- trait DeviceMonitorExt::fn add_filter -->
Adds a filter for which `Device` will be monitored, any device that matches
all these classes and the `Caps` will be returned.

If this function is called multiple times to add more filters, each will be
matched independently. That is, adding more filters will not further restrict
what devices are matched.

The `Caps` supported by the device as returned by `DeviceExt::get_caps` are
not intersected with caps filters added using this function.

Filters must be added before the `DeviceMonitor` is started.
## `classes`
device classes to use as filter or `None` for any class
## `caps`
the `Caps` to filter or `None` for ANY

# Returns

The id of the new filter or 0 if no provider matched the filter's
 classes.
<!-- trait DeviceMonitorExt::fn get_bus -->
Gets the `Bus` of this `DeviceMonitor`

# Returns

a `Bus`
<!-- trait DeviceMonitorExt::fn get_devices -->
Gets a list of devices from all of the relevant monitors. This may actually
probe the hardware if the monitor is not currently started.

# Returns

a `glib::List` of
 `Device`
<!-- trait DeviceMonitorExt::fn get_providers -->
Get a list of the currently selected device provider factories.

This

# Returns


 A list of device provider factory names that are currently being
 monitored by `self` or `None` when nothing is being monitored.
<!-- trait DeviceMonitorExt::fn get_show_all_devices -->
Get if `self` is curretly showing all devices, even those from hidden
providers.

# Returns

`true` when all devices will be shown.
<!-- trait DeviceMonitorExt::fn remove_filter -->
Removes a filter from the `DeviceMonitor` using the id that was returned
by `DeviceMonitorExt::add_filter`.
## `filter_id`
the id of the filter

# Returns

`true` of the filter id was valid, `false` otherwise
<!-- trait DeviceMonitorExt::fn set_show_all_devices -->
Set if all devices should be visible, even those devices from hidden
providers. Setting `show_all` to true might show some devices multiple times.
## `show_all`
show all devices
<!-- trait DeviceMonitorExt::fn start -->
Starts monitoring the devices, one this has succeeded, the
`MessageType::DeviceAdded` and `MessageType::DeviceRemoved` messages
will be emitted on the bus when the list of devices changes.

# Returns

`true` if the device monitoring could be started
<!-- trait DeviceMonitorExt::fn stop -->
Stops monitoring the devices.
<!-- struct DeviceProvider -->
A `DeviceProvider` subclass is provided by a plugin that handles devices
if there is a way to programatically list connected devices. It can also
optionally provide updates to the list of connected devices.

Each `DeviceProvider` subclass is a singleton, a plugin should
normally provide a single subclass for all devices.

Applications would normally use a `DeviceMonitor` to monitor devices
from all relevant providers.

# Implements

[`DeviceProviderExt`](trait.DeviceProviderExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait DeviceProviderExt -->
Trait containing all `DeviceProvider` methods.

# Implementors

[`DeviceProvider`](struct.DeviceProvider.html)
<!-- impl DeviceProvider::fn register -->
Create a new device providerfactory capable of instantiating objects of the
`type_` and add the factory to `plugin`.
## `plugin`
`Plugin` to register the device provider with, or `None` for
 a static device provider.
## `name`
name of device providers of this type
## `rank`
rank of device provider (higher rank means more importance when autoplugging)
## `type_`
GType of device provider to register

# Returns

`true`, if the registering succeeded, `false` on error
<!-- trait DeviceProviderExt::fn device_add -->
Posts a message on the provider's `Bus` to inform applications that
a new device has been added.

This is for use by subclasses.
## `device`
a `Device` that has been added
<!-- trait DeviceProviderExt::fn device_remove -->
Posts a message on the provider's `Bus` to inform applications that
a device has been removed.

This is for use by subclasses.
## `device`
a `Device` that has been removed
<!-- trait DeviceProviderExt::fn get_bus -->
Gets the `Bus` of this `DeviceProvider`

# Returns

a `Bus`
<!-- trait DeviceProviderExt::fn get_devices -->
Gets a list of devices that this provider understands. This may actually
probe the hardware if the provider is not currently started.

# Returns

a `glib::List` of
 `Device`
<!-- trait DeviceProviderExt::fn get_factory -->
Retrieves the factory that was used to create this device provider.

# Returns

the `DeviceProviderFactory` used for
 creating this device provider. no refcounting is needed.
<!-- trait DeviceProviderExt::fn get_hidden_providers -->
Get the provider factory names of the `DeviceProvider` instances that
are hidden by `self`.

# Returns


 a list of hidden providers factory names or `None` when
 nothing is hidden by `self`. Free with g_strfreev.
<!-- trait DeviceProviderExt::fn hide_provider -->
Make `self` hide the devices from the factory with `name`.

This function is used when `self` will also provide the devices reported
by provider factory `name`. A monitor should stop monitoring the
device provider with `name` to avoid duplicate devices.
## `name`
a provider factory name
<!-- trait DeviceProviderExt::fn start -->
Starts providering the devices. This will cause `MessageType::DeviceAdded`
and `MessageType::DeviceRemoved` messages to be posted on the provider's bus
when devices are added or removed from the system.

Since the `DeviceProvider` is a singleton,
`DeviceProviderExt::start` may already have been called by another
user of the object, `DeviceProviderExt::stop` needs to be called the same
number of times.

# Returns

`true` if the device providering could be started
<!-- trait DeviceProviderExt::fn stop -->
Decreases the use-count by one. If the use count reaches zero, this
`DeviceProvider` will stop providering the devices. This needs to be
called the same number of times that `DeviceProviderExt::start` was called.
<!-- trait DeviceProviderExt::fn unhide_provider -->
Make `self` unhide the devices from factory `name`.

This function is used when `self` will no longer provide the devices
reported by provider factory `name`. A monitor should start
monitoring the devices from provider factory `name` in order to see
all devices again.
## `name`
a provider factory name
<!-- struct DeviceProviderFactory -->
`DeviceProviderFactory` is used to create instances of device providers. A
GstDeviceProviderfactory can be added to a `Plugin` as it is also a
`PluginFeature`.

Use the `DeviceProviderFactory::find` and
`DeviceProviderFactoryExt::get` functions to create device
provider instances or use `DeviceProviderFactory::get_by_name` as a
convenient shortcut.

# Implements

[`DeviceProviderFactoryExt`](trait.DeviceProviderFactoryExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait DeviceProviderFactoryExt -->
Trait containing all `DeviceProviderFactory` methods.

# Implementors

[`DeviceProviderFactory`](struct.DeviceProviderFactory.html)
<!-- impl DeviceProviderFactory::fn find -->
Search for an device provider factory of the given name. Refs the returned
device provider factory; caller is responsible for unreffing.
## `name`
name of factory to find

# Returns

`DeviceProviderFactory` if
found, `None` otherwise
<!-- impl DeviceProviderFactory::fn get_by_name -->
Returns the device provider of the type defined by the given device
provider factory.
## `factoryname`
a named factory to instantiate

# Returns

a `DeviceProvider` or `None`
if unable to create device provider
<!-- impl DeviceProviderFactory::fn list_get_device_providers -->
Get a list of factories with a rank greater or equal to `minrank`.
The list of factories is returned by decreasing rank.
## `minrank`
Minimum rank

# Returns


a `glib::List` of `DeviceProviderFactory` device providers. Use
`PluginFeature::list_free` after usage.
<!-- trait DeviceProviderFactoryExt::fn get -->
Returns the device provider of the type defined by the given device
providerfactory.

# Returns

the `DeviceProvider` or `None`
if the device provider couldn't be created
<!-- trait DeviceProviderFactoryExt::fn get_device_provider_type -->
Get the `glib::Type` for device providers managed by this factory. The type can
only be retrieved if the device provider factory is loaded, which can be
assured with `PluginFeature::load`.

# Returns

the `glib::Type` for device providers managed by this factory.
<!-- trait DeviceProviderFactoryExt::fn get_metadata -->
Get the metadata on `self` with `key`.
## `key`
a key

# Returns

the metadata with `key` on `self` or `None`
when there was no metadata with the given `key`.
<!-- trait DeviceProviderFactoryExt::fn get_metadata_keys -->
Get the available keys for the metadata on `self`.

# Returns


a `None`-terminated array of key strings, or `None` when there is no
metadata. Free with `g_strfreev` when no longer needed.
<!-- trait DeviceProviderFactoryExt::fn has_classes -->
Check if `self` matches all of the given `classes`
## `classes`
a "/" separate list of classes to match, only match
 if all classes are matched

# Returns

`true` if `self` matches or if `classes` is `None`.
<!-- trait DeviceProviderFactoryExt::fn has_classesv -->
Check if `self` matches all of the given classes
## `classes`
a `None` terminated array
 of classes to match, only match if all classes are matched

# Returns

`true` if `self` matches.
<!-- struct Element -->
GstElement is the abstract base class needed to construct an element that
can be used in a GStreamer pipeline. Please refer to the plugin writers
guide for more information on creating `Element` subclasses.

The name of a `Element` can be get with `gst_element_get_name` and set with
`gst_element_set_name`. For speed, GST_ELEMENT_NAME() can be used in the
core when using the appropriate locking. Do not use this in plug-ins or
applications in order to retain ABI compatibility.

Elements can have pads (of the type `Pad`). These pads link to pads on
other elements. `Buffer` flow between these linked pads.
A `Element` has a `glib::List` of `Pad` structures for all their input (or sink)
and output (or source) pads.
Core and plug-in writers can add and remove pads with `ElementExt::add_pad`
and `ElementExt::remove_pad`.

An existing pad of an element can be retrieved by name with
`ElementExt::get_static_pad`. A new dynamic pad can be created using
`ElementExt::request_pad` with a `PadTemplate`.
An iterator of all pads can be retrieved with `ElementExt::iterate_pads`.

Elements can be linked through their pads.
If the link is straightforward, use the `ElementExt::link`
convenience function to link two elements, or `ElementExt::link_many`
for more elements in a row.
Use `ElementExt::link_filtered` to link two elements constrained by
a specified set of `Caps`.
For finer control, use `ElementExt::link_pads` and
`ElementExt::link_pads_filtered` to specify the pads to link on
each element by name.

Each element has a state (see `State`). You can get and set the state
of an element with `ElementExt::get_state` and `ElementExt::set_state`.
Setting a state triggers a `StateChange`. To get a string representation
of a `State`, use `Element::state_get_name`.

You can get and set a `Clock` on an element using `ElementExt::get_clock`
and `ElementExt::set_clock`.
Some elements can provide a clock for the pipeline if
the `ElementFlags::ProvideClock` flag is set. With the
`ElementExt::provide_clock` method one can retrieve the clock provided by
such an element.
Not all elements require a clock to operate correctly. If the
`ElementFlags::RequireClock`() flag is set, a clock should be set on the
element with `ElementExt::set_clock`.

Note that clock selection and distribution is normally handled by the
toplevel `Pipeline` so the clock functions are only to be used in very
specific situations.

# Implements

[`ElementExt`](trait.ElementExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait ElementExt -->
Trait containing all `Element` methods.

# Implementors

[`Bin`](struct.Bin.html), [`Element`](struct.Element.html), [`TagSetter`](struct.TagSetter.html)
<!-- impl Element::fn make_from_uri -->
Creates an element for handling the given URI.
## `type_`
Whether to create a source or a sink
## `uri`
URI to create an element for
## `elementname`
Name of created element, can be `None`.

# Returns

a new element or `None` if none could be created
<!-- impl Element::fn register -->
Create a new elementfactory capable of instantiating objects of the
`type_` and add the factory to `plugin`.
## `plugin`
`Plugin` to register the element with, or `None` for
 a static element.
## `name`
name of elements of this type
## `rank`
rank of element (higher rank means more importance when autoplugging)
## `type_`
GType of element to register

# Returns

`true`, if the registering succeeded, `false` on error
<!-- impl Element::fn state_change_return_get_name -->
Gets a string representing the given state change result.
## `state_ret`
a `StateChangeReturn` to get the name of.

# Returns

a string with the name of the state
 result.
<!-- impl Element::fn state_get_name -->
Gets a string representing the given state.
## `state`
a `State` to get the name of.

# Returns

a string with the name of the state.
<!-- trait ElementExt::fn abort_state -->
Abort the state change of the element. This function is used
by elements that do asynchronous state changes and find out
something is wrong.

This function should be called with the STATE_LOCK held.

MT safe.
<!-- trait ElementExt::fn add_pad -->
Adds a pad (link point) to `self`. `pad`'s parent will be set to `self`;
see `GstObjectExt::set_parent` for refcounting information.

Pads are not automatically activated so elements should perform the needed
steps to activate the pad in case this pad is added in the PAUSED or PLAYING
state. See `PadExt::set_active` for more information about activating pads.

The pad and the element should be unlocked when calling this function.

This function will emit the `Element::pad-added` signal on the element.
## `pad`
the `Pad` to add to the element.

# Returns

`true` if the pad could be added. This function can fail when
a pad with the same name already existed or the pad already had another
parent.

MT safe.
<!-- trait ElementExt::fn add_property_deep_notify_watch -->

Feature: `v1_10`

## `property_name`
name of property to watch for changes, or
 NULL to watch all properties
## `include_value`
whether to include the new property value in the message

# Returns

a watch id, which can be used in connection with
 `ElementExt::remove_property_notify_watch` to remove the watch again.
<!-- trait ElementExt::fn add_property_notify_watch -->

Feature: `v1_10`

## `property_name`
name of property to watch for changes, or
 NULL to watch all properties
## `include_value`
whether to include the new property value in the message

# Returns

a watch id, which can be used in connection with
 `ElementExt::remove_property_notify_watch` to remove the watch again.
<!-- trait ElementExt::fn call_async -->
Calls `func` from another thread and passes `user_data` to it. This is to be
used for cases when a state change has to be performed from a streaming
thread, directly via `ElementExt::set_state` or indirectly e.g. via SEEK
events.

Calling those functions directly from the streaming thread will cause
deadlocks in many situations, as they might involve waiting for the
streaming thread to shut down from this very streaming thread.

MT safe.

Feature: `v1_10`

## `func`
Function to call asynchronously from another thread
## `user_data`
Data to pass to `func`
## `destroy_notify`
GDestroyNotify for `user_data`
<!-- trait ElementExt::fn change_state -->
Perform `transition` on `self`.

This function must be called with STATE_LOCK held and is mainly used
internally.
## `transition`
the requested transition

# Returns

the `StateChangeReturn` of the state transition.
<!-- trait ElementExt::fn continue_state -->
Commit the state change of the element and proceed to the next
pending state if any. This function is used
by elements that do asynchronous state changes.
The core will normally call this method automatically when an
element returned `StateChangeReturn::Success` from the state change function.

If after calling this method the element still has not reached
the pending state, the next state change is performed.

This method is used internally and should normally not be called by plugins
or applications.
## `ret`
The previous state return value

# Returns

The result of the commit state change.

MT safe.
<!-- trait ElementExt::fn create_all_pads -->
Creates a pad for each pad template that is always available.
This function is only useful during object initialization of
subclasses of `Element`.
<!-- trait ElementExt::fn get_base_time -->
Returns the base time of the element. The base time is the
absolute time of the clock when this element was last put to
PLAYING. Subtracting the base time from the clock time gives
the running time of the element.

# Returns

the base time of the element.

MT safe.
<!-- trait ElementExt::fn get_bus -->
Returns the bus of the element. Note that only a `Pipeline` will provide a
bus for the application.

# Returns

the element's `Bus`. unref after usage.

MT safe.
<!-- trait ElementExt::fn get_clock -->
Gets the currently configured clock of the element. This is the clock as was
last set with `ElementExt::set_clock`.

Elements in a pipeline will only have their clock set when the
pipeline is in the PLAYING state.

# Returns

the `Clock` of the element. unref after usage.

MT safe.
<!-- trait ElementExt::fn get_compatible_pad -->
Looks for an unlinked pad to which the given pad can link. It is not
guaranteed that linking the pads will work, though it should work in most
cases.

This function will first attempt to find a compatible unlinked ALWAYS pad,
and if none can be found, it will request a compatible REQUEST pad by looking
at the templates of `self`.
## `pad`
the `Pad` to find a compatible one for.
## `caps`
the `Caps` to use as a filter.

# Returns

the `Pad` to which a link
 can be made, or `None` if one cannot be found. `GstObjectExt::unref`
 after usage.
<!-- trait ElementExt::fn get_compatible_pad_template -->
Retrieves a pad template from `self` that is compatible with `compattempl`.
Pads from compatible templates can be linked together.
## `compattempl`
the `PadTemplate` to find a compatible
 template for

# Returns

a compatible `PadTemplate`,
 or `None` if none was found. No unreferencing is necessary.
<!-- trait ElementExt::fn get_context -->
Gets the context with `context_type` set on the element or NULL.

MT safe.
## `context_type`
a name of a context to retrieve

# Returns

A `Context` or NULL
<!-- trait ElementExt::fn get_context_unlocked -->
Gets the context with `context_type` set on the element or NULL.
## `context_type`
a name of a context to retrieve

# Returns

A `Context` or NULL
<!-- trait ElementExt::fn get_contexts -->
Gets the contexts set on the element.

MT safe.

# Returns

List of `Context`
<!-- trait ElementExt::fn get_factory -->
Retrieves the factory that was used to create this element.

# Returns

the `ElementFactory` used for creating this
 element. no refcounting is needed.
<!-- trait ElementExt::fn get_request_pad -->
Retrieves a pad from the element by name (e.g. "src_\%d"). This version only
retrieves request pads. The pad should be released with
`ElementExt::release_request_pad`.

This method is slower than manually getting the pad template and calling
`ElementExt::request_pad` if the pads should have a specific name (e.g.
`name` is "src_1" instead of "src_\%u").
## `name`
the name of the request `Pad` to retrieve.

# Returns

requested `Pad` if found,
 otherwise `None`. Release after usage.
<!-- trait ElementExt::fn get_start_time -->
Returns the start time of the element. The start time is the
running time of the clock when this element was last put to PAUSED.

Usually the start_time is managed by a toplevel element such as
`Pipeline`.

MT safe.

# Returns

the start time of the element.
<!-- trait ElementExt::fn get_state -->
Gets the state of the element.

For elements that performed an ASYNC state change, as reported by
`ElementExt::set_state`, this function will block up to the
specified timeout value for the state change to complete.
If the element completes the state change or goes into
an error, this function returns immediately with a return value of
`StateChangeReturn::Success` or `StateChangeReturn::Failure` respectively.

For elements that did not return `StateChangeReturn::Async`, this function
returns the current and pending state immediately.

This function returns `StateChangeReturn::NoPreroll` if the element
successfully changed its state but is not able to provide data yet.
This mostly happens for live sources that only produce data in
`State::Playing`. While the state change return is equivalent to
`StateChangeReturn::Success`, it is returned to the application to signal that
some sink elements might not be able to complete their state change because
an element is not producing data to complete the preroll. When setting the
element to playing, the preroll will complete and playback will start.
## `state`
a pointer to `State` to hold the state.
 Can be `None`.
## `pending`
a pointer to `State` to hold the pending
 state. Can be `None`.
## `timeout`
a `ClockTime` to specify the timeout for an async
 state change or `GST_CLOCK_TIME_NONE` for infinite timeout.

# Returns

`StateChangeReturn::Success` if the element has no more pending state
 and the last state change succeeded, `StateChangeReturn::Async` if the
 element is still performing a state change or
 `StateChangeReturn::Failure` if the last state change failed.

MT safe.
<!-- trait ElementExt::fn get_static_pad -->
Retrieves a pad from `self` by name. This version only retrieves
already-existing (i.e. 'static') pads.
## `name`
the name of the static `Pad` to retrieve.

# Returns

the requested `Pad` if
 found, otherwise `None`. unref after usage.

MT safe.
<!-- trait ElementExt::fn is_locked_state -->
Checks if the state of an element is locked.
If the state of an element is locked, state changes of the parent don't
affect the element.
This way you can leave currently unused elements inside bins. Just lock their
state before changing the state from `State::Null`.

MT safe.

# Returns

`true`, if the element's state is locked.
<!-- trait ElementExt::fn iterate_pads -->
Retrieves an iterator of `self`'s pads. The iterator should
be freed after usage. Also more specialized iterators exists such as
`ElementExt::iterate_src_pads` or `ElementExt::iterate_sink_pads`.

The order of pads returned by the iterator will be the order in which
the pads were added to the element.

# Returns

the `Iterator` of `Pad`.

MT safe.
<!-- trait ElementExt::fn iterate_sink_pads -->
Retrieves an iterator of `self`'s sink pads.

The order of pads returned by the iterator will be the order in which
the pads were added to the element.

# Returns

the `Iterator` of `Pad`.

MT safe.
<!-- trait ElementExt::fn iterate_src_pads -->
Retrieves an iterator of `self`'s source pads.

The order of pads returned by the iterator will be the order in which
the pads were added to the element.

# Returns

the `Iterator` of `Pad`.

MT safe.
<!-- trait ElementExt::fn link -->
Links `self` to `dest`. The link must be from source to
destination; the other direction will not be tried. The function looks for
existing pads that aren't linked yet. It will request new pads if necessary.
Such pads need to be released manually when unlinking.
If multiple links are possible, only one is established.

Make sure you have added your elements to a bin or pipeline with
`BinExt::add` before trying to link them.
## `dest`
the `Element` containing the destination pad.

# Returns

`true` if the elements could be linked, `false` otherwise.
<!-- trait ElementExt::fn link_filtered -->
Links `self` to `dest` using the given caps as filtercaps.
The link must be from source to
destination; the other direction will not be tried. The function looks for
existing pads that aren't linked yet. It will request new pads if necessary.
If multiple links are possible, only one is established.

Make sure you have added your elements to a bin or pipeline with
`BinExt::add` before trying to link them.
## `dest`
the `Element` containing the destination pad.
## `filter`
the `Caps` to filter the link,
 or `None` for no filter.

# Returns

`true` if the pads could be linked, `false` otherwise.
<!-- trait ElementExt::fn link_many -->
Chain together a series of elements. Uses `ElementExt::link`.
Make sure you have added your elements to a bin or pipeline with
`BinExt::add` before trying to link them.
## `element_2`
the second `Element` in the link chain.

# Returns

`true` on success, `false` otherwise.
<!-- trait ElementExt::fn link_pads -->
Links the two named pads of the source and destination elements.
Side effect is that if one of the pads has no parent, it becomes a
child of the parent of the other element. If they have different
parents, the link fails.
## `srcpadname`
the name of the `Pad` in source element
 or `None` for any pad.
## `dest`
the `Element` containing the destination pad.
## `destpadname`
the name of the `Pad` in destination element,
or `None` for any pad.

# Returns

`true` if the pads could be linked, `false` otherwise.
<!-- trait ElementExt::fn link_pads_filtered -->
Links the two named pads of the source and destination elements. Side effect
is that if one of the pads has no parent, it becomes a child of the parent of
the other element. If they have different parents, the link fails. If `caps`
is not `None`, makes sure that the caps of the link is a subset of `caps`.
## `srcpadname`
the name of the `Pad` in source element
 or `None` for any pad.
## `dest`
the `Element` containing the destination pad.
## `destpadname`
the name of the `Pad` in destination element
 or `None` for any pad.
## `filter`
the `Caps` to filter the link,
 or `None` for no filter.

# Returns

`true` if the pads could be linked, `false` otherwise.
<!-- trait ElementExt::fn link_pads_full -->
Links the two named pads of the source and destination elements.
Side effect is that if one of the pads has no parent, it becomes a
child of the parent of the other element. If they have different
parents, the link fails.

Calling `ElementExt::link_pads_full` with `flags` == `PadLinkCheck::Default`
is the same as calling `ElementExt::link_pads` and the recommended way of
linking pads with safety checks applied.

This is a convenience function for `PadExt::link_full`.
## `srcpadname`
the name of the `Pad` in source element
 or `None` for any pad.
## `dest`
the `Element` containing the destination pad.
## `destpadname`
the name of the `Pad` in destination element,
or `None` for any pad.
## `flags`
the `PadLinkCheck` to be performed when linking pads.

# Returns

`true` if the pads could be linked, `false` otherwise.
<!-- trait ElementExt::fn lost_state -->
Brings the element to the lost state. The current state of the
element is copied to the pending state so that any call to
`ElementExt::get_state` will return `StateChangeReturn::Async`.

An ASYNC_START message is posted. If the element was PLAYING, it will
go to PAUSED. The element will be restored to its PLAYING state by
the parent pipeline when it prerolls again.

This is mostly used for elements that lost their preroll buffer
in the `State::Paused` or `State::Playing` state after a flush,
they will go to their pending state again when a new preroll buffer is
queued. This function can only be called when the element is currently
not in error or an async state change.

This function is used internally and should normally not be called from
plugins or applications.
<!-- trait ElementExt::fn message_full -->
Post an error, warning or info message on the bus from inside an element.

`type_` must be of `MessageType::Error`, `MessageType::Warning` or
`MessageType::Info`.

MT safe.
## `type_`
the `MessageType`
## `domain`
the GStreamer GError domain this message belongs to
## `code`
the GError code belonging to the domain
## `text`
an allocated text string to be used
 as a replacement for the default message connected to code,
 or `None`
## `debug`
an allocated debug message to be
 used as a replacement for the default debugging information,
 or `None`
## `file`
the source code file where the error was generated
## `function`
the source code function where the error was generated
## `line`
the source code line where the error was generated
<!-- trait ElementExt::fn message_full_with_details -->
Post an error, warning or info message on the bus from inside an element.

`type_` must be of `MessageType::Error`, `MessageType::Warning` or
`MessageType::Info`.

Feature: `v1_10`

## `type_`
the `MessageType`
## `domain`
the GStreamer GError domain this message belongs to
## `code`
the GError code belonging to the domain
## `text`
an allocated text string to be used
 as a replacement for the default message connected to code,
 or `None`
## `debug`
an allocated debug message to be
 used as a replacement for the default debugging information,
 or `None`
## `file`
the source code file where the error was generated
## `function`
the source code function where the error was generated
## `line`
the source code line where the error was generated
## `structure`
optional details structure
<!-- trait ElementExt::fn no_more_pads -->
Use this function to signal that the element does not expect any more pads
to show up in the current pipeline. This function should be called whenever
pads have been added by the element itself. Elements with `PadPresence::Sometimes`
pad templates use this in combination with autopluggers to figure out that
the element is done initializing its pads.

This function emits the `Element::no-more-pads` signal.

MT safe.
<!-- trait ElementExt::fn post_message -->
Post a message on the element's `Bus`. This function takes ownership of the
message; if you want to access the message after this call, you should add an
additional reference before calling.
## `message`
a `Message` to post

# Returns

`true` if the message was successfully posted. The function returns
`false` if the element did not have a bus.

MT safe.
<!-- trait ElementExt::fn provide_clock -->
Get the clock provided by the given element.
> An element is only required to provide a clock in the PAUSED
> state. Some elements can provide a clock in other states.

# Returns

the GstClock provided by the
element or `None` if no clock could be provided. Unref after usage.

MT safe.
<!-- trait ElementExt::fn query -->
Performs a query on the given element.

For elements that don't implement a query handler, this function
forwards the query to a random srcpad or to the peer of a
random linked sinkpad of this element.

Please note that some queries might need a running pipeline to work.
## `query`
the `Query`.

# Returns

`true` if the query could be performed.

MT safe.
<!-- trait ElementExt::fn query_convert -->
Queries an element to convert `src_val` in `src_format` to `dest_format`.
## `src_format`
a `Format` to convert from.
## `src_val`
a value to convert.
## `dest_format`
the `Format` to convert to.
## `dest_val`
a pointer to the result.

# Returns

`true` if the query could be performed.
<!-- trait ElementExt::fn query_duration -->
Queries an element (usually top-level pipeline or playbin element) for the
total stream duration in nanoseconds. This query will only work once the
pipeline is prerolled (i.e. reached PAUSED or PLAYING state). The application
will receive an ASYNC_DONE message on the pipeline bus when that is the case.

If the duration changes for some reason, you will get a DURATION_CHANGED
message on the pipeline bus, in which case you should re-query the duration
using this function.
## `format`
the `Format` requested
## `duration`
A location in which to store the total duration, or `None`.

# Returns

`true` if the query could be performed.
<!-- trait ElementExt::fn query_position -->
Queries an element (usually top-level pipeline or playbin element) for the
stream position in nanoseconds. This will be a value between 0 and the
stream duration (if the stream duration is known). This query will usually
only work once the pipeline is prerolled (i.e. reached PAUSED or PLAYING
state). The application will receive an ASYNC_DONE message on the pipeline
bus when that is the case.

If one repeatedly calls this function one can also create a query and reuse
it in `Element::query`.
## `format`
the `Format` requested
## `cur`
a location in which to store the current
 position, or `None`.

# Returns

`true` if the query could be performed.
<!-- trait ElementExt::fn release_request_pad -->
Makes the element free the previously requested pad as obtained
with `ElementExt::request_pad`.

This does not unref the pad. If the pad was created by using
`ElementExt::request_pad`, `ElementExt::release_request_pad` needs to be
followed by `GstObjectExt::unref` to free the `pad`.

MT safe.
## `pad`
the `Pad` to release.
<!-- trait ElementExt::fn remove_pad -->
Removes `pad` from `self`. `pad` will be destroyed if it has not been
referenced elsewhere using `GstObjectExt::unparent`.

This function is used by plugin developers and should not be used
by applications. Pads that were dynamically requested from elements
with `ElementExt::request_pad` should be released with the
`ElementExt::release_request_pad` function instead.

Pads are not automatically deactivated so elements should perform the needed
steps to deactivate the pad in case this pad is removed in the PAUSED or
PLAYING state. See `PadExt::set_active` for more information about
deactivating pads.

The pad and the element should be unlocked when calling this function.

This function will emit the `Element::pad-removed` signal on the element.
## `pad`
the `Pad` to remove from the element.

# Returns

`true` if the pad could be removed. Can return `false` if the
pad does not belong to the provided element.

MT safe.
<!-- trait ElementExt::fn remove_property_notify_watch -->

Feature: `v1_10`

## `watch_id`
watch id to remove
<!-- trait ElementExt::fn request_pad -->
Retrieves a request pad from the element according to the provided template.
Pad templates can be looked up using
`ElementFactory::get_static_pad_templates`.

The pad should be released with `ElementExt::release_request_pad`.
## `templ`
a `PadTemplate` of which we want a pad of.
## `name`
the name of the request `Pad`
to retrieve. Can be `None`.
## `caps`
the caps of the pad we want to
request. Can be `None`.

# Returns

requested `Pad` if found,
 otherwise `None`. Release after usage.
<!-- trait ElementExt::fn seek -->
Sends a seek event to an element. See `Event::new_seek` for the details of
the parameters. The seek event is sent to the element using
`Element::send_event`.

MT safe.
## `rate`
The new playback rate
## `format`
The format of the seek values
## `flags`
The optional seek flags.
## `start_type`
The type and flags for the new start position
## `start`
The value of the new start position
## `stop_type`
The type and flags for the new stop position
## `stop`
The value of the new stop position

# Returns

`true` if the event was handled. Flushing seeks will trigger a
preroll, which will emit `MessageType::AsyncDone`.
<!-- trait ElementExt::fn seek_simple -->
Simple API to perform a seek on the given element, meaning it just seeks
to the given position relative to the start of the stream. For more complex
operations like segment seeks (e.g. for looping) or changing the playback
rate or seeking relative to the last configured playback segment you should
use `ElementExt::seek`.

In a completely prerolled PAUSED or PLAYING pipeline, seeking is always
guaranteed to return `true` on a seekable media type or `false` when the media
type is certainly not seekable (such as a live stream).

Some elements allow for seeking in the READY state, in this
case they will store the seek event and execute it when they are put to
PAUSED. If the element supports seek in READY, it will always return `true` when
it receives the event in the READY state.
## `format`
a `Format` to execute the seek in, such as `Format::Time`
## `seek_flags`
seek options; playback applications will usually want to use
 GST_SEEK_FLAG_FLUSH | GST_SEEK_FLAG_KEY_UNIT here
## `seek_pos`
position to seek to (relative to the start); if you are doing
 a seek in `Format::Time` this value is in nanoseconds -
 multiply with `GST_SECOND` to convert seconds to nanoseconds or
 with `GST_MSECOND` to convert milliseconds to nanoseconds.

# Returns

`true` if the seek operation succeeded. Flushing seeks will trigger a
preroll, which will emit `MessageType::AsyncDone`.
<!-- trait ElementExt::fn send_event -->
Sends an event to an element. If the element doesn't implement an
event handler, the event will be pushed on a random linked sink pad for
downstream events or a random linked source pad for upstream events.

This function takes ownership of the provided event so you should
`gst_event_ref` it if you want to reuse the event after this call.

MT safe.
## `event`
the `Event` to send to the element.

# Returns

`true` if the event was handled. Events that trigger a preroll (such
as flushing seeks and steps) will emit `MessageType::AsyncDone`.
<!-- trait ElementExt::fn set_base_time -->
Set the base time of an element. See `ElementExt::get_base_time`.

MT safe.
## `time`
the base time to set.
<!-- trait ElementExt::fn set_bus -->
Sets the bus of the element. Increases the refcount on the bus.
For internal use only, unless you're testing elements.

MT safe.
## `bus`
the `Bus` to set.
<!-- trait ElementExt::fn set_clock -->
Sets the clock for the element. This function increases the
refcount on the clock. Any previously set clock on the object
is unreffed.
## `clock`
the `Clock` to set for the element.

# Returns

`true` if the element accepted the clock. An element can refuse a
clock when it, for example, is not able to slave its internal clock to the
`clock` or when it requires a specific clock to operate.

MT safe.
<!-- trait ElementExt::fn set_context -->
Sets the context of the element. Increases the refcount of the context.

MT safe.
## `context`
the `Context` to set.
<!-- trait ElementExt::fn set_locked_state -->
Locks the state of an element, so state changes of the parent don't affect
this element anymore.

MT safe.
## `locked_state`
`true` to lock the element's state

# Returns

`true` if the state was changed, `false` if bad parameters were given
or the elements state-locking needed no change.
<!-- trait ElementExt::fn set_start_time -->
Set the start time of an element. The start time of the element is the
running time of the element when it last went to the PAUSED state. In READY
or after a flushing seek, it is set to 0.

Toplevel elements like `Pipeline` will manage the start_time and
base_time on its children. Setting the start_time to `GST_CLOCK_TIME_NONE`
on such a toplevel element will disable the distribution of the base_time to
the children and can be useful if the application manages the base_time
itself, for example if you want to synchronize capture from multiple
pipelines, and you can also ensure that the pipelines have the same clock.

MT safe.
## `time`
the base time to set.
<!-- trait ElementExt::fn set_state -->
Sets the state of the element. This function will try to set the
requested state by going through all the intermediary states and calling
the class's state change function for each.

This function can return `StateChangeReturn::Async`, in which case the
element will perform the remainder of the state change asynchronously in
another thread.
An application can use `ElementExt::get_state` to wait for the completion
of the state change or it can wait for a `MessageType::AsyncDone` or
`MessageType::StateChanged` on the bus.

State changes to `State::Ready` or `State::Null` never return
`StateChangeReturn::Async`.
## `state`
the element's new `State`.

# Returns

Result of the state change using `StateChangeReturn`.

MT safe.
<!-- trait ElementExt::fn sync_state_with_parent -->
Tries to change the state of the element to the same as its parent.
If this function returns `false`, the state of element is undefined.

# Returns

`true`, if the element's state could be synced to the parent's state.

MT safe.
<!-- trait ElementExt::fn unlink -->
Unlinks all source pads of the source element with all sink pads
of the sink element to which they are linked.

If the link has been made using `ElementExt::link`, it could have created an
requestpad, which has to be released using `ElementExt::release_request_pad`.
## `dest`
the sink `Element` to unlink.
<!-- trait ElementExt::fn unlink_many -->
Unlinks a series of elements. Uses `ElementExt::unlink`.
## `element_2`
the second `Element` in the link chain.
<!-- trait ElementExt::fn unlink_pads -->
Unlinks the two named pads of the source and destination elements.

This is a convenience function for `PadExt::unlink`.
## `srcpadname`
the name of the `Pad` in source element.
## `dest`
a `Element` containing the destination pad.
## `destpadname`
the name of the `Pad` in destination element.
<!-- struct ElementFactory -->
`ElementFactory` is used to create instances of elements. A
GstElementFactory can be added to a `Plugin` as it is also a
`PluginFeature`.

Use the `ElementFactory::find` and `ElementFactory::create`
functions to create element instances or use `ElementFactory::make` as a
convenient shortcut.

The following code example shows you how to create a GstFileSrc element.

## Using an element factory

```C
  #include &lt;gst/gst.h&gt;

  GstElement *src;
  GstElementFactory *srcfactory;

  gst_init (&amp;argc, &amp;argv);

  srcfactory = gst_element_factory_find ("filesrc");
  g_return_if_fail (srcfactory != NULL);
  src = gst_element_factory_create (srcfactory, "src");
  g_return_if_fail (src != NULL);
  ...
```

# Implements

[`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- impl ElementFactory::fn find -->
Search for an element factory of the given name. Refs the returned
element factory; caller is responsible for unreffing.
## `name`
name of factory to find

# Returns

`ElementFactory` if found,
`None` otherwise
<!-- impl ElementFactory::fn list_filter -->
Filter out all the elementfactories in `list` that can handle `caps` in
the given direction.

If `subsetonly` is `true`, then only the elements whose pads templates
are a complete superset of `caps` will be returned. Else any element
whose pad templates caps can intersect with `caps` will be returned.
## `list`
a `glib::List` of
 `ElementFactory` to filter
## `caps`
a `Caps`
## `direction`
a `PadDirection` to filter on
## `subsetonly`
whether to filter on caps subsets or not.

# Returns

a `glib::List` of
 `ElementFactory` elements that match the given requisites.
 Use `PluginFeature::list_free` after usage.
<!-- impl ElementFactory::fn list_get_elements -->
Get a list of factories that match the given `type_`. Only elements
with a rank greater or equal to `minrank` will be returned.
The list of factories is returned by decreasing rank.
## `type_`
a `ElementFactoryListType`
## `minrank`
Minimum rank

# Returns

a `glib::List` of
 `ElementFactory` elements. Use `PluginFeature::list_free` after
 usage.
<!-- impl ElementFactory::fn make -->
Create a new element of the type defined by the given element factory.
If name is `None`, then the element will receive a guaranteed unique name,
consisting of the element factory name and a number.
If name is given, it will be given the name supplied.
## `factoryname`
a named factory to instantiate
## `name`
name of new element, or `None` to automatically create
 a unique name

# Returns

new `Element` or `None`
if unable to create element
<!-- impl ElementFactory::fn can_sink_all_caps -->
Checks if the factory can sink all possible capabilities.
## `caps`
the caps to check

# Returns

`true` if the caps are fully compatible.
<!-- impl ElementFactory::fn can_sink_any_caps -->
Checks if the factory can sink any possible capability.
## `caps`
the caps to check

# Returns

`true` if the caps have a common subset.
<!-- impl ElementFactory::fn can_src_all_caps -->
Checks if the factory can src all possible capabilities.
## `caps`
the caps to check

# Returns

`true` if the caps are fully compatible.
<!-- impl ElementFactory::fn can_src_any_caps -->
Checks if the factory can src any possible capability.
## `caps`
the caps to check

# Returns

`true` if the caps have a common subset.
<!-- impl ElementFactory::fn create -->
Create a new element of the type defined by the given elementfactory.
It will be given the name supplied, since all elements require a name as
their first argument.
## `name`
name of new element, or `None` to automatically create
 a unique name

# Returns

new `Element` or `None`
 if the element couldn't be created
<!-- impl ElementFactory::fn get_element_type -->
Get the `glib::Type` for elements managed by this factory. The type can
only be retrieved if the element factory is loaded, which can be
assured with `PluginFeature::load`.

# Returns

the `glib::Type` for elements managed by this factory or 0 if
the factory is not loaded.
<!-- impl ElementFactory::fn get_metadata -->
Get the metadata on `self` with `key`.
## `key`
a key

# Returns

the metadata with `key` on `self` or `None`
when there was no metadata with the given `key`.
<!-- impl ElementFactory::fn get_metadata_keys -->
Get the available keys for the metadata on `self`.

# Returns


a `None`-terminated array of key strings, or `None` when there is no
metadata. Free with `g_strfreev` when no longer needed.
<!-- impl ElementFactory::fn get_num_pad_templates -->
Gets the number of pad_templates in this factory.

# Returns

the number of pad_templates
<!-- impl ElementFactory::fn get_static_pad_templates -->
Gets the `glib::List` of `StaticPadTemplate` for this factory.

# Returns

the
 static pad templates
<!-- impl ElementFactory::fn get_uri_protocols -->
Gets a `None`-terminated array of protocols this element supports or `None` if
no protocols are supported. You may not change the contents of the returned
array, as it is still owned by the element factory. Use `g_strdupv` to
make a copy of the protocol string array if you need to.

# Returns

the supported protocols
 or `None`
<!-- impl ElementFactory::fn get_uri_type -->
Gets the type of URIs the element supports or `URIType::Unknown` if none.

# Returns

type of URIs this element supports
<!-- impl ElementFactory::fn has_interface -->
Check if `self` implements the interface with name `interfacename`.
## `interfacename`
an interface name

# Returns

`true` when `self` implement the interface.
<!-- impl ElementFactory::fn list_is_type -->
Check if `self` is of the given types.
## `type_`
a `ElementFactoryListType`

# Returns

`true` if `self` is of `type_`.
<!-- struct Event -->
The event class provides factory methods to construct events for sending
and functions to query (parse) received events.

Events are usually created with gst_event_new_*() which takes event-type
specific parameters as arguments.
To send an event application will usually use `Element::send_event` and
elements will use `Pad::send_event` or `Pad::push_event`.
The event should be unreffed with `gst_event_unref` if it has not been sent.

Events that have been received can be parsed with their respective
gst_event_parse_*() functions. It is valid to pass `None` for unwanted details.

Events are passed between elements in parallel to the data stream. Some events
are serialized with buffers, others are not. Some events only travel downstream,
others only upstream. Some events can travel both upstream and downstream.

The events are used to signal special conditions in the datastream such as
EOS (end of stream) or the start of a new stream-segment.
Events are also used to flush the pipeline of any pending data.

Most of the event API is used inside plugins. Applications usually only
construct and use seek events.
To do that `Event::new_seek` is used to create a seek event. It takes
the needed parameters to specify seeking time and mode.

```C
  GstEvent *event;
  gboolean result;
  ...
  // construct a seek event to play the media from second 2 to 5, flush
  // the pipeline to decrease latency.
  event = gst_event_new_seek (1.0,
     GST_FORMAT_TIME,
     GST_SEEK_FLAG_FLUSH,
     GST_SEEK_TYPE_SET, 2 * GST_SECOND,
     GST_SEEK_TYPE_SET, 5 * GST_SECOND);
  ...
  result = gst_element_send_event (pipeline, event);
  if (!result)
    g_warning ("seek failed");
  ...
```
<!-- impl Event::fn new_buffer_size -->
Create a new buffersize event. The event is sent downstream and notifies
elements that they should provide a buffer of the specified dimensions.

When the `async` flag is set, a thread boundary is preferred.
## `format`
buffer format
## `minsize`
minimum buffer size
## `maxsize`
maximum buffer size
## `async`
thread behavior

# Returns

a new `Event`
<!-- impl Event::fn new_caps -->
Create a new CAPS event for `caps`. The caps event can only travel downstream
synchronized with the buffer flow and contains the format of the buffers
that will follow after the event.
## `caps`
a `Caps`

# Returns

the new CAPS event.
<!-- impl Event::fn new_custom -->
Create a new custom-typed event. This can be used for anything not
handled by other event-specific functions to pass an event to another
element.

Make sure to allocate an event type with the `GST_EVENT_MAKE_TYPE` macro,
assigning a free number and filling in the correct direction and
serialization flags.

New custom events can also be created by subclassing the event type if
needed.
## `type_`
The type of the new event
## `structure`
the structure for the event. The event will
 take ownership of the structure.

# Returns

the new custom event.
<!-- impl Event::fn new_eos -->
Create a new EOS event. The eos event can only travel downstream
synchronized with the buffer flow. Elements that receive the EOS
event on a pad can return `FlowReturn::Eos` as a `FlowReturn`
when data after the EOS event arrives.

The EOS event will travel down to the sink elements in the pipeline
which will then post the `MessageType::Eos` on the bus after they have
finished playing any buffered data.

When all sinks have posted an EOS message, an EOS message is
forwarded to the application.

The EOS event itself will not cause any state transitions of the pipeline.

# Returns

the new EOS event.
<!-- impl Event::fn new_flush_start -->
Allocate a new flush start event. The flush start event can be sent
upstream and downstream and travels out-of-bounds with the dataflow.

It marks pads as being flushing and will make them return
`FlowReturn::Flushing` when used for data flow with `Pad::push`,
`Pad::chain`, `Pad::get_range` and `Pad::pull_range`.
Any event (except a `EventType::FlushStop`) received
on a flushing pad will return `false` immediately.

Elements should unlock any blocking functions and exit their streaming
functions as fast as possible when this event is received.

This event is typically generated after a seek to flush out all queued data
in the pipeline so that the new media is played as soon as possible.

# Returns

a new flush start event.
<!-- impl Event::fn new_flush_stop -->
Allocate a new flush stop event. The flush stop event can be sent
upstream and downstream and travels serialized with the dataflow.
It is typically sent after sending a FLUSH_START event to make the
pads accept data again.

Elements can process this event synchronized with the dataflow since
the preceding FLUSH_START event stopped the dataflow.

This event is typically generated to complete a seek and to resume
dataflow.
## `reset_time`
if time should be reset

# Returns

a new flush stop event.
<!-- impl Event::fn new_gap -->
Create a new GAP event. A gap event can be thought of as conceptually
equivalent to a buffer to signal that there is no data for a certain
amount of time. This is useful to signal a gap to downstream elements
which may wait for data, such as muxers or mixers or overlays, especially
for sparse streams such as subtitle streams.
## `timestamp`
the start time (pts) of the gap
## `duration`
the duration of the gap

# Returns

the new GAP event.
<!-- impl Event::fn new_latency -->
Create a new latency event. The event is sent upstream from the sinks and
notifies elements that they should add an additional `latency` to the
running time before synchronising against the clock.

The latency is mostly used in live sinks and is always expressed in
the time format.
## `latency`
the new latency value

# Returns

a new `Event`
<!-- impl Event::fn new_navigation -->
Create a new navigation event from the given description.
## `structure`
description of the event. The event will take
 ownership of the structure.

# Returns

a new `Event`
<!-- impl Event::fn new_protection -->
Creates a new event containing information specific to a particular
protection system (uniquely identified by `system_id`), by which that
protection system can acquire key(s) to decrypt a protected stream.

In order for a decryption element to decrypt media
protected using a specific system, it first needs all the
protection system specific information necessary to acquire the decryption
key(s) for that stream. The functions defined here enable this information
to be passed in events from elements that extract it
(e.g., ISOBMFF demuxers, MPEG DASH demuxers) to protection decrypter
elements that use it.

Events containing protection system specific information are created using
`Event::new_protection`, and they can be parsed by downstream elements
using `Event::parse_protection`.

In Common Encryption, protection system specific information may be located
within ISOBMFF files, both in movie (moov) boxes and movie fragment (moof)
boxes; it may also be contained in ContentProtection elements within MPEG
DASH MPDs. The events created by `Event::new_protection` contain data
identifying from which of these locations the encapsulated protection system
specific information originated. This origin information is required as
some protection systems use different encodings depending upon where the
information originates.

The events returned by `Event::new_protection` are implemented
in such a way as to ensure that the most recently-pushed protection info
event of a particular `origin` and `system_id` will
be stuck to the output pad of the sending element.
## `system_id`
a string holding a UUID that uniquely
identifies a protection system.
## `data`
a `Buffer` holding protection system specific
information. The reference count of the buffer will be incremented by one.
## `origin`
a string indicating where the protection
information carried in the event was extracted from. The allowed values
of this string will depend upon the protection scheme.

# Returns

a `EventType::Protection` event, if successful; `None`
if unsuccessful.
<!-- impl Event::fn new_qos -->
Allocate a new qos event with the given values.
The QOS event is generated in an element that wants an upstream
element to either reduce or increase its rate because of
high/low CPU load or other resource usage such as network performance or
throttling. Typically sinks generate these events for each buffer
they receive.

`type_` indicates the reason for the QoS event. `QOSType::Overflow` is
used when a buffer arrived in time or when the sink cannot keep up with
the upstream datarate. `QOSType::Underflow` is when the sink is not
receiving buffers fast enough and thus has to drop late buffers.
`QOSType::Throttle` is used when the datarate is artificially limited
by the application, for example to reduce power consumption.

`proportion` indicates the real-time performance of the streaming in the
element that generated the QoS event (usually the sink). The value is
generally computed based on more long term statistics about the streams
timestamps compared to the clock.
A value < 1.0 indicates that the upstream element is producing data faster
than real-time. A value > 1.0 indicates that the upstream element is not
producing data fast enough. 1.0 is the ideal `proportion` value. The
proportion value can safely be used to lower or increase the quality of
the element.

`diff` is the difference against the clock in running time of the last
buffer that caused the element to generate the QOS event. A negative value
means that the buffer with `timestamp` arrived in time. A positive value
indicates how late the buffer with `timestamp` was. When throttling is
enabled, `diff` will be set to the requested throttling interval.

`timestamp` is the timestamp of the last buffer that cause the element
to generate the QOS event. It is expressed in running time and thus an ever
increasing value.

The upstream element can use the `diff` and `timestamp` values to decide
whether to process more buffers. For positive `diff`, all buffers with
timestamp <= `timestamp` + `diff` will certainly arrive late in the sink
as well. A (negative) `diff` value so that `timestamp` + `diff` would yield a
result smaller than 0 is not allowed.

The application can use general event probes to intercept the QoS
event and implement custom application specific QoS handling.
## `type_`
the QoS type
## `proportion`
the proportion of the qos message
## `diff`
The time difference of the last Clock sync
## `timestamp`
The timestamp of the buffer

# Returns

a new QOS event.
<!-- impl Event::fn new_reconfigure -->
Create a new reconfigure event. The purpose of the reconfigure event is
to travel upstream and make elements renegotiate their caps or reconfigure
their buffer pools. This is useful when changing properties on elements
or changing the topology of the pipeline.

# Returns

a new `Event`
<!-- impl Event::fn new_seek -->
Allocate a new seek event with the given parameters.

The seek event configures playback of the pipeline between `start` to `stop`
at the speed given in `rate`, also called a playback segment.
The `start` and `stop` values are expressed in `format`.

A `rate` of 1.0 means normal playback rate, 2.0 means double speed.
Negatives values means backwards playback. A value of 0.0 for the
rate is not allowed and should be accomplished instead by PAUSING the
pipeline.

A pipeline has a default playback segment configured with a start
position of 0, a stop position of -1 and a rate of 1.0. The currently
configured playback segment can be queried with `QueryType::Segment`.

`start_type` and `stop_type` specify how to adjust the currently configured
start and stop fields in playback segment. Adjustments can be made relative
or absolute to the last configured values. A type of `SeekType::None`
means that the position should not be updated.

When the rate is positive and `start` has been updated, playback will start
from the newly configured start position.

For negative rates, playback will start from the newly configured stop
position (if any). If the stop position is updated, it must be different from
-1 (`GST_CLOCK_TIME_NONE`) for negative rates.

It is not possible to seek relative to the current playback position, to do
this, PAUSE the pipeline, query the current playback position with
`QueryType::Position` and update the playback segment current position with a
`SeekType::Set` to the desired position.
## `rate`
The new playback rate
## `format`
The format of the seek values
## `flags`
The optional seek flags
## `start_type`
The type and flags for the new start position
## `start`
The value of the new start position
## `stop_type`
The type and flags for the new stop position
## `stop`
The value of the new stop position

# Returns

a new seek event.
<!-- impl Event::fn new_segment -->
Create a new SEGMENT event for `segment`. The segment event can only travel
downstream synchronized with the buffer flow and contains timing information
and playback properties for the buffers that will follow.

The segment event marks the range of buffers to be processed. All
data not within the segment range is not to be processed. This can be
used intelligently by plugins to apply more efficient methods of skipping
unneeded data. The valid range is expressed with the `start` and `stop`
values.

The time value of the segment is used in conjunction with the start
value to convert the buffer timestamps into the stream time. This is
usually done in sinks to report the current stream_time.
`time` represents the stream_time of a buffer carrying a timestamp of
`start`. `time` cannot be -1.

`start` cannot be -1, `stop` can be -1. If there
is a valid `stop` given, it must be greater or equal the `start`, including
when the indicated playback `rate` is < 0.

The `applied_rate` value provides information about any rate adjustment that
has already been made to the timestamps and content on the buffers of the
stream. (`rate` * `applied_rate`) should always equal the rate that has been
requested for playback. For example, if an element has an input segment
with intended playback `rate` of 2.0 and applied_rate of 1.0, it can adjust
incoming timestamps and buffer content by half and output a segment event
with `rate` of 1.0 and `applied_rate` of 2.0

After a segment event, the buffer stream time is calculated with:

 time + (TIMESTAMP(buf) - start) * ABS (rate * applied_rate)
## `segment`
a `Segment`

# Returns

the new SEGMENT event.
<!-- impl Event::fn new_segment_done -->
Create a new segment-done event. This event is sent by elements that
finish playback of a segment as a result of a segment seek.
## `format`
The format of the position being done
## `position`
The position of the segment being done

# Returns

a new `Event`
<!-- impl Event::fn new_select_streams -->
Allocate a new select-streams event.

The select-streams event requests the specified `streams` to be activated.

The list of `streams` corresponds to the "Stream ID" of each stream to be
activated. Those ID can be obtained via the `Stream` objects present
in `EventType::StreamStart`, `EventType::StreamCollection` or
`GST_MESSSAGE_STREAM_COLLECTION`.

Feature: `v1_10`

## `streams`
the list of streams to
activate

# Returns

a new select-streams event.
<!-- impl Event::fn new_sink_message -->
Create a new sink-message event. The purpose of the sink-message event is
to instruct a sink to post the message contained in the event synchronized
with the stream.

`name` is used to store multiple sticky events on one pad.
## `name`
a name for the event
## `msg`
the `Message` to be posted

# Returns

a new `Event`
<!-- impl Event::fn new_step -->
Create a new step event. The purpose of the step event is to instruct a sink
to skip `amount` (expressed in `format`) of media. It can be used to implement
stepping through the video frame by frame or for doing fast trick modes.

A rate of <= 0.0 is not allowed. Pause the pipeline, for the effect of rate
= 0.0 or first reverse the direction of playback using a seek event to get
the same effect as rate < 0.0.

The `flush` flag will clear any pending data in the pipeline before starting
the step operation.

The `intermediate` flag instructs the pipeline that this step operation is
part of a larger step operation.
## `format`
the format of `amount`
## `amount`
the amount of data to step
## `rate`
the step rate
## `flush`
flushing steps
## `intermediate`
intermediate steps

# Returns

a new `Event`
<!-- impl Event::fn new_stream_collection -->
Create a new STREAM_COLLECTION event. The stream collection event can only
travel downstream synchronized with the buffer flow.

Source elements, demuxers and other elements that manage collections
of streams and post `StreamCollection` messages on the bus also send
this event downstream on each pad involved in the collection, so that
activation of a new collection can be tracked through the downstream
data flow.

Feature: `v1_10`

## `collection`
Active collection for this data flow

# Returns

the new STREAM_COLLECTION event.
<!-- impl Event::fn new_stream_group_done -->
Create a new Stream Group Done event. The stream-group-done event can
only travel downstream synchronized with the buffer flow. Elements
that receive the event on a pad should handle it mostly like EOS,
and emit any data or pending buffers that would depend on more data
arriving and unblock, since there won't be any more data.

This event is followed by EOS at some point in the future, and is
generally used when switching pads - to unblock downstream so that
new pads can be exposed before sending EOS on the existing pads.

Feature: `v1_10`

## `group_id`
the group id of the stream group which is ending

# Returns

the new stream-group-done event.
<!-- impl Event::fn new_stream_start -->
Create a new STREAM_START event. The stream start event can only
travel downstream synchronized with the buffer flow. It is expected
to be the first event that is sent for a new stream.

Source elements, demuxers and other elements that create new streams
are supposed to send this event as the first event of a new stream. It
should not be sent after a flushing seek or in similar situations
and is used to mark the beginning of a new logical stream. Elements
combining multiple streams must ensure that this event is only forwarded
downstream once and not for every single input stream.

The `stream_id` should be a unique string that consists of the upstream
stream-id, / as separator and a unique stream-id for this specific
stream. A new stream-id should only be created for a stream if the upstream
stream is split into (potentially) multiple new streams, e.g. in a demuxer,
but not for every single element in the pipeline.
`PadExt::create_stream_id` or `PadExt::create_stream_id_printf` can be
used to create a stream-id. There are no particular semantics for the
stream-id, though it should be deterministic (to support stream matching)
and it might be used to order streams (besides any information conveyed by
stream flags).
## `stream_id`
Identifier for this stream

# Returns

the new STREAM_START event.
<!-- impl Event::fn new_tag -->
Generates a metadata tag event from the given `taglist`.

The scope of the taglist specifies if the taglist applies to the
complete medium or only to this specific stream. As the tag event
is a sticky event, elements should merge tags received from
upstream with a given scope with their own tags with the same
scope and create a new tag event from it.
## `taglist`
metadata list. The event will take ownership
 of the taglist.

# Returns

a new `Event`
<!-- impl Event::fn new_toc -->
Generate a TOC event from the given `toc`. The purpose of the TOC event is to
inform elements that some kind of the TOC was found.
## `toc`
`Toc` structure.
## `updated`
whether `toc` was updated or not.

# Returns

a new `Event`.
<!-- impl Event::fn new_toc_select -->
Generate a TOC select event with the given `uid`. The purpose of the
TOC select event is to start playback based on the TOC's entry with the
given `uid`.
## `uid`
UID in the TOC to start playback from.

# Returns

a new `Event`.
<!-- impl Event::fn copy_segment -->
Parses a segment `self` and copies the `Segment` into the location
given by `segment`.
## `segment`
a pointer to a `Segment`
<!-- impl Event::fn get_running_time_offset -->
Retrieve the accumulated running time offset of the event.

Events passing through `GstPads` that have a running time
offset set via `PadExt::set_offset` will get their offset
adjusted according to the pad's offset.

If the event contains any information that related to the
running time, this information will need to be updated
before usage with this offset.

# Returns

The event's running time offset

MT safe.
<!-- impl Event::fn get_seqnum -->
Retrieve the sequence number of a event.

Events have ever-incrementing sequence numbers, which may also be set
explicitly via `Event::set_seqnum`. Sequence numbers are typically used to
indicate that a event corresponds to some other set of events or messages,
for example an EOS event corresponding to a SEEK event. It is considered good
practice to make this correspondence when possible, though it is not
required.

Note that events and messages share the same sequence number incrementor;
two events or messages will never have the same sequence number unless
that correspondence was made explicitly.

# Returns

The event's sequence number.

MT safe.
<!-- impl Event::fn get_structure -->
Access the structure of the event.

# Returns

The structure of the event. The structure is still
owned by the event, which means that you should not free it and
that the pointer becomes invalid when you free the event.

MT safe.
<!-- impl Event::fn has_name -->
Checks if `self` has the given `name`. This function is usually used to
check the name of a custom event.
## `name`
name to check

# Returns

`true` if `name` matches the name of the event structure.
<!-- impl Event::fn parse_buffer_size -->
Get the format, minsize, maxsize and async-flag in the buffersize event.
## `format`
A pointer to store the format in
## `minsize`
A pointer to store the minsize in
## `maxsize`
A pointer to store the maxsize in
## `async`
A pointer to store the async-flag in
<!-- impl Event::fn parse_caps -->
Get the caps from `self`. The caps remains valid as long as `self` remains
valid.
## `caps`
A pointer to the caps
<!-- impl Event::fn parse_flush_stop -->
Parse the FLUSH_STOP event and retrieve the `reset_time` member.
## `reset_time`
if time should be reset
<!-- impl Event::fn parse_gap -->
Extract timestamp and duration from a new GAP event.
## `timestamp`
location where to store the
 start time (pts) of the gap, or `None`
## `duration`
location where to store the duration of
 the gap, or `None`
<!-- impl Event::fn parse_group_id -->
## `group_id`
address of variable where to store the group id

# Returns

`true` if a group id was set on the event and could be parsed,
 `false` otherwise.
<!-- impl Event::fn parse_latency -->
Get the latency in the latency event.
## `latency`
A pointer to store the latency in.
<!-- impl Event::fn parse_protection -->
Parses an event containing protection system specific information and stores
the results in `system_id`, `data` and `origin`. The data stored in `system_id`,
`origin` and `data` are valid until `self` is released.
## `system_id`
pointer to store the UUID
string uniquely identifying a content protection system.
## `data`
pointer to store a `Buffer`
holding protection system specific information.
## `origin`
pointer to store a value that
indicates where the protection information carried by `self` was extracted
from.
<!-- impl Event::fn parse_qos -->
Get the type, proportion, diff and timestamp in the qos event. See
`Event::new_qos` for more information about the different QoS values.

`timestamp` will be adjusted for any pad offsets of pads it was passing through.
## `type_`
A pointer to store the QoS type in
## `proportion`
A pointer to store the proportion in
## `diff`
A pointer to store the diff in
## `timestamp`
A pointer to store the timestamp in
<!-- impl Event::fn parse_seek -->
Parses a seek `self` and stores the results in the given result locations.
## `rate`
result location for the rate
## `format`
result location for the stream format
## `flags`
result location for the `SeekFlags`
## `start_type`
result location for the `SeekType` of the start position
## `start`
result location for the start position expressed in `format`
## `stop_type`
result location for the `SeekType` of the stop position
## `stop`
result location for the stop position expressed in `format`
<!-- impl Event::fn parse_segment -->
Parses a segment `self` and stores the result in the given `segment` location.
`segment` remains valid only until the `self` is freed. Don't modify the segment
and make a copy if you want to modify it or store it for later use.
## `segment`
a pointer to a `Segment`
<!-- impl Event::fn parse_segment_done -->
Extracts the position and format from the segment done message.
## `format`
Result location for the format, or `None`
## `position`
Result location for the position, or `None`
<!-- impl Event::fn parse_select_streams -->
Parse the SELECT_STREAMS event and retrieve the contained streams.

Feature: `v1_10`

## `streams`
the streams
<!-- impl Event::fn parse_sink_message -->
Parse the sink-message event. Unref `msg` after usage.
## `msg`
a pointer to store the `Message` in.
<!-- impl Event::fn parse_step -->
Parse the step event.
## `format`
a pointer to store the format in
## `amount`
a pointer to store the amount in
## `rate`
a pointer to store the rate in
## `flush`
a pointer to store the flush boolean in
## `intermediate`
a pointer to store the intermediate
 boolean in
<!-- impl Event::fn parse_stream -->
Parse a stream-start `self` and extract the `Stream` from it.

Feature: `v1_10`

## `stream`
adress of variable to store the stream
<!-- impl Event::fn parse_stream_collection -->
Retrieve new `StreamCollection` from STREAM_COLLECTION event `self`.

Feature: `v1_10`

## `collection`
pointer to store the collection
<!-- impl Event::fn parse_stream_flags -->
## `flags`
address of variable where to store the stream flags
<!-- impl Event::fn parse_stream_group_done -->
Parse a stream-group-done `self` and store the result in the given
`group_id` location.

Feature: `v1_10`

## `group_id`
address of variable to store the group id into
<!-- impl Event::fn parse_stream_start -->
Parse a stream-id `self` and store the result in the given `stream_id`
location. The string stored in `stream_id` must not be modified and will
remain valid only until `self` gets freed. Make a copy if you want to
modify it or store it for later use.
## `stream_id`
pointer to store the stream-id
<!-- impl Event::fn parse_tag -->
Parses a tag `self` and stores the results in the given `taglist` location.
No reference to the taglist will be returned, it remains valid only until
the `self` is freed. Don't modify or free the taglist, make a copy if you
want to modify it or store it for later use.
## `taglist`
pointer to metadata list
<!-- impl Event::fn parse_toc -->
Parse a TOC `self` and store the results in the given `toc` and `updated` locations.
## `toc`
pointer to `Toc` structure.
## `updated`
pointer to store TOC updated flag.
<!-- impl Event::fn parse_toc_select -->
Parse a TOC select `self` and store the results in the given `uid` location.
## `uid`
storage for the selection UID.
<!-- impl Event::fn set_group_id -->
All streams that have the same group id are supposed to be played
together, i.e. all streams inside a container file should have the
same group id but different stream ids. The group id should change
each time the stream is started, resulting in different group ids
each time a file is played for example.

Use `gst_util_group_id_next` to get a new group id.
## `group_id`
the group id to set
<!-- impl Event::fn set_running_time_offset -->
Set the running time offset of a event. See
`Event::get_running_time_offset` for more information.

MT safe.
## `offset`
A the new running time offset
<!-- impl Event::fn set_seqnum -->
Set the sequence number of a event.

This function might be called by the creator of a event to indicate that the
event relates to other events or messages. See `Event::get_seqnum` for
more information.

MT safe.
## `seqnum`
A sequence number.
<!-- impl Event::fn set_stream -->
Set the `stream` on the stream-start `self`

Feature: `v1_10`

## `stream`
the stream object to set
<!-- impl Event::fn set_stream_flags -->
## `flags`
the stream flags to set
<!-- impl Event::fn writable_structure -->
Get a writable version of the structure.

# Returns

The structure of the event. The structure
is still owned by the event, which means that you should not free
it and that the pointer becomes invalid when you free the event.
This function checks if `self` is writable and will never return
`None`.

MT safe.
<!-- enum EventType -->
`EventType` lists the standard event types that can be sent in a pipeline.

The custom event types can be used for private messages between elements
that can't be expressed using normal
GStreamer buffer passing semantics. Custom events carry an arbitrary
`Structure`.
Specific custom events are distinguished by the name of the structure.
<!-- enum EventType::variant Unknown -->
unknown event.
<!-- enum EventType::variant FlushStart -->
Start a flush operation. This event clears all data
 from the pipeline and unblock all streaming threads.
<!-- enum EventType::variant FlushStop -->
Stop a flush operation. This event resets the
 running-time of the pipeline.
<!-- enum EventType::variant StreamStart -->
Event to mark the start of a new stream. Sent before any
 other serialized event and only sent at the start of a new stream,
 not after flushing seeks.
<!-- enum EventType::variant Caps -->
`Caps` event. Notify the pad of a new media type.
<!-- enum EventType::variant Segment -->
A new media segment follows in the dataflow. The
 segment events contains information for clipping buffers and
 converting buffer timestamps to running-time and
 stream-time.
<!-- enum EventType::variant StreamCollection -->
A new `StreamCollection` is available (Since 1.10)
<!-- enum EventType::variant Tag -->
A new set of metadata tags has been found in the stream.
<!-- enum EventType::variant Buffersize -->
Notification of buffering requirements. Currently not
 used yet.
<!-- enum EventType::variant SinkMessage -->
An event that sinks turn into a message. Used to
 send messages that should be emitted in sync with
 rendering.
<!-- enum EventType::variant StreamGroupDone -->
Indicates that there is no more data for
 the stream group ID in the message. Sent before EOS
 in some instances and should be handled mostly the same. (Since 1.10)
<!-- enum EventType::variant Eos -->
End-Of-Stream. No more data is to be expected to follow
 without either a STREAM_START event, or a FLUSH_STOP and a SEGMENT
 event.
<!-- enum EventType::variant Toc -->
An event which indicates that a new table of contents (TOC)
 was found or updated.
<!-- enum EventType::variant Protection -->
An event which indicates that new or updated
 encryption information has been found in the stream.
<!-- enum EventType::variant SegmentDone -->
Marks the end of a segment playback.
<!-- enum EventType::variant Gap -->
Marks a gap in the datastream.
<!-- enum EventType::variant Qos -->
A quality message. Used to indicate to upstream elements
 that the downstream elements should adjust their processing
 rate.
<!-- enum EventType::variant Seek -->
A request for a new playback position and rate.
<!-- enum EventType::variant Navigation -->
Navigation events are usually used for communicating
 user requests, such as mouse or keyboard movements,
 to upstream elements.
<!-- enum EventType::variant Latency -->
Notification of new latency adjustment. Sinks will use
 the latency information to adjust their synchronisation.
<!-- enum EventType::variant Step -->
A request for stepping through the media. Sinks will usually
 execute the step operation.
<!-- enum EventType::variant Reconfigure -->
A request for upstream renegotiating caps and reconfiguring.
<!-- enum EventType::variant TocSelect -->
A request for a new playback position based on TOC
 entry's UID.
<!-- enum EventType::variant SelectStreams -->
A request to select one or more streams (Since 1.10)
<!-- enum EventType::variant CustomUpstream -->
Upstream custom event
<!-- enum EventType::variant CustomDownstream -->
Downstream custom event that travels in the
 data flow.
<!-- enum EventType::variant CustomDownstreamOob -->
Custom out-of-band downstream event.
<!-- enum EventType::variant CustomDownstreamSticky -->
Custom sticky downstream event.
<!-- enum EventType::variant CustomBoth -->
Custom upstream or downstream event.
 In-band when travelling downstream.
<!-- enum EventType::variant CustomBothOob -->
Custom upstream or downstream out-of-band event.
<!-- enum FlowReturn -->
The result of passing data to a pad.

Note that the custom return values should not be exposed outside of the
element scope.
<!-- enum FlowReturn::variant CustomSuccess2 -->
Pre-defined custom success code.
<!-- enum FlowReturn::variant CustomSuccess1 -->
Pre-defined custom success code (define your
 custom success code to this to avoid compiler
 warnings).
<!-- enum FlowReturn::variant CustomSuccess -->
Elements can use values starting from
 this (and higher) to define custom success
 codes.
<!-- enum FlowReturn::variant Ok -->
Data passing was ok.
<!-- enum FlowReturn::variant NotLinked -->
Pad is not linked.
<!-- enum FlowReturn::variant Flushing -->
Pad is flushing.
<!-- enum FlowReturn::variant Eos -->
Pad is EOS.
<!-- enum FlowReturn::variant NotNegotiated -->
Pad is not negotiated.
<!-- enum FlowReturn::variant Error -->
Some (fatal) error occurred. Element generating
 this error should post an error message with more
 details.
<!-- enum FlowReturn::variant NotSupported -->
This operation is not supported.
<!-- enum FlowReturn::variant CustomError -->
Elements can use values starting from
 this (and lower) to define custom error codes.
<!-- enum FlowReturn::variant CustomError1 -->
Pre-defined custom error code (define your
 custom error code to this to avoid compiler
 warnings).
<!-- enum FlowReturn::variant CustomError2 -->
Pre-defined custom error code.
<!-- enum Format -->
Standard predefined formats
<!-- enum Format::variant Undefined -->
undefined format
<!-- enum Format::variant Default -->
the default format of the pad/element. This can be
 samples for raw audio, frames/fields for raw video (some, but not all,
 elements support this; use `Format::Time` if you don't have a good
 reason to query for samples/frames)
<!-- enum Format::variant Bytes -->
bytes
<!-- enum Format::variant Time -->
time in nanoseconds
<!-- enum Format::variant Buffers -->
buffers (few, if any, elements implement this as of
 May 2009)
<!-- enum Format::variant Percent -->
percentage of stream (few, if any, elements implement
 this as of May 2009)
<!-- struct GhostPad -->
GhostPads are useful when organizing pipelines with `Bin` like elements.
The idea here is to create hierarchical element graphs. The bin element
contains a sub-graph. Now one would like to treat the bin-element like any
other `Element`. This is where GhostPads come into play. A GhostPad acts as
a proxy for another pad. Thus the bin can have sink and source ghost-pads
that are associated with sink and source pads of the child elements.

If the target pad is known at creation time, `GhostPad::new` is the
function to use to get a ghost-pad. Otherwise one can use `GhostPad::new_no_target`
to create the ghost-pad and use `GhostPadExt::set_target` to establish the
association later on.

Note that GhostPads add overhead to the data processing of a pipeline.

# Implements

[`GhostPadExt`](trait.GhostPadExt.html), [`ProxyPadExt`](trait.ProxyPadExt.html), [`PadExt`](trait.PadExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait GhostPadExt -->
Trait containing all `GhostPad` methods.

# Implementors

[`GhostPad`](struct.GhostPad.html)
<!-- impl GhostPad::fn new -->
Create a new ghostpad with `target` as the target. The direction will be taken
from the target pad. `target` must be unlinked.

Will ref the target.
## `name`
the name of the new pad, or `None` to assign a default name
## `target`
the pad to ghost.

# Returns

a new `Pad`, or `None` in
case of an error.
<!-- impl GhostPad::fn new_from_template -->
Create a new ghostpad with `target` as the target. The direction will be taken
from the target pad. The template used on the ghostpad will be `template`.

Will ref the target.
## `name`
the name of the new pad, or `None` to assign a default name.
## `target`
the pad to ghost.
## `templ`
the `PadTemplate` to use on the ghostpad.

# Returns

a new `Pad`, or `None` in
case of an error.
<!-- impl GhostPad::fn new_no_target -->
Create a new ghostpad without a target with the given direction.
A target can be set on the ghostpad later with the
`GhostPadExt::set_target` function.

The created ghostpad will not have a padtemplate.
## `name`
the name of the new pad, or `None` to assign a default name.
## `dir`
the direction of the ghostpad

# Returns

a new `Pad`, or `None` in
case of an error.
<!-- impl GhostPad::fn new_no_target_from_template -->
Create a new ghostpad based on `templ`, without setting a target. The
direction will be taken from the `templ`.
## `name`
the name of the new pad, or `None` to assign a default name
## `templ`
the `PadTemplate` to create the ghostpad from.

# Returns

a new `Pad`, or `None` in
case of an error.
<!-- impl GhostPad::fn activate_mode_default -->
Invoke the default activate mode function of a ghost pad.
## `pad`
the `Pad` to activate or deactivate.
## `parent`
the parent of `pad` or `None`
## `mode`
the requested activation mode
## `active`
whether the pad should be active or not.

# Returns

`true` if the operation was successful.
<!-- impl GhostPad::fn internal_activate_mode_default -->
Invoke the default activate mode function of a proxy pad that is
owned by a ghost pad.
## `pad`
the `Pad` to activate or deactivate.
## `parent`
the parent of `pad` or `None`
## `mode`
the requested activation mode
## `active`
whether the pad should be active or not.

# Returns

`true` if the operation was successful.
<!-- trait GhostPadExt::fn construct -->
Finish initialization of a newly allocated ghost pad.

This function is most useful in language bindings and when subclassing
`GhostPad`; plugin and application developers normally will not call this
function. Call this function directly after a call to g_object_new
(GST_TYPE_GHOST_PAD, "direction", `dir`, ..., NULL).

# Returns

`true` if the construction succeeds, `false` otherwise.
<!-- trait GhostPadExt::fn get_target -->
Get the target pad of `self`. Unref target pad after usage.

# Returns

the target `Pad`, can be
`None` if the ghostpad has no target set. Unref target pad after
usage.
<!-- trait GhostPadExt::fn set_target -->
Set the new target of the ghostpad `self`. Any existing target
is unlinked and links to the new target are established. if `newtarget` is
`None` the target will be cleared.
## `newtarget`
the new pad target

# Returns

`true` if the new target could be set. This function
 can return `false` when the internal pads could not be linked.
<!-- struct Iterator -->
A GstIterator is used to retrieve multiple objects from another object in
a threadsafe way.

Various GStreamer objects provide access to their internal structures using
an iterator.

Note that if calling a GstIterator function results in your code receiving
a refcounted object (with, say, `gobject::Value::get_object`), the refcount for that
object will not be increased. Your code is responsible for taking a reference
if it wants to continue using it later.

The basic use pattern of an iterator is as follows:

```C
  GstIterator *it = _get_iterator(object);
  GValue item = G_VALUE_INIT;
  done = FALSE;
  while (!done) {
    switch (gst_iterator_next (it, &amp;item)) {
      case GST_ITERATOR_OK:
        ...get/use/change item here...
        g_value_reset (&amp;item);
        break;
      case GST_ITERATOR_RESYNC:
        ...rollback changes to items...
        gst_iterator_resync (it);
        break;
      case GST_ITERATOR_ERROR:
        ...wrong parameters were given...
        done = TRUE;
        break;
      case GST_ITERATOR_DONE:
        done = TRUE;
        break;
    }
  }
  g_value_unset (&amp;item);
  gst_iterator_free (it);
```
<!-- impl Iterator::fn new -->
Create a new iterator. This function is mainly used for objects
implementing the next/resync/free function to iterate a data structure.

For each item retrieved, the `item` function is called with the lock
held. The `free` function is called when the iterator is freed.
## `size`
the size of the iterator structure
## `type_`
`glib::Type` of children
## `lock`
pointer to a `GMutex`.
## `master_cookie`
pointer to a guint32 that is changed when the items in the
 iterator changed.
## `copy`
copy function
## `next`
function to get next item
## `item`
function to call on each item retrieved
## `resync`
function to resync the iterator
## `free`
function to free the iterator

# Returns

the new `Iterator`.

MT safe.
<!-- impl Iterator::fn new_list -->
Create a new iterator designed for iterating `list`.

The list you iterate is usually part of a data structure `owner` and is
protected with `lock`.

The iterator will use `lock` to retrieve the next item of the list and it
will then call the `item` function before releasing `lock` again.

When a concurrent update to the list is performed, usually by `owner` while
holding `lock`, `master_cookie` will be updated. The iterator implementation
will notice the update of the cookie and will return `IteratorResult::Resync` to
the user of the iterator in the next call to `Iterator::next`.
## `type_`
`glib::Type` of elements
## `lock`
pointer to a `GMutex` protecting the list.
## `master_cookie`
pointer to a guint32 that is incremented when the list
 is changed.
## `list`
pointer to the list
## `owner`
object owning the list
## `item`
function to call on each item retrieved

# Returns

the new `Iterator` for `list`.

MT safe.
<!-- impl Iterator::fn new_single -->
This `Iterator` is a convenient iterator for the common
case where a `Iterator` needs to be returned but only
a single object has to be considered. This happens often
for the `GstPadIterIntLinkFunction`.
## `type_`
`glib::Type` of the passed object
## `object`
object that this iterator should return

# Returns

the new `Iterator` for `object`.
<!-- impl Iterator::fn copy -->
Copy the iterator and its state.

# Returns

a new copy of `self`.
<!-- impl Iterator::fn filter -->
Create a new iterator from an existing iterator. The new iterator
will only return those elements that match the given compare function `func`.
The first parameter that is passed to `func` is the `gobject::Value` of the current
iterator element and the second parameter is `user_data`. `func` should
return 0 for elements that should be included in the filtered iterator.

When this iterator is freed, `self` will also be freed.
## `func`
the compare function to select elements
## `user_data`
user data passed to the compare function

# Returns

a new `Iterator`.

MT safe.
<!-- impl Iterator::fn find_custom -->
Find the first element in `self` that matches the compare function `func`.
`func` should return 0 when the element is found. The first parameter
to `func` will be the current element of the iterator and the
second parameter will be `user_data`.
The result will be stored in `elem` if a result is found.

The iterator will not be freed.

This function will return `false` if an error happened to the iterator
or if the element wasn't found.
## `func`
the compare function to use
## `elem`
pointer to a `gobject::Value` where to store the result
## `user_data`
user data passed to the compare function

# Returns

Returns `true` if the element was found, else `false`.

MT safe.
<!-- impl Iterator::fn fold -->
Folds `func` over the elements of `iter`. That is to say, `func` will be called
as `func` (object, `ret`, `user_data`) for each object in `self`. The normal use
of this procedure is to accumulate the results of operating on the objects in
`ret`.

This procedure can be used (and is used internally) to implement the
`Iterator::foreach` and `Iterator::find_custom` operations.

The fold will proceed as long as `func` returns `true`. When the iterator has no
more arguments, `IteratorResult::Done` will be returned. If `func` returns `false`,
the fold will stop, and `IteratorResult::Ok` will be returned. Errors or resyncs
will cause fold to return `IteratorResult::Error` or `IteratorResult::Resync` as
appropriate.

The iterator will not be freed.
## `func`
the fold function
## `ret`
the seed value passed to the fold function
## `user_data`
user data passed to the fold function

# Returns

A `IteratorResult`, as described above.

MT safe.
<!-- impl Iterator::fn foreach -->
Iterate over all element of `self` and call the given function `func` for
each element.
## `func`
the function to call for each element.
## `user_data`
user data passed to the function

# Returns

the result call to `Iterator::fold`. The iterator will not be
freed.

MT safe.
<!-- impl Iterator::fn free -->
Free the iterator.

MT safe.
<!-- impl Iterator::fn next -->
Get the next item from the iterator in `elem`.

Only when this function returns `IteratorResult::Ok`, `elem` will contain a valid
value. `elem` must have been initialized to the type of the iterator or
initialized to zeroes with `gobject::Value::unset`. The caller is responsible for
unsetting or resetting `elem` with `gobject::Value::unset` or `gobject::Value::reset`
after usage.

When this function returns `IteratorResult::Done`, no more elements can be
retrieved from `self`.

A return value of `IteratorResult::Resync` indicates that the element list was
concurrently updated. The user of `self` should call `Iterator::resync` to
get the newly updated list.

A return value of `IteratorResult::Error` indicates an unrecoverable fatal error.
## `elem`
pointer to hold next element

# Returns

The result of the iteration. Unset `elem` after usage.

MT safe.
<!-- impl Iterator::fn push -->
Pushes `other` iterator onto `self`. All calls performed on `self` are
forwarded to `other`. If `other` returns `IteratorResult::Done`, it is
popped again and calls are handled by `self` again.

This function is mainly used by objects implementing the iterator
next function to recurse into substructures.

When `Iterator::resync` is called on `self`, `other` will automatically be
popped.

MT safe.
## `other`
The `Iterator` to push
<!-- impl Iterator::fn resync -->
Resync the iterator. this function is mostly called
after `Iterator::next` returned `IteratorResult::Resync`.

When an iterator was pushed on `self`, it will automatically be popped again
with this function.

MT safe.
<!-- enum IteratorResult -->
The result of `Iterator::next`.
<!-- enum IteratorResult::variant Done -->
No more items in the iterator
<!-- enum IteratorResult::variant Ok -->
An item was retrieved
<!-- enum IteratorResult::variant Resync -->
Datastructure changed while iterating
<!-- enum IteratorResult::variant Error -->
An error happened
<!-- enum LibraryError -->
Library errors are for errors from the library being used by elements
(initializing, finalizing, settings, ...)
<!-- enum LibraryError::variant Failed -->
a general error which doesn't fit in any other
category. Make sure you add a custom message to the error call.
<!-- enum LibraryError::variant TooLazy -->
do not use this except as a placeholder for
deciding where to go while developing code.
<!-- enum LibraryError::variant Init -->
used when the library could not be opened.
<!-- enum LibraryError::variant Shutdown -->
used when the library could not be closed.
<!-- enum LibraryError::variant Settings -->
used when the library doesn't accept settings.
<!-- enum LibraryError::variant Encode -->
used when the library generated an encoding error.
<!-- enum LibraryError::variant NumErrors -->
the number of library error types.
<!-- struct Message -->
Messages are implemented as a subclass of `MiniObject` with a generic
`Structure` as the content. This allows for writing custom messages without
requiring an API change while allowing a wide range of different types
of messages.

Messages are posted by objects in the pipeline and are passed to the
application using the `Bus`.

The basic use pattern of posting a message on a `Bus` is as follows:

```C
  gst_bus_post (bus, gst_message_new_eos());
```

A `Element` usually posts messages on the bus provided by the parent
container using `ElementExt::post_message`.
<!-- impl Message::fn new_application -->
Create a new application-typed message. GStreamer will never create these
messages; they are a gift from us to you. Enjoy.
## `src`
The object originating the message.
## `structure`
the structure for the message. The message
 will take ownership of the structure.

# Returns

The new application message.

MT safe.
<!-- impl Message::fn new_async_done -->
The message is posted when elements completed an ASYNC state change.
`running_time` contains the time of the desired running_time when this
elements goes to PLAYING. A value of `GST_CLOCK_TIME_NONE` for `running_time`
means that the element has no clock interaction and thus doesn't care about
the running_time of the pipeline.
## `src`
The object originating the message.
## `running_time`
the desired running_time

# Returns

The new async_done message.

MT safe.
<!-- impl Message::fn new_async_start -->
This message is posted by elements when they start an ASYNC state change.
## `src`
The object originating the message.

# Returns

The new async_start message.

MT safe.
<!-- impl Message::fn new_buffering -->
Create a new buffering message. This message can be posted by an element that
needs to buffer data before it can continue processing. `percent` should be a
value between 0 and 100. A value of 100 means that the buffering completed.

When `percent` is < 100 the application should PAUSE a PLAYING pipeline. When
`percent` is 100, the application can set the pipeline (back) to PLAYING.
The application must be prepared to receive BUFFERING messages in the
PREROLLING state and may only set the pipeline to PLAYING after receiving a
message with `percent` set to 100, which can happen after the pipeline
completed prerolling.

MT safe.
## `src`
The object originating the message.
## `percent`
The buffering percent

# Returns

The new buffering message.
<!-- impl Message::fn new_clock_lost -->
Create a clock lost message. This message is posted whenever the
clock is not valid anymore.

If this message is posted by the pipeline, the pipeline will
select a new clock again when it goes to PLAYING. It might therefore
be needed to set the pipeline to PAUSED and PLAYING again.
## `src`
The object originating the message.
## `clock`
the clock that was lost

# Returns

The new clock lost message.

MT safe.
<!-- impl Message::fn new_clock_provide -->
Create a clock provide message. This message is posted whenever an
element is ready to provide a clock or lost its ability to provide
a clock (maybe because it paused or became EOS).

This message is mainly used internally to manage the clock
selection.
## `src`
The object originating the message.
## `clock`
the clock it provides
## `ready`
`true` if the sender can provide a clock

# Returns

the new provide clock message.

MT safe.
<!-- impl Message::fn new_custom -->
Create a new custom-typed message. This can be used for anything not
handled by other message-specific functions to pass a message to the
app. The structure field can be `None`.
## `type_`
The `MessageType` to distinguish messages
## `src`
The object originating the message.
## `structure`
the structure for the
 message. The message will take ownership of the structure.

# Returns

The new message.

MT safe.
<!-- impl Message::fn new_device_added -->
Creates a new device-added message. The device-added message is produced by
`DeviceProvider` or a `DeviceMonitor`. They announce the appearance
of monitored devices.
## `src`
The `Object` that created the message
## `device`
The new `Device`

# Returns

a newly allocated `Message`
<!-- impl Message::fn new_device_removed -->
Creates a new device-removed message. The device-removed message is produced
by `DeviceProvider` or a `DeviceMonitor`. They announce the
disappearance of monitored devices.
## `src`
The `Object` that created the message
## `device`
The removed `Device`

# Returns

a newly allocated `Message`
<!-- impl Message::fn new_duration_changed -->
Create a new duration changed message. This message is posted by elements
that know the duration of a stream when the duration changes. This message
is received by bins and is used to calculate the total duration of a
pipeline.
## `src`
The object originating the message.

# Returns

The new duration-changed message.

MT safe.
<!-- impl Message::fn new_element -->
Create a new element-specific message. This is meant as a generic way of
allowing one-way communication from an element to an application, for example
"the firewire cable was unplugged". The format of the message should be
documented in the element's documentation. The structure field can be `None`.
## `src`
The object originating the message.
## `structure`
The structure for the
 message. The message will take ownership of the structure.

# Returns

The new element message.

MT safe.
<!-- impl Message::fn new_eos -->
Create a new eos message. This message is generated and posted in
the sink elements of a GstBin. The bin will only forward the EOS
message to the application if all sinks have posted an EOS message.
## `src`
The object originating the message.

# Returns

The new eos message.

MT safe.
<!-- impl Message::fn new_error -->
Create a new error message. The message will copy `error` and
`debug`. This message is posted by element when a fatal event
occurred. The pipeline will probably (partially) stop. The application
receiving this message should stop the pipeline.
## `src`
The object originating the message.
## `error`
The GError for this message.
## `debug`
A debugging string.

# Returns

the new error message.

MT safe.
<!-- impl Message::fn new_error_with_details -->
Create a new error message. The message will copy `error` and
`debug`. This message is posted by element when a fatal event
occurred. The pipeline will probably (partially) stop. The application
receiving this message should stop the pipeline.

Feature: `v1_10`

## `src`
The object originating the message.
## `error`
The GError for this message.
## `debug`
A debugging string.
## `details`
(allow-none): A GstStructure with details

# Returns

the new error message.
<!-- impl Message::fn new_have_context -->
This message is posted when an element has a new local `Context`.
## `src`
The object originating the message.
## `context`
the context

# Returns

The new have-context message.

MT safe.
<!-- impl Message::fn new_info -->
Create a new info message. The message will make copies of `error` and
`debug`.
## `src`
The object originating the message.
## `error`
The GError for this message.
## `debug`
A debugging string.

# Returns

the new info message.

MT safe.
<!-- impl Message::fn new_info_with_details -->
Create a new info message. The message will make copies of `error` and
`debug`.

Feature: `v1_10`

## `src`
The object originating the message.
## `error`
The GError for this message.
## `debug`
A debugging string.
## `details`
(allow-none): A GstStructure with details

# Returns

the new warning message.
<!-- impl Message::fn new_latency -->
This message can be posted by elements when their latency requirements have
changed.
## `src`
The object originating the message.

# Returns

The new latency message.

MT safe.
<!-- impl Message::fn new_need_context -->
This message is posted when an element needs a specific `Context`.
## `src`
The object originating the message.
## `context_type`
The context type that is needed

# Returns

The new need-context message.

MT safe.
<!-- impl Message::fn new_new_clock -->
Create a new clock message. This message is posted whenever the
pipeline selects a new clock for the pipeline.
## `src`
The object originating the message.
## `clock`
the new selected clock

# Returns

The new new clock message.

MT safe.
<!-- impl Message::fn new_progress -->
Progress messages are posted by elements when they use an asynchronous task
to perform actions triggered by a state change.

`code` contains a well defined string describing the action.
`text` should contain a user visible string detailing the current action.
## `src`
The object originating the message.
## `type_`
a `ProgressType`
## `code`
a progress code
## `text`
free, user visible text describing the progress

# Returns

The new qos message.
<!-- impl Message::fn new_property_notify -->

Feature: `v1_10`

## `src`
The `Object` whose property changed (may or may not be a `Element`)
## `property_name`
name of the property that changed
## `val`
new property value, or `None`

# Returns

a newly allocated `Message`
<!-- impl Message::fn new_qos -->
A QOS message is posted on the bus whenever an element decides to drop a
buffer because of QoS reasons or whenever it changes its processing strategy
because of QoS reasons (quality adjustments such as processing at lower
accuracy).

This message can be posted by an element that performs synchronisation against the
clock (live) or it could be dropped by an element that performs QoS because of QOS
events received from a downstream element (!live).

`running_time`, `stream_time`, `timestamp`, `duration` should be set to the
respective running-time, stream-time, timestamp and duration of the (dropped)
buffer that generated the QoS event. Values can be left to
GST_CLOCK_TIME_NONE when unknown.
## `src`
The object originating the message.
## `live`
if the message was generated by a live element
## `running_time`
the running time of the buffer that generated the message
## `stream_time`
the stream time of the buffer that generated the message
## `timestamp`
the timestamps of the buffer that generated the message
## `duration`
the duration of the buffer that generated the message

# Returns

The new qos message.

MT safe.
<!-- impl Message::fn new_redirect -->
Creates a new redirect message and adds a new entry to it. Redirect messages
are posted when an element detects that the actual data has to be retrieved
from a different location. This is useful if such a redirection cannot be
handled inside a source element, for example when HTTP 302/303 redirects
return a non-HTTP URL.

The redirect message can hold multiple entries. The first one is added
when the redirect message is created, with the given location, tag_list,
entry_struct arguments. Use `Message::add_redirect_entry` to add more
entries.

Each entry has a location, a tag list, and a structure. All of these are
optional. The tag list and structure are useful for additional metadata,
such as bitrate statistics for the given location.

By default, message recipients should treat entries in the order they are
stored. The recipient should therefore try entry `0` first, and if this
entry is not acceptable or working, try entry `1` etc. Senders must make
sure that they add entries in this order. However, recipients are free to
ignore the order and pick an entry that is "best" for them. One example
would be a recipient that scans the entries for the one with the highest
bitrate tag.

The specified location string is copied. However, ownership over the tag
list and structure are transferred to the message.

Feature: `v1_10`

## `src`
The `Object` whose property changed (may or may not be a `Element`)
## `location`
location string for the new entry
## `tag_list`
tag list for the new entry
## `entry_struct`
structure for the new entry

# Returns

a newly allocated `Message`
<!-- impl Message::fn new_request_state -->
This message can be posted by elements when they want to have their state
changed. A typical use case would be an audio server that wants to pause the
pipeline because a higher priority stream is being played.
## `src`
The object originating the message.
## `state`
The new requested state

# Returns

the new request state message.

MT safe.
<!-- impl Message::fn new_reset_time -->
This message is posted when the pipeline running-time should be reset to
`running_time`, like after a flushing seek.
## `src`
The object originating the message.
## `running_time`
the requested running-time

# Returns

The new reset_time message.

MT safe.
<!-- impl Message::fn new_segment_done -->
Create a new segment done message. This message is posted by elements that
finish playback of a segment as a result of a segment seek. This message
is received by the application after all elements that posted a segment_start
have posted the segment_done.
## `src`
The object originating the message.
## `format`
The format of the position being done
## `position`
The position of the segment being done

# Returns

the new segment done message.

MT safe.
<!-- impl Message::fn new_segment_start -->
Create a new segment message. This message is posted by elements that
start playback of a segment as a result of a segment seek. This message
is not received by the application but is used for maintenance reasons in
container elements.
## `src`
The object originating the message.
## `format`
The format of the position being played
## `position`
The position of the segment being played

# Returns

the new segment start message.

MT safe.
<!-- impl Message::fn new_state_changed -->
Create a state change message. This message is posted whenever an element
changed its state.
## `src`
The object originating the message.
## `oldstate`
the previous state
## `newstate`
the new (current) state
## `pending`
the pending (target) state

# Returns

the new state change message.

MT safe.
<!-- impl Message::fn new_state_dirty -->
Create a state dirty message. This message is posted whenever an element
changed its state asynchronously and is used internally to update the
states of container objects.
## `src`
The object originating the message

# Returns

the new state dirty message.

MT safe.
<!-- impl Message::fn new_step_done -->
This message is posted by elements when they complete a part, when `intermediate` set
to `true`, or a complete step operation.

`duration` will contain the amount of time (in GST_FORMAT_TIME) of the stepped
`amount` of media in format `format`.
## `src`
The object originating the message.
## `format`
the format of `amount`
## `amount`
the amount of stepped data
## `rate`
the rate of the stepped amount
## `flush`
is this an flushing step
## `intermediate`
is this an intermediate step
## `duration`
the duration of the data
## `eos`
the step caused EOS

# Returns

the new step_done message.

MT safe.
<!-- impl Message::fn new_step_start -->
This message is posted by elements when they accept or activate a new step
event for `amount` in `format`.

`active` is set to `false` when the element accepted the new step event and has
queued it for execution in the streaming threads.

`active` is set to `true` when the element has activated the step operation and
is now ready to start executing the step in the streaming thread. After this
message is emitted, the application can queue a new step operation in the
element.
## `src`
The object originating the message.
## `active`
if the step is active or queued
## `format`
the format of `amount`
## `amount`
the amount of stepped data
## `rate`
the rate of the stepped amount
## `flush`
is this an flushing step
## `intermediate`
is this an intermediate step

# Returns

The new step_start message.

MT safe.
<!-- impl Message::fn new_stream_collection -->
Creates a new stream-collection message. The message is used to announce new
`StreamCollection`

Feature: `v1_10`

## `src`
The `Object` that created the message
## `collection`
The `StreamCollection`

# Returns

a newly allocated `Message`
<!-- impl Message::fn new_stream_start -->
Create a new stream_start message. This message is generated and posted in
the sink elements of a GstBin. The bin will only forward the STREAM_START
message to the application if all sinks have posted an STREAM_START message.
## `src`
The object originating the message.

# Returns

The new stream_start message.

MT safe.
<!-- impl Message::fn new_stream_status -->
Create a new stream status message. This message is posted when a streaming
thread is created/destroyed or when the state changed.
## `src`
The object originating the message.
## `type_`
The stream status type.
## `owner`
the owner element of `src`.

# Returns

the new stream status message.

MT safe.
<!-- impl Message::fn new_streams_selected -->
Creates a new steams-selected message. The message is used to announce
that an array of streams has been selected. This is generally in response
to a `EventType::SelectStreams` event, or when an element (such as decodebin3)
makes an initial selection of streams.

The message also contains the `StreamCollection` to which the various streams
belong to.

Users of `Message::new_streams_selected` can add the selected streams with
`Message::streams_selected_add`.

Feature: `v1_10`

## `src`
The `Object` that created the message
## `collection`
The `StreamCollection`

# Returns

a newly allocated `Message`
<!-- impl Message::fn new_structure_change -->
Create a new structure change message. This message is posted when the
structure of a pipeline is in the process of being changed, for example
when pads are linked or unlinked.

`src` should be the sinkpad that unlinked or linked.
## `src`
The object originating the message.
## `type_`
The change type.
## `owner`
The owner element of `src`.
## `busy`
Whether the structure change is busy.

# Returns

the new structure change message.

MT safe.
<!-- impl Message::fn new_tag -->
Create a new tag message. The message will take ownership of the tag list.
The message is posted by elements that discovered a new taglist.
## `src`
The object originating the message.
## `tag_list`
the tag list for the message.

# Returns

the new tag message.

MT safe.
<!-- impl Message::fn new_toc -->
Create a new TOC message. The message is posted by elements
that discovered or updated a TOC.
## `src`
the object originating the message.
## `toc`
`Toc` structure for the message.
## `updated`
whether TOC was updated or not.

# Returns

a new TOC message.

MT safe.
<!-- impl Message::fn new_warning -->
Create a new warning message. The message will make copies of `error` and
`debug`.
## `src`
The object originating the message.
## `error`
The GError for this message.
## `debug`
A debugging string.

# Returns

the new warning message.

MT safe.
<!-- impl Message::fn new_warning_with_details -->
Create a new warning message. The message will make copies of `error` and
`debug`.

Feature: `v1_10`

## `src`
The object originating the message.
## `error`
The GError for this message.
## `debug`
A debugging string.
## `details`
(allow-none): A GstStructure with details

# Returns

the new warning message.
<!-- impl Message::fn add_redirect_entry -->
Creates and appends a new entry.

The specified location string is copied. However, ownership over the tag
list and structure are transferred to the message.

Feature: `v1_10`

## `location`
location string for the new entry
## `tag_list`
tag list for the new entry
## `entry_struct`
structure for the new entry
<!-- impl Message::fn get_num_redirect_entries -->

Feature: `v1_10`


# Returns

the number of entries stored in the message
<!-- impl Message::fn get_seqnum -->
Retrieve the sequence number of a message.

Messages have ever-incrementing sequence numbers, which may also be set
explicitly via `Message::set_seqnum`. Sequence numbers are typically used
to indicate that a message corresponds to some other set of messages or
events, for example a SEGMENT_DONE message corresponding to a SEEK event. It
is considered good practice to make this correspondence when possible, though
it is not required.

Note that events and messages share the same sequence number incrementor;
two events or messages will never have the same sequence number unless
that correspondence was made explicitly.

# Returns

The message's sequence number.

MT safe.
<!-- impl Message::fn get_stream_status_object -->
Extracts the object managing the streaming thread from `self`.

# Returns

a GValue containing the object that manages the streaming thread.
This object is usually of type GstTask but other types can be added in the
future. The object remains valid as long as `self` is valid.
<!-- impl Message::fn get_structure -->
Access the structure of the message.

# Returns

The structure of the message. The structure is
still owned by the message, which means that you should not free it and
that the pointer becomes invalid when you free the message.

MT safe.
<!-- impl Message::fn has_name -->
Checks if `self` has the given `name`. This function is usually used to
check the name of a custom message.
## `name`
name to check

# Returns

`true` if `name` matches the name of the message structure.
<!-- impl Message::fn parse_async_done -->
Extract the running_time from the async_done message.

MT safe.
## `running_time`
Result location for the running_time or `None`
<!-- impl Message::fn parse_buffering -->
Extracts the buffering percent from the GstMessage. see also
`Message::new_buffering`.

MT safe.
## `percent`
Return location for the percent.
<!-- impl Message::fn parse_buffering_stats -->
Extracts the buffering stats values from `self`.
## `mode`
a buffering mode, or `None`
## `avg_in`
the average input rate, or `None`
## `avg_out`
the average output rate, or `None`
## `buffering_left`
amount of buffering time left in
 milliseconds, or `None`
<!-- impl Message::fn parse_clock_lost -->
Extracts the lost clock from the GstMessage.
The clock object returned remains valid until the message is freed.

MT safe.
## `clock`
a pointer to hold the lost clock
<!-- impl Message::fn parse_clock_provide -->
Extracts the clock and ready flag from the GstMessage.
The clock object returned remains valid until the message is freed.

MT safe.
## `clock`
a pointer to hold a clock
 object, or `None`
## `ready`
a pointer to hold the ready flag, or `None`
<!-- impl Message::fn parse_context_type -->
Parse a context type from an existing GST_MESSAGE_NEED_CONTEXT message.
## `context_type`
the context type, or `None`

# Returns

a `gboolean` indicating if the parsing succeeded.
<!-- impl Message::fn parse_device_added -->
Parses a device-added message. The device-added message is produced by
`DeviceProvider` or a `DeviceMonitor`. It announces the appearance
of monitored devices.
## `device`
A location where to store a
 pointer to the new `Device`, or `None`
<!-- impl Message::fn parse_device_removed -->
Parses a device-removed message. The device-removed message is produced by
`DeviceProvider` or a `DeviceMonitor`. It announces the
disappearance of monitored devices.
## `device`
A location where to store a
 pointer to the removed `Device`, or `None`
<!-- impl Message::fn parse_error -->
Extracts the GError and debug string from the GstMessage. The values returned
in the output arguments are copies; the caller must free them when done.

Typical usage of this function might be:

```C
  ...
  switch (GST_MESSAGE_TYPE (msg)) {
    case GST_MESSAGE_ERROR: {
      GError *err = NULL;
      gchar *dbg_info = NULL;

      gst_message_parse_error (msg, &amp;err, &amp;dbg_info);
      g_printerr ("ERROR from element %s: %s\n",
          GST_OBJECT_NAME (msg->src), err->message);
      g_printerr ("Debugging info: %s\n", (dbg_info) ? dbg_info : "none");
      g_error_free (err);
      g_free (dbg_info);
      break;
    }
    ...
  }
  ...
```

MT safe.
## `gerror`
location for the GError
## `debug`
location for the debug message,
 or `None`
<!-- impl Message::fn parse_error_details -->
Returns the optional details structure, may be NULL if none.
The returned structure must not be freed.

Feature: `v1_10`

## `structure`
A pointer to the returned details
<!-- impl Message::fn parse_group_id -->
Extract the group from the STREAM_START message.
## `group_id`
Result location for the group id or
 `None`

# Returns

`true` if the message had a group id set, `false` otherwise

MT safe.
<!-- impl Message::fn parse_have_context -->
Extract the context from the HAVE_CONTEXT message.

MT safe.
## `context`
Result location for the
 context or `None`
<!-- impl Message::fn parse_info -->
Extracts the GError and debug string from the GstMessage. The values returned
in the output arguments are copies; the caller must free them when done.

MT safe.
## `gerror`
location for the GError
## `debug`
location for the debug message,
 or `None`
<!-- impl Message::fn parse_info_details -->
Returns the optional details structure, may be NULL if none
The returned structure must not be freed.

Feature: `v1_10`

## `structure`
A pointer to the returned details structure
<!-- impl Message::fn parse_new_clock -->
Extracts the new clock from the GstMessage.
The clock object returned remains valid until the message is freed.

MT safe.
## `clock`
a pointer to hold the selected
 new clock
<!-- impl Message::fn parse_progress -->
Parses the progress `type_`, `code` and `text`.
## `type_`
location for the type
## `code`
location for the code
## `text`
location for the text
<!-- impl Message::fn parse_property_notify -->
Parses a property-notify message. These will be posted on the bus only
when set up with `ElementExt::add_property_notify_watch` or
`ElementExt::add_property_deep_notify_watch`.

Feature: `v1_10`

## `object`
location where to store a
 pointer to the object whose property got changed, or `None`
## `property_name`
return location for the name of the
 property that got changed, or `None`
## `property_value`
return location for the new value of
 the property that got changed, or `None`. This will only be set if the
 property notify watch was told to include the value when it was set up
<!-- impl Message::fn parse_qos -->
Extract the timestamps and live status from the QoS message.

The returned values give the running_time, stream_time, timestamp and
duration of the dropped buffer. Values of GST_CLOCK_TIME_NONE mean unknown
values.

MT safe.
## `live`
if the message was generated by a live element
## `running_time`
the running time of the buffer that
 generated the message
## `stream_time`
the stream time of the buffer that
 generated the message
## `timestamp`
the timestamps of the buffer that
 generated the message
## `duration`
the duration of the buffer that
 generated the message
<!-- impl Message::fn parse_qos_stats -->
Extract the QoS stats representing the history of the current continuous
pipeline playback period.

When `format` is `Format::Undefined` both `dropped` and `processed` are
invalid. Values of -1 for either `processed` or `dropped` mean unknown values.

MT safe.
## `format`
Units of the 'processed' and 'dropped' fields.
 Video sinks and video filters will use GST_FORMAT_BUFFERS (frames).
 Audio sinks and audio filters will likely use GST_FORMAT_DEFAULT
 (samples).
## `processed`
Total number of units correctly processed
 since the last state change to READY or a flushing operation.
## `dropped`
Total number of units dropped since the last
 state change to READY or a flushing operation.
<!-- impl Message::fn parse_qos_values -->
Extract the QoS values that have been calculated/analysed from the QoS data

MT safe.
## `jitter`
The difference of the running-time against
 the deadline.
## `proportion`
Long term prediction of the ideal rate
 relative to normal rate to get optimal quality.
## `quality`
An element dependent integer value that
 specifies the current quality level of the element. The default
 maximum quality is 1000000.
<!-- impl Message::fn parse_redirect_entry -->
Parses the location and/or structure from the entry with the given index.
The index must be between 0 and `Message::get_num_redirect_entries` - 1.
Returned pointers are valid for as long as this message exists.

Feature: `v1_10`

## `entry_index`
index of the entry to parse
## `location`
return location for
 the pointer to the entry's location string, or `None`
## `tag_list`
return location for
 the pointer to the entry's tag list, or `None`
## `entry_struct`
return location
 for the pointer to the entry's structure, or `None`
<!-- impl Message::fn parse_request_state -->
Extract the requested state from the request_state message.

MT safe.
## `state`
Result location for the requested state or `None`
<!-- impl Message::fn parse_reset_time -->
Extract the running-time from the RESET_TIME message.

MT safe.
## `running_time`
Result location for the running_time or
 `None`
<!-- impl Message::fn parse_segment_done -->
Extracts the position and format from the segment done message.

MT safe.
## `format`
Result location for the format, or `None`
## `position`
Result location for the position, or `None`
<!-- impl Message::fn parse_segment_start -->
Extracts the position and format from the segment start message.

MT safe.
## `format`
Result location for the format, or `None`
## `position`
Result location for the position, or `None`
<!-- impl Message::fn parse_state_changed -->
Extracts the old and new states from the GstMessage.

Typical usage of this function might be:

```C
  ...
  switch (GST_MESSAGE_TYPE (msg)) {
    case GST_MESSAGE_STATE_CHANGED: {
      GstState old_state, new_state;

      gst_message_parse_state_changed (msg, &amp;old_state, &amp;new_state, NULL);
      g_print ("Element %s changed state from %s to %s.\n",
          GST_OBJECT_NAME (msg->src),
          gst_element_state_get_name (old_state),
          gst_element_state_get_name (new_state));
      break;
    }
    ...
  }
  ...
```

MT safe.
## `oldstate`
the previous state, or `None`
## `newstate`
the new (current) state, or `None`
## `pending`
the pending (target) state, or `None`
<!-- impl Message::fn parse_step_done -->
Extract the values the step_done message.

MT safe.
## `format`
result location for the format
## `amount`
result location for the amount
## `rate`
result location for the rate
## `flush`
result location for the flush flag
## `intermediate`
result location for the intermediate flag
## `duration`
result location for the duration
## `eos`
result location for the EOS flag
<!-- impl Message::fn parse_step_start -->
Extract the values from step_start message.

MT safe.
## `active`
result location for the active flag
## `format`
result location for the format
## `amount`
result location for the amount
## `rate`
result location for the rate
## `flush`
result location for the flush flag
## `intermediate`
result location for the intermediate flag
<!-- impl Message::fn parse_stream_collection -->
Parses a stream-collection message.

Feature: `v1_10`

## `collection`
A location where to store a
 pointer to the `StreamCollection`, or `None`
<!-- impl Message::fn parse_stream_status -->
Extracts the stream status type and owner the GstMessage. The returned
owner remains valid for as long as the reference to `self` is valid and
should thus not be unreffed.

MT safe.
## `type_`
A pointer to hold the status type
## `owner`
The owner element of the message source
<!-- impl Message::fn parse_streams_selected -->
Parses a streams-selected message.

Feature: `v1_10`

## `collection`
A location where to store a
 pointer to the `StreamCollection`, or `None`
<!-- impl Message::fn parse_structure_change -->
Extracts the change type and completion status from the GstMessage.

MT safe.
## `type_`
A pointer to hold the change type
## `owner`
The owner element of the
 message source
## `busy`
a pointer to hold whether the change is in
 progress or has been completed
<!-- impl Message::fn parse_tag -->
Extracts the tag list from the GstMessage. The tag list returned in the
output argument is a copy; the caller must free it when done.

Typical usage of this function might be:

```C
  ...
  switch (GST_MESSAGE_TYPE (msg)) {
    case GST_MESSAGE_TAG: {
      GstTagList *tags = NULL;

      gst_message_parse_tag (msg, &amp;tags);
      g_print ("Got tags from element %s\n", GST_OBJECT_NAME (msg->src));
      handle_tags (tags);
      gst_tag_list_unref (tags);
      break;
    }
    ...
  }
  ...
```

MT safe.
## `tag_list`
return location for the tag-list.
<!-- impl Message::fn parse_toc -->
Extract the TOC from the `Message`. The TOC returned in the
output argument is a copy; the caller must free it with
`gst_toc_unref` when done.

MT safe.
## `toc`
return location for the TOC.
## `updated`
return location for the updated flag.
<!-- impl Message::fn parse_warning -->
Extracts the GError and debug string from the GstMessage. The values returned
in the output arguments are copies; the caller must free them when done.

MT safe.
## `gerror`
location for the GError
## `debug`
location for the debug message,
 or `None`
<!-- impl Message::fn parse_warning_details -->
Returns the optional details structure, may be NULL if none
The returned structure must not be freed.

Feature: `v1_10`

## `structure`
A pointer to the returned details structure
<!-- impl Message::fn set_buffering_stats -->
Configures the buffering stats values in `self`.
## `mode`
a buffering mode
## `avg_in`
the average input rate
## `avg_out`
the average output rate
## `buffering_left`
amount of buffering time left in milliseconds
<!-- impl Message::fn set_group_id -->
Sets the group id on the stream-start message.

All streams that have the same group id are supposed to be played
together, i.e. all streams inside a container file should have the
same group id but different stream ids. The group id should change
each time the stream is started, resulting in different group ids
each time a file is played for example.

MT safe.
## `group_id`
the group id
<!-- impl Message::fn set_qos_stats -->
Set the QoS stats representing the history of the current continuous pipeline
playback period.

When `format` is `Format::Undefined` both `dropped` and `processed` are
invalid. Values of -1 for either `processed` or `dropped` mean unknown values.

MT safe.
## `format`
Units of the 'processed' and 'dropped' fields. Video sinks and video
filters will use GST_FORMAT_BUFFERS (frames). Audio sinks and audio filters
will likely use GST_FORMAT_DEFAULT (samples).
## `processed`
Total number of units correctly processed since the last state
change to READY or a flushing operation.
## `dropped`
Total number of units dropped since the last state change to READY
or a flushing operation.
<!-- impl Message::fn set_qos_values -->
Set the QoS values that have been calculated/analysed from the QoS data

MT safe.
## `jitter`
The difference of the running-time against the deadline.
## `proportion`
Long term prediction of the ideal rate relative to normal rate
to get optimal quality.
## `quality`
An element dependent integer value that specifies the current
quality level of the element. The default maximum quality is 1000000.
<!-- impl Message::fn set_seqnum -->
Set the sequence number of a message.

This function might be called by the creator of a message to indicate that
the message relates to other messages or events. See `Message::get_seqnum`
for more information.

MT safe.
## `seqnum`
A sequence number.
<!-- impl Message::fn set_stream_status_object -->
Configures the object handling the streaming thread. This is usually a
GstTask object but other objects might be added in the future.
## `object`
the object controlling the streaming
<!-- impl Message::fn streams_selected_add -->
Adds the `stream` to the `self`.

Feature: `v1_10`

## `stream`
a `Stream` to add to `self`
<!-- impl Message::fn streams_selected_get_size -->
Returns the number of streams contained in the `self`.

Feature: `v1_10`


# Returns

The number of streams contained within.
<!-- impl Message::fn streams_selected_get_stream -->
Retrieves the `Stream` with index `index` from the `self`.

Feature: `v1_10`

## `idx`
Index of the stream to retrieve

# Returns

A `Stream`
<!-- struct Object -->
`Object` provides a root for the object hierarchy tree filed in by the
GStreamer library. It is currently a thin wrapper on top of
`gobject::InitiallyUnowned`. It is an abstract class that is not very usable on its own.

`Object` gives us basic refcounting, parenting functionality and locking.
Most of the functions are just extended for special GStreamer needs and can be
found under the same name in the base class of `Object` which is `gobject::Object`
(e.g. `gobject::ObjectExt::ref` becomes `GstObjectExt::ref`).

Since `Object` derives from `gobject::InitiallyUnowned`, it also inherits the
floating reference. Be aware that functions such as `BinExt::add` and
`ElementExt::add_pad` take ownership of the floating reference.

In contrast to `gobject::Object` instances, `Object` adds a name property. The functions
`GstObjectExt::set_name` and `GstObjectExt::get_name` are used to set/get the name
of the object.

## controlled properties

Controlled properties offers a lightweight way to adjust gobject properties
over stream-time. It works by using time-stamped value pairs that are queued
for element-properties. At run-time the elements continuously pull value
changes for the current stream-time.

What needs to be changed in a `Element`?
Very little - it is just two steps to make a plugin controllable!

 * mark gobject-properties paramspecs that make sense to be controlled,
 by GST_PARAM_CONTROLLABLE.

 * when processing data (get, chain, loop function) at the beginning call
 gst_object_sync_values(element,timestamp).
 This will make the controller update all GObject properties that are
 under its control with the current values based on the timestamp.

What needs to be done in applications? Again it's not a lot to change.

 * create a `ControlSource`.
 csource = gst_interpolation_control_source_new ();
 g_object_set (csource, "mode", GST_INTERPOLATION_MODE_LINEAR, NULL);

 * Attach the `ControlSource` on the controller to a property.
 gst_object_add_control_binding (object, gst_direct_control_binding_new (object, "prop1", csource));

 * Set the control values
 gst_timed_value_control_source_set ((GstTimedValueControlSource *)csource,0 * GST_SECOND, value1);
 gst_timed_value_control_source_set ((GstTimedValueControlSource *)csource,1 * GST_SECOND, value2);

 * start your pipeline

# Implements

[`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait GstObjectExt -->
Trait containing all `Object` methods.

# Implementors

[`Bus`](struct.Bus.html), [`Clock`](struct.Clock.html), [`DeviceMonitor`](struct.DeviceMonitor.html), [`DeviceProvider`](struct.DeviceProvider.html), [`Device`](struct.Device.html), [`Element`](struct.Element.html), [`Object`](struct.Object.html), [`PadTemplate`](struct.PadTemplate.html), [`Pad`](struct.Pad.html), [`Plugin`](struct.Plugin.html), [`StreamCollection`](struct.StreamCollection.html), [`Stream`](struct.Stream.html)
<!-- impl Object::fn check_uniqueness -->
Checks to see if there is any object named `name` in `list`. This function
does not do any locking of any kind. You might want to protect the
provided list with the lock of the owner of the list. This function
will lock each `Object` in the list to compare the name, so be
careful when passing a list with a locked object.
## `list`
a list of `Object` to
 check through
## `name`
the name to search for

# Returns

`true` if a `Object` named `name` does not appear in `list`,
`false` if it does.

MT safe. Grabs and releases the LOCK of each object in the list.
<!-- impl Object::fn default_deep_notify -->
A default deep_notify signal callback for an object. The user data
should contain a pointer to an array of strings that should be excluded
from the notify. The default handler will print the new value of the property
using g_print.

MT safe. This function grabs and releases `object`'s LOCK for getting its
 path string.
## `object`
the `gobject::Object` that signalled the notify.
## `orig`
a `Object` that initiated the notify.
## `pspec`
a `gobject::ParamSpec` of the property.
## `excluded_props`

 a set of user-specified properties to exclude or `None` to show
 all changes.
<!-- impl Object::fn ref_sink -->
Increase the reference count of `object`, and possibly remove the floating
reference, if `object` has a floating reference.

In other words, if the object is floating, then this call "assumes ownership"
of the floating reference, converting it to a normal reference by clearing
the floating flag while leaving the reference count unchanged. If the object
is not floating, then this call adds a new normal reference increasing the
reference count by one.
## `object`
a `Object` to sink
<!-- impl Object::fn replace -->
Atomically modifies a pointer to point to a new object.
The reference count of `oldobj` is decreased and the reference count of
`newobj` is increased.

Either `newobj` and the value pointed to by `oldobj` may be `None`.
## `oldobj`
pointer to a place of
 a `Object` to replace
## `newobj`
a new `Object`

# Returns

`true` if `newobj` was different from `oldobj`
<!-- trait GstObjectExt::fn add_control_binding -->
Attach the `ControlBinding` to the object. If there already was a
`ControlBinding` for this property it will be replaced.

The `self` will take ownership of the `binding`.
## `binding`
the `ControlBinding` that should be used

# Returns

`false` if the given `binding` has not been setup for this object or
has been setup for a non suitable property, `true` otherwise.
<!-- trait GstObjectExt::fn default_error -->
A default error function that uses `g_printerr` to display the error message
and the optional debug sting..

The default handler will simply print the error string using g_print.
## `error`
the GError.
## `debug`
an additional debug information string, or `None`
<!-- trait GstObjectExt::fn get_control_binding -->
Gets the corresponding `ControlBinding` for the property. This should be
unreferenced again after use.
## `property_name`
name of the property

# Returns

the `ControlBinding` for
`property_name` or `None` if the property is not controlled.
<!-- trait GstObjectExt::fn get_control_rate -->
Obtain the control-rate for this `self`. Audio processing `Element`
objects will use this rate to sub-divide their processing loop and call
`GstObjectExt::sync_values` inbetween. The length of the processing segment
should be up to `control`-rate nanoseconds.

If the `self` is not under property control, this will return
`GST_CLOCK_TIME_NONE`. This allows the element to avoid the sub-dividing.

The control-rate is not expected to change if the element is in
`State::Paused` or `State::Playing`.

# Returns

the control rate in nanoseconds
<!-- trait GstObjectExt::fn get_g_value_array -->
Gets a number of `GValues` for the given controlled property starting at the
requested time. The array `values` need to hold enough space for `n_values` of
`gobject::Value`.

This function is useful if one wants to e.g. draw a graph of the control
curve or apply a control curve sample by sample.
## `property_name`
the name of the property to get
## `timestamp`
the time that should be processed
## `interval`
the time spacing between subsequent values
## `n_values`
the number of values
## `values`
array to put control-values in

# Returns

`true` if the given array could be filled, `false` otherwise
<!-- trait GstObjectExt::fn get_name -->
Returns a copy of the name of `self`.
Caller should `g_free` the return value after usage.
For a nameless object, this returns `None`, which you can safely `g_free`
as well.

Free-function: g_free

# Returns

the name of `self`. `g_free`
after usage.

MT safe. This function grabs and releases `self`'s LOCK.
<!-- trait GstObjectExt::fn get_parent -->
Returns the parent of `self`. This function increases the refcount
of the parent object so you should `GstObjectExt::unref` it after usage.

# Returns

parent of `self`, this can be
 `None` if `self` has no parent. unref after usage.

MT safe. Grabs and releases `self`'s LOCK.
<!-- trait GstObjectExt::fn get_path_string -->
Generates a string describing the path of `self` in
the object hierarchy. Only useful (or used) for debugging.

Free-function: g_free

# Returns

a string describing the path of `self`. You must
 `g_free` the string after usage.

MT safe. Grabs and releases the `Object`'s LOCK for all objects
 in the hierarchy.
<!-- trait GstObjectExt::fn get_value -->
Gets the value for the given controlled property at the requested time.
## `property_name`
the name of the property to get
## `timestamp`
the time the control-change should be read from

# Returns

the GValue of the property at the given time,
or `None` if the property isn't controlled.
<!-- trait GstObjectExt::fn get_value_array -->
Gets a number of values for the given controlled property starting at the
requested time. The array `values` need to hold enough space for `n_values` of
the same type as the objects property's type.

This function is useful if one wants to e.g. draw a graph of the control
curve or apply a control curve sample by sample.

The values are unboxed and ready to be used. The similar function
`GstObjectExt::get_g_value_array` returns the array as `GValues` and is
better suites for bindings.
## `property_name`
the name of the property to get
## `timestamp`
the time that should be processed
## `interval`
the time spacing between subsequent values
## `n_values`
the number of values
## `values`
array to put control-values in

# Returns

`true` if the given array could be filled, `false` otherwise
<!-- trait GstObjectExt::fn has_active_control_bindings -->
Check if the `self` has active controlled properties.

# Returns

`true` if the object has active controlled properties
<!-- trait GstObjectExt::fn has_ancestor -->
Check if `self` has an ancestor `ancestor` somewhere up in
the hierarchy. One can e.g. check if a `Element` is inside a `Pipeline`.

# Deprecated

Use `GstObjectExt::has_as_ancestor` instead.

MT safe. Grabs and releases `self`'s locks.
## `ancestor`
a `Object` to check as ancestor

# Returns

`true` if `ancestor` is an ancestor of `self`.
<!-- trait GstObjectExt::fn has_as_ancestor -->
Check if `self` has an ancestor `ancestor` somewhere up in
the hierarchy. One can e.g. check if a `Element` is inside a `Pipeline`.
## `ancestor`
a `Object` to check as ancestor

# Returns

`true` if `ancestor` is an ancestor of `self`.

MT safe. Grabs and releases `self`'s locks.
<!-- trait GstObjectExt::fn has_as_parent -->
Check if `parent` is the parent of `self`.
E.g. a `Element` can check if it owns a given `Pad`.
## `parent`
a `Object` to check as parent

# Returns

`false` if either `self` or `parent` is `None`. `true` if `parent` is
 the parent of `self`. Otherwise `false`.

MT safe. Grabs and releases `self`'s locks.
<!-- trait GstObjectExt::fn ref -->
Increments the reference count on `self`. This function
does not take the lock on `self` because it relies on
atomic refcounting.

This object returns the input parameter to ease writing
constructs like :
 result = gst_object_ref (object->parent);

# Returns

A pointer to `self`
<!-- trait GstObjectExt::fn remove_control_binding -->
Removes the corresponding `ControlBinding`. If it was the
last ref of the binding, it will be disposed.
## `binding`
the binding

# Returns

`true` if the binding could be removed.
<!-- trait GstObjectExt::fn set_control_binding_disabled -->
This function is used to disable the control bindings on a property for
some time, i.e. `GstObjectExt::sync_values` will do nothing for the
property.
## `property_name`
property to disable
## `disabled`
boolean that specifies whether to disable the controller
or not.
<!-- trait GstObjectExt::fn set_control_bindings_disabled -->
This function is used to disable all controlled properties of the `self` for
some time, i.e. `GstObjectExt::sync_values` will do nothing.
## `disabled`
boolean that specifies whether to disable the controller
or not.
<!-- trait GstObjectExt::fn set_control_rate -->
Change the control-rate for this `self`. Audio processing `Element`
objects will use this rate to sub-divide their processing loop and call
`GstObjectExt::sync_values` inbetween. The length of the processing segment
should be up to `control`-rate nanoseconds.

The control-rate should not change if the element is in `State::Paused` or
`State::Playing`.
## `control_rate`
the new control-rate in nanoseconds.
<!-- trait GstObjectExt::fn set_name -->
Sets the name of `self`, or gives `self` a guaranteed unique
name (if `name` is `None`).
This function makes a copy of the provided name, so the caller
retains ownership of the name it sent.
## `name`
new name of object

# Returns

`true` if the name could be set. Since Objects that have
a parent cannot be renamed, this function returns `false` in those
cases.

MT safe. This function grabs and releases `self`'s LOCK.
<!-- trait GstObjectExt::fn set_parent -->
Sets the parent of `self` to `parent`. The object's reference count will
be incremented, and any floating reference will be removed (see `Object::ref_sink`).
## `parent`
new parent of object

# Returns

`true` if `parent` could be set or `false` when `self`
already had a parent or `self` and `parent` are the same.

MT safe. Grabs and releases `self`'s LOCK.
<!-- trait GstObjectExt::fn suggest_next_sync -->
Returns a suggestion for timestamps where buffers should be split
to get best controller results.

# Returns

Returns the suggested timestamp or `GST_CLOCK_TIME_NONE`
if no control-rate was set.
<!-- trait GstObjectExt::fn sync_values -->
Sets the properties of the object, according to the `GstControlSources` that
(maybe) handle them and for the given timestamp.

If this function fails, it is most likely the application developers fault.
Most probably the control sources are not setup correctly.
## `timestamp`
the time that should be processed

# Returns

`true` if the controller values could be applied to the object
properties, `false` otherwise
<!-- trait GstObjectExt::fn unparent -->
Clear the parent of `self`, removing the associated reference.
This function decreases the refcount of `self`.

MT safe. Grabs and releases `self`'s lock.
<!-- trait GstObjectExt::fn unref -->
Decrements the reference count on `self`. If reference count hits
zero, destroy `self`. This function does not take the lock
on `self` as it relies on atomic refcounting.

The unref method should never be called with the LOCK held since
this might deadlock the dispose function.
<!-- struct Pad -->
A `Element` is linked to other elements via "pads", which are extremely
light-weight generic link points.

Pads have a `PadDirection`, source pads produce data, sink pads consume
data.

Pads are typically created from a `PadTemplate` with
`Pad::new_from_template` and are then added to a `Element`. This usually
happens when the element is created but it can also happen dynamically based
on the data that the element is processing or based on the pads that the
application requests.

Pads without pad templates can be created with `Pad::new`,
which takes a direction and a name as an argument. If the name is `None`,
then a guaranteed unique name will be assigned to it.

A `Element` creating a pad will typically use the various
gst_pad_set_*`_function` calls to register callbacks for events, queries or
dataflow on the pads.

`gst_pad_get_parent` will retrieve the `Element` that owns the pad.

After two pads are retrieved from an element by `ElementExt::get_static_pad`,
the pads can be linked with `PadExt::link`. (For quick links,
you can also use `ElementExt::link`, which will make the obvious
link for you if it's straightforward.). Pads can be unlinked again with
`PadExt::unlink`. `PadExt::get_peer` can be used to check what the pad is
linked to.

Before dataflow is possible on the pads, they need to be activated with
`PadExt::set_active`.

`Pad::query` and `Pad::peer_query` can be used to query various
properties of the pad and the stream.

To send a `Event` on a pad, use `Pad::send_event` and
`Pad::push_event`. Some events will be sticky on the pad, meaning that
after they pass on the pad they can be queried later with
`PadExt::get_sticky_event` and `PadExt::sticky_events_foreach`.
`PadExt::get_current_caps` and `PadExt::has_current_caps` are convenience
functions to query the current sticky CAPS event on a pad.

GstElements will use `Pad::push` and `Pad::pull_range` to push out
or pull in a buffer.

The dataflow, events and queries that happen on a pad can be monitored with
probes that can be installed with `PadExt::add_probe`. `PadExt::is_blocked`
can be used to check if a block probe is installed on the pad.
`PadExt::is_blocking` checks if the blocking probe is currently blocking the
pad. `Pad::remove_probe` is used to remove a previously installed probe
and unblock blocking probes if any.

Pad have an offset that can be retrieved with `PadExt::get_offset`. This
offset will be applied to the running_time of all data passing over the pad.
`PadExt::set_offset` can be used to change the offset.

Convenience functions exist to start, pause and stop the task on a pad with
`PadExt::start_task`, `PadExt::pause_task` and `PadExt::stop_task`
respectively.

# Implements

[`PadExt`](trait.PadExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait PadExt -->
Trait containing all `Pad` methods.

# Implementors

[`Pad`](struct.Pad.html), [`ProxyPad`](struct.ProxyPad.html)
<!-- impl Pad::fn new -->
Creates a new pad with the given name in the given direction.
If name is `None`, a guaranteed unique name (across all pads)
will be assigned.
This function makes a copy of the name so you can safely free the name.
## `name`
the name of the new pad.
## `direction`
the `PadDirection` of the pad.

# Returns

a new `Pad`, or `None` in
case of an error.

MT safe.
<!-- impl Pad::fn new_from_static_template -->
Creates a new pad with the given name from the given static template.
If name is `None`, a guaranteed unique name (across all pads)
will be assigned.
This function makes a copy of the name so you can safely free the name.
## `templ`
the `StaticPadTemplate` to use
## `name`
the name of the pad

# Returns

a new `Pad`, or `None` in
case of an error.
<!-- impl Pad::fn new_from_template -->
Creates a new pad with the given name from the given template.
If name is `None`, a guaranteed unique name (across all pads)
will be assigned.
This function makes a copy of the name so you can safely free the name.
## `templ`
the pad template to use
## `name`
the name of the pad

# Returns

a new `Pad`, or `None` in
case of an error.
<!-- impl Pad::fn link_get_name -->
Gets a string representing the given pad-link return.
## `ret`
a `PadLinkReturn` to get the name of.

# Returns

a static string with the name of the pad-link return.
<!-- trait PadExt::fn activate_mode -->
Activates or deactivates the given pad in `mode` via dispatching to the
pad's activatemodefunc. For use from within pad activation functions only.

If you don't know what this is, you probably don't want to call it.
## `mode`
the requested activation mode
## `active`
whether or not the pad should be active.

# Returns

`true` if the operation was successful.

MT safe.
<!-- trait PadExt::fn add_probe -->
Be notified of different states of pads. The provided callback is called for
every state that matches `mask`.

Probes are called in groups: First GST_PAD_PROBE_TYPE_BLOCK probes are
called, then others, then finally GST_PAD_PROBE_TYPE_IDLE. The only
exception here are GST_PAD_PROBE_TYPE_IDLE probes that are called
immediately if the pad is already idle while calling `PadExt::add_probe`.
In each of the groups, probes are called in the order in which they were
added.
## `mask`
the probe mask
## `callback`
`GstPadProbeCallback` that will be called with notifications of
 the pad state
## `user_data`
user data passed to the callback
## `destroy_data`
`GDestroyNotify` for user_data

# Returns

an id or 0 if no probe is pending. The id can be used to remove the
probe with `Pad::remove_probe`. When using GST_PAD_PROBE_TYPE_IDLE it can
happen that the probe can be run immediately and if the probe returns
GST_PAD_PROBE_REMOVE this functions returns 0.

MT safe.
<!-- trait PadExt::fn can_link -->
Checks if the source pad and the sink pad are compatible so they can be
linked.
## `sinkpad`
the sink `Pad`.

# Returns

`true` if the pads can be linked.
<!-- trait PadExt::fn chain -->
Chain a buffer to `self`.

The function returns `FlowReturn::Flushing` if the pad was flushing.

If the buffer type is not acceptable for `self` (as negotiated with a
preceding GST_EVENT_CAPS event), this function returns
`FlowReturn::NotNegotiated`.

The function proceeds calling the chain function installed on `self` (see
`gst_pad_set_chain_function`) and the return value of that function is
returned to the caller. `FlowReturn::NotSupported` is returned if `self` has no
chain function.

In all cases, success or failure, the caller loses its reference to `buffer`
after calling this function.
## `buffer`
the `Buffer` to send, return GST_FLOW_ERROR
 if not.

# Returns

a `FlowReturn` from the pad.

MT safe.
<!-- trait PadExt::fn chain_list -->
Chain a bufferlist to `self`.

The function returns `FlowReturn::Flushing` if the pad was flushing.

If `self` was not negotiated properly with a CAPS event, this function
returns `FlowReturn::NotNegotiated`.

The function proceeds calling the chainlist function installed on `self` (see
`gst_pad_set_chain_list_function`) and the return value of that function is
returned to the caller. `FlowReturn::NotSupported` is returned if `self` has no
chainlist function.

In all cases, success or failure, the caller loses its reference to `list`
after calling this function.

MT safe.
## `list`
the `BufferList` to send, return GST_FLOW_ERROR
 if not.

# Returns

a `FlowReturn` from the pad.
<!-- trait PadExt::fn check_reconfigure -->
Check and clear the `PadFlags::NeedReconfigure` flag on `self` and return `true`
if the flag was set.

# Returns

`true` is the GST_PAD_FLAG_NEED_RECONFIGURE flag was set on `self`.
<!-- trait PadExt::fn create_stream_id -->
Creates a stream-id for the source `Pad` `self` by combining the
upstream information with the optional `stream_id` of the stream
of `self`. `self` must have a parent `Element` and which must have zero
or one sinkpad. `stream_id` can only be `None` if the parent element
of `self` has only a single source pad.

This function generates an unique stream-id by getting the upstream
stream-start event stream ID and appending `stream_id` to it. If the
element has no sinkpad it will generate an upstream stream-id by
doing an URI query on the element and in the worst case just uses
a random number. Source elements that don't implement the URI
handler interface should ideally generate a unique, deterministic
stream-id manually instead.

Since stream IDs are sorted alphabetically, any numbers in the
stream ID should be printed with a fixed number of characters,
preceded by 0's, such as by using the format \%03u instead of \%u.
## `parent`
Parent `Element` of `self`
## `stream_id`
The stream-id

# Returns

A stream-id for `self`. `g_free` after usage.
<!-- trait PadExt::fn create_stream_id_printf -->
Creates a stream-id for the source `Pad` `self` by combining the
upstream information with the optional `stream_id` of the stream
of `self`. `self` must have a parent `Element` and which must have zero
or one sinkpad. `stream_id` can only be `None` if the parent element
of `self` has only a single source pad.

This function generates an unique stream-id by getting the upstream
stream-start event stream ID and appending `stream_id` to it. If the
element has no sinkpad it will generate an upstream stream-id by
doing an URI query on the element and in the worst case just uses
a random number. Source elements that don't implement the URI
handler interface should ideally generate a unique, deterministic
stream-id manually instead.
## `parent`
Parent `Element` of `self`
## `stream_id`
The stream-id

# Returns

A stream-id for `self`. `g_free` after usage.
<!-- trait PadExt::fn create_stream_id_printf_valist -->
Creates a stream-id for the source `Pad` `self` by combining the
upstream information with the optional `stream_id` of the stream
of `self`. `self` must have a parent `Element` and which must have zero
or one sinkpad. `stream_id` can only be `None` if the parent element
of `self` has only a single source pad.

This function generates an unique stream-id by getting the upstream
stream-start event stream ID and appending `stream_id` to it. If the
element has no sinkpad it will generate an upstream stream-id by
doing an URI query on the element and in the worst case just uses
a random number. Source elements that don't implement the URI
handler interface should ideally generate a unique, deterministic
stream-id manually instead.
## `parent`
Parent `Element` of `self`
## `stream_id`
The stream-id
## `var_args`
parameters for the `stream_id` format string

# Returns

A stream-id for `self`. `g_free` after usage.
<!-- trait PadExt::fn event_default -->
Invokes the default event handler for the given pad.

The EOS event will pause the task associated with `self` before it is forwarded
to all internally linked pads,

The event is sent to all pads internally linked to `self`. This function
takes ownership of `event`.
## `parent`
the parent of `self` or `None`
## `event`
the `Event` to handle.

# Returns

`true` if the event was sent successfully.
<!-- trait PadExt::fn forward -->
Calls `forward` for all internally linked pads of `self`. This function deals with
dynamically changing internal pads and will make sure that the `forward`
function is only called once for each pad.

When `forward` returns `true`, no further pads will be processed.
## `forward`
a `GstPadForwardFunction`
## `user_data`
user data passed to `forward`

# Returns

`true` if one of the dispatcher functions returned `true`.
<!-- trait PadExt::fn get_allowed_caps -->
Gets the capabilities of the allowed media types that can flow through
`self` and its peer.

The allowed capabilities is calculated as the intersection of the results of
calling `PadExt::query_caps` on `self` and its peer. The caller owns a reference
on the resulting caps.

# Returns

the allowed `Caps` of the
 pad link. Unref the caps when you no longer need it. This
 function returns `None` when `self` has no peer.

MT safe.
<!-- trait PadExt::fn get_current_caps -->
Gets the capabilities currently configured on `self` with the last
`EventType::Caps` event.

# Returns

the current caps of the pad with
incremented ref-count or `None` when pad has no caps. Unref after usage.
<!-- trait PadExt::fn get_direction -->
Gets the direction of the pad. The direction of the pad is
decided at construction time so this function does not take
the LOCK.

# Returns

the `PadDirection` of the pad.

MT safe.
<!-- trait PadExt::fn get_element_private -->
Gets the private data of a pad.
No locking is performed in this function.

# Returns

a `gpointer` to the private data.
<!-- trait PadExt::fn get_last_flow_return -->
Gets the `FlowReturn` return from the last data passed by this pad.
<!-- trait PadExt::fn get_offset -->
Get the offset applied to the running time of `self`. `self` has to be a source
pad.

# Returns

the offset.
<!-- trait PadExt::fn get_pad_template -->
Gets the template for `self`.

# Returns

the `PadTemplate` from which
 this pad was instantiated, or `None` if this pad has no
 template. Unref after usage.
<!-- trait PadExt::fn get_pad_template_caps -->
Gets the capabilities for `self`'s template.

# Returns

the `Caps` of this pad template.
Unref after usage.
<!-- trait PadExt::fn get_parent_element -->
Gets the parent of `self`, cast to a `Element`. If a `self` has no parent or
its parent is not an element, return `None`.

# Returns

the parent of the pad. The
caller has a reference on the parent, so unref when you're finished
with it.

MT safe.
<!-- trait PadExt::fn get_peer -->
Gets the peer of `self`. This function refs the peer pad so
you need to unref it after use.

# Returns

the peer `Pad`. Unref after usage.

MT safe.
<!-- trait PadExt::fn get_range -->
When `self` is flushing this function returns `FlowReturn::Flushing`
immediately and `buffer` is `None`.

Calls the getrange function of `self`, see `GstPadGetRangeFunction` for a
description of a getrange function. If `self` has no getrange function
installed (see `gst_pad_set_getrange_function`) this function returns
`FlowReturn::NotSupported`.

If `buffer` points to a variable holding `None`, a valid new `Buffer` will be
placed in `buffer` when this function returns `FlowReturn::Ok`. The new buffer
must be freed with `gst_buffer_unref` after usage.

When `buffer` points to a variable that points to a valid `Buffer`, the
buffer will be filled with the result data when this function returns
`FlowReturn::Ok`. If the provided buffer is larger than `size`, only
`size` bytes will be filled in the result buffer and its size will be updated
accordingly.

Note that less than `size` bytes can be returned in `buffer` when, for example,
an EOS condition is near or when `buffer` is not large enough to hold `size`
bytes. The caller should check the result buffer size to get the result size.

When this function returns any other result value than `FlowReturn::Ok`, `buffer`
will be unchanged.

This is a lowlevel function. Usually `Pad::pull_range` is used.
## `offset`
The start offset of the buffer
## `size`
The length of the buffer
## `buffer`
a pointer to hold the `Buffer`,
 returns `FlowReturn::Error` if `None`.

# Returns

a `FlowReturn` from the pad.

MT safe.
<!-- trait PadExt::fn get_sticky_event -->
Returns a new reference of the sticky event of type `event_type`
from the event.
## `event_type`
the `EventType` that should be retrieved.
## `idx`
the index of the event

# Returns

a `Event` of type
`event_type` or `None` when no event of `event_type` was on
`self`. Unref after usage.
<!-- trait PadExt::fn get_stream -->
Returns the current `Stream` for the `self`, or `None` if none has been
set yet, i.e. the pad has not received a stream-start event yet.

This is a convenience wrapper around `PadExt::get_sticky_event` and
`Event::parse_stream`.

Feature: `v1_10`


# Returns

the current `Stream` for `self`, or `None`.
 unref the returned stream when no longer needed.
<!-- trait PadExt::fn get_stream_id -->
Returns the current stream-id for the `self`, or `None` if none has been
set yet, i.e. the pad has not received a stream-start event yet.

This is a convenience wrapper around `PadExt::get_sticky_event` and
`Event::parse_stream_start`.

The returned stream-id string should be treated as an opaque string, its
contents should not be interpreted.

# Returns

a newly-allocated copy of the stream-id for
 `self`, or `None`. `g_free` the returned string when no longer
 needed.
<!-- trait PadExt::fn get_task_state -->
Get `self` task state. If no task is currently
set, `TaskState::Stopped` is returned.

Feature: `v1_12`


# Returns

The current state of `self`'s task.
<!-- trait PadExt::fn has_current_caps -->
Check if `self` has caps set on it with a `EventType::Caps` event.

# Returns

`true` when `self` has caps associated with it.
<!-- trait PadExt::fn is_active -->
Query if a pad is active

# Returns

`true` if the pad is active.

MT safe.
<!-- trait PadExt::fn is_blocked -->
Checks if the pad is blocked or not. This function returns the
last requested state of the pad. It is not certain that the pad
is actually blocking at this point (see `PadExt::is_blocking`).

# Returns

`true` if the pad is blocked.

MT safe.
<!-- trait PadExt::fn is_blocking -->
Checks if the pad is blocking or not. This is a guaranteed state
of whether the pad is actually blocking on a `Buffer` or a `Event`.

# Returns

`true` if the pad is blocking.

MT safe.
<!-- trait PadExt::fn is_linked -->
Checks if a `self` is linked to another pad or not.

# Returns

`true` if the pad is linked, `false` otherwise.

MT safe.
<!-- trait PadExt::fn iterate_internal_links -->
Gets an iterator for the pads to which the given pad is linked to inside
of the parent element.

Each `Pad` element yielded by the iterator will have its refcount increased,
so unref after use.

Free-function: gst_iterator_free

# Returns

a new `Iterator` of `Pad`
 or `None` when the pad does not have an iterator function
 configured. Use `Iterator::free` after usage.
<!-- trait PadExt::fn iterate_internal_links_default -->
Iterate the list of pads to which the given pad is linked to inside of
the parent element.
This is the default handler, and thus returns an iterator of all of the
pads inside the parent element with opposite direction.

The caller must free this iterator after use with `Iterator::free`.
## `parent`
the parent of `self` or `None`

# Returns

a `Iterator` of `Pad`, or `None` if `self`
has no parent. Unref each returned pad with `GstObjectExt::unref`.
<!-- trait PadExt::fn link -->
Links the source pad and the sink pad.
## `sinkpad`
the sink `Pad` to link.

# Returns

A result code indicating if the connection worked or
 what went wrong.

MT Safe.
<!-- trait PadExt::fn link_full -->
Links the source pad and the sink pad.

This variant of `PadExt::link` provides a more granular control on the
checks being done when linking. While providing some considerable speedups
the caller of this method must be aware that wrong usage of those flags
can cause severe issues. Refer to the documentation of `PadLinkCheck`
for more information.

MT Safe.
## `sinkpad`
the sink `Pad` to link.
## `flags`
the checks to validate when linking

# Returns

A result code indicating if the connection worked or
 what went wrong.
<!-- trait PadExt::fn link_maybe_ghosting -->
Links `self` to `sink`, creating any `GhostPad`'s in between as necessary.

This is a convenience function to save having to create and add intermediate
`GhostPad`'s as required for linking across `Bin` boundaries.

If `self` or `sink` pads don't have parent elements or do not share a common
ancestor, the link will fail.

Feature: `v1_10`

## `sink`
a `Pad`

# Returns

whether the link succeeded.
<!-- trait PadExt::fn link_maybe_ghosting_full -->
Links `self` to `sink`, creating any `GhostPad`'s in between as necessary.

This is a convenience function to save having to create and add intermediate
`GhostPad`'s as required for linking across `Bin` boundaries.

If `self` or `sink` pads don't have parent elements or do not share a common
ancestor, the link will fail.

Calling `PadExt::link_maybe_ghosting_full` with
`flags` == `PadLinkCheck::Default` is the recommended way of linking
pads with safety checks applied.

Feature: `v1_10`

## `sink`
a `Pad`
## `flags`
some `PadLinkCheck` flags

# Returns

whether the link succeeded.
<!-- trait PadExt::fn mark_reconfigure -->
Mark a pad for needing reconfiguration. The next call to
`PadExt::check_reconfigure` will return `true` after this call.
<!-- trait PadExt::fn needs_reconfigure -->
Check the `PadFlags::NeedReconfigure` flag on `self` and return `true`
if the flag was set.

# Returns

`true` is the GST_PAD_FLAG_NEED_RECONFIGURE flag is set on `self`.
<!-- trait PadExt::fn pause_task -->
Pause the task of `self`. This function will also wait until the
function executed by the task is finished if this function is not
called from the task function.

# Returns

a `true` if the task could be paused or `false` when the pad
has no task.
<!-- trait PadExt::fn peer_query -->
Performs `Pad::query` on the peer of `self`.

The caller is responsible for both the allocation and deallocation of
the query structure.
## `query`
the `Query` to perform.

# Returns

`true` if the query could be performed. This function returns `false`
if `self` has no peer.
<!-- trait PadExt::fn peer_query_accept_caps -->
Check if the peer of `self` accepts `caps`. If `self` has no peer, this function
returns `true`.
## `caps`
a `Caps` to check on the pad

# Returns

`true` if the peer of `self` can accept the caps or `self` has no peer.
<!-- trait PadExt::fn peer_query_caps -->
Gets the capabilities of the peer connected to this pad. Similar to
`PadExt::query_caps`.

When called on srcpads `filter` contains the caps that
upstream could produce in the order preferred by upstream. When
called on sinkpads `filter` contains the caps accepted by
downstream in the preferred order. `filter` might be `None` but
if it is not `None` the returned caps will be a subset of `filter`.
## `filter`
a `Caps` filter, or `None`.

# Returns

the caps of the peer pad with incremented
ref-count. When there is no peer pad, this function returns `filter` or,
when `filter` is `None`, ANY caps.
<!-- trait PadExt::fn peer_query_convert -->
Queries the peer pad of a given sink pad to convert `src_val` in `src_format`
to `dest_format`.
## `src_format`
a `Format` to convert from.
## `src_val`
a value to convert.
## `dest_format`
the `Format` to convert to.
## `dest_val`
a pointer to the result.

# Returns

`true` if the query could be performed.
<!-- trait PadExt::fn peer_query_duration -->
Queries the peer pad of a given sink pad for the total stream duration.
## `format`
the `Format` requested
## `duration`
a location in which to store the total
 duration, or `None`.

# Returns

`true` if the query could be performed.
<!-- trait PadExt::fn peer_query_position -->
Queries the peer of a given sink pad for the stream position.
## `format`
the `Format` requested
## `cur`
a location in which to store the current
 position, or `None`.

# Returns

`true` if the query could be performed.
<!-- trait PadExt::fn proxy_query_accept_caps -->
Checks if all internally linked pads of `self` accepts the caps in `query` and
returns the intersection of the results.

This function is useful as a default accept caps query function for an element
that can handle any stream format, but requires caps that are acceptable for
all opposite pads.
## `query`
an ACCEPT_CAPS `Query`.

# Returns

`true` if `query` could be executed
<!-- trait PadExt::fn proxy_query_caps -->
Calls `PadExt::query_caps` for all internally linked pads of `self` and returns
the intersection of the results.

This function is useful as a default caps query function for an element
that can handle any stream format, but requires all its pads to have
the same caps. Two such elements are tee and adder.
## `query`
a CAPS `Query`.

# Returns

`true` if `query` could be executed
<!-- trait PadExt::fn pull_range -->
Pulls a `buffer` from the peer pad or fills up a provided buffer.

This function will first trigger the pad block signal if it was
installed.

When `self` is not linked `FlowReturn::NotLinked` is returned else this
function returns the result of `Pad::get_range` on the peer pad.
See `Pad::get_range` for a list of return values and for the
semantics of the arguments of this function.

If `buffer` points to a variable holding `None`, a valid new `Buffer` will be
placed in `buffer` when this function returns `FlowReturn::Ok`. The new buffer
must be freed with `gst_buffer_unref` after usage. When this function
returns any other result value, `buffer` will still point to `None`.

When `buffer` points to a variable that points to a valid `Buffer`, the
buffer will be filled with the result data when this function returns
`FlowReturn::Ok`. When this function returns any other result value,
`buffer` will be unchanged. If the provided buffer is larger than `size`, only
`size` bytes will be filled in the result buffer and its size will be updated
accordingly.

Note that less than `size` bytes can be returned in `buffer` when, for example,
an EOS condition is near or when `buffer` is not large enough to hold `size`
bytes. The caller should check the result buffer size to get the result size.
## `offset`
The start offset of the buffer
## `size`
The length of the buffer
## `buffer`
a pointer to hold the `Buffer`, returns
 GST_FLOW_ERROR if `None`.

# Returns

a `FlowReturn` from the peer pad.

MT safe.
<!-- trait PadExt::fn push -->
Pushes a buffer to the peer of `self`.

This function will call installed block probes before triggering any
installed data probes.

The function proceeds calling `Pad::chain` on the peer pad and returns
the value from that function. If `self` has no peer, `FlowReturn::NotLinked` will
be returned.

In all cases, success or failure, the caller loses its reference to `buffer`
after calling this function.
## `buffer`
the `Buffer` to push returns GST_FLOW_ERROR
 if not.

# Returns

a `FlowReturn` from the peer pad.

MT safe.
<!-- trait PadExt::fn push_event -->
Sends the event to the peer of the given pad. This function is
mainly used by elements to send events to their peer
elements.

This function takes ownership of the provided event so you should
`gst_event_ref` it if you want to reuse the event after this call.
## `event`
the `Event` to send to the pad.

# Returns

`true` if the event was handled.

MT safe.
<!-- trait PadExt::fn push_list -->
Pushes a buffer list to the peer of `self`.

This function will call installed block probes before triggering any
installed data probes.

The function proceeds calling the chain function on the peer pad and returns
the value from that function. If `self` has no peer, `FlowReturn::NotLinked` will
be returned. If the peer pad does not have any installed chainlist function
every group buffer of the list will be merged into a normal `Buffer` and
chained via `Pad::chain`.

In all cases, success or failure, the caller loses its reference to `list`
after calling this function.
## `list`
the `BufferList` to push returns GST_FLOW_ERROR
 if not.

# Returns

a `FlowReturn` from the peer pad.

MT safe.
<!-- trait PadExt::fn query -->
Dispatches a query to a pad. The query should have been allocated by the
caller via one of the type-specific allocation functions. The element that
the pad belongs to is responsible for filling the query with an appropriate
response, which should then be parsed with a type-specific query parsing
function.

Again, the caller is responsible for both the allocation and deallocation of
the query structure.

Please also note that some queries might need a running pipeline to work.
## `query`
the `Query` to perform.

# Returns

`true` if the query could be performed.
<!-- trait PadExt::fn query_accept_caps -->
Check if the given pad accepts the caps.
## `caps`
a `Caps` to check on the pad

# Returns

`true` if the pad can accept the caps.
<!-- trait PadExt::fn query_caps -->
Gets the capabilities this pad can produce or consume.
Note that this method doesn't necessarily return the caps set by sending a
`Event::new_caps` - use `PadExt::get_current_caps` for that instead.
gst_pad_query_caps returns all possible caps a pad can operate with, using
the pad's CAPS query function, If the query fails, this function will return
`filter`, if not `None`, otherwise ANY.

When called on sinkpads `filter` contains the caps that
upstream could produce in the order preferred by upstream. When
called on srcpads `filter` contains the caps accepted by
downstream in the preferred order. `filter` might be `None` but
if it is not `None` the returned caps will be a subset of `filter`.

Note that this function does not return writable `Caps`, use
`gst_caps_make_writable` before modifying the caps.
## `filter`
suggested `Caps`, or `None`

# Returns

the caps of the pad with incremented ref-count.
<!-- trait PadExt::fn query_convert -->
Queries a pad to convert `src_val` in `src_format` to `dest_format`.
## `src_format`
a `Format` to convert from.
## `src_val`
a value to convert.
## `dest_format`
the `Format` to convert to.
## `dest_val`
a pointer to the result.

# Returns

`true` if the query could be performed.
<!-- trait PadExt::fn query_default -->
Invokes the default query handler for the given pad.
The query is sent to all pads internally linked to `self`. Note that
if there are many possible sink pads that are internally linked to
`self`, only one will be sent the query.
Multi-sinkpad elements should implement custom query handlers.
## `parent`
the parent of `self` or `None`
## `query`
the `Query` to handle.

# Returns

`true` if the query was performed successfully.
<!-- trait PadExt::fn query_duration -->
Queries a pad for the total stream duration.
## `format`
the `Format` requested
## `duration`
a location in which to store the total
 duration, or `None`.

# Returns

`true` if the query could be performed.
<!-- trait PadExt::fn query_position -->
Queries a pad for the stream position.
## `format`
the `Format` requested
## `cur`
A location in which to store the current position, or `None`.

# Returns

`true` if the query could be performed.
<!-- trait PadExt::fn remove_probe -->
Remove the probe with `id` from `self`.

MT safe.
## `id`
the probe id to remove
<!-- trait PadExt::fn send_event -->
Sends the event to the pad. This function can be used
by applications to send events in the pipeline.

If `self` is a source pad, `event` should be an upstream event. If `self` is a
sink pad, `event` should be a downstream event. For example, you would not
send a `EventType::Eos` on a src pad; EOS events only propagate downstream.
Furthermore, some downstream events have to be serialized with data flow,
like EOS, while some can travel out-of-band, like `EventType::FlushStart`. If
the event needs to be serialized with data flow, this function will take the
pad's stream lock while calling its event function.

To find out whether an event type is upstream, downstream, or downstream and
serialized, see `EventTypeFlags`, `EventType::get_flags`,
`GST_EVENT_IS_UPSTREAM`, `GST_EVENT_IS_DOWNSTREAM`, and
`GST_EVENT_IS_SERIALIZED`. Note that in practice that an application or
plugin doesn't need to bother itself with this information; the core handles
all necessary locks and checks.

This function takes ownership of the provided event so you should
`gst_event_ref` it if you want to reuse the event after this call.
## `event`
the `Event` to send to the pad.

# Returns

`true` if the event was handled.
<!-- trait PadExt::fn set_activate_function_full -->
Sets the given activate function for `self`. The activate function will
dispatch to `PadExt::activate_mode` to perform the actual activation.
Only makes sense to set on sink pads.

Call this function if your sink pad can start a pull-based task.
## `activate`
the `GstPadActivateFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `activate` will not be used anymore.
<!-- trait PadExt::fn set_activatemode_function_full -->
Sets the given activate_mode function for the pad. An activate_mode function
prepares the element for data passing.
## `activatemode`
the `GstPadActivateModeFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `activatemode` will not be used anymore.
<!-- trait PadExt::fn set_active -->
Activates or deactivates the given pad.
Normally called from within core state change functions.

If `active`, makes sure the pad is active. If it is already active, either in
push or pull mode, just return. Otherwise dispatches to the pad's activate
function to perform the actual activation.

If not `active`, calls `PadExt::activate_mode` with the pad's current mode
and a `false` argument.
## `active`
whether or not the pad should be active.

# Returns

`true` if the operation was successful.

MT safe.
<!-- trait PadExt::fn set_chain_function_full -->
Sets the given chain function for the pad. The chain function is called to
process a `Buffer` input buffer. see `GstPadChainFunction` for more details.
## `chain`
the `GstPadChainFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `chain` will not be used anymore.
<!-- trait PadExt::fn set_chain_list_function_full -->
Sets the given chain list function for the pad. The chainlist function is
called to process a `BufferList` input buffer list. See
`GstPadChainListFunction` for more details.
## `chainlist`
the `GstPadChainListFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `chainlist` will not be used anymore.
<!-- trait PadExt::fn set_element_private -->
Set the given private data gpointer on the pad.
This function can only be used by the element that owns the pad.
No locking is performed in this function.
## `priv_`
The private data to attach to the pad.
<!-- trait PadExt::fn set_event_full_function_full -->
Sets the given event handler for the pad.
## `event`
the `GstPadEventFullFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `event` will not be used anymore.
<!-- trait PadExt::fn set_event_function_full -->
Sets the given event handler for the pad.
## `event`
the `GstPadEventFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `event` will not be used anymore.
<!-- trait PadExt::fn set_getrange_function_full -->
Sets the given getrange function for the pad. The getrange function is
called to produce a new `Buffer` to start the processing pipeline. see
`GstPadGetRangeFunction` for a description of the getrange function.
## `get`
the `GstPadGetRangeFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `get` will not be used anymore.
<!-- trait PadExt::fn set_iterate_internal_links_function_full -->
Sets the given internal link iterator function for the pad.
## `iterintlink`
the `GstPadIterIntLinkFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `iterintlink` will not be used anymore.
<!-- trait PadExt::fn set_link_function_full -->
Sets the given link function for the pad. It will be called when
the pad is linked with another pad.

The return value `PadLinkReturn::Ok` should be used when the connection can be
made.

The return value `PadLinkReturn::Refused` should be used when the connection
cannot be made for some reason.

If `link` is installed on a source pad, it should call the `GstPadLinkFunction`
of the peer sink pad, if present.
## `link`
the `GstPadLinkFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `link` will not be used anymore.
<!-- trait PadExt::fn set_offset -->
Set the offset that will be applied to the running time of `self`.
## `offset`
the offset
<!-- trait PadExt::fn set_query_function_full -->
Set the given query function for the pad.
## `query`
the `GstPadQueryFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `query` will not be used anymore.
<!-- trait PadExt::fn set_unlink_function_full -->
Sets the given unlink function for the pad. It will be called
when the pad is unlinked.
## `unlink`
the `GstPadUnlinkFunction` to set.
## `user_data`
user_data passed to `notify`
## `notify`
notify called when `unlink` will not be used anymore.
<!-- trait PadExt::fn start_task -->
Starts a task that repeatedly calls `func` with `user_data`. This function
is mostly used in pad activation functions to start the dataflow.
The `GST_PAD_STREAM_LOCK` of `self` will automatically be acquired
before `func` is called.
## `func`
the task function to call
## `user_data`
user data passed to the task function
## `notify`
called when `user_data` is no longer referenced

# Returns

a `true` if the task could be started.
<!-- trait PadExt::fn sticky_events_foreach -->
Iterates all sticky events on `self` and calls `foreach_func` for every
event. If `foreach_func` returns `false` the iteration is immediately stopped.
## `foreach_func`
the `GstPadStickyEventsForeachFunction` that
 should be called for every event.
## `user_data`
the optional user data.
<!-- trait PadExt::fn stop_task -->
Stop the task of `self`. This function will also make sure that the
function executed by the task will effectively stop if not called
from the GstTaskFunction.

This function will deadlock if called from the GstTaskFunction of
the task. Use `Task::pause` instead.

Regardless of whether the pad has a task, the stream lock is acquired and
released so as to ensure that streaming through this pad has finished.

# Returns

a `true` if the task could be stopped or `false` on error.
<!-- trait PadExt::fn store_sticky_event -->
Store the sticky `event` on `self`
## `event`
a `Event`

# Returns

`FlowReturn::Ok` on success, `FlowReturn::Flushing` when the pad
was flushing or `FlowReturn::Eos` when the pad was EOS.
<!-- trait PadExt::fn unlink -->
Unlinks the source pad from the sink pad. Will emit the `Pad::unlinked`
signal on both pads.
## `sinkpad`
the sink `Pad` to unlink.

# Returns

`true` if the pads were unlinked. This function returns `false` if
the pads were not linked together.

MT safe.
<!-- trait PadExt::fn use_fixed_caps -->
A helper function you can use that sets the FIXED_CAPS flag
This way the default CAPS query will always return the negotiated caps
or in case the pad is not negotiated, the padtemplate caps.

The negotiated caps are the caps of the last CAPS event that passed on the
pad. Use this function on a pad that, once it negotiated to a CAPS, cannot
be renegotiated to something else.
<!-- enum PadDirection -->
The direction of a pad.
<!-- enum PadDirection::variant Unknown -->
direction is unknown.
<!-- enum PadDirection::variant Src -->
the pad is a source pad.
<!-- enum PadDirection::variant Sink -->
the pad is a sink pad.
<!-- enum PadLinkReturn -->
Result values from gst_pad_link and friends.
<!-- enum PadLinkReturn::variant Ok -->
link succeeded
<!-- enum PadLinkReturn::variant WrongHierarchy -->
pads have no common grandparent
<!-- enum PadLinkReturn::variant WasLinked -->
pad was already linked
<!-- enum PadLinkReturn::variant WrongDirection -->
pads have wrong direction
<!-- enum PadLinkReturn::variant Noformat -->
pads do not have common format
<!-- enum PadLinkReturn::variant Nosched -->
pads cannot cooperate in scheduling
<!-- enum PadLinkReturn::variant Refused -->
refused for some reason
<!-- enum PadMode -->
The status of a GstPad. After activating a pad, which usually happens when the
parent element goes from READY to PAUSED, the GstPadMode defines if the
pad operates in push or pull mode.
<!-- enum PadMode::variant None -->
Pad will not handle dataflow
<!-- enum PadMode::variant Push -->
Pad handles dataflow in downstream push mode
<!-- enum PadMode::variant Pull -->
Pad handles dataflow in upstream pull mode
<!-- enum PadPresence -->
Indicates when this pad will become available.
<!-- enum PadPresence::variant Always -->
the pad is always available
<!-- enum PadPresence::variant Sometimes -->
the pad will become available depending on the media stream
<!-- enum PadPresence::variant Request -->
the pad is only available on request with
 `ElementExt::request_pad`.
<!-- enum PadProbeReturn -->
Different return values for the `GstPadProbeCallback`.
<!-- enum PadProbeReturn::variant Drop -->
drop data in data probes. For push mode this means that
 the data item is not sent downstream. For pull mode, it means that
 the data item is not passed upstream. In both cases, no other probes
 are called for this item and `FlowReturn::Ok` or `true` is returned to the
 caller.
<!-- enum PadProbeReturn::variant Ok -->
normal probe return value. This leaves the probe in
 place, and defers decisions about dropping or passing data to other
 probes, if any. If there are no other probes, the default behaviour
 for the probe type applies ('block' for blocking probes,
 and 'pass' for non-blocking probes).
<!-- enum PadProbeReturn::variant Remove -->
remove this probe.
<!-- enum PadProbeReturn::variant Pass -->
pass the data item in the block probe and block on the
 next item.
<!-- enum PadProbeReturn::variant Handled -->
Data has been handled in the probe and will not be
 forwarded further. For events and buffers this is the same behaviour as
 `PadProbeReturn::Drop` (except that in this case you need to unref the buffer
 or event yourself). For queries it will also return `true` to the caller.
 The probe can also modify the `FlowReturn` value by using the
 `GST_PAD_PROBE_INFO_FLOW_RETURN`() accessor.
 Note that the resulting query must contain valid entries.
 Since: 1.6
<!-- struct PadTemplate -->
Padtemplates describe the possible media types a pad or an elementfactory can
handle. This allows for both inspection of handled types before loading the
element plugin as well as identifying pads on elements that are not yet
created (request or sometimes pads).

Pad and PadTemplates have `Caps` attached to it to describe the media type
they are capable of dealing with. `PadTemplateExt::get_caps` or
GST_PAD_TEMPLATE_CAPS() are used to get the caps of a padtemplate. It's not
possible to modify the caps of a padtemplate after creation.

PadTemplates have a `PadPresence` property which identifies the lifetime
of the pad and that can be retrieved with GST_PAD_TEMPLATE_PRESENCE(). Also
the direction of the pad can be retrieved from the `PadTemplate` with
GST_PAD_TEMPLATE_DIRECTION().

The GST_PAD_TEMPLATE_NAME_TEMPLATE () is important for GST_PAD_REQUEST pads
because it has to be used as the name in the `ElementExt::get_request_pad`
call to instantiate a pad from this template.

Padtemplates can be created with `PadTemplate::new` or with
gst_static_pad_template_get (), which creates a `PadTemplate` from a
`StaticPadTemplate` that can be filled with the
convenient GST_STATIC_PAD_TEMPLATE() macro.

A padtemplate can be used to create a pad (see `Pad::new_from_template`
or gst_pad_new_from_static_template ()) or to add to an element class
(see gst_element_class_add_static_pad_template ()).

The following code example shows the code to create a pad from a padtemplate.

```C
  GstStaticPadTemplate my_template =
  GST_STATIC_PAD_TEMPLATE (
    "sink",          // the name of the pad
    GST_PAD_SINK,    // the direction of the pad
    GST_PAD_ALWAYS,  // when this pad will be present
    GST_STATIC_CAPS (        // the capabilities of the padtemplate
      "audio/x-raw, "
        "channels = (int) [ 1, 6 ]"
    )
  );
  void
  my_method (void)
  {
    GstPad *pad;
    pad = gst_pad_new_from_static_template (&amp;my_template, "sink");
    ...
  }
```

The following example shows you how to add the padtemplate to an
element class, this is usually done in the class_init of the class:

```C
  static void
  my_element_class_init (GstMyElementClass *klass)
  {
    GstElementClass *gstelement_class = GST_ELEMENT_CLASS (klass);

    gst_element_class_add_static_pad_template (gstelement_class, &amp;my_template);
  }
```

# Implements

[`PadTemplateExt`](trait.PadTemplateExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait PadTemplateExt -->
Trait containing all `PadTemplate` methods.

# Implementors

[`PadTemplate`](struct.PadTemplate.html)
<!-- impl PadTemplate::fn new -->
Creates a new pad template with a name according to the given template
and with the given arguments.
## `name_template`
the name template.
## `direction`
the `PadDirection` of the template.
## `presence`
the `PadPresence` of the pad.
## `caps`
a `Caps` set for the template.

# Returns

a new `PadTemplate`.
<!-- trait PadTemplateExt::fn get_caps -->
Gets the capabilities of the pad template.

# Returns

the `Caps` of the pad template.
Unref after usage.
<!-- trait PadTemplateExt::fn pad_created -->
Emit the pad-created signal for this template when created by this pad.
## `pad`
the `Pad` that created it
<!-- enum ParseError -->
The different parsing errors that can occur.
<!-- enum ParseError::variant Syntax -->
A syntax error occurred.
<!-- enum ParseError::variant NoSuchElement -->
The description contained an unknown element
<!-- enum ParseError::variant NoSuchProperty -->
An element did not have a specified property
<!-- enum ParseError::variant Link -->
There was an error linking two pads.
<!-- enum ParseError::variant CouldNotSetProperty -->
There was an error setting a property
<!-- enum ParseError::variant EmptyBin -->
An empty bin was specified.
<!-- enum ParseError::variant Empty -->
An empty description was specified
<!-- enum ParseError::variant DelayedLink -->
A delayed link did not get resolved.
<!-- struct Pipeline -->
A `Pipeline` is a special `Bin` used as the toplevel container for
the filter graph. The `Pipeline` will manage the selection and
distribution of a global `Clock` as well as provide a `Bus` to the
application.

`Pipeline::new` is used to create a pipeline. when you are done with
the pipeline, use `GstObjectExt::unref` to free its resources including all
added `Element` objects (if not otherwise referenced).

Elements are added and removed from the pipeline using the `Bin`
methods like `BinExt::add` and `BinExt::remove` (see `Bin`).

Before changing the state of the `Pipeline` (see `Element`) a `Bus`
can be retrieved with `Pipeline::get_bus`. This bus can then be
used to receive `Message` from the elements in the pipeline.

By default, a `Pipeline` will automatically flush the pending `Bus`
messages when going to the NULL state to ensure that no circular
references exist when no messages are read from the `Bus`. This
behaviour can be changed with `PipelineExt::set_auto_flush_bus`.

When the `Pipeline` performs the PAUSED to PLAYING state change it will
select a clock for the elements. The clock selection algorithm will by
default select a clock provided by an element that is most upstream
(closest to the source). For live pipelines (ones that return
`StateChangeReturn::NoPreroll` from the `ElementExt::set_state` call) this
will select the clock provided by the live source. For normal pipelines
this will select a clock provided by the sinks (most likely the audio
sink). If no element provides a clock, a default `SystemClock` is used.

The clock selection can be controlled with the `PipelineExt::use_clock`
method, which will enforce a given clock on the pipeline. With
`PipelineExt::auto_clock` the default clock selection algorithm can be
restored.

A `Pipeline` maintains a running time for the elements. The running
time is defined as the difference between the current clock time and
the base time. When the pipeline goes to READY or a flushing seek is
performed on it, the running time is reset to 0. When the pipeline is
set from PLAYING to PAUSED, the current clock time is sampled and used to
configure the base time for the elements when the pipeline is set
to PLAYING again. The effect is that the running time (as the difference
between the clock time and the base time) will count how much time was spent
in the PLAYING state. This default behaviour can be changed with the
`ElementExt::set_start_time` method.

# Implements

[`PipelineExt`](trait.PipelineExt.html), [`BinExt`](trait.BinExt.html), [`ElementExt`](trait.ElementExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ChildProxyExt`](trait.ChildProxyExt.html)
<!-- trait PipelineExt -->
Trait containing all `Pipeline` methods.

# Implementors

[`Pipeline`](struct.Pipeline.html)
<!-- impl Pipeline::fn new -->
Create a new pipeline with the given name.
## `name`
name of new pipeline

# Returns

newly created GstPipeline

MT safe.
<!-- trait PipelineExt::fn auto_clock -->
Let `self` select a clock automatically. This is the default
behaviour.

Use this function if you previous forced a fixed clock with
`PipelineExt::use_clock` and want to restore the default
pipeline clock selection algorithm.

MT safe.
<!-- trait PipelineExt::fn get_auto_flush_bus -->
Check if `self` will automatically flush messages when going to
the NULL state.

# Returns

whether the pipeline will automatically flush its bus when
going from READY to NULL state or not.

MT safe.
<!-- trait PipelineExt::fn get_bus -->
Gets the `Bus` of `self`. The bus allows applications to receive
`Message` packets.

# Returns

a `Bus`, unref after usage.

MT safe.
<!-- trait PipelineExt::fn get_clock -->
Gets the current clock used by `self`. Users of object
oriented languages should use `PipelineExt::get_pipeline_clock`
to avoid confusion with `ElementExt::get_clock` which has a different behavior.

Unlike `ElementExt::get_clock`, this function will always return a
clock, even if the pipeline is not in the PLAYING state.

# Returns

a `Clock`, unref after usage.
<!-- trait PipelineExt::fn get_delay -->
Get the configured delay (see `PipelineExt::set_delay`).

# Returns

The configured delay.

MT safe.
<!-- trait PipelineExt::fn get_latency -->
Gets the latency that should be configured on the pipeline. See
`PipelineExt::set_latency`.

# Returns

Latency to configure on the pipeline or GST_CLOCK_TIME_NONE
<!-- trait PipelineExt::fn get_pipeline_clock -->
Gets the current clock used by `self`.

Unlike `ElementExt::get_clock`, this function will always return a
clock, even if the pipeline is not in the PLAYING state.

# Returns

a `Clock`, unref after usage.
<!-- trait PipelineExt::fn set_auto_flush_bus -->
Usually, when a pipeline goes from READY to NULL state, it automatically
flushes all pending messages on the bus, which is done for refcounting
purposes, to break circular references.

This means that applications that update state using (async) bus messages
(e.g. do certain things when a pipeline goes from PAUSED to READY) might
not get to see messages when the pipeline is shut down, because they might
be flushed before they can be dispatched in the main thread. This behaviour
can be disabled using this function.

It is important that all messages on the bus are handled when the
automatic flushing is disabled else memory leaks will be introduced.

MT safe.
## `auto_flush`
whether or not to automatically flush the bus when
the pipeline goes from READY to NULL state
<!-- trait PipelineExt::fn set_clock -->
Set the clock for `self`. The clock will be distributed
to all the elements managed by the pipeline.
## `clock`
the clock to set

# Returns

`true` if the clock could be set on the pipeline. `false` if
 some element did not accept the clock.

MT safe.
<!-- trait PipelineExt::fn set_delay -->
Set the expected delay needed for all elements to perform the
PAUSED to PLAYING state change. `delay` will be added to the
base time of the elements so that they wait an additional `delay`
amount of time before starting to process buffers and cannot be
`GST_CLOCK_TIME_NONE`.

This option is used for tuning purposes and should normally not be
used.

MT safe.
## `delay`
the delay
<!-- trait PipelineExt::fn set_latency -->
Sets the latency that should be configured on the pipeline. Setting
GST_CLOCK_TIME_NONE will restore the default behaviour of using the minimum
latency from the LATENCY query. Setting this is usually not required and
the pipeline will figure out an appropriate latency automatically.

Setting a too low latency, especially lower than the minimum latency from
the LATENCY query, will most likely cause the pipeline to fail.
## `latency`
latency to configure
<!-- trait PipelineExt::fn use_clock -->
Force `self` to use the given `clock`. The pipeline will
always use the given clock even if new clock providers are added
to this pipeline.

If `clock` is `None` all clocking will be disabled which will make
the pipeline run as fast as possible.

MT safe.
## `clock`
the clock to use
<!-- struct Plugin -->
GStreamer is extensible, so `Element` instances can be loaded at runtime.
A plugin system can provide one or more of the basic
`<application>`GStreamer`</application>` `PluginFeature` subclasses.

A plugin should export a symbol `<symbol>`gst_plugin_desc`</symbol>` that is a
struct of type `PluginDesc`.
the plugin loader will check the version of the core library the plugin was
linked against and will create a new `Plugin`. It will then call the
`GstPluginInitFunc` function that was provided in the
`<symbol>`gst_plugin_desc`</symbol>`.

Once you have a handle to a `Plugin` (e.g. from the `Registry`), you
can add any object that subclasses `PluginFeature`.

Usually plugins are always automatically loaded so you don't need to call
`Plugin::load` explicitly to bring it into memory. There are options to
statically link plugins to an app or even use GStreamer without a plugin
repository in which case `Plugin::load` can be needed to bring the plugin
into memory.

# Implements

[`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- impl Plugin::fn list_free -->
Unrefs each member of `list`, then frees the list.
## `list`
list of `Plugin`
<!-- impl Plugin::fn load_by_name -->
Load the named plugin. Refs the plugin.
## `name`
name of plugin to load

# Returns

a reference to a loaded plugin, or `None` on error.
<!-- impl Plugin::fn load_file -->
Loads the given plugin and refs it. Caller needs to unref after use.
## `filename`
the plugin filename to load

# Returns

a reference to the existing loaded GstPlugin, a
reference to the newly-loaded GstPlugin, or `None` if an error occurred.
<!-- impl Plugin::fn register_static -->
Registers a static plugin, ie. a plugin which is private to an application
or library and contained within the application or library (as opposed to
being shipped as a separate module file).

You must make sure that GStreamer has been initialised (with `gst_init` or
via `gst_init_get_option_group`) before calling this function.
## `major_version`
the major version number of the GStreamer core that the
 plugin was compiled for, you can just use GST_VERSION_MAJOR here
## `minor_version`
the minor version number of the GStreamer core that the
 plugin was compiled for, you can just use GST_VERSION_MINOR here
## `name`
a unique name of the plugin (ideally prefixed with an application- or
 library-specific namespace prefix in order to avoid name conflicts in
 case a similar plugin with the same name ever gets added to GStreamer)
## `description`
description of the plugin
## `init_func`
pointer to the init function of this plugin.
## `version`
version string of the plugin
## `license`
effective license of plugin. Must be one of the approved licenses
 (see `PluginDesc` above) or the plugin will not be registered.
## `source`
source module plugin belongs to
## `package`
shipped package plugin belongs to
## `origin`
URL to provider of plugin

# Returns

`true` if the plugin was registered correctly, otherwise `false`.
<!-- impl Plugin::fn register_static_full -->
Registers a static plugin, ie. a plugin which is private to an application
or library and contained within the application or library (as opposed to
being shipped as a separate module file) with a `GstPluginInitFullFunc`
which allows user data to be passed to the callback function (useful
for bindings).

You must make sure that GStreamer has been initialised (with `gst_init` or
via `gst_init_get_option_group`) before calling this function.
## `major_version`
the major version number of the GStreamer core that the
 plugin was compiled for, you can just use GST_VERSION_MAJOR here
## `minor_version`
the minor version number of the GStreamer core that the
 plugin was compiled for, you can just use GST_VERSION_MINOR here
## `name`
a unique name of the plugin (ideally prefixed with an application- or
 library-specific namespace prefix in order to avoid name conflicts in
 case a similar plugin with the same name ever gets added to GStreamer)
## `description`
description of the plugin
## `init_full_func`
pointer to the init function with user data
 of this plugin.
## `version`
version string of the plugin
## `license`
effective license of plugin. Must be one of the approved licenses
 (see `PluginDesc` above) or the plugin will not be registered.
## `source`
source module plugin belongs to
## `package`
shipped package plugin belongs to
## `origin`
URL to provider of plugin
## `user_data`
gpointer to user data

# Returns

`true` if the plugin was registered correctly, otherwise `false`.
<!-- impl Plugin::fn add_dependency -->
Make GStreamer aware of external dependencies which affect the feature
set of this plugin (ie. the elements or typefinders associated with it).

GStreamer will re-inspect plugins with external dependencies whenever any
of the external dependencies change. This is useful for plugins which wrap
other plugin systems, e.g. a plugin which wraps a plugin-based visualisation
library and makes visualisations available as GStreamer elements, or a
codec loader which exposes elements and/or caps dependent on what external
codec libraries are currently installed.
## `env_vars`
`None`-terminated array of environment variables affecting the
 feature set of the plugin (e.g. an environment variable containing
 paths where to look for additional modules/plugins of a library),
 or `None`. Environment variable names may be followed by a path component
 which will be added to the content of the environment variable, e.g.
 "HOME/.mystuff/plugins".
## `paths`
`None`-terminated array of directories/paths where dependent files
 may be, or `None`.
## `names`
`None`-terminated array of file names (or file name suffixes,
 depending on `flags`) to be used in combination with the paths from
 `paths` and/or the paths extracted from the environment variables in
 `env_vars`, or `None`.
## `flags`
optional flags, or `PluginDependencyFlags::None`
<!-- impl Plugin::fn add_dependency_simple -->
Make GStreamer aware of external dependencies which affect the feature
set of this plugin (ie. the elements or typefinders associated with it).

GStreamer will re-inspect plugins with external dependencies whenever any
of the external dependencies change. This is useful for plugins which wrap
other plugin systems, e.g. a plugin which wraps a plugin-based visualisation
library and makes visualisations available as GStreamer elements, or a
codec loader which exposes elements and/or caps dependent on what external
codec libraries are currently installed.

Convenience wrapper function for `Plugin::add_dependency` which
takes simple strings as arguments instead of string arrays, with multiple
arguments separated by predefined delimiters (see above).
## `env_vars`
one or more environment variables (separated by ':', ';' or ','),
 or `None`. Environment variable names may be followed by a path component
 which will be added to the content of the environment variable, e.g.
 "HOME/.mystuff/plugins:MYSTUFF_PLUGINS_PATH"
## `paths`
one ore more directory paths (separated by ':' or ';' or ','),
 or `None`. Example: "/usr/lib/mystuff/plugins"
## `names`
one or more file names or file name suffixes (separated by commas),
 or `None`
## `flags`
optional flags, or `PluginDependencyFlags::None`
<!-- impl Plugin::fn get_cache_data -->
Gets the plugin specific data cache. If it is `None` there is no cached data
stored. This is the case when the registry is getting rebuilt.

# Returns

The cached data as a
`Structure` or `None`.
<!-- impl Plugin::fn get_description -->
Get the long descriptive name of the plugin

# Returns

the long name of the plugin
<!-- impl Plugin::fn get_filename -->
get the filename of the plugin

# Returns

the filename of the plugin
<!-- impl Plugin::fn get_license -->
get the license of the plugin

# Returns

the license of the plugin
<!-- impl Plugin::fn get_name -->
Get the short name of the plugin

# Returns

the name of the plugin
<!-- impl Plugin::fn get_origin -->
get the URL where the plugin comes from

# Returns

the origin of the plugin
<!-- impl Plugin::fn get_package -->
get the package the plugin belongs to.

# Returns

the package of the plugin
<!-- impl Plugin::fn get_release_date_string -->
Get the release date (and possibly time) in form of a string, if available.

For normal GStreamer plugin releases this will usually just be a date in
the form of "YYYY-MM-DD", while pre-releases and builds from git may contain
a time component after the date as well, in which case the string will be
formatted like "YYYY-MM-DDTHH:MMZ" (e.g. "2012-04-30T09:30Z").

There may be plugins that do not have a valid release date set on them.

# Returns

the date string of the plugin, or `None` if not
available.
<!-- impl Plugin::fn get_source -->
get the source module the plugin belongs to.

# Returns

the source of the plugin
<!-- impl Plugin::fn get_version -->
get the version of the plugin

# Returns

the version of the plugin
<!-- impl Plugin::fn is_loaded -->
queries if the plugin is loaded into memory

# Returns

`true` is loaded, `false` otherwise
<!-- impl Plugin::fn load -->
Loads `self`. Note that the *return value* is the loaded plugin; `self` is
untouched. The normal use pattern of this function goes like this:


```text
GstPlugin *loaded_plugin;
loaded_plugin = gst_plugin_load (plugin);
// presumably, we're no longer interested in the potentially-unloaded plugin
gst_object_unref (plugin);
plugin = loaded_plugin;
```

# Returns

a reference to a loaded plugin, or `None` on error.
<!-- impl Plugin::fn set_cache_data -->
Adds plugin specific data to cache. Passes the ownership of the structure to
the `self`.

The cache is flushed every time the registry is rebuilt.
## `cache_data`
a structure containing the data to cache
<!-- enum PluginError -->
The plugin loading errors
<!-- enum PluginError::variant Module -->
The plugin could not be loaded
<!-- enum PluginError::variant Dependencies -->
The plugin has unresolved dependencies
<!-- enum PluginError::variant NameMismatch -->
The plugin has already be loaded from a different file
<!-- struct Preset -->
This interface offers methods to query and manipulate parameter preset sets.
A preset is a bunch of property settings, together with meta data and a name.
The name of a preset serves as key for subsequent method calls to manipulate
single presets.
All instances of one type will share the list of presets. The list is created
on demand, if presets are not used, the list is not created.

The interface comes with a default implementation that serves most plugins.
Wrapper plugins will override most methods to implement support for the
native preset format of those wrapped plugins.
One method that is useful to be overridden is `Preset::get_property_names`.
With that one can control which properties are saved and in which order.
When implementing support for read-only presets, one should set the vmethods
for `Preset::save_preset` and `Preset::delete_preset` to `None`.
Applications can use `Preset::is_editable` to check for that.

The default implementation supports presets located in a system directory,
application specific directory and in the users home directory. When getting
a list of presets individual presets are read and overlaid in 1) system,
2) application and 3) user order. Whenever an earlier entry is newer, the
later entries will be updated. Since 1.8 you can also provide extra paths
where to find presets through the GST_PRESET_PATH environment variable.
Presets found in those paths will be concidered as "app presets".

# Implements

[`PresetExt`](trait.PresetExt.html)
<!-- trait PresetExt -->
Trait containing all `Preset` methods.

# Implementors

[`Preset`](struct.Preset.html)
<!-- impl Preset::fn get_app_dir -->
Gets the directory for application specific presets if set by the
application.

# Returns

the directory or `None`, don't free or modify
the string
<!-- impl Preset::fn set_app_dir -->
Sets an extra directory as an absolute path that should be considered when
looking for presets. Any presets in the application dir will shadow the
system presets.
## `app_dir`
the application specific preset dir

# Returns

`true` for success, `false` if the dir already has been set
<!-- trait PresetExt::fn delete_preset -->
Delete the given preset.
## `name`
preset name to remove

# Returns

`true` for success, `false` if e.g. there is no preset with that `name`
<!-- trait PresetExt::fn get_meta -->
Gets the `value` for an existing meta data `tag`. Meta data `tag` names can be
something like e.g. "comment". Returned values need to be released when done.
## `name`
preset name
## `tag`
meta data item name
## `value`
value

# Returns

`true` for success, `false` if e.g. there is no preset with that `name`
or no value for the given `tag`
<!-- trait PresetExt::fn get_preset_names -->
Get a copy of preset names as a `None` terminated string array.

# Returns


 list with names, use `g_strfreev` after usage.
<!-- trait PresetExt::fn get_property_names -->
Get a the names of the GObject properties that can be used for presets.

# Returns

an
 array of property names which should be freed with `g_strfreev` after use.
<!-- trait PresetExt::fn is_editable -->
Check if one can add new presets, change existing ones and remove presets.

# Returns

`true` if presets are editable or `false` if they are static
<!-- trait PresetExt::fn load_preset -->
Load the given preset.
## `name`
preset name to load

# Returns

`true` for success, `false` if e.g. there is no preset with that `name`
<!-- trait PresetExt::fn rename_preset -->
Renames a preset. If there is already a preset by the `new_name` it will be
overwritten.
## `old_name`
current preset name
## `new_name`
new preset name

# Returns

`true` for success, `false` if e.g. there is no preset with `old_name`
<!-- trait PresetExt::fn save_preset -->
Save the current object settings as a preset under the given name. If there
is already a preset by this `name` it will be overwritten.
## `name`
preset name to save

# Returns

`true` for success, `false`
<!-- trait PresetExt::fn set_meta -->
Sets a new `value` for an existing meta data item or adds a new item. Meta
data `tag` names can be something like e.g. "comment". Supplying `None` for the
`value` will unset an existing value.
## `name`
preset name
## `tag`
meta data item name
## `value`
new value

# Returns

`true` for success, `false` if e.g. there is no preset with that `name`
<!-- enum ProgressType -->
The type of a `MessageType::Progress`. The progress messages inform the
application of the status of asynchronous tasks.
<!-- enum ProgressType::variant Start -->
A new task started.
<!-- enum ProgressType::variant Continue -->
A task completed and a new one continues.
<!-- enum ProgressType::variant Complete -->
A task completed.
<!-- enum ProgressType::variant Canceled -->
A task was canceled.
<!-- enum ProgressType::variant Error -->
A task caused an error. An error message is also
 posted on the bus.
<!-- struct ProxyPad -->


# Implements

[`ProxyPadExt`](trait.ProxyPadExt.html), [`PadExt`](trait.PadExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait ProxyPadExt -->
Trait containing all `ProxyPad` methods.

# Implementors

[`GhostPad`](struct.GhostPad.html), [`ProxyPad`](struct.ProxyPad.html)
<!-- impl ProxyPad::fn chain_default -->
Invoke the default chain function of the proxy pad.
## `pad`
a sink `Pad`, returns GST_FLOW_ERROR if not.
## `parent`
the parent of `pad` or `None`
## `buffer`
the `Buffer` to send, return GST_FLOW_ERROR
 if not.

# Returns

a `FlowReturn` from the pad.
<!-- impl ProxyPad::fn chain_list_default -->
Invoke the default chain list function of the proxy pad.
## `pad`
a sink `Pad`, returns GST_FLOW_ERROR if not.
## `parent`
the parent of `pad` or `None`
## `list`
the `BufferList` to send, return GST_FLOW_ERROR
 if not.

# Returns

a `FlowReturn` from the pad.
<!-- impl ProxyPad::fn getrange_default -->
Invoke the default getrange function of the proxy pad.
## `pad`
a src `Pad`, returns `FlowReturn::Error` if not.
## `parent`
the parent of `pad`
## `offset`
The start offset of the buffer
## `size`
The length of the buffer
## `buffer`
a pointer to hold the `Buffer`,
 returns `FlowReturn::Error` if `None`.

# Returns

a `FlowReturn` from the pad.
<!-- impl ProxyPad::fn iterate_internal_links_default -->
Invoke the default iterate internal links function of the proxy pad.
## `pad`
the `Pad` to get the internal links of.
## `parent`
the parent of `pad` or `None`

# Returns

a `Iterator` of `Pad`, or `None` if `pad`
has no parent. Unref each returned pad with `GstObjectExt::unref`.
<!-- trait ProxyPadExt::fn get_internal -->
Get the internal pad of `self`. Unref target pad after usage.

The internal pad of a `GhostPad` is the internally used
pad of opposite direction, which is used to link to the target.

# Returns

the target `ProxyPad`, can
be `None`. Unref target pad after usage.
<!-- enum QOSType -->
The different types of QoS events that can be given to the
`Event::new_qos` method.
<!-- enum QOSType::variant Overflow -->
The QoS event type that is produced when upstream
 elements are producing data too quickly and the element can't keep up
 processing the data. Upstream should reduce their production rate. This
 type is also used when buffers arrive early or in time.
<!-- enum QOSType::variant Underflow -->
The QoS event type that is produced when upstream
 elements are producing data too slowly and need to speed up their
 production rate.
<!-- enum QOSType::variant Throttle -->
The QoS event type that is produced when the
 application enabled throttling to limit the data rate.
<!-- struct Query -->
Queries can be performed on pads (`Pad::query`) and elements
(`Element::query`). Please note that some queries might need a running
pipeline to work.

Queries can be created using the gst_query_new_*() functions.
Query values can be set using gst_query_set_*(), and parsed using
gst_query_parse_*() helpers.

The following example shows how to query the duration of a pipeline:

```C
  GstQuery *query;
  gboolean res;
  query = gst_query_new_duration (GST_FORMAT_TIME);
  res = gst_element_query (pipeline, query);
  if (res) {
    gint64 duration;
    gst_query_parse_duration (query, NULL, &amp;duration);
    g_print ("duration = %"GST_TIME_FORMAT, GST_TIME_ARGS (duration));
  } else {
    g_print ("duration query failed...");
  }
  gst_query_unref (query);
```
<!-- impl Query::fn new_accept_caps -->
Constructs a new query object for querying if `caps` are accepted.

Free-function: `gst_query_unref`
## `caps`
a fixed `Caps`

# Returns

a new `Query`
<!-- impl Query::fn new_allocation -->
Constructs a new query object for querying the allocation properties.

Free-function: `gst_query_unref`
## `caps`
the negotiated caps
## `need_pool`
return a pool

# Returns

a new `Query`
<!-- impl Query::fn new_buffering -->
Constructs a new query object for querying the buffering status of
a stream.

Free-function: `gst_query_unref`
## `format`
the default `Format` for the new query

# Returns

a new `Query`
<!-- impl Query::fn new_caps -->
Constructs a new query object for querying the caps.

The CAPS query should return the allowable caps for a pad in the context
of the element's state, its link to other elements, and the devices or files
it has opened. These caps must be a subset of the pad template caps. In the
NULL state with no links, the CAPS query should ideally return the same caps
as the pad template. In rare circumstances, an object property can affect
the caps returned by the CAPS query, but this is discouraged.

For most filters, the caps returned by CAPS query is directly affected by the
allowed caps on other pads. For demuxers and decoders, the caps returned by
the srcpad's getcaps function is directly related to the stream data. Again,
the CAPS query should return the most specific caps it reasonably can, since this
helps with autoplugging.

The `filter` is used to restrict the result caps, only the caps matching
`filter` should be returned from the CAPS query. Specifying a filter might
greatly reduce the amount of processing an element needs to do.

Free-function: `gst_query_unref`
## `filter`
a filter

# Returns

a new `Query`
<!-- impl Query::fn new_context -->
Constructs a new query object for querying the pipeline-local context.

Free-function: `gst_query_unref`
## `context_type`
Context type to query

# Returns

a new `Query`
<!-- impl Query::fn new_convert -->
Constructs a new convert query object. Use `gst_query_unref`
when done with it. A convert query is used to ask for a conversion between
one format and another.

Free-function: `gst_query_unref`
## `src_format`
the source `Format` for the new query
## `value`
the value to convert
## `dest_format`
the target `Format`

# Returns

a `Query`
<!-- impl Query::fn new_custom -->
Constructs a new custom query object. Use `gst_query_unref`
when done with it.

Free-function: `gst_query_unref`
## `type_`
the query type
## `structure`
a structure for the query

# Returns

a new `Query`
<!-- impl Query::fn new_drain -->
Constructs a new query object for querying the drain state.

Free-function: `gst_query_unref`

# Returns

a new `Query`
<!-- impl Query::fn new_duration -->
Constructs a new stream duration query object to query in the given format.
Use `gst_query_unref` when done with it. A duration query will give the
total length of the stream.

Free-function: `gst_query_unref`
## `format`
the `Format` for this duration query

# Returns

a new `Query`
<!-- impl Query::fn new_formats -->
Constructs a new query object for querying formats of
the stream.

Free-function: `gst_query_unref`

# Returns

a new `Query`
<!-- impl Query::fn new_latency -->
Constructs a new latency query object.
Use `gst_query_unref` when done with it. A latency query is usually performed
by sinks to compensate for additional latency introduced by elements in the
pipeline.

Free-function: `gst_query_unref`

# Returns

a `Query`
<!-- impl Query::fn new_position -->
Constructs a new query stream position query object. Use `gst_query_unref`
when done with it. A position query is used to query the current position
of playback in the streams, in some format.

Free-function: `gst_query_unref`
## `format`
the default `Format` for the new query

# Returns

a new `Query`
<!-- impl Query::fn new_scheduling -->
Constructs a new query object for querying the scheduling properties.

Free-function: `gst_query_unref`

# Returns

a new `Query`
<!-- impl Query::fn new_seeking -->
Constructs a new query object for querying seeking properties of
the stream.

Free-function: `gst_query_unref`
## `format`
the default `Format` for the new query

# Returns

a new `Query`
<!-- impl Query::fn new_segment -->
Constructs a new segment query object. Use `gst_query_unref`
when done with it. A segment query is used to discover information about the
currently configured segment for playback.

Free-function: `gst_query_unref`
## `format`
the `Format` for the new query

# Returns

a new `Query`
<!-- impl Query::fn new_uri -->
Constructs a new query URI query object. Use `gst_query_unref`
when done with it. An URI query is used to query the current URI
that is used by the source or sink.

Free-function: `gst_query_unref`

# Returns

a new `Query`
<!-- impl Query::fn add_allocation_meta -->
Add `api` with `params` as one of the supported metadata API to `self`.
## `api`
the metadata API
## `params`
API specific parameters
<!-- impl Query::fn add_allocation_param -->
Add `allocator` and its `params` as a supported memory allocator.
## `allocator`
the memory allocator
## `params`
a `AllocationParams`
<!-- impl Query::fn add_allocation_pool -->
Set the pool parameters in `self`.
## `pool`
the `BufferPool`
## `size`
the buffer size
## `min_buffers`
the min buffers
## `max_buffers`
the max buffers
<!-- impl Query::fn add_buffering_range -->
Set the buffering-ranges array field in `self`. The current last
start position of the array should be inferior to `start`.
## `start`
start position of the range
## `stop`
stop position of the range

# Returns

a `gboolean` indicating if the range was added or not.
<!-- impl Query::fn add_scheduling_mode -->
Add `mode` as one of the supported scheduling modes to `self`.
## `mode`
a `PadMode`
<!-- impl Query::fn find_allocation_meta -->
Check if `self` has metadata `api` set. When this function returns `true`,
`index` will contain the index where the requested API and the parameters
can be found.
## `api`
the metadata API
## `index`
the index

# Returns

`true` when `api` is in the list of metadata.
<!-- impl Query::fn get_n_allocation_metas -->
Retrieve the number of values currently stored in the
meta API array of the query's structure.

# Returns

the metadata API array size as a `guint`.
<!-- impl Query::fn get_n_allocation_params -->
Retrieve the number of values currently stored in the
allocator params array of the query's structure.

If no memory allocator is specified, the downstream element can handle
the default memory allocator. The first memory allocator in the query
should be generic and allow mapping to system memory, all following
allocators should be ordered by preference with the preferred one first.

# Returns

the allocator array size as a `guint`.
<!-- impl Query::fn get_n_allocation_pools -->
Retrieve the number of values currently stored in the
pool array of the query's structure.

# Returns

the pool array size as a `guint`.
<!-- impl Query::fn get_n_buffering_ranges -->
Retrieve the number of values currently stored in the
buffered-ranges array of the query's structure.

# Returns

the range array size as a `guint`.
<!-- impl Query::fn get_n_scheduling_modes -->
Retrieve the number of values currently stored in the
scheduling mode array of the query's structure.

# Returns

the scheduling mode array size as a `guint`.
<!-- impl Query::fn get_structure -->
Get the structure of a query.

# Returns

the `Structure` of the query. The structure is
 still owned by the query and will therefore be freed when the query
 is unreffed.
<!-- impl Query::fn has_scheduling_mode -->
Check if `self` has scheduling mode set.

> When checking if upstream supports pull mode, it is usually not
> enough to just check for GST_PAD_MODE_PULL with this function, you
> also want to check whether the scheduling flags returned by
> `Query::parse_scheduling` have the seeking flag set (meaning
> random access is supported, not only sequential pulls).
## `mode`
the scheduling mode

# Returns

`true` when `mode` is in the list of scheduling modes.
<!-- impl Query::fn has_scheduling_mode_with_flags -->
Check if `self` has scheduling mode set and `flags` is set in
query scheduling flags.
## `mode`
the scheduling mode
## `flags`
`SchedulingFlags`

# Returns

`true` when `mode` is in the list of scheduling modes
 and `flags` are compatible with query flags.
<!-- impl Query::fn parse_accept_caps -->
Get the caps from `self`. The caps remains valid as long as `self` remains
valid.
## `caps`
A pointer to the caps
<!-- impl Query::fn parse_accept_caps_result -->
Parse the result from `self` and store in `result`.
## `result`
location for the result
<!-- impl Query::fn parse_allocation -->
Parse an allocation query, writing the requested caps in `caps` and
whether a pool is needed in `need_pool`, if the respective parameters
are non-`None`.

Pool details can be retrieved using `Query::get_n_allocation_pools` and
`Query::parse_nth_allocation_pool`.
## `caps`
The `Caps`
## `need_pool`
Whether a `BufferPool` is needed
<!-- impl Query::fn parse_buffering_percent -->
Get the percentage of buffered data. This is a value between 0 and 100.
The `busy` indicator is `true` when the buffering is in progress.
## `busy`
if buffering is busy, or `None`
## `percent`
a buffering percent, or `None`
<!-- impl Query::fn parse_buffering_range -->
Parse an available query, writing the format into `format`, and
other results into the passed parameters, if the respective parameters
are non-`None`
## `format`
the format to set for the `segment_start`
 and `segment_end` values, or `None`
## `start`
the start to set, or `None`
## `stop`
the stop to set, or `None`
## `estimated_total`
estimated total amount of download
 time remaining in milliseconds, or `None`
<!-- impl Query::fn parse_buffering_stats -->
Extracts the buffering stats values from `self`.
## `mode`
a buffering mode, or `None`
## `avg_in`
the average input rate, or `None`
## `avg_out`
the average output rat, or `None`
## `buffering_left`
amount of buffering time left in
 milliseconds, or `None`
<!-- impl Query::fn parse_caps -->
Get the filter from the caps `self`. The caps remains valid as long as
`self` remains valid.
## `filter`
A pointer to the caps filter
<!-- impl Query::fn parse_caps_result -->
Get the caps result from `self`. The caps remains valid as long as
`self` remains valid.
## `caps`
A pointer to the caps
<!-- impl Query::fn parse_context -->
Get the context from the context `self`. The context remains valid as long as
`self` remains valid.
## `context`
A pointer to store the `Context`
<!-- impl Query::fn parse_context_type -->
Parse a context type from an existing GST_QUERY_CONTEXT query.
## `context_type`
the context type, or `None`

# Returns

a `gboolean` indicating if the parsing succeeded.
<!-- impl Query::fn parse_convert -->
Parse a convert query answer. Any of `src_format`, `src_value`, `dest_format`,
and `dest_value` may be `None`, in which case that value is omitted.
## `src_format`
the storage for the `Format` of the
 source value, or `None`
## `src_value`
the storage for the source value, or `None`
## `dest_format`
the storage for the `Format` of the
 destination value, or `None`
## `dest_value`
the storage for the destination value,
 or `None`
<!-- impl Query::fn parse_duration -->
Parse a duration query answer. Write the format of the duration into `format`,
and the value into `duration`, if the respective variables are non-`None`.
## `format`
the storage for the `Format` of the duration
 value, or `None`.
## `duration`
the storage for the total duration, or `None`.
<!-- impl Query::fn parse_latency -->
Parse a latency query answer.
## `live`
storage for live or `None`
## `min_latency`
the storage for the min latency or `None`
## `max_latency`
the storage for the max latency or `None`
<!-- impl Query::fn parse_n_formats -->
Parse the number of formats in the formats `self`.
## `n_formats`
the number of formats in this query.
<!-- impl Query::fn parse_nth_allocation_meta -->
Parse an available query and get the metadata API
at `index` of the metadata API array.
## `index`
position in the metadata API array to read
## `params`
API specific parameters

# Returns

a `glib::Type` of the metadata API at `index`.
<!-- impl Query::fn parse_nth_allocation_param -->
Parse an available query and get the allocator and its params
at `index` of the allocator array.
## `index`
position in the allocator array to read
## `allocator`
variable to hold the result
## `params`
parameters for the allocator
<!-- impl Query::fn parse_nth_allocation_pool -->
Get the pool parameters in `self`.

Unref `pool` with `GstObjectExt::unref` when it's not needed any more.
## `index`
index to parse
## `pool`
the `BufferPool`
## `size`
the buffer size
## `min_buffers`
the min buffers
## `max_buffers`
the max buffers
<!-- impl Query::fn parse_nth_buffering_range -->
Parse an available query and get the start and stop values stored
at the `index` of the buffered ranges array.
## `index`
position in the buffered-ranges array to read
## `start`
the start position to set, or `None`
## `stop`
the stop position to set, or `None`

# Returns

a `gboolean` indicating if the parsing succeeded.
<!-- impl Query::fn parse_nth_format -->
Parse the format query and retrieve the `nth` format from it into
`format`. If the list contains less elements than `nth`, `format` will be
set to GST_FORMAT_UNDEFINED.
## `nth`
the nth format to retrieve.
## `format`
a pointer to store the nth format
<!-- impl Query::fn parse_nth_scheduling_mode -->
Parse an available query and get the scheduling mode
at `index` of the scheduling modes array.
## `index`
position in the scheduling modes array to read

# Returns

a `PadMode` of the scheduling mode at `index`.
<!-- impl Query::fn parse_position -->
Parse a position query, writing the format into `format`, and the position
into `cur`, if the respective parameters are non-`None`.
## `format`
the storage for the `Format` of the
 position values (may be `None`)
## `cur`
the storage for the current position (may be `None`)
<!-- impl Query::fn parse_scheduling -->
Set the scheduling properties.
## `flags`
`SchedulingFlags`
## `minsize`
the suggested minimum size of pull requests
## `maxsize`
the suggested maximum size of pull requests:
## `align`
the suggested alignment of pull requests
<!-- impl Query::fn parse_seeking -->
Parse a seeking query, writing the format into `format`, and
other results into the passed parameters, if the respective parameters
are non-`None`
## `format`
the format to set for the `segment_start`
 and `segment_end` values, or `None`
## `seekable`
the seekable flag to set, or `None`
## `segment_start`
the segment_start to set, or `None`
## `segment_end`
the segment_end to set, or `None`
<!-- impl Query::fn parse_segment -->
Parse a segment query answer. Any of `rate`, `format`, `start_value`, and
`stop_value` may be `None`, which will cause this value to be omitted.

See `Query::set_segment` for an explanation of the function arguments.
## `rate`
the storage for the rate of the segment, or `None`
## `format`
the storage for the `Format` of the values,
 or `None`
## `start_value`
the storage for the start value, or `None`
## `stop_value`
the storage for the stop value, or `None`
<!-- impl Query::fn parse_uri -->
Parse an URI query, writing the URI into `uri` as a newly
allocated string, if the respective parameters are non-`None`.
Free the string with `g_free` after usage.
## `uri`
the storage for the current URI
 (may be `None`)
<!-- impl Query::fn parse_uri_redirection -->
Parse an URI query, writing the URI into `uri` as a newly
allocated string, if the respective parameters are non-`None`.
Free the string with `g_free` after usage.
## `uri`
the storage for the redirect URI
 (may be `None`)
<!-- impl Query::fn parse_uri_redirection_permanent -->
Parse an URI query, and set `permanent` to `true` if there is a redirection
and it should be considered permanent. If a redirection is permanent,
applications should update their internal storage of the URI, otherwise
they should make all future requests to the original URI.
## `permanent`
if the URI redirection is permanent
 (may be `None`)
<!-- impl Query::fn remove_nth_allocation_meta -->
Remove the metadata API at `index` of the metadata API array.
## `index`
position in the metadata API array to remove
<!-- impl Query::fn remove_nth_allocation_param -->
Remove the allocation param at `index` of the allocation param array.
## `index`
position in the allocation param array to remove
<!-- impl Query::fn remove_nth_allocation_pool -->
Remove the allocation pool at `index` of the allocation pool array.
## `index`
position in the allocation pool array to remove
<!-- impl Query::fn set_accept_caps_result -->
Set `result` as the result for the `self`.
## `result`
the result to set
<!-- impl Query::fn set_buffering_percent -->
Set the percentage of buffered data. This is a value between 0 and 100.
The `busy` indicator is `true` when the buffering is in progress.
## `busy`
if buffering is busy
## `percent`
a buffering percent
<!-- impl Query::fn set_buffering_range -->
Set the available query result fields in `self`.
## `format`
the format to set for the `start` and `stop` values
## `start`
the start to set
## `stop`
the stop to set
## `estimated_total`
estimated total amount of download time remaining in
 milliseconds
<!-- impl Query::fn set_buffering_stats -->
Configures the buffering stats values in `self`.
## `mode`
a buffering mode
## `avg_in`
the average input rate
## `avg_out`
the average output rate
## `buffering_left`
amount of buffering time left in milliseconds
<!-- impl Query::fn set_caps_result -->
Set the `caps` result in `self`.
## `caps`
A pointer to the caps
<!-- impl Query::fn set_context -->
Answer a context query by setting the requested context.
## `context`
the requested `Context`
<!-- impl Query::fn set_convert -->
Answer a convert query by setting the requested values.
## `src_format`
the source `Format`
## `src_value`
the source value
## `dest_format`
the destination `Format`
## `dest_value`
the destination value
<!-- impl Query::fn set_duration -->
Answer a duration query by setting the requested value in the given format.
## `format`
the `Format` for the duration
## `duration`
the duration of the stream
<!-- impl Query::fn set_formats -->
Set the formats query result fields in `self`. The number of formats passed
must be equal to `n_formats`.
## `n_formats`
the number of formats to set.
<!-- impl Query::fn set_formatsv -->
Set the formats query result fields in `self`. The number of formats passed
in the `formats` array must be equal to `n_formats`.
## `n_formats`
the number of formats to set.
## `formats`
an array containing `n_formats`
 `Format` values.
<!-- impl Query::fn set_latency -->
Answer a latency query by setting the requested values in the given format.
## `live`
if there is a live element upstream
## `min_latency`
the minimal latency of the upstream elements
## `max_latency`
the maximal latency of the upstream elements
<!-- impl Query::fn set_nth_allocation_param -->
Parse an available query and get the allocator and its params
at `index` of the allocator array.
## `index`
position in the allocator array to set
## `allocator`
new allocator to set
## `params`
parameters for the allocator
<!-- impl Query::fn set_nth_allocation_pool -->
Set the pool parameters in `self`.
## `index`
index to modify
## `pool`
the `BufferPool`
## `size`
the size
## `min_buffers`
the min buffers
## `max_buffers`
the max buffers
<!-- impl Query::fn set_position -->
Answer a position query by setting the requested value in the given format.
## `format`
the requested `Format`
## `cur`
the position to set
<!-- impl Query::fn set_scheduling -->
Set the scheduling properties.
## `flags`
`SchedulingFlags`
## `minsize`
the suggested minimum size of pull requests
## `maxsize`
the suggested maximum size of pull requests
## `align`
the suggested alignment of pull requests
<!-- impl Query::fn set_seeking -->
Set the seeking query result fields in `self`.
## `format`
the format to set for the `segment_start` and `segment_end` values
## `seekable`
the seekable flag to set
## `segment_start`
the segment_start to set
## `segment_end`
the segment_end to set
<!-- impl Query::fn set_segment -->
Answer a segment query by setting the requested values. The normal
playback segment of a pipeline is 0 to duration at the default rate of
1.0. If a seek was performed on the pipeline to play a different
segment, this query will return the range specified in the last seek.

`start_value` and `stop_value` will respectively contain the configured
playback range start and stop values expressed in `format`.
The values are always between 0 and the duration of the media and
`start_value` <= `stop_value`. `rate` will contain the playback rate. For
negative rates, playback will actually happen from `stop_value` to
`start_value`.
## `rate`
the rate of the segment
## `format`
the `Format` of the segment values (`start_value` and `stop_value`)
## `start_value`
the start value
## `stop_value`
the stop value
<!-- impl Query::fn set_uri -->
Answer a URI query by setting the requested URI.
## `uri`
the URI to set
<!-- impl Query::fn set_uri_redirection -->
Answer a URI query by setting the requested URI redirection.
## `uri`
the URI to set
<!-- impl Query::fn set_uri_redirection_permanent -->
Answer a URI query by setting the requested URI redirection
to permanent or not.
## `permanent`
whether the redirect is permanent or not
<!-- impl Query::fn writable_structure -->
Get the structure of a query. This method should be called with a writable
`self` so that the returned structure is guaranteed to be writable.

# Returns

the `Structure` of the query. The structure is
 still owned by the query and will therefore be freed when the query
 is unreffed.
<!-- enum ResourceError -->
Resource errors are for any resource used by an element:
memory, files, network connections, process space, ...
They're typically used by source and sink elements.
<!-- enum ResourceError::variant Failed -->
a general error which doesn't fit in any other
category. Make sure you add a custom message to the error call.
<!-- enum ResourceError::variant TooLazy -->
do not use this except as a placeholder for
deciding where to go while developing code.
<!-- enum ResourceError::variant NotFound -->
used when the resource could not be found.
<!-- enum ResourceError::variant Busy -->
used when resource is busy.
<!-- enum ResourceError::variant OpenRead -->
used when resource fails to open for reading.
<!-- enum ResourceError::variant OpenWrite -->
used when resource fails to open for writing.
<!-- enum ResourceError::variant OpenReadWrite -->
used when resource cannot be opened for
both reading and writing, or either (but unspecified which).
<!-- enum ResourceError::variant Close -->
used when the resource can't be closed.
<!-- enum ResourceError::variant Read -->
used when the resource can't be read from.
<!-- enum ResourceError::variant Write -->
used when the resource can't be written to.
<!-- enum ResourceError::variant Seek -->
used when a seek on the resource fails.
<!-- enum ResourceError::variant Sync -->
used when a synchronize on the resource fails.
<!-- enum ResourceError::variant Settings -->
used when settings can't be manipulated on.
<!-- enum ResourceError::variant NoSpaceLeft -->
used when the resource has no space left.
<!-- enum ResourceError::variant NotAuthorized -->
used when the resource can't be opened
 due to missing authorization.
 (Since 1.2.4)
<!-- enum ResourceError::variant NumErrors -->
the number of resource error types.
<!-- enum SeekType -->
The different types of seek events. When constructing a seek event with
`Event::new_seek` or when doing gst_segment_do_seek ().
<!-- enum SeekType::variant None -->
no change in position is required
<!-- enum SeekType::variant Set -->
absolute position is requested
<!-- enum SeekType::variant End -->
relative position to duration is requested
<!-- struct Segment -->
This helper structure holds the relevant values for tracking the region of
interest in a media file, called a segment.

The structure can be used for two purposes:

 * performing seeks (handling seek events)
 * tracking playback regions (handling newsegment events)

The segment is usually configured by the application with a seek event which
is propagated upstream and eventually handled by an element that performs the seek.

The configured segment is then propagated back downstream with a newsegment event.
This information is then used to clip media to the segment boundaries.

A segment structure is initialized with `Segment::init`, which takes a `Format`
that will be used as the format of the segment values. The segment will be configured
with a start value of 0 and a stop/duration of -1, which is undefined. The default
rate and applied_rate is 1.0.

The public duration field contains the duration of the segment. When using
the segment for seeking, the start and time members should normally be left
to their default 0 value. The stop position is left to -1 unless explicitly
configured to a different value after a seek event.

The current position in the segment should be set by changing the position
member in the structure.

For elements that perform seeks, the current segment should be updated with the
`Segment::do_seek` and the values from the seek event. This method will update
all the segment fields. The position field will contain the new playback position.
If the start_type was different from GST_SEEK_TYPE_NONE, playback continues from
the position position, possibly with updated flags or rate.

For elements that want to use `Segment` to track the playback region,
update the segment fields with the information from the newsegment event.
The `Segment::clip` method can be used to check and clip
the media data to the segment boundaries.

For elements that want to synchronize to the pipeline clock, `Segment::to_running_time`
can be used to convert a timestamp to a value that can be used to synchronize
to the clock. This function takes into account the base as well as
any rate or applied_rate conversions.

For elements that need to perform operations on media data in stream_time,
`Segment::to_stream_time` can be used to convert a timestamp and the segment
info to stream time (which is always between 0 and the duration of the stream).
<!-- impl Segment::fn new -->
Allocate a new `Segment` structure and initialize it using
`Segment::init`.

Free-function: gst_segment_free

# Returns

a new `Segment`, free with `Segment::free`.
<!-- impl Segment::fn clip -->
Clip the given `start` and `stop` values to the segment boundaries given
in `self`. `start` and `stop` are compared and clipped to `self`
start and stop values.

If the function returns `false`, `start` and `stop` are known to fall
outside of `self` and `clip_start` and `clip_stop` are not updated.

When the function returns `true`, `clip_start` and `clip_stop` will be
updated. If `clip_start` or `clip_stop` are different from `start` or `stop`
respectively, the region fell partially in the segment.

Note that when `stop` is -1, `clip_stop` will be set to the end of the
segment. Depending on the use case, this may or may not be what you want.
## `format`
the format of the segment.
## `start`
the start position in the segment
## `stop`
the stop position in the segment
## `clip_start`
the clipped start position in the segment
## `clip_stop`
the clipped stop position in the segment

# Returns

`true` if the given `start` and `stop` times fall partially or
 completely in `self`, `false` if the values are completely outside
 of the segment.
<!-- impl Segment::fn copy -->
Create a copy of given `self`.

Free-function: gst_segment_free

# Returns

a new `Segment`, free with `Segment::free`.
<!-- impl Segment::fn copy_into -->
Copy the contents of `self` into `dest`.
## `dest`
a `Segment`
<!-- impl Segment::fn do_seek -->
Update the segment structure with the field values of a seek event (see
`Event::new_seek`).

After calling this method, the segment field position and time will
contain the requested new position in the segment. The new requested
position in the segment depends on `rate` and `start_type` and `stop_type`.

For positive `rate`, the new position in the segment is the new `self`
start field when it was updated with a `start_type` different from
`SeekType::None`. If no update was performed on `self` start position
(`SeekType::None`), `start` is ignored and `self` position is
unmodified.

For negative `rate`, the new position in the segment is the new `self`
stop field when it was updated with a `stop_type` different from
`SeekType::None`. If no stop was previously configured in the segment, the
duration of the segment will be used to update the stop position.
If no update was performed on `self` stop position (`SeekType::None`),
`stop` is ignored and `self` position is unmodified.

The applied rate of the segment will be set to 1.0 by default.
If the caller can apply a rate change, it should update `self`
rate and applied_rate after calling this function.

`update` will be set to `true` if a seek should be performed to the segment
position field. This field can be `false` if, for example, only the `rate`
has been changed but not the playback position.
## `rate`
the rate of the segment.
## `format`
the format of the segment.
## `flags`
the segment flags for the segment
## `start_type`
the seek method
## `start`
the seek start value
## `stop_type`
the seek method
## `stop`
the seek stop value
## `update`
boolean holding whether position was updated.

# Returns

`true` if the seek could be performed.
<!-- impl Segment::fn free -->
Free the allocated segment `self`.
<!-- impl Segment::fn init -->
The start/position fields are set to 0 and the stop/duration
fields are set to -1 (unknown). The default rate of 1.0 and no
flags are set.

Initialize `self` to its default values.
## `format`
the format of the segment.
<!-- impl Segment::fn is_equal -->
Checks for two segments being equal. Equality here is defined
as perfect equality, including floating point values.
## `s1`
a `Segment` structure.

# Returns

`true` if the segments are equal, `false` otherwise.
<!-- impl Segment::fn offset_running_time -->
Adjust the values in `self` so that `offset` is applied to all
future running-time calculations.
## `format`
the format of the segment.
## `offset`
the offset to apply in the segment

# Returns

`true` if the segment could be updated successfully. If `false` is
returned, `offset` is not in `self`.
<!-- impl Segment::fn position_from_running_time -->
Convert `running_time` into a position in the segment so that
`Segment::to_running_time` with that position returns `running_time`.
## `format`
the format of the segment.
## `running_time`
the running_time in the segment

# Returns

the position in the segment for `running_time`. This function returns
-1 when `running_time` is -1 or when it is not inside `self`.
<!-- impl Segment::fn position_from_running_time_full -->
Translate `running_time` to the segment position using the currently configured
segment. Compared to `Segment::position_from_running_time` this function can
return negative segment position.

This function is typically used by elements that need to synchronize buffers
against the clock or each other.

`running_time` can be any value and the result of this function for values
outside of the segment is extrapolated.

When 1 is returned, `running_time` resulted in a positive position returned
in `position`.

When this function returns -1, the returned `position` should be negated
to get the real negative segment position.
## `format`
the format of the segment.
## `running_time`
the running-time
## `position`
the resulting position in the segment

# Returns

a 1 or -1 on success, 0 on failure.
<!-- impl Segment::fn position_from_stream_time -->
Convert `stream_time` into a position in the segment so that
`Segment::to_stream_time` with that position returns `stream_time`.
## `format`
the format of the segment.
## `stream_time`
the stream_time in the segment

# Returns

the position in the segment for `stream_time`. This function returns
-1 when `stream_time` is -1 or when it is not inside `self`.
<!-- impl Segment::fn position_from_stream_time_full -->
Translate `stream_time` to the segment position using the currently configured
segment. Compared to `Segment::position_from_stream_time` this function can
return negative segment position.

This function is typically used by elements that need to synchronize buffers
against the clock or each other.

`stream_time` can be any value and the result of this function for values outside
of the segment is extrapolated.

When 1 is returned, `stream_time` resulted in a positive position returned
in `position`.

When this function returns -1, the returned `position` should be negated
to get the real negative segment position.
## `format`
the format of the segment.
## `stream_time`
the stream-time
## `position`
the resulting position in the segment

# Returns

a 1 or -1 on success, 0 on failure.
<!-- impl Segment::fn set_running_time -->
Adjust the start/stop and base values of `self` such that the next valid
buffer will be one with `running_time`.
## `format`
the format of the segment.
## `running_time`
the running_time in the segment

# Returns

`true` if the segment could be updated successfully. If `false` is
returned, `running_time` is -1 or not in `self`.
<!-- impl Segment::fn to_position -->
Convert `running_time` into a position in the segment so that
`Segment::to_running_time` with that position returns `running_time`.
## `format`
the format of the segment.
## `running_time`
the running_time in the segment

# Returns

the position in the segment for `running_time`. This function returns
-1 when `running_time` is -1 or when it is not inside `self`.

Deprecated. Use `Segment::position_from_running_time` instead.
<!-- impl Segment::fn to_running_time -->
Translate `position` to the total running time using the currently configured
segment. Position is a value between `self` start and stop time.

This function is typically used by elements that need to synchronize to the
global clock in a pipeline. The running time is a constantly increasing value
starting from 0. When `Segment::init` is called, this value will reset to
0.

This function returns -1 if the position is outside of `self` start and stop.
## `format`
the format of the segment.
## `position`
the position in the segment

# Returns

the position as the total running time or -1 when an invalid position
was given.
<!-- impl Segment::fn to_running_time_full -->
Translate `position` to the total running time using the currently configured
segment. Compared to `Segment::to_running_time` this function can return
negative running-time.

This function is typically used by elements that need to synchronize buffers
against the clock or eachother.

`position` can be any value and the result of this function for values outside
of the segment is extrapolated.

When 1 is returned, `position` resulted in a positive running-time returned
in `running_time`.

When this function returns -1, the returned `running_time` should be negated
to get the real negative running time.
## `format`
the format of the segment.
## `position`
the position in the segment
## `running_time`
result running-time

# Returns

a 1 or -1 on success, 0 on failure.
<!-- impl Segment::fn to_stream_time -->
Translate `position` to stream time using the currently configured
segment. The `position` value must be between `self` start and
stop value.

This function is typically used by elements that need to operate on
the stream time of the buffers it receives, such as effect plugins.
In those use cases, `position` is typically the buffer timestamp or
clock time that one wants to convert to the stream time.
The stream time is always between 0 and the total duration of the
media stream.
## `format`
the format of the segment.
## `position`
the position in the segment

# Returns

the position in stream_time or -1 when an invalid position
was given.
<!-- impl Segment::fn to_stream_time_full -->
Translate `position` to the total stream time using the currently configured
segment. Compared to `Segment::to_stream_time` this function can return
negative stream-time.

This function is typically used by elements that need to synchronize buffers
against the clock or eachother.

`position` can be any value and the result of this function for values outside
of the segment is extrapolated.

When 1 is returned, `position` resulted in a positive stream-time returned
in `stream_time`.

When this function returns -1, the returned `stream_time` should be negated
to get the real negative stream time.
## `format`
the format of the segment.
## `position`
the position in the segment
## `stream_time`
result stream-time

# Returns

a 1 or -1 on success, 0 on failure.
<!-- enum State -->
The possible states an element can be in. States can be changed using
`ElementExt::set_state` and checked using `ElementExt::get_state`.
<!-- enum State::variant VoidPending -->
no pending state.
<!-- enum State::variant Null -->
the NULL state or initial state of an element.
<!-- enum State::variant Ready -->
the element is ready to go to PAUSED.
<!-- enum State::variant Paused -->
the element is PAUSED, it is ready to accept and
 process data. Sink elements however only accept one
 buffer and then block.
<!-- enum State::variant Playing -->
the element is PLAYING, the `Clock` is running and
 the data is flowing.
<!-- enum StateChange -->
These are the different state changes an element goes through.
`State::Null` &rArr; `State::Playing` is called an upwards state change
and `State::Playing` &rArr; `State::Null` a downwards state change.
<!-- enum StateChange::variant NullToReady -->
state change from NULL to READY.
 * The element must check if the resources it needs are available. Device
 sinks and -sources typically try to probe the device to constrain their
 caps.
 * The element opens the device (in case feature need to be probed).
<!-- enum StateChange::variant ReadyToPaused -->
state change from READY to PAUSED.
 * The element pads are activated in order to receive data in PAUSED.
 Streaming threads are started.
 * Some elements might need to return `StateChangeReturn::Async` and complete
 the state change when they have enough information. It is a requirement
 for sinks to return `StateChangeReturn::Async` and complete the state change
 when they receive the first buffer or `EventType::Eos` (preroll).
 Sinks also block the dataflow when in PAUSED.
 * A pipeline resets the running_time to 0.
 * Live sources return `StateChangeReturn::NoPreroll` and don't generate data.
<!-- enum StateChange::variant PausedToPlaying -->
state change from PAUSED to PLAYING.
 * Most elements ignore this state change.
 * The pipeline selects a `Clock` and distributes this to all the children
 before setting them to PLAYING. This means that it is only allowed to
 synchronize on the `Clock` in the PLAYING state.
 * The pipeline uses the `Clock` and the running_time to calculate the
 base_time. The base_time is distributed to all children when performing
 the state change.
 * Sink elements stop blocking on the preroll buffer or event and start
 rendering the data.
 * Sinks can post `MessageType::Eos` in the PLAYING state. It is not allowed
 to post `MessageType::Eos` when not in the PLAYING state.
 * While streaming in PAUSED or PLAYING elements can create and remove
 sometimes pads.
 * Live sources start generating data and return `StateChangeReturn::Success`.
<!-- enum StateChange::variant PlayingToPaused -->
state change from PLAYING to PAUSED.
 * Most elements ignore this state change.
 * The pipeline calculates the running_time based on the last selected
 `Clock` and the base_time. It stores this information to continue
 playback when going back to the PLAYING state.
 * Sinks unblock any `Clock` wait calls.
 * When a sink does not have a pending buffer to play, it returns
 `StateChangeReturn::Async` from this state change and completes the state
 change when it receives a new buffer or an `EventType::Eos`.
 * Any queued `MessageType::Eos` items are removed since they will be reposted
 when going back to the PLAYING state. The EOS messages are queued in
 `Bin` containers.
 * Live sources stop generating data and return `StateChangeReturn::NoPreroll`.
<!-- enum StateChange::variant PausedToReady -->
state change from PAUSED to READY.
 * Sinks unblock any waits in the preroll.
 * Elements unblock any waits on devices
 * Chain or get_range functions return `FlowReturn::Flushing`.
 * The element pads are deactivated so that streaming becomes impossible and
 all streaming threads are stopped.
 * The sink forgets all negotiated formats
 * Elements remove all sometimes pads
<!-- enum StateChange::variant ReadyToNull -->
state change from READY to NULL.
 * Elements close devices
 * Elements reset any internal state.
<!-- enum StateChangeReturn -->
The possible return values from a state change function such as
`ElementExt::set_state`. Only `StateChangeReturn::Failure` is a real failure.
<!-- enum StateChangeReturn::variant Failure -->
the state change failed
<!-- enum StateChangeReturn::variant Success -->
the state change succeeded
<!-- enum StateChangeReturn::variant Async -->
the state change will happen asynchronously
<!-- enum StateChangeReturn::variant NoPreroll -->
the state change succeeded but the element
 cannot produce data in `State::Paused`.
 This typically happens with live sources.
<!-- struct Stream -->
A high-level object representing a single stream. It might be backed, or
not, by an actual flow of data in a pipeline (`Pad`).

A `Stream` does not care about data changes (such as decoding, encoding,
parsing,...) as long as the underlying data flow corresponds to the same
high-level flow (ex: a certain audio track).

A `Stream` contains all the information pertinent to a stream, such as
stream-id, tags, caps, type, ...

Elements can subclass a `Stream` for internal usage (to contain information
pertinent to streams of data).

Feature: `v1_10`

# Implements

[`StreamExt`](trait.StreamExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait StreamExt -->
Trait containing all `Stream` methods.

Feature: `v1_10`

# Implementors

[`Stream`](struct.Stream.html)
<!-- impl Stream::fn new -->
Create a new `Stream` for the given `stream_id`, `caps`, `type_`
and `flags`

Feature: `v1_10`

## `stream_id`
the id for the new stream. If `None`,
a new one will be automatically generated
## `caps`
the `Caps` of the stream
## `type_`
the `StreamType` of the stream
## `flags`
the `StreamFlags` of the stream

# Returns

The new `Stream`
<!-- trait StreamExt::fn get_caps -->
Retrieve the caps for `self`, if any

Feature: `v1_10`


# Returns

The `Caps` for `self`
<!-- trait StreamExt::fn get_stream_flags -->
Retrieve the current stream flags for `self`

Feature: `v1_10`


# Returns

The `StreamFlags` for `self`
<!-- trait StreamExt::fn get_stream_id -->
Returns the stream ID of `self`.

Feature: `v1_10`


# Returns

the stream ID of `self`. Only valid
during the lifetime of `self`.
<!-- trait StreamExt::fn get_stream_type -->
Retrieve the stream type for `self`

Feature: `v1_10`


# Returns

The `StreamType` for `self`
<!-- trait StreamExt::fn get_tags -->
Retrieve the tags for `self`, if any

Feature: `v1_10`


# Returns

The `TagList` for `self`
<!-- trait StreamExt::fn set_caps -->
Set the caps for the `Stream`

Feature: `v1_10`

## `caps`
a `Caps`
<!-- trait StreamExt::fn set_stream_flags -->
Set the `flags` for the `self`.

Feature: `v1_10`

## `flags`
the flags to set on `self`
<!-- trait StreamExt::fn set_stream_type -->
Set the stream type of `self`

Feature: `v1_10`

## `stream_type`
the type to set on `self`
<!-- trait StreamExt::fn set_tags -->
Set the tags for the `Stream`

Feature: `v1_10`

## `tags`
a `TagList`
<!-- struct StreamCollection -->
A collection of `Stream` that are available.

A `StreamCollection` will be provided by elements that can make those
streams available. Applications can use the collection to show the user
what streams are available by using `StreamCollectionExt::get_stream`()

Once posted, a `StreamCollection` is immutable. Updates are made by sending
a new `StreamCollection` message, which may or may not share some of
the `Stream` objects from the collection it replaces. The receiver can check
the sender of a stream collection message to know which collection is
obsoleted.

Several elements in a pipeline can provide `StreamCollection`.

Applications can activate streams from a collection by using the
`EventType::SelectStreams` event on a pipeline, bin or element.

Feature: `v1_10`

# Implements

[`StreamCollectionExt`](trait.StreamCollectionExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait StreamCollectionExt -->
Trait containing all `StreamCollection` methods.

Feature: `v1_10`

# Implementors

[`StreamCollection`](struct.StreamCollection.html)
<!-- impl StreamCollection::fn new -->
Create a new `StreamCollection`.

Feature: `v1_10`

## `upstream_id`
The stream id of the parent stream

# Returns

The new `StreamCollection`.
<!-- trait StreamCollectionExt::fn add_stream -->
Add the given `stream` to the `self`.

Feature: `v1_10`

## `stream`
the `Stream` to add

# Returns

`true` if the `stream` was properly added, else `false`
<!-- trait StreamCollectionExt::fn get_size -->
Get the number of streams this collection contains

Feature: `v1_10`


# Returns

The number of streams that `self` contains
<!-- trait StreamCollectionExt::fn get_stream -->
Retrieve the `Stream` with index `index` from the collection.

The caller should not modify the returned `Stream`

Feature: `v1_10`

## `index`
Index of the stream to retrieve

# Returns

A `Stream`
<!-- trait StreamCollectionExt::fn get_upstream_id -->
Returns the upstream id of the `self`.

Feature: `v1_10`


# Returns

The upstream id
<!-- enum StreamError -->
Stream errors are for anything related to the stream being processed:
format errors, media type errors, ...
They're typically used by decoders, demuxers, converters, ...
<!-- enum StreamError::variant Failed -->
a general error which doesn't fit in any other
category. Make sure you add a custom message to the error call.
<!-- enum StreamError::variant TooLazy -->
do not use this except as a placeholder for
deciding where to go while developing code.
<!-- enum StreamError::variant NotImplemented -->
use this when you do not want to implement
this functionality yet.
<!-- enum StreamError::variant TypeNotFound -->
used when the element doesn't know the
stream's type.
<!-- enum StreamError::variant WrongType -->
used when the element doesn't handle this type
of stream.
<!-- enum StreamError::variant CodecNotFound -->
used when there's no codec to handle the
stream's type.
<!-- enum StreamError::variant Decode -->
used when decoding fails.
<!-- enum StreamError::variant Encode -->
used when encoding fails.
<!-- enum StreamError::variant Demux -->
used when demuxing fails.
<!-- enum StreamError::variant Mux -->
used when muxing fails.
<!-- enum StreamError::variant Format -->
used when the stream is of the wrong format
(for example, wrong caps).
<!-- enum StreamError::variant Decrypt -->
used when the stream is encrypted and can't be
decrypted because this is not supported by the element.
<!-- enum StreamError::variant DecryptNokey -->
used when the stream is encrypted and
can't be decrypted because no suitable key is available.
<!-- enum StreamError::variant NumErrors -->
the number of stream error types.
<!-- enum StreamStatusType -->
The type of a `MessageType::StreamStatus`. The stream status messages inform the
application of new streaming threads and their status.
<!-- enum StreamStatusType::variant Create -->
A new thread need to be created.
<!-- enum StreamStatusType::variant Enter -->
a thread entered its loop function
<!-- enum StreamStatusType::variant Leave -->
a thread left its loop function
<!-- enum StreamStatusType::variant Destroy -->
a thread is destroyed
<!-- enum StreamStatusType::variant Start -->
a thread is started
<!-- enum StreamStatusType::variant Pause -->
a thread is paused
<!-- enum StreamStatusType::variant Stop -->
a thread is stopped
<!-- struct Structure -->
A `Structure` is a collection of key/value pairs. The keys are expressed
as GQuarks and the values can be of any GType.

In addition to the key/value pairs, a `Structure` also has a name. The name
starts with a letter and can be filled by letters, numbers and any of "/-_.:".

`Structure` is used by various GStreamer subsystems to store information
in a flexible and extensible way. A `Structure` does not have a refcount
because it usually is part of a higher level object such as `Caps`,
`Message`, `Event`, `Query`. It provides a means to enforce mutability
using the refcount of the parent with the `Structure::set_parent_refcount`
method.

A `Structure` can be created with `Structure::new_empty` or
`Structure::new`, which both take a name and an optional set of
key/value pairs along with the types of the values.

Field values can be changed with `Structure::set_value` or
`Structure::set`.

Field values can be retrieved with `Structure::get_value` or the more
convenient gst_structure_get_*() functions.

Fields can be removed with `Structure::remove_field` or
`Structure::remove_fields`.

Strings in structures must be ASCII or UTF-8 encoded. Other encodings are
not allowed. Strings may be `None` however.

Be aware that the current `Caps` / `Structure` serialization into string
has limited support for nested `Caps` / `Structure` fields. It can only
support one level of nesting. Using more levels will lead to unexpected
behavior when using serialization features, such as `Caps::to_string` or
`gst_value_serialize` and their counterparts.
<!-- impl Structure::fn new -->
Creates a new `Structure` with the given name. Parses the
list of variable arguments and sets fields to the values listed.
Variable arguments should be passed as field name, field type,
and value. Last variable argument should be `None`.

Free-function: gst_structure_free
## `name`
name of new structure
## `firstfield`
name of first field to set

# Returns

a new `Structure`
<!-- impl Structure::fn new_empty -->
Creates a new, empty `Structure` with the given `name`.

See `Structure::set_name` for constraints on the `name` parameter.

Free-function: gst_structure_free
## `name`
name of new structure

# Returns

a new, empty `Structure`
<!-- impl Structure::fn new_from_string -->
Creates a `Structure` from a string representation.
If end is not `None`, a pointer to the place inside the given string
where parsing ended will be returned.

The current implementation of serialization will lead to unexpected results
when there are nested `Caps` / `Structure` deeper than one level.

Free-function: gst_structure_free
## `string`
a string representation of a `Structure`

# Returns

a new `Structure` or `None`
 when the string could not be parsed. Free with
 `Structure::free` after use.
<!-- impl Structure::fn new_id -->
Creates a new `Structure` with the given name as a GQuark, followed by
fieldname quark, GType, argument(s) "triplets" in the same format as
`Structure::id_set`. Basically a convenience wrapper around
`Structure::new_id_empty` and `Structure::id_set`.

The last variable argument must be `None` (or 0).

Free-function: gst_structure_free
## `name_quark`
name of new structure
## `field_quark`
the GQuark for the name of the field to set

# Returns

a new `Structure`
<!-- impl Structure::fn new_id_empty -->
Creates a new, empty `Structure` with the given name as a GQuark.

Free-function: gst_structure_free
## `quark`
name of new structure

# Returns

a new, empty `Structure`
<!-- impl Structure::fn new_valist -->
Creates a new `Structure` with the given `name`. Structure fields
are set according to the varargs in a manner similar to
`Structure::new`.

See `Structure::set_name` for constraints on the `name` parameter.

Free-function: gst_structure_free
## `name`
name of new structure
## `firstfield`
name of first field to set
## `varargs`
variable argument list

# Returns

a new `Structure`
<!-- impl Structure::fn can_intersect -->
Tries intersecting `self` and `struct2` and reports whether the result
would not be empty.
## `struct2`
a `Structure`

# Returns

`true` if intersection would not be empty
<!-- impl Structure::fn copy -->
Duplicates a `Structure` and all its fields and values.

Free-function: gst_structure_free

# Returns

a new `Structure`.
<!-- impl Structure::fn filter_and_map_in_place -->
Calls the provided function once for each field in the `Structure`. In
contrast to `Structure::foreach`, the function may modify the fields.
In contrast to `Structure::map_in_place`, the field is removed from
the structure if `false` is returned from the function.
The structure must be mutable.
## `func`
a function to call for each field
## `user_data`
private data
<!-- impl Structure::fn fixate -->
Fixate all values in `self` using `gst_value_fixate`.
`self` will be modified in-place and should be writable.
<!-- impl Structure::fn fixate_field -->
Fixates a `Structure` by changing the given field with its fixated value.
## `field_name`
a field in `self`

# Returns

`true` if the structure field could be fixated
<!-- impl Structure::fn fixate_field_boolean -->
Fixates a `Structure` by changing the given `field_name` field to the given
`target` boolean if that field is not fixed yet.
## `field_name`
a field in `self`
## `target`
the target value of the fixation

# Returns

`true` if the structure could be fixated
<!-- impl Structure::fn fixate_field_nearest_double -->
Fixates a `Structure` by changing the given field to the nearest
double to `target` that is a subset of the existing field.
## `field_name`
a field in `self`
## `target`
the target value of the fixation

# Returns

`true` if the structure could be fixated
<!-- impl Structure::fn fixate_field_nearest_fraction -->
Fixates a `Structure` by changing the given field to the nearest
fraction to `target_numerator`/`target_denominator` that is a subset
of the existing field.
## `field_name`
a field in `self`
## `target_numerator`
The numerator of the target value of the fixation
## `target_denominator`
The denominator of the target value of the fixation

# Returns

`true` if the structure could be fixated
<!-- impl Structure::fn fixate_field_nearest_int -->
Fixates a `Structure` by changing the given field to the nearest
integer to `target` that is a subset of the existing field.
## `field_name`
a field in `self`
## `target`
the target value of the fixation

# Returns

`true` if the structure could be fixated
<!-- impl Structure::fn fixate_field_string -->
Fixates a `Structure` by changing the given `field_name` field to the given
`target` string if that field is not fixed yet.
## `field_name`
a field in `self`
## `target`
the target value of the fixation

# Returns

`true` if the structure could be fixated
<!-- impl Structure::fn foreach -->
Calls the provided function once for each field in the `Structure`. The
function must not modify the fields. Also see `Structure::map_in_place`
and `Structure::filter_and_map_in_place`.
## `func`
a function to call for each field
## `user_data`
private data

# Returns

`true` if the supplied function returns `true` For each of the fields,
`false` otherwise.
<!-- impl Structure::fn free -->
Frees a `Structure` and all its fields and values. The structure must not
have a parent when this function is called.
<!-- impl Structure::fn get -->
Parses the variable arguments and reads fields from `self` accordingly.
Variable arguments should be in the form field name, field type
(as a GType), pointer(s) to a variable(s) to hold the return value(s).
The last variable argument should be `None`.

For refcounted (mini)objects you will receive a new reference which
you must release with a suitable `_unref` when no longer needed. For
strings and boxed types you will receive a copy which you will need to
release with either `g_free` or the suitable function for the boxed type.
## `first_fieldname`
the name of the first field to read

# Returns

`false` if there was a problem reading any of the fields (e.g.
 because the field requested did not exist, or was of a type other
 than the type specified), otherwise `true`.
<!-- impl Structure::fn get_array -->
This is useful in language bindings where unknown `gobject::Value` types are not
supported. This function will convert the `GST_TYPE_ARRAY` and
`GST_TYPE_LIST` into a newly allocated `gobject::ValueArray` and return it through
`array`. Be aware that this is slower then getting the `gobject::Value` directly.
## `fieldname`
the name of a field
## `array`
a pointer to a `gobject::ValueArray`

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain an int, this function
returns `false`.
<!-- impl Structure::fn get_boolean -->
Sets the boolean pointed to by `value` corresponding to the value of the
given field. Caller is responsible for making sure the field exists
and has the correct type.
## `fieldname`
the name of a field
## `value`
a pointer to a `gboolean` to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a boolean, this
function returns `false`.
<!-- impl Structure::fn get_clock_time -->
Sets the clock time pointed to by `value` corresponding to the clock time
of the given field. Caller is responsible for making sure the field exists
and has the correct type.
## `fieldname`
the name of a field
## `value`
a pointer to a `ClockTime` to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a `ClockTime`, this
function returns `false`.
<!-- impl Structure::fn get_date -->
Sets the date pointed to by `value` corresponding to the date of the
given field. Caller is responsible for making sure the field exists
and has the correct type.

On success `value` will point to a newly-allocated copy of the date which
should be freed with `glib::Date::free` when no longer needed (note: this is
inconsistent with e.g. `Structure::get_string` which doesn't return a
copy of the string).
## `fieldname`
the name of a field
## `value`
a pointer to a `glib::Date` to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a data, this function
returns `false`.
<!-- impl Structure::fn get_date_time -->
Sets the datetime pointed to by `value` corresponding to the datetime of the
given field. Caller is responsible for making sure the field exists
and has the correct type.

On success `value` will point to a reference of the datetime which
should be unreffed with `DateTime::unref` when no longer needed
(note: this is inconsistent with e.g. `Structure::get_string`
which doesn't return a copy of the string).
## `fieldname`
the name of a field
## `value`
a pointer to a `DateTime` to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a data, this function
returns `false`.
<!-- impl Structure::fn get_double -->
Sets the double pointed to by `value` corresponding to the value of the
given field. Caller is responsible for making sure the field exists
and has the correct type.
## `fieldname`
the name of a field
## `value`
a pointer to a gdouble to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a double, this
function returns `false`.
<!-- impl Structure::fn get_enum -->
Sets the int pointed to by `value` corresponding to the value of the
given field. Caller is responsible for making sure the field exists,
has the correct type and that the enumtype is correct.
## `fieldname`
the name of a field
## `enumtype`
the enum type of a field
## `value`
a pointer to an int to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain an enum of the given
type, this function returns `false`.
<!-- impl Structure::fn get_field_type -->
Finds the field with the given name, and returns the type of the
value it contains. If the field is not found, G_TYPE_INVALID is
returned.
## `fieldname`
the name of the field

# Returns

the `gobject::Value` of the field
<!-- impl Structure::fn get_flagset -->
Read the GstFlagSet flags and mask out of the structure into the
provided pointers.
## `fieldname`
the name of a field
## `value_flags`
a pointer to a guint for the flags field
## `value_mask`
a pointer to a guint for the mask field

# Returns

`true` if the values could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a GstFlagSet, this
function returns `false`.
<!-- impl Structure::fn get_fraction -->
Sets the integers pointed to by `value_numerator` and `value_denominator`
corresponding to the value of the given field. Caller is responsible
for making sure the field exists and has the correct type.
## `fieldname`
the name of a field
## `value_numerator`
a pointer to an int to set
## `value_denominator`
a pointer to an int to set

# Returns

`true` if the values could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a GstFraction, this
function returns `false`.
<!-- impl Structure::fn get_int -->
Sets the int pointed to by `value` corresponding to the value of the
given field. Caller is responsible for making sure the field exists
and has the correct type.
## `fieldname`
the name of a field
## `value`
a pointer to an int to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain an int, this function
returns `false`.
<!-- impl Structure::fn get_int64 -->
Sets the `gint64` pointed to by `value` corresponding to the value of the
given field. Caller is responsible for making sure the field exists
and has the correct type.
## `fieldname`
the name of a field
## `value`
a pointer to a `gint64` to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a `gint64`, this function
returns `false`.
<!-- impl Structure::fn get_list -->
This is useful in language bindings where unknown `gobject::Value` types are not
supported. This function will convert the `GST_TYPE_ARRAY` and
`GST_TYPE_LIST` into a newly allocated GValueArray and return it through
`array`. Be aware that this is slower then getting the `gobject::Value` directly.
## `fieldname`
the name of a field
## `array`
a pointer to a `gobject::ValueArray`

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain an int, this function
returns `false`.

Since 1.12
<!-- impl Structure::fn get_name -->
Get the name of `self` as a string.

# Returns

the name of the structure.
<!-- impl Structure::fn get_name_id -->
Get the name of `self` as a GQuark.

# Returns

the quark representing the name of the structure.
<!-- impl Structure::fn get_string -->
Finds the field corresponding to `fieldname`, and returns the string
contained in the field's value. Caller is responsible for making
sure the field exists and has the correct type.

The string should not be modified, and remains valid until the next
call to a gst_structure_*() function with the given structure.
## `fieldname`
the name of a field

# Returns

a pointer to the string or `None` when the
field did not exist or did not contain a string.
<!-- impl Structure::fn get_uint -->
Sets the uint pointed to by `value` corresponding to the value of the
given field. Caller is responsible for making sure the field exists
and has the correct type.
## `fieldname`
the name of a field
## `value`
a pointer to a uint to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a uint, this function
returns `false`.
<!-- impl Structure::fn get_uint64 -->
Sets the `guint64` pointed to by `value` corresponding to the value of the
given field. Caller is responsible for making sure the field exists
and has the correct type.
## `fieldname`
the name of a field
## `value`
a pointer to a `guint64` to set

# Returns

`true` if the value could be set correctly. If there was no field
with `fieldname` or the existing field did not contain a `guint64`, this function
returns `false`.
<!-- impl Structure::fn get_valist -->
Parses the variable arguments and reads fields from `self` accordingly.
valist-variant of `Structure::get`. Look at the documentation of
`Structure::get` for more details.
## `first_fieldname`
the name of the first field to read
## `args`
variable arguments

# Returns

`true`, or `false` if there was a problem reading any of the fields
<!-- impl Structure::fn get_value -->
Get the value of the field with name `fieldname`.
## `fieldname`
the name of the field to get

# Returns

the `gobject::Value` corresponding to the field with the given name.
<!-- impl Structure::fn has_field -->
Check if `self` contains a field named `fieldname`.
## `fieldname`
the name of a field

# Returns

`true` if the structure contains a field with the given name
<!-- impl Structure::fn has_field_typed -->
Check if `self` contains a field named `fieldname` and with GType `type_`.
## `fieldname`
the name of a field
## `type_`
the type of a value

# Returns

`true` if the structure contains a field with the given name and type
<!-- impl Structure::fn has_name -->
Checks if the structure has the given name
## `name`
structure name to check for

# Returns

`true` if `name` matches the name of the structure.
<!-- impl Structure::fn id_get -->
Parses the variable arguments and reads fields from `self` accordingly.
Variable arguments should be in the form field id quark, field type
(as a GType), pointer(s) to a variable(s) to hold the return value(s).
The last variable argument should be `None` (technically it should be a
0 quark, but we require `None` so compilers that support it can check for
the `None` terminator and warn if it's not there).

This function is just like `Structure::get` only that it is slightly
more efficient since it saves the string-to-quark lookup in the global
quark hashtable.

For refcounted (mini)objects you will receive a new reference which
you must release with a suitable `_unref` when no longer needed. For
strings and boxed types you will receive a copy which you will need to
release with either `g_free` or the suitable function for the boxed type.
## `first_field_id`
the quark of the first field to read

# Returns

`false` if there was a problem reading any of the fields (e.g.
 because the field requested did not exist, or was of a type other
 than the type specified), otherwise `true`.
<!-- impl Structure::fn id_get_valist -->
Parses the variable arguments and reads fields from `self` accordingly.
valist-variant of `Structure::id_get`. Look at the documentation of
`Structure::id_get` for more details.
## `first_field_id`
the quark of the first field to read
## `args`
variable arguments

# Returns

`true`, or `false` if there was a problem reading any of the fields
<!-- impl Structure::fn id_get_value -->
Get the value of the field with GQuark `field`.
## `field`
the `glib::Quark` of the field to get

# Returns

the `gobject::Value` corresponding to the field with the given name
 identifier.
<!-- impl Structure::fn id_has_field -->
Check if `self` contains a field named `field`.
## `field`
`glib::Quark` of the field name

# Returns

`true` if the structure contains a field with the given name
<!-- impl Structure::fn id_has_field_typed -->
Check if `self` contains a field named `field` and with GType `type_`.
## `field`
`glib::Quark` of the field name
## `type_`
the type of a value

# Returns

`true` if the structure contains a field with the given name and type
<!-- impl Structure::fn id_set -->
Identical to gst_structure_set, except that field names are
passed using the GQuark for the field name. This allows more efficient
setting of the structure if the caller already knows the associated
quark values.
The last variable argument must be `None`.
## `fieldname`
the GQuark for the name of the field to set
<!-- impl Structure::fn id_set_valist -->
va_list form of `Structure::id_set`.
## `fieldname`
the name of the field to set
## `varargs`
variable arguments
<!-- impl Structure::fn id_set_value -->
Sets the field with the given GQuark `field` to `value`. If the field
does not exist, it is created. If the field exists, the previous
value is replaced and freed.
## `field`
a `glib::Quark` representing a field
## `value`
the new value of the field
<!-- impl Structure::fn id_take_value -->
Sets the field with the given GQuark `field` to `value`. If the field
does not exist, it is created. If the field exists, the previous
value is replaced and freed.
## `field`
a `glib::Quark` representing a field
## `value`
the new value of the field
<!-- impl Structure::fn intersect -->
Intersects `self` and `struct2` and returns the intersection.
## `struct2`
a `Structure`

# Returns

Intersection of `self` and `struct2`
<!-- impl Structure::fn is_equal -->
Tests if the two `Structure` are equal.
## `structure2`
a `Structure`.

# Returns

`true` if the two structures have the same name and field.
<!-- impl Structure::fn is_subset -->
Checks if `self` is a subset of `superset`, i.e. has the same
structure name and for all fields that are existing in `superset`,
`self` has a value that is a subset of the value in `superset`.
## `superset`
a potentially greater `Structure`

# Returns

`true` if `self` is a subset of `superset`
<!-- impl Structure::fn map_in_place -->
Calls the provided function once for each field in the `Structure`. In
contrast to `Structure::foreach`, the function may modify but not delete the
fields. The structure must be mutable.
## `func`
a function to call for each field
## `user_data`
private data

# Returns

`true` if the supplied function returns `true` For each of the fields,
`false` otherwise.
<!-- impl Structure::fn n_fields -->
Get the number of fields in the structure.

# Returns

the number of fields in the structure
<!-- impl Structure::fn nth_field_name -->
Get the name of the given field number, counting from 0 onwards.
## `index`
the index to get the name of

# Returns

the name of the given field number
<!-- impl Structure::fn remove_all_fields -->
Removes all fields in a GstStructure.
<!-- impl Structure::fn remove_field -->
Removes the field with the given name. If the field with the given
name does not exist, the structure is unchanged.
## `fieldname`
the name of the field to remove
<!-- impl Structure::fn remove_fields -->
Removes the fields with the given names. If a field does not exist, the
argument is ignored.
## `fieldname`
the name of the field to remove
<!-- impl Structure::fn remove_fields_valist -->
va_list form of `Structure::remove_fields`.
## `fieldname`
the name of the field to remove
## `varargs`
`None`-terminated list of more fieldnames to remove
<!-- impl Structure::fn set -->
Parses the variable arguments and sets fields accordingly. Fields that
weren't already part of the structure are added as needed.
Variable arguments should be in the form field name, field type
(as a GType), value(s). The last variable argument should be `None`.
## `fieldname`
the name of the field to set
<!-- impl Structure::fn set_array -->
This is useful in language bindings where unknown GValue types are not
supported. This function will convert a `array` to `GST_TYPE_ARRAY` and set
the field specified by `fieldname`. Be aware that this is slower then using
`GST_TYPE_ARRAY` in a `gobject::Value` directly.

Since 1.12
## `fieldname`
the name of a field
## `array`
a pointer to a `gobject::ValueArray`
<!-- impl Structure::fn set_list -->
This is useful in language bindings where unknown GValue types are not
supported. This function will convert a `array` to `GST_TYPE_ARRAY` and set
the field specified by `fieldname`. Be aware that this is slower then using
`GST_TYPE_ARRAY` in a `gobject::Value` directly.

Since 1.12
## `fieldname`
the name of a field
## `array`
a pointer to a `gobject::ValueArray`
<!-- impl Structure::fn set_name -->
Sets the name of the structure to the given `name`. The string
provided is copied before being used. It must not be empty, start with a
letter and can be followed by letters, numbers and any of "/-_.:".
## `name`
the new name of the structure
<!-- impl Structure::fn set_parent_refcount -->
Sets the parent_refcount field of `Structure`. This field is used to
determine whether a structure is mutable or not. This function should only be
called by code implementing parent objects of `Structure`, as described in
the MT Refcounting section of the design documents.
## `refcount`
a pointer to the parent's refcount

# Returns

`true` if the parent refcount could be set.
<!-- impl Structure::fn set_valist -->
va_list form of `Structure::set`.
## `fieldname`
the name of the field to set
## `varargs`
variable arguments
<!-- impl Structure::fn set_value -->
Sets the field with the given name `field` to `value`. If the field
does not exist, it is created. If the field exists, the previous
value is replaced and freed.
## `fieldname`
the name of the field to set
## `value`
the new value of the field
<!-- impl Structure::fn take_value -->
Sets the field with the given name `field` to `value`. If the field
does not exist, it is created. If the field exists, the previous
value is replaced and freed. The function will take ownership of `value`.
## `fieldname`
the name of the field to set
## `value`
the new value of the field
<!-- impl Structure::fn to_string -->
Converts `self` to a human-readable string representation.

For debugging purposes its easier to do something like this:

```C
GST_LOG ("structure is %" GST_PTR_FORMAT, structure);
```
This prints the structure in human readable form.

The current implementation of serialization will lead to unexpected results
when there are nested `Caps` / `Structure` deeper than one level.

Free-function: g_free

# Returns

a pointer to string allocated by `g_malloc`.
 `g_free` after usage.
<!-- impl Structure::fn from_string -->
Creates a `Structure` from a string representation.
If end is not `None`, a pointer to the place inside the given string
where parsing ended will be returned.

Free-function: gst_structure_free
## `string`
a string representation of a `Structure`.
## `end`
pointer to store the end of the string in.

# Returns

a new `Structure` or `None`
 when the string could not be parsed. Free with
 `Structure::free` after use.
<!-- enum StructureChangeType -->
The type of a `MessageType::StructureChange`.
<!-- enum StructureChangeType::variant Link -->
Pad linking is starting or done.
<!-- enum StructureChangeType::variant Unlink -->
Pad unlinking is starting or done.
<!-- struct TagList -->
List of tags and values used to describe media metadata.

Strings in structures must be ASCII or UTF-8 encoded. Other encodings are
not allowed. Strings must not be empty or `None`.
<!-- impl TagList::fn new -->
Creates a new taglist and appends the values for the given tags. It expects
tag-value pairs like `TagList::add`, and a `None` terminator after the
last pair. The type of the values is implicit and is documented in the API
reference, but can also be queried at runtime with `gst_tag_get_type`. It
is an error to pass a value of a type not matching the tag type into this
function. The tag list will make copies of any arguments passed
(e.g. strings, buffers).

After creation you might also want to set a `TagScope` on the returned
taglist to signal if the contained tags are global or stream tags. By
default stream scope is assumes. See `TagList::set_scope`.

Free-function: gst_tag_list_unref
## `tag`
tag

# Returns

a new `TagList`. Free with `gst_tag_list_unref`
 when no longer needed.
<!-- impl TagList::fn new_empty -->
Creates a new empty GstTagList.

Free-function: gst_tag_list_unref

# Returns

An empty tag list
<!-- impl TagList::fn new_from_string -->
Deserializes a tag list.
## `str`
a string created with `TagList::to_string`

# Returns

a new `TagList`, or `None` in case of an
error.
<!-- impl TagList::fn new_valist -->
Just like `TagList::new`, only that it takes a va_list argument.
Useful mostly for language bindings.

Free-function: gst_tag_list_unref
## `var_args`
tag / value pairs to set

# Returns

a new `TagList`. Free with `gst_tag_list_unref`
 when no longer needed.
<!-- impl TagList::fn add -->
Sets the values for the given tags using the specified mode.
## `mode`
the mode to use
## `tag`
tag
<!-- impl TagList::fn add_valist -->
Sets the values for the given tags using the specified mode.
## `mode`
the mode to use
## `tag`
tag
## `var_args`
tag / value pairs to set
<!-- impl TagList::fn add_valist_values -->
Sets the GValues for the given tags using the specified mode.
## `mode`
the mode to use
## `tag`
tag
## `var_args`
tag / GValue pairs to set
<!-- impl TagList::fn add_value -->
Sets the GValue for a given tag using the specified mode.
## `mode`
the mode to use
## `tag`
tag
## `value`
GValue for this tag
<!-- impl TagList::fn add_values -->
Sets the GValues for the given tags using the specified mode.
## `mode`
the mode to use
## `tag`
tag
<!-- impl TagList::fn foreach -->
Calls the given function for each tag inside the tag list. Note that if there
is no tag, the function won't be called at all.
## `func`
function to be called for each tag
## `user_data`
user specified data
<!-- impl TagList::fn get_boolean -->
Copies the contents for the given tag into the value, merging multiple values
into one if multiple values are associated with the tag.
## `tag`
tag to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_boolean_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_date -->
Copies the first date for the given tag in the taglist into the variable
pointed to by `value`. Free the date with `glib::Date::free` when it is no longer
needed.

Free-function: g_date_free
## `tag`
tag to read out
## `value`
address of a GDate pointer
 variable to store the result into

# Returns

`true`, if a date was copied, `false` if the tag didn't exist in the
 given list or if it was `None`.
<!-- impl TagList::fn get_date_index -->
Gets the date that is at the given index for the given tag in the given
list and copies it into the variable pointed to by `value`. Free the date
with `glib::Date::free` when it is no longer needed.

Free-function: g_date_free
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list or if it was `None`.
<!-- impl TagList::fn get_date_time -->
Copies the first datetime for the given tag in the taglist into the variable
pointed to by `value`. Unref the date with `DateTime::unref` when
it is no longer needed.

Free-function: gst_date_time_unref
## `tag`
tag to read out
## `value`
address of a `DateTime`
 pointer variable to store the result into

# Returns

`true`, if a datetime was copied, `false` if the tag didn't exist in
 the given list or if it was `None`.
<!-- impl TagList::fn get_date_time_index -->
Gets the datetime that is at the given index for the given tag in the given
list and copies it into the variable pointed to by `value`. Unref the datetime
with `DateTime::unref` when it is no longer needed.

Free-function: gst_date_time_unref
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list or if it was `None`.
<!-- impl TagList::fn get_double -->
Copies the contents for the given tag into the value, merging multiple values
into one if multiple values are associated with the tag.
## `tag`
tag to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_double_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_float -->
Copies the contents for the given tag into the value, merging multiple values
into one if multiple values are associated with the tag.
## `tag`
tag to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_float_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_int -->
Copies the contents for the given tag into the value, merging multiple values
into one if multiple values are associated with the tag.
## `tag`
tag to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_int64_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_int_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_pointer -->
Copies the contents for the given tag into the value, merging multiple values
into one if multiple values are associated with the tag.
## `tag`
tag to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_pointer_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_sample -->
Copies the first sample for the given tag in the taglist into the variable
pointed to by `sample`. Free the sample with `gst_sample_unref` when it is
no longer needed. You can retrieve the buffer from the sample using
`Sample::get_buffer` and the associated caps (if any) with
`Sample::get_caps`.

Free-function: gst_sample_unref
## `tag`
tag to read out
## `sample`
address of a GstSample
 pointer variable to store the result into

# Returns

`true`, if a sample was returned, `false` if the tag didn't exist in
 the given list or if it was `None`.
<!-- impl TagList::fn get_sample_index -->
Gets the sample that is at the given index for the given tag in the given
list and copies it into the variable pointed to by `sample`. Free the sample
with `gst_sample_unref` when it is no longer needed. You can retrieve the
buffer from the sample using `Sample::get_buffer` and the associated
caps (if any) with `Sample::get_caps`.

Free-function: gst_sample_unref
## `tag`
tag to read out
## `index`
number of entry to read out
## `sample`
address of a GstSample
 pointer variable to store the result into

# Returns

`true`, if a sample was copied, `false` if the tag didn't exist in the
 given list or if it was `None`.
<!-- impl TagList::fn get_scope -->
Gets the scope of `self`.

# Returns

The scope of `self`
<!-- impl TagList::fn get_string -->
Copies the contents for the given tag into the value, possibly merging
multiple values into one if multiple values are associated with the tag.

Use gst_tag_list_get_string_index (list, tag, 0, value) if you want
to retrieve the first string associated with this tag unmodified.

The resulting string in `value` will be in UTF-8 encoding and should be
freed by the caller using g_free when no longer needed. The
returned string is also guaranteed to be non-`None` and non-empty.

Free-function: g_free
## `tag`
tag to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_string_index -->
Gets the value that is at the given index for the given tag in the given
list.

The resulting string in `value` will be in UTF-8 encoding and should be
freed by the caller using g_free when no longer needed. The
returned string is also guaranteed to be non-`None` and non-empty.

Free-function: g_free
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_tag_size -->
Checks how many value are stored in this tag list for the given tag.
## `tag`
the tag to query

# Returns

The number of tags stored
<!-- impl TagList::fn get_uint -->
Copies the contents for the given tag into the value, merging multiple values
into one if multiple values are associated with the tag.
## `tag`
tag to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_uint64 -->
Copies the contents for the given tag into the value, merging multiple values
into one if multiple values are associated with the tag.
## `tag`
tag to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_uint64_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_uint_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn get_value_index -->
Gets the value that is at the given index for the given tag in the given
list.
## `tag`
tag to read out
## `index`
number of entry to read out

# Returns

The GValue for the specified
 entry or `None` if the tag wasn't available or the tag
 doesn't have as many entries
<!-- impl TagList::fn insert -->
Inserts the tags of the `from` list into the first list using the given mode.
## `from`
list to merge from
## `mode`
the mode to use
<!-- impl TagList::fn is_empty -->
Checks if the given taglist is empty.

# Returns

`true` if the taglist is empty, otherwise `false`.
<!-- impl TagList::fn is_equal -->
Checks if the two given taglists are equal.
## `list2`
a `TagList`.

# Returns

`true` if the taglists are equal, otherwise `false`
<!-- impl TagList::fn merge -->
Merges the two given lists into a new list. If one of the lists is `None`, a
copy of the other is returned. If both lists are `None`, `None` is returned.

Free-function: gst_tag_list_unref
## `list2`
second list to merge
## `mode`
the mode to use

# Returns

the new list
<!-- impl TagList::fn n_tags -->
Get the number of tags in `self`.

# Returns

The number of tags in `self`.
<!-- impl TagList::fn nth_tag_name -->
Get the name of the tag in `self` at `index`.
## `index`
the index

# Returns

The name of the tag at `index`.
<!-- impl TagList::fn peek_string_index -->
Peeks at the value that is at the given index for the given tag in the given
list.

The resulting string in `value` will be in UTF-8 encoding and doesn't need
to be freed by the caller. The returned string is also guaranteed to
be non-`None` and non-empty.
## `tag`
tag to read out
## `index`
number of entry to read out
## `value`
location for the result

# Returns

`true`, if a value was set, `false` if the tag didn't exist in the
 given list.
<!-- impl TagList::fn remove_tag -->
Removes the given tag from the taglist.
## `tag`
tag to remove
<!-- impl TagList::fn set_scope -->
Sets the scope of `self` to `scope`. By default the scope
of a taglist is stream scope.
## `scope`
new scope for `self`
<!-- impl TagList::fn to_string -->
Serializes a tag list to a string.

# Returns

a newly-allocated string, or `None` in case of
 an error. The string must be freed with `g_free` when no longer
 needed.
<!-- impl TagList::fn copy_value -->
Copies the contents for the given tag into the value,
merging multiple values into one if multiple values are associated
with the tag.
You must `gobject::Value::unset` the value after use.
## `dest`
uninitialized `gobject::Value` to copy into
## `list`
list to get the tag from
## `tag`
tag to read out

# Returns

`true`, if a value was copied, `false` if the tag didn't exist in the
 given list.
<!-- enum TagMergeMode -->
The different tag merging modes are basically replace, overwrite and append,
but they can be seen from two directions. Given two taglists: (A) the tags
already in the element and (B) the ones that are supplied to the element (
e.g. via `TagSetter::merge_tags` / `TagSetter::add_tags` or a
`EventType::Tag`), how are these tags merged?
In the table below this is shown for the cases that a tag exists in the list
(A) or does not exists (!A) and combinations thereof.

<table frame="all" colsep="1" rowsep="1">
 `<title>`merge mode`</title>`
 <tgroup cols='5' align='left'>
 `<thead>`
 `<row>`
 `<entry>`merge mode`</entry>`
 `<entry>`A + B`</entry>`
 `<entry>`A + !B`</entry>`
 `<entry>`!A + B`</entry>`
 `<entry>`!A + !B`</entry>`
 `</row>`
 `</thead>`
 `<tbody>`
 `<row>`
 `<entry>`REPLACE_ALL`</entry>`
 `<entry>`B`</entry>`
 `<entry>`-`</entry>`
 `<entry>`B`</entry>`
 `<entry>`-`</entry>`
 `</row>`
 `<row>`
 `<entry>`REPLACE`</entry>`
 `<entry>`B`</entry>`
 `<entry>`A`</entry>`
 `<entry>`B`</entry>`
 `<entry>`-`</entry>`
 `</row>`
 `<row>`
 `<entry>`APPEND`</entry>`
 `<entry>`A, B`</entry>`
 `<entry>`A`</entry>`
 `<entry>`B`</entry>`
 `<entry>`-`</entry>`
 `</row>`
 `<row>`
 `<entry>`PREPEND`</entry>`
 `<entry>`B, A`</entry>`
 `<entry>`A`</entry>`
 `<entry>`B`</entry>`
 `<entry>`-`</entry>`
 `</row>`
 `<row>`
 `<entry>`KEEP`</entry>`
 `<entry>`A`</entry>`
 `<entry>`A`</entry>`
 `<entry>`B`</entry>`
 `<entry>`-`</entry>`
 `</row>`
 `<row>`
 `<entry>`KEEP_ALL`</entry>`
 `<entry>`A`</entry>`
 `<entry>`A`</entry>`
 `<entry>`-`</entry>`
 `<entry>`-`</entry>`
 `</row>`
 `</tbody>`
 `</tgroup>`
`</table>`
<!-- enum TagMergeMode::variant Undefined -->
undefined merge mode
<!-- enum TagMergeMode::variant ReplaceAll -->
replace all tags (clear list and append)
<!-- enum TagMergeMode::variant Replace -->
replace tags
<!-- enum TagMergeMode::variant Append -->
append tags
<!-- enum TagMergeMode::variant Prepend -->
prepend tags
<!-- enum TagMergeMode::variant Keep -->
keep existing tags
<!-- enum TagMergeMode::variant KeepAll -->
keep all existing tags
<!-- enum TagMergeMode::variant Count -->
the number of merge modes
<!-- struct TagSetter -->
Element interface that allows setting of media metadata.

Elements that support changing a stream's metadata will implement this
interface. Examples of such elements are 'vorbisenc', 'theoraenc' and
'id3v2mux'.

If you just want to retrieve metadata in your application then all you
need to do is watch for tag messages on your pipeline's bus. This
interface is only for setting metadata, not for extracting it. To set tags
from the application, find tagsetter elements and set tags using e.g.
`TagSetter::merge_tags` or `TagSetter::add_tags`. Also consider
setting the `TagMergeMode` that is used for tag events that arrive at the
tagsetter element (default mode is to keep existing tags).
The application should do that before the element goes to `State::Paused`.

Elements implementing the `TagSetter` interface often have to merge
any tags received from upstream and the tags set by the application via
the interface. This can be done like this:


```C
GstTagMergeMode merge_mode;
const GstTagList *application_tags;
const GstTagList *event_tags;
GstTagSetter *tagsetter;
GstTagList *result;

tagsetter = GST_TAG_SETTER (element);

merge_mode = gst_tag_setter_get_tag_merge_mode (tagsetter);
application_tags = gst_tag_setter_get_tag_list (tagsetter);
event_tags = (const GstTagList *) element->event_tags;

GST_LOG_OBJECT (tagsetter, "merging tags, merge mode = %d", merge_mode);
GST_LOG_OBJECT (tagsetter, "event tags: %" GST_PTR_FORMAT, event_tags);
GST_LOG_OBJECT (tagsetter, "set   tags: %" GST_PTR_FORMAT, application_tags);

result = gst_tag_list_merge (application_tags, event_tags, merge_mode);

GST_LOG_OBJECT (tagsetter, "final tags: %" GST_PTR_FORMAT, result);
```

# Implements

[`TagSetterExt`](trait.TagSetterExt.html), [`ElementExt`](trait.ElementExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait TagSetterExt -->
Trait containing all `TagSetter` methods.

# Implementors

[`TagSetter`](struct.TagSetter.html)
<!-- trait TagSetterExt::fn add_tag_valist -->
Adds the given tag / value pairs on the setter using the given merge mode.
The list must be terminated with `None`.
## `mode`
the mode to use
## `tag`
tag to set
## `var_args`
tag / value pairs to set
<!-- trait TagSetterExt::fn add_tag_valist_values -->
Adds the given tag / GValue pairs on the setter using the given merge mode.
The list must be terminated with `None`.
## `mode`
the mode to use
## `tag`
tag to set
## `var_args`
tag / GValue pairs to set
<!-- trait TagSetterExt::fn add_tag_value -->
Adds the given tag / GValue pair on the setter using the given merge mode.
## `mode`
the mode to use
## `tag`
tag to set
## `value`
GValue to set for the tag
<!-- trait TagSetterExt::fn add_tag_values -->
Adds the given tag / GValue pairs on the setter using the given merge mode.
The list must be terminated with `None`.
## `mode`
the mode to use
## `tag`
tag to set
<!-- trait TagSetterExt::fn add_tags -->
Adds the given tag / value pairs on the setter using the given merge mode.
The list must be terminated with `None`.
## `mode`
the mode to use
## `tag`
tag to set
<!-- trait TagSetterExt::fn get_tag_list -->
Returns the current list of tags the setter uses. The list should not be
modified or freed.

This function is not thread-safe.

# Returns

a current snapshot of the
 taglist used in the setter or `None` if none is used.
<!-- trait TagSetterExt::fn get_tag_merge_mode -->
Queries the mode by which tags inside the setter are overwritten by tags
from events

# Returns

the merge mode used inside the element.
<!-- trait TagSetterExt::fn merge_tags -->
Merges the given list into the setter's list using the given mode.
## `list`
a tag list to merge from
## `mode`
the mode to merge with
<!-- trait TagSetterExt::fn reset_tags -->
Reset the internal taglist. Elements should call this from within the
state-change handler.
<!-- trait TagSetterExt::fn set_tag_merge_mode -->
Sets the given merge mode that is used for adding tags from events to tags
specified by this interface. The default is `TagMergeMode::Keep`, which keeps
the tags set with this interface and discards tags from events.
## `mode`
The mode with which tags are added
<!-- enum URIError -->
Different URI-related errors that can occur.
<!-- enum URIError::variant UnsupportedProtocol -->
The protocol is not supported
<!-- enum URIError::variant BadUri -->
There was a problem with the URI
<!-- enum URIError::variant BadState -->
Could not set or change the URI because the
 URI handler was in a state where that is not possible or not permitted
<!-- enum URIError::variant BadReference -->
There was a problem with the entity that
 the URI references
<!-- struct URIHandler -->
The `URIHandler` is an interface that is implemented by Source and Sink
`Element` to unify handling of URI.

An application can use the following functions to quickly get an element
that handles the given URI for reading or writing
(`Element::make_from_uri`).

Source and Sink plugins should implement this interface when possible.

# Implements

[`URIHandlerExt`](trait.URIHandlerExt.html)
<!-- trait URIHandlerExt -->
Trait containing all `URIHandler` methods.

# Implementors

[`URIHandler`](struct.URIHandler.html)
<!-- trait URIHandlerExt::fn get_protocols -->
Gets the list of protocols supported by `self`. This list may not be
modified.

# Returns

the
 supported protocols. Returns `None` if the `self` isn't
 implemented properly, or the `self` doesn't support any
 protocols.
<!-- trait URIHandlerExt::fn get_uri -->
Gets the currently handled URI.

# Returns

the URI currently handled by
 the `self`. Returns `None` if there are no URI currently
 handled. The returned string must be freed with `g_free` when no
 longer needed.
<!-- trait URIHandlerExt::fn get_uri_type -->
Gets the type of the given URI handler

# Returns

the `URIType` of the URI handler.
Returns `URIType::Unknown` if the `self` isn't implemented correctly.
<!-- trait URIHandlerExt::fn set_uri -->
Tries to set the URI of the given handler.
## `uri`
URI to set

# Returns

`true` if the URI was set successfully, else `false`.
<!-- enum URIType -->
The different types of URI direction.
<!-- enum URIType::variant Unknown -->
The URI direction is unknown
<!-- enum URIType::variant Sink -->
The URI is a consumer.
<!-- enum URIType::variant Src -->
The URI is a producer.
