<!-- file * -->
<!-- struct NetClientClock -->
`NetClientClock` implements a custom `gst::Clock` that synchronizes its time
to a remote time provider such as `NetTimeProvider`. `NtpClock`
implements a `gst::Clock` that synchronizes its time to a remote NTPv4 server.

A new clock is created with `NetClientClock::new` or
`NtpClock::new`, which takes the address and port of the remote time
provider along with a name and an initial time.

This clock will poll the time provider and will update its calibration
parameters based on the local and remote observations.

The "round-trip" property limits the maximum round trip packets can take.

Various parameters of the clock can be configured with the parent `gst::Clock`
"timeout", "window-size" and "window-threshold" object properties.

A `NetClientClock` and `NtpClock` is typically set on a `gst::Pipeline` with
`gst::Pipeline::use_clock`.

If you set a `gst::Bus` on the clock via the "bus" object property, it will
send `gst::MessageType::Element` messages with an attached `gst::Structure` containing
statistics about clock accuracy and network traffic.

# Implements

[`ClockExt`](trait.ClockExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- impl NetClientClock::fn new -->
Create a new `GstNetClientInternalClock` that will report the time
provided by the `NetTimeProvider` on `remote_address` and
`remote_port`.
## `name`
a name for the clock
## `remote_address`
the address or hostname of the remote clock provider
## `remote_port`
the port of the remote clock provider
## `base_time`
initial time of the clock

# Returns

a new `gst::Clock` that receives a time from the remote
clock.
<!-- struct NetTimeProvider -->
This object exposes the time of a `gst::Clock` on the network.

A `NetTimeProvider` is created with `NetTimeProvider::new` which
takes a `gst::Clock`, an address and a port number as arguments.

After creating the object, a client clock such as `NetClientClock` can
query the exposed clock over the network for its values.

The `NetTimeProvider` typically wraps the clock used by a `gst::Pipeline`.

# Implements

[`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- impl NetTimeProvider::fn new -->
Allows network clients to get the current time of `clock`.
## `clock`
a `gst::Clock` to export over the network
## `address`
an address to bind on as a dotted quad
 (xxx.xxx.xxx.xxx), IPv6 address, or NULL to bind to all addresses
## `port`
a port to bind on, or 0 to let the kernel choose

# Returns

the new `NetTimeProvider`, or NULL on error
<!-- struct NtpClock -->


# Implements

[`NetClientClockExt`](trait.NetClientClockExt.html), [`ClockExt`](trait.ClockExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- impl NtpClock::fn new -->
Create a new `NtpClock` that will report the time provided by
the NTPv4 server on `remote_address` and `remote_port`.
## `name`
a name for the clock
## `remote_address`
the address or hostname of the remote clock provider
## `remote_port`
the port of the remote clock provider
## `base_time`
initial time of the clock

# Returns

a new `gst::Clock` that receives a time from the remote
clock.
<!-- struct PtpClock -->
GstPtpClock implements a PTP (IEEE1588:2008) ordinary clock in slave-only
mode, that allows a GStreamer pipeline to synchronize to a PTP network
clock in some specific domain.

The PTP subsystem can be initialized with `gst_ptp_init`, which then starts
a helper process to do the actual communication via the PTP ports. This is
required as PTP listens on ports < 1024 and thus requires special
privileges. Once this helper process is started, the main process will
synchronize to all PTP domains that are detected on the selected
interfaces.

`PtpClock::new` then allows to create a GstClock that provides the PTP
time from a master clock inside a specific PTP domain. This clock will only
return valid timestamps once the timestamps in the PTP domain are known. To
check this, you can use `gst::ClockExt::wait_for_sync`, the GstClock::synced
signal and `gst::ClockExt::is_synced`.

To gather statistics about the PTP clock synchronization,
`gst_ptp_statistics_callback_add` can be used. This gives the application
the possibility to collect all kinds of statistics from the clock
synchronization.

# Implements

[`ClockExt`](trait.ClockExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- impl PtpClock::fn new -->
Creates a new PTP clock instance that exports the PTP time of the master
clock in `domain`. This clock can be slaved to other clocks as needed.

If `gst_ptp_init` was not called before, this will call `gst_ptp_init` with
default parameters.

This clock only returns valid timestamps after it received the first
times from the PTP master clock on the network. Once this happens the
GstPtpClock::internal-clock property will become non-NULL. You can
check this with `gst::ClockExt::wait_for_sync`, the GstClock::synced signal and
`gst::ClockExt::is_synced`.
## `name`
Name of the clock
## `domain`
PTP domain
