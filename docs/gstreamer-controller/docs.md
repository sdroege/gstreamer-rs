<!-- file * -->
<!-- struct ARGBControlBinding -->
A value mapping object that attaches multiple control sources to a guint
gobject properties representing a color. A control value of 0.0 will turn the
color component off and a value of 1.0 will be the color level.

# Implements

[`ARGBControlBindingExt`](trait@crate::ARGBControlBindingExt), [`trait@gst::ControlBindingExt`], [`trait@gst::ObjectExt`]
<!-- trait ARGBControlBindingExt -->
Trait containing all `ARGBControlBinding` methods.

# Implementors

[`ARGBControlBinding`](struct@crate::ARGBControlBinding)
<!-- impl ARGBControlBinding::fn new -->
Create a new control-binding that attaches the given `gst::ControlSource` to the
`glib::object::Object` property.
## `object`
the object of the property
## `property_name`
the property-name to attach the control source
## `cs_a`
the control source for the alpha channel
## `cs_r`
the control source for the red channel
## `cs_g`
the control source for the green channel
## `cs_b`
the control source for the blue channel

# Returns

the new `ARGBControlBinding`
<!-- struct ControlPoint -->
An internal structure for value+time and various temporary
values used for interpolation. This "inherits" from
GstTimedValue.
<!-- impl ControlPoint::fn copy -->
Copies a `ControlPoint`

# Returns

A copy of `self`
<!-- impl ControlPoint::fn free -->
Frees all data allocated by a `ControlPoint` instance.
<!-- struct DirectControlBinding -->
A value mapping object that attaches control sources to gobject properties. It
will map the control values directly to the target property range. If a
non-absolute direct control binding is used, the value range [0.0 ... 1.0]
is mapped to full target property range, and all values outside the range
will be clipped. An absolute control binding will not do any value
transformations.

# Implements

[`DirectControlBindingExt`](trait@crate::DirectControlBindingExt), [`trait@gst::ControlBindingExt`], [`trait@gst::ObjectExt`]
<!-- trait DirectControlBindingExt -->
Trait containing all `DirectControlBinding` methods.

# Implementors

[`DirectControlBinding`](struct@crate::DirectControlBinding)
<!-- impl DirectControlBinding::fn new -->
Create a new control-binding that attaches the `gst::ControlSource` to the
`glib::object::Object` property. It will map the control source range [0.0 ... 1.0] to
the full target property range, and clip all values outside this range.
## `object`
the object of the property
## `property_name`
the property-name to attach the control source
## `cs`
the control source

# Returns

the new `DirectControlBinding`
<!-- impl DirectControlBinding::fn new_absolute -->
Create a new control-binding that attaches the `gst::ControlSource` to the
`glib::object::Object` property. It will directly map the control source values to the
target property range without any transformations.
## `object`
the object of the property
## `property_name`
the property-name to attach the control source
## `cs`
the control source

# Returns

the new `DirectControlBinding`
<!-- struct InterpolationControlSource -->
`InterpolationControlSource` is a `gst::ControlSource`, that interpolates values between user-given
control points. It supports several interpolation modes and property types.

To use `InterpolationControlSource` get a new instance by calling
`InterpolationControlSource::new`, bind it to a `glib::object::ParamSpec` and set some
control points by calling `TimedValueControlSourceExt::set`.

All functions are MT-safe.

# Implements

[`InterpolationControlSourceExt`](trait@crate::InterpolationControlSourceExt), [`TimedValueControlSourceExt`](trait@crate::TimedValueControlSourceExt), [`trait@gst::ControlSourceExt`], [`trait@gst::ObjectExt`]
<!-- trait InterpolationControlSourceExt -->
Trait containing all `InterpolationControlSource` methods.

# Implementors

[`InterpolationControlSource`](struct@crate::InterpolationControlSource)
<!-- impl InterpolationControlSource::fn new -->
This returns a new, unbound `InterpolationControlSource`.

# Returns

