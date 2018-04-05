<!-- file * -->
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
<!-- struct WebRTCRTPReceiver -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- struct WebRTCRTPSender -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- struct WebRTCRTPTransceiver -->


# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- enum WebRTCSDPType -->
GST_WEBRTC_SDP_TYPE_OFFER: offer
GST_WEBRTC_SDP_TYPE_PRANSWER: pranswer
GST_WEBRTC_SDP_TYPE_ANSWER: answer
GST_WEBRTC_SDP_TYPE_ROLLBACK: rollback
See <ulink url="http://w3c.github.io/webrtc-pc/`rtcsdptype`">http://w3c.github.io/webrtc-pc/`rtcsdptype``</ulink>`
<!-- struct WebRTCSessionDescription -->
sdp: the `gst_sdp::SDPMessage` of the description
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
