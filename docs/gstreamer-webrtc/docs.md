<!-- file * -->
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
<!-- enum WebRTCPeerConnectionState -->
GST_WEBRTC_PEER_CONNECTION_STATE_NEW: new
GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTING: connecting
GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTED: connected
GST_WEBRTC_PEER_CONNECTION_STATE_DISCONNECTED: disconnected
GST_WEBRTC_PEER_CONNECTION_STATE_FAILED: failed
GST_WEBRTC_PEER_CONNECTION_STATE_CLOSED: closed
See <ulink url="http://w3c.github.io/webrtc-pc/`dom`-rtcpeerconnectionstate">http://w3c.github.io/webrtc-pc/`dom`-rtcpeerconnectionstate`</ulink>`
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
