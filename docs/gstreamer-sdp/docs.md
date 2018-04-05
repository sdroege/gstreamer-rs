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
<!-- struct MIKEYMessage -->
Structure holding the information of the MIKEY message
<!-- impl MIKEYMessage::fn new -->
Make a new MIKEY message.

# Returns

a new `MIKEYMessage` on success
<!-- impl MIKEYMessage::fn new_from_bytes -->
Make a new `MIKEYMessage` from `bytes`.
## `bytes`
a `glib::Bytes`
## `info`
a `MIKEYDecryptInfo`

# Returns

a new `MIKEYMessage`
<!-- impl MIKEYMessage::fn new_from_caps -->
Makes mikey message including:
 - Security Policy Payload
 - Key Data Transport Payload
 - Key Data Sub-Payload
## `caps`
a `gst::Caps`, including SRTP parameters (srtp/srtcp cipher, authorization, key data)

# Returns

a `MIKEYMessage`,
or `None` if there is no srtp information in the caps.
<!-- impl MIKEYMessage::fn new_from_data -->
Parse `size` bytes from `data` into a `MIKEYMessage`. `info` contains the
parameters to decrypt and verify the data.
## `data`
bytes to read
## `size`
length of `data`
## `info`
`MIKEYDecryptInfo`

# Returns

a `MIKEYMessage` on success or `None` when parsing failed and
`error` will be set.
<!-- impl MIKEYMessage::fn add_cs_srtp -->
Add a Crypto policy for SRTP to `self`.
## `policy`
The security policy applied for the stream with `ssrc`
## `ssrc`
the SSRC that must be used for the stream
## `roc`
current rollover counter

# Returns

`true` on success
<!-- impl MIKEYMessage::fn add_payload -->
Add a new payload to `self`.
## `payload`
a `MIKEYPayload`

# Returns

`true` on success
<!-- impl MIKEYMessage::fn add_pke -->
Add a new PKE payload to `self` with the given parameters.
## `C`
envelope key cache indicator
## `data_len`
the length of `data`
## `data`
the encrypted envelope key

# Returns

`true` on success
<!-- impl MIKEYMessage::fn add_rand -->
Add a new RAND payload to `self` with the given parameters.
## `len`
the length of `rand`
## `rand`
random data

# Returns

`true` on success
<!-- impl MIKEYMessage::fn add_rand_len -->
Add a new RAND payload to `self` with `len` random bytes.
## `len`
length

# Returns

`true` on success
<!-- impl MIKEYMessage::fn add_t -->
Add a new T payload to `self` with the given parameters.
## `type_`
specifies the timestamp type used
## `ts_value`
The timestamp value of the specified `type_`

# Returns

`true` on success
<!-- impl MIKEYMessage::fn add_t_now_ntp_utc -->
Add a new T payload to `self` that contains the current time
in NTP-UTC format.

# Returns

`true` on success
<!-- impl MIKEYMessage::fn base64_encode -->

# Returns

a `gchar`, base64-encoded data
<!-- impl MIKEYMessage::fn find_payload -->
Find the `nth` occurence of the payload with `type_` in `self`.
## `type_`
a `MIKEYPayloadType`
## `nth`
payload to find

# Returns

the `nth` `MIKEYPayload` of `type_`.
<!-- impl MIKEYMessage::fn get_cs_srtp -->
Get the policy information of `self` at `idx`.
## `idx`
an index

# Returns

a `MIKEYMapSRTP`
<!-- impl MIKEYMessage::fn get_n_cs -->
Get the number of crypto sessions in `self`.

# Returns

the number of crypto sessions
<!-- impl MIKEYMessage::fn get_n_payloads -->
Get the number of payloads in `self`.

# Returns

the number of payloads in `self`
<!-- impl MIKEYMessage::fn get_payload -->
Get the `MIKEYPayload` at `idx` in `self`
## `idx`
an index

# Returns

the `MIKEYPayload` at `idx`. The payload
remains valid for as long as it is part of `self`.
<!-- impl MIKEYMessage::fn insert_cs_srtp -->
Insert a Crypto Session map for SRTP in `self` at `idx`

When `idx` is -1, the policy will be appended.
## `idx`
the index to insert at
## `map`
the map info

# Returns

`true` on success
<!-- impl MIKEYMessage::fn insert_payload -->
Insert the `payload` at index `idx` in `self`. If `idx` is -1, the payload
will be appended to `self`.
## `idx`
an index
## `payload`
a `MIKEYPayload`

# Returns

`true` on success
<!-- impl MIKEYMessage::fn remove_cs_srtp -->
Remove the SRTP policy at `idx`.
## `idx`
the index to remove

# Returns

`true` on success
<!-- impl MIKEYMessage::fn remove_payload -->
Remove the payload in `self` at `idx`
## `idx`
an index

# Returns

`true` on success
<!-- impl MIKEYMessage::fn replace_cs_srtp -->
Replace a Crypto Session map for SRTP in `self` at `idx` with `map`.
## `idx`
the index to insert at
## `map`
the map info

# Returns

`true` on success
<!-- impl MIKEYMessage::fn replace_payload -->
Replace the payload at `idx` in `self` with `payload`.
## `idx`
an index
## `payload`
a `MIKEYPayload`

# Returns

`true` on success
<!-- impl MIKEYMessage::fn set_info -->
Set the information in `self`.
## `version`
a version
## `type_`
a `MIKEYType`
## `V`
verify flag
## `prf_func`
the `MIKEYPRFFunc` function to use
## `CSB_id`
the Crypto Session Bundle id
## `map_type`
the `GstMIKEYCSIDMapType`

