<!-- file * -->
<!-- struct AppSink -->
Appsink is a sink plugin that supports many different methods for making
the application get a handle on the GStreamer data in a pipeline. Unlike
most GStreamer elements, Appsink provides external API functions.

appsink can be used by linking to the gstappsink.h header file to access the
methods or by using the appsink action signals and properties.

The normal way of retrieving samples from appsink is by using the
`AppSinkExt::pull_sample` and `AppSinkExt::pull_preroll` methods.
These methods block until a sample becomes available in the sink or when the
sink is shut down or reaches EOS. There are also timed variants of these
methods, `AppSinkExt::try_pull_sample` and `AppSinkExt::try_pull_preroll`,
which accept a timeout parameter to limit the amount of time to wait.

Appsink will internally use a queue to collect buffers from the streaming
thread. If the application is not pulling samples fast enough, this queue
will consume a lot of memory over time. The "max-buffers" property can be
used to limit the queue size. The "drop" property controls whether the
streaming thread blocks or if older buffers are dropped when the maximum
queue size is reached. Note that blocking the streaming thread can negatively
affect real-time performance and should be avoided.

If a blocking behaviour is not desirable, setting the "emit-signals" property
to `true` will make appsink emit the "new-sample" and "new-preroll" signals
when a sample can be pulled without blocking.

The "caps" property on appsink can be used to control the formats that
appsink can receive. This property can contain non-fixed caps, the format of
the pulled samples can be obtained by getting the sample caps.

If one of the pull-preroll or pull-sample methods return `None`, the appsink
is stopped or in the EOS state. You can check for the EOS state with the
"eos" property or with the `AppSinkExt::is_eos` method.

The eos signal can also be used to be informed when the EOS state is reached
to avoid polling.

# Implements

