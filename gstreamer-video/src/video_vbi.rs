use crate::VideoFormat;
pub(super) const VBI_HD_MIN_PIXEL_WIDTH: u32 = 1280;

// rustdoc-stripper-ignore-next
/// Video Vertical Blanking Interval related Errors.
#[derive(thiserror::Error, Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoVBIError {
    #[error("Format and/or pixel_width is not supported")]
    Unsupported,

    #[error("Not enough space left in the current line")]
    NotEnoughSpace,

    #[error("Not enough data left in the current line")]
    NotEnoughData,

    #[error("Insufficient line buffer length {found}. Expected: {expected}")]
    InsufficientLineBufLen { found: usize, expected: usize },
}

// rustdoc-stripper-ignore-next
/// Returns the buffer length needed to store the line.
pub(super) fn line_buffer_len(format: VideoFormat, width: u32) -> usize {
    skip_assert_initialized!();
    // Taken from gst-plugins-base/gst-libs/gst/video/video-info.c:fill_planes
    match format {
        VideoFormat::V210 => ((width as usize + 47) / 48) * 128,
        VideoFormat::Uyvy => {
            // round up width to the next multiple of 4
            // FIXME: {integer}::next_multiple_of was stabilised in rustc 1.73.0
            ((width as usize * 2) + 3) & !3
        }
        _ => unreachable!(),
    }
}