# Returns

`true` on success
<!-- impl MIKEYMessage::fn to_bytes -->
Convert `self` to a `glib::Bytes`.
## `info`
a `MIKEYEncryptInfo`

# Returns

a new `glib::Bytes` for `self`.
<!-- impl MIKEYMessage::fn to_caps -->

Feature: `v1_8_1`

## `caps`
a `gst::Caps` to be filled with SRTP parameters (srtp/srtcp cipher, authorization, key data)

# Returns

`true` on success
<!-- enum MIKEYPRFFunc -->
The PRF function that has been/will be used for key derivation
<!-- enum MIKEYPRFFunc::variant MikeyPrfMikey1 -->
MIKEY-1 PRF function
<!-- struct MIKEYPayload -->
Hold the common fields for all payloads
<!-- impl MIKEYPayload::fn new -->
Make a new `MIKEYPayload` with `type_`.
## `type_`
a `MIKEYPayloadType`

# Returns

a new `MIKEYPayload` or `None` on failure.
<!-- impl MIKEYPayload::fn kemac_add_sub -->
Add a new sub payload to `self`.
## `newpay`
a `MIKEYPayload` to add

# Returns

`true` on success.
<!-- impl MIKEYPayload::fn kemac_get_n_sub -->
Get the number of sub payloads of `self`. `self` should be of type
`MIKEYPayloadType::Kemac`.

# Returns

the number of sub payloads in `self`
<!-- impl MIKEYPayload::fn kemac_get_sub -->
Get the sub payload of `self` at `idx`. `self` should be of type
`MIKEYPayloadType::Kemac`.
## `idx`
an index

# Returns

the `MIKEYPayload` at `idx`.
<!-- impl MIKEYPayload::fn kemac_remove_sub -->
Remove the sub payload at `idx` in `self`.
## `idx`
the index to remove

# Returns

`true` on success.
<!-- impl MIKEYPayload::fn kemac_set -->
Set the KEMAC parameters. `self` should point to a `MIKEYPayloadType::Kemac`
payload.
## `enc_alg`
the `MIKEYEncAlg`
## `mac_alg`
a `MIKEYMacAlg`

# Returns

`true` on success
<!-- impl MIKEYPayload::fn key_data_set_interval -->
Set the key validity period in the `MIKEYPayloadType::KeyData` `self`.
## `vf_len`
the length of `vf_data`
## `vf_data`
the Valid From data
## `vt_len`
the length of `vt_data`
## `vt_data`
the Valid To data

# Returns

`true` on success
<!-- impl MIKEYPayload::fn key_data_set_key -->
Set `key_len` bytes of `key_data` of type `key_type` as the key for the
`MIKEYPayloadType::KeyData` `self`.
## `key_type`
a `MIKEYKeyDataType`
## `key_len`
the length of `key_data`
## `key_data`
the key of type `key_type`

# Returns

`true` on success
<!-- impl MIKEYPayload::fn key_data_set_salt -->
Set the salt key data. If `salt_len` is 0 and `salt_data` is `None`, the
salt data will be removed.
## `salt_len`
the length of `salt_data`
## `salt_data`
the salt

# Returns

`true` on success
<!-- impl MIKEYPayload::fn key_data_set_spi -->
Set the SPI/MKI validity in the `MIKEYPayloadType::KeyData` `self`.
## `spi_len`
the length of `spi_data`
## `spi_data`
the SPI/MKI data

# Returns

`true` on success
<!-- impl MIKEYPayload::fn pke_set -->
Set the PKE values in `self`. `self` must be of type
`MIKEYPayloadType::Pke`.
## `C`
envelope key cache indicator
## `data_len`
the length of `data`
## `data`
the encrypted envelope key

# Returns

`true` on success
<!-- impl MIKEYPayload::fn rand_set -->
Set the random values in a `MIKEYPayloadType::Rand` `self`.
## `len`
the length of `rand`
## `rand`
random values

# Returns

`true` on success
<!-- impl MIKEYPayload::fn sp_add_param -->
Add a new parameter to the `MIKEYPayloadType::Sp` `self` with `type_`, `len`
and `val`.
## `type_`
a type
## `len`
a length
## `val`
`len` bytes of data

# Returns

`true` on success
<!-- impl MIKEYPayload::fn sp_get_n_params -->
Get the number of security policy parameters in a `MIKEYPayloadType::Sp`
`self`.

# Returns

the number of parameters in `self`
<!-- impl MIKEYPayload::fn sp_get_param -->
Get the Security Policy parameter in a `MIKEYPayloadType::Sp` `self`
at `idx`.
## `idx`
an index

# Returns

the `MIKEYPayloadSPParam` at `idx` in `self`
<!-- impl MIKEYPayload::fn sp_remove_param -->
Remove the Security Policy parameters from a `MIKEYPayloadType::Sp`
`self` at `idx`.
## `idx`
an index

# Returns

`true` on success
<!-- impl MIKEYPayload::fn sp_set -->
Set the Security Policy parameters for `self`.
## `policy`
the policy number
## `proto`
a `MIKEYSecProto`

# Returns

`true` on success
<!-- impl MIKEYPayload::fn t_set -->
Set the timestamp in a `MIKEYPayloadType::T` `self`.
## `type_`
the `MIKEYTSType`
## `ts_value`
the timestamp value

# Returns

`true` on success
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
