// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{gobject_ffi, translate::*, ParamSpec};

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

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ParamSpecFraction as *const ParamSpec) }
    }
}

unsafe impl glib::ParamSpecType for ParamSpecFraction {}

impl glib::HasParamSpec for crate::Fraction {
    type ParamSpec = ParamSpecFraction;

    type SetValue = crate::Fraction;
    type BuilderFn = for<'a> fn(&'a str) -> ParamSpecFractionBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        ParamSpecFraction::builder
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for ParamSpecFraction {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
        from_glib_full(ptr as *mut ffi::GstParamSpecFraction)
    }
}

impl ParamSpecFraction {
    #[doc(alias = "gst_param_spec_fraction")]
    pub fn builder(name: &str) -> ParamSpecFractionBuilder {
        assert_initialized_main_thread!();
        ParamSpecFractionBuilder::new(name)
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        min: crate::Fraction,
        max: crate::Fraction,
        default: crate::Fraction,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        unsafe {
            from_glib_none(ffi::gst_param_spec_fraction(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
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

    #[inline]
    pub fn minimum(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.as_ptr();

            crate::Fraction::new((*ptr).min_num, (*ptr).min_den)
        }
    }

    #[inline]
    pub fn maximum(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.as_ptr();

            crate::Fraction::new((*ptr).max_num, (*ptr).max_den)
        }
    }

    #[inline]
    pub fn default_value(&self) -> crate::Fraction {
        unsafe {
            let ptr = self.as_ptr();

            crate::Fraction::new((*ptr).def_num, (*ptr).def_den)
        }
    }

    #[inline]
    pub fn upcast(self) -> ParamSpec {
        unsafe {
            from_glib_full(
                IntoGlibPtr::<*mut ffi::GstParamSpecFraction>::into_glib_ptr(self)
                    as *mut gobject_ffi::GParamSpec,
            )
        }
    }

    #[inline]
    pub fn upcast_ref(&self) -> &ParamSpec {
        self
    }
}

#[derive(Default)]
#[must_use]
pub struct ParamSpecFractionBuilder<'a> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: glib::ParamFlags,
    minimum: Option<crate::Fraction>,
    maximum: Option<crate::Fraction>,
    default_value: Option<crate::Fraction>,
}

impl<'a> ParamSpecFractionBuilder<'a> {
    fn new(name: &'a str) -> Self {
        assert_initialized_main_thread!();
        Self {
            name,
            ..Default::default()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Default: `-i32::MAX/1`
    pub fn minimum(mut self, minimum: crate::Fraction) -> Self {
        self.minimum = Some(minimum);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `i32::MAX/1`
    pub fn maximum(mut self, maximum: crate::Fraction) -> Self {
        self.maximum = Some(maximum);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Default: `0/1`
    pub fn default_value(mut self, default_value: crate::Fraction) -> Self {
        self.default_value = Some(default_value);
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecFraction::new_unchecked(
                self.name,
                self.nick.unwrap_or(self.name),
                self.blurb.unwrap_or(self.name),
                self.minimum
                    .unwrap_or_else(|| crate::Fraction::new(-i32::MAX, 1)),
                self.maximum
                    .unwrap_or_else(|| crate::Fraction::new(i32::MAX, 1)),
                self.default_value
                    .unwrap_or_else(|| crate::Fraction::new(0, 1)),
                self.flags,
            )
        }
    }
}

impl<'a> glib::prelude::ParamSpecBuilderExt<'a> for ParamSpecFractionBuilder<'a> {
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: glib::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> glib::ParamFlags {
        self.flags
    }
}

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstParamSpecArray")]
    pub struct ParamSpecArray(Shared<ffi::GstParamSpecArray>);

    match fn {
        ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr as *mut gobject_ffi::GParamSpec),
        unref => |ptr| gobject_ffi::g_param_spec_unref(ptr as *mut gobject_ffi::GParamSpec),
        type_ => || ffi::gst_param_spec_array_get_type(),
    }
}

unsafe impl Send for ParamSpecArray {}
unsafe impl Sync for ParamSpecArray {}

impl std::ops::Deref for ParamSpecArray {
    type Target = ParamSpec;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const ParamSpecArray as *const ParamSpec) }
    }
}

unsafe impl glib::ParamSpecType for ParamSpecArray {}

impl glib::HasParamSpec for crate::Array {
    type ParamSpec = ParamSpecArray;

    type SetValue = crate::Array;
    type BuilderFn = for<'a> fn(&'a str) -> ParamSpecArrayBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        ParamSpecArray::builder
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for ParamSpecArray {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
        from_glib_full(ptr as *mut ffi::GstParamSpecArray)
    }
}

