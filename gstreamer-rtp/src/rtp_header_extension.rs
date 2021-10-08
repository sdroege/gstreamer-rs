use crate::RTPHeaderExtension;
use crate::RTPHeaderExtensionFlags;
use glib::object::IsA;
use glib::translate::*;

pub trait RTPHeaderExtensionExtManual: 'static {
    #[doc(alias = "gst_rtp_header_extension_read")]
    fn read(
        &self,
        read_flags: RTPHeaderExtensionFlags,
        data: &[u8],
        buffer: &mut gst::BufferRef,
    ) -> bool;

    #[doc(alias = "gst_rtp_header_extension_write")]
    fn write(
        &self,
        input_meta: &gst::Buffer,
        write_flags: RTPHeaderExtensionFlags,
        output: &mut gst::BufferRef,
        data: &mut [u8],
    ) -> Result<usize, glib::BoolError>;

    #[doc(alias = "gst_rtp_header_extension_set_caps_from_attributes")]
    fn set_caps_from_attributes(&self, caps: &mut gst::CapsRef) -> bool;

    #[doc(alias = "gst_rtp_header_extension_set_caps_from_attributes_helper")]
    fn set_caps_from_attributes_helper(&self, caps: &mut gst::CapsRef, attributes: &str) -> bool;

    #[doc(alias = "gst_rtp_header_extension_update_non_rtp_src_caps")]
    fn update_non_rtp_src_caps(&self, caps: &mut gst::CapsRef) -> bool;
}

impl<O: IsA<RTPHeaderExtension>> RTPHeaderExtensionExtManual for O {
    fn read(
        &self,
        read_flags: RTPHeaderExtensionFlags,
        data: &[u8],
        buffer: &mut gst::BufferRef,
    ) -> bool {
        let size = data.len() as usize;
        unsafe {
            from_glib(ffi::gst_rtp_header_extension_read(
                self.as_ref().to_glib_none().0,
                read_flags.into_glib(),
                data.to_glib_none().0,
                size,
                buffer.as_mut_ptr(),
            ))
        }
    }

    fn write(
        &self,
        input_meta: &gst::Buffer,
        write_flags: RTPHeaderExtensionFlags,
        output: &mut gst::BufferRef,
        data: &mut [u8],
    ) -> Result<usize, glib::BoolError> {
        let size = data.len() as usize;
        unsafe {
            let res = ffi::gst_rtp_header_extension_write(
                self.as_ref().to_glib_none().0,
                input_meta.to_glib_none().0,
                write_flags.into_glib(),
                output.as_mut_ptr(),
                data.to_glib_none().0,
                size,
            );

            if res < 0 {
                Err(glib::bool_error!("Failed to write header extension"))
            } else {
                Ok(res as usize)
            }
        }
    }

    fn set_caps_from_attributes(&self, caps: &mut gst::CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_header_extension_set_caps_from_attributes(
                self.as_ref().to_glib_none().0,
                caps.as_mut_ptr(),
            ))
        }
    }

    fn set_caps_from_attributes_helper(&self, caps: &mut gst::CapsRef, attributes: &str) -> bool {
        unsafe {
            from_glib(
                ffi::gst_rtp_header_extension_set_caps_from_attributes_helper(
                    self.as_ref().to_glib_none().0,
                    caps.as_mut_ptr(),
                    attributes.to_glib_none().0,
                ),
            )
        }
    }

    fn update_non_rtp_src_caps(&self, caps: &mut gst::CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_header_extension_update_non_rtp_src_caps(
                self.as_ref().to_glib_none().0,
                caps.as_mut_ptr(),
            ))
        }
    }
}
