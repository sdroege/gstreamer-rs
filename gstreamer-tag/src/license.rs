// Take a look at the license at the top of the repository in the LICENSE file.

pub use crate::TagLicenseFlags as LicenseFlags;

pub use crate::functions::{
    tag_get_license_description as description, tag_get_license_flags as flags,
    tag_get_license_jurisdiction as jurisdiction, tag_get_license_nick as nick,
    tag_get_license_title as title, tag_get_license_version as version,
    tag_get_licenses as all_licenses,
};
