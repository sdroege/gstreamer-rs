<!-- file * -->
<!-- struct TestClock -->
GstTestClock is an implementation of `gst::Clock` which has different
behaviour compared to `gst::SystemClock`. Time for `gst::SystemClock` advances
according to the system time, while time for `TestClock` changes only
when `TestClock::set_time` or `TestClock::advance_time` are
called. `TestClock` provides unit tests with the possibility to
precisely advance the time in a deterministic manner, independent of the
system time or any other external factors.

## Advancing the time of a `TestClock`


```C
  #include <gst/gst.h>
  #include <gst/check/gsttestclock.h>

  GstClock *clock;
  GstTestClock *test_clock;

  clock = gst_test_clock_new ();
  test_clock = GST_TEST_CLOCK (clock);
  GST_INFO ("Time: %" GST_TIME_FORMAT, GST_TIME_ARGS (gst_clock_get_time (clock)));
  gst_test_clock_advance_time ( test_clock, 1 * GST_SECOND);
  GST_INFO ("Time: %" GST_TIME_FORMAT, GST_TIME_ARGS (gst_clock_get_time (clock)));
  g_usleep (10 * G_USEC_PER_SEC);
  GST_INFO ("Time: %" GST_TIME_FORMAT, GST_TIME_ARGS (gst_clock_get_time (clock)));
  gst_test_clock_set_time (test_clock, 42 * GST_SECOND);
  GST_INFO ("Time: %" GST_TIME_FORMAT, GST_TIME_ARGS (gst_clock_get_time (clock)));
  ...
```

`gst::Clock` allows for setting up single shot or periodic clock notifications
as well as waiting for these notifications synchronously (using
`gst::Clock::id_wait`) or asynchronously (using `gst::Clock::id_wait_async` or
`gst::Clock::id_wait_async`). This is used by many GStreamer elements,
among them `GstBaseSrc` and `GstBaseSink`.

`TestClock` keeps track of these clock notifications. By calling
`TestClock::wait_for_next_pending_id` or
`TestClock::wait_for_multiple_pending_ids` a unit tests may wait for the
next one or several clock notifications to be requested. Additionally unit
tests may release blocked waits in a controlled fashion by calling
`TestClock::process_next_clock_id`. This way a unit test can control the
inaccuracy (jitter) of clock notifications, since the test can decide to
release blocked waits when the clock time has advanced exactly to, or past,
the requested clock notification time.

There are also interfaces for determining if a notification belongs to a
`TestClock` or not, as well as getting the number of requested clock
notifications so far.

N.B.: When a unit test waits for a certain amount of clock notifications to
be requested in `TestClock::wait_for_next_pending_id` or
`TestClock::wait_for_multiple_pending_ids` then these functions may block
for a long time. If they block forever then the expected clock notifications
were never requested from `TestClock`, and so the assumptions in the code
of the unit test are wrong. The unit test case runner in gstcheck is
expected to catch these cases either by the default test case timeout or the
one set for the unit test by calling tcase_set_timeout\(\).

The sample code below assumes that the element under test will delay a
buffer pushed on the source pad by some latency until it arrives on the sink
pad. Moreover it is assumed that the element will at some point call
`gst::Clock::id_wait` to synchronously wait for a specific time. The first
buffer sent will arrive exactly on time only delayed by the latency. The
second buffer will arrive a little late (7ms) due to simulated jitter in the
clock notification.

## Demonstration of how to work with clock notifications and `TestClock`


