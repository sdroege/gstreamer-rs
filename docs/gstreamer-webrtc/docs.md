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
GST_WEBRTC_DTLS_SETUP_NONE: none
GST_WEBRTC_DTLS_SETUP_ACTPASS: actpass
GST_WEBRTC_DTLS_SETUP_ACTIVE: sendonly
GST_WEBRTC_DTLS_SETUP_PASSIVE: recvonly
<!-- struct WebRTCDTLSTransport -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- enum WebRTCDTLSTransportState -->
GST_WEBRTC_DTLS_TRANSPORT_STATE_NEW: new
GST_WEBRTC_DTLS_TRANSPORT_STATE_CLOSED: closed
GST_WEBRTC_DTLS_TRANSPORT_STATE_FAILED: failed
GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTING: connecting
GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTED: connected
<!-- enum WebRTCDataChannelState -->
GST_WEBRTC_DATA_CHANNEL_STATE_NEW: new
GST_WEBRTC_DATA_CHANNEL_STATE_CONNECTING: connection
GST_WEBRTC_DATA_CHANNEL_STATE_OPEN: open
GST_WEBRTC_DATA_CHANNEL_STATE_CLOSING: closing
GST_WEBRTC_DATA_CHANNEL_STATE_CLOSED: closed
See <ulink url="http://w3c.github.io/webrtc-pc/`dom`-rtcdatachannelstate">http://w3c.github.io/webrtc-pc/`dom`-rtcdatachannelstate`</ulink>`

Feature: `v1_16`

<!-- enum WebRTCFECType -->
<!-- enum WebRTCFECType::variant None -->
none
<!-- enum WebRTCFECType::variant UlpRed -->
ulpfec + red

Feature: `v1_14_1`

<!-- enum WebRTCICEComponent -->
GST_WEBRTC_ICE_COMPONENT_RTP,
GST_WEBRTC_ICE_COMPONENT_RTCP,
<!-- enum WebRTCICEConnectionState -->
GST_WEBRTC_ICE_CONNECTION_STATE_NEW: new
GST_WEBRTC_ICE_CONNECTION_STATE_CHECKING: checking
GST_WEBRTC_ICE_CONNECTION_STATE_CONNECTED: connected
GST_WEBRTC_ICE_CONNECTION_STATE_COMPLETED: completed
GST_WEBRTC_ICE_CONNECTION_STATE_FAILED: failed
GST_WEBRTC_ICE_CONNECTION_STATE_DISCONNECTED: disconnected
GST_WEBRTC_ICE_CONNECTION_STATE_CLOSED: closed
See <ulink url="http://w3c.github.io/webrtc-pc/`dom`-rtciceconnectionstate">http://w3c.github.io/webrtc-pc/`dom`-rtciceconnectionstate`</ulink>`
<!-- enum WebRTCICEGatheringState -->
GST_WEBRTC_ICE_GATHERING_STATE_NEW: new
GST_WEBRTC_ICE_GATHERING_STATE_GATHERING: gathering
GST_WEBRTC_ICE_GATHERING_STATE_COMPLETE: complete
See <ulink url="http://w3c.github.io/webrtc-pc/`dom`-rtcicegatheringstate">http://w3c.github.io/webrtc-pc/`dom`-rtcicegatheringstate`</ulink>`
<!-- enum WebRTCICERole -->
GST_WEBRTC_ICE_ROLE_CONTROLLED: controlled
GST_WEBRTC_ICE_ROLE_CONTROLLING: controlling
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
GST_WEBRTC_PEER_CONNECTION_STATE_NEW: new
GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTING: connecting
GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTED: connected
GST_WEBRTC_PEER_CONNECTION_STATE_DISCONNECTED: disconnected
GST_WEBRTC_PEER_CONNECTION_STATE_FAILED: failed
GST_WEBRTC_PEER_CONNECTION_STATE_CLOSED: closed
See <ulink url="http://w3c.github.io/webrtc-pc/`dom`-rtcpeerconnectionstate">http://w3c.github.io/webrtc-pc/`dom`-rtcpeerconnectionstate`</ulink>`
<!-- enum WebRTCPriorityType -->
GST_WEBRTC_PRIORITY_TYPE_VERY_LOW: very-low
GST_WEBRTC_PRIORITY_TYPE_LOW: low
GST_WEBRTC_PRIORITY_TYPE_MEDIUM: medium
GST_WEBRTC_PRIORITY_TYPE_HIGH: high
See <ulink url="http://w3c.github.io/webrtc-pc/`dom`-rtcprioritytype">http://w3c.github.io/webrtc-pc/`dom`-rtcprioritytype`</ulink>`

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
<!-- enum WebRTCRTPTransceiverDirection -->
<!-- enum WebRTCSCTPTransportState -->
GST_WEBRTC_SCTP_TRANSPORT_STATE_NEW: new
GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTING: connecting
GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTED: connected
GST_WEBRTC_SCTP_TRANSPORT_STATE_CLOSED: closed
See <ulink url="http://w3c.github.io/webrtc-pc/`dom`-rtcsctptransportstate">http://w3c.github.io/webrtc-pc/`dom`-rtcsctptransportstate`</ulink>`

Feature: `v1_16`

<!-- enum WebRTCSDPType -->
GST_WEBRTC_SDP_TYPE_OFFER: offer
GST_WEBRTC_SDP_TYPE_PRANSWER: pranswer
GST_WEBRTC_SDP_TYPE_ANSWER: answer
GST_WEBRTC_SDP_TYPE_ROLLBACK: rollback
See <ulink url="http://w3c.github.io/webrtc-pc/`rtcsdptype`">http://w3c.github.io/webrtc-pc/`rtcsdptype``</ulink>`
<!-- struct WebRTCSessionDescription -->
See <ulink url="https://www.w3.org/TR/webrtc/`rtcsessiondescription`-class">https://www.w3.org/TR/webrtc/`rtcsessiondescription`-class`</ulink>`
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
GST_WEBRTC_SIGNALING_STATE_STABLE: stable
GST_WEBRTC_SIGNALING_STATE_CLOSED: closed
GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_OFFER: have-local-offer
GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_OFFER: have-remote-offer
GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_PRANSWER: have-local-pranswer
GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_PRANSWER: have-remote-pranswer
See <ulink url="http://w3c.github.io/webrtc-pc/`dom`-rtcsignalingstate">http://w3c.github.io/webrtc-pc/`dom`-rtcsignalingstate`</ulink>`
<!-- enum WebRTCStatsType -->
GST_WEBRTC_STATS_CODEC: codec
GST_WEBRTC_STATS_INBOUND_RTP: inbound-rtp
GST_WEBRTC_STATS_OUTBOUND_RTP: outbound-rtp
GST_WEBRTC_STATS_REMOTE_INBOUND_RTP: remote-inbound-rtp
GST_WEBRTC_STATS_REMOTE_OUTBOUND_RTP: remote-outbound-rtp
GST_WEBRTC_STATS_CSRC: csrc
GST_WEBRTC_STATS_PEER_CONNECTION: peer-connectiion
GST_WEBRTC_STATS_DATA_CHANNEL: data-channel
GST_WEBRTC_STATS_STREAM: stream
GST_WEBRTC_STATS_TRANSPORT: transport
GST_WEBRTC_STATS_CANDIDATE_PAIR: candidate-pair
GST_WEBRTC_STATS_LOCAL_CANDIDATE: local-candidate
GST_WEBRTC_STATS_REMOTE_CANDIDATE: remote-candidate
GST_WEBRTC_STATS_CERTIFICATE: certificate
