<!-- file * -->
<!-- enum WebRTCBundlePolicy -->
GST_WEBRTC_BUNDLE_POLICY_NONE: none
GST_WEBRTC_BUNDLE_POLICY_BALANCED: balanced
GST_WEBRTC_BUNDLE_POLICY_MAX_COMPAT: max-compat
GST_WEBRTC_BUNDLE_POLICY_MAX_BUNDLE: max-bundle
See https://tools.ietf.org/html/draft-ietf-rtcweb-jsep-24`section`-4.1.1
for more information.

Feature: `v1_16`

<!-- enum WebRTCDTLSSetup -->
<!-- enum WebRTCDTLSSetup::variant None -->
none
<!-- enum WebRTCDTLSSetup::variant Actpass -->
actpass
<!-- enum WebRTCDTLSSetup::variant Active -->
sendonly
<!-- enum WebRTCDTLSSetup::variant Passive -->
recvonly
<!-- struct WebRTCDTLSTransport -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- enum WebRTCDTLSTransportState -->
<!-- enum WebRTCDTLSTransportState::variant New -->
new
<!-- enum WebRTCDTLSTransportState::variant Closed -->
closed
<!-- enum WebRTCDTLSTransportState::variant Failed -->
failed
<!-- enum WebRTCDTLSTransportState::variant Connecting -->
connecting
<!-- enum WebRTCDTLSTransportState::variant Connected -->
connected
<!-- struct WebRTCDataChannel -->


Feature: `v1_18`

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl WebRTCDataChannel::fn close -->
Close the `self`.

Feature: `v1_18`

<!-- impl WebRTCDataChannel::fn on_buffered_amount_low -->
Signal that the data channel reached a low buffered amount. Should only be used by subclasses.

Feature: `v1_18`

<!-- impl WebRTCDataChannel::fn on_close -->
Signal that the data channel was closed. Should only be used by subclasses.

Feature: `v1_18`

<!-- impl WebRTCDataChannel::fn on_error -->
Signal that the data channel had an error. Should only be used by subclasses.

Feature: `v1_18`

## `error`
a `glib::Error`
<!-- impl WebRTCDataChannel::fn on_message_data -->
Signal that the data channel received a data message. Should only be used by subclasses.

Feature: `v1_18`

## `data`
a `glib::Bytes` or `None`
<!-- impl WebRTCDataChannel::fn on_message_string -->
Signal that the data channel received a string message. Should only be used by subclasses.

Feature: `v1_18`

## `str`
a string or `None`
<!-- impl WebRTCDataChannel::fn on_open -->
Signal that the data channel was opened. Should only be used by subclasses.

Feature: `v1_18`

<!-- impl WebRTCDataChannel::fn send_data -->
Send `data` as a data message over `self`.

Feature: `v1_18`

## `data`
a `glib::Bytes` or `None`
<!-- impl WebRTCDataChannel::fn send_string -->
Send `str` as a string message over `self`.

Feature: `v1_18`

## `str`
a string or `None`
<!-- impl WebRTCDataChannel::fn connect_close -->
Close the data channel
<!-- impl WebRTCDataChannel::fn connect_on_error -->
## `error`
the `glib::Error` thrown
<!-- impl WebRTCDataChannel::fn connect_on_message_data -->
## `data`
a `glib::Bytes` of the data received
<!-- impl WebRTCDataChannel::fn connect_on_message_string -->
## `data`
the data received as a string
<!-- impl WebRTCDataChannel::fn connect_send_data -->
## `data`
a `glib::Bytes` with the data
<!-- impl WebRTCDataChannel::fn connect_send_string -->
## `data`
the data to send as a string
<!-- enum WebRTCDataChannelState -->
GST_WEBRTC_DATA_CHANNEL_STATE_NEW: new
GST_WEBRTC_DATA_CHANNEL_STATE_CONNECTING: connection
GST_WEBRTC_DATA_CHANNEL_STATE_OPEN: open
GST_WEBRTC_DATA_CHANNEL_STATE_CLOSING: closing
GST_WEBRTC_DATA_CHANNEL_STATE_CLOSED: closed
See <http://w3c.github.io/webrtc-pc/`dom`-rtcdatachannelstate>

Feature: `v1_16`

<!-- enum WebRTCFECType -->
<!-- enum WebRTCFECType::variant None -->
none
<!-- enum WebRTCFECType::variant UlpRed -->
ulpfec + red

Feature: `v1_14_1`

<!-- enum WebRTCICEComponent -->
<!-- enum WebRTCICEComponent::variant Rtp -->
RTP component
<!-- enum WebRTCICEComponent::variant Rtcp -->
RTCP component
<!-- enum WebRTCICEConnectionState -->
See <http://w3c.github.io/webrtc-pc/`dom`-rtciceconnectionstate>
<!-- enum WebRTCICEConnectionState::variant New -->
new
<!-- enum WebRTCICEConnectionState::variant Checking -->
checking
<!-- enum WebRTCICEConnectionState::variant Connected -->
connected
<!-- enum WebRTCICEConnectionState::variant Completed -->
completed
<!-- enum WebRTCICEConnectionState::variant Failed -->
failed
<!-- enum WebRTCICEConnectionState::variant Disconnected -->
disconnected
<!-- enum WebRTCICEConnectionState::variant Closed -->
closed
<!-- enum WebRTCICEGatheringState -->
See <http://w3c.github.io/webrtc-pc/`dom`-rtcicegatheringstate>
<!-- enum WebRTCICEGatheringState::variant New -->
new
<!-- enum WebRTCICEGatheringState::variant Gathering -->
gathering
<!-- enum WebRTCICEGatheringState::variant Complete -->
complete
<!-- enum WebRTCICERole -->
<!-- enum WebRTCICERole::variant Controlled -->
controlled
<!-- enum WebRTCICERole::variant Controlling -->
controlling
<!-- struct WebRTCICETransport -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- enum WebRTCICETransportPolicy -->
GST_WEBRTC_ICE_TRANSPORT_POLICY_ALL: all
GST_WEBRTC_ICE_TRANSPORT_POLICY_RELAY: relay
See https://tools.ietf.org/html/draft-ietf-rtcweb-jsep-24`section`-4.1.1
for more information.