```C
  #include <gst/gst.h>
  #include <gst/check/gstcheck.h>
  #include <gst/check/gsttestclock.h>

  GstClockTime latency;
  GstElement *element;
  GstPad *srcpad;
  GstClock *clock;
  GstTestClock *test_clock;
  GstBuffer buf;
  GstClockID pending_id;
  GstClockID processed_id;

  latency = 42 * GST_MSECOND;
  element = create_element (latency, ...);
  srcpad = get_source_pad (element);

  clock = gst_test_clock_new ();
  test_clock = GST_TEST_CLOCK (clock);
  gst_element_set_clock (element, clock);

  GST_INFO ("Set time, create and push the first buffer\n");
  gst_test_clock_set_time (test_clock, 0);
  buf = create_test_buffer (gst_clock_get_time (clock), ...);
  gst_assert_cmpint (gst_pad_push (srcpad, buf), ==, GST_FLOW_OK);

  GST_INFO ("Block until element is waiting for a clock notification\n");
  gst_test_clock_wait_for_next_pending_id (test_clock, &pending_id);
  GST_INFO ("Advance to the requested time of the clock notification\n");
  gst_test_clock_advance_time (test_clock, latency);
  GST_INFO ("Release the next blocking wait and make sure it is the one from element\n");
  processed_id = gst_test_clock_process_next_clock_id (test_clock);
  g_assert (processed_id == pending_id);
  g_assert_cmpint (GST_CLOCK_ENTRY_STATUS (processed_id), ==, GST_CLOCK_OK);
  gst_clock_id_unref (pending_id);
  gst_clock_id_unref (processed_id);

  GST_INFO ("Validate that element produced an output buffer and check its timestamp\n");
  g_assert_cmpint (get_number_of_output_buffer (...), ==, 1);
  buf = get_buffer_pushed_by_element (element, ...);
  g_assert_cmpint (GST_BUFFER_TIMESTAMP (buf), ==, latency);
  gst_buffer_unref (buf);
  GST_INFO ("Check that element does not wait for any clock notification\n");
  g_assert (!gst_test_clock_peek_next_pending_id (test_clock, NULL));

  GST_INFO ("Set time, create and push the second buffer\n");
  gst_test_clock_advance_time (test_clock, 10 * GST_SECOND);
  buf = create_test_buffer (gst_clock_get_time (clock), ...);
  gst_assert_cmpint (gst_pad_push (srcpad, buf), ==, GST_FLOW_OK);

  GST_INFO ("Block until element is waiting for a new clock notification\n");
  (gst_test_clock_wait_for_next_pending_id (test_clock, &pending_id);
  GST_INFO ("Advance past 7ms beyond the requested time of the clock notification\n");
  gst_test_clock_advance_time (test_clock, latency + 7 * GST_MSECOND);
  GST_INFO ("Release the next blocking wait and make sure it is the one from element\n");
  processed_id = gst_test_clock_process_next_clock_id (test_clock);
  g_assert (processed_id == pending_id);
  g_assert_cmpint (GST_CLOCK_ENTRY_STATUS (processed_id), ==, GST_CLOCK_OK);
  gst_clock_id_unref (pending_id);
  gst_clock_id_unref (processed_id);

  GST_INFO ("Validate that element produced an output buffer and check its timestamp\n");
  g_assert_cmpint (get_number_of_output_buffer (...), ==, 1);
  buf = get_buffer_pushed_by_element (element, ...);
  g_assert_cmpint (GST_BUFFER_TIMESTAMP (buf), ==,
      10 * GST_SECOND + latency + 7 * GST_MSECOND);
  gst_buffer_unref (buf);
  GST_INFO ("Check that element does not wait for any clock notification\n");
  g_assert (!gst_test_clock_peek_next_pending_id (test_clock, NULL));
  ...
```

Since `TestClock` is only supposed to be used in unit tests it calls
`g_assert`, `g_assert_cmpint` or `g_assert_cmpuint` to validate all function
arguments. This will highlight any issues with the unit test code itself.

# Implements

