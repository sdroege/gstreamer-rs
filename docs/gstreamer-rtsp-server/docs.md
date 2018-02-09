<!-- file * -->
<!-- struct RTSPAddress -->
An address
<!-- impl RTSPAddress::fn copy -->
Make a copy of `self`.

# Returns

a copy of `self`.
<!-- impl RTSPAddress::fn free -->
Free `self` and releasing it back into the pool when owned by a
pool.
<!-- struct RTSPAddressPool -->
An address pool, all member are private

# Implements

[`RTSPAddressPoolExt`](trait.RTSPAddressPoolExt.html)
<!-- trait RTSPAddressPoolExt -->
Trait containing all `RTSPAddressPool` methods.

# Implementors

[`RTSPAddressPool`](struct.RTSPAddressPool.html)
<!-- impl RTSPAddressPool::fn new -->
Make a new `RTSPAddressPool`.

# Returns

a new `RTSPAddressPool`
<!-- trait RTSPAddressPoolExt::fn acquire_address -->
Take an address and ports from `self`. `flags` can be used to control the
allocation. `n_ports` consecutive ports will be allocated of which the first
one can be found in `port`.
## `flags`
flags
## `n_ports`
the amount of ports

# Returns

a `RTSPAddress` that should be freed with
gst_rtsp_address_free after use or `None` when no address could be
acquired.
<!-- trait RTSPAddressPoolExt::fn add_range -->
Adds the addresses from `min_addess` to `max_address` (inclusive)
to `self`. The valid port range for the addresses will be from `min_port` to
`max_port` inclusive.

When `ttl` is 0, `min_address` and `max_address` should be unicast addresses.
`min_address` and `max_address` can be set to
`GST_RTSP_ADDRESS_POOL_ANY_IPV4` or `GST_RTSP_ADDRESS_POOL_ANY_IPV6` to bind
to all available IPv4 or IPv6 addresses.

When `ttl` > 0, `min_address` and `max_address` should be multicast addresses.
## `min_address`
a minimum address to add
## `max_address`
a maximum address to add
## `min_port`
the minimum port
## `max_port`
the maximum port
## `ttl`
a TTL or 0 for unicast addresses

# Returns

`true` if the addresses could be added.
<!-- trait RTSPAddressPoolExt::fn clear -->
Clear all addresses in `self`. There should be no outstanding
allocations.
<!-- trait RTSPAddressPoolExt::fn dump -->
Dump the free and allocated addresses to stdout.
<!-- trait RTSPAddressPoolExt::fn has_unicast_addresses -->
Used to know if the pool includes any unicast addresses.

# Returns

`true` if the pool includes any unicast addresses, `false` otherwise
<!-- trait RTSPAddressPoolExt::fn reserve_address -->
Take a specific address and ports from `self`. `n_ports` consecutive
ports will be allocated of which the first one can be found in
`port`.

If `ttl` is 0, `address` should be a unicast address. If `ttl` > 0, `address`
should be a valid multicast address.
## `ip_address`
The IP address to reserve
## `port`
The first port to reserve
## `n_ports`
The number of ports
## `ttl`
The requested ttl
## `address`
storage for a `RTSPAddress`

# Returns

`RTSPAddressPoolResult::Ok` if an address was reserved. The address
is returned in `address` and should be freed with gst_rtsp_address_free
after use.
<!-- enum RTSPAddressPoolResult -->
Result codes from RTSP address pool functions.
<!-- enum RTSPAddressPoolResult::variant Ok -->
no error
<!-- enum RTSPAddressPoolResult::variant Einval -->
invalid arguments were provided to a function
<!-- enum RTSPAddressPoolResult::variant Ereserved -->
the addres has already been reserved
<!-- enum RTSPAddressPoolResult::variant Erange -->
the address is not in the pool
<!-- enum RTSPAddressPoolResult::variant Elast -->
last error
<!-- struct RTSPAuth -->
The authentication structure.

# Implements

[`RTSPAuthExt`](trait.RTSPAuthExt.html)
<!-- trait RTSPAuthExt -->
Trait containing all `RTSPAuth` methods.

# Implementors

[`RTSPAuth`](struct.RTSPAuth.html)
<!-- impl RTSPAuth::fn new -->
Create a new `RTSPAuth` instance.

# Returns

a new `RTSPAuth`
<!-- impl RTSPAuth::fn check -->
Check if `check` is allowed in the current context.
## `check`
the item to check

# Returns

FALSE if check failed.
<!-- impl RTSPAuth::fn make_basic -->
Construct a Basic authorisation token from `user` and `pass`.
## `user`
a userid
## `pass`
a password

# Returns

the base64 encoding of the string `user`:`pass`.
`g_free` after usage.
<!-- trait RTSPAuthExt::fn add_basic -->
Add a basic token for the default authentication algorithm that
enables the client with privileges listed in `token`.
## `basic`
the basic token
## `token`
authorisation token
<!-- trait RTSPAuthExt::fn add_digest -->
Add a digest `user` and `pass` for the default authentication algorithm that
enables the client with privileges listed in `token`.

Feature: `v1_12`

## `user`
the digest user name
## `pass`
the digest password
## `token`
authorisation token
<!-- trait RTSPAuthExt::fn get_default_token -->
Get the default token for `self`. This token will be used for unauthenticated
users.

# Returns

the `RTSPToken` of `self`. `gst_rtsp_token_unref` after
usage.
<!-- trait RTSPAuthExt::fn get_supported_methods -->
Gets the supported authentication methods of `self`.

Feature: `v1_12`


# Returns

The supported authentication methods
<!-- trait RTSPAuthExt::fn get_tls_authentication_mode -->
Get the `gio::TlsAuthenticationMode`.

# Returns

the `gio::TlsAuthenticationMode`.
<!-- trait RTSPAuthExt::fn get_tls_certificate -->
Get the `gio::TlsCertificate` used for negotiating TLS `self`.

# Returns

the `gio::TlsCertificate` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPAuthExt::fn get_tls_database -->
Get the `gio::TlsDatabase` used for verifying client certificate.

# Returns

the `gio::TlsDatabase` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPAuthExt::fn remove_basic -->
Removes `basic` authentication token.
## `basic`
the basic token
<!-- trait RTSPAuthExt::fn remove_digest -->
Removes a digest user.

Feature: `v1_12`

## `user`
the digest user name
<!-- trait RTSPAuthExt::fn set_default_token -->
Set the default `RTSPToken` to `token` in `self`. The default token will
be used for unauthenticated users.
## `token`
a `RTSPToken`
<!-- trait RTSPAuthExt::fn set_supported_methods -->
Sets the supported authentication `methods` for `self`.

Feature: `v1_12`

## `methods`
supported methods
<!-- trait RTSPAuthExt::fn set_tls_authentication_mode -->
The `gio::TlsAuthenticationMode` to set on the underlying GTlsServerConnection.
When set to another value than `gio::TlsAuthenticationMode::None`,
`RTSPAuth::accept-certificate` signal will be emitted and must be handled.
## `mode`
a `gio::TlsAuthenticationMode`
<!-- trait RTSPAuthExt::fn set_tls_certificate -->
Set the TLS certificate for the auth. Client connections will only
be accepted when TLS is negotiated.
## `cert`
a `gio::TlsCertificate`
<!-- trait RTSPAuthExt::fn set_tls_database -->
Sets the certificate database that is used to verify peer certificates.
If set to `None` (the default), then peer certificate validation will always
set the `gio::TlsCertificateFlags::UnknownCa` error.

Since 1.6
## `database`
a `gio::TlsDatabase`
<!-- trait RTSPAuthExt::fn connect_accept_certificate -->
Emitted during the TLS handshake after the client certificate has
been received. See also `RTSPAuthExt::set_tls_authentication_mode`.
## `connection`
a `gio::TlsConnection`
## `peer_cert`
the peer's `gio::TlsCertificate`
## `errors`
the problems with `peer_cert`.

# Returns

`true` to accept `peer_cert` (which will also
immediately end the signal emission). `false` to allow the signal
emission to continue, which will cause the handshake to fail if
no one else overrides it.
<!-- struct RTSPClient -->
The client object represents the connection and its state with a client.

# Implements

[`RTSPClientExt`](trait.RTSPClientExt.html)
<!-- trait RTSPClientExt -->
Trait containing all `RTSPClient` methods.

# Implementors

[`RTSPClient`](struct.RTSPClient.html)
<!-- impl RTSPClient::fn new -->
Create a new `RTSPClient` instance.

# Returns

a new `RTSPClient`
<!-- trait RTSPClientExt::fn attach -->
Attaches `self` to `context`. When the mainloop for `context` is run, the
client will be dispatched. When `context` is `None`, the default context will be
used).

This function should be called when the client properties and urls are fully
configured and the client is ready to start.
## `context`
a `glib::MainContext`

# Returns

the ID (greater than 0) for the source within the GMainContext.
<!-- trait RTSPClientExt::fn close -->
Close the connection of `self` and remove all media it was managing.
<!-- trait RTSPClientExt::fn get_auth -->
Get the `RTSPAuth` used as the authentication manager of `self`.

# Returns

