// Take a look at the license at the top of the repository in the LICENSE file.

use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops;
use std::ptr;

use glib::translate::*;
use glib::StaticType;

use crate::sdp_attribute::SDPAttribute;
use crate::sdp_bandwidth::SDPBandwidth;
use crate::sdp_connection::SDPConnection;
use crate::sdp_key::SDPKey;
use crate::sdp_media::SDPMedia;
use crate::sdp_media::SDPMediaRef;
use crate::sdp_origin::SDPOrigin;
use crate::sdp_time::SDPTime;
use crate::sdp_zone::SDPZone;

glib::wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstSDPMessage")]
    pub struct SDPMessage(Boxed<ffi::GstSDPMessage>);

    match fn {
        copy => |ptr| {
            let mut copy = std::ptr::null_mut();
            assert_eq!(ffi::gst_sdp_message_copy(ptr, &mut copy), ffi::GST_SDP_OK);
            copy
        },
        free => |ptr| assert_eq!(ffi::gst_sdp_message_free(ptr), ffi::GST_SDP_OK),
        type_ => || ffi::gst_sdp_message_get_type(),
    }
}

unsafe impl Send for SDPMessage {}
unsafe impl Sync for SDPMessage {}

impl Default for SDPMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Deref for SDPMessage {
    type Target = SDPMessageRef;

    fn deref(&self) -> &SDPMessageRef {
        unsafe { &*(self.to_glib_none().0 as *const SDPMessageRef) }
    }
}

impl ops::DerefMut for SDPMessage {
    fn deref_mut(&mut self) -> &mut SDPMessageRef {
        unsafe { &mut *(self.to_glib_none_mut().0 as *mut SDPMessageRef) }
    }
}

impl fmt::Debug for SDPMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <SDPMessageRef as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for SDPMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <SDPMessageRef as fmt::Display>::fmt(self, f)
    }
}

impl SDPMessage {
    #[doc(alias = "gst_sdp_message_new")]
    pub fn new() -> SDPMessage {
        assert_initialized_main_thread!();
        unsafe {
            let mut msg = ptr::null_mut();
            ffi::gst_sdp_message_new(&mut msg);
            from_glib_full(msg)
        }
    }

    #[doc(alias = "gst_sdp_message_parse_buffer")]
    pub fn parse_buffer(data: &[u8]) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let size = data.len() as u32;
            let mut msg = ptr::null_mut();
            ffi::gst_sdp_message_new(&mut msg);
            let result = ffi::gst_sdp_message_parse_buffer(data.to_glib_none().0, size, msg);
            match result {
                ffi::GST_SDP_OK => Ok(from_glib_full(msg)),
                _ => {
                    ffi::gst_sdp_message_uninit(msg);
                    Err(glib::bool_error!("Failed to parse buffer"))
                }
            }
        }
    }

    #[doc(alias = "gst_sdp_message_parse_uri")]
    pub fn parse_uri(uri: &str) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut msg = ptr::null_mut();
            ffi::gst_sdp_message_new(&mut msg);
            let result = ffi::gst_sdp_message_parse_uri(uri.to_glib_none().0, msg);
            match result {
                ffi::GST_SDP_OK => Ok(from_glib_full(msg)),
                _ => {
                    ffi::gst_sdp_message_uninit(msg);
                    Err(glib::bool_error!("Failed to parse URI"))
                }
            }
        }
    }
}

#[repr(transparent)]
#[doc(alias = "GstSDPMessage")]
pub struct SDPMessageRef(ffi::GstSDPMessage);

