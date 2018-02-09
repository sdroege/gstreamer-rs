<!-- file * -->
<!-- enum MIKEYCacheType -->
The different cache types
<!-- enum MIKEYCacheType::variant None -->
The envelope key MUST NOT be cached
<!-- enum MIKEYCacheType::variant Always -->
The envelope key MUST be cached
<!-- enum MIKEYCacheType::variant ForCsb -->
The envelope key MUST be cached, but only
 to be used for the specific CSB.
<!-- enum MIKEYEncAlg -->
The encryption algorithm used to encrypt the Encr data field
<!-- enum MIKEYEncAlg::variant Null -->
no encryption
<!-- enum MIKEYEncAlg::variant AesCm128 -->
AES-CM using a 128-bit key
<!-- enum MIKEYEncAlg::variant AesKw128 -->
AES Key Wrap using a 128-bit key
<!-- enum MIKEYKVType -->
The key validity type
<!-- enum MIKEYKVType::variant Null -->
No specific usage rule
<!-- enum MIKEYKVType::variant Spi -->
The key is associated with the SPI/MKI
<!-- enum MIKEYKVType::variant Interval -->
The key has a start and expiration time
<!-- enum MIKEYKeyDataType -->
The type of key.
<!-- enum MIKEYKeyDataType::variant Tgk -->
a TEK Generation Key
<!-- enum MIKEYKeyDataType::variant Tek -->
Traffic-Encrypting Key
<!-- enum MIKEYMacAlg -->
Specifies the authentication algorithm used
<!-- enum MIKEYMacAlg::variant Null -->
no authentication
<!-- enum MIKEYMacAlg::variant HmacSha1160 -->
HMAC-SHA-1-160
<!-- enum MIKEYMapType -->
Specifies the method of uniquely mapping Crypto Sessions to the security
protocol sessions.
<!-- enum MIKEYPRFFunc -->
The PRF function that has been/will be used for key derivation
<!-- enum MIKEYPRFFunc::variant MikeyPrfMikey1 -->
MIKEY-1 PRF function
<!-- enum MIKEYPayloadType -->
Different MIKEY Payload types.
<!-- enum MIKEYPayloadType::variant Last -->
Last payload
<!-- enum MIKEYPayloadType::variant Kemac -->
Key data transport payload
<!-- enum MIKEYPayloadType::variant Pke -->
Envelope data payload
<!-- enum MIKEYPayloadType::variant Dh -->
DH data payload
<!-- enum MIKEYPayloadType::variant Sign -->
Signature payload
<!-- enum MIKEYPayloadType::variant T -->
Timestamp payload
<!-- enum MIKEYPayloadType::variant Id -->
ID payload
<!-- enum MIKEYPayloadType::variant Cert -->
Certificate Payload
<!-- enum MIKEYPayloadType::variant Chash -->
Cert hash payload
<!-- enum MIKEYPayloadType::variant V -->
Verfication message payload
<!-- enum MIKEYPayloadType::variant Sp -->
Security Policy payload
<!-- enum MIKEYPayloadType::variant Rand -->
RAND payload
<!-- enum MIKEYPayloadType::variant Err -->
Error payload
<!-- enum MIKEYPayloadType::variant KeyData -->
Key data sub-payload
<!-- enum MIKEYPayloadType::variant GenExt -->
General Extension Payload
<!-- enum MIKEYSecProto -->
Specifies the security protocol
<!-- enum MIKEYSecSRTP -->
This policy specifies the parameters for SRTP and SRTCP
<!-- enum MIKEYSecSRTP::variant EncAlg -->
Encryption algorithm
<!-- enum MIKEYSecSRTP::variant EncKeyLen -->
Session Encr. key length
<!-- enum MIKEYSecSRTP::variant AuthAlg -->
Authentication algorithm
<!-- enum MIKEYSecSRTP::variant AuthKeyLen -->
Session Auth. key length
<!-- enum MIKEYSecSRTP::variant SaltKeyLen -->
Session Salt key length
<!-- enum MIKEYSecSRTP::variant Prf -->
SRTP Pseudo Random Function
<!-- enum MIKEYSecSRTP::variant KeyDerivRate -->
Key derivation rate
<!-- enum MIKEYSecSRTP::variant SrtpEnc -->
SRTP encryption off/on, 0 if off, 1 if on
<!-- enum MIKEYSecSRTP::variant SrtcpEnc -->
SRTCP encryption off/on, 0 if off, 1 if on
<!-- enum MIKEYSecSRTP::variant FecOrder -->
sender's FEC order
<!-- enum MIKEYSecSRTP::variant SrtpAuth -->
SRTP authentication off/on, 0 if off, 1 if on
<!-- enum MIKEYSecSRTP::variant AuthTagLen -->
Authentication tag length
<!-- enum MIKEYSecSRTP::variant SrtpPrefixLen -->
SRTP prefix length
<!-- enum MIKEYTSType -->
Specifies the timestamp type.
<!-- enum MIKEYTSType::variant NtpUtc -->
an NTP time in UTC timezone
<!-- enum MIKEYTSType::variant Ntp -->
an NTP time
<!-- enum MIKEYTSType::variant Counter -->
a counter
<!-- enum MIKEYType -->
Different MIKEY data types.
<!-- enum MIKEYType::variant Invalid -->
Invalid type
<!-- enum MIKEYType::variant PskInit -->
Initiator's pre-shared key message
<!-- enum MIKEYType::variant PskVerify -->
Verification message of a Pre-shared key message
<!-- enum MIKEYType::variant PkInit -->
Initiator's public-key transport message
<!-- enum MIKEYType::variant PkVerify -->
Verification message of a public-key message
<!-- enum MIKEYType::variant DhInit -->
Initiator's DH exchange message
<!-- enum MIKEYType::variant DhResp -->
Responder's DH exchange message
<!-- enum MIKEYType::variant Error -->
Error message
<!-- enum SDPResult -->
Return values for the SDP functions.
<!-- enum SDPResult::variant Ok -->
A successful return value
<!-- enum SDPResult::variant Einval -->
a function was given invalid parameters