the `RTSPAuth` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPClientExt::fn get_connection -->
Get the `gst_rtsp::RTSPConnection` of `self`.

# Returns

the `gst_rtsp::RTSPConnection` of `self`.
The connection object returned remains valid until the client is freed.
<!-- trait RTSPClientExt::fn get_mount_points -->
Get the `RTSPMountPoints` object that `self` uses to manage its sessions.

# Returns

a `RTSPMountPoints`, unref after usage.
<!-- trait RTSPClientExt::fn get_session_pool -->
Get the `RTSPSessionPool` object that `self` uses to manage its sessions.

# Returns

a `RTSPSessionPool`, unref after usage.
<!-- trait RTSPClientExt::fn get_thread_pool -->
Get the `RTSPThreadPool` used as the thread pool of `self`.

# Returns

the `RTSPThreadPool` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPClientExt::fn handle_message -->
Let the client handle `message`.
## `message`
an `gst_rtsp::RTSPMessage`

# Returns

a `gst_rtsp::RTSPResult`.
<!-- trait RTSPClientExt::fn send_message -->
Send a message message to the remote end. `message` must be a
`gst_rtsp::RTSPMsgType::Request` or a `gst_rtsp::RTSPMsgType::Response`.
## `session`
a `RTSPSession` to send
 the message to or `None`
## `message`
The `gst_rtsp::RTSPMessage` to send
<!-- trait RTSPClientExt::fn session_filter -->
Call `func` for each session managed by `self`. The result value of `func`
determines what happens to the session. `func` will be called with `self`
locked so no further actions on `self` can be performed from `func`.

If `func` returns `RTSPFilterResult::Remove`, the session will be removed from
`self`.

If `func` returns `RTSPFilterResult::Keep`, the session will remain in `self`.

If `func` returns `RTSPFilterResult::Ref`, the session will remain in `self` but
will also be added with an additional ref to the result `glib::List` of this
function..

When `func` is `None`, `RTSPFilterResult::Ref` will be assumed for each session.
## `func`
a callback
## `user_data`
user data passed to `func`

# Returns

a `glib::List` with all
sessions for which `func` returned `RTSPFilterResult::Ref`. After usage, each
element in the `glib::List` should be unreffed before the list is freed.
<!-- trait RTSPClientExt::fn set_auth -->
configure `auth` to be used as the authentication manager of `self`.
## `auth`
a `RTSPAuth`
<!-- trait RTSPClientExt::fn set_connection -->
Set the `gst_rtsp::RTSPConnection` of `self`. This function takes ownership of
`conn`.
## `conn`
a `gst_rtsp::RTSPConnection`

# Returns

`true` on success.
<!-- trait RTSPClientExt::fn set_mount_points -->
Set `mounts` as the mount points for `self` which it will use to map urls
to media streams. These mount points are usually inherited from the server that
created the client but can be overriden later.
## `mounts`
a `RTSPMountPoints`
<!-- trait RTSPClientExt::fn set_send_func -->
Set `func` as the callback that will be called when a new message needs to be
sent to the client. `user_data` is passed to `func` and `notify` is called when
`user_data` is no longer in use.

By default, the client will send the messages on the `gst_rtsp::RTSPConnection` that
was configured with `RTSPClientExt::attach` was called.
## `func`
a `GstRTSPClientSendFunc`
## `user_data`
user data passed to `func`
## `notify`
called when `user_data` is no longer in use
<!-- trait RTSPClientExt::fn set_session_pool -->
Set `pool` as the sessionpool for `self` which it will use to find
or allocate sessions. the sessionpool is usually inherited from the server
that created the client but can be overridden later.
## `pool`
a `RTSPSessionPool`
<!-- trait RTSPClientExt::fn set_thread_pool -->
configure `pool` to be used as the thread pool of `self`.
## `pool`
a `RTSPThreadPool`
<!-- trait RTSPClientExt::fn connect_announce_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_check_requirements -->
## `ctx`
a `RTSPContext`
## `arr`
a NULL-terminated array of strings

# Returns

a newly allocated string with comma-separated list of
 unsupported options. An empty string must be returned if
 all options are supported.
<!-- trait RTSPClientExt::fn connect_describe_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_get_parameter_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_handle_response -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_options_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_pause_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_play_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_pre_announce_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_describe_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_get_parameter_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_options_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_pause_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_play_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_record_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_set_parameter_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_setup_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_pre_teardown_request -->

Feature: `v1_12`

## `ctx`
a `RTSPContext`

# Returns

a `gst_rtsp::RTSPStatusCode`, GST_RTSP_STS_OK in case of success,
 otherwise an appropriate return code
<!-- trait RTSPClientExt::fn connect_record_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_send_message -->
## `session`
The session
## `message`
The message
<!-- trait RTSPClientExt::fn connect_set_parameter_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_setup_request -->
## `ctx`
a `RTSPContext`
<!-- trait RTSPClientExt::fn connect_teardown_request -->
## `ctx`
a `RTSPContext`
<!-- struct RTSPMedia -->
A class that contains the GStreamer element along with a list of
`RTSPStream` objects that can produce data.

This object is usually created from a `RTSPMediaFactory`.

# Implements

[`RTSPMediaExt`](trait.RTSPMediaExt.html)
<!-- trait RTSPMediaExt -->
Trait containing all `RTSPMedia` methods.

# Implementors

[`RTSPMedia`](struct.RTSPMedia.html)
<!-- impl RTSPMedia::fn new -->
Create a new `RTSPMedia` instance. `element` is the bin element that
provides the different streams. The `RTSPMedia` object contains the
element to produce RTP data for one or more related (audio/video/..)
streams.

Ownership is taken of `element`.
## `element`
a `gst::Element`

# Returns

a new `RTSPMedia` object.
<!-- trait RTSPMediaExt::fn collect_streams -->
Find all payloader elements, they should be named pay\%d in the
element of `self`, and create `GstRTSPStreams` for them.

Collect all dynamic elements, named dynpay\%d, and add them to
the list of dynamic elements.

Find all depayloader elements, they should be named depay\%d in the
element of `self`, and create `GstRTSPStreams` for them.
<!-- trait RTSPMediaExt::fn create_stream -->
Create a new stream in `self` that provides RTP data on `pad`.
`pad` should be a pad of an element inside `self`->element.
## `payloader`
a `gst::Element`
## `pad`
a `gst::Pad`

# Returns

a new `RTSPStream` that remains valid for as long
as `self` exists.
<!-- trait RTSPMediaExt::fn find_stream -->
Find a stream in `self` with `control` as the control uri.
## `control`
the control of the stream

# Returns

the `RTSPStream` with
control uri `control` or `None` when a stream with that control did
not exist.
<!-- trait RTSPMediaExt::fn get_address_pool -->
Get the `RTSPAddressPool` used as the address pool of `self`.

# Returns

the `RTSPAddressPool` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPMediaExt::fn get_base_time -->
Get the base_time that is used by the pipeline in `self`.

`self` must be prepared before this method returns a valid base_time.

# Returns

the base_time used by `self`.
<!-- trait RTSPMediaExt::fn get_buffer_size -->
Get the kernel UDP buffer size.

# Returns

the kernel UDP buffer size.
<!-- trait RTSPMediaExt::fn get_clock -->
Get the clock that is used by the pipeline in `self`.

`self` must be prepared before this method returns a valid clock object.

# Returns

the `gst::Clock` used by `self`. unref after usage.
<!-- trait RTSPMediaExt::fn get_element -->
Get the element that was used when constructing `self`.

# Returns

a `gst::Element`. Unref after usage.
<!-- trait RTSPMediaExt::fn get_latency -->
Get the latency that is used for receiving media.

# Returns

latency in milliseconds
<!-- trait RTSPMediaExt::fn get_multicast_iface -->
Get the multicast interface used for `self`.

# Returns

the multicast interface for `self`. `g_free` after
usage.
<!-- trait RTSPMediaExt::fn get_permissions -->
Get the permissions object from `self`.

# Returns

a `RTSPPermissions` object, unref after usage.
<!-- trait RTSPMediaExt::fn get_profiles -->
Get the allowed profiles of `self`.

# Returns

a `gst_rtsp::RTSPProfile`
<!-- trait RTSPMediaExt::fn get_protocols -->
Get the allowed protocols of `self`.

# Returns

a `gst_rtsp::RTSPLowerTrans`
<!-- trait RTSPMediaExt::fn get_publish_clock_mode -->
Gets if and how the media clock should be published according to RFC7273.

# Returns

The GstRTSPPublishClockMode
<!-- trait RTSPMediaExt::fn get_range_string -->
Get the current range as a string. `self` must be prepared with
gst_rtsp_media_prepare ().
## `play`
for the PLAY request
## `unit`
the unit to use for the string

# Returns

The range as a string, `g_free` after usage.
<!-- trait RTSPMediaExt::fn get_retransmission_time -->
Get the amount of time to store retransmission data.

# Returns

the amount of time to store retransmission data.
<!-- trait RTSPMediaExt::fn get_status -->
Get the status of `self`. When `self` is busy preparing, this function waits
until `self` is prepared or in error.

# Returns

