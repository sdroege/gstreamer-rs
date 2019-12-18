<!-- file * -->
<!-- struct Adapter -->
This class is for elements that receive buffers in an undesired size.
While for example raw video contains one image per buffer, the same is not
true for a lot of other formats, especially those that come directly from
a file. So if you have undefined buffer sizes and require a specific size,
this object is for you.

An adapter is created with `Adapter::new`. It can be freed again with
`gobject::ObjectExt::unref`.

The theory of operation is like this: All buffers received are put
into the adapter using `Adapter::push` and the data is then read back
in chunks of the desired size using `Adapter::map`/`Adapter::unmap`
and/or `Adapter::copy`. After the data has been processed, it is freed
using `Adapter::unmap`.

Other methods such as `Adapter::take` and `Adapter::take_buffer`
combine `Adapter::map` and `Adapter::unmap` in one method and are
potentially more convenient for some use cases.

For example, a sink pad's chain function that needs to pass data to a library
in 512-byte chunks could be implemented like this:

```C
static GstFlowReturn
sink_pad_chain (GstPad *pad, GstObject *parent, GstBuffer *buffer)
{
  MyElement *this;
  GstAdapter *adapter;
  GstFlowReturn ret = GST_FLOW_OK;

  this = MY_ELEMENT (parent);

  adapter = this->adapter;

  // put buffer into adapter
  gst_adapter_push (adapter, buffer);

  // while we can read out 512 bytes, process them
  while (gst_adapter_available (adapter) >= 512 && ret == GST_FLOW_OK) {
    const guint8 *data = gst_adapter_map (adapter, 512);
    // use flowreturn as an error value
    ret = my_library_foo (data);
    gst_adapter_unmap (adapter);
    gst_adapter_flush (adapter, 512);
  }
  return ret;
}
```

For another example, a simple element inside GStreamer that uses `Adapter`
is the libvisual element.

An element using `Adapter` in its sink pad chain function should ensure that
when the FLUSH_STOP event is received, that any queued data is cleared using
`Adapter::clear`. Data should also be cleared or processed on EOS and
when changing state from `gst::State::Paused` to `gst::State::Ready`.

Also check the GST_BUFFER_FLAG_DISCONT flag on the buffer. Some elements might
need to clear the adapter after a discontinuity.

The adapter will keep track of the timestamps of the buffers
that were pushed. The last seen timestamp before the current position
can be queried with `Adapter::prev_pts`. This function can
optionally return the number of bytes between the start of the buffer that
carried the timestamp and the current adapter position. The distance is
useful when dealing with, for example, raw audio samples because it allows
you to calculate the timestamp of the current adapter position by using the
last seen timestamp and the amount of bytes since. Additionally, the
`Adapter::prev_pts_at_offset` can be used to determine the last
seen timestamp at a particular offset in the adapter.

The adapter will also keep track of the offset of the buffers
(`GST_BUFFER_OFFSET`) that were pushed. The last seen offset before the
current position can be queried with `Adapter::prev_offset`. This function
can optionally return the number of bytes between the start of the buffer
that carried the offset and the current adapter position.

Additionally the adapter also keeps track of the PTS, DTS and buffer offset
at the last discontinuity, which can be retrieved with
`Adapter::pts_at_discont`, `Adapter::dts_at_discont` and
`Adapter::offset_at_discont`. The number of bytes that were consumed
since then can be queried with `Adapter::distance_from_discont`.

A last thing to note is that while `Adapter` is pretty optimized,
merging buffers still might be an operation that requires a `malloc` and
`memcpy` operation, and these operations are not the fastest. Because of
this, some functions like `Adapter::available_fast` are provided to help
speed up such cases should you want to. To avoid repeated memory allocations,
`Adapter::copy` can be used to copy data into a (statically allocated)
user provided buffer.

`Adapter` is not MT safe. All operations on an adapter must be serialized by
the caller. This is not normally a problem, however, as the normal use case
of `Adapter` is inside one pad's chain function, in which case access is
serialized via the pad's STREAM_LOCK.

Note that `Adapter::push` takes ownership of the buffer passed. Use
`gst_buffer_ref` before pushing it into the adapter if you still want to
access the buffer later. The adapter will never modify the data in the
buffer pushed in it.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl Adapter::fn new -->
Creates a new `Adapter`. Free with `gobject::ObjectExt::unref`.

# Returns

a new `Adapter`
<!-- impl Adapter::fn available -->
Gets the maximum amount of bytes available, that is it returns the maximum
value that can be supplied to `Adapter::map` without that function
returning `None`.

# Returns

number of bytes available in `self`
<!-- impl Adapter::fn available_fast -->
Gets the maximum number of bytes that are immediately available without
requiring any expensive operations (like copying the data into a
temporary buffer).

# Returns

number of bytes that are available in `self` without expensive
operations
<!-- impl Adapter::fn clear -->
Removes all buffers from `self`.
<!-- impl Adapter::fn copy -->
Copies `size` bytes of data starting at `offset` out of the buffers
contained in `Adapter` into an array `dest` provided by the caller.

The array `dest` should be large enough to contain `size` bytes.
The user should check that the adapter has (`offset` + `size`) bytes
available before calling this function.
## `dest`

 the memory to copy into
## `offset`
the bytes offset in the adapter to start from
## `size`
the number of bytes to copy
<!-- impl Adapter::fn copy_bytes -->
Similar to gst_adapter_copy, but more suitable for language bindings. `size`
bytes of data starting at `offset` will be copied out of the buffers contained
in `self` and into a new `glib::Bytes` structure which is returned. Depending on
the value of the `size` argument an empty `glib::Bytes` structure may be returned.
## `offset`
the bytes offset in the adapter to start from
## `size`
the number of bytes to copy

# Returns

A new `glib::Bytes` structure containing the copied data.
<!-- impl Adapter::fn dts_at_discont -->
Get the DTS that was on the last buffer with the GST_BUFFER_FLAG_DISCONT
flag, or GST_CLOCK_TIME_NONE.

Feature: `v1_10`


# Returns

The DTS at the last discont or GST_CLOCK_TIME_NONE.
<!-- impl Adapter::fn flush -->
Flushes the first `flush` bytes in the `self`. The caller must ensure that
at least this many bytes are available.

See also: `Adapter::map`, `Adapter::unmap`
## `flush`
the number of bytes to flush
<!-- impl Adapter::fn get_buffer -->
Returns a `gst::Buffer` containing the first `nbytes` of the `self`, but
does not flush them from the adapter. See `Adapter::take_buffer`
for details.

Caller owns a reference to the returned buffer. `gst_buffer_unref` after
usage.

Free-function: gst_buffer_unref
## `nbytes`
the number of bytes to get

# Returns

a `gst::Buffer` containing the first
 `nbytes` of the adapter, or `None` if `nbytes` bytes are not available.
 `gst_buffer_unref` when no longer needed.
<!-- impl Adapter::fn get_buffer_fast -->
Returns a `gst::Buffer` containing the first `nbytes` of the `self`, but
does not flush them from the adapter. See `Adapter::take_buffer_fast`
for details.

Caller owns a reference to the returned buffer. `gst_buffer_unref` after
usage.

Free-function: gst_buffer_unref
## `nbytes`
the number of bytes to get

# Returns

a `gst::Buffer` containing the first
 `nbytes` of the adapter, or `None` if `nbytes` bytes are not available.
 `gst_buffer_unref` when no longer needed.
<!-- impl Adapter::fn get_buffer_list -->
Returns a `gst::BufferList` of buffers containing the first `nbytes` bytes of
the `self` but does not flush them from the adapter. See
`Adapter::take_buffer_list` for details.

Caller owns the returned list. Call `gst_buffer_list_unref` to free
the list after usage.
## `nbytes`
the number of bytes to get

# Returns

a `gst::BufferList` of buffers containing
 the first `nbytes` of the adapter, or `None` if `nbytes` bytes are not
 available
<!-- impl Adapter::fn get_list -->
Returns a `glib::List` of buffers containing the first `nbytes` bytes of the
`self`, but does not flush them from the adapter. See
`Adapter::take_list` for details.

Caller owns returned list and contained buffers. `gst_buffer_unref` each
buffer in the list before freeing the list after usage.
## `nbytes`
the number of bytes to get

# Returns

a `glib::List` of
 buffers containing the first `nbytes` of the adapter, or `None` if `nbytes`
 bytes are not available
<!-- impl Adapter::fn map -->
Gets the first `size` bytes stored in the `self`. The returned pointer is
valid until the next function is called on the adapter.

Note that setting the returned pointer as the data of a `gst::Buffer` is
incorrect for general-purpose plugins. The reason is that if a downstream
element stores the buffer so that it has access to it outside of the bounds
of its chain function, the buffer will have an invalid data pointer after
your element flushes the bytes. In that case you should use
`Adapter::take`, which returns a freshly-allocated buffer that you can set
as `gst::Buffer` memory or the potentially more performant
`Adapter::take_buffer`.

Returns `None` if `size` bytes are not available.
## `size`
the number of bytes to map/peek

# Returns


 a pointer to the first `size` bytes of data, or `None`
<!-- impl Adapter::fn masked_scan_uint32 -->
Scan for pattern `pattern` with applied mask `mask` in the adapter data,
starting from offset `offset`.

The bytes in `pattern` and `mask` are interpreted left-to-right, regardless
of endianness. All four bytes of the pattern must be present in the
adapter for it to match, even if the first or last bytes are masked out.

It is an error to call this function without making sure that there is
enough data (offset+size bytes) in the adapter.

This function calls `Adapter::masked_scan_uint32_peek` passing `None`
for value.
## `mask`
mask to apply to data before matching against `pattern`
## `pattern`
pattern to match (after mask is applied)
## `offset`
offset into the adapter data from which to start scanning, returns
 the last scanned position.
## `size`
number of bytes to scan from offset

# Returns

offset of the first match, or -1 if no match was found.

Example:

