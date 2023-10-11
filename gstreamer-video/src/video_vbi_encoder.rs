// Take a look at the license at the top of the repository in the LICENSE file.

use crate::VideoFormat;
use glib::translate::*;

use crate::video_vbi::line_buffer_len;
use crate::{VideoAncillaryDID, VideoAncillaryDID16, VideoVBIError, VBI_HD_MIN_PIXEL_WIDTH};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct VideoVBIEncoderInner(Boxed<ffi::GstVideoVBIEncoder>);

    match fn {
        copy => |ptr| ffi::gst_video_vbi_encoder_copy(ptr),
        free => |ptr| ffi::gst_video_vbi_encoder_free(ptr),
        type_ => || ffi::gst_video_vbi_encoder_get_type(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoVBIEncoder {
    inner: VideoVBIEncoderInner,
    format: VideoFormat,
    pixel_width: u32,
    line_buffer_len: usize,
    anc_len: usize,
}

unsafe impl Send for VideoVBIEncoder {}
unsafe impl Sync for VideoVBIEncoder {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoAFDDescriptionMode {
    Composite,
    Component,
}

impl VideoAFDDescriptionMode {
    pub fn is_composite(&self) -> bool {
        matches!(self, VideoAFDDescriptionMode::Composite)
    }

    pub fn is_component(&self) -> bool {
        matches!(self, VideoAFDDescriptionMode::Component)
    }
}

impl VideoVBIEncoder {
    #[doc(alias = "gst_video_vbi_encoder_new")]
    pub fn try_new(
        format: VideoFormat,
        pixel_width: u32,
    ) -> Result<VideoVBIEncoder, VideoVBIError> {
        skip_assert_initialized!();
        let res: Option<VideoVBIEncoderInner> = unsafe {
            from_glib_full(ffi::gst_video_vbi_encoder_new(
                format.into_glib(),
                pixel_width,
            ))
        };

        Ok(VideoVBIEncoder {
            inner: res.ok_or(VideoVBIError::Unsupported)?,
            format,
            pixel_width,
            line_buffer_len: line_buffer_len(format, pixel_width),
            anc_len: 0,
        })
    }

    // rustdoc-stripper-ignore-next
    /// Adds the provided ancillary data as a DID and block number AFD.
    pub fn add_did_ancillary(
        &mut self,
        adf_mode: VideoAFDDescriptionMode,
        did: VideoAncillaryDID,
        block_number: u8,
        data: &[u8],
    ) -> Result<(), VideoVBIError> {
        self.add_ancillary(adf_mode, did.into_glib() as u8, block_number, data)
    }

    // rustdoc-stripper-ignore-next
    /// Adds the provided ancillary data as a DID16 (DID & SDID) AFD.
    pub fn add_did16_ancillary(
        &mut self,
        adf_mode: VideoAFDDescriptionMode,
        did16: VideoAncillaryDID16,
        data: &[u8],
    ) -> Result<(), VideoVBIError> {
        let did16 = did16.into_glib();

        self.add_ancillary(
            adf_mode,
            ((did16 & 0xff00) >> 8) as u8,
            (did16 & 0xff) as u8,
            data,
        )
    }

    #[doc(alias = "gst_video_vbi_encoder_add_ancillary")]
    pub fn add_ancillary(
        &mut self,
        adf_mode: VideoAFDDescriptionMode,
        did: u8,
        sdid_block_number: u8,
        data: &[u8],
    ) -> Result<(), VideoVBIError> {
        let data_count = data.len() as _;
        let res: bool = unsafe {
            from_glib(ffi::gst_video_vbi_encoder_add_ancillary(
                self.inner.to_glib_none_mut().0,
                adf_mode.is_composite().into_glib(),
                did,
                sdid_block_number,
                data.to_glib_none().0,
                data_count,
            ))
        };

        if !res {
            return Err(VideoVBIError::NotEnoughSpace);
        }

        // AFD: 1 byte (+2 if component)
        // DID + SDID_block_number + Data Count: 3 bytes
        // DATA: data_count bytes
        // Checksum: 1 byte
        let mut len = 1 + 3 + (data_count as usize) + 1;
        if adf_mode.is_component() {
            len += 2;
        }

        if matches!(self.format, VideoFormat::V210) {
            // 10bits payload on 16bits for now: will be packed when writing the line
            len *= 2;
        }

        self.anc_len += len;

        Ok(())
    }

    // rustdoc-stripper-ignore-next
    /// Returns the buffer length needed to store the line.
    pub fn line_buffer_len(&self) -> usize {
        self.line_buffer_len
    }

    // rustdoc-stripper-ignore-next
    /// Writes the ancillaries encoded for VBI to the provided buffer.
    ///
    /// Use [`Self::line_buffer_len`] to get the expected buffer length.
    ///
    /// Resets the internal state, so this [`VideoVBIEncoder`] can be reused for
    /// subsequent VBI encodings.
    ///
    /// # Returns
    ///
    /// - `Ok` with the written length in bytes in the line buffer containing the encoded
    ///   ancilliaries previously added using [`VideoVBIEncoder::add_ancillary`],
    ///   [`VideoVBIEncoder::add_did_ancillary`] or [`VideoVBIEncoder::add_did16_ancillary`].
    /// - `Err` if the ancillary could not be added.
    #[doc(alias = "gst_video_vbi_encoder_write_line")]
    pub fn write_line(&mut self, data: &mut [u8]) -> Result<usize, VideoVBIError> {
        if data.len() < self.line_buffer_len {
            return Err(VideoVBIError::InsufficientLineBufLen {
                found: data.len(),
                expected: self.line_buffer_len,
            });
        }

        unsafe {
            let dest = data.as_mut_ptr();
            ffi::gst_video_vbi_encoder_write_line(self.inner.to_glib_none_mut().0, dest);
        }

        let mut anc_len = std::mem::take(&mut self.anc_len);
        match self.format {
            VideoFormat::V210 => {
                // Anc data consists in 10bits stored in 16bits word
                let word_count = anc_len / 2;

                if self.pixel_width < VBI_HD_MIN_PIXEL_WIDTH {
                    // SD: Packs 12x 10bits data in 4x 32bits word
                    anc_len =
                        4 * 4 * ((word_count / 12) + if word_count % 12 == 0 { 0 } else { 1 });
                } else {
                    // HD: Packs 3x 10bits data in 1x 32bits word interleaving UV and Y components
                    //     (where Y starts at buffer offset 0 and UV starts at buffer offset pixel_width)
                    //     so we get 6 (uv,y) pairs every 4x 32bits word in the resulting line
                    // FIXME: {integer}::div_ceil was stabilised in rustc 1.73.0
                    let pair_count = usize::min(word_count, self.pixel_width as usize);
                    anc_len = 4 * 4 * ((pair_count / 6) + if pair_count % 6 == 0 { 0 } else { 1 });
                }
            }
            VideoFormat::Uyvy => {
                // Anc data stored as bytes

                if self.pixel_width < VBI_HD_MIN_PIXEL_WIDTH {
                    // SD: Stores 4x bytes in 4x bytes let's keep 32 bits alignment
                    anc_len = 4 * ((anc_len / 4) + if anc_len % 4 == 0 { 0 } else { 1 });
                } else {
                    // HD: Stores 4x bytes in 4x bytes interleaving UV and Y components
                    //     (where Y starts at buffer offset 0 and UV starts at buffer offset pixel_width)
                    //     so we get 2 (uv,y) pairs every 4x bytes in the resulting line
                    // let's keep 32 bits alignment
                    // FIXME: {integer}::div_ceil was stabilised in rustc 1.73.0
                    let pair_count = usize::min(anc_len, self.pixel_width as usize);
                    anc_len = 4 * ((pair_count / 2) + if pair_count % 2 == 0 { 0 } else { 1 });
                }
            }
            _ => unreachable!(),
        }

        assert!(anc_len < self.line_buffer_len);

        Ok(anc_len)
    }
}

impl<'a> TryFrom<&'a crate::VideoInfo> for VideoVBIEncoder {
    type Error = VideoVBIError;

    fn try_from(info: &'a crate::VideoInfo) -> Result<VideoVBIEncoder, VideoVBIError> {
        skip_assert_initialized!();
        VideoVBIEncoder::try_new(info.format(), info.width())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cea608_component() {
        let mut encoder =
            VideoVBIEncoder::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Component,
                VideoAncillaryDID16::S334Eia608,
                &[0x80, 0x94, 0x2c],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(32, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [
                0x00, 0x00, 0x00, 0x00, 0xff, 0x03, 0xf0, 0x3f, 0x00, 0x84, 0x05, 0x00, 0x02, 0x01,
                0x30, 0x20, 0x00, 0x00, 0x06, 0x00, 0x94, 0x01, 0xc0, 0x12, 0x00, 0x98, 0x0a, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    fn cea608_component_sd() {
        let mut encoder = VideoVBIEncoder::try_new(VideoFormat::V210, 768).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Component,
                VideoAncillaryDID16::S334Eia608,
                &[0x80, 0x94, 0x2c],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(16, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [
                0x00, 0xfc, 0xff, 0x3f, 0x61, 0x09, 0x34, 0x20, 0x80, 0x51, 0xc6, 0x12, 0xa6, 0x02,
                0x00, 0x00
            ]
        );
    }

    #[test]
    fn cea608_composite() {
        let mut encoder =
            VideoVBIEncoder::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Composite,
                VideoAncillaryDID16::S334Eia608,
                &[0x15, 0x94, 0x2c],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(32, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [
                0x00, 0xf0, 0x0f, 0x00, 0x61, 0x01, 0x20, 0x10, 0x00, 0x0c, 0x08, 0x00, 0x15, 0x01,
                0x40, 0x19, 0x00, 0xb0, 0x04, 0x00, 0x3b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    fn cea608_composite_sd() {
        let mut encoder = VideoVBIEncoder::try_new(VideoFormat::V210, 768).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Composite,
                VideoAncillaryDID16::S334Eia608,
                &[0x15, 0x94, 0x2c],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(16, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [
                0xfc, 0x87, 0x25, 0x10, 0x03, 0x56, 0x44, 0x19, 0x2c, 0xed, 0x08, 0x00, 0x00, 0x00,
                0x00, 0x00
            ]
        );
    }

    #[test]
    fn cea608_component_uyvy() {
        let mut encoder =
            VideoVBIEncoder::try_new(VideoFormat::Uyvy, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Component,
                VideoAncillaryDID16::S334Eia608,
                &[0x80, 0x94, 0x2c],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(20, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [
                0x00, 0x00, 0x00, 0xff, 0x00, 0xff, 0x00, 0x61, 0x00, 0x02, 0x00, 0x03, 0x00, 0x80,
                0x00, 0x94, 0x00, 0x2c, 0x00, 0xa6
            ]
        );
    }

    #[test]
    fn cea608_component_sd_uyvy() {
        let mut encoder = VideoVBIEncoder::try_new(VideoFormat::Uyvy, 768).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Component,
                VideoAncillaryDID16::S334Eia608,
                &[0x80, 0x94, 0x2c],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(12, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [0x00, 0xff, 0xff, 0x61, 0x02, 0x03, 0x80, 0x94, 0x2c, 0xa6, 0x00, 0x00]
        );
    }

    #[test]
    fn cea608_composite_uyvy() {
        let mut encoder =
            VideoVBIEncoder::try_new(VideoFormat::Uyvy, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Composite,
                VideoAncillaryDID16::S334Eia608,
                &[0x15, 0x94, 0x2c],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(16, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [
                0x00, 0xfc, 0x00, 0x61, 0x00, 0x02, 0x00, 0x03, 0x00, 0x15, 0x00, 0x94, 0x00, 0x2c,
                0x00, 0x3b
            ]
        );
    }

    #[test]
    fn cea608_composite_sd_uyvy() {
        let mut encoder = VideoVBIEncoder::try_new(VideoFormat::Uyvy, 768).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Composite,
                VideoAncillaryDID16::S334Eia608,
                &[0x15, 0x94, 0x2c],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(8, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [0xfc, 0x61, 0x02, 0x03, 0x15, 0x94, 0x2c, 0x3b]
        );
    }

    #[test]
    fn insufficient_line_buf_len() {
        let mut encoder =
            VideoVBIEncoder::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Component,
                VideoAncillaryDID16::S334Eia608,
                &[0x80, 0x94, 0x2c],
            )
            .unwrap();
        let mut buf = vec![0; 10];
        assert_eq!(
            encoder.write_line(buf.as_mut_slice()).unwrap_err(),
            VideoVBIError::InsufficientLineBufLen {
                found: 10,
                expected: encoder.line_buffer_len()
            },
        );
    }

    #[test]
    fn cea708_component() {
        let mut encoder =
            VideoVBIEncoder::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Component,
                VideoAncillaryDID16::S334Eia708,
                &[
                    0x96, 0x69, 0x55, 0x3f, 0x43, 0x00, 0x00, 0x72, 0xf8, 0xfc, 0x94, 0x2c, 0xf9,
                    0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00,
                    0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                    0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                    0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00,
                    0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                    0xfa, 0x00, 0x00, 0x74, 0x00, 0x00, 0x1b,
                ],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(256, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [
                0x00, 0x00, 0x00, 0x00, 0xff, 0x03, 0xf0, 0x3f, 0x00, 0x84, 0x05, 0x00, 0x01, 0x01,
                0x50, 0x25, 0x00, 0x58, 0x0a, 0x00, 0x69, 0x02, 0x50, 0x25, 0x00, 0xfc, 0x08, 0x00,
                0x43, 0x01, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0x72, 0x02, 0x80, 0x1f, 0x00, 0xf0,
                0x0b, 0x00, 0x94, 0x01, 0xc0, 0x12, 0x00, 0xe4, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20,
                0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02,
                0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00,
                0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8,
                0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20,
                0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02,
                0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00,
                0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8,
                0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20,
                0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02,
                0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00,
                0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8,
                0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20,
                0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xd0, 0x09, 0x00, 0x00, 0x02,
                0x00, 0x20, 0x00, 0x6c, 0x08, 0x00, 0xb7, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00
            ]
        );
    }

    #[test]
    fn cea608_and_cea708_component() {
        let mut encoder =
            VideoVBIEncoder::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Component,
                VideoAncillaryDID16::S334Eia608,
                &[0x80, 0x94, 0x2c],
            )
            .unwrap();

        encoder
            .add_did16_ancillary(
                VideoAFDDescriptionMode::Component,
                VideoAncillaryDID16::S334Eia708,
                &[
                    0x96, 0x69, 0x55, 0x3f, 0x43, 0x00, 0x00, 0x72, 0xf8, 0xfc, 0x94, 0x2c, 0xf9,
                    0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00,
                    0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                    0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                    0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00,
                    0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                    0xfa, 0x00, 0x00, 0x74, 0x00, 0x00, 0x1b,
                ],
            )
            .unwrap();

        let mut buf = vec![0; encoder.line_buffer_len()];
        let anc_len = encoder.write_line(buf.as_mut_slice()).unwrap();
        assert_eq!(272, anc_len);
        assert_eq!(
            buf[0..anc_len],
            [
                0x00, 0x00, 0x00, 0x00, 0xff, 0x03, 0xf0, 0x3f, 0x00, 0x84, 0x05, 0x00, 0x02, 0x01,
                0x30, 0x20, 0x00, 0x00, 0x06, 0x00, 0x94, 0x01, 0xc0, 0x12, 0x00, 0x98, 0x0a, 0x00,
                0x00, 0x00, 0xf0, 0x3f, 0x00, 0xfc, 0x0f, 0x00, 0x61, 0x01, 0x10, 0x10, 0x00, 0x54,
                0x09, 0x00, 0x96, 0x02, 0x90, 0x26, 0x00, 0x54, 0x09, 0x00, 0x3f, 0x02, 0x30, 0x14,
                0x00, 0x00, 0x08, 0x00, 0x00, 0x02, 0x20, 0x27, 0x00, 0xe0, 0x07, 0x00, 0xfc, 0x02,
                0x40, 0x19, 0x00, 0xb0, 0x04, 0x00, 0xf9, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00,
                0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00,
                0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20,
                0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02,
                0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00,
                0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00,
                0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20,
                0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02,
                0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00,
                0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00,
                0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20,
                0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02,
                0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00,
                0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0x74, 0x02, 0x00, 0x20, 0x00, 0x00,
                0x08, 0x00, 0x1b, 0x02, 0x70, 0x2b
            ]
        );
    }
}