the status of `self`.
<!-- trait RTSPMediaExt::fn get_stream -->
Retrieve the stream with index `idx` from `self`.
## `idx`
the stream index

# Returns

the `RTSPStream` at index
`idx` or `None` when a stream with that index did not exist.
<!-- trait RTSPMediaExt::fn get_suspend_mode -->
Get how `self` will be suspended.

# Returns

`RTSPSuspendMode`.
<!-- trait RTSPMediaExt::fn get_time_provider -->
Get the `gst_net::NetTimeProvider` for the clock used by `self`. The time provider
will listen on `address` and `port` for client time requests.
## `address`
an address or `None`
## `port`
a port or 0

# Returns

the `gst_net::NetTimeProvider` of `self`.
<!-- trait RTSPMediaExt::fn get_transport_mode -->
Check if the pipeline for `self` can be used for PLAY or RECORD methods.

# Returns

The transport mode.
<!-- trait RTSPMediaExt::fn handle_sdp -->
Configure an SDP on `self` for receiving streams
## `sdp`
a `gst_sdp::SDPMessage`

# Returns

TRUE on success.
<!-- trait RTSPMediaExt::fn is_eos_shutdown -->
Check if the pipeline for `self` will send an EOS down the pipeline before
unpreparing.

# Returns

`true` if the media will send EOS before unpreparing.
<!-- trait RTSPMediaExt::fn is_reusable -->
Check if the pipeline for `self` can be reused after an unprepare.

# Returns

`true` if the media can be reused
<!-- trait RTSPMediaExt::fn is_shared -->
Check if the pipeline for `self` can be shared between multiple clients.

# Returns

`true` if the media can be shared between clients.
<!-- trait RTSPMediaExt::fn is_stop_on_disconnect -->
Check if the pipeline for `self` will be stopped when a client disconnects
without sending TEARDOWN.

# Returns

`true` if the media will be stopped when a client disconnects
 without sending TEARDOWN.
<!-- trait RTSPMediaExt::fn is_time_provider -->
Check if `self` can provide a `gst_net::NetTimeProvider` for its pipeline clock.

Use `RTSPMediaExt::get_time_provider` to get the network clock.

# Returns

`true` if `self` can provide a `gst_net::NetTimeProvider`.
<!-- trait RTSPMediaExt::fn n_streams -->
Get the number of streams in this media.

# Returns

The number of streams.
<!-- trait RTSPMediaExt::fn prepare -->
Prepare `self` for streaming. This function will create the objects
to manage the streaming. A pipeline must have been set on `self` with
`RTSPMediaExt::take_pipeline`.

It will preroll the pipeline and collect vital information about the streams
such as the duration.
## `thread`
a `RTSPThread` to run the
 bus handler or `None`

# Returns

`true` on success.
<!-- trait RTSPMediaExt::fn seek -->
Seek the pipeline of `self` to `range`. `self` must be prepared with
`RTSPMediaExt::prepare`.
## `range`
a `gst_rtsp::RTSPTimeRange`

# Returns

`true` on success.
<!-- trait RTSPMediaExt::fn set_address_pool -->
configure `pool` to be used as the address pool of `self`.
## `pool`
a `RTSPAddressPool`
<!-- trait RTSPMediaExt::fn set_buffer_size -->
Set the kernel UDP buffer size.
## `size`
the new value
<!-- trait RTSPMediaExt::fn set_clock -->
Configure the clock used for the media.
## `clock`
`gst::Clock` to be used
<!-- trait RTSPMediaExt::fn set_eos_shutdown -->
Set or unset if an EOS event will be sent to the pipeline for `self` before
it is unprepared.
## `eos_shutdown`
the new value
<!-- trait RTSPMediaExt::fn set_latency -->
Configure the latency used for receiving media.
## `latency`
latency in milliseconds
<!-- trait RTSPMediaExt::fn set_multicast_iface -->
configure `multicast_iface` to be used for `self`.
## `multicast_iface`
a multicast interface name
<!-- trait RTSPMediaExt::fn set_permissions -->
Set `permissions` on `self`.
## `permissions`
a `RTSPPermissions`
<!-- trait RTSPMediaExt::fn set_pipeline_state -->
Set the state of the pipeline managed by `self` to `state`
## `state`
the target state of the pipeline
<!-- trait RTSPMediaExt::fn set_profiles -->
Configure the allowed lower transport for `self`.
## `profiles`
the new flags
<!-- trait RTSPMediaExt::fn set_protocols -->
Configure the allowed lower transport for `self`.
## `protocols`
the new flags
<!-- trait RTSPMediaExt::fn set_publish_clock_mode -->
Sets if and how the media clock should be published according to RFC7273.
## `mode`
the clock publish mode
<!-- trait RTSPMediaExt::fn set_retransmission_time -->
Set the amount of time to store retransmission packets.
## `time`
the new value
<!-- trait RTSPMediaExt::fn set_reusable -->
Set or unset if the pipeline for `self` can be reused after the pipeline has
been unprepared.
## `reusable`
the new value
<!-- trait RTSPMediaExt::fn set_shared -->
Set or unset if the pipeline for `self` can be shared will multiple clients.
When `shared` is `true`, client requests for this media will share the media
pipeline.
## `shared`
the new value
<!-- trait RTSPMediaExt::fn set_state -->
Set the state of `self` to `state` and for the transports in `transports`.

`self` must be prepared with `RTSPMediaExt::prepare`;
## `state`
the target state of the media
## `transports`

a `glib::PtrArray` of `RTSPStreamTransport` pointers

# Returns

`true` on success.
<!-- trait RTSPMediaExt::fn set_stop_on_disconnect -->
Set or unset if the pipeline for `self` should be stopped when a
client disconnects without sending TEARDOWN.
## `stop_on_disconnect`
the new value
<!-- trait RTSPMediaExt::fn set_suspend_mode -->
Control how @ media will be suspended after the SDP has been generated and
after a PAUSE request has been performed.

Media must be unprepared when setting the suspend mode.
## `mode`
the new `RTSPSuspendMode`
<!-- trait RTSPMediaExt::fn set_transport_mode -->
Sets if the media pipeline can work in PLAY or RECORD mode
## `mode`
the new value
<!-- trait RTSPMediaExt::fn setup_sdp -->
Add `self` specific info to `sdp`. `info` is used to configure the connection
information in the SDP.
## `sdp`
a `gst_sdp::SDPMessage`
## `info`
a `SDPInfo`

# Returns

TRUE on success.
<!-- trait RTSPMediaExt::fn suspend -->
Suspend `self`. The state of the pipeline managed by `self` is set to
GST_STATE_NULL but all streams are kept. `self` can be prepared again
with `RTSPMediaExt::unsuspend`

`self` must be prepared with `RTSPMediaExt::prepare`;

# Returns

`true` on success.
<!-- trait RTSPMediaExt::fn take_pipeline -->
Set `pipeline` as the `gst::Pipeline` for `self`. Ownership is
taken of `pipeline`.
## `pipeline`
a `gst::Pipeline`
<!-- trait RTSPMediaExt::fn unprepare -->
Unprepare `self`. After this call, the media should be prepared again before
it can be used again. If the media is set to be non-reusable, a new instance
must be created.

# Returns

`true` on success.
<!-- trait RTSPMediaExt::fn unsuspend -->
Unsuspend `self` if it was in a suspended state. This method does nothing
when the media was not in the suspended state.

# Returns

`true` on success.
<!-- trait RTSPMediaExt::fn use_time_provider -->
Set `self` to provide a `gst_net::NetTimeProvider`.
## `time_provider`
if a `gst_net::NetTimeProvider` should be used
<!-- struct RTSPMediaFactory -->
The definition and logic for constructing the pipeline for a media. The media
can contain multiple streams like audio and video.

# Implements

[`RTSPMediaFactoryExt`](trait.RTSPMediaFactoryExt.html)
<!-- trait RTSPMediaFactoryExt -->
Trait containing all `RTSPMediaFactory` methods.

# Implementors

[`RTSPMediaFactoryURI`](struct.RTSPMediaFactoryURI.html), [`RTSPMediaFactory`](struct.RTSPMediaFactory.html)
<!-- impl RTSPMediaFactory::fn new -->
Create a new `RTSPMediaFactory` instance.

# Returns

a new `RTSPMediaFactory` object.
<!-- trait RTSPMediaFactoryExt::fn add_role -->
A convenience method to add `role` with `fieldname` and additional arguments to
the permissions of `self`. If `self` had no permissions, new permissions
will be created and the role will be added to it.
## `role`
a role
## `fieldname`
the first field name
<!-- trait RTSPMediaFactoryExt::fn construct -->
Construct the media object and create its streams. Implementations
should create the needed gstreamer elements and add them to the result
object. No state changes should be performed on them yet.

One or more GstRTSPStream objects should be created from the result
with gst_rtsp_media_create_stream ().

After the media is constructed, it can be configured and then prepared
with gst_rtsp_media_prepare ().
## `url`
the url used

# Returns

a new `RTSPMedia` if the media could be prepared.
<!-- trait RTSPMediaFactoryExt::fn create_element -->
Construct and return a `gst::Element` that is a `gst::Bin` containing
the elements to use for streaming the media.

