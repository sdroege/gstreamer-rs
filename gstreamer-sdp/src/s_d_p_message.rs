// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib_ffi;

use auto::SDPResult;
use m_i_k_e_y_message::MikeyMessage;
use s_d_p_media::SDPMedia;
use s_d_p_attribute::SDPAttribute;
use s_d_p_bandwidth::SDPBandwidth;
use s_d_p_connection::SDPConnection;
use s_d_p_key::SDPKey;
use s_d_p_origin::SDPOrigin;
use s_d_p_time::SDPTime;
use s_d_p_zone::SDPZone;

glib_wrapper! {
    pub struct SDPMessage(Boxed<ffi::GstSDPMessage>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::gst_sdp_message_get_type(), ptr as *mut _) as *mut ffi::GstSDPMessage,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::gst_sdp_message_get_type(), ptr as *mut _),
        get_type => || ffi::gst_sdp_message_get_type(),
    }
}

impl SDPMessage {
    pub fn new() -> SDPMessage {
        assert_initialized_main_thread!();
        unsafe {
            let msg = glib_ffi::g_malloc(mem::size_of::<ffi::GstSDPMessage>());
            ffi::gst_sdp_message_new(msg);
            from_glib_full(msg)
        }
    }

    pub fn add_attribute<'a, P: Into<Option<&'a str>>>(&mut self, key: &str, value: P) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_add_attribute(self.to_glib_none_mut().0, key.to_glib_none().0, value.into().to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_bandwidth(&mut self, bwtype: &str, bandwidth: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_add_bandwidth(self.to_glib_none_mut().0, bwtype.to_glib_none().0, bandwidth))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_email(&mut self, email: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_add_email(self.to_glib_none_mut().0, email.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_media(&mut self, media: SDPMedia) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_add_media(self.to_glib_none_mut().0, media.to_glib_full()))
        };
        mem::forget(media);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_phone(&mut self, phone: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_add_phone(self.to_glib_none_mut().0, phone.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_time(&mut self, start: &str, stop: &str, repeat: &[&str]) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_add_time(self.to_glib_none_mut().0, start.to_glib_none().0, stop.to_glib_none().0, repeat.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_zone(&mut self, adj_time: &str, typed_time: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_add_zone(self.to_glib_none_mut().0, adj_time.to_glib_none().0, typed_time.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn as_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_sdp_message_as_text(self.to_glib_none().0))
        }
    }

    pub fn attributes_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_message_attributes_len(self.to_glib_none().0)
        }
    }

    pub fn attributes_to_caps(&self, caps: &mut gst::CapsRef) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_attributes_to_caps(self.to_glib_none().0, caps.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn bandwidths_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_message_bandwidths_len(self.to_glib_none().0)
        }
    }

