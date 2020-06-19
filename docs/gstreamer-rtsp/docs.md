<!-- file * -->
<!-- enum RTSPAuthMethod -->
Authentication methods, ordered by strength
<!-- enum RTSPAuthMethod::variant None -->
no authentication
<!-- enum RTSPAuthMethod::variant Basic -->
basic authentication
<!-- enum RTSPAuthMethod::variant Digest -->
digest authentication
<!-- struct RTSPAuthParam -->
RTSP Authentication parameter

Feature: `v1_12`
<!-- enum RTSPFamily -->
The possible network families.
<!-- enum RTSPFamily::variant None -->
unknown network family
<!-- enum RTSPFamily::variant Inet -->
internet
<!-- enum RTSPFamily::variant Inet6 -->
internet V6
<!-- enum RTSPHeaderField -->
Enumeration of rtsp header fields
<!-- enum RTSPMsgType -->
The type of a message.
<!-- enum RTSPMsgType::variant Invalid -->
invalid message type
<!-- enum RTSPMsgType::variant Request -->
RTSP request message
<!-- enum RTSPMsgType::variant Response -->
RTSP response message
<!-- enum RTSPMsgType::variant HttpRequest -->
HTTP request message.
<!-- enum RTSPMsgType::variant HttpResponse -->
HTTP response message.
<!-- enum RTSPMsgType::variant Data -->
data message
<!-- enum RTSPRangeUnit -->
Different possible time range units.
<!-- enum RTSPRangeUnit::variant Smpte -->
SMPTE timecode
<!-- enum RTSPRangeUnit::variant Smpte30Drop -->
29.97 frames per second
<!-- enum RTSPRangeUnit::variant Smpte25 -->
25 frames per second
<!-- enum RTSPRangeUnit::variant Npt -->
Normal play time
<!-- enum RTSPRangeUnit::variant Clock -->
Absolute time expressed as ISO 8601 timestamps
<!-- enum RTSPResult -->
Result codes from the RTSP functions.
<!-- enum RTSPResult::variant Ok -->
no error
<!-- enum RTSPResult::variant Error -->
some unspecified error occurred
<!-- enum RTSPResult::variant Einval -->
invalid arguments were provided to a function
<!-- enum RTSPResult::variant Eintr -->
an operation was canceled
<!-- enum RTSPResult::variant Enomem -->
no memory was available for the operation
<!-- enum RTSPResult::variant Eresolv -->
a host resolve error occurred
<!-- enum RTSPResult::variant Enotimpl -->
function not implemented
<!-- enum RTSPResult::variant Esys -->
a system error occurred, errno contains more details
<!-- enum RTSPResult::variant Eparse -->
a parsing error occurred
<!-- enum RTSPResult::variant Ewsastart -->
windows networking could not start
<!-- enum RTSPResult::variant Ewsaversion -->
windows networking stack has wrong version
<!-- enum RTSPResult::variant Eeof -->
end-of-file was reached
<!-- enum RTSPResult::variant Enet -->
a network problem occurred, h_errno contains more details
<!-- enum RTSPResult::variant Enotip -->
the host is not an IP host
<!-- enum RTSPResult::variant Etimeout -->
a timeout occurred
<!-- enum RTSPResult::variant Etget -->
the tunnel GET request has been performed
<!-- enum RTSPResult::variant Etpost -->
the tunnel POST request has been performed
<!-- enum RTSPResult::variant Elast -->
last error
<!-- enum RTSPState -->
The different RTSP states.
<!-- enum RTSPState::variant Invalid -->
invalid state
<!-- enum RTSPState::variant Init -->
initializing
<!-- enum RTSPState::variant Ready -->
ready for operation
<!-- enum RTSPState::variant Seeking -->
seeking in progress
<!-- enum RTSPState::variant Playing -->
playing
<!-- enum RTSPState::variant Recording -->
recording
<!-- enum RTSPStatusCode -->
Enumeration of rtsp status codes
<!-- enum RTSPTimeType -->
Possible time types.
<!-- enum RTSPTimeType::variant Seconds -->
seconds
<!-- enum RTSPTimeType::variant Now -->
now
<!-- enum RTSPTimeType::variant End -->
end
<!-- enum RTSPTimeType::variant Frames -->
frames and subframes
<!-- enum RTSPTimeType::variant Utc -->
UTC time
<!-- struct RTSPUrl -->
Provides helper functions to handle RTSP urls.
<!-- impl RTSPUrl::fn copy -->
Make a copy of `self`.

# Returns

a copy of `self`. Free with gst_rtsp_url_free () after usage.
<!-- impl RTSPUrl::fn decode_path_components -->
Splits the path of `self` on '/' boundaries, decoding the resulting components,

The decoding performed by this routine is "URI decoding", as defined in RFC
3986, commonly known as percent-decoding. For example, a string "foo\%2fbar"
will decode to "foo/bar" -- the \%2f being replaced by the corresponding byte
with hex value 0x2f. Note that there is no guarantee that the resulting byte
sequence is valid in any given encoding. As a special case, \%00 is not
unescaped to NUL, as that would prematurely terminate the string.

Also note that since paths usually start with a slash, the first component
will usually be the empty string.

# Returns

`None`-terminated array of URL components. Free with
`g_strfreev` when no longer needed.
<!-- impl RTSPUrl::fn free -->
Free the memory used by `self`.
<!-- impl RTSPUrl::fn get_port -->
Get the port number of `self`.
## `port`
location to hold the port

# Returns

`RTSPResult::Ok`.
<!-- impl RTSPUrl::fn get_request_uri -->
Get a newly allocated string describing the request URI for `self`.

# Returns

a string with the request URI. `g_free` after usage.
<!-- impl RTSPUrl::fn get_request_uri_with_control -->
Get a newly allocated string describing the request URI for `self`
combined with the control path for `control_path`

Feature: `v1_18`

## `control_path`
an RTSP aggregate control path

# Returns

a string with the request URI combined with the control path.
`g_free` after usage.
<!-- impl RTSPUrl::fn set_port -->
Set the port number in `self` to `port`.
## `port`
the port

# Returns

`RTSPResult::Ok`.
<!-- impl RTSPUrl::fn parse -->
Parse the RTSP `urlstr` into a newly allocated `RTSPUrl`. Free after usage
with `RTSPUrl::free`.
## `urlstr`
the url string to parse
## `url`
location to hold the result.

# Returns

a `RTSPResult`.