The bin should contain payloaders pay\%d for each stream. The default
implementation of this function returns the bin created from the
launch parameter.
## `url`
the url used

# Returns

a new `gst::Element`.
<!-- trait RTSPMediaFactoryExt::fn get_address_pool -->
Get the `RTSPAddressPool` used as the address pool of `self`.

# Returns

the `RTSPAddressPool` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPMediaFactoryExt::fn get_buffer_size -->
Get the kernel UDP buffer size.

# Returns

the kernel UDP buffer size.
<!-- trait RTSPMediaFactoryExt::fn get_clock -->
Returns the clock that is going to be used by the pipelines
of all medias created from this factory.

# Returns

The GstClock
<!-- trait RTSPMediaFactoryExt::fn get_latency -->
Get the latency that is used for receiving media

# Returns

latency in milliseconds
<!-- trait RTSPMediaFactoryExt::fn get_launch -->
Get the `gst_parse_launch` pipeline description that will be used in the
default prepare vmethod.

# Returns

the configured launch description. `g_free` after
usage.
<!-- trait RTSPMediaFactoryExt::fn get_media_gtype -->
Return the GType of the GstRTSPMedia subclass this
factory will create.
<!-- trait RTSPMediaFactoryExt::fn get_multicast_iface -->
Get the multicast interface used for `self`.

# Returns

the multicast interface for `self`. `g_free` after
usage.
<!-- trait RTSPMediaFactoryExt::fn get_permissions -->
Get the permissions object from `self`.

# Returns

a `RTSPPermissions` object, unref after usage.
<!-- trait RTSPMediaFactoryExt::fn get_profiles -->
Get the allowed profiles of `self`.

# Returns

a `gst_rtsp::RTSPProfile`
<!-- trait RTSPMediaFactoryExt::fn get_protocols -->
Get the allowed protocols of `self`.

# Returns

a `gst_rtsp::RTSPLowerTrans`
<!-- trait RTSPMediaFactoryExt::fn get_publish_clock_mode -->
Gets if and how the media clock should be published according to RFC7273.

# Returns

The GstRTSPPublishClockMode
<!-- trait RTSPMediaFactoryExt::fn get_retransmission_time -->
Get the time that is stored for retransmission purposes

# Returns

a `gst::ClockTime`
<!-- trait RTSPMediaFactoryExt::fn get_suspend_mode -->
Get how media created from this factory will be suspended.

# Returns

a `RTSPSuspendMode`.
<!-- trait RTSPMediaFactoryExt::fn get_transport_mode -->
Get if media created from this factory can be used for PLAY or RECORD
methods.

# Returns

The supported transport modes.
<!-- trait RTSPMediaFactoryExt::fn is_eos_shutdown -->
Get if media created from this factory will have an EOS event sent to the
pipeline before shutdown.

# Returns

`true` if the media will receive EOS before shutdown.
<!-- trait RTSPMediaFactoryExt::fn is_shared -->
Get if media created from this factory can be shared between clients.

# Returns

`true` if the media will be shared between clients.
<!-- trait RTSPMediaFactoryExt::fn set_address_pool -->
configure `pool` to be used as the address pool of `self`.
## `pool`
a `RTSPAddressPool`
<!-- trait RTSPMediaFactoryExt::fn set_buffer_size -->
Set the kernel UDP buffer size.
## `size`
the new value
<!-- trait RTSPMediaFactoryExt::fn set_clock -->
Configures a specific clock to be used by the pipelines
of all medias created from this factory.
## `clock`
the clock to be used by the media factory
<!-- trait RTSPMediaFactoryExt::fn set_eos_shutdown -->
Configure if media created from this factory will have an EOS sent to the
pipeline before shutdown.
## `eos_shutdown`
the new value
<!-- trait RTSPMediaFactoryExt::fn set_latency -->
Configure the latency used for receiving media
## `latency`
latency in milliseconds
<!-- trait RTSPMediaFactoryExt::fn set_launch -->
The `gst_parse_launch` line to use for constructing the pipeline in the
default prepare vmethod.

The pipeline description should return a GstBin as the toplevel element
which can be accomplished by enclosing the description with brackets '('
')'.

The description should return a pipeline with payloaders named pay0, pay1,
etc.. Each of the payloaders will result in a stream.
## `launch`
the launch description
<!-- trait RTSPMediaFactoryExt::fn set_media_gtype -->
Configure the GType of the GstRTSPMedia subclass to
create (by default, overridden construct vmethods
may of course do something different)
## `media_gtype`
the GType of the class to create
<!-- trait RTSPMediaFactoryExt::fn set_multicast_iface -->
configure `multicast_iface` to be used for `self`.
## `multicast_iface`
a multicast interface name
<!-- trait RTSPMediaFactoryExt::fn set_permissions -->
Set `permissions` on `self`.
## `permissions`
a `RTSPPermissions`
<!-- trait RTSPMediaFactoryExt::fn set_profiles -->
Configure the allowed profiles for `self`.
## `profiles`
the new flags
<!-- trait RTSPMediaFactoryExt::fn set_protocols -->
Configure the allowed lower transport for `self`.
## `protocols`
the new flags
<!-- trait RTSPMediaFactoryExt::fn set_publish_clock_mode -->
Sets if and how the media clock should be published according to RFC7273.
## `mode`
the clock publish mode
<!-- trait RTSPMediaFactoryExt::fn set_retransmission_time -->
Configure the time to store for possible retransmission
## `time`
a `gst::ClockTime`
<!-- trait RTSPMediaFactoryExt::fn set_shared -->
Configure if media created from this factory can be shared between clients.
## `shared`
the new value
<!-- trait RTSPMediaFactoryExt::fn set_stop_on_disconnect -->
Configure if media created from this factory should be stopped
when a client disconnects without sending TEARDOWN.
## `stop_on_disconnect`
the new value
<!-- trait RTSPMediaFactoryExt::fn set_suspend_mode -->
Configure how media created from this factory will be suspended.
## `mode`
the new `RTSPSuspendMode`
<!-- trait RTSPMediaFactoryExt::fn set_transport_mode -->
Configure if this factory creates media for PLAY or RECORD modes.
## `mode`
the new value
<!-- struct RTSPMediaFactoryURI -->
A media factory that creates a pipeline to play and uri.

# Implements

[`RTSPMediaFactoryURIExt`](trait.RTSPMediaFactoryURIExt.html), [`RTSPMediaFactoryExt`](trait.RTSPMediaFactoryExt.html)
<!-- trait RTSPMediaFactoryURIExt -->
Trait containing all `RTSPMediaFactoryURI` methods.

# Implementors

[`RTSPMediaFactoryURI`](struct.RTSPMediaFactoryURI.html)
<!-- impl RTSPMediaFactoryURI::fn new -->
Create a new `RTSPMediaFactoryURI` instance.

# Returns

a new `RTSPMediaFactoryURI` object.
<!-- trait RTSPMediaFactoryURIExt::fn get_uri -->
Get the URI that will provide media for this factory.

# Returns

the configured URI. `g_free` after usage.
<!-- trait RTSPMediaFactoryURIExt::fn set_uri -->
Set the URI of the resource that will be streamed by this factory.
## `uri`
the uri the stream
<!-- enum RTSPMediaStatus -->
The state of the media pipeline.
<!-- enum RTSPMediaStatus::variant Unprepared -->
media pipeline not prerolled
<!-- enum RTSPMediaStatus::variant Unpreparing -->
media pipeline is busy doing a clean
 shutdown.
<!-- enum RTSPMediaStatus::variant Preparing -->
media pipeline is prerolling
<!-- enum RTSPMediaStatus::variant Prepared -->
media pipeline is prerolled
<!-- enum RTSPMediaStatus::variant Suspended -->
media is suspended
<!-- enum RTSPMediaStatus::variant Error -->
media pipeline is in error
<!-- struct RTSPMountPoints -->
Creates a `RTSPMediaFactory` object for a given url.

# Implements

[`RTSPMountPointsExt`](trait.RTSPMountPointsExt.html)
<!-- trait RTSPMountPointsExt -->
Trait containing all `RTSPMountPoints` methods.

# Implementors

[`RTSPMountPoints`](struct.RTSPMountPoints.html)
<!-- impl RTSPMountPoints::fn new -->
Make a new mount points object.

# Returns

a new `RTSPMountPoints`
<!-- trait RTSPMountPointsExt::fn add_factory -->
Attach `factory` to the mount point `path` in `self`.

`path` is of the form (/node)+. Any previous mount point will be freed.

Ownership is taken of the reference on `factory` so that `factory` should not be
used after calling this function.
## `path`
a mount point
## `factory`
a `RTSPMediaFactory`
<!-- trait RTSPMountPointsExt::fn make_path -->
Make a path string from `url`.
## `url`
a `gst_rtsp::RTSPUrl`

# Returns

a path string for `url`, `g_free` after usage.
<!-- trait RTSPMountPointsExt::fn match -->
Find the factory in `self` that has the longest match with `path`.

If `matched` is `None`, `path` will match the factory exactly otherwise
the amount of characters that matched is returned in `matched`.
## `path`
a mount point
## `matched`
the amount of `path` matched

# Returns