```text
// Assume the adapter contains 0x00 0x01 0x02 ... 0xfe 0xff

gst_adapter_masked_scan_uint32 (adapter, 0xffffffff, 0x00010203, 0, 256);
// -> returns 0
gst_adapter_masked_scan_uint32 (adapter, 0xffffffff, 0x00010203, 1, 255);
// -> returns -1
gst_adapter_masked_scan_uint32 (adapter, 0xffffffff, 0x01020304, 1, 255);
// -> returns 1
gst_adapter_masked_scan_uint32 (adapter, 0xffff, 0x0001, 0, 256);
// -> returns -1
gst_adapter_masked_scan_uint32 (adapter, 0xffff, 0x0203, 0, 256);
// -> returns 0
gst_adapter_masked_scan_uint32 (adapter, 0xffff0000, 0x02030000, 0, 256);
// -> returns 2
gst_adapter_masked_scan_uint32 (adapter, 0xffff0000, 0x02030000, 0, 4);
// -> returns -1
```
<!-- impl Adapter::fn masked_scan_uint32_peek -->
Scan for pattern `pattern` with applied mask `mask` in the adapter data,
starting from offset `offset`. If a match is found, the value that matched
is returned through `value`, otherwise `value` is left untouched.

The bytes in `pattern` and `mask` are interpreted left-to-right, regardless
of endianness. All four bytes of the pattern must be present in the
adapter for it to match, even if the first or last bytes are masked out.

It is an error to call this function without making sure that there is
enough data (offset+size bytes) in the adapter.
## `mask`
mask to apply to data before matching against `pattern`
## `pattern`
pattern to match (after mask is applied)
## `offset`
offset into the adapter data from which to start scanning, returns
 the last scanned position.
## `size`
number of bytes to scan from offset
## `value`
pointer to uint32 to return matching data

# Returns

offset of the first match, or -1 if no match was found.
<!-- impl Adapter::fn offset_at_discont -->
Get the offset that was on the last buffer with the GST_BUFFER_FLAG_DISCONT
flag, or GST_BUFFER_OFFSET_NONE.

Feature: `v1_10`


# Returns

The offset at the last discont or GST_BUFFER_OFFSET_NONE.
<!-- impl Adapter::fn prev_dts -->
Get the dts that was before the current byte in the adapter. When
`distance` is given, the amount of bytes between the dts and the current
position is returned.

The dts is reset to GST_CLOCK_TIME_NONE and the distance is set to 0 when
the adapter is first created or when it is cleared. This also means that before
the first byte with a dts is removed from the adapter, the dts
and distance returned are GST_CLOCK_TIME_NONE and 0 respectively.
## `distance`
pointer to location for distance, or `None`

# Returns

The previously seen dts.
<!-- impl Adapter::fn prev_dts_at_offset -->
Get the dts that was before the byte at offset `offset` in the adapter. When
`distance` is given, the amount of bytes between the dts and the current
position is returned.

The dts is reset to GST_CLOCK_TIME_NONE and the distance is set to 0 when
the adapter is first created or when it is cleared. This also means that before
the first byte with a dts is removed from the adapter, the dts
and distance returned are GST_CLOCK_TIME_NONE and 0 respectively.
## `offset`
the offset in the adapter at which to get timestamp
## `distance`
pointer to location for distance, or `None`

# Returns

The previously seen dts at given offset.
<!-- impl Adapter::fn prev_offset -->
Get the offset that was before the current byte in the adapter. When
`distance` is given, the amount of bytes between the offset and the current
position is returned.

The offset is reset to GST_BUFFER_OFFSET_NONE and the distance is set to 0
when the adapter is first created or when it is cleared. This also means that
before the first byte with an offset is removed from the adapter, the offset
and distance returned are GST_BUFFER_OFFSET_NONE and 0 respectively.

Feature: `v1_10`

## `distance`
pointer to a location for distance, or `None`

# Returns

The previous seen offset.
<!-- impl Adapter::fn prev_pts -->
Get the pts that was before the current byte in the adapter. When
`distance` is given, the amount of bytes between the pts and the current
position is returned.

The pts is reset to GST_CLOCK_TIME_NONE and the distance is set to 0 when
the adapter is first created or when it is cleared. This also means that before
the first byte with a pts is removed from the adapter, the pts
and distance returned are GST_CLOCK_TIME_NONE and 0 respectively.
## `distance`
pointer to location for distance, or `None`

# Returns

The previously seen pts.
<!-- impl Adapter::fn prev_pts_at_offset -->
Get the pts that was before the byte at offset `offset` in the adapter. When
`distance` is given, the amount of bytes between the pts and the current
position is returned.

The pts is reset to GST_CLOCK_TIME_NONE and the distance is set to 0 when
the adapter is first created or when it is cleared. This also means that before
the first byte with a pts is removed from the adapter, the pts
and distance returned are GST_CLOCK_TIME_NONE and 0 respectively.
## `offset`
the offset in the adapter at which to get timestamp
## `distance`
pointer to location for distance, or `None`

# Returns

The previously seen pts at given offset.
<!-- impl Adapter::fn pts_at_discont -->
Get the PTS that was on the last buffer with the GST_BUFFER_FLAG_DISCONT
flag, or GST_CLOCK_TIME_NONE.

Feature: `v1_10`


# Returns

The PTS at the last discont or GST_CLOCK_TIME_NONE.
<!-- impl Adapter::fn push -->
Adds the data from `buf` to the data stored inside `self` and takes
ownership of the buffer.
## `buf`
a `gst::Buffer` to add to queue in the adapter
<!-- impl Adapter::fn take -->
Returns a freshly allocated buffer containing the first `nbytes` bytes of the
`self`. The returned bytes will be flushed from the adapter.

Caller owns returned value. g_free after usage.

Free-function: g_free
## `nbytes`
the number of bytes to take

# Returns


 oven-fresh hot data, or `None` if `nbytes` bytes are not available
<!-- impl Adapter::fn take_buffer -->
Returns a `gst::Buffer` containing the first `nbytes` bytes of the
`self`. The returned bytes will be flushed from the adapter.
This function is potentially more performant than
`Adapter::take` since it can reuse the memory in pushed buffers
by subbuffering or merging. This function will always return a
buffer with a single memory region.

Note that no assumptions should be made as to whether certain buffer
flags such as the DISCONT flag are set on the returned buffer, or not.
The caller needs to explicitly set or unset flags that should be set or
unset.

Since 1.6 this will also copy over all GstMeta of the input buffers except
for meta with the `gst::MetaFlags::Pooled` flag or with the "memory" tag.

Caller owns a reference to the returned buffer. `gst_buffer_unref` after
usage.

Free-function: gst_buffer_unref
## `nbytes`
the number of bytes to take

# Returns

a `gst::Buffer` containing the first
 `nbytes` of the adapter, or `None` if `nbytes` bytes are not available.
 `gst_buffer_unref` when no longer needed.
<!-- impl Adapter::fn take_buffer_fast -->
Returns a `gst::Buffer` containing the first `nbytes` of the `self`.
The returned bytes will be flushed from the adapter. This function
is potentially more performant than `Adapter::take_buffer` since
it can reuse the memory in pushed buffers by subbuffering or
merging. Unlike `Adapter::take_buffer`, the returned buffer may
be composed of multiple non-contiguous `gst::Memory` objects, no
copies are made.

Note that no assumptions should be made as to whether certain buffer
flags such as the DISCONT flag are set on the returned buffer, or not.
The caller needs to explicitly set or unset flags that should be set or
unset.

This will also copy over all GstMeta of the input buffers except
for meta with the `gst::MetaFlags::Pooled` flag or with the "memory" tag.

This function can return buffer up to the return value of
`Adapter::available` without making copies if possible.

Caller owns a reference to the returned buffer. `gst_buffer_unref` after
usage.

Free-function: gst_buffer_unref
## `nbytes`
the number of bytes to take

# Returns

a `gst::Buffer` containing the first
 `nbytes` of the adapter, or `None` if `nbytes` bytes are not available.
 `gst_buffer_unref` when no longer needed.
<!-- impl Adapter::fn take_buffer_list -->
Returns a `gst::BufferList` of buffers containing the first `nbytes` bytes of
the `self`. The returned bytes will be flushed from the adapter.
When the caller can deal with individual buffers, this function is more
performant because no memory should be copied.

Caller owns the returned list. Call `gst_buffer_list_unref` to free
the list after usage.
## `nbytes`
the number of bytes to take

# Returns

a `gst::BufferList` of buffers containing
 the first `nbytes` of the adapter, or `None` if `nbytes` bytes are not
 available
<!-- impl Adapter::fn take_list -->
Returns a `glib::List` of buffers containing the first `nbytes` bytes of the
`self`. The returned bytes will be flushed from the adapter.
When the caller can deal with individual buffers, this function is more
performant because no memory should be copied.

Caller owns returned list and contained buffers. `gst_buffer_unref` each
buffer in the list before freeing the list after usage.
## `nbytes`
the number of bytes to take

# Returns

a `glib::List` of
 buffers containing the first `nbytes` of the adapter, or `None` if `nbytes`
 bytes are not available
<!-- impl Adapter::fn unmap -->
Releases the memory obtained with the last `Adapter::map`.
<!-- struct Aggregator -->
Manages a set of pads with the purpose of aggregating their buffers.
Control is given to the subclass when all pads have data.

 * Base class for mixers and muxers. Subclasses should at least implement
 the `AggregatorClass.aggregate`() virtual method.

 * Installs a `GstPadChainFunction`, a `GstPadEventFullFunction` and a
 `GstPadQueryFunction` to queue all serialized data packets per sink pad.
 Subclasses should not overwrite those, but instead implement
 `AggregatorClass.sink_event`() and `AggregatorClass.sink_query`() as
 needed.

 * When data is queued on all pads, the aggregate vmethod is called.

 * One can peek at the data on any given GstAggregatorPad with the
 gst_aggregator_pad_peek_buffer () method, and remove it from the pad
 with the gst_aggregator_pad_pop_buffer () method. When a buffer
 has been taken with pop_buffer (), a new buffer can be queued
 on that pad.

 * If the subclass wishes to push a buffer downstream in its aggregate
 implementation, it should do so through the
 gst_aggregator_finish_buffer () method. This method will take care
 of sending and ordering mandatory events such as stream start, caps
 and segment.

 * Same goes for EOS events, which should not be pushed directly by the
 subclass, it should instead return GST_FLOW_EOS in its aggregate
 implementation.

 * Note that the aggregator logic regarding gap event handling is to turn
 these into gap buffers with matching PTS and duration. It will also
 flag these buffers with GST_BUFFER_FLAG_GAP and GST_BUFFER_FLAG_DROPPABLE
 to ease their identification and subsequent processing.

 * Subclasses must use (a subclass of) `AggregatorPad` for both their
 sink and source pads.
 See `gst::ElementClass::add_static_pad_template_with_gtype`.

