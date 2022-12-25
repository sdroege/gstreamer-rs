// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::{Borrow, BorrowMut, ToOwned},
    ffi::CStr,
    fmt, mem, ops, ptr,
};

use glib::translate::*;

use crate::{
    sdp_attribute::SDPAttribute, sdp_bandwidth::SDPBandwidth, sdp_connection::SDPConnection,
    sdp_key::SDPKey,
};

glib::wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstSDPMedia")]
    pub struct SDPMedia(Boxed<ffi::GstSDPMedia>);

    match fn {
        copy => |ptr| {
            let mut copy = ptr::null_mut();
            let res = ffi::gst_sdp_media_copy(ptr, &mut copy);
            debug_assert_eq!(res, ffi::GST_SDP_OK);
            copy
        },
        free => |ptr| {
            let res = ffi::gst_sdp_media_free(ptr);
            debug_assert_eq!(res, ffi::GST_SDP_OK);
        },
    }
}

impl SDPMedia {
    #[doc(alias = "gst_sdp_media_new")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut media = ptr::null_mut();
            ffi::gst_sdp_media_new(&mut media);
            from_glib_full(media)
        }
    }
}

impl Default for SDPMedia {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for SDPMedia {}
unsafe impl Sync for SDPMedia {}

impl ops::Deref for SDPMedia {
    type Target = SDPMediaRef;

    fn deref(&self) -> &SDPMediaRef {
        unsafe { &*(self.to_glib_none().0 as *const SDPMediaRef) }
    }
}

impl ops::DerefMut for SDPMedia {
    fn deref_mut(&mut self) -> &mut SDPMediaRef {
        unsafe { &mut *(self.to_glib_none_mut().0 as *mut SDPMediaRef) }
    }
}

impl fmt::Debug for SDPMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <SDPMediaRef as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for SDPMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <SDPMediaRef as fmt::Display>::fmt(self, f)
    }
}

#[repr(transparent)]
#[doc(alias = "GstSDPMedia")]
pub struct SDPMediaRef(ffi::GstSDPMedia);

impl fmt::Debug for SDPMediaRef {
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

        f.debug_struct("SDPMedia")
            .field("formats", &DebugIter(RefCell::new(self.formats())))
            .field("connections", &DebugIter(RefCell::new(self.connections())))
            .field("bandwidths", &DebugIter(RefCell::new(self.bandwidths())))
            .field("attributes", &DebugIter(RefCell::new(self.attributes())))
            .field("information", &self.information())
            .field("key", &self.key())
            .field("media", &self.media())
            .field("port", &self.port())
            .field("num-ports", &self.num_ports())
            .field("proto", &self.proto())
            .finish()
    }
}

impl fmt::Display for SDPMediaRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_text() {
            Ok(text) => f.write_str(text.as_str()),
            Err(_) => Err(fmt::Error),
        }
    }
}

unsafe impl Send for SDPMediaRef {}
unsafe impl Sync for SDPMediaRef {}

impl SDPMediaRef {
    #[doc(alias = "gst_sdp_media_add_attribute")]
    pub fn add_attribute(&mut self, key: &str, value: Option<&str>) {
        let value = value.to_glib_none();
        unsafe { ffi::gst_sdp_media_add_attribute(&mut self.0, key.to_glib_none().0, value.0) };
    }

    #[doc(alias = "gst_sdp_media_add_bandwidth")]
    pub fn add_bandwidth(&mut self, bwtype: &str, bandwidth: u32) {
        unsafe {
            ffi::gst_sdp_media_add_bandwidth(&mut self.0, bwtype.to_glib_none().0, bandwidth)
        };
    }

    #[doc(alias = "gst_sdp_media_add_connection")]
    pub fn add_connection(
        &mut self,
        nettype: &str,
        addrtype: &str,
        address: &str,
        ttl: u32,
        addr_number: u32,
    ) {
        unsafe {
            ffi::gst_sdp_media_add_connection(
                &mut self.0,
                nettype.to_glib_none().0,
                addrtype.to_glib_none().0,
                address.to_glib_none().0,
                ttl,
                addr_number,
            )
        };
    }