a new, unbound `InterpolationControlSource`.
<!-- enum InterpolationMode -->
The various interpolation modes available.
<!-- enum InterpolationMode::variant None -->
steps-like interpolation, default
<!-- enum InterpolationMode::variant Linear -->
linear interpolation
<!-- enum InterpolationMode::variant Cubic -->
cubic interpolation (natural), may overshoot
 the min or max values set by the control point, but is more 'curvy'
<!-- enum InterpolationMode::variant CubicMonotonic -->
monotonic cubic interpolation, will not
 produce any values outside of the min-max range set by the control points
 (Since: 1.8)
<!-- struct LFOControlSource -->
`LFOControlSource` is a `gst::ControlSource`, that provides several periodic
waveforms as control values.

To use `LFOControlSource` get a new instance by calling
`LFOControlSource::new`, bind it to a `glib::object::ParamSpec` and set the relevant
properties.

All functions are MT-safe.

# Implements

[`LFOControlSourceExt`](trait@crate::LFOControlSourceExt), [`trait@gst::ControlSourceExt`], [`trait@gst::ObjectExt`]
<!-- trait LFOControlSourceExt -->
Trait containing all `LFOControlSource` methods.

# Implementors

[`LFOControlSource`](struct@crate::LFOControlSource)
<!-- impl LFOControlSource::fn new -->
This returns a new, unbound `LFOControlSource`.

# Returns

a new, unbound `LFOControlSource`.
<!-- trait LFOControlSourceExt::fn get_property_amplitude -->
Specifies the amplitude for the waveform of this `LFOControlSource`.
<!-- trait LFOControlSourceExt::fn set_property_amplitude -->
Specifies the amplitude for the waveform of this `LFOControlSource`.
<!-- trait LFOControlSourceExt::fn get_property_frequency -->
Specifies the frequency that should be used for the waveform
of this `LFOControlSource`. It should be large enough
so that the period is longer than one nanosecond.
<!-- trait LFOControlSourceExt::fn set_property_frequency -->
Specifies the frequency that should be used for the waveform
of this `LFOControlSource`. It should be large enough
so that the period is longer than one nanosecond.
<!-- trait LFOControlSourceExt::fn get_property_offset -->
Specifies the value offset for the waveform of this `LFOControlSource`.
<!-- trait LFOControlSourceExt::fn set_property_offset -->
Specifies the value offset for the waveform of this `LFOControlSource`.
<!-- trait LFOControlSourceExt::fn get_property_timeshift -->
Specifies the timeshift to the right that should be used for the waveform
of this `LFOControlSource` in nanoseconds.

To get a n nanosecond shift to the left use
"(GST_SECOND / frequency) - n".
<!-- trait LFOControlSourceExt::fn set_property_timeshift -->
Specifies the timeshift to the right that should be used for the waveform
of this `LFOControlSource` in nanoseconds.

To get a n nanosecond shift to the left use
"(GST_SECOND / frequency) - n".
<!-- trait LFOControlSourceExt::fn get_property_waveform -->
Specifies the waveform that should be used for this `LFOControlSource`.
<!-- trait LFOControlSourceExt::fn set_property_waveform -->
Specifies the waveform that should be used for this `LFOControlSource`.
<!-- enum LFOWaveform -->
The various waveform modes available.
<!-- enum LFOWaveform::variant Sine -->
sine waveform
<!-- enum LFOWaveform::variant Square -->
square waveform
<!-- enum LFOWaveform::variant Saw -->
saw waveform
<!-- enum LFOWaveform::variant ReverseSaw -->
reverse saw waveform
<!-- enum LFOWaveform::variant Triangle -->
triangle waveform
<!-- struct ProxyControlBinding -->
A `gst::ControlBinding` that forwards requests to another `gst::ControlBinding`

Feature: `v1_12`

# Implements

[`trait@gst::ControlBindingExt`], [`trait@gst::ObjectExt`]
<!-- impl ProxyControlBinding::fn new -->
`ProxyControlBinding` forwards all access to data or `sync_values()`
requests from `property_name` on `object` to the control binding at
`ref_property_name` on `ref_object`.

Feature: `v1_12`

## `object`
a `gst::Object`
## `property_name`
the property name in `object` to control
## `ref_object`
a `gst::Object` to forward all
 `gst::ControlBinding` requests to
