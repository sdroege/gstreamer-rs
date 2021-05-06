// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

pub trait GstParamSpecExt {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn new_array(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: &glib::ParamSpec,
        flags: glib::ParamFlags,
    ) -> Self;

    fn new_fraction(
        name: &str,
        nick: &str,
        blurb: &str,
        min: crate::Fraction,
        max: crate::Fraction,
        default: crate::Fraction,
        flags: glib::ParamFlags,
    ) -> Self;
}

impl GstParamSpecExt for glib::ParamSpec {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn new_array(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: &glib::ParamSpec,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_param_spec_array(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    fn new_fraction(
        name: &str,
        nick: &str,
        blurb: &str,
        min: crate::Fraction,
        max: crate::Fraction,
        default: crate::Fraction,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_param_spec_fraction(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                *min.numer(),
                *min.denom(),
                *max.numer(),
                *max.denom(),
                *default.numer(),
                *default.denom(),
                flags.into_glib(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_trait() {
        crate::init().unwrap();

        let _pspec = glib::ParamSpec::new_fraction(
            "foo",
            "Foo",
            "Foo Bar",
            (0, 1).into(),
            (100, 1).into(),
            (1, 1).into(),
            glib::ParamFlags::READWRITE,
        );
    }
}