[`AppSinkExt`](trait.AppSinkExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait AppSinkExt -->
Trait containing all `AppSink` methods.

# Implementors

[`AppSink`](struct.AppSink.html)
<!-- trait AppSinkExt::fn get_buffer_list_support -->
Check if `self` supports buffer lists.

Feature: `v1_12`


# Returns

`true` if `self` supports buffer lists.
<!-- trait AppSinkExt::fn get_caps -->
Get the configured caps on `self`.

# Returns

the `gst::Caps` accepted by the sink. `gst_caps_unref` after usage.
<!-- trait AppSinkExt::fn get_drop -->
Check if `self` will drop old buffers when the maximum amount of queued
buffers is reached.

# Returns

`true` if `self` is dropping old buffers when the queue is
filled.
<!-- trait AppSinkExt::fn get_emit_signals -->
Check if appsink will emit the "new-preroll" and "new-sample" signals.

# Returns

`true` if `self` is emiting the "new-preroll" and "new-sample"
signals.
<!-- trait AppSinkExt::fn get_max_buffers -->
Get the maximum amount of buffers that can be queued in `self`.

# Returns

The maximum amount of buffers that can be queued.
<!-- trait AppSinkExt::fn get_wait_on_eos -->
Check if `self` will wait for all buffers to be consumed when an EOS is
received.

# Returns

`true` if `self` will wait for all buffers to be consumed when an
EOS is received.
<!-- trait AppSinkExt::fn is_eos -->
Check if `self` is EOS, which is when no more samples can be pulled because
an EOS event was received.

This function also returns `true` when the appsink is not in the PAUSED or
PLAYING state.

# Returns

`true` if no more samples can be pulled and the appsink is EOS.
<!-- trait AppSinkExt::fn pull_preroll -->
Get the last preroll sample in `self`. This was the sample that caused the
appsink to preroll in the PAUSED state. This sample can be pulled many times
and remains available to the application even after EOS.

This function is typically used when dealing with a pipeline in the PAUSED
state. Calling this function after doing a seek will give the sample right
after the seek position.

Note that the preroll sample will also be returned as the first sample
when calling `AppSinkExt::pull_sample`.

If an EOS event was received before any buffers, this function returns
`None`. Use gst_app_sink_is_eos () to check for the EOS condition.

This function blocks until a preroll sample or EOS is received or the appsink
element is set to the READY/NULL state.

# Returns

a `gst::Sample` or NULL when the appsink is stopped or EOS.
 Call `gst_sample_unref` after usage.
<!-- trait AppSinkExt::fn pull_sample -->
This function blocks until a sample or EOS becomes available or the appsink
element is set to the READY/NULL state.

This function will only return samples when the appsink is in the PLAYING
state. All rendered buffers will be put in a queue so that the application
can pull samples at its own rate. Note that when the application does not
pull samples fast enough, the queued buffers could consume a lot of memory,
especially when dealing with raw video frames.

If an EOS event was received before any buffers, this function returns
`None`. Use gst_app_sink_is_eos () to check for the EOS condition.

# Returns

a `gst::Sample` or NULL when the appsink is stopped or EOS.
 Call `gst_sample_unref` after usage.
<!-- trait AppSinkExt::fn set_buffer_list_support -->
Instruct `self` to enable or disable buffer list support.

For backwards-compatibility reasons applications need to opt in
to indicate that they will be able to handle buffer lists.

Feature: `v1_12`

## `enable_lists`
enable or disable buffer list support
<!-- trait AppSinkExt::fn set_callbacks -->
Set callbacks which will be executed for each new preroll, new sample and eos.
This is an alternative to using the signals, it has lower overhead and is thus
less expensive, but also less flexible.

If callbacks are installed, no signals will be emitted for performance
reasons.
## `callbacks`
the callbacks
## `user_data`
a user_data argument for the callbacks
## `notify`
a destroy notify function
<!-- trait AppSinkExt::fn set_caps -->
Set the capabilities on the appsink element. This function takes
a copy of the caps structure. After calling this method, the sink will only
accept caps that match `caps`. If `caps` is non-fixed, or incomplete,
you must check the caps on the samples to get the actual used caps.
## `caps`
caps to set
<!-- trait AppSinkExt::fn set_drop -->
Instruct `self` to drop old buffers when the maximum amount of queued
buffers is reached.
## `drop`
the new state
<!-- trait AppSinkExt::fn set_emit_signals -->
Make appsink emit the "new-preroll" and "new-sample" signals. This option is
by default disabled because signal emission is expensive and unneeded when
the application prefers to operate in pull mode.
## `emit`
the new state
<!-- trait AppSinkExt::fn set_max_buffers -->
Set the maximum amount of buffers that can be queued in `self`. After this
amount of buffers are queued in appsink, any more buffers will block upstream
elements until a sample is pulled from `self`.
## `max`
the maximum number of buffers to queue
<!-- trait AppSinkExt::fn set_wait_on_eos -->
Instruct `self` to wait for all buffers to be consumed when an EOS is received.
## `wait`
the new state
<!-- trait AppSinkExt::fn try_pull_preroll -->
Get the last preroll sample in `self`. This was the sample that caused the
appsink to preroll in the PAUSED state. This sample can be pulled many times
and remains available to the application even after EOS.

This function is typically used when dealing with a pipeline in the PAUSED
state. Calling this function after doing a seek will give the sample right
after the seek position.

Note that the preroll sample will also be returned as the first sample
when calling `AppSinkExt::pull_sample`.

If an EOS event was received before any buffers or the timeout expires,
this function returns `None`. Use gst_app_sink_is_eos () to check for the EOS
condition.

This function blocks until a preroll sample or EOS is received, the appsink
element is set to the READY/NULL state, or the timeout expires.

Feature: `v1_10`

## `timeout`
the maximum amount of time to wait for the preroll sample

# Returns

a `gst::Sample` or NULL when the appsink is stopped or EOS or the timeout expires.
 Call `gst_sample_unref` after usage.
<!-- trait AppSinkExt::fn try_pull_sample -->
This function blocks until a sample or EOS becomes available or the appsink
element is set to the READY/NULL state or the timeout expires.

This function will only return samples when the appsink is in the PLAYING
state. All rendered buffers will be put in a queue so that the application
can pull samples at its own rate. Note that when the application does not
pull samples fast enough, the queued buffers could consume a lot of memory,
especially when dealing with raw video frames.

If an EOS event was received before any buffers or the timeout expires,
this function returns `None`. Use gst_app_sink_is_eos () to check for the EOS
condition.

Feature: `v1_10`

## `timeout`
the maximum amount of time to wait for a sample

# Returns

a `gst::Sample` or NULL when the appsink is stopped or EOS or the timeout expires.
Call `gst_sample_unref` after usage.
<!-- struct AppSrc -->
The appsrc element can be used by applications to insert data into a
GStreamer pipeline. Unlike most GStreamer elements, appsrc provides
external API functions.

appsrc can be used by linking with the libgstapp library to access the
methods directly or by using the appsrc action signals.

Before operating appsrc, the caps property must be set to fixed caps
describing the format of the data that will be pushed with appsrc. An
exception to this is when pushing buffers with unknown caps, in which case no
caps should be set. This is typically true of file-like sources that push raw
byte buffers. If you don't want to explicitly set the caps, you can use
gst_app_src_push_sample. This method gets the caps associated with the
sample and sets them on the appsrc replacing any previously set caps (if
different from sample's caps).

The main way of handing data to the appsrc element is by calling the
`AppSrcExt::push_buffer` method or by emitting the push-buffer action signal.
This will put the buffer onto a queue from which appsrc will read from in its
streaming thread. It is important to note that data transport will not happen
from the thread that performed the push-buffer call.

The "max-bytes" property controls how much data can be queued in appsrc
before appsrc considers the queue full. A filled internal queue will always
signal the "enough-data" signal, which signals the application that it should
stop pushing data into appsrc. The "block" property will cause appsrc to
block the push-buffer method until free data becomes available again.

When the internal queue is running out of data, the "need-data" signal is
emitted, which signals the application that it should start pushing more data
into appsrc.

In addition to the "need-data" and "enough-data" signals, appsrc can emit the
"seek-data" signal when the "stream-mode" property is set to "seekable" or
"random-access". The signal argument will contain the new desired position in
the stream expressed in the unit set with the "format" property. After
receiving the seek-data signal, the application should push-buffers from the
new position.

These signals allow the application to operate the appsrc in two different
ways:

The push mode, in which the application repeatedly calls the push-buffer/push-sample
method with a new buffer/sample. Optionally, the queue size in the appsrc
can be controlled with the enough-data and need-data signals by respectively
stopping/starting the push-buffer/push-sample calls. This is a typical
mode of operation for the stream-type "stream" and "seekable". Use this
mode when implementing various network protocols or hardware devices.

The pull mode, in which the need-data signal triggers the next push-buffer call.
This mode is typically used in the "random-access" stream-type. Use this
mode for file access or other randomly accessable sources. In this mode, a
buffer of exactly the amount of bytes given by the need-data signal should be
pushed into appsrc.

In all modes, the size property on appsrc should contain the total stream
size in bytes. Setting this property is mandatory in the random-access mode.
For the stream and seekable modes, setting this property is optional but
recommended.

When the application has finished pushing data into appsrc, it should call
`AppSrcExt::end_of_stream` or emit the end-of-stream action signal. After
this call, no more buffers can be pushed into appsrc until a flushing seek
occurs or the state of the appsrc has gone through READY.

# Implements

[`AppSrcExt`](trait.AppSrcExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait AppSrcExt -->
Trait containing all `AppSrc` methods.

# Implementors

[`AppSrc`](struct.AppSrc.html)
<!-- trait AppSrcExt::fn end_of_stream -->
Indicates to the appsrc element that the last buffer queued in the
element is the last buffer of the stream.

# Returns

`gst::FlowReturn::Ok` when the EOS was successfuly queued.
`gst::FlowReturn::Flushing` when `self` is not PAUSED or PLAYING.
<!-- trait AppSrcExt::fn get_caps -->
Get the configured caps on `self`.

# Returns

the `gst::Caps` produced by the source. `gst_caps_unref` after usage.
<!-- trait AppSrcExt::fn get_current_level_bytes -->
Get the number of currently queued bytes inside `self`.

# Returns

The number of currently queued bytes.
<!-- trait AppSrcExt::fn get_duration -->
Get the duration of the stream in nanoseconds. A value of GST_CLOCK_TIME_NONE means that the duration is
not known.

Feature: `v1_10`


# Returns

the duration of the stream previously set with `AppSrcExt::set_duration`;
<!-- trait AppSrcExt::fn get_emit_signals -->
Check if appsrc will emit the "new-preroll" and "new-buffer" signals.

# Returns

`true` if `self` is emitting the "new-preroll" and "new-buffer"
signals.
<!-- trait AppSrcExt::fn get_latency -->
Retrieve the min and max latencies in `min` and `max` respectively.
## `min`
the min latency
## `max`
the min latency
<!-- trait AppSrcExt::fn get_max_bytes -->
Get the maximum amount of bytes that can be queued in `self`.

# Returns

The maximum amount of bytes that can be queued.
<!-- trait AppSrcExt::fn get_size -->
Get the size of the stream in bytes. A value of -1 means that the size is
not known.

# Returns

the size of the stream previously set with `AppSrcExt::set_size`;
<!-- trait AppSrcExt::fn get_stream_type -->
Get the stream type. Control the stream type of `self`
with `AppSrcExt::set_stream_type`.

# Returns

the stream type.
<!-- trait AppSrcExt::fn push_buffer -->
Adds a buffer to the queue of buffers that the appsrc element will
push to its source pad. This function takes ownership of the buffer.

When the block property is TRUE, this function can block until free
space becomes available in the queue.
## `buffer`
a `gst::Buffer` to push

# Returns

`gst::FlowReturn::Ok` when the buffer was successfuly queued.
`gst::FlowReturn::Flushing` when `self` is not PAUSED or PLAYING.
`gst::FlowReturn::Eos` when EOS occured.
<!-- trait AppSrcExt::fn push_sample -->
Extract a buffer from the provided sample and adds it to the queue of
buffers that the appsrc element will push to its source pad. Any
previous caps that were set on appsrc will be replaced by the caps
associated with the sample if not equal.

When the block property is TRUE, this function can block until free
space becomes available in the queue.
## `sample`
a `gst::Sample` from which buffer and caps may be
extracted

# Returns

`gst::FlowReturn::Ok` when the buffer was successfuly queued.
`gst::FlowReturn::Flushing` when `self` is not PAUSED or PLAYING.
`gst::FlowReturn::Eos` when EOS occured.
<!-- trait AppSrcExt::fn set_callbacks -->
Set callbacks which will be executed when data is needed, enough data has
been collected or when a seek should be performed.
This is an alternative to using the signals, it has lower overhead and is thus
less expensive, but also less flexible.

If callbacks are installed, no signals will be emitted for performance
reasons.
## `callbacks`
the callbacks
## `user_data`
a user_data argument for the callbacks
## `notify`
a destroy notify function
<!-- trait AppSrcExt::fn set_caps -->
Set the capabilities on the appsrc element. This function takes
a copy of the caps structure. After calling this method, the source will
only produce caps that match `caps`. `caps` must be fixed and the caps on the
buffers must match the caps or left NULL.
## `caps`
caps to set
<!-- trait AppSrcExt::fn set_duration -->
Set the duration of the stream in nanoseconds. A value of GST_CLOCK_TIME_NONE means that the duration is
not known.

Feature: `v1_10`

## `duration`
the duration to set
<!-- trait AppSrcExt::fn set_emit_signals -->
Make appsrc emit the "new-preroll" and "new-buffer" signals. This option is
by default disabled because signal emission is expensive and unneeded when
the application prefers to operate in pull mode.
## `emit`
the new state
<!-- trait AppSrcExt::fn set_latency -->
Configure the `min` and `max` latency in `src`. If `min` is set to -1, the
default latency calculations for pseudo-live sources will be used.
## `min`
the min latency
## `max`
the min latency
<!-- trait AppSrcExt::fn set_max_bytes -->
Set the maximum amount of bytes that can be queued in `self`.
After the maximum amount of bytes are queued, `self` will emit the
"enough-data" signal.
## `max`
the maximum number of bytes to queue
<!-- trait AppSrcExt::fn set_size -->
Set the size of the stream in bytes. A value of -1 means that the size is
not known.
## `size`
the size to set
<!-- trait AppSrcExt::fn set_stream_type -->
Set the stream type on `self`. For seekable streams, the "seek" signal must
be connected to.

A stream_type stream
## `type_`
the new state
<!-- enum AppStreamType -->
The stream type.
<!-- enum AppStreamType::variant Stream -->
No seeking is supported in the stream, such as a
live stream.
<!-- enum AppStreamType::variant Seekable -->
The stream is seekable but seeking might not
be very fast, such as data from a webserver.
<!-- enum AppStreamType::variant RandomAccess -->
The stream is seekable and seeking is fast,
such as in a local file.