impl fmt::Debug for SDPMessageRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::cell::RefCell;

        struct DebugIter<I>(RefCell<I>);
        impl<I: Iterator> fmt::Debug for DebugIter<I>
        where
            I::Item: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_list().entries(&mut *self.0.borrow_mut()).finish()
            }
        }

        f.debug_struct("SDPMessage")
            .field("connection", &self.connection())
            .field("information", &self.information())
            .field("key", &self.key())
            .field("origin", &self.origin())
            .field("session-name", &self.session_name())
            .field("uri", &self.uri())
            .field("version", &self.version())
            .field("attributes", &DebugIter(RefCell::new(self.attributes())))
            .field("bandwidths", &DebugIter(RefCell::new(self.bandwidths())))
            .field("emails", &DebugIter(RefCell::new(self.emails())))
            .field("medias", &DebugIter(RefCell::new(self.medias())))
            .field("phones", &DebugIter(RefCell::new(self.phones())))
            .field("times", &DebugIter(RefCell::new(self.times())))
            .field("zones", &DebugIter(RefCell::new(self.zones())))
            .finish()
    }
}

impl fmt::Display for SDPMessageRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_text() {
            Ok(text) => f.write_str(text.as_str()),
            Err(_) => Err(fmt::Error),
        }
    }
}

unsafe impl Send for SDPMessageRef {}
unsafe impl Sync for SDPMessageRef {}