the `RTSPMediaFactory` for `path`.
`gobject::Object::unref` after usage.
<!-- trait RTSPMountPointsExt::fn remove_factory -->
Remove the `RTSPMediaFactory` associated with `path` in `self`.
## `path`
a mount point
<!-- enum RTSPPublishClockMode -->
Whether the clock and possibly RTP/clock offset should be published according to RFC7273.
<!-- enum RTSPPublishClockMode::variant None -->
Publish nothing
<!-- enum RTSPPublishClockMode::variant Clock -->
Publish the clock but not the offset
<!-- enum RTSPPublishClockMode::variant ClockAndOffset -->
Publish the clock and offset
<!-- struct RTSPServer -->
This object listens on a port, creates and manages the clients connected to
it.

# Implements

[`RTSPServerExt`](trait.RTSPServerExt.html)
<!-- trait RTSPServerExt -->
Trait containing all `RTSPServer` methods.

# Implementors

[`RTSPServer`](struct.RTSPServer.html)
<!-- impl RTSPServer::fn new -->
Create a new `RTSPServer` instance.

# Returns

a new `RTSPServer`
<!-- impl RTSPServer::fn io_func -->
A default `GSocketSourceFunc` that creates a new `RTSPClient` to accept and handle a
new connection on `socket` or `server`.
## `socket`
a `gio::Socket`
## `condition`
the condition on `source`
## `server`
a `RTSPServer`

# Returns

TRUE if the source could be connected, FALSE if an error occurred.
<!-- trait RTSPServerExt::fn attach -->
Attaches `self` to `context`. When the mainloop for `context` is run, the
server will be dispatched. When `context` is `None`, the default context will be
used).

This function should be called when the server properties and urls are fully
configured and the server is ready to start.

This takes a reference on `self` until the source is destroyed. Note that
if `context` is not the default main context as returned by
`glib::MainContext::default` (or `None`), `glib::Source::remove` cannot be used to
destroy the source. In that case it is recommended to use
`RTSPServerExt::create_source` and attach it to `context` manually.
## `context`
a `glib::MainContext`

# Returns

the ID (greater than 0) for the source within the GMainContext.
<!-- trait RTSPServerExt::fn client_filter -->
Call `func` for each client managed by `self`. The result value of `func`
determines what happens to the client. `func` will be called with `self`
locked so no further actions on `self` can be performed from `func`.

If `func` returns `RTSPFilterResult::Remove`, the client will be removed from
`self`.

If `func` returns `RTSPFilterResult::Keep`, the client will remain in `self`.

If `func` returns `RTSPFilterResult::Ref`, the client will remain in `self` but
will also be added with an additional ref to the result `glib::List` of this
function..

When `func` is `None`, `RTSPFilterResult::Ref` will be assumed for each client.
## `func`
a callback
## `user_data`
user data passed to `func`

# Returns

a `glib::List` with all
clients for which `func` returned `RTSPFilterResult::Ref`. After usage, each
element in the `glib::List` should be unreffed before the list is freed.
<!-- trait RTSPServerExt::fn create_socket -->
Create a `gio::Socket` for `self`. The socket will listen on the
configured service.
## `cancellable`
a `gio::Cancellable`

# Returns

the `gio::Socket` for `self` or `None` when an error
occurred.
<!-- trait RTSPServerExt::fn create_source -->
Create a `glib::Source` for `self`. The new source will have a default
`GSocketSourceFunc` of `RTSPServer::io_func`.

`cancellable` if not `None` can be used to cancel the source, which will cause
the source to trigger, reporting the current condition (which is likely 0
unless cancellation happened at the same time as a condition change). You can
check for this in the callback using `gio::Cancellable::is_cancelled`.

This takes a reference on `self` until `source` is destroyed.
## `cancellable`
a `gio::Cancellable` or `None`.

# Returns

the `glib::Source` for `self` or `None` when an error
occurred. Free with g_source_unref ()
<!-- trait RTSPServerExt::fn get_address -->
Get the address on which the server will accept connections.

# Returns

the server address. `g_free` after usage.
<!-- trait RTSPServerExt::fn get_auth -->
Get the `RTSPAuth` used as the authentication manager of `self`.

# Returns

the `RTSPAuth` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPServerExt::fn get_backlog -->
The maximum amount of queued requests for the server.

# Returns

the server backlog.
<!-- trait RTSPServerExt::fn get_bound_port -->
Get the port number where the server was bound to.

# Returns

the port number
<!-- trait RTSPServerExt::fn get_mount_points -->
Get the `RTSPMountPoints` used as the mount points of `self`.

# Returns

the `RTSPMountPoints` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPServerExt::fn get_service -->
Get the service on which the server will accept connections.

# Returns

the service. use `g_free` after usage.
<!-- trait RTSPServerExt::fn get_session_pool -->
Get the `RTSPSessionPool` used as the session pool of `self`.

# Returns

the `RTSPSessionPool` used for sessions. `gobject::Object::unref` after
usage.
<!-- trait RTSPServerExt::fn get_thread_pool -->
Get the `RTSPThreadPool` used as the thread pool of `self`.

# Returns

the `RTSPThreadPool` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPServerExt::fn set_address -->
Configure `self` to accept connections on the given address.

This function must be called before the server is bound.
## `address`
the address
<!-- trait RTSPServerExt::fn set_auth -->
configure `auth` to be used as the authentication manager of `self`.
## `auth`
a `RTSPAuth`
<!-- trait RTSPServerExt::fn set_backlog -->
configure the maximum amount of requests that may be queued for the
server.

This function must be called before the server is bound.
## `backlog`
the backlog
<!-- trait RTSPServerExt::fn set_mount_points -->
configure `mounts` to be used as the mount points of `self`.
## `mounts`
a `RTSPMountPoints`
<!-- trait RTSPServerExt::fn set_service -->
Configure `self` to accept connections on the given service.
`service` should be a string containing the service name (see services(5)) or
a string containing a port number between 1 and 65535.

When `service` is set to "0", the server will listen on a random free
port. The actual used port can be retrieved with
`RTSPServerExt::get_bound_port`.

This function must be called before the server is bound.
## `service`
the service
<!-- trait RTSPServerExt::fn set_session_pool -->
configure `pool` to be used as the session pool of `self`.
## `pool`
a `RTSPSessionPool`
<!-- trait RTSPServerExt::fn set_thread_pool -->
configure `pool` to be used as the thread pool of `self`.
## `pool`
a `RTSPThreadPool`
<!-- trait RTSPServerExt::fn transfer_connection -->
Take an existing network socket and use it for an RTSP connection. This
is used when transferring a socket from an HTTP server which should be used
as an RTSP over HTTP tunnel. The `initial_buffer` contains any remaining data
that the HTTP server read from the socket while parsing the HTTP header.
## `socket`
a network socket
## `ip`
the IP address of the remote client
## `port`
the port used by the other end
## `initial_buffer`
any initial data that was already read from the socket

# Returns

TRUE if all was ok, FALSE if an error occurred.
<!-- struct RTSPSession -->
Session information kept by the server for a specific client.
One client session, identified with a session id, can handle multiple medias
identified with the url of a media.

# Implements

[`RTSPSessionExt`](trait.RTSPSessionExt.html)
<!-- trait RTSPSessionExt -->
Trait containing all `RTSPSession` methods.

# Implementors

[`RTSPSession`](struct.RTSPSession.html)
<!-- impl RTSPSession::fn new -->
Create a new `RTSPSession` instance with `sessionid`.
## `sessionid`
a session id

# Returns

a new `RTSPSession`
<!-- trait RTSPSessionExt::fn allow_expire -->
Allow `self` to expire. This method must be called an equal
amount of time as `RTSPSessionExt::prevent_expire`.
<!-- trait RTSPSessionExt::fn filter -->
Call `func` for each media in `self`. The result value of `func` determines
what happens to the media. `func` will be called with `self`
locked so no further actions on `self` can be performed from `func`.

If `func` returns `RTSPFilterResult::Remove`, the media will be removed from
`self`.

If `func` returns `RTSPFilterResult::Keep`, the media will remain in `self`.

If `func` returns `RTSPFilterResult::Ref`, the media will remain in `self` but
will also be added with an additional ref to the result `glib::List` of this
function..

When `func` is `None`, `RTSPFilterResult::Ref` will be assumed for all media.
## `func`
a callback
## `user_data`
user data passed to `func`

# Returns

a GList with all
media for which `func` returned `RTSPFilterResult::Ref`. After usage, each
element in the `glib::List` should be unreffed before the list is freed.
<!-- trait RTSPSessionExt::fn get_header -->
Get the string that can be placed in the Session header field.

# Returns

the Session header of `self`. `g_free` after usage.
<!-- trait RTSPSessionExt::fn get_media -->
Get the session media for `path`. `matched` will contain the number of matched
characters of `path`.
## `path`
the path for the media
## `matched`
the amount of matched characters

# Returns

the configuration for `path` in `self`.
<!-- trait RTSPSessionExt::fn get_sessionid -->
Get the sessionid of `self`.

# Returns

the sessionid of `self`. The value remains valid
as long as `self` is alive.
<!-- trait RTSPSessionExt::fn get_timeout -->
Get the timeout value of `self`.