    #[doc(alias = "gst_sdp_media_add_format")]
    pub fn add_format(&mut self, format: &str) {
        unsafe { ffi::gst_sdp_media_add_format(&mut self.0, format.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_media_as_text")]
    pub fn as_text(&self) -> Result<String, glib::error::BoolError> {
        unsafe {
            match from_glib_full(ffi::gst_sdp_media_as_text(&self.0)) {
                Some(s) => Ok(s),
                None => Err(glib::bool_error!(
                    "Failed to convert the contents of media to a text string"
                )),
            }
        }
    }

    pub fn attributes(&self) -> AttributesIter {
        AttributesIter::new(self)
    }

    pub fn formats(&self) -> FormatsIter {
        FormatsIter::new(self)
    }

    pub fn bandwidths(&self) -> BandwidthsIter {
        BandwidthsIter::new(self)
    }

    pub fn connections(&self) -> ConnectionsIter {
        ConnectionsIter::new(self)
    }

    #[doc(alias = "gst_sdp_media_attributes_len")]
    pub fn attributes_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_media_attributes_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_media_attributes_to_caps")]
    pub fn attributes_to_caps(&self, caps: &mut gst::CapsRef) -> Result<(), glib::BoolError> {
        let result = unsafe { ffi::gst_sdp_media_attributes_to_caps(&self.0, caps.as_mut_ptr()) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to store attributes in caps")),
        }
    }

    #[doc(alias = "gst_sdp_media_bandwidths_len")]
    pub fn bandwidths_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_media_bandwidths_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_media_connections_len")]
    pub fn connections_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_media_connections_len(&self.0) }
    }

    #[doc(alias = "gst_sdp_media_formats_len")]
    pub fn formats_len(&self) -> u32 {
        unsafe { ffi::gst_sdp_media_formats_len(&self.0) }
    }

    #[doc(alias = "get_attribute")]
    #[doc(alias = "gst_sdp_media_get_attribute")]
    pub fn attribute(&self, idx: u32) -> Option<&SDPAttribute> {
        if idx >= self.attributes_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_media_get_attribute(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPAttribute))
            }
        }
    }

    #[doc(alias = "get_attribute_val")]
    #[doc(alias = "gst_sdp_media_get_attribute_val")]
    pub fn attribute_val(&self, key: &str) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_media_get_attribute_val(&self.0, key.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_attribute_val_n")]
    #[doc(alias = "gst_sdp_media_get_attribute_val_n")]
    pub fn attribute_val_n(&self, key: &str, nth: u32) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_media_get_attribute_val_n(&self.0, key.to_glib_none().0, nth);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_bandwidth")]
    #[doc(alias = "gst_sdp_media_get_bandwidth")]
    pub fn bandwidth(&self, idx: u32) -> Option<&SDPBandwidth> {
        if idx >= self.bandwidths_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_media_get_bandwidth(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPBandwidth))
            }
        }
    }

    #[doc(alias = "get_caps_from_media")]
    #[doc(alias = "gst_sdp_media_get_caps_from_media")]
    pub fn caps_from_media(&self, pt: i32) -> Option<gst::Caps> {
        unsafe { from_glib_full(ffi::gst_sdp_media_get_caps_from_media(&self.0, pt)) }
    }

    #[doc(alias = "get_connection")]
    #[doc(alias = "gst_sdp_media_get_connection")]
    pub fn connection(&self, idx: u32) -> Option<&SDPConnection> {
        if idx >= self.connections_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_media_get_connection(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPConnection))
            }
        }
    }

    #[doc(alias = "get_format")]
    #[doc(alias = "gst_sdp_media_get_format")]
    pub fn format(&self, idx: u32) -> Option<&str> {
        if idx >= self.formats_len() {
            return None;
        }

        unsafe {
            let ptr = ffi::gst_sdp_media_get_format(&self.0, idx);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_information")]
    #[doc(alias = "gst_sdp_media_get_information")]
    pub fn information(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_media_get_information(&self.0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_key")]
    #[doc(alias = "gst_sdp_media_get_key")]
    pub fn key(&self) -> Option<&SDPKey> {
        unsafe {
            let ptr = ffi::gst_sdp_media_get_key(&self.0);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *mut SDPKey))
            }
        }
    }

    #[doc(alias = "get_media")]
    #[doc(alias = "gst_sdp_media_get_media")]
    pub fn media(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_media_get_media(&self.0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "get_num_ports")]
    #[doc(alias = "gst_sdp_media_get_num_ports")]
    pub fn num_ports(&self) -> u32 {
        unsafe { ffi::gst_sdp_media_get_num_ports(&self.0) }
    }

    #[doc(alias = "get_port")]
    #[doc(alias = "gst_sdp_media_get_port")]
    pub fn port(&self) -> u32 {
        unsafe { ffi::gst_sdp_media_get_port(&self.0) }
    }

    #[doc(alias = "get_proto")]
    #[doc(alias = "gst_sdp_media_get_proto")]
    pub fn proto(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::gst_sdp_media_get_proto(&self.0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "gst_sdp_media_insert_attribute")]
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
        let result = unsafe { ffi::gst_sdp_media_insert_attribute(&mut self.0, idx, &mut attr.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert attribute")),
        }
    }

    #[doc(alias = "gst_sdp_media_insert_bandwidth")]
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
        let result = unsafe { ffi::gst_sdp_media_insert_bandwidth(&mut self.0, idx, &mut bw.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert attribute")),
        }
    }

    #[doc(alias = "gst_sdp_media_insert_connection")]
    pub fn insert_connection(
        &mut self,
        idx: Option<u32>,
        conn: SDPConnection,
    ) -> Result<(), glib::BoolError> {
        if let Some(idx) = idx {
            if idx >= self.connections_len() {
                return Err(glib::bool_error!("Failed to insert connection"));
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let mut conn = mem::ManuallyDrop::new(conn);
        let result = unsafe { ffi::gst_sdp_media_insert_connection(&mut self.0, idx, &mut conn.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert connection")),
        }
    }

    #[doc(alias = "gst_sdp_media_insert_format")]
    pub fn insert_format(&mut self, idx: Option<u32>, format: &str) -> Result<(), glib::BoolError> {
        if let Some(idx) = idx {
            if idx >= self.formats_len() {
                return Err(glib::bool_error!("Failed to insert format"));
            }
        }

        let idx = idx.map(|idx| idx as i32).unwrap_or(-1);
        let result =
            unsafe { ffi::gst_sdp_media_insert_format(&mut self.0, idx, format.to_glib_none().0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to insert format")),
        }
    }

    #[doc(alias = "gst_sdp_media_remove_attribute")]
    pub fn remove_attribute(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.attributes_len() {
            return Err(glib::bool_error!("Failed to remove attribute"));
        }

        let result = unsafe { ffi::gst_sdp_media_remove_attribute(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove attribute")),
        }
    }

    #[doc(alias = "gst_sdp_media_remove_bandwidth")]
    pub fn remove_bandwidth(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.bandwidths_len() {
            return Err(glib::bool_error!("Failed to remove bandwidth"));
        }

        let result = unsafe { ffi::gst_sdp_media_remove_bandwidth(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove bandwidth")),
        }
    }

    #[doc(alias = "gst_sdp_media_remove_connection")]
    pub fn remove_connection(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.connections_len() {
            return Err(glib::bool_error!("Failed to remove connection"));
        }

        let result = unsafe { ffi::gst_sdp_media_remove_connection(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove connection")),
        }
    }

    #[doc(alias = "gst_sdp_media_remove_format")]
    pub fn remove_format(&mut self, idx: u32) -> Result<(), glib::BoolError> {
        if idx >= self.formats_len() {
            return Err(glib::bool_error!("Failed to remove format"));
        }

        let result = unsafe { ffi::gst_sdp_media_remove_format(&mut self.0, idx) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to remove format")),
        }
    }

    #[doc(alias = "gst_sdp_media_replace_attribute")]
    pub fn replace_attribute(
        &mut self,
        idx: u32,
        attr: SDPAttribute,
    ) -> Result<(), glib::BoolError> {
        if idx >= self.attributes_len() {
            return Err(glib::bool_error!("Failed to replace attribute"));
        }

        let mut attr = mem::ManuallyDrop::new(attr);
        let result = unsafe { ffi::gst_sdp_media_replace_attribute(&mut self.0, idx, &mut attr.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace attribute")),
        }
    }

    #[doc(alias = "gst_sdp_media_replace_bandwidth")]
    pub fn replace_bandwidth(&mut self, idx: u32, bw: SDPBandwidth) -> Result<(), glib::BoolError> {
        if idx >= self.bandwidths_len() {
            return Err(glib::bool_error!("Failed to replace bandwidth"));
        }

        let mut bw = mem::ManuallyDrop::new(bw);
        let result = unsafe { ffi::gst_sdp_media_replace_bandwidth(&mut self.0, idx, &mut bw.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace bandwidth")),
        }
    }

    #[doc(alias = "gst_sdp_media_replace_connection")]
    pub fn replace_connection(
        &mut self,
        idx: u32,
        conn: SDPConnection,
    ) -> Result<(), glib::BoolError> {
        if idx >= self.connections_len() {
            return Err(glib::bool_error!("Failed to replace connection"));
        }

        let mut conn = mem::ManuallyDrop::new(conn);
        let result =
            unsafe { ffi::gst_sdp_media_replace_connection(&mut self.0, idx, &mut conn.0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace connection")),
        }
    }

    #[doc(alias = "gst_sdp_media_replace_format")]
    pub fn replace_format(&mut self, idx: u32, format: &str) -> Result<(), glib::BoolError> {
        if idx >= self.formats_len() {
            return Err(glib::bool_error!("Failed to replace format"));
        }

        let result =
            unsafe { ffi::gst_sdp_media_replace_format(&mut self.0, idx, format.to_glib_none().0) };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to replace format")),
        }
    }

    #[doc(alias = "gst_sdp_media_set_information")]
    pub fn set_information(&mut self, information: &str) {
        unsafe { ffi::gst_sdp_media_set_information(&mut self.0, information.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_media_set_key")]
    pub fn set_key(&mut self, type_: &str, data: &str) {
        unsafe {
            ffi::gst_sdp_media_set_key(&mut self.0, type_.to_glib_none().0, data.to_glib_none().0)
        };
    }

    #[doc(alias = "gst_sdp_media_set_media")]
    pub fn set_media(&mut self, med: &str) {
        unsafe { ffi::gst_sdp_media_set_media(&mut self.0, med.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_media_set_port_info")]
    pub fn set_port_info(&mut self, port: u32, num_ports: u32) {
        unsafe { ffi::gst_sdp_media_set_port_info(&mut self.0, port, num_ports) };
    }

    #[doc(alias = "gst_sdp_media_set_proto")]
    pub fn set_proto(&mut self, proto: &str) {
        unsafe { ffi::gst_sdp_media_set_proto(&mut self.0, proto.to_glib_none().0) };
    }

    #[doc(alias = "gst_sdp_media_set_media_from_caps")]
    pub fn set_media_from_caps(
        caps: &gst::CapsRef,
        media: &mut SDPMedia,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        let result = unsafe {
            ffi::gst_sdp_media_set_media_from_caps(caps.as_ptr(), media.to_glib_none_mut().0)
        };
        match result {
            ffi::GST_SDP_OK => Ok(()),
            _ => Err(glib::bool_error!("Failed to set media from caps")),
        }
    }
}

impl Borrow<SDPMediaRef> for SDPMedia {
    fn borrow(&self) -> &SDPMediaRef {
        self
    }
}

impl BorrowMut<SDPMediaRef> for SDPMedia {
    fn borrow_mut(&mut self) -> &mut SDPMediaRef {
        &mut *self
    }
}

impl ToOwned for SDPMediaRef {
    type Owned = SDPMedia;

    fn to_owned(&self) -> SDPMedia {
        unsafe {
            let mut ptr = ptr::null_mut();
            ffi::gst_sdp_media_copy(&self.0, &mut ptr);
            from_glib_full(ptr)
        }
    }
}

macro_rules! define_iter(
    ($name:ident, $typ:ty, $get_item:expr, $get_len:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        media: &'a SDPMediaRef,
        idx: usize,
        len: usize,
    }

    impl<'a> $name<'a> {
        fn new(media: &'a SDPMediaRef) -> $name<'a> {
            skip_assert_initialized!();
            let len = $get_len(media);

            $name {
                media,
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

            let item = $get_item(self.media, self.idx as u32).unwrap();
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
                Some($get_item(self.media, end as u32).unwrap())
            }
        }

        fn last(self) -> Option<Self::Item> {
            if self.idx == self.len {
                None
            } else {
                Some($get_item(self.media, self.len as u32 - 1).unwrap())
            }
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.len {
                return None;
            }

            self.len -= 1;

            Some($get_item(self.media, self.len as u32).unwrap())
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.len.overflowing_sub(n);
            if end <= self.idx || overflow {
                self.idx = self.len;
                None
            } else {
                self.len = end - 1;
                Some($get_item(self.media, self.len as u32).unwrap())
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}

    impl<'a> std::iter::FusedIterator for $name<'a> {}
    }
);

define_iter!(
    BandwidthsIter,
    &'a SDPBandwidth,
    |media: &'a SDPMediaRef, idx| media.bandwidth(idx),
    |media: &SDPMediaRef| media.bandwidths_len()
);
define_iter!(
    FormatsIter,
    &'a str,
    |media: &'a SDPMediaRef, idx| media.format(idx),
    |media: &SDPMediaRef| media.formats_len()
);
define_iter!(
    ConnectionsIter,
    &'a SDPConnection,
    |media: &'a SDPMediaRef, idx| media.connection(idx),
    |media: &SDPMediaRef| media.connections_len()
);
define_iter!(
    AttributesIter,
    &'a SDPAttribute,
    |media: &'a SDPMediaRef, idx| media.attribute(idx),
    |media: &SDPMediaRef| media.attributes_len()
);

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        gst::init().unwrap();
    }

    #[test]
    fn debug_impl() {
        init();

        let sdp = SDPMedia::new();
        assert!(!format!("{:?}", sdp).is_empty());
    }
}