impl SDPMessageRef {
    #[doc(alias = "gst_sdp_message_add_attribute")]
    pub fn add_attribute(&mut self, key: &str, value: Option<&str>) {
        unsafe {
            ffi::gst_sdp_message_add_attribute(
                &mut self.0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_sdp_message_add_email")]
    pub fn add_email(&mut self, email: &str) {
        unsafe { ffi::gst_sdp_message_add_email(&mut self.0, email.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_message_add_media")]
    pub fn add_media(&mut self, media: SDPMedia) {
        unsafe {
            ffi::gst_sdp_message_add_media(
                &mut self.0,
                media.to_glib_none().0 as *mut ffi::GstSDPMedia,
            );
        }
    }

    #[doc(alias = "gst_sdp_message_add_phone")]
    pub fn add_phone(&mut self, phone: &str) {
        unsafe { ffi::gst_sdp_message_add_phone(&mut self.0, phone.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_message_add_time")]
    pub fn add_time(&mut self, start: &str, stop: &str, repeat: &[&str]) {
        unsafe {
            ffi::gst_sdp_message_add_time(
                &mut self.0,
                start.to_glib_none().0,
                stop.to_glib_none().0,
                repeat.to_glib_none().0,
            )
        };
    }

    #[doc(alias = "gst_sdp_message_add_zone")]
    pub fn add_zone(&mut self, adj_time: &str, typed_time: &str) {
        unsafe {
            ffi::gst_sdp_message_add_zone(
                &mut self.0,
                adj_time.to_glib_none().0,
                typed_time.to_glib_none().0,
            )
        };
    }

    #[doc(alias = "gst_sdp_message_as_text")]
    pub fn as_text(&self) -> Result<String, glib::error::BoolError> {
        unsafe {
            match from_glib_full(ffi::gst_sdp_message_as_text(&self.0)) {
                Some(s) => Ok(s),
                None => Err(glib::bool_error!(
                    "Failed to convert the contents of message to a text string"
                )),
            }
        }
    }

    #[doc(alias = "gst_sdp_message_attributes_len")]
    pub fn attributes_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_message_attributes_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_message_attributes_to_caps")]
    pub fn attributes_to_caps(&self, caps: &mut gst::CapsRef) -> Result<(), glib::BoolError> {
        let result = unsafe { ffi::gst_sdp_message_attributes_to_caps(&self.0, caps.as_mut_ptr()) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to store attributes in caps")),
        }
    }

    #[doc(alias = "gst_sdp_message_bandwidths_len")]
    pub fn bandwidths_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_message_bandwidths_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_message_dump")]
    pub fn dump(&self) {
        unsafe { ffi::gst_sdp_message_dump(&self.0) };
    }

    #[doc(alias = "gst_sdp_message_emails_len")]
    pub fn emails_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_message_emails_len(&self.0) }
    }

    #[doc(alias = "get_attribute")]
    #[doc(alias = "gst_sdp_message_get_attribute")]
    pub fn attribute(&self, idx: u32) -> Option<&SDPAttribute> {
        if idx >= self.attributes_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_message_get_attribute(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPAttribute))
            }
        }
    }

    #[doc(alias = "get_attribute_val")]
    #[doc(alias = "gst_sdp_message_get_attribute_val")]
    pub fn attribute_val(&self, key: &str) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_attribute_val(&self.0, key.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_attribute_val_n")]
    #[doc(alias = "gst_sdp_message_get_attribute_val_n")]
    pub fn attribute_val_n(&self, key: &str, nth: u32) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_attribute_val_n(&self.0, key.to_glib_none().0, nth);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_bandwidth")]
    #[doc(alias = "gst_sdp_message_get_bandwidth")]
    pub fn bandwidth(&self, idx: u32) -> Option<&SDPBandwidth> {
        if idx >= self.bandwidths_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_message_get_bandwidth(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPBandwidth))
            }
        }
    }

    #[doc(alias = "get_connection")]
    #[doc(alias = "gst_sdp_message_get_connection")]
    pub fn connection(&self) -> Option<&SDPConnection> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_connection(&self.0);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPConnection))
            }
        }
    }

    #[doc(alias = "get_email")]
    #[doc(alias = "gst_sdp_message_get_email")]
    pub fn email(&self, idx: u32) -> Option<&str> {
        if idx >= self.emails_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_message_get_email(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_information")]
    #[doc(alias = "gst_sdp_message_get_information")]
    pub fn information(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_information(&self.0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_key")]
    #[doc(alias = "gst_sdp_message_get_key")]
    pub fn key(&self) -> Option<&SDPKey> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_key(&self.0);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPKey))
            }
        }
    }

    #[doc(alias = "get_media")]
    #[doc(alias = "gst_sdp_message_get_media")]
    pub fn media(&self, idx: u32) -> Option<&SDPMediaRef> {
        if idx >= self.medias_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_message_get_media(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *const SDPMediaRef))
            }
        }
    }

    #[doc(alias = "get_media_mut")]
    pub fn media_mut(&mut self, idx: u32) -> Option<&mut SDPMediaRef> {
        if idx >= self.medias_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_message_get_media(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&mut *(ptr as *mut SDPMediaRef))
            }
        }
    }

    #[doc(alias = "get_origin")]
    #[doc(alias = "gst_sdp_message_get_origin")]
    pub fn origin(&self) -> Option<&SDPOrigin> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_origin(&self.0);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPOrigin))
            }
        }
    }

    #[doc(alias = "get_phone")]
    #[doc(alias = "gst_sdp_message_get_phone")]
    pub fn phone(&self, idx: u32) -> Option<&str> {
        if idx >= self.phones_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_message_get_phone(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_session_name")]
    #[doc(alias = "gst_sdp_message_get_session_name")]
    pub fn session_name(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_session_name(&self.0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_time")]
    #[doc(alias = "gst_sdp_message_get_time")]
    pub fn time(&self, idx: u32) -> Option<&SDPTime> {
        if idx >= self.times_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_message_get_time(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPTime))
            }
        }
    }

    #[doc(alias = "get_uri")]
    #[doc(alias = "gst_sdp_message_get_uri")]
    pub fn uri(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_uri(&self.0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_version")]
    #[doc(alias = "gst_sdp_message_get_version")]
    pub fn version(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_message_get_version(&self.0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_zone")]
    #[doc(alias = "gst_sdp_message_get_zone")]
    pub fn zone(&self, idx: u32) -> Option<&SDPZone> {
        if idx >= self.zones_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_message_get_zone(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPZone))
            }
        }
    }

    #[doc(alias = "gst_sdp_message_insert_attribute")]
    pub fn insert_attribute(
        &mut self,
        idx: Option<u32>,
        attr: SDPAttribute,
    ) -> Result<(), glib::BoolError> {
        if let Some(idx) = idx {
            if idx >= self.attributes_len() {
                return Err(glib::bool_error!("Failed to insert attribute"));
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let mut attr = mem::ManuallyDrop::new(attr);
        let result =
            unsafe { ffi::gst_sdp_message_insert_attribute(&mut self.0, idx, &mut attr.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert attribute")),
        }
    }

    #[doc(alias = "gst_sdp_message_insert_bandwidth")]
    pub fn insert_bandwidth(
        &mut self,
        idx: Option<u32>,
        bw: SDPBandwidth,
    ) -> Result<(), glib::BoolError> {
        if let Some(idx) = idx {
            if idx >= self.bandwidths_len() {
                return Err(glib::bool_error!("Failed to insert bandwidth"));
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let mut bw = mem::ManuallyDrop::new(bw);
        let result = unsafe { ffi::gst_sdp_message_insert_bandwidth(&mut self.0, idx, &mut bw.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert bandwidth")),
        }
    }

    #[doc(alias = "gst_sdp_message_insert_email")]
    pub fn insert_email(&mut self, idx: Option<u32>, email: &str) -> Result<(), glib::BoolError> {
        if let Some(idx) = idx {
            if idx >= self.emails_len() {
                return Err(glib::bool_error!("Failed to insert email"));
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let result =
            unsafe { ffi::gst_sdp_message_insert_email(&mut self.0, idx, email.to_glib_none().0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert email")),
        }
    }

    #[doc(alias = "gst_sdp_message_insert_phone")]
    pub fn insert_phone(&mut self, idx: Option<u32>, phone: &str) -> Result<(), glib::BoolError> {
        if let Some(idx) = idx {
            if idx >= self.phones_len() {
                return Err(glib::bool_error!("Failed to insert phone"));
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let result =
            unsafe { ffi::gst_sdp_message_insert_phone(&mut self.0, idx, phone.to_glib_none().0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert phone")),
        }
    }

    #[doc(alias = "gst_sdp_message_insert_time")]
    pub fn insert_time(&mut self, idx: Option<u32>, time: SDPTime) -> Result<(), glib::BoolError> {
        if let Some(idx) = idx {
            if idx >= self.times_len() {
                return Err(glib::bool_error!("Failed to insert time"));
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let mut time = mem::ManuallyDrop::new(time);
        let result = unsafe { ffi::gst_sdp_message_insert_time(&mut self.0, idx, &mut time.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert time")),
        }
    }

    #[doc(alias = "gst_sdp_message_insert_zone")]
    pub fn insert_zone(&mut self, idx: Option<u32>, zone: SDPZone) -> Result<(), glib::BoolError> {
        if let Some(idx) = idx {
            if idx >= self.zones_len() {
                return Err(glib::bool_error!("Failed to insert zone"));
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let mut zone = mem::ManuallyDrop::new(zone);
        let result = unsafe { ffi::gst_sdp_message_insert_zone(&mut self.0, idx, &mut zone.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert zone")),
        }
    }

    #[doc(alias = "gst_sdp_message_medias_len")]
    pub fn medias_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_message_medias_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_message_phones_len")]
    pub fn phones_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_message_phones_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_message_remove_attribute")]
    pub fn remove_attribute(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.attributes_len() {
            return Err(glib::bool_error!("Failed to remove attribute"));
        }

        let result = unsafe { ffi::gst_sdp_message_remove_attribute(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove attribute")),
        }
    }

    #[doc(alias = "gst_sdp_message_remove_bandwidth")]
    pub fn remove_bandwidth(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.bandwidths_len() {
            return Err(glib::bool_error!("Failed to remove bandwidth"));
        }

        let result = unsafe { ffi::gst_sdp_message_remove_bandwidth(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove bandwidth")),
        }
    }

    #[doc(alias = "gst_sdp_message_remove_email")]
    pub fn remove_email(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.emails_len() {
            return Err(glib::bool_error!("Failed to remove email"));
        }

        let result = unsafe { ffi::gst_sdp_message_remove_email(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove email")),
        }
    }

    #[doc(alias = "gst_sdp_message_remove_phone")]
    pub fn remove_phone(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.phones_len() {
            return Err(glib::bool_error!("Failed to remove phone"));
        }

        let result = unsafe { ffi::gst_sdp_message_remove_phone(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove phone")),
        }
    }

    #[doc(alias = "gst_sdp_message_remove_time")]
    pub fn remove_time(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.times_len() {
            return Err(glib::bool_error!("Failed to remove time"));
        }

        let result = unsafe { ffi::gst_sdp_message_remove_time(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove time")),
        }
    }

    #[doc(alias = "gst_sdp_message_remove_zone")]
    pub fn remove_zone(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.zones_len() {
            return Err(glib::bool_error!("Failed to remove zone"));
        }

        let result = unsafe { ffi::gst_sdp_message_remove_zone(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove zone")),
        }
    }

    #[doc(alias = "gst_sdp_message_replace_attribute")]
    pub fn replace_attribute(
        &mut self,
        idx: u32,
        attr: SDPAttribute,
    ) -> Result<(), glib::BoolError> {
        if idx >= self.attributes_len() {
            return Err(glib::bool_error!("Failed to replace attribute"));
        }

        let mut attr = mem::ManuallyDrop::new(attr);
        let result =
            unsafe { ffi::gst_sdp_message_replace_attribute(&mut self.0, idx, &mut attr.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace attribute")),
        }
    }

    #[doc(alias = "gst_sdp_message_replace_bandwidth")]
    pub fn replace_bandwidth(&mut self, idx: u32, bw: SDPBandwidth) -> Result<(), glib::BoolError> {
        if idx >= self.bandwidths_len() {
            return Err(glib::bool_error!("Failed to replace bandwidth"));
        }

        let mut bw = mem::ManuallyDrop::new(bw);
        let result = unsafe { ffi::gst_sdp_message_replace_bandwidth(&mut self.0, idx, &mut bw.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace bandwidth")),
        }
    }

    #[doc(alias = "gst_sdp_message_replace_email")]
    pub fn replace_email(&mut self, idx: u32, email: &str) -> Result<(), glib::BoolError> {
        if idx >= self.emails_len() {
            return Err(glib::bool_error!("Failed to replace email"));
        }

        let result =
            unsafe { ffi::gst_sdp_message_replace_email(&mut self.0, idx, email.to_glib_none().0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace email")),
        }
    }

    #[doc(alias = "gst_sdp_message_replace_phone")]
    pub fn replace_phone(&mut self, idx: u32, phone: &str) -> Result<(), glib::BoolError> {
        if idx >= self.phones_len() {
            return Err(glib::bool_error!("Failed to replace phone"));
        }

        let result =
            unsafe { ffi::gst_sdp_message_replace_phone(&mut self.0, idx, phone.to_glib_none().0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace phone")),
        }
    }

    #[doc(alias = "gst_sdp_message_replace_time")]
    pub fn replace_time(&mut self, idx: u32, time: SDPTime) -> Result<(), glib::BoolError> {
        if idx >= self.times_len() {
            return Err(glib::bool_error!("Failed to replace time"));
        }

        let mut time = mem::ManuallyDrop::new(time);
        let result = unsafe { ffi::gst_sdp_message_replace_time(&mut self.0, idx, &mut time.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace time")),
        }
    }

    #[doc(alias = "gst_sdp_message_replace_zone")]
    pub fn replace_zone(&mut self, idx: u32, zone: SDPZone) -> Result<(), glib::BoolError> {
        if idx >= self.zones_len() {
            return Err(glib::bool_error!("Failed to replace zone"));
        }

        let mut zone = mem::ManuallyDrop::new(zone);
        let result = unsafe { ffi::gst_sdp_message_replace_zone(&mut self.0, idx, &mut zone.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace zone")),
        }
    }

    #[doc(alias = "gst_sdp_message_set_connection")]
    pub fn set_connection(
        &mut self,
        nettype: &str,
        addrtype: &str,
        address: &str,
        ttl: u32,
        addr_number: u32,
    ) {
        unsafe {
            ffi::gst_sdp_message_set_connection(
                &mut self.0,
                nettype.to_glib_none().0,
                addrtype.to_glib_none().0,
                address.to_glib_none().0,
                ttl,
                addr_number,
            )
        };
    }

    #[doc(alias = "gst_sdp_message_set_information")]
    pub fn set_information(&mut self, information: &str) {
        unsafe { ffi::gst_sdp_message_set_information(&mut self.0, information.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_message_set_key")]
    pub fn set_key(&mut self, type_: &str, data: &str) {
        unsafe {
            ffi::gst_sdp_message_set_key(&mut self.0, type_.to_glib_none().0, data.to_glib_none().0)
        };
    }

    #[doc(alias = "gst_sdp_message_set_origin")]
    pub fn set_origin(
        &mut self,
        username: &str,
        sess_id: &str,
        sess_version: &str,
        nettype: &str,
        addrtype: &str,
        addr: &str,
    ) {
        unsafe {
            ffi::gst_sdp_message_set_origin(
                &mut self.0,
                username.to_glib_none().0,
                sess_id.to_glib_none().0,
                sess_version.to_glib_none().0,
                nettype.to_glib_none().0,
                addrtype.to_glib_none().0,
                addr.to_glib_none().0,
            )
        };
    }

    #[doc(alias = "gst_sdp_message_set_session_name")]
    pub fn set_session_name(&mut self, session_name: &str) {
        unsafe {
            ffi::gst_sdp_message_set_session_name(&mut self.0, session_name.to_glib_none().0)
        };
    }

    #[doc(alias = "gst_sdp_message_set_uri")]
    pub fn set_uri(&mut self, uri: &str) {
        unsafe { ffi::gst_sdp_message_set_uri(&mut self.0, uri.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_message_set_version")]
    pub fn set_version(&mut self, version: &str) {
        unsafe { ffi::gst_sdp_message_set_version(&mut self.0, version.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_message_times_len")]
    pub fn times_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_message_times_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_message_zones_len")]
    pub fn zones_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_message_zones_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_message_as_uri")]
    pub fn as_uri(&self, scheme: &str) -> Result<String, glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            match from_glib_full(ffi::gst_sdp_message_as_uri(
                scheme.to_glib_none().0,
                &self.0,
            )) {
                Some(s) => Ok(s),
                None => Err(glib::bool_error!("Failed to create an URI from message")),
            }
        }
    }

    pub fn attributes(&self) -> AttributesIter {
        AttributesIter::new(self)
    }

    pub fn bandwidths(&self) -> BandwidthsIter {
        BandwidthsIter::new(self)
    }

    pub fn emails(&self) -> EmailsIter {
        EmailsIter::new(self)
    }

    pub fn medias(&self) -> MediasIter {
        MediasIter::new(self)
    }

    pub fn medias_mut(&mut self) -> MediasIterMut {
        MediasIterMut::new(self)
    }

    pub fn phones(&self) -> PhonesIter {
        PhonesIter::new(self)
    }

    pub fn times(&self) -> TimesIter {
        TimesIter::new(self)
    }

    pub fn zones(&self) -> ZonesIter {
        ZonesIter::new(self)
    }
}

impl Borrow<SDPMessageRef> for SDPMessage {
    fn borrow(&self) -> &SDPMessageRef {
        self
    }
}

impl BorrowMut<SDPMessageRef> for SDPMessage {
    fn borrow_mut(&mut self) -> &mut SDPMessageRef {
        &mut *self
    }
}

impl ToOwned for SDPMessageRef {
    type Owned = SDPMessage;

    fn to_owned(&self) -> SDPMessage {
        unsafe {
            let mut ptr = ptr::null_mut();
            ffi::gst_sdp_message_copy(&self.0, &mut ptr);
            from_glib_full(ptr)
        }
    }
}

impl glib::types::StaticType for SDPMessageRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_sdp_message_get_type()) }
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for &'a SDPMessageRef {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        &*(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut SDPMessageRef)
    }
}

impl glib::value::ToValue for SDPMessageRef {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<SDPMessage>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                &self.0 as *const ffi::GstSDPMessage as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for SDPMessageRef {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<SDPMessage>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.map(|s| &s.0 as *const ffi::GstSDPMessage)
                    .unwrap_or(ptr::null()) as *mut _,
            )
        }
        value
    }
}

macro_rules! define_iter(
    ($name:ident, $typ:ty, $get_item:expr, $get_len:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        message: &'a SDPMessageRef,
        idx: usize,
        len: usize,
    }

    impl<'a> $name<'a> {
        fn new(message: &'a SDPMessageRef) -> $name<'a> {
            skip_assert_initialized!();
            let len = $get_len(message);

            $name {
                message,
                idx: 0,
                len: len as usize,
            }
        }
    }

    impl<'a> Iterator for $name<'a> {
        type Item = $typ;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.len {
                return None;
            }

            let item = $get_item(self.message, self.idx as u32).unwrap();
            self.idx += 1;
            Some(item)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.len - self.idx;

            (remaining, Some(remaining))
        }

        fn count(self) -> usize {
            self.len - self.idx
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.idx.overflowing_add(n);
            if end >= self.len || overflow {
                self.idx = self.len;
                None
            } else {
                self.idx = end + 1;
                Some($get_item(self.message, end as u32).unwrap())
            }
        }

        fn last(self) -> Option<Self::Item> {
            if self.idx == self.len {
                None
            } else {
                Some($get_item(self.message, self.len as u32 - 1).unwrap())
            }
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.len {
                return None;
            }

            self.len -= 1;

            Some($get_item(self.message, self.len as u32).unwrap())
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.len.overflowing_sub(n);
            if end <= self.idx || overflow {
                self.idx = self.len;
                None
            } else {
                self.len = end - 1;
                Some($get_item(self.message, self.len as u32).unwrap())
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    impl<'a> std::iter::FusedIterator for $name<'a> {}
    }
);

macro_rules! define_iter_mut(
    ($name:ident, $typ:ty, $get_item:expr, $get_len:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        message: &'a mut SDPMessageRef,
        idx: usize,
        len: usize,
    }

    impl<'a> $name<'a> {
        fn new(message: &'a mut SDPMessageRef) -> $name<'a> {
            skip_assert_initialized!();
            let len = $get_len(message);

            $name {
                message,
                idx: 0,
                len: len as usize,
            }
        }
    }

    impl<'a> Iterator for $name<'a> {
        type Item = $typ;

        fn next(&mut self) -> Option<Self::Item> {
            // The lifetime of the passed self is not 'a but we need to return a message reference
            // with that lifetime here. The reason here is that the compiler does not and can't
            // know that it's statically impossible to mutate the message between different calls
            // to next(), so it has to assume that every call to next() will invalidate any
            // returned references.
            //
            // In theory we could have a function on the iterator that mutates the message and for
            // example removes the message that was returned here at an earlier time. The compiler
            // would be correct to complain in that case, but we don't provide such a function.
            let message = unsafe {
                &mut *(self.message as *mut SDPMessageRef)
            };
            if self.idx >= self.len {
                return None;
            }

            let item = $get_item(message, self.idx as u32).unwrap();
            self.idx += 1;
            Some(item)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.len - self.idx;

            (remaining, Some(remaining))
        }


        fn count(self) -> usize {
            self.len - self.idx
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            let message = unsafe {
                &mut *(self.message as *mut SDPMessageRef)
            };
            let (end, overflow) = self.idx.overflowing_add(n);
            if end >= self.len || overflow {
                self.idx = self.len;
                None
            } else {
                self.idx = end + 1;
                Some($get_item(message, end as u32).unwrap())
            }
        }

        fn last(self) -> Option<Self::Item> {
            if self.idx == self.len {
                None
            } else {
                Some($get_item(self.message, self.len as u32 - 1).unwrap())
            }
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            let message = unsafe {
                &mut *(self.message as *mut SDPMessageRef)
            };
            if self.idx == self.len {
                return None;
            }

            self.len -= 1;
            Some($get_item(message, self.len as u32).unwrap())
        }


        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let message = unsafe {
                &mut *(self.message as *mut SDPMessageRef)
            };
            let (end, overflow) = self.len.overflowing_sub(n);
            if end <= self.idx || overflow {
                self.idx = self.len;
                None
            } else {
                self.len = end - 1;
                Some($get_item(message, self.len as u32).unwrap())
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    impl<'a> std::iter::FusedIterator for $name<'a> {}
    }
);

define_iter!(
    AttributesIter,
    &'a SDPAttribute,
    |message: &'a SDPMessageRef, idx| message.attribute(idx),
    |message: &SDPMessageRef| message.attributes_len()
);
define_iter!(
    BandwidthsIter,
    &'a SDPBandwidth,
    |message: &'a SDPMessageRef, idx| message.bandwidth(idx),
    |message: &SDPMessageRef| message.bandwidths_len()
);
define_iter!(
    EmailsIter,
    &'a str,
    |message: &'a SDPMessageRef, idx| message.email(idx),
    |message: &SDPMessageRef| message.emails_len()
);
define_iter!(
    MediasIter,
    &'a SDPMediaRef,
    |message: &'a SDPMessageRef, idx| message.media(idx),
    |message: &SDPMessageRef| message.medias_len()
);
define_iter_mut!(
    MediasIterMut,
    &'a mut SDPMediaRef,
    |message: &'a mut SDPMessageRef, idx| message.media_mut(idx),
    |message: &SDPMessageRef| message.medias_len()
);
define_iter!(
    PhonesIter,
    &'a str,
    |message: &'a SDPMessageRef, idx| message.phone(idx),
    |message: &SDPMessageRef| message.phones_len()
);
define_iter!(
    TimesIter,
    &'a SDPTime,
    |message: &'a SDPMessageRef, idx| message.time(idx),
    |message: &SDPMessageRef| message.times_len()
);
define_iter!(
    ZonesIter,
    &'a SDPZone,
    |message: &'a SDPMessageRef, idx| message.zone(idx),
    |message: &SDPMessageRef| message.zones_len()
);

#[cfg(test)]
mod tests {
    use crate::SDPMessage;

    fn init() {
        gst::init().unwrap();
    }

    #[test]
    fn media_from_message() {
        init();

        let sdp = "v=0\r\no=- 1938737043334325940 0 IN IP4 0.0.0.0\r\ns=-\r\nt=0 0\r\na=ice-options:trickle\r\nm=video 9 UDP/TLS/RTP/SAVPF 96\r\nc=IN IP4 0.0.0.0\r\na=setup:actpass\r\na=ice-ufrag:YZxU9JlWHzHcF6O2U09/q3PvBhbTPdZW\r\na=ice-pwd:fyrt730GWo5mFGc9m2z/vbUu3z1lewla\r\na=sendrecv\r\na=rtcp-mux\r\na=rtcp-rsize\r\na=rtpmap:96 VP8/90000\r\na=rtcp-fb:96 nack\r\na=rtcp-fb:96 nack pli\r\na=framerate:30\r\na=mid:video0\r\na=fingerprint:sha-256 DB:48:8F:18:13:F3:AA:13:31:B3:75:3D:1A:D3:BA:88:4A:ED:1B:56:14:C3:09:CD:BC:4D:18:42:B9:6A:5F:98\r\nm=audio 9 UDP/TLS/RTP/SAVPF 97\r\nc=IN IP4 0.0.0.0\r\na=setup:actpass\r\na=ice-ufrag:04KZM9qE2S4r06AN6A9CeXOM6mzO0LZY\r\na=ice-pwd:cJTSfHF6hHDAcsTJXZVJeuYCC6rKqBvW\r\na=sendrecv\r\na=rtcp-mux\r\na=rtcp-rsize\r\na=rtpmap:97 OPUS/48000/2\r\na=rtcp-fb:97 nack\r\na=rtcp-fb:97 nack pli\r\na=mid:audio1\r\na=fingerprint:sha-256 DB:48:8F:18:13:F3:AA:13:31:B3:75:3D:1A:D3:BA:88:4A:ED:1B:56:14:C3:09:CD:BC:4D:18:42:B9:6A:5F:98\r\n";
        let sdp = SDPMessage::parse_buffer(sdp.as_bytes()).unwrap();
        let media = sdp.media(0).unwrap();
        assert_eq!(media.formats_len(), 1);
    }

    #[test]
    fn debug_impl() {
        init();

        let sdp = SDPMessage::new();
        assert!(!format!("{:?}", sdp).is_empty());
    }
}