## `ref_property_name`
the property_name in `ref_object` to control

# Returns

a new `gst::ControlBinding` that proxies the control interface between
properties on different `gst::Object`'s
<!-- struct TimedValueControlSource -->
Base class for `gst::ControlSource` that use time-stamped values.

When overriding bind, chain up first to give this bind implementation a
chance to setup things.

All functions are MT-safe.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`TimedValueControlSourceExt`](trait@crate::TimedValueControlSourceExt), [`trait@gst::ControlSourceExt`], [`trait@gst::ObjectExt`]
<!-- trait TimedValueControlSourceExt -->
Trait containing all `TimedValueControlSource` methods.

# Implementors

[`InterpolationControlSource`](struct@crate::InterpolationControlSource), [`TimedValueControlSource`](struct@crate::TimedValueControlSource), [`TriggerControlSource`](struct@crate::TriggerControlSource)
<!-- trait TimedValueControlSourceExt::fn find_control_point_iter -->
Find last value before given timestamp in control point list.
If all values in the control point list come after the given
timestamp or no values exist, `None` is returned.

For use in control source implementations.
## `timestamp`
the search key

# Returns

the found `glib::SequenceIter` or `None`
<!-- trait TimedValueControlSourceExt::fn all -->
Returns a read-only copy of the list of `gst::TimedValue` for the given property.
Free the list after done with it.

# Returns

a copy
of the list, or `None` if the property isn't handled by the controller
<!-- trait TimedValueControlSourceExt::fn count -->
Get the number of control points that are set.

# Returns

the number of control points that are set.
<!-- trait TimedValueControlSourceExt::fn set -->
Set the value of given controller-handled property at a certain time.
## `timestamp`
the time the control-change is scheduled for
## `value`
the control-value

# Returns

FALSE if the values couldn't be set, TRUE otherwise.
<!-- trait TimedValueControlSourceExt::fn set_from_list -->
Sets multiple timed values at once.
## `timedvalues`
a list
with `gst::TimedValue` items

# Returns

FALSE if the values couldn't be set, TRUE otherwise.
<!-- trait TimedValueControlSourceExt::fn unset -->
Used to remove the value of given controller-handled property at a certain
time.
## `timestamp`
the time the control-change should be removed from

# Returns

FALSE if the value couldn't be unset (i.e. not found, TRUE otherwise.
<!-- trait TimedValueControlSourceExt::fn unset_all -->
Used to remove all time-stamped values of given controller-handled property
<!-- trait TimedValueControlSourceExt::fn connect_value_added -->
Emitted right after the new value has been added to `self_`
## `timed_value`
The newly added `gst::TimedValue`
<!-- trait TimedValueControlSourceExt::fn connect_value_changed -->
Emitted right after the new value has been set on `timed_signals`
## `timed_value`
The `gst::TimedValue` where the value changed
<!-- trait TimedValueControlSourceExt::fn connect_value_removed -->
Emitted when `timed_value` is removed from `self_`
## `timed_value`
The removed `gst::TimedValue`
<!-- struct TriggerControlSource -->
`TriggerControlSource` is a `gst::ControlSource`, that returns values from user-given
control points. It allows for a tolerance on the time-stamps.

To use `TriggerControlSource` get a new instance by calling
`TriggerControlSource::new`, bind it to a `glib::object::ParamSpec` and set some
control points by calling `TimedValueControlSourceExt::set`.

All functions are MT-safe.

# Implements

[`TriggerControlSourceExt`](trait@crate::TriggerControlSourceExt), [`TimedValueControlSourceExt`](trait@crate::TimedValueControlSourceExt), [`trait@gst::ControlSourceExt`], [`trait@gst::ObjectExt`]
<!-- trait TriggerControlSourceExt -->
Trait containing all `TriggerControlSource` methods.

# Implementors

[`TriggerControlSource`](struct@crate::TriggerControlSource)
<!-- impl TriggerControlSource::fn new -->
This returns a new, unbound `TriggerControlSource`.

# Returns

a new, unbound `TriggerControlSource`.
