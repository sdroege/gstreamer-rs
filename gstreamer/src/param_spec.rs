// Take a look at the license at the top of the repository in the LICENSE file.

use glib::gobject_ffi;
use glib::translate::*;
use glib::ParamSpec;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstParamSpecFraction")]
    pub struct ParamSpecFraction(Shared<ffi::GstParamSpecFraction>);

    match fn {
        ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr as *mut gobject_ffi::GParamSpec),
        unref => |ptr| gobject_ffi::g_param_spec_unref(ptr as *mut gobject_ffi::GParamSpec),
        type_ => || ffi::gst_param_spec_fraction_get_type(),
    }
}

unsafe impl Send for ParamSpecFraction {}
unsafe impl Sync for ParamSpecFraction {}

impl std::ops::Deref for ParamSpecFraction {
    type Target = ParamSpec;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ParamSpecFraction as *const ParamSpec) }
    }
}

unsafe impl glib::ParamSpecType for ParamSpecFraction {}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for ParamSpecFraction {
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
        from_glib_full(ptr as *mut ffi::GstParamSpecFraction)
    }
}

impl ParamSpecFraction {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "gst_param_spec_fraction")]
    pub fn new(
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
            from_glib_none(ffi::gst_param_spec_fraction(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                min.numer(),
                min.denom(),
                max.numer(),
                max.denom(),
                default.numer(),
                default.denom(),
                flags.into_glib(),
            ))
        }
    }

    pub fn minimum(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.to_glib_none().0;

            crate::Fraction::new((*ptr).min_num, (*ptr).min_den)
        }
    }

    pub fn maximum(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.to_glib_none().0;

            crate::Fraction::new((*ptr).max_num, (*ptr).max_den)
        }
    }

    pub fn default_value(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.to_glib_none().0;

            crate::Fraction::new((*ptr).def_num, (*ptr).def_den)
        }
    }

    pub fn upcast(self) -> ParamSpec {
        unsafe { from_glib_full(self.to_glib_full() as *mut gobject_ffi::GParamSpec) }
    }

    pub fn upcast_ref(&self) -> &ParamSpec {
        &*self
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstParamSpecArray")]
    pub struct ParamSpecArray(Shared<ffi::GstParamSpecArray>);

    match fn {
        ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr as *mut gobject_ffi::GParamSpec),
        unref => |ptr| gobject_ffi::g_param_spec_unref(ptr as *mut gobject_ffi::GParamSpec),
        type_ => || ffi::gst_param_spec_fraction_get_type(),
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
unsafe impl Send for ParamSpecArray {}
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
unsafe impl Sync for ParamSpecArray {}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
impl std::ops::Deref for ParamSpecArray {
    type Target = ParamSpec;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ParamSpecArray as *const ParamSpec) }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
unsafe impl glib::ParamSpecType for ParamSpecArray {}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for ParamSpecArray {
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
        from_glib_full(ptr as *mut ffi::GstParamSpecArray)
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
impl ParamSpecArray {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "gst_param_spec_array")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: Option<&glib::ParamSpec>,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_param_spec_array(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    pub fn element_spec(&self) -> Option<ParamSpec> {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib_none((*ptr).element_spec)
        }
    }

    pub fn upcast(self) -> ParamSpec {
        unsafe { from_glib_full(self.to_glib_full() as *mut gobject_ffi::GParamSpec) }
    }

    pub fn upcast_ref(&self) -> &ParamSpec {
        &*self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait() {
        crate::init().unwrap();

        let _pspec = ParamSpecFraction::new(
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
