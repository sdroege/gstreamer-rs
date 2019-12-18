<!-- file * -->
<!-- enum RTCPFBType -->
Different types of feedback messages.
<!-- enum RTCPFBType::variant FbTypeInvalid -->
Invalid type
<!-- enum RTCPFBType::variant RtpfbTypeNack -->
Generic NACK
<!-- enum RTCPFBType::variant RtpfbTypeTmmbr -->
Temporary Maximum Media Stream Bit Rate Request
<!-- enum RTCPFBType::variant RtpfbTypeTmmbn -->
Temporary Maximum Media Stream Bit Rate
 Notification
<!-- enum RTCPFBType::variant RtpfbTypeRtcpSrReq -->
Request an SR packet for early
 synchronization
<!-- enum RTCPFBType::variant PsfbTypePli -->
Picture Loss Indication
<!-- enum RTCPFBType::variant PsfbTypeSli -->
Slice Loss Indication
<!-- enum RTCPFBType::variant PsfbTypeRpsi -->
Reference Picture Selection Indication
<!-- enum RTCPFBType::variant PsfbTypeAfb -->
Application layer Feedback
<!-- enum RTCPFBType::variant PsfbTypeFir -->
Full Intra Request Command
<!-- enum RTCPFBType::variant PsfbTypeTstr -->
Temporal-Spatial Trade-off Request
<!-- enum RTCPFBType::variant PsfbTypeTstn -->
Temporal-Spatial Trade-off Notification
<!-- enum RTCPFBType::variant PsfbTypeVbcn -->
Video Back Channel Message
<!-- enum RTCPSDESType -->
Different types of SDES content.
<!-- enum RTCPSDESType::variant Invalid -->
Invalid SDES entry
<!-- enum RTCPSDESType::variant End -->
End of SDES list
<!-- enum RTCPSDESType::variant Cname -->
Canonical name
<!-- enum RTCPSDESType::variant Name -->
User name
<!-- enum RTCPSDESType::variant Email -->
User's electronic mail address
<!-- enum RTCPSDESType::variant Phone -->
User's phone number
<!-- enum RTCPSDESType::variant Loc -->
Geographic user location
<!-- enum RTCPSDESType::variant Tool -->
Name of application or tool
<!-- enum RTCPSDESType::variant Note -->
Notice about the source
<!-- enum RTCPSDESType::variant Priv -->
Private extensions
<!-- enum RTCPType -->
Different RTCP packet types.
<!-- enum RTCPType::variant Invalid -->
Invalid type
<!-- enum RTCPType::variant Sr -->
Sender report
<!-- enum RTCPType::variant Rr -->
Receiver report
<!-- enum RTCPType::variant Sdes -->
Source description
<!-- enum RTCPType::variant Bye -->
Goodbye
<!-- enum RTCPType::variant App -->
Application defined
<!-- enum RTCPType::variant Rtpfb -->
Transport layer feedback.
<!-- enum RTCPType::variant Psfb -->
Payload-specific feedback.
<!-- enum RTCPType::variant Xr -->
Extended report.
<!-- enum RTCPXRType -->
Types of RTCP Extended Reports, those are defined in RFC 3611 and other RFCs
according to the [IANA registry](https://www.iana.org/assignments/rtcp-xr-block-types/rtcp-xr-block-types.xhtml).
<!-- enum RTCPXRType::variant Invalid -->
Invalid XR Report Block
<!-- enum RTCPXRType::variant Lrle -->
Loss RLE Report Block
<!-- enum RTCPXRType::variant Drle -->
Duplicate RLE Report Block
<!-- enum RTCPXRType::variant Prt -->
Packet Receipt Times Report Block
<!-- enum RTCPXRType::variant Rrt -->
Receiver Reference Time Report Block
<!-- enum RTCPXRType::variant Dlrr -->
Delay since the last Receiver Report
<!-- enum RTCPXRType::variant Ssumm -->
Statistics Summary Report Block
<!-- enum RTCPXRType::variant VoipMetrics -->
VoIP Metrics Report Block

Feature: `v1_16`

<!-- enum RTPPayload -->
Standard predefined fixed payload types.

The official list is at:
http://www.iana.org/assignments/rtp-parameters

Audio:
reserved: 19
unassigned: 20-23,

Video:
unassigned: 24, 27, 29, 30, 35-71, 77-95
Reserved for RTCP conflict avoidance: 72-76
<!-- enum RTPPayload::variant Pcmu -->
ITU-T G.711. mu-law audio (RFC 3551)
<!-- enum RTPPayload::variant 1016 -->
RFC 3551 says reserved
<!-- enum RTPPayload::variant G721 -->
RFC 3551 says reserved
<!-- enum RTPPayload::variant Gsm -->
GSM audio
<!-- enum RTPPayload::variant G723 -->
ITU G.723.1 audio
<!-- enum RTPPayload::variant Dvi48000 -->
IMA ADPCM wave type (RFC 3551)
<!-- enum RTPPayload::variant Dvi416000 -->
IMA ADPCM wave type (RFC 3551)
<!-- enum RTPPayload::variant Lpc -->
experimental linear predictive encoding
<!-- enum RTPPayload::variant Pcma -->
ITU-T G.711 A-law audio (RFC 3551)
<!-- enum RTPPayload::variant G722 -->
ITU-T G.722 (RFC 3551)
<!-- enum RTPPayload::variant L16Stereo -->
stereo PCM
<!-- enum RTPPayload::variant L16Mono -->
mono PCM
<!-- enum RTPPayload::variant Qcelp -->
EIA & TIA standard IS-733
<!-- enum RTPPayload::variant Cn -->
Comfort Noise (RFC 3389)
<!-- enum RTPPayload::variant Mpa -->
Audio MPEG 1-3.
<!-- enum RTPPayload::variant G728 -->
ITU-T G.728 Speech coder (RFC 3551)
<!-- enum RTPPayload::variant Dvi411025 -->
IMA ADPCM wave type (RFC 3551)
<!-- enum RTPPayload::variant Dvi422050 -->
IMA ADPCM wave type (RFC 3551)
<!-- enum RTPPayload::variant G729 -->
ITU-T G.729 Speech coder (RFC 3551)
<!-- enum RTPPayload::variant Cellb -->
See RFC 2029
<!-- enum RTPPayload::variant Jpeg -->
ISO Standards 10918-1 and 10918-2 (RFC 2435)
<!-- enum RTPPayload::variant Nv -->
nv encoding by Ron Frederick
<!-- enum RTPPayload::variant H261 -->
ITU-T Recommendation H.261 (RFC 2032)
<!-- enum RTPPayload::variant Mpv -->
Video MPEG 1 & 2 (RFC 2250)
<!-- enum RTPPayload::variant Mp2t -->
MPEG-2 transport stream (RFC 2250)
<!-- enum RTPPayload::variant H263 -->
Video H263 (RFC 2190)
<!-- enum RTPProfile -->
The transfer profile to use.
<!-- enum RTPProfile::variant Unknown -->
invalid profile
<!-- enum RTPProfile::variant Avp -->
the Audio/Visual profile (RFC 3551)
<!-- enum RTPProfile::variant Savp -->
the secure Audio/Visual profile (RFC 3711)
<!-- enum RTPProfile::variant Avpf -->
the Audio/Visual profile with feedback (RFC 4585)
<!-- enum RTPProfile::variant Savpf -->
the secure Audio/Visual profile with feedback (RFC 5124)