This class used to live in gst-plugins-bad and was moved to core.

Feature: `v1_14`

# Implements

[`AggregatorExt`](trait.AggregatorExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AggregatorExt -->
Trait containing all `Aggregator` methods.

Feature: `v1_14`

# Implementors

[`Aggregator`](struct.Aggregator.html)
<!-- trait AggregatorExt::fn finish_buffer -->
This method will push the provided output buffer downstream. If needed,
mandatory events such as stream-start, caps, and segment events will be
sent before pushing the buffer.

Feature: `v1_14`

## `buffer`
the `gst::Buffer` to push.
<!-- trait AggregatorExt::fn get_allocator -->
Lets `Aggregator` sub-classes get the memory `allocator`
acquired by the base class and its `params`.

Unref the `allocator` after use it.

Feature: `v1_14`

## `allocator`
the `gst::Allocator`
used
## `params`
the
`gst::AllocationParams` of `allocator`
<!-- trait AggregatorExt::fn get_buffer_pool -->

Feature: `v1_14`


# Returns

the instance of the `gst::BufferPool` used
by `trans`; free it after use it
<!-- trait AggregatorExt::fn get_latency -->
Retrieves the latency values reported by `self` in response to the latency
query, or `GST_CLOCK_TIME_NONE` if there is not live source connected and the element
will not wait for the clock.

Typically only called by subclasses.

Feature: `v1_14`


# Returns

The latency or `GST_CLOCK_TIME_NONE` if the element does not sync
<!-- trait AggregatorExt::fn set_latency -->
Lets `Aggregator` sub-classes tell the baseclass what their internal
latency is. Will also post a LATENCY message on the bus so the pipeline
can reconfigure its global latency.

Feature: `v1_14`

## `min_latency`
minimum latency
## `max_latency`
maximum latency
<!-- trait AggregatorExt::fn set_src_caps -->
Sets the caps to be used on the src pad.

Feature: `v1_14`

## `caps`
The `gst::Caps` to set on the src pad.
<!-- trait AggregatorExt::fn simple_get_next_time -->
This is a simple `Aggregator::get_next_time` implementation that
just looks at the `gst::Segment` on the srcpad of the aggregator and bases
the next time on the running time there.

This is the desired behaviour in most cases where you have a live source
and you have a dead line based aggregator subclass.

Feature: `v1_16`


# Returns

The running time based on the position
<!-- trait AggregatorExt::fn get_property_min_upstream_latency -->
Force minimum upstream latency (in nanoseconds). When sources with a
higher latency are expected to be plugged in dynamically after the
aggregator has started playing, this allows overriding the minimum
latency reported by the initial source(s). This is only taken into
account when larger than the actually reported minimum latency.

Feature: `v1_16`

<!-- trait AggregatorExt::fn set_property_min_upstream_latency -->
Force minimum upstream latency (in nanoseconds). When sources with a
higher latency are expected to be plugged in dynamically after the
aggregator has started playing, this allows overriding the minimum
latency reported by the initial source(s). This is only taken into
account when larger than the actually reported minimum latency.

Feature: `v1_16`

<!-- struct AggregatorPad -->
Pads managed by a `GstAggregor` subclass.

This class used to live in gst-plugins-bad and was moved to core.

Feature: `v1_14`

# Implements

[`AggregatorPadExt`](trait.AggregatorPadExt.html), [`gst::PadExt`](../gst/trait.PadExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AggregatorPadExt -->
Trait containing all `AggregatorPad` methods.

Feature: `v1_14`

# Implementors

[`AggregatorPad`](struct.AggregatorPad.html)
<!-- trait AggregatorPadExt::fn drop_buffer -->
Drop the buffer currently queued in `self`.

Feature: `v1_14`


# Returns

TRUE if there was a buffer queued in `self`, or FALSE if not.
<!-- trait AggregatorPadExt::fn has_buffer -->
This checks if a pad has a buffer available that will be returned by
a call to `AggregatorPadExt::peek_buffer` or
`AggregatorPadExt::pop_buffer`.

Feature: `v1_14_1`


# Returns

`true` if the pad has a buffer available as the next thing.
<!-- trait AggregatorPadExt::fn is_eos -->

Feature: `v1_14`


# Returns

`true` if the pad is EOS, otherwise `false`.
<!-- trait AggregatorPadExt::fn peek_buffer -->

Feature: `v1_14`


# Returns

A reference to the buffer in `self` or
NULL if no buffer was queued. You should unref the buffer after
usage.
<!-- trait AggregatorPadExt::fn pop_buffer -->
Steal the ref to the buffer currently queued in `self`.

Feature: `v1_14`


# Returns

The buffer in `self` or NULL if no buffer was
 queued. You should unref the buffer after usage.
<!-- trait AggregatorPadExt::fn get_property_emit_signals -->
Enables the emission of signals such as `AggregatorPad::buffer-consumed`

Feature: `v1_16`

<!-- trait AggregatorPadExt::fn set_property_emit_signals -->
Enables the emission of signals such as `AggregatorPad::buffer-consumed`

Feature: `v1_16`

<!-- struct BaseParse -->
This base class is for parser elements that process data and splits it
into separate audio/video/whatever frames.

It provides for:

 * provides one sink pad and one source pad
 * handles state changes
 * can operate in pull mode or push mode
 * handles seeking in both modes
 * handles events (SEGMENT/EOS/FLUSH)
 * handles queries (POSITION/DURATION/SEEKING/FORMAT/CONVERT)
 * handles flushing

The purpose of this base class is to provide the basic functionality of
a parser and share a lot of rather complex code.

# Description of the parsing mechanism:

## Set-up phase

 * `BaseParse` calls `BaseParseClass.start`() to inform subclass
 that data processing is about to start now.

 * `BaseParse` class calls `BaseParseClass.set_sink_caps`() to
 inform the subclass about incoming sinkpad caps. Subclass could
 already set the srcpad caps accordingly, but this might be delayed
 until calling `BaseParse::finish_frame` with a non-queued frame.

 * At least at this point subclass needs to tell the `BaseParse` class
 how big data chunks it wants to receive (minimum frame size ). It can
 do this with `BaseParseExt::set_min_frame_size`.

 * `BaseParse` class sets up appropriate data passing mode (pull/push)
 and starts to process the data.

## Parsing phase

 * `BaseParse` gathers at least min_frame_size bytes of data either
 by pulling it from upstream or collecting buffers in an internal
 `Adapter`.

 * A buffer of (at least) min_frame_size bytes is passed to subclass
 with `BaseParseClass.handle_frame`(). Subclass checks the contents
 and can optionally return `gst::FlowReturn::Ok` along with an amount of data
 to be skipped to find a valid frame (which will result in a
 subsequent DISCONT). If, otherwise, the buffer does not hold a
 complete frame, `BaseParseClass.handle_frame`() can merely return
 and will be called again when additional data is available. In push
 mode this amounts to an additional input buffer (thus minimal
 additional latency), in pull mode this amounts to some arbitrary
 reasonable buffer size increase.

 Of course, `BaseParseExt::set_min_frame_size` could also be used if
 a very specific known amount of additional data is required. If,
 however, the buffer holds a complete valid frame, it can pass the
 size of this frame to `BaseParse::finish_frame`.

 If acting as a converter, it can also merely indicate consumed input
 data while simultaneously providing custom output data. Note that
 baseclass performs some processing (such as tracking overall consumed
 data rate versus duration) for each finished frame, but other state
 is only updated upon each call to `BaseParseClass.handle_frame`()
 (such as tracking upstream input timestamp).

 Subclass is also responsible for setting the buffer metadata
 (e.g. buffer timestamp and duration, or keyframe if applicable).
 (although the latter can also be done by `BaseParse` if it is
 appropriately configured, see below). Frame is provided with
 timestamp derived from upstream (as much as generally possible),
 duration obtained from configuration (see below), and offset
 if meaningful (in pull mode).

 Note that `BaseParseClass.handle_frame`() might receive any small
 amount of input data when leftover data is being drained (e.g. at
 EOS).

 * As part of finish frame processing, just prior to actually pushing
 the buffer in question, it is passed to
 `BaseParseClass.pre_push_frame`() which gives subclass yet one last
 chance to examine buffer metadata, or to send some custom (tag)
 events, or to perform custom (segment) filtering.

 * During the parsing process `BaseParseClass` will handle both srcpad
 and sinkpad events. They will be passed to subclass if
 `BaseParseClass.event`() or `BaseParseClass.src_event`()
 implementations have been provided.

## Shutdown phase

* `BaseParse` class calls `BaseParseClass.stop`() to inform the
 subclass that data parsing will be stopped.

Subclass is responsible for providing pad template caps for source and
sink pads. The pads need to be named "sink" and "src". It also needs to
set the fixed caps on srcpad, when the format is ensured (e.g. when
base class calls subclass' `BaseParseClass.set_sink_caps`() function).

This base class uses `gst::Format::Default` as a meaning of frames. So,
subclass conversion routine needs to know that conversion from
`gst::Format::Time` to `gst::Format::Default` must return the
frame number that can be found from the given byte position.

`BaseParse` uses subclasses conversion methods also for seeking (or
otherwise uses its own default one, see also below).

Subclass `start` and `stop` functions will be called to inform the beginning
and end of data processing.

Things that subclass need to take care of:

* Provide pad templates
* Fixate the source pad caps when appropriate
* Inform base class how big data chunks should be retrieved. This is
 done with `BaseParseExt::set_min_frame_size` function.
* Examine data chunks passed to subclass with
 `BaseParseClass.handle_frame`() and pass proper frame(s) to
 `BaseParse::finish_frame`, and setting src pad caps and timestamps
 on frame.
* Provide conversion functions
* Update the duration information with `BaseParse::set_duration`
* Optionally passthrough using `BaseParseExt::set_passthrough`
* Configure various baseparse parameters using
 `BaseParseExt::set_average_bitrate`, `BaseParseExt::set_syncable`
 and `BaseParse::set_frame_rate`.

* In particular, if subclass is unable to determine a duration, but
 parsing (or specs) yields a frames per seconds rate, then this can be
 provided to `BaseParse` to enable it to cater for buffer time
 metadata (which will be taken from upstream as much as
 possible). Internally keeping track of frame durations and respective
 sizes that have been pushed provides `BaseParse` with an estimated
 bitrate. A default `BaseParseClass.convert`() (used if not
 overridden) will then use these rates to perform obvious conversions.
 These rates are also used to update (estimated) duration at regular
 frame intervals.

# Implements

[`BaseParseExt`](trait.BaseParseExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait BaseParseExt -->
Trait containing all `BaseParse` methods.

# Implementors

[`BaseParse`](struct.BaseParse.html)
<!-- trait BaseParseExt::fn add_index_entry -->
Adds an entry to the index associating `offset` to `ts`. It is recommended
to only add keyframe entries. `force` allows to bypass checks, such as
whether the stream is (upstream) seekable, another entry is already "close"
to the new entry, etc.
## `offset`
offset of entry
## `ts`
timestamp associated with offset
## `key`
whether entry refers to keyframe
## `force`
add entry disregarding sanity checks

# Returns

`gboolean` indicating whether entry was added
<!-- trait BaseParseExt::fn convert_default -->
Default implementation of `BaseParseClass.convert`().
## `src_format`
`gst::Format` describing the source format.
## `src_value`
Source value to be converted.
## `dest_format`
`gst::Format` defining the converted format.
## `dest_value`
Pointer where the conversion result will be put.

# Returns

`true` if conversion was successful.
<!-- trait BaseParseExt::fn drain -->
Drains the adapter until it is empty. It decreases the min_frame_size to
match the current adapter size and calls chain method until the adapter
is emptied or chain returns with error.

Feature: `v1_12`

<!-- trait BaseParseExt::fn finish_frame -->
Collects parsed data and pushes this downstream.
Source pad caps must be set when this is called.

If `frame`'s out_buffer is set, that will be used as subsequent frame data.
Otherwise, `size` samples will be taken from the input and used for output,
and the output's metadata (timestamps etc) will be taken as (optionally)
set by the subclass on `frame`'s (input) buffer (which is otherwise
ignored for any but the above purpose/information).

Note that the latter buffer is invalidated by this call, whereas the
caller retains ownership of `frame`.
## `frame`
a `BaseParseFrame`
## `size`
consumed input data represented by frame

# Returns

a `gst::FlowReturn` that should be escalated to caller (of caller)
<!-- trait BaseParseExt::fn merge_tags -->
Sets the parser subclass's tags and how they should be merged with any
upstream stream tags. This will override any tags previously-set
with `BaseParseExt::merge_tags`.

Note that this is provided for convenience, and the subclass is
not required to use this and can still do tag handling on its own.
## `tags`
a `gst::TagList` to merge, or NULL to unset
 previously-set tags
## `mode`
the `gst::TagMergeMode` to use, usually `gst::TagMergeMode::Replace`
<!-- trait BaseParseExt::fn push_frame -->
Pushes the frame's buffer downstream, sends any pending events and
does some timestamp and segment handling. Takes ownership of
frame's buffer, though caller retains ownership of `frame`.

This must be called with sinkpad STREAM_LOCK held.
## `frame`
a `BaseParseFrame`

# Returns

`gst::FlowReturn`
<!-- trait BaseParseExt::fn set_average_bitrate -->
Optionally sets the average bitrate detected in media (if non-zero),
e.g. based on metadata, as it will be posted to the application.

By default, announced average bitrate is estimated. The average bitrate
is used to estimate the total duration of the stream and to estimate
a seek position, if there's no index and the format is syncable
(see `BaseParseExt::set_syncable`).
## `bitrate`
average bitrate in bits/second
<!-- trait BaseParseExt::fn set_duration -->
Sets the duration of the currently playing media. Subclass can use this
when it is able to determine duration and/or notices a change in the media
duration. Alternatively, if `interval` is non-zero (default), then stream
duration is determined based on estimated bitrate, and updated every `interval`
frames.
## `fmt`
`gst::Format`.
## `duration`
duration value.
## `interval`
how often to update the duration estimate based on bitrate, or 0.
<!-- trait BaseParseExt::fn set_frame_rate -->
If frames per second is configured, parser can take care of buffer duration
and timestamping. When performing segment clipping, or seeking to a specific
location, a corresponding decoder might need an initial `lead_in` and a
following `lead_out` number of frames to ensure the desired segment is
entirely filled upon decoding.
## `fps_num`
frames per second (numerator).
## `fps_den`
frames per second (denominator).
## `lead_in`
frames needed before a segment for subsequent decode
## `lead_out`
frames needed after a segment
<!-- trait BaseParseExt::fn set_has_timing_info -->
Set if frames carry timing information which the subclass can (generally)
parse and provide. In particular, intrinsic (rather than estimated) time
can be obtained following a seek.
## `has_timing`
whether frames carry timing information
<!-- trait BaseParseExt::fn set_infer_ts -->
By default, the base class might try to infer PTS from DTS and vice
versa. While this is generally correct for audio data, it may not
be otherwise. Sub-classes implementing such formats should disable
timestamp inferring.
## `infer_ts`
`true` if parser should infer DTS/PTS from each other
<!-- trait BaseParseExt::fn set_latency -->
Sets the minimum and maximum (which may likely be equal) latency introduced
by the parsing process. If there is such a latency, which depends on the
particular parsing of the format, it typically corresponds to 1 frame duration.
## `min_latency`
minimum parse latency
## `max_latency`
maximum parse latency
<!-- trait BaseParseExt::fn set_min_frame_size -->
Subclass can use this function to tell the base class that it needs to
be given buffers of at least `min_size` bytes.
## `min_size`
Minimum size in bytes of the data that this base class should
 give to subclass.
<!-- trait BaseParseExt::fn set_passthrough -->
Set if the nature of the format or configuration does not allow (much)
parsing, and the parser should operate in passthrough mode (which only
applies when operating in push mode). That is, incoming buffers are
pushed through unmodified, i.e. no `BaseParseClass.handle_frame`()
will be invoked, but `BaseParseClass.pre_push_frame`() will still be
invoked, so subclass can perform as much or as little is appropriate for
passthrough semantics in `BaseParseClass.pre_push_frame`().
## `passthrough`
`true` if parser should run in passthrough mode
<!-- trait BaseParseExt::fn set_pts_interpolation -->
By default, the base class will guess PTS timestamps using a simple
interpolation (previous timestamp + duration), which is incorrect for
data streams with reordering, where PTS can go backward. Sub-classes
implementing such formats should disable PTS interpolation.
## `pts_interpolate`
`true` if parser should interpolate PTS timestamps
<!-- trait BaseParseExt::fn set_syncable -->
Set if frame starts can be identified. This is set by default and
determines whether seeking based on bitrate averages
is possible for a format/stream.
## `syncable`
set if frame starts can be identified
<!-- trait BaseParseExt::fn set_ts_at_offset -->
This function should only be called from a `handle_frame` implementation.

`BaseParse` creates initial timestamps for frames by using the last
timestamp seen in the stream before the frame starts. In certain
cases, the correct timestamps will occur in the stream after the
start of the frame, but before the start of the actual picture data.
This function can be used to set the timestamps based on the offset
into the frame data that the picture starts.
## `offset`
offset into current buffer
<!-- trait BaseParseExt::fn get_property_disable_passthrough -->
If set to `true`, baseparse will unconditionally force parsing of the
incoming data. This can be required in the rare cases where the incoming
side-data (caps, pts, dts, ...) is not trusted by the user and wants to
force validation and parsing of the incoming data.
If set to `false`, decision of whether to parse the data or not is up to
the implementation (standard behaviour).
<!-- trait BaseParseExt::fn set_property_disable_passthrough -->
If set to `true`, baseparse will unconditionally force parsing of the
incoming data. This can be required in the rare cases where the incoming
side-data (caps, pts, dts, ...) is not trusted by the user and wants to
force validation and parsing of the incoming data.
If set to `false`, decision of whether to parse the data or not is up to
the implementation (standard behaviour).
<!-- struct BaseParseFrame -->
Frame (context) data passed to each frame parsing virtual methods. In
addition to providing the data to be checked for a valid frame or an already
identified frame, it conveys additional metadata or control information
from and to the subclass w.r.t. the particular frame in question (rather
than global parameters). Some of these may apply to each parsing stage, others
only to some a particular one. These parameters are effectively zeroed at start
of each frame's processing, i.e. parsing virtual method invocation sequence.
<!-- impl BaseParseFrame::fn new -->
Allocates a new `BaseParseFrame`. This function is mainly for bindings,
elements written in C should usually allocate the frame on the stack and
then use `BaseParseFrame::init` to initialise it.
## `buffer`
a `gst::Buffer`
## `flags`
the flags
## `overhead`
number of bytes in this frame which should be counted as
 metadata overhead, ie. not used to calculate the average bitrate.
 Set to -1 to mark the entire frame as metadata. If in doubt, set to 0.

# Returns

a newly-allocated `BaseParseFrame`. Free with
 `BaseParseFrame::free` when no longer needed.
<!-- impl BaseParseFrame::fn copy -->
Copies a `BaseParseFrame`.

# Returns

A copy of `self`
<!-- impl BaseParseFrame::fn free -->
Frees the provided `self`.
<!-- impl BaseParseFrame::fn init -->
Sets a `BaseParseFrame` to initial state. Currently this means
all public fields are zero-ed and a private flag is set to make
sure `BaseParseFrame::free` only frees the contents but not
the actual frame. Use this function to initialise a `BaseParseFrame`
allocated on the stack.
<!-- struct BaseSink -->
`BaseSink` is the base class for sink elements in GStreamer, such as
xvimagesink or filesink. It is a layer on top of `gst::Element` that provides a
simplified interface to plugin writers. `BaseSink` handles many details
for you, for example: preroll, clock synchronization, state changes,
activation in push or pull mode, and queries.

In most cases, when writing sink elements, there is no need to implement
class methods from `gst::Element` or to set functions on pads, because the
`BaseSink` infrastructure should be sufficient.

`BaseSink` provides support for exactly one sink pad, which should be
named "sink". A sink implementation (subclass of `BaseSink`) should
install a pad template in its class_init function, like so:

```C
static void
my_element_class_init (GstMyElementClass *klass)
{
  GstElementClass *gstelement_class = GST_ELEMENT_CLASS (klass);

  // sinktemplate should be a #GstStaticPadTemplate with direction
  // %GST_PAD_SINK and name "sink"
  gst_element_class_add_static_pad_template (gstelement_class, &amp;sinktemplate);

  gst_element_class_set_static_metadata (gstelement_class,
      "Sink name",
      "Sink",
      "My Sink element",
      "The author <my.sink@my.email>");
}
```

`BaseSink` will handle the prerolling correctly. This means that it will
return `gst::StateChangeReturn::Async` from a state change to PAUSED until the first
buffer arrives in this element. The base class will call the
`BaseSinkClass.preroll`() vmethod with this preroll buffer and will then
commit the state change to the next asynchronously pending state.

When the element is set to PLAYING, `BaseSink` will synchronise on the
clock using the times returned from `BaseSinkClass.get_times`(). If this
function returns `GST_CLOCK_TIME_NONE` for the start time, no synchronisation
will be done. Synchronisation can be disabled entirely by setting the object
`BaseSink:sync` property to `false`.

After synchronisation the virtual method `BaseSinkClass.render`() will be
called. Subclasses should minimally implement this method.

Subclasses that synchronise on the clock in the `BaseSinkClass.render`()
method are supported as well. These classes typically receive a buffer in
the render method and can then potentially block on the clock while
rendering. A typical example is an audiosink.
These subclasses can use `BaseSink::wait_preroll` to perform the
blocking wait.

Upon receiving the EOS event in the PLAYING state, `BaseSink` will wait
for the clock to reach the time indicated by the stop time of the last
`BaseSinkClass.get_times`() call before posting an EOS message. When the
element receives EOS in PAUSED, preroll completes, the event is queued and an
EOS message is posted when going to PLAYING.

`BaseSink` will internally use the `gst::EventType::Segment` events to schedule
synchronisation and clipping of buffers. Buffers that fall completely outside
of the current segment are dropped. Buffers that fall partially in the
segment are rendered (and prerolled). Subclasses should do any subbuffer
clipping themselves when needed.

`BaseSink` will by default report the current playback position in
`gst::Format::Time` based on the current clock time and segment information.
If no clock has been set on the element, the query will be forwarded
upstream.

The `BaseSinkClass.set_caps`() function will be called when the subclass
should configure itself to process a specific media type.

The `BaseSinkClass.start`() and `BaseSinkClass.stop`() virtual methods
will be called when resources should be allocated. Any
`BaseSinkClass.preroll`(), `BaseSinkClass.render`() and
`BaseSinkClass.set_caps`() function will be called between the
`BaseSinkClass.start`() and `BaseSinkClass.stop`() calls.

The `BaseSinkClass.event`() virtual method will be called when an event is
received by `BaseSink`. Normally this method should only be overridden by
very specific elements (such as file sinks) which need to handle the
newsegment event specially.

The `BaseSinkClass.unlock`() method is called when the elements should
unblock any blocking operations they perform in the
`BaseSinkClass.render`() method. This is mostly useful when the
`BaseSinkClass.render`() method performs a blocking write on a file
descriptor, for example.

The `BaseSink:max-lateness` property affects how the sink deals with
buffers that arrive too late in the sink. A buffer arrives too late in the
sink when the presentation time (as a combination of the last segment, buffer
timestamp and element base_time) plus the duration is before the current
time of the clock.
If the frame is later than max-lateness, the sink will drop the buffer
without calling the render method.
This feature is disabled if sync is disabled, the
`BaseSinkClass.get_times`() method does not return a valid start time or
max-lateness is set to -1 (the default).
Subclasses can use `BaseSinkExt::set_max_lateness` to configure the
max-lateness value.

The `BaseSink:qos` property will enable the quality-of-service features of
the basesink which gather statistics about the real-time performance of the
clock synchronisation. For each buffer received in the sink, statistics are
gathered and a QOS event is sent upstream with these numbers. This
information can then be used by upstream elements to reduce their processing
rate, for example.

The `BaseSink:async` property can be used to instruct the sink to never
perform an ASYNC state change. This feature is mostly usable when dealing
with non-synchronized streams or sparse streams.

# Implements

[`BaseSinkExt`](trait.BaseSinkExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait BaseSinkExt -->
Trait containing all `BaseSink` methods.

# Implementors

[`BaseSink`](struct.BaseSink.html)
<!-- trait BaseSinkExt::fn do_preroll -->
If the `self` spawns its own thread for pulling buffers from upstream it
should call this method after it has pulled a buffer. If the element needed
to preroll, this function will perform the preroll and will then block
until the element state is changed.

This function should be called with the PREROLL_LOCK held.
## `obj`
the mini object that caused the preroll

# Returns

`gst::FlowReturn::Ok` if the preroll completed and processing can
continue. Any other return value should be returned from the render vmethod.
<!-- trait BaseSinkExt::fn get_blocksize -->
Get the number of bytes that the sink will pull when it is operating in pull
mode.

# Returns

the number of bytes `self` will pull in pull mode.
<!-- trait BaseSinkExt::fn get_drop_out_of_segment -->
Checks if `self` is currently configured to drop buffers which are outside
the current segment

Feature: `v1_12`


# Returns

`true` if the sink is configured to drop buffers outside the
current segment.
<!-- trait BaseSinkExt::fn get_last_sample -->
Get the last sample that arrived in the sink and was used for preroll or for
rendering. This property can be used to generate thumbnails.

The `gst::Caps` on the sample can be used to determine the type of the buffer.

Free-function: gst_sample_unref

# Returns

a `gst::Sample`. `gst_sample_unref` after
 usage. This function returns `None` when no buffer has arrived in the
 sink yet or when the sink is not in PAUSED or PLAYING.
<!-- trait BaseSinkExt::fn get_latency -->
Get the currently configured latency.

# Returns

The configured latency.
<!-- trait BaseSinkExt::fn get_max_bitrate -->
Get the maximum amount of bits per second that the sink will render.

# Returns

the maximum number of bits per second `self` will render.
<!-- trait BaseSinkExt::fn get_max_lateness -->
Gets the max lateness value. See `BaseSinkExt::set_max_lateness` for
more details.

# Returns

The maximum time in nanoseconds that a buffer can be late
before it is dropped and not rendered. A value of -1 means an
unlimited time.
<!-- trait BaseSinkExt::fn get_processing_deadline -->
Get the processing deadline of `self`. see
`BaseSinkExt::set_processing_deadline` for more information about
the processing deadline.

Feature: `v1_16`


# Returns

the processing deadline
<!-- trait BaseSinkExt::fn get_render_delay -->
Get the render delay of `self`. see `BaseSinkExt::set_render_delay` for more
information about the render delay.

# Returns

the render delay of `self`.
<!-- trait BaseSinkExt::fn get_sync -->
Checks if `self` is currently configured to synchronize against the
clock.

# Returns

`true` if the sink is configured to synchronize against the clock.
<!-- trait BaseSinkExt::fn get_throttle_time -->
Get the time that will be inserted between frames to control the
maximum buffers per second.

# Returns

the number of nanoseconds `self` will put between frames.
<!-- trait BaseSinkExt::fn get_ts_offset -->
Get the synchronisation offset of `self`.

# Returns

The synchronisation offset.
<!-- trait BaseSinkExt::fn is_async_enabled -->
Checks if `self` is currently configured to perform asynchronous state
changes to PAUSED.

# Returns

`true` if the sink is configured to perform asynchronous state
changes.
<!-- trait BaseSinkExt::fn is_last_sample_enabled -->
Checks if `self` is currently configured to store the last received sample in
the last-sample property.

# Returns

`true` if the sink is configured to store the last received sample.
<!-- trait BaseSinkExt::fn is_qos_enabled -->
Checks if `self` is currently configured to send Quality-of-Service events
upstream.

# Returns

`true` if the sink is configured to perform Quality-of-Service.
<!-- trait BaseSinkExt::fn query_latency -->
Query the sink for the latency parameters. The latency will be queried from
the upstream elements. `live` will be `true` if `self` is configured to
synchronize against the clock. `upstream_live` will be `true` if an upstream
element is live.

If both `live` and `upstream_live` are `true`, the sink will want to compensate
for the latency introduced by the upstream elements by setting the
`min_latency` to a strictly positive value.

This function is mostly used by subclasses.
## `live`
if the sink is live
## `upstream_live`
if an upstream element is live
## `min_latency`
the min latency of the upstream elements
## `max_latency`
the max latency of the upstream elements

# Returns

`true` if the query succeeded.
<!-- trait BaseSinkExt::fn set_async_enabled -->
Configures `self` to perform all state changes asynchronously. When async is
disabled, the sink will immediately go to PAUSED instead of waiting for a
preroll buffer. This feature is useful if the sink does not synchronize
against the clock or when it is dealing with sparse streams.
## `enabled`
the new async value.
<!-- trait BaseSinkExt::fn set_blocksize -->
Set the number of bytes that the sink will pull when it is operating in pull
mode.
## `blocksize`
the blocksize in bytes
<!-- trait BaseSinkExt::fn set_drop_out_of_segment -->
Configure `self` to drop buffers which are outside the current segment

Feature: `v1_12`

## `drop_out_of_segment`
drop buffers outside the segment
<!-- trait BaseSinkExt::fn set_last_sample_enabled -->
Configures `self` to store the last received sample in the last-sample
property.
## `enabled`
the new enable-last-sample value.
<!-- trait BaseSinkExt::fn set_max_bitrate -->
Set the maximum amount of bits per second that the sink will render.
## `max_bitrate`
the max_bitrate in bits per second
<!-- trait BaseSinkExt::fn set_max_lateness -->
Sets the new max lateness value to `max_lateness`. This value is
used to decide if a buffer should be dropped or not based on the
buffer timestamp and the current clock time. A value of -1 means
an unlimited time.
## `max_lateness`
the new max lateness value.
<!-- trait BaseSinkExt::fn set_processing_deadline -->
Maximum amount of time (in nanoseconds) that the pipeline can take
for processing the buffer. This is added to the latency of live
pipelines.

This function is usually called by subclasses.

Feature: `v1_16`

## `processing_deadline`
the new processing deadline in nanoseconds.
<!-- trait BaseSinkExt::fn set_qos_enabled -->
Configures `self` to send Quality-of-Service events upstream.
## `enabled`
the new qos value.
<!-- trait BaseSinkExt::fn set_render_delay -->
Set the render delay in `self` to `delay`. The render delay is the time
between actual rendering of a buffer and its synchronisation time. Some
devices might delay media rendering which can be compensated for with this
function.

After calling this function, this sink will report additional latency and
other sinks will adjust their latency to delay the rendering of their media.

This function is usually called by subclasses.
## `delay`
the new delay
<!-- trait BaseSinkExt::fn set_sync -->
Configures `self` to synchronize on the clock or not. When
`sync` is `false`, incoming samples will be played as fast as
possible. If `sync` is `true`, the timestamps of the incoming
buffers will be used to schedule the exact render time of its
contents.
## `sync`
the new sync value.
<!-- trait BaseSinkExt::fn set_throttle_time -->
Set the time that will be inserted between rendered buffers. This
can be used to control the maximum buffers per second that the sink
will render.
## `throttle`
the throttle time in nanoseconds
<!-- trait BaseSinkExt::fn set_ts_offset -->
Adjust the synchronisation of `self` with `offset`. A negative value will
render buffers earlier than their timestamp. A positive value will delay
rendering. This function can be used to fix playback of badly timestamped
buffers.
## `offset`
the new offset
<!-- trait BaseSinkExt::fn wait -->
This function will wait for preroll to complete and will then block until `time`
is reached. It is usually called by subclasses that use their own internal
synchronisation but want to let some synchronization (like EOS) be handled
by the base class.

This function should only be called with the PREROLL_LOCK held (like when
receiving an EOS event in the ::event vmethod or when handling buffers in
::render).

The `time` argument should be the running_time of when the timeout should happen
and will be adjusted with any latency and offset configured in the sink.
## `time`
the running_time to be reached
## `jitter`
the jitter to be filled with time diff, or `None`

# Returns

`gst::FlowReturn`
<!-- trait BaseSinkExt::fn wait_clock -->
This function will block until `time` is reached. It is usually called by
subclasses that use their own internal synchronisation.

If `time` is not valid, no synchronisation is done and `gst::ClockReturn::Badtime` is
returned. Likewise, if synchronisation is disabled in the element or there
is no clock, no synchronisation is done and `gst::ClockReturn::Badtime` is returned.

This function should only be called with the PREROLL_LOCK held, like when
receiving an EOS event in the `BaseSinkClass.event`() vmethod or when
receiving a buffer in
the `BaseSinkClass.render`() vmethod.

The `time` argument should be the running_time of when this method should
return and is not adjusted with any latency or offset configured in the
sink.
## `time`
the running_time to be reached
## `jitter`
the jitter to be filled with time diff, or `None`

# Returns

`gst::ClockReturn`
<!-- trait BaseSinkExt::fn wait_preroll -->
If the `BaseSinkClass.render`() method performs its own synchronisation
against the clock it must unblock when going from PLAYING to the PAUSED state
and call this method before continuing to render the remaining data.

If the `BaseSinkClass.render`() method can block on something else than
the clock, it must also be ready to unblock immediately on
the `BaseSinkClass.unlock`() method and cause the
`BaseSinkClass.render`() method to immediately call this function.
In this case, the subclass must be prepared to continue rendering where it
left off if this function returns `gst::FlowReturn::Ok`.

This function will block until a state change to PLAYING happens (in which
case this function returns `gst::FlowReturn::Ok`) or the processing must be stopped due
to a state change to READY or a FLUSH event (in which case this function
returns `gst::FlowReturn::Flushing`).

This function should only be called with the PREROLL_LOCK held, like in the
render function.

# Returns

`gst::FlowReturn::Ok` if the preroll completed and processing can
continue. Any other return value should be returned from the render vmethod.
<!-- trait BaseSinkExt::fn get_property_async -->
If set to `true`, the basesink will perform asynchronous state changes.
When set to `false`, the sink will not signal the parent when it prerolls.
Use this option when dealing with sparse streams or when synchronisation is
not required.
<!-- trait BaseSinkExt::fn set_property_async -->
If set to `true`, the basesink will perform asynchronous state changes.
When set to `false`, the sink will not signal the parent when it prerolls.
Use this option when dealing with sparse streams or when synchronisation is
not required.
<!-- trait BaseSinkExt::fn get_property_blocksize -->
The amount of bytes to pull when operating in pull mode.
<!-- trait BaseSinkExt::fn set_property_blocksize -->
The amount of bytes to pull when operating in pull mode.
<!-- trait BaseSinkExt::fn get_property_enable_last_sample -->
Enable the last-sample property. If `false`, basesink doesn't keep a
reference to the last buffer arrived and the last-sample property is always
set to `None`. This can be useful if you need buffers to be released as soon
as possible, eg. if you're using a buffer pool.
<!-- trait BaseSinkExt::fn set_property_enable_last_sample -->
Enable the last-sample property. If `false`, basesink doesn't keep a
reference to the last buffer arrived and the last-sample property is always
set to `None`. This can be useful if you need buffers to be released as soon
as possible, eg. if you're using a buffer pool.
<!-- trait BaseSinkExt::fn get_property_last_sample -->
The last buffer that arrived in the sink and was used for preroll or for
rendering. This property can be used to generate thumbnails. This property
can be `None` when the sink has not yet received a buffer.
<!-- trait BaseSinkExt::fn get_property_max_bitrate -->
Control the maximum amount of bits that will be rendered per second.
Setting this property to a value bigger than 0 will make the sink delay
rendering of the buffers when it would exceed to max-bitrate.
<!-- trait BaseSinkExt::fn set_property_max_bitrate -->
Control the maximum amount of bits that will be rendered per second.
Setting this property to a value bigger than 0 will make the sink delay
rendering of the buffers when it would exceed to max-bitrate.
<!-- trait BaseSinkExt::fn get_property_processing_deadline -->
Maximum amount of time (in nanoseconds) that the pipeline can take
for processing the buffer. This is added to the latency of live
pipelines.

Feature: `v1_16`

<!-- trait BaseSinkExt::fn set_property_processing_deadline -->
Maximum amount of time (in nanoseconds) that the pipeline can take
for processing the buffer. This is added to the latency of live
pipelines.

Feature: `v1_16`

<!-- trait BaseSinkExt::fn get_property_render_delay -->
The additional delay between synchronisation and actual rendering of the
media. This property will add additional latency to the device in order to
make other sinks compensate for the delay.
<!-- trait BaseSinkExt::fn set_property_render_delay -->
The additional delay between synchronisation and actual rendering of the
media. This property will add additional latency to the device in order to
make other sinks compensate for the delay.
<!-- trait BaseSinkExt::fn get_property_throttle_time -->
The time to insert between buffers. This property can be used to control
the maximum amount of buffers per second to render. Setting this property
to a value bigger than 0 will make the sink create THROTTLE QoS events.
<!-- trait BaseSinkExt::fn set_property_throttle_time -->
The time to insert between buffers. This property can be used to control
the maximum amount of buffers per second to render. Setting this property
to a value bigger than 0 will make the sink create THROTTLE QoS events.
<!-- trait BaseSinkExt::fn get_property_ts_offset -->
Controls the final synchronisation, a negative value will render the buffer
earlier while a positive value delays playback. This property can be
used to fix synchronisation in bad files.
<!-- trait BaseSinkExt::fn set_property_ts_offset -->
Controls the final synchronisation, a negative value will render the buffer
earlier while a positive value delays playback. This property can be
used to fix synchronisation in bad files.
<!-- struct BaseSrc -->
This is a generic base class for source elements. The following
types of sources are supported:

 * random access sources like files
 * seekable sources
 * live sources

The source can be configured to operate in any `gst::Format` with the
`BaseSrcExt::set_format` method. The currently set format determines
the format of the internal `gst::Segment` and any `gst::EventType::Segment`
events. The default format for `BaseSrc` is `gst::Format::Bytes`.

`BaseSrc` always supports push mode scheduling. If the following
conditions are met, it also supports pull mode scheduling:

 * The format is set to `gst::Format::Bytes` (default).
 * `BaseSrcClass.is_seekable`() returns `true`.

If all the conditions are met for operating in pull mode, `BaseSrc` is
automatically seekable in push mode as well. The following conditions must
be met to make the element seekable in push mode when the format is not
`gst::Format::Bytes`:

* `BaseSrcClass.is_seekable`() returns `true`.
* `BaseSrcClass.query`() can convert all supported seek formats to the
 internal format as set with `BaseSrcExt::set_format`.
* `BaseSrcClass.do_seek`() is implemented, performs the seek and returns
 `true`.

When the element does not meet the requirements to operate in pull mode, the
offset and length in the `BaseSrcClass.create`() method should be ignored.
It is recommended to subclass `PushSrc` instead, in this situation. If the
element can operate in pull mode but only with specific offsets and
lengths, it is allowed to generate an error when the wrong values are passed
to the `BaseSrcClass.create`() function.

`BaseSrc` has support for live sources. Live sources are sources that when
paused discard data, such as audio or video capture devices. A typical live
source also produces data at a fixed rate and thus provides a clock to publish
this rate.
Use `BaseSrcExt::set_live` to activate the live source mode.

A live source does not produce data in the PAUSED state. This means that the
`BaseSrcClass.create`() method will not be called in PAUSED but only in
PLAYING. To signal the pipeline that the element will not produce data, the
return value from the READY to PAUSED state will be
`gst::StateChangeReturn::NoPreroll`.

A typical live source will timestamp the buffers it creates with the
current running time of the pipeline. This is one reason why a live source
can only produce data in the PLAYING state, when the clock is actually
distributed and running.

Live sources that synchronize and block on the clock (an audio source, for
example) can use `BaseSrc::wait_playing` when the
`BaseSrcClass.create`() function was interrupted by a state change to
PAUSED.

The `BaseSrcClass.get_times`() method can be used to implement pseudo-live
sources. It only makes sense to implement the `BaseSrcClass.get_times`()
function if the source is a live source. The `BaseSrcClass.get_times`()
function should return timestamps starting from 0, as if it were a non-live
source. The base class will make sure that the timestamps are transformed
into the current running_time. The base source will then wait for the
calculated running_time before pushing out the buffer.

For live sources, the base class will by default report a latency of 0.
For pseudo live sources, the base class will by default measure the difference
between the first buffer timestamp and the start time of get_times and will
report this value as the latency.
Subclasses should override the query function when this behaviour is not
acceptable.

There is only support in `BaseSrc` for exactly one source pad, which
should be named "src". A source implementation (subclass of `BaseSrc`)
should install a pad template in its class_init function, like so:

```C
static void
my_element_class_init (GstMyElementClass *klass)
{
  GstElementClass *gstelement_class = GST_ELEMENT_CLASS (klass);
  // srctemplate should be a #GstStaticPadTemplate with direction
  // %GST_PAD_SRC and name "src"
  gst_element_class_add_static_pad_template (gstelement_class, &amp;srctemplate);

  gst_element_class_set_static_metadata (gstelement_class,
     "Source name",
     "Source",
     "My Source element",
     "The author <my.sink@my.email>");
}
```

## Controlled shutdown of live sources in applications

Applications that record from a live source may want to stop recording
in a controlled way, so that the recording is stopped, but the data
already in the pipeline is processed to the end (remember that many live
sources would go on recording forever otherwise). For that to happen the
application needs to make the source stop recording and send an EOS
event down the pipeline. The application would then wait for an
EOS message posted on the pipeline's bus to know when all data has
been processed and the pipeline can safely be stopped.

An application may send an EOS event to a source element to make it
perform the EOS logic (send EOS event downstream or post a
`gst::MessageType::SegmentDone` on the bus). This can typically be done
with the `gst::ElementExt::send_event` function on the element or its parent bin.

After the EOS has been sent to the element, the application should wait for
an EOS message to be posted on the pipeline's bus. Once this EOS message is
received, it may safely shut down the entire pipeline.

# Implements

[`BaseSrcExt`](trait.BaseSrcExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait BaseSrcExt -->
Trait containing all `BaseSrc` methods.

# Implementors

[`BaseSrc`](struct.BaseSrc.html), [`PushSrc`](struct.PushSrc.html)
<!-- trait BaseSrcExt::fn get_allocator -->
Lets `BaseSrc` sub-classes to know the memory `allocator`
used by the base class and its `params`.

Unref the `allocator` after usage.
## `allocator`
the `gst::Allocator`
used
## `params`
the
`gst::AllocationParams` of `allocator`
<!-- trait BaseSrcExt::fn get_blocksize -->
Get the number of bytes that `self` will push out with each buffer.

# Returns

the number of bytes pushed with each buffer.
<!-- trait BaseSrcExt::fn get_buffer_pool -->

# Returns

the instance of the `gst::BufferPool` used
by the src; unref it after usage.
<!-- trait BaseSrcExt::fn get_do_timestamp -->
Query if `self` timestamps outgoing buffers based on the current running_time.

# Returns

`true` if the base class will automatically timestamp outgoing buffers.
<!-- trait BaseSrcExt::fn is_async -->
Get the current async behaviour of `self`. See also `BaseSrcExt::set_async`.

# Returns

`true` if `self` is operating in async mode.
<!-- trait BaseSrcExt::fn is_live -->
Check if an element is in live mode.

# Returns

`true` if element is in live mode.
<!-- trait BaseSrcExt::fn new_seamless_segment -->
Prepare a new seamless segment for emission downstream. This function must
only be called by derived sub-classes, and only from the `create` function,
as the stream-lock needs to be held.

The format for the new segment will be the current format of the source, as
configured with `BaseSrcExt::set_format`
## `start`
The new start value for the segment
## `stop`
Stop value for the new segment
## `time`
The new time value for the start of the new segment

# Returns

`true` if preparation of the seamless segment succeeded.
<!-- trait BaseSrcExt::fn query_latency -->
Query the source for the latency parameters. `live` will be `true` when `self` is
configured as a live source. `min_latency` and `max_latency` will be set
to the difference between the running time and the timestamp of the first
buffer.

This function is mostly used by subclasses.
## `live`
if the source is live
## `min_latency`
the min latency of the source
## `max_latency`
the max latency of the source

# Returns

`true` if the query succeeded.
<!-- trait BaseSrcExt::fn set_async -->
Configure async behaviour in `self`, no state change will block. The open,
close, start, stop, play and pause virtual methods will be executed in a
different thread and are thus allowed to perform blocking operations. Any
blocking operation should be unblocked with the unlock vmethod.
## `async`
new async mode
<!-- trait BaseSrcExt::fn set_automatic_eos -->
If `automatic_eos` is `true`, `self` will automatically go EOS if a buffer
after the total size is returned. By default this is `true` but sources
that can't return an authoritative size and only know that they're EOS
when trying to read more should set this to `false`.

When `self` operates in `gst::Format::Time`, `BaseSrc` will send an EOS
when a buffer outside of the currently configured segment is pushed if
`automatic_eos` is `true`. Since 1.16, if `automatic_eos` is `false` an
EOS will be pushed only when the `BaseSrc.create` implementation
returns `gst::FlowReturn::Eos`.
## `automatic_eos`
automatic eos
<!-- trait BaseSrcExt::fn set_blocksize -->
Set the number of bytes that `self` will push out with each buffer. When
`blocksize` is set to -1, a default length will be used.
## `blocksize`
the new blocksize in bytes
<!-- trait BaseSrcExt::fn set_caps -->
Set new caps on the basesrc source pad.
## `caps`
a `gst::Caps`

# Returns

`true` if the caps could be set
<!-- trait BaseSrcExt::fn set_do_timestamp -->
Configure `self` to automatically timestamp outgoing buffers based on the
current running_time of the pipeline. This property is mostly useful for live
sources.
## `timestamp`
enable or disable timestamping
<!-- trait BaseSrcExt::fn set_dynamic_size -->
If not `dynamic`, size is only updated when needed, such as when trying to
read past current tracked size. Otherwise, size is checked for upon each
read.
## `dynamic`
new dynamic size mode
<!-- trait BaseSrcExt::fn set_format -->
Sets the default format of the source. This will be the format used
for sending SEGMENT events and for performing seeks.

If a format of GST_FORMAT_BYTES is set, the element will be able to
operate in pull mode if the `BaseSrcClass.is_seekable`() returns `true`.

This function must only be called in states < `gst::State::Paused`.
## `format`
the format to use
<!-- trait BaseSrcExt::fn set_live -->
If the element listens to a live source, `live` should
be set to `true`.

A live source will not produce data in the PAUSED state and
will therefore not be able to participate in the PREROLL phase
of a pipeline. To signal this fact to the application and the
pipeline, the state change return value of the live source will
be GST_STATE_CHANGE_NO_PREROLL.
## `live`
new live-mode
<!-- trait BaseSrcExt::fn start_complete -->
Complete an asynchronous start operation. When the subclass overrides the
start method, it should call `BaseSrc::start_complete` when the start
operation completes either from the same thread or from an asynchronous
helper thread.
## `ret`
a `gst::FlowReturn`
<!-- trait BaseSrcExt::fn start_wait -->
Wait until the start operation completes.

# Returns

a `gst::FlowReturn`.
<!-- trait BaseSrcExt::fn submit_buffer_list -->
Subclasses can call this from their create virtual method implementation
to submit a buffer list to be pushed out later. This is useful in
cases where the create function wants to produce multiple buffers to be
pushed out in one go in form of a `gst::BufferList`, which can reduce overhead
drastically, especially for packetised inputs (for data streams where
the packetisation/chunking is not important it is usually more efficient
to return larger buffers instead).

Subclasses that use this function from their create function must return
`gst::FlowReturn::Ok` and no buffer from their create virtual method implementation.
If a buffer is returned after a buffer list has also been submitted via this
function the behaviour is undefined.

Subclasses must only call this function once per create function call and
subclasses must only call this function when the source operates in push
mode.

Feature: `v1_14`

## `buffer_list`
a `gst::BufferList`
<!-- trait BaseSrcExt::fn wait_playing -->
If the `BaseSrcClass.create`() method performs its own synchronisation
against the clock it must unblock when going from PLAYING to the PAUSED state
and call this method before continuing to produce the remaining data.

This function will block until a state change to PLAYING happens (in which
case this function returns `gst::FlowReturn::Ok`) or the processing must be stopped due
to a state change to READY or a FLUSH event (in which case this function
returns `gst::FlowReturn::Flushing`).

# Returns

`gst::FlowReturn::Ok` if `self` is PLAYING and processing can
continue. Any other return value should be returned from the create vmethod.
<!-- struct BaseTransform -->
This base class is for filter elements that process data. Elements
that are suitable for implementation using `BaseTransform` are ones
where the size and caps of the output is known entirely from the input
caps and buffer sizes. These include elements that directly transform
one buffer into another, modify the contents of a buffer in-place, as
well as elements that collate multiple input buffers into one output buffer,
or that expand one input buffer into multiple output buffers. See below
for more concrete use cases.

It provides for:

* one sinkpad and one srcpad
* Possible formats on sink and source pad implemented
 with custom transform_caps function. By default uses
 same format on sink and source.

* Handles state changes
* Does flushing
* Push mode
* Pull mode if the sub-class transform can operate on arbitrary data

# Use Cases

## Passthrough mode

 * Element has no interest in modifying the buffer. It may want to inspect it,
 in which case the element should have a transform_ip function. If there
 is no transform_ip function in passthrough mode, the buffer is pushed
 intact.

 * The `BaseTransformClass.passthrough_on_same_caps` variable
 will automatically set/unset passthrough based on whether the
 element negotiates the same caps on both pads.

 * `BaseTransformClass.passthrough_on_same_caps` on an element that
 doesn't implement a transform_caps function is useful for elements that
 only inspect data (such as level)

 * Example elements

 * Level
 * Videoscale, audioconvert, videoconvert, audioresample in certain modes.

## Modifications in-place - input buffer and output buffer are the same thing.

* The element must implement a transform_ip function.
* Output buffer size must <= input buffer size
* If the always_in_place flag is set, non-writable buffers will be copied
 and passed to the transform_ip function, otherwise a new buffer will be
 created and the transform function called.

* Incoming writable buffers will be passed to the transform_ip function
 immediately.
* only implementing transform_ip and not transform implies always_in_place = `true`

 * Example elements:
 * Volume
 * Audioconvert in certain modes (signed/unsigned conversion)
 * videoconvert in certain modes (endianness swapping)

## Modifications only to the caps/metadata of a buffer

* The element does not require writable data, but non-writable buffers
 should be subbuffered so that the meta-information can be replaced.

* Elements wishing to operate in this mode should replace the
 prepare_output_buffer method to create subbuffers of the input buffer
 and set always_in_place to `true`

* Example elements
 * Capsfilter when setting caps on outgoing buffers that have
 none.
 * identity when it is going to re-timestamp buffers by
 datarate.

## Normal mode
 * always_in_place flag is not set, or there is no transform_ip function
 * Element will receive an input buffer and output buffer to operate on.
 * Output buffer is allocated by calling the prepare_output_buffer function.
 * Example elements:
 * Videoscale, videoconvert, audioconvert when doing
 scaling/conversions

## Special output buffer allocations
 * Elements which need to do special allocation of their output buffers
 beyond allocating output buffers via the negotiated allocator or
 buffer pool should implement the prepare_output_buffer method.

 * Example elements:
 * efence

# Sub-class settable flags on GstBaseTransform

* passthrough

 * Implies that in the current configuration, the sub-class is not interested in modifying the buffers.
 * Elements which are always in passthrough mode whenever the same caps has been negotiated on both pads can set the class variable passthrough_on_same_caps to have this behaviour automatically.

* always_in_place
 * Determines whether a non-writable buffer will be copied before passing
 to the transform_ip function.

 * Implied `true` if no transform function is implemented.
 * Implied `false` if ONLY transform function is implemented.

# Implements

[`BaseTransformExt`](trait.BaseTransformExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait BaseTransformExt -->
Trait containing all `BaseTransform` methods.

# Implementors

[`BaseTransform`](struct.BaseTransform.html)
<!-- trait BaseTransformExt::fn get_allocator -->
Lets `BaseTransform` sub-classes to know the memory `allocator`
used by the base class and its `params`.

Unref the `allocator` after use it.
## `allocator`
the `gst::Allocator`
used
## `params`
the
`gst::AllocationParams` of `allocator`
<!-- trait BaseTransformExt::fn get_buffer_pool -->

# Returns

the instance of the `gst::BufferPool` used
by `self`; free it after use it
<!-- trait BaseTransformExt::fn is_in_place -->
See if `self` is configured as a in_place transform.

# Returns

`true` is the transform is configured in in_place mode.

MT safe.
<!-- trait BaseTransformExt::fn is_passthrough -->
See if `self` is configured as a passthrough transform.

# Returns

`true` is the transform is configured in passthrough mode.

MT safe.
<!-- trait BaseTransformExt::fn is_qos_enabled -->
Queries if the transform will handle QoS.

# Returns

`true` if QoS is enabled.

MT safe.
<!-- trait BaseTransformExt::fn reconfigure_sink -->
Instructs `self` to request renegotiation upstream. This function is
typically called after properties on the transform were set that
influence the input format.
<!-- trait BaseTransformExt::fn reconfigure_src -->
Instructs `self` to renegotiate a new downstream transform on the next
buffer. This function is typically called after properties on the transform
were set that influence the output format.
<!-- trait BaseTransformExt::fn set_gap_aware -->
If `gap_aware` is `false` (the default), output buffers will have the
`gst::BufferFlags::Gap` flag unset.

If set to `true`, the element must handle output buffers with this flag set
correctly, i.e. it can assume that the buffer contains neutral data but must
unset the flag if the output is no neutral data.

MT safe.
## `gap_aware`
New state
<!-- trait BaseTransformExt::fn set_in_place -->
Determines whether a non-writable buffer will be copied before passing
to the transform_ip function.

 * Always `true` if no transform function is implemented.
 * Always `false` if ONLY transform function is implemented.

MT safe.
## `in_place`
Boolean value indicating that we would like to operate
on in_place buffers.
<!-- trait BaseTransformExt::fn set_passthrough -->
Set passthrough mode for this filter by default. This is mostly
useful for filters that do not care about negotiation.

Always `true` for filters which don't implement either a transform
or transform_ip method.

MT safe.
## `passthrough`
boolean indicating passthrough mode.
<!-- trait BaseTransformExt::fn set_prefer_passthrough -->
If `prefer_passthrough` is `true` (the default), `self` will check and
prefer passthrough caps from the list of caps returned by the
transform_caps vmethod.

If set to `false`, the element must order the caps returned from the
transform_caps function in such a way that the preferred format is
first in the list. This can be interesting for transforms that can do
passthrough transforms but prefer to do something else, like a
capsfilter.

MT safe.
## `prefer_passthrough`
New state
<!-- trait BaseTransformExt::fn set_qos_enabled -->
Enable or disable QoS handling in the transform.

MT safe.
## `enabled`
new state
<!-- trait BaseTransformExt::fn update_qos -->
Set the QoS parameters in the transform. This function is called internally
when a QOS event is received but subclasses can provide custom information
when needed.

MT safe.
## `proportion`
the proportion
## `diff`
the diff against the clock
## `timestamp`
the timestamp of the buffer generating the QoS expressed in
running_time.
<!-- trait BaseTransformExt::fn update_src_caps -->
Updates the srcpad caps and send the caps downstream. This function
can be used by subclasses when they have already negotiated their caps
but found a change in them (or computed new information). This way,
they can notify downstream about that change without losing any
buffer.
## `updated_caps`
An updated version of the srcpad caps to be pushed
downstream

# Returns

`true` if the caps could be send downstream `false` otherwise
<!-- struct FlowCombiner -->
Utility struct to help handling `gst::FlowReturn` combination. Useful for
`gst::Element`<!-- -->s that have multiple source pads and need to combine
the different `gst::FlowReturn` for those pads.

`FlowCombiner` works by using the last `gst::FlowReturn` for all `gst::Pad`
it has in its list and computes the combined return value and provides
it to the caller.

To add a new pad to the `FlowCombiner` use `FlowCombiner::add_pad`.
The new `gst::Pad` is stored with a default value of `gst::FlowReturn::Ok`.

In case you want a `gst::Pad` to be removed, use `FlowCombiner::remove_pad`.

Please be aware that this struct isn't thread safe as its designed to be
 used by demuxers, those usually will have a single thread operating it.

These functions will take refs on the passed `gst::Pad`<!-- -->s.

Aside from reducing the user's code size, the main advantage of using this
helper struct is to follow the standard rules for `gst::FlowReturn` combination.
These rules are:

* `gst::FlowReturn::Eos`: only if all returns are EOS too
* `gst::FlowReturn::NotLinked`: only if all returns are NOT_LINKED too
* `gst::FlowReturn::Error` or below: if at least one returns an error return
* `gst::FlowReturn::NotNegotiated`: if at least one returns a not-negotiated return
* `gst::FlowReturn::Flushing`: if at least one returns flushing
* `gst::FlowReturn::Ok`: otherwise

`gst::FlowReturn::Error` or below, GST_FLOW_NOT_NEGOTIATED and GST_FLOW_FLUSHING are
returned immediately from the `FlowCombiner::update_flow` function.
<!-- impl FlowCombiner::fn new -->
Creates a new `FlowCombiner`, use `FlowCombiner::free` to free it.

# Returns

A new `FlowCombiner`
<!-- impl FlowCombiner::fn add_pad -->
Adds a new `gst::Pad` to the `FlowCombiner`.
## `pad`
the `gst::Pad` that is being added
<!-- impl FlowCombiner::fn clear -->
Removes all pads from a `FlowCombiner` and resets it to its initial state.
<!-- impl FlowCombiner::fn free -->
Frees a `FlowCombiner` struct and all its internal data.
<!-- impl FlowCombiner::fn ref -->
Increments the reference count on the `FlowCombiner`.

Feature: `v1_12_1`


# Returns

the `FlowCombiner`.
<!-- impl FlowCombiner::fn remove_pad -->
Removes a `gst::Pad` from the `FlowCombiner`.
## `pad`
the `gst::Pad` to remove
<!-- impl FlowCombiner::fn reset -->
Reset flow combiner and all pads to their initial state without removing pads.
<!-- impl FlowCombiner::fn unref -->
Decrements the reference count on the `FlowCombiner`.

Feature: `v1_12_1`

<!-- impl FlowCombiner::fn update_flow -->
Computes the combined flow return for the pads in it.

The `gst::FlowReturn` parameter should be the last flow return update for a pad
in this `FlowCombiner`. It will use this value to be able to shortcut some
combinations and avoid looking over all pads again. e.g. The last combined
return is the same as the latest obtained `gst::FlowReturn`.
## `fret`
the latest `gst::FlowReturn` received for a pad in this `FlowCombiner`

# Returns

The combined `gst::FlowReturn`
<!-- impl FlowCombiner::fn update_pad_flow -->
Sets the provided pad's last flow return to provided value and computes
the combined flow return for the pads in it.

The `gst::FlowReturn` parameter should be the last flow return update for a pad
in this `FlowCombiner`. It will use this value to be able to shortcut some
combinations and avoid looking over all pads again. e.g. The last combined
return is the same as the latest obtained `gst::FlowReturn`.
## `pad`
the `gst::Pad` whose `gst::FlowReturn` to update
## `fret`
the latest `gst::FlowReturn` received for a pad in this `FlowCombiner`

# Returns

The combined `gst::FlowReturn`
<!-- struct PushSrc -->
This class is mostly useful for elements that cannot do
random access, or at least very slowly. The source usually
prefers to push out a fixed size buffer.

Subclasses usually operate in a format that is different from the
default GST_FORMAT_BYTES format of `BaseSrc`.

Classes extending this base class will usually be scheduled
in a push based mode. If the peer accepts to operate without
offsets and within the limits of the allowed block size, this
class can operate in getrange based mode automatically. To make
this possible, the subclass should implement and override the
SCHEDULING query.

The subclass should extend the methods from the baseclass in
addition to the ::create method.

Seeking, flushing, scheduling and sync is all handled by this
base class.

# Implements

[`BaseSrcExt`](trait.BaseSrcExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