# Returns

the timeout of `self` in seconds.
<!-- trait RTSPSessionExt::fn is_expired -->
Check if `self` timeout out.

# Deprecated

Use `RTSPSessionExt::is_expired_usec` instead.
## `now`
the current system time

# Returns

`true` if `self` timed out
<!-- trait RTSPSessionExt::fn is_expired_usec -->
Check if `self` timeout out.
## `now`
the current monotonic time

# Returns

`true` if `self` timed out
<!-- trait RTSPSessionExt::fn manage_media -->
Manage the media object `obj` in `self`. `path` will be used to retrieve this
media from the session with `RTSPSessionExt::get_media`.

Ownership is taken from `media`.
## `path`
the path for the media
## `media`
a `RTSPMedia`

# Returns

a new `RTSPSessionMedia` object.
<!-- trait RTSPSessionExt::fn next_timeout -->
Get the amount of milliseconds till the session will expire.

# Deprecated

Use `RTSPSessionExt::next_timeout_usec` instead.
## `now`
the current system time

# Returns

the amount of milliseconds since the session will time out.
<!-- trait RTSPSessionExt::fn next_timeout_usec -->
Get the amount of milliseconds till the session will expire.
## `now`
the current monotonic time

# Returns

the amount of milliseconds since the session will time out.
<!-- trait RTSPSessionExt::fn prevent_expire -->
Prevent `self` from expiring.
<!-- trait RTSPSessionExt::fn release_media -->
Release the managed `media` in `self`, freeing the memory allocated by it.
## `media`
a `RTSPMedia`

# Returns

`true` if there are more media session left in `self`.
<!-- trait RTSPSessionExt::fn set_timeout -->
Configure `self` for a timeout of `timeout` seconds. The session will be
cleaned up when there is no activity for `timeout` seconds.
## `timeout`
the new timeout
<!-- trait RTSPSessionExt::fn touch -->
Update the last_access time of the session to the current time.
<!-- struct RTSPSessionMedia -->
State of a client session regarding a specific media identified by path.

# Implements

[`RTSPSessionMediaExt`](trait.RTSPSessionMediaExt.html)
<!-- trait RTSPSessionMediaExt -->
Trait containing all `RTSPSessionMedia` methods.

# Implementors

[`RTSPSessionMedia`](struct.RTSPSessionMedia.html)
<!-- impl RTSPSessionMedia::fn new -->
Create a new `RTSPSessionMedia` that manages the streams
in `media` for `path`. `media` should be prepared.

Ownership is taken of `media`.
## `path`
the path
## `media`
the `RTSPMedia`

# Returns

a new `RTSPSessionMedia`.
<!-- trait RTSPSessionMediaExt::fn alloc_channels -->
Fill `range` with the next available min and max channels for
interleaved transport.
## `range`
a `gst_rtsp::RTSPRange`

# Returns

`true` on success.
<!-- trait RTSPSessionMediaExt::fn get_base_time -->
Get the base_time of the `RTSPMedia` in `self`

# Returns

the base_time of the media.
<!-- trait RTSPSessionMediaExt::fn get_media -->
Get the `RTSPMedia` that was used when constructing `self`

# Returns

the `RTSPMedia` of `self`. Remains valid as long
as `self` is valid.
<!-- trait RTSPSessionMediaExt::fn get_rtpinfo -->
Retrieve the RTP-Info header string for all streams in `self`
with configured transports.

# Returns

The RTP-Info as a string or
`None` when no RTP-Info could be generated, `g_free` after usage.
<!-- trait RTSPSessionMediaExt::fn get_rtsp_state -->
Get the current RTSP state of `self`.

# Returns

the current RTSP state of `self`.
<!-- trait RTSPSessionMediaExt::fn get_transport -->
Get a previously created `RTSPStreamTransport` for the stream at `idx`.
## `idx`
the stream index

# Returns

a `RTSPStreamTransport` that is valid until the
session of `self` is unreffed.
<!-- trait RTSPSessionMediaExt::fn matches -->
Check if the path of `self` matches `path`. It `path` matches, the amount of
matched characters is returned in `matched`.
## `path`
a path
## `matched`
the amount of matched characters of `path`

# Returns

`true` when `path` matches the path of `self`.
<!-- trait RTSPSessionMediaExt::fn set_rtsp_state -->
Set the RTSP state of `self` to `state`.
## `state`
a `gst_rtsp::RTSPState`
<!-- trait RTSPSessionMediaExt::fn set_state -->
Tell the media object `self` to change to `state`.
## `state`
the new state

# Returns

`true` on success.
<!-- trait RTSPSessionMediaExt::fn set_transport -->
Configure the transport for `stream` to `tr` in `self`.
## `stream`
a `RTSPStream`
## `tr`
a `gst_rtsp::RTSPTransport`

# Returns

the new or updated `RTSPStreamTransport` for `stream`.
<!-- struct RTSPSessionPool -->
An object that keeps track of the active sessions. This object is usually
attached to a `RTSPServer` object to manage the sessions in that server.

# Implements

[`RTSPSessionPoolExt`](trait.RTSPSessionPoolExt.html)
<!-- trait RTSPSessionPoolExt -->
Trait containing all `RTSPSessionPool` methods.

# Implementors

[`RTSPSessionPool`](struct.RTSPSessionPool.html)
<!-- impl RTSPSessionPool::fn new -->
Create a new `RTSPSessionPool` instance.

# Returns

A new `RTSPSessionPool`. `gobject::Object::unref` after
usage.
<!-- trait RTSPSessionPoolExt::fn cleanup -->
Inspect all the sessions in `self` and remove the sessions that are inactive
for more than their timeout.

# Returns

the amount of sessions that got removed.
<!-- trait RTSPSessionPoolExt::fn create -->
Create a new `RTSPSession` object in `self`.

# Returns

a new `RTSPSession`.
<!-- trait RTSPSessionPoolExt::fn create_watch -->
Create a `glib::Source` that will be dispatched when the session should be cleaned
up.

# Returns

a `glib::Source`
<!-- trait RTSPSessionPoolExt::fn filter -->
Call `func` for each session in `self`. The result value of `func` determines
what happens to the session. `func` will be called with the session pool
locked so no further actions on `self` can be performed from `func`.

If `func` returns `RTSPFilterResult::Remove`, the session will be set to the
expired state with `gst_rtsp_session_set_expired` and removed from
`self`.

If `func` returns `RTSPFilterResult::Keep`, the session will remain in `self`.

If `func` returns `RTSPFilterResult::Ref`, the session will remain in `self` but
will also be added with an additional ref to the result GList of this
function..

When `func` is `None`, `RTSPFilterResult::Ref` will be assumed for all sessions.
## `func`
a callback
## `user_data`
user data passed to `func`

# Returns

a GList with all
sessions for which `func` returned `RTSPFilterResult::Ref`. After usage, each
element in the GList should be unreffed before the list is freed.
<!-- trait RTSPSessionPoolExt::fn find -->
Find the session with `sessionid` in `self`. The access time of the session
will be updated with `RTSPSessionExt::touch`.
## `sessionid`
the session id

# Returns

the `RTSPSession` with `sessionid`
or `None` when the session did not exist. `gobject::Object::unref` after usage.
<!-- trait RTSPSessionPoolExt::fn get_max_sessions -->
Get the maximum allowed number of sessions in `self`. 0 means an unlimited
amount of sessions.

# Returns

the maximum allowed number of sessions.
<!-- trait RTSPSessionPoolExt::fn get_n_sessions -->
Get the amount of active sessions in `self`.

# Returns

the amount of active sessions in `self`.
<!-- trait RTSPSessionPoolExt::fn remove -->
Remove `sess` from `self`, releasing the ref that the pool has on `sess`.
## `sess`
a `RTSPSession`

# Returns

`true` if the session was found and removed.
<!-- trait RTSPSessionPoolExt::fn set_max_sessions -->
Configure the maximum allowed number of sessions in `self` to `max`.
A value of 0 means an unlimited amount of sessions.
## `max`
the maximum number of sessions
<!-- struct RTSPStream -->
The definition of a media stream.

# Implements

[`RTSPStreamExt`](trait.RTSPStreamExt.html)
<!-- trait RTSPStreamExt -->
Trait containing all `RTSPStream` methods.

# Implementors

[`RTSPStream`](struct.RTSPStream.html)
<!-- impl RTSPStream::fn new -->
Create a new media stream with index `idx` that handles RTP data on
`pad` and has a payloader element `payloader` if `pad` is a source pad
or a depayloader element `payloader` if `pad` is a sink pad.
## `idx`
an index
## `payloader`
a `gst::Element`
## `pad`
a `gst::Pad`

# Returns

a new `RTSPStream`
<!-- trait RTSPStreamExt::fn add_transport -->
Add the transport in `trans` to `self`. The media of `self` will
then also be send to the values configured in `trans`.

`self` must be joined to a bin.

`trans` must contain a valid `gst_rtsp::RTSPTransport`.
## `trans`
a `RTSPStreamTransport`

# Returns

`true` if `trans` was added
<!-- trait RTSPStreamExt::fn allocate_udp_sockets -->
Allocates RTP and RTCP ports.