impl ParamSpecArray {
    #[doc(alias = "gst_param_spec_array")]
    pub fn builder(name: &str) -> ParamSpecArrayBuilder {
        assert_initialized_main_thread!();
        ParamSpecArrayBuilder::new(name)
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        element_spec: Option<&glib::ParamSpec>,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        unsafe {
            from_glib_none(ffi::gst_param_spec_array(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[inline]
    pub fn element_spec(&self) -> Option<&ParamSpec> {
        unsafe {
            let ptr = self.as_ptr();

            if (*ptr).element_spec.is_null() {
                None
            } else {
                Some(
                    &*(&(*ptr).element_spec as *const *mut glib::gobject_ffi::GParamSpec
                        as *const glib::ParamSpec),
                )
            }
        }
    }

    #[inline]
    pub fn upcast(self) -> ParamSpec {
        unsafe {
            from_glib_full(
                IntoGlibPtr::<*mut ffi::GstParamSpecArray>::into_glib_ptr(self)
                    as *mut gobject_ffi::GParamSpec,
            )
        }
    }

    #[inline]
    pub fn upcast_ref(&self) -> &ParamSpec {
        self
    }
}

#[derive(Default)]
#[must_use]
pub struct ParamSpecArrayBuilder<'a> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: glib::ParamFlags,
    element_spec: Option<&'a glib::ParamSpec>,
}

impl<'a> ParamSpecArrayBuilder<'a> {
    fn new(name: &'a str) -> Self {
        assert_initialized_main_thread!();
        Self {
            name,
            ..Default::default()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Default: `None`
    pub fn element_spec(mut self, element_spec: impl Into<Option<&'a glib::ParamSpec>>) -> Self {
        self.element_spec = element_spec.into();
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecArray::new_unchecked(
                self.name,
                self.nick.unwrap_or(self.name),
                self.blurb.unwrap_or(self.name),
                self.element_spec,
                self.flags,
            )
        }
    }
}

impl<'a> glib::prelude::ParamSpecBuilderExt<'a> for ParamSpecArrayBuilder<'a> {
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: glib::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> glib::ParamFlags {
        self.flags
    }
}

pub trait GstParamSpecBuilderExt<'a>: glib::prelude::ParamSpecBuilderExt<'a> {
    // rustdoc-stripper-ignore-next
    /// Mark the property as controllable
    fn controllable(self) -> Self {
        let flags = self.current_flags() | crate::PARAM_FLAG_CONTROLLABLE;
        self.flags(flags)
    }

    // rustdoc-stripper-ignore-next
    /// Mark the property as mutable in ready state
    fn mutable_ready(self) -> Self {
        let flags = self.current_flags() | crate::PARAM_FLAG_MUTABLE_READY;
        self.flags(flags)
    }

    // rustdoc-stripper-ignore-next
    /// Mark the property as mutable in paused state
    fn mutable_paused(self) -> Self {
        let flags = self.current_flags() | crate::PARAM_FLAG_MUTABLE_PAUSED;
        self.flags(flags)
    }

    // rustdoc-stripper-ignore-next
    /// Mark the property as mutable in playing state
    fn mutable_playing(self) -> Self {
        let flags = self.current_flags() | crate::PARAM_FLAG_MUTABLE_PLAYING;
        self.flags(flags)
    }

    #[cfg(feature = "v1_18")]
    // rustdoc-stripper-ignore-next
    /// Mark the property for showing the default value in the docs
    fn doc_show_default(self) -> Self {
        let flags = self.current_flags() | crate::PARAM_FLAG_DOC_SHOW_DEFAULT;
        self.flags(flags)
    }

    #[cfg(feature = "v1_18")]
    // rustdoc-stripper-ignore-next
    /// Mark the property for being only conditionally available
    fn conditionally_available(self) -> Self {
        let flags = self.current_flags() | crate::PARAM_FLAG_CONDITIONALLY_AVAILABLE;
        self.flags(flags)
    }
}

impl<'a, T: glib::prelude::ParamSpecBuilderExt<'a>> GstParamSpecBuilderExt<'a> for T {}

#[cfg(test)]
mod tests {
    use glib::prelude::*;

    use super::*;

    #[test]
    fn test_trait() {
        crate::init().unwrap();

        let _pspec = ParamSpecFraction::builder("foo")
            .nick("Foo")
            .blurb("Foo Bar")
            .minimum((0, 1).into())
            .maximum((100, 1).into())
            .default_value((1, 1).into())
            .readwrite()
            .mutable_playing()
            .build();
    }
}
