// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
// //
// // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// // option. This file may not be copied, modified, or distributed
// // except according to those terms.

use std::mem;
use std::ptr;

use ffi;
use glib::translate::*;
use glib_ffi;
use gst;

use auto::SDPResult;
use m_i_k_e_y_message::MIKEYMessage;
use s_d_p_attribute::SDPAttribute;
use s_d_p_bandwidth::SDPBandwidth;
use s_d_p_connection::SDPConnection;
use s_d_p_key::SDPKey;

glib_wrapper! {
    pub struct SDPMedia(Boxed<ffi::GstSDPMedia>);

    match fn {
        copy => |ptr| ffi::gst_sdp_media_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_sdp_media_free(ptr),
    }
}

impl SDPMedia {
    pub fn add_attribute<'a, P: Into<Option<&'a str>>>(&mut self, key: &str, value: P) -> Result<_, ()> {
        let value = value.into();
        let value = value.to_glib_none();
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_add_attribute(self.to_glib_none_mut().0, key.to_glib_none().0, value.0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_bandwidth(&mut self, bwtype: &str, bandwidth: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_add_bandwidth(self.to_glib_none_mut().0, bwtype.to_glib_none().0, bandwidth))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_connection(&mut self, nettype: &str, addrtype: &str, address: &str, ttl: u32, addr_number: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_add_connection(self.to_glib_none_mut().0, nettype.to_glib_none().0, addrtype.to_glib_none().0, address.to_glib_none().0, ttl, addr_number))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn add_format(&mut self, format: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_add_format(self.to_glib_none_mut().0, format.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn as_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_sdp_media_as_text(self.to_glib_none().0))
        }
    }

    pub fn attributes_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_attributes_len(self.to_glib_none().0)
        }
    }

    pub fn attributes_to_caps(&self, caps: &gst::Caps) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_attributes_to_caps(self.to_glib_none().0, caps.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn bandwidths_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_bandwidths_len(self.to_glib_none().0)
        }
    }

    pub fn connections_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_connections_len(self.to_glib_none().0)
        }
    }

    pub fn formats_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_formats_len(self.to_glib_none().0)
        }
    }

    pub fn get_attribute(&self, idx: u32) -> Option<SDPAttribute> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_attribute(self.to_glib_none().0, idx) )
        }
    }

    pub fn get_attribute_val(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_attribute_val(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_attribute_val_n(&self, key: &str, nth: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_attribute_val_n(self.to_glib_none().0, key.to_glib_none().0, nth))
        }
    }

    pub fn get_bandwidth(&self, idx: u32) -> Option<SDPBandwidth> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_bandwidth(self.to_glib_none().0, idx))
        }
    }

    pub fn get_caps_from_media(&self, pt: i32) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_sdp_media_get_caps_from_media(self.to_glib_none().0, pt))
        }
    }

    pub fn get_connection(&self, idx: u32) -> Option<SDPConnection> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_connection(self.to_glib_none().0, idx))
        }
    }

    pub fn get_format(&self, idx: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_format(self.to_glib_none().0, idx))
        }
    }

    pub fn get_information(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_information(self.to_glib_none().0))
        }
    }

    pub fn get_key(&self) -> Option<SDPKey> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_key(self.to_glib_none().0))
        }
    }

    pub fn get_media(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_media(self.to_glib_none().0))
        }
    }

    pub fn get_num_ports(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_get_num_ports(self.to_glib_none().0)
        }
    }

    pub fn get_port(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_get_port(self.to_glib_none().0)
        }
    }

    pub fn get_proto(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_proto(self.to_glib_none().0))
        }
    }

    pub fn init(&mut self) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_init(self.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_attribute(&mut self, idx: i32, attr: &mut SDPAttribute) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_insert_attribute(self.to_glib_none_mut().0, idx, attr.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_bandwidth(&mut self, idx: i32, bw: &mut SDPBandwidth) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_insert_bandwidth(self.to_glib_none_mut().0, idx, self.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_connection(&mut self, idx: i32, conn: &mut SDPConnection) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_insert_connection(self.to_glib_none_mut().0, idx, conn.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn insert_format(&mut self, idx: i32, format: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_insert_format(self.to_glib_none_mut().0, idx, format.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    #[cfg(any(feature = "v1_8_1", feature = "dox"))]
    pub fn parse_keymgmt(&self) -> Result<MIKEYMessage, SDPResult> {
        unsafe {
            let mut mikey = ptr::null_mut();
            let result = from_glib(ffi::gst_sdp_media_parse_keymgmt(self.to_glib_none().0, &mut mikey));
            match result {
                SDPResult::ok => Some(from_glib_full(mikey)),
                _ => Err(result),
            }
        }
    }

    pub fn remove_attribute(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_remove_attribute(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn remove_bandwidth(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_remove_bandwidth(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn remove_connection(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_remove_connection(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn remove_format(&mut self, idx: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_remove_format(self.to_glib_none_mut().0, idx))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_attribute(&mut self, idx: u32, attr: &mut SDPAttribute) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_replace_attribute(self.to_glib_none_mut().0, idx, attr.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_bandwidth(&mut self, idx: u32, bw: &mut SDPBandwidth) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_replace_bandwidth(self.to_glib_none_mut().0, idx, bw.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_connection(&mut self, idx: u32, conn: &mut SDPConnection) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_replace_connection(self.to_glib_none_mut().0, idx, conn.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn replace_format(&mut self, idx: u32, format: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_replace_format(self.to_glib_none_mut().0, idx, format.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_information(&mut self, information: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_set_information(self.to_glib_none_mut().0, information.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_key(&mut self, type_: &str, data: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_set_key(self.to_glib_none_mut().0, type_.to_glib_none().0, data.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_media(&mut self, med: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_set_media(self.to_glib_none_mut().0, med.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_port_info(&mut self, port: u32, num_ports: u32) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_set_port_info(self.to_glib_none_mut().0, port, num_ports))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn set_proto(&mut self, proto: &str) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_set_proto(self.to_glib_none_mut().0, proto.to_glib_none().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn uninit(&mut self) -> Result<_, ()> {
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_uninit(self.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }

    pub fn new() -> (SDPResult, SDPMedia) {
        assert_initialized_main_thread!();
        unsafe {
            let mut media = ptr::null_mut();
            let ret = from_glib(ffi::gst_sdp_media_new(&mut media));
            (ret, from_glib_full(media))
        }
    }

    pub fn set_media_from_caps(caps: &gst::Caps, media: &mut SDPMedia) -> Result<_, ()> {
        assert_initialized_main_thread!();
        let result = unsafe {
            from_glib(ffi::gst_sdp_media_set_media_from_caps(caps.to_glib_none().0, media.to_glib_none_mut().0))
        };
        match result {
            SDPResult::ok => Some(_),
            _ => Err(()),
            }
        }
    }
}

impl Default for SDPMedia {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for SDPMedia {}
