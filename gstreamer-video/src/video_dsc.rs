// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::ffi;
use glib::translate::IntoGlibPtr;

#[doc(alias = "GstH274DigitallySignedContentInitialization")]
#[repr(transparent)]
pub struct H274DigitallySignedContentInitialization(
    ffi::GstH274DigitallySignedContentInitialization,
);

impl H274DigitallySignedContentInitialization {
    pub fn new(
        hash_method_type: u8,
        content_uuid: Option<[u8; 16]>,
        key_source_uri: Option<&str>,
    ) -> Self {
        skip_assert_initialized!();
        let key_source_uri_ptr = key_source_uri
            .map(|s| glib::GString::from(s).into_glib_ptr())
            .unwrap_or(std::ptr::null_mut());

        let ffi = ffi::GstH274DigitallySignedContentInitialization {
            id: 0,
            hash_method_type,
            key_retrieval_mode_idc: 0,
            use_key_register_idx_flag: 0,
            key_register_idx: 0,
            content_uuid_present_flag: content_uuid.is_some() as u8,
            content_uuid: content_uuid.unwrap_or([0u8; 16]),
            num_verification_substreams: 1,
            ref_substream_flag: std::ptr::null_mut(),
            ref_substream_flag_len: 0,
            vss_implicit_association_mode_flag: 0,
            signed_content_start_flag: 1,
            sei_signing_flag: 0,
            key_source_uri: key_source_uri_ptr,
        };

        Self(ffi)
    }

    pub fn id(&self) -> u8 {
        self.0.id
    }

    pub fn hash_method_type(&self) -> u8 {
        self.0.hash_method_type
    }

    pub fn key_retrieval_mode_idc(&self) -> u32 {
        self.0.key_retrieval_mode_idc
    }

    pub fn use_key_register_idx_flag(&self) -> bool {
        self.0.use_key_register_idx_flag != 0
    }

    pub fn key_register_idx(&self) -> u32 {
        self.0.key_register_idx
    }

    pub fn content_uuid_present_flag(&self) -> bool {
        self.0.content_uuid_present_flag != 0
    }

    pub fn content_uuid(&self) -> &[u8; 16] {
        &self.0.content_uuid
    }

    pub fn num_verification_substreams(&self) -> u32 {
        self.0.num_verification_substreams
    }

    pub fn ref_substream_flag(&self) -> &[u8] {
        if self.0.ref_substream_flag.is_null() {
            &[]
        } else {
            unsafe {
                std::slice::from_raw_parts(self.0.ref_substream_flag, self.0.ref_substream_flag_len)
            }
        }
    }

    pub fn vss_implicit_association_mode_flag(&self) -> bool {
        self.0.vss_implicit_association_mode_flag != 0
    }

    pub fn signed_content_start_flag(&self) -> bool {
        self.0.signed_content_start_flag != 0
    }

    pub fn sei_signing_flag(&self) -> bool {
        self.0.sei_signing_flag != 0
    }

    pub fn key_source_uri(&self) -> Option<&str> {
        if self.0.key_source_uri.is_null() {
            None
        } else {
            unsafe { Some(glib::GStr::from_ptr(self.0.key_source_uri).as_str()) }
        }
    }

    #[inline]
    pub unsafe fn from_glib_ptr_borrow(
        ptr: &ffi::GstH274DigitallySignedContentInitialization,
    ) -> &Self {
        unsafe { &*(ptr as *const ffi::GstH274DigitallySignedContentInitialization as *const Self) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstH274DigitallySignedContentInitialization {
        &self.0
    }

    // rustdoc-stripper-ignore-next
    /// Set the `key_retrieval_mode_idc` field (H.274 §D.3.24).
    ///
    /// 0 = trust record (e.g. C2PA manifest), 1 = certificate (X.509 PEM).
    pub fn set_key_retrieval_mode_idc(&mut self, mode_idc: u32) {
        self.0.key_retrieval_mode_idc = mode_idc;
    }
}

impl Clone for H274DigitallySignedContentInitialization {
    fn clone(&self) -> Self {
        unsafe {
            let mut dst = std::mem::zeroed();
            ffi::gst_h274_dsc_initialization_copy(&mut dst, &self.0);
            Self(dst)
        }
    }
}

impl Drop for H274DigitallySignedContentInitialization {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_h274_dsc_initialization_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for H274DigitallySignedContentInitialization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("H274DigitallySignedContentInitialization")
            .field("id", &self.id())
            .field("hash_method_type", &self.hash_method_type())
            .field("key_retrieval_mode_idc", &self.key_retrieval_mode_idc())
            .field(
                "use_key_register_idx_flag",
                &self.use_key_register_idx_flag(),
            )
            .field("key_register_idx", &self.key_register_idx())
            .field(
                "content_uuid_present_flag",
                &self.content_uuid_present_flag(),
            )
            .field("content_uuid", &self.content_uuid())
            .field(
                "num_verification_substreams",
                &self.num_verification_substreams(),
            )
            .field("ref_substream_flag", &self.ref_substream_flag())
            .field(
                "vss_implicit_association_mode_flag",
                &self.vss_implicit_association_mode_flag(),
            )
            .field(
                "signed_content_start_flag",
                &self.signed_content_start_flag(),
            )
            .field("sei_signing_flag", &self.sei_signing_flag())
            .field("key_source_uri", &self.key_source_uri())
            .finish()
    }
}

#[doc(alias = "GstH274DigitallySignedContentSelection")]
#[repr(transparent)]
pub struct H274DigitallySignedContentSelection(ffi::GstH274DigitallySignedContentSelection);

impl H274DigitallySignedContentSelection {
    pub fn new(verification_substream_id: u8) -> Self {
        skip_assert_initialized!();
        let ffi = ffi::GstH274DigitallySignedContentSelection {
            id: 0,
            verification_substream_id,
        };

        Self(ffi)
    }