# Deprecated

This function shouldn't have been made public
## `family`
protocol family
## `transport`
transport method
## `use_client_setttings`
Whether to use client settings or not

# Returns

`true` if the RTP and RTCP sockets have been succeccully allocated.
<!-- trait RTSPStreamExt::fn get_address_pool -->
Get the `RTSPAddressPool` used as the address pool of `self`.

# Returns

the `RTSPAddressPool` of `self`. `gobject::Object::unref` after
usage.
<!-- trait RTSPStreamExt::fn get_buffer_size -->
Get the size of the UDP transmission buffer (in bytes)

# Returns

the size of the UDP TX buffer
<!-- trait RTSPStreamExt::fn get_caps -->
Retrieve the current caps of `self`.

# Returns

the `gst::Caps` of `self`. use `gst_caps_unref`
after usage.
<!-- trait RTSPStreamExt::fn get_control -->
Get the control string to identify this stream.

# Returns

the control string. `g_free` after usage.
<!-- trait RTSPStreamExt::fn get_dscp_qos -->
Get the configured DSCP QoS in of the outgoing sockets.

# Returns

the DSCP QoS value of the outgoing sockets, or -1 if disbled.
<!-- trait RTSPStreamExt::fn get_index -->
Get the stream index.

# Returns

the stream index.
<!-- trait RTSPStreamExt::fn get_joined_bin -->
Get the previous joined bin with `RTSPStreamExt::join_bin` or NULL.

# Returns

the joined bin or NULL.
<!-- trait RTSPStreamExt::fn get_mtu -->
Get the configured MTU in the payloader of `self`.

# Returns

the MTU of the payloader.
<!-- trait RTSPStreamExt::fn get_multicast_address -->
Get the multicast address of `self` for `family`. The original
`RTSPAddress` is cached and copy is returned, so freeing the return value
won't release the address from the pool.
## `family`
the `gio::SocketFamily`

# Returns

the `RTSPAddress` of `self`
or `None` when no address could be allocated. `RTSPAddress::free`
after usage.
<!-- trait RTSPStreamExt::fn get_multicast_iface -->
Get the multicast interface used for `self`.

# Returns

the multicast interface for `self`. `g_free` after
usage.
<!-- trait RTSPStreamExt::fn get_profiles -->
Get the allowed profiles of `self`.

# Returns

a `gst_rtsp::RTSPProfile`
<!-- trait RTSPStreamExt::fn get_protocols -->
Get the allowed protocols of `self`.

# Returns

a `gst_rtsp::RTSPLowerTrans`
<!-- trait RTSPStreamExt::fn get_pt -->
Get the stream payload type.

# Returns

the stream payload type.
<!-- trait RTSPStreamExt::fn get_publish_clock_mode -->
Gets if and how the stream clock should be published according to RFC7273.

# Returns

The GstRTSPPublishClockMode
<!-- trait RTSPStreamExt::fn get_retransmission_pt -->
Get the payload-type used for retransmission of this stream

# Returns

The retransmission PT.
<!-- trait RTSPStreamExt::fn get_retransmission_time -->
Get the amount of time to store retransmission data.

# Returns

the amount of time to store retransmission data.
<!-- trait RTSPStreamExt::fn get_rtcp_socket -->
Get the RTCP socket from `self` for a `family`.

`self` must be joined to a bin.
## `family`
the socket family

# Returns

the RTCP socket or `None` if no
socket could be allocated for `family`. Unref after usage
<!-- trait RTSPStreamExt::fn get_rtp_socket -->
Get the RTP socket from `self` for a `family`.

`self` must be joined to a bin.
## `family`
the socket family

# Returns

the RTP socket or `None` if no
socket could be allocated for `family`. Unref after usage
<!-- trait RTSPStreamExt::fn get_rtpinfo -->
Retrieve the current rtptime, seq and running-time. This is used to
construct a RTPInfo reply header.
## `rtptime`
result RTP timestamp
## `seq`
result RTP seqnum
## `clock_rate`
the clock rate
## `running_time`
result running-time

# Returns

`true` when rtptime, seq and running-time could be determined.
<!-- trait RTSPStreamExt::fn get_rtpsession -->
Get the RTP session of this stream.

# Returns

The RTP session of this stream. Unref after usage.
<!-- trait RTSPStreamExt::fn get_server_port -->
Fill `server_port` with the port pair used by the server. This function can
only be called when `self` has been joined.
## `server_port`
result server port
## `family`
the port family to get
<!-- trait RTSPStreamExt::fn get_sinkpad -->
Get the sinkpad associated with `self`.

# Returns

the sinkpad. Unref after usage.
<!-- trait RTSPStreamExt::fn get_srcpad -->
Get the srcpad associated with `self`.

# Returns

the srcpad. Unref after usage.
<!-- trait RTSPStreamExt::fn get_srtp_encoder -->
Get the SRTP encoder for this stream.

# Returns

The SRTP encoder for this stream. Unref after usage.
<!-- trait RTSPStreamExt::fn get_ssrc -->
Get the SSRC used by the RTP session of this stream. This function can only
be called when `self` has been joined.
## `ssrc`
result ssrc
<!-- trait RTSPStreamExt::fn has_control -->
Check if `self` has the control string `control`.
## `control`
a control string

# Returns

`true` is `self` has `control` as the control string
<!-- trait RTSPStreamExt::fn is_blocking -->
Check if `self` is blocking on a `gst::Buffer`.

# Returns

`true` if `self` is blocking
<!-- trait RTSPStreamExt::fn is_client_side -->
See `RTSPStreamExt::set_client_side`

# Returns

TRUE if this `RTSPStream` is client-side.
<!-- trait RTSPStreamExt::fn is_transport_supported -->
Check if `transport` can be handled by stream
## `transport`
a `gst_rtsp::RTSPTransport`

# Returns

`true` if `transport` can be handled by `self`.
<!-- trait RTSPStreamExt::fn join_bin -->
Join the `gst::Bin` `bin` that contains the element `rtpbin`.

`self` will link to `rtpbin`, which must be inside `bin`. The elements
added to `bin` will be set to the state given in `state`.
## `bin`
a `gst::Bin` to join
## `rtpbin`
a rtpbin element in `bin`
## `state`
the target state of the new elements

# Returns

`true` on success.
<!-- trait RTSPStreamExt::fn leave_bin -->
Remove the elements of `self` from `bin`.
## `bin`
a `gst::Bin`
## `rtpbin`
a rtpbin `gst::Element`

# Returns

`true` on success.
<!-- trait RTSPStreamExt::fn query_position -->
Query the position of the stream in `gst::Format::Time`. This only considers
the RTP parts of the pipeline and not the RTCP parts.

# Returns

`true` if the position could be queried
<!-- trait RTSPStreamExt::fn query_stop -->
Query the stop of the stream in `gst::Format::Time`. This only considers
the RTP parts of the pipeline and not the RTCP parts.

# Returns

`true` if the stop could be queried
<!-- trait RTSPStreamExt::fn recv_rtcp -->
Handle an RTCP buffer for the stream. This method is usually called when a
message has been received from a client using the TCP transport.

This function takes ownership of `buffer`.
## `buffer`
a `gst::Buffer`

# Returns

a GstFlowReturn.
<!-- trait RTSPStreamExt::fn recv_rtp -->
Handle an RTP buffer for the stream. This method is usually called when a
message has been received from a client using the TCP transport.

This function takes ownership of `buffer`.
## `buffer`
a `gst::Buffer`

# Returns

a GstFlowReturn.
<!-- trait RTSPStreamExt::fn remove_transport -->
Remove the transport in `trans` from `self`. The media of `self` will
not be sent to the values configured in `trans`.

`self` must be joined to a bin.

`trans` must contain a valid `gst_rtsp::RTSPTransport`.
## `trans`
a `RTSPStreamTransport`

# Returns

`true` if `trans` was removed
<!-- trait RTSPStreamExt::fn request_aux_sender -->
Creating a rtxsend bin
## `sessid`
the session id

# Returns

a `gst::Element`.
<!-- trait RTSPStreamExt::fn reserve_address -->
Reserve `address` and `port` as the address and port of `self`. The original
`RTSPAddress` is cached and copy is returned, so freeing the return value
won't release the address from the pool.
## `address`
an address
## `port`
a port
## `n_ports`
n_ports
## `ttl`
a TTL

# Returns

the `RTSPAddress` of `self` or `None` when
the address could be reserved. `RTSPAddress::free` after usage.
<!-- trait RTSPStreamExt::fn set_address_pool -->
configure `pool` to be used as the address pool of `self`.
## `pool`
a `RTSPAddressPool`
<!-- trait RTSPStreamExt::fn set_blocked -->
Blocks or unblocks the dataflow on `self`.
## `blocked`
boolean indicating we should block or unblock

# Returns