    pub fn dump(&self) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_dump(self.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn emails_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_message_emails_len(self.to_glib_none().0)
        }
    }

    pub fn get_attribute(&self, idx: u32) -> Option<SDPAttribute> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_attribute(self.to_glib_none().0, idx))
        }
    }

    pub fn get_attribute_val(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_attribute_val(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_attribute_val_n(&self, key: &str, nth: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_attribute_val_n(self.to_glib_none().0, key.to_glib_none().0, nth))
        }
    }

    pub fn get_bandwidth(&self, idx: u32) -> Option<SDPBandwidth> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_bandwidth())
        }
    }

    pub fn get_connection(&self) -> Option<SDPConnection> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_connection())
        }
    }

    pub fn get_email(&self, idx: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_email(self.to_glib_none().0, idx))
        }
    }

    pub fn get_information(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_information(self.to_glib_none().0))
        }
    }

    pub fn get_key(&self) -> Option<SDPKey> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_key())
        }
    }

    pub fn get_media(&self, idx: u32) -> Option<SDPMedia> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_media(self.to_glib_none().0, idx))
        }
    }

    pub fn get_origin(&self) -> Option<SDPOrigin> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_origin())
        }
    }

    pub fn get_phone(&self, idx: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_phone(self.to_glib_none().0, idx))
        }
    }

    pub fn get_session_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_session_name(self.to_glib_none().0))
        }
    }

    pub fn get_time(&self, idx: u32) -> Option<SDPTime> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_time())
        }
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_uri(self.to_glib_none().0))
        }
    }

    pub fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_version(self.to_glib_none().0))
        }
    }

    pub fn get_zone(&self, idx: u32) -> Option<SDPZone> {
        unsafe {
            from_glib_none(ffi::gst_sdp_message_get_zone())
        }
    }

    pub fn insert_attribute(&mut self, idx: i32, attr: SDPAttribute) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_insert_attribute(self.to_glib_none_mut().0, idx, attr.to_glib_full()))
        };
        mem::forget(attr);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_bandwidth(&mut self, idx: i32, bw: SDPBandwidth) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_insert_bandwidth(self.to_glib_none_mut().0, idx, bw.to_glib_full()))
        };
        mem::forget(bw);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_email(&mut self, idx: i32, email: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_insert_email(self.to_glib_none_mut().0, idx, email.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_phone(&mut self, idx: i32, phone: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_insert_phone(self.to_glib_none_mut().0, idx, phone.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_time(&mut self, idx: i32, time: SDPTime) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_insert_time(self.to_glib_none_mut().0, idx, time.to_glib_full()))
        };
        mem::forget(time);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_zone(&mut self, idx: i32, zone: SDPZone) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_insert_zone(self.to_glib_none_mut().0, idx, zone.to_glib_full()))
        };
        mem:forget(zone);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn medias_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_message_medias_len(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_8_1", feature = "dox"))]
    pub fn parse_keymgmt(&self, mikey: MIKEYMessage) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_parse_keymgmt(self.to_glib_none().0, &mut mikey))
        };
        mem::forget(mikey);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn phones_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_message_phones_len(self.to_glib_none().0)
        }
    }

    pub fn remove_attribute(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_remove_attribute(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn remove_bandwidth(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_remove_bandwidth(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn remove_email(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_remove_email(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn remove_phone(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_remove_phone(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn remove_time(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_remove_time(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn remove_zone(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_remove_zone(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_attribute(&mut self, idx: u32, attr: SDPAttribute) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_replace_attribute(self.to_glib_none_mut().0, idx, attr.to_glib_full()))
        };
        mem::forget(attr);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_bandwidth(&mut self, idx: u32, bw: SDPBandwidth) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_replace_bandwidth(self.to_glib_none_mut().0, idx, bw.to_glib_full()))
        };
        mem::forget(bw);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_email(&mut self, idx: u32, email: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_replace_email(self.to_glib_none_mut().0, idx, email.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_phone(&mut self, idx: u32, phone: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_replace_phone(self.to_glib_none_mut().0, idx, phone.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_time(&mut self, idx: u32, time: SDPTime) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_replace_time(self.to_glib_none_mut().0, idx, time.to_glib_full()))
        };
        mem::forget(time);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_zone(&mut self, idx: u32, zone: SDPZone) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_replace_zone(self.to_glib_none_mut().0, idx, zone.to_glib_full()))
        };
        mem::forget(zone);
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_connection(&mut self, nettype: &str, addrtype: &str, address: &str, ttl: u32, addr_number: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_set_connection(self.to_glib_none_mut().0, nettype.to_glib_none().0, addrtype.to_glib_none().0, address.to_glib_none().0, ttl, addr_number))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_information(&mut self, information: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_set_information(self.to_glib_none_mut().0, information.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_key(&mut self, type_: &str, data: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_set_key(self.to_glib_none_mut().0, type_.to_glib_none().0, data.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_origin(&mut self, username: &str, sess_id: &str, sess_version: &str, nettype: &str, addrtype: &str, addr: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_set_origin(self.to_glib_none_mut().0, username.to_glib_none().0, sess_id.to_glib_none().0, sess_version.to_glib_none().0, nettype.to_glib_none().0, addrtype.to_glib_none().0, addr.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_session_name(&mut self, session_name: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_set_session_name(self.to_glib_none_mut().0, session_name.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_uri(&mut self, uri: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_set_uri(self.to_glib_none_mut().0, uri.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_version(&mut self, version: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_message_set_version(self.to_glib_none_mut().0, version.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn times_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_message_times_len(self.to_glib_none().0)
        }
    }

    pub fn zones_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_message_zones_len(self.to_glib_none().0)
        }
    }

    pub fn as_uri(scheme: &str, msg: &SDPMessage) -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_sdp_message_as_uri(scheme.to_glib_none().0, msg.to_glib_none().0))
        }
    }

    pub fn parse_buffer(data: &[u8]) -> Result<Self, SDPResult> {
        assert_initialized_main_thread!();
        unsafe {
            let size = data.len() as u32;
            let mut msg = glib_ffi::g_malloc(mem::size_of::<ffi::GstSDPMessage>());
            let result = from_glib(ffi::gst_sdp_message_parse_buffer(data.to_glib_none().0, size, &mut msg));
            match result {
                SDPResult::ok => Some(from_glib_full(msg)),
                _ => Err(result),
            }
        }
    }

    pub fn parse_uri(uri: &str, msg: &mut SDPMessage) -> Result<Self, SDPResult> {
        assert_initialized_main_thread!();
        unsafe {
            let mut msg = glib_ffi::g_malloc(mem::size_of::<ffi::GstSDPMessage>());
            let result = from_glib(ffi::gst_sdp_message_parse_uri(uri.to_glib_none().0, &mut msg));
            match result {
                SDPResult::ok => Some(from_glib_full(msg)),
                _ => Err(result),
            }
        }
    }
}

unsafe impl Send for SDPMessage {}