    pub fn id(&self) -> u8 {
        self.0.id
    }

    pub fn verification_substream_id(&self) -> u8 {
        self.0.verification_substream_id
    }

    #[inline]
    pub unsafe fn from_glib_ptr_borrow(ptr: &ffi::GstH274DigitallySignedContentSelection) -> &Self {
        unsafe { &*(ptr as *const ffi::GstH274DigitallySignedContentSelection as *const Self) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstH274DigitallySignedContentSelection {
        &self.0
    }
}

impl Clone for H274DigitallySignedContentSelection {
    fn clone(&self) -> Self {
        unsafe {
            let mut dst = std::mem::zeroed();
            ffi::gst_h274_dsc_selection_copy(&mut dst, &self.0);
            Self(dst)
        }
    }
}

impl fmt::Debug for H274DigitallySignedContentSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("H274DigitallySignedContentSelection")
            .field("id", &self.id())
            .field(
                "verification_substream_id",
                &self.verification_substream_id(),
            )
            .finish()
    }
}

#[doc(alias = "GstH274DigitallySignedContentVerification")]
#[repr(transparent)]
pub struct H274DigitallySignedContentVerification(ffi::GstH274DigitallySignedContentVerification);

impl H274DigitallySignedContentVerification {
    pub fn new(verification_substream_id: u8, signature: &[u8]) -> Self {
        skip_assert_initialized!();
        assert!(!signature.is_empty(), "DSC signature must not be empty");
        let signature_length_in_octets_minus1 = (signature.len() - 1) as u32;

        let slice: glib::Slice<u8> = signature.into();
        let signature_ptr = slice.into_glib_ptr();

        let ffi = ffi::GstH274DigitallySignedContentVerification {
            id: 0,
            verification_substream_id,
            signature_length_in_octets_minus1,
            signature: signature_ptr,
            signed_content_end_flag: 1,
        };

        Self(ffi)
    }

    pub fn id(&self) -> u8 {
        self.0.id
    }

    pub fn verification_substream_id(&self) -> u8 {
        self.0.verification_substream_id
    }

    pub fn signature(&self) -> &[u8] {
        if self.0.signature.is_null() {
            &[]
        } else {
            let len = (self.0.signature_length_in_octets_minus1 + 1) as usize;
            unsafe { std::slice::from_raw_parts(self.0.signature, len) }
        }
    }

    pub fn signature_length_in_octets(&self) -> u32 {
        self.0.signature_length_in_octets_minus1 + 1
    }

    pub fn signed_content_end_flag(&self) -> bool {
        self.0.signed_content_end_flag != 0
    }

    #[inline]
    pub unsafe fn from_glib_ptr_borrow(
        ptr: &ffi::GstH274DigitallySignedContentVerification,
    ) -> &Self {
        unsafe { &*(ptr as *const ffi::GstH274DigitallySignedContentVerification as *const Self) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstH274DigitallySignedContentVerification {
        &self.0
    }
}

impl Clone for H274DigitallySignedContentVerification {
    fn clone(&self) -> Self {
        unsafe {
            let mut dst = std::mem::zeroed();
            ffi::gst_h274_dsc_verification_copy(&mut dst, &self.0);
            Self(dst)
        }
    }
}

impl Drop for H274DigitallySignedContentVerification {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_h274_dsc_verification_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for H274DigitallySignedContentVerification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("H274DigitallySignedContentVerification")
            .field("id", &self.id())
            .field(
                "verification_substream_id",
                &self.verification_substream_id(),
            )
            .field("signature_length", &self.signature().len())
            .field("signature", &self.signature())
            .field("signed_content_end_flag", &self.signed_content_end_flag())
            .finish()
    }
}