`true` on success
<!-- trait RTSPStreamExt::fn set_buffer_size -->
Set the size of the UDP transmission buffer (in bytes)
Needs to be set before the stream is joined to a bin.
## `size`
the buffer size
<!-- trait RTSPStreamExt::fn set_client_side -->
Sets the `RTSPStream` as a 'client side' stream - used for sending
streams to an RTSP server via RECORD. This has the practical effect
of changing which UDP port numbers are used when setting up the local
side of the stream sending to be either the 'server' or 'client' pair
of a configured UDP transport.
## `client_side`
TRUE if this `RTSPStream` is running on the 'client' side of
an RTSP connection.
<!-- trait RTSPStreamExt::fn set_control -->
Set the control string in `self`.
## `control`
a control string
<!-- trait RTSPStreamExt::fn set_dscp_qos -->
Configure the dscp qos of the outgoing sockets to `dscp_qos`.
## `dscp_qos`
a new dscp qos value (0-63, or -1 to disable)
<!-- trait RTSPStreamExt::fn set_mtu -->
Configure the mtu in the payloader of `self` to `mtu`.
## `mtu`
a new MTU
<!-- trait RTSPStreamExt::fn set_multicast_iface -->
configure `multicast_iface` to be used for `self`.
## `multicast_iface`
a multicast interface name
<!-- trait RTSPStreamExt::fn set_profiles -->
Configure the allowed profiles for `self`.
## `profiles`
the new profiles
<!-- trait RTSPStreamExt::fn set_protocols -->
Configure the allowed lower transport for `self`.
## `protocols`
the new flags
<!-- trait RTSPStreamExt::fn set_pt_map -->
Configure a pt map between `pt` and `caps`.
## `pt`
the pt
## `caps`
a `gst::Caps`
<!-- trait RTSPStreamExt::fn set_publish_clock_mode -->
Sets if and how the stream clock should be published according to RFC7273.
## `mode`
the clock publish mode
<!-- trait RTSPStreamExt::fn set_retransmission_pt -->
Set the payload type (pt) for retransmission of this stream.
## `rtx_pt`
a `guint`
<!-- trait RTSPStreamExt::fn set_retransmission_time -->
Set the amount of time to store retransmission packets.
## `time`
a `gst::ClockTime`
<!-- trait RTSPStreamExt::fn transport_filter -->
Call `func` for each transport managed by `self`. The result value of `func`
determines what happens to the transport. `func` will be called with `self`
locked so no further actions on `self` can be performed from `func`.

If `func` returns `RTSPFilterResult::Remove`, the transport will be removed from
`self`.

If `func` returns `RTSPFilterResult::Keep`, the transport will remain in `self`.

If `func` returns `RTSPFilterResult::Ref`, the transport will remain in `self` but
will also be added with an additional ref to the result `glib::List` of this
function..

When `func` is `None`, `RTSPFilterResult::Ref` will be assumed for each transport.
## `func`
a callback
## `user_data`
user data passed to `func`

# Returns

a `glib::List` with all
transports for which `func` returned `RTSPFilterResult::Ref`. After usage, each
element in the `glib::List` should be unreffed before the list is freed.
<!-- trait RTSPStreamExt::fn update_crypto -->
Update the new crypto information for `ssrc` in `self`. If information
for `ssrc` did not exist, it will be added. If information
for `ssrc` existed, it will be replaced. If `crypto` is `None`, it will
be removed from `self`.
## `ssrc`
the SSRC
## `crypto`
a `gst::Caps` with crypto info

# Returns

`true` if `crypto` could be updated
<!-- struct RTSPStreamTransport -->
A Transport description for a stream

# Implements

[`RTSPStreamTransportExt`](trait.RTSPStreamTransportExt.html)
<!-- trait RTSPStreamTransportExt -->
Trait containing all `RTSPStreamTransport` methods.

# Implementors

[`RTSPStreamTransport`](struct.RTSPStreamTransport.html)
<!-- impl RTSPStreamTransport::fn new -->
Create a new `RTSPStreamTransport` that can be used to manage
`stream` with transport `tr`.
## `stream`
a `RTSPStream`
## `tr`
a GstRTSPTransport

# Returns

a new `RTSPStreamTransport`
<!-- trait RTSPStreamTransportExt::fn get_rtpinfo -->
Get the RTP-Info string for `self` and `start_time`.
## `start_time`
a star time

# Returns

the RTPInfo string for `self`
and `start_time` or `None` when the RTP-Info could not be
determined. `g_free` after usage.
<!-- trait RTSPStreamTransportExt::fn get_stream -->
Get the `RTSPStream` used when constructing `self`.

# Returns

the stream used when constructing `self`.
<!-- trait RTSPStreamTransportExt::fn get_transport -->
Get the transport configured in `self`.

# Returns

the transport configured in `self`. It remains
valid for as long as `self` is valid.
<!-- trait RTSPStreamTransportExt::fn get_url -->
Get the url configured in `self`.

# Returns

the url configured in `self`. It remains
valid for as long as `self` is valid.
<!-- trait RTSPStreamTransportExt::fn is_timed_out -->
Check if `self` is timed out.

# Returns

`true` if `self` timed out.
<!-- trait RTSPStreamTransportExt::fn keep_alive -->
Signal the installed keep_alive callback for `self`.
<!-- trait RTSPStreamTransportExt::fn recv_data -->
Receive `buffer` on `channel` `self`.
## `channel`
a channel
## `buffer`
a `gst::Buffer`

# Returns

a `gst::FlowReturn`. Returns GST_FLOW_NOT_LINKED when `channel` is not
 configured in the transport of `self`.
<!-- trait RTSPStreamTransportExt::fn send_rtcp -->
Send `buffer` to the installed RTCP callback for `self`.
## `buffer`
a `gst::Buffer`

# Returns

`true` on success
<!-- trait RTSPStreamTransportExt::fn send_rtp -->
Send `buffer` to the installed RTP callback for `self`.
## `buffer`
a `gst::Buffer`

# Returns

`true` on success
<!-- trait RTSPStreamTransportExt::fn set_active -->
Activate or deactivate datatransfer configured in `self`.
## `active`
new state of `self`

# Returns

`true` when the state was changed.
<!-- trait RTSPStreamTransportExt::fn set_callbacks -->
Install callbacks that will be called when data for a stream should be sent
to a client. This is usually used when sending RTP/RTCP over TCP.
## `send_rtp`
a callback called when RTP should be sent
## `send_rtcp`
a callback called when RTCP should be sent
## `user_data`
user data passed to callbacks
## `notify`
called with the user_data when no longer needed.
<!-- trait RTSPStreamTransportExt::fn set_keepalive -->
Install callbacks that will be called when RTCP packets are received from the
receiver of `self`.
## `keep_alive`
a callback called when the receiver is active
## `user_data`
user data passed to callback
## `notify`
called with the user_data when no longer needed.
<!-- trait RTSPStreamTransportExt::fn set_timed_out -->
Set the timed out state of `self` to `timedout`
## `timedout`
timed out value
<!-- trait RTSPStreamTransportExt::fn set_transport -->
Set `tr` as the client transport. This function takes ownership of the
passed `tr`.
## `tr`
a client `gst_rtsp::RTSPTransport`
<!-- trait RTSPStreamTransportExt::fn set_url -->
Set `url` as the client url.
## `url`
a client `gst_rtsp::RTSPUrl`
<!-- enum RTSPSuspendMode -->
The suspend mode of the media pipeline. A media pipeline is suspended right
after creating the SDP and when the client performs a PAUSED request.
<!-- enum RTSPSuspendMode::variant None -->
Media is not suspended
<!-- enum RTSPSuspendMode::variant Pause -->
Media is PAUSED in suspend
<!-- enum RTSPSuspendMode::variant Reset -->
The media is set to NULL when suspended
<!-- struct RTSPThreadPool -->
The thread pool structure.

# Implements

[`RTSPThreadPoolExt`](trait.RTSPThreadPoolExt.html)
<!-- trait RTSPThreadPoolExt -->
Trait containing all `RTSPThreadPool` methods.

# Implementors

[`RTSPThreadPool`](struct.RTSPThreadPool.html)
<!-- impl RTSPThreadPool::fn new -->
Create a new `RTSPThreadPool` instance.

# Returns

a new `RTSPThreadPool`
<!-- impl RTSPThreadPool::fn cleanup -->
Wait for all tasks to be stopped and free all allocated resources. This is
mainly used in test suites to ensure proper cleanup of internal data
structures.
<!-- trait RTSPThreadPoolExt::fn get_max_threads -->
Get the maximum number of threads used for client connections.
See `RTSPThreadPoolExt::set_max_threads`.

# Returns

the maximum number of threads.
<!-- trait RTSPThreadPoolExt::fn get_thread -->
Get a new `RTSPThread` for `type_` and `ctx`.
## `type_`
the `RTSPThreadType`
## `ctx`
a `RTSPContext`

# Returns

a new `RTSPThread`, `RTSPThread::stop` after usage
<!-- trait RTSPThreadPoolExt::fn set_max_threads -->
Set the maximum threads used by the pool to handle client requests.
A value of 0 will use the pool mainloop, a value of -1 will use an
unlimited number of threads.
## `max_threads`
maximum threads
<!-- enum RTSPThreadType -->
Different thread types
<!-- enum RTSPThreadType::variant Client -->
a thread to handle the client communication
<!-- enum RTSPThreadType::variant Media -->
a thread to handle media
