// Take a look at the license at the top of the repository in the LICENSE file.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[doc(alias = "gst_tag_register_musicbrainz_tags")]
pub fn tag_register_tags() {
    skip_assert_initialized!();
    unsafe { crate::ffi::gst_tag_register_musicbrainz_tags() };
}