[`gst::ClockExt`](../gst/trait.ClockExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl TestClock::fn new -->
Creates a new test clock with its time set to zero.

MT safe.

# Returns

a `TestClock` cast to `gst::Clock`.
<!-- impl TestClock::fn new_with_start_time -->
Creates a new test clock with its time set to the specified time.

MT safe.
## `start_time`
a `gst::ClockTime` set to the desired start time of the clock.

# Returns

a `TestClock` cast to `gst::Clock`.
<!-- impl TestClock::fn id_list_get_latest_time -->
Finds the latest time inside the list.

MT safe.
## `pending_list`
List
 of of pending `GstClockIDs`
<!-- impl TestClock::fn advance_time -->
Advances the time of the `self` by the amount given by `delta`. The
time of `self` is monotonically increasing, therefore providing a
`delta` which is negative or zero is a programming error.

MT safe.
## `delta`
a positive `gst::ClockTimeDiff` to be added to the time of the clock
<!-- impl TestClock::fn crank -->
A "crank" consists of three steps:
1: Wait for a `gst::ClockID` to be registered with the `TestClock`.
2: Advance the `TestClock` to the time the `gst::ClockID` is waiting, unless
 the clock time is already passed the clock id (Since: 1.18).
3: Release the `gst::ClockID` wait.
A "crank" can be though of as the notion of
manually driving the clock forward to its next logical step.

# Returns

`true` if the crank was successful, `false` otherwise.

MT safe.
<!-- impl TestClock::fn get_next_entry_time -->
Retrieve the requested time for the next pending clock notification.

MT safe.

# Returns

a `gst::ClockTime` set to the time of the next pending clock
notification. If no clock notifications have been requested
`GST_CLOCK_TIME_NONE` will be returned.
<!-- impl TestClock::fn has_id -->
Checks whether `self` was requested to provide the clock notification
given by `id`.

MT safe.
## `id`
a `gst::ClockID` clock notification

# Returns

`true` if the clock has been asked to provide the given clock
notification, `false` otherwise.
<!-- impl TestClock::fn peek_id_count -->
Determine the number of pending clock notifications that have been
requested from the `self`.

MT safe.

# Returns

the number of pending clock notifications.
<!-- impl TestClock::fn peek_next_pending_id -->
Determines if the `pending_id` is the next clock notification scheduled to
be triggered given the current time of the `self`.

MT safe.
## `pending_id`
a `gst::ClockID` clock
notification to look for

# Returns

`true` if `pending_id` is the next clock notification to be
triggered, `false` otherwise.
<!-- impl TestClock::fn process_id -->
Processes and releases the pending ID.

MT safe.

Feature: `v1_18`

## `pending_id`
`gst::ClockID`
<!-- impl TestClock::fn process_id_list -->
Processes and releases the pending IDs in the list.

MT safe.
## `pending_list`
List
 of pending `GstClockIDs`
<!-- impl TestClock::fn process_next_clock_id -->
MT safe.

# Returns

a `gst::ClockID` containing the next pending clock
notification.
<!-- impl TestClock::fn set_time -->
Sets the time of `self` to the time given by `new_time`. The time of
`self` is monotonically increasing, therefore providing a `new_time`
which is earlier or equal to the time of the clock as given by
`gst::ClockExt::get_time` is a programming error.

MT safe.
## `new_time`
a `gst::ClockTime` later than that returned by `gst::ClockExt::get_time`
<!-- impl TestClock::fn timed_wait_for_multiple_pending_ids -->
Blocks until at least `count` clock notifications have been requested from
`self`, or the timeout expires.

MT safe.

Feature: `v1_16`

## `count`
the number of pending clock notifications to wait for
## `timeout_ms`
the timeout in milliseconds
## `pending_list`
Address
 of a `glib::List` pointer variable to store the list of pending `GstClockIDs`
 that expired, or `None`

# Returns

a `gboolean` `true` if the waits have been registered, `false` if not.
(Could be that it timed out waiting or that more waits than waits was found)
<!-- impl TestClock::fn wait_for_multiple_pending_ids -->
Blocks until at least `count` clock notifications have been requested from
`self`. There is no timeout for this wait, see the main description of
`TestClock`.

MT safe.
## `count`
the number of pending clock notifications to wait for
## `pending_list`
Address
 of a `glib::List` pointer variable to store the list of pending `GstClockIDs`
 that expired, or `None`
<!-- impl TestClock::fn wait_for_next_pending_id -->
Waits until a clock notification is requested from `self`. There is no
timeout for this wait, see the main description of `TestClock`. A reference
to the pending clock notification is stored in `pending_id`.

MT safe.
## `pending_id`
`gst::ClockID`
with information about the pending clock notification
<!-- impl TestClock::fn wait_for_pending_id_count -->
Blocks until at least `count` clock notifications have been requested from
`self`. There is no timeout for this wait, see the main description of
`TestClock`.

# Deprecated

use `TestClock::wait_for_multiple_pending_ids` instead.
## `count`
the number of pending clock notifications to wait for
<!-- impl TestClock::fn get_property_start_time -->
When a `TestClock` is constructed it will have a certain start time set.
If the clock was created using `TestClock::new_with_start_time` then
this property contains the value of the `start_time` argument. If
`TestClock::new` was called the clock started at time zero, and thus
this property contains the value 0.
<!-- impl TestClock::fn set_property_start_time -->
When a `TestClock` is constructed it will have a certain start time set.
If the clock was created using `TestClock::new_with_start_time` then
this property contains the value of the `start_time` argument. If
`TestClock::new` was called the clock started at time zero, and thus
this property contains the value 0.