Feature: `v1_16`

<!-- enum WebRTCPeerConnectionState -->
See <http://w3c.github.io/webrtc-pc/`dom`-rtcpeerconnectionstate>
<!-- enum WebRTCPeerConnectionState::variant New -->
new
<!-- enum WebRTCPeerConnectionState::variant Connecting -->
connecting
<!-- enum WebRTCPeerConnectionState::variant Connected -->
connected
<!-- enum WebRTCPeerConnectionState::variant Disconnected -->
disconnected
<!-- enum WebRTCPeerConnectionState::variant Failed -->
failed
<!-- enum WebRTCPeerConnectionState::variant Closed -->
closed
<!-- enum WebRTCPriorityType -->
GST_WEBRTC_PRIORITY_TYPE_VERY_LOW: very-low
GST_WEBRTC_PRIORITY_TYPE_LOW: low
GST_WEBRTC_PRIORITY_TYPE_MEDIUM: medium
GST_WEBRTC_PRIORITY_TYPE_HIGH: high
See <http://w3c.github.io/webrtc-pc/`dom`-rtcprioritytype>

Feature: `v1_16`

<!-- struct WebRTCRTPReceiver -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- struct WebRTCRTPSender -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- struct WebRTCRTPTransceiver -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl WebRTCRTPTransceiver::fn get_property_direction -->
Direction of the transceiver.

Feature: `v1_18`

<!-- impl WebRTCRTPTransceiver::fn set_property_direction -->
Direction of the transceiver.

Feature: `v1_18`

<!-- enum WebRTCRTPTransceiverDirection -->
<!-- enum WebRTCRTPTransceiverDirection::variant None -->
none
<!-- enum WebRTCRTPTransceiverDirection::variant Inactive -->
inactive
<!-- enum WebRTCRTPTransceiverDirection::variant Sendonly -->
sendonly
<!-- enum WebRTCRTPTransceiverDirection::variant Recvonly -->
recvonly
<!-- enum WebRTCRTPTransceiverDirection::variant Sendrecv -->
sendrecv
<!-- enum WebRTCSCTPTransportState -->
GST_WEBRTC_SCTP_TRANSPORT_STATE_NEW: new
GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTING: connecting
GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTED: connected
GST_WEBRTC_SCTP_TRANSPORT_STATE_CLOSED: closed
See <http://w3c.github.io/webrtc-pc/`dom`-rtcsctptransportstate>

Feature: `v1_16`

<!-- enum WebRTCSDPType -->
See <http://w3c.github.io/webrtc-pc/`rtcsdptype`>
<!-- enum WebRTCSDPType::variant Offer -->
offer
<!-- enum WebRTCSDPType::variant Pranswer -->
pranswer
<!-- enum WebRTCSDPType::variant Answer -->
answer
<!-- enum WebRTCSDPType::variant Rollback -->
rollback
<!-- struct WebRTCSessionDescription -->
See <https://www.w3.org/TR/webrtc/`rtcsessiondescription`-class>
<!-- impl WebRTCSessionDescription::fn new -->
## `type_`
a `WebRTCSDPType`
## `sdp`
a `gst_sdp::SDPMessage`

# Returns

a new `WebRTCSessionDescription` from `type_`
 and `sdp`
<!-- impl WebRTCSessionDescription::fn copy -->

# Returns

a new copy of `self`
<!-- impl WebRTCSessionDescription::fn free -->
Free `self` and all associated resources
<!-- enum WebRTCSignalingState -->
See <http://w3c.github.io/webrtc-pc/`dom`-rtcsignalingstate>
<!-- enum WebRTCSignalingState::variant Stable -->
stable
<!-- enum WebRTCSignalingState::variant Closed -->
closed
<!-- enum WebRTCSignalingState::variant HaveLocalOffer -->
have-local-offer
<!-- enum WebRTCSignalingState::variant HaveRemoteOffer -->
have-remote-offer
<!-- enum WebRTCSignalingState::variant HaveLocalPranswer -->
have-local-pranswer
<!-- enum WebRTCSignalingState::variant HaveRemotePranswer -->
have-remote-pranswer
<!-- enum WebRTCStatsType -->
<!-- enum WebRTCStatsType::variant Codec -->
codec
<!-- enum WebRTCStatsType::variant InboundRtp -->
inbound-rtp
<!-- enum WebRTCStatsType::variant OutboundRtp -->
outbound-rtp
<!-- enum WebRTCStatsType::variant RemoteInboundRtp -->
remote-inbound-rtp
<!-- enum WebRTCStatsType::variant RemoteOutboundRtp -->
remote-outbound-rtp
<!-- enum WebRTCStatsType::variant Csrc -->
csrc
<!-- enum WebRTCStatsType::variant PeerConnection -->
peer-connectiion
<!-- enum WebRTCStatsType::variant DataChannel -->
data-channel
<!-- enum WebRTCStatsType::variant Stream -->
stream
<!-- enum WebRTCStatsType::variant Transport -->
transport
<!-- enum WebRTCStatsType::variant CandidatePair -->
candidate-pair
<!-- enum WebRTCStatsType::variant LocalCandidate -->
local-candidate
<!-- enum WebRTCStatsType::variant RemoteCandidate -->
remote-candidate
<!-- enum WebRTCStatsType::variant Certificate -->
certificate
