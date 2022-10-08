use ffi::{GstGLFilter, GstGLFilterClass, GstGLMemory};
use gst::ffi::GstBuffer;

use super::prelude::*;
use crate::prelude::*;

use glib::translate::*;

use gst::{result_from_gboolean, Buffer, Caps, LoggableError, PadDirection, CAT_RUST};

use crate::GLFilter;
use crate::GLMemory;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GLFilterMode {
    Buffer,
    Texture,
}

pub trait GLFilterImpl: GLFilterImplExt + GLBaseFilterImpl {
    const MODE: GLFilterMode;
    // rustdoc-stripper-ignore-next
    /// Calls [`add_rgba_pad_templates`](ffi::gst_gl_filter_add_rgba_pad_templates)
    /// in [`GLFilter::class_init`] if [`true`].
    const ADD_RGBA_PAD_TEMPLATES: bool = true;

    fn set_caps(&self, incaps: &Caps, outcaps: &Caps) -> Result<(), LoggableError> {
        GLFilterImplExt::parent_set_caps(self, incaps, outcaps)
    }

    fn filter(&self, input: &Buffer, output: &Buffer) -> Result<(), LoggableError> {
        self.parent_filter(input, output)
    }

    fn filter_texture(&self, input: &GLMemory, output: &GLMemory) -> Result<(), LoggableError> {
        self.parent_filter_texture(input, output)
    }

    fn init_fbo(&self) -> Result<(), LoggableError> {
        self.parent_init_fbo()
    }

    fn transform_internal_caps(
        &self,
        direction: PadDirection,
        caps: &Caps,
        filter_caps: Option<&Caps>,
    ) -> Option<Caps> {
        self.parent_transform_internal_caps(direction, caps, filter_caps)
    }
}

pub trait GLFilterImplExt: ObjectSubclass {
    fn parent_set_caps(&self, incaps: &Caps, outcaps: &Caps) -> Result<(), LoggableError>;

    fn parent_filter(&self, input: &Buffer, output: &Buffer) -> Result<(), LoggableError>;

    fn parent_filter_texture(
        &self,
        input: &GLMemory,
        output: &GLMemory,
    ) -> Result<(), LoggableError>;

    fn parent_init_fbo(&self) -> Result<(), LoggableError>;

    fn parent_transform_internal_caps(
        &self,
        direction: PadDirection,
        caps: &Caps,
        filter_caps: Option<&Caps>,
    ) -> Option<Caps>;
}

impl<T: GLFilterImpl> GLFilterImplExt for T {
    fn parent_set_caps(&self, incaps: &Caps, outcaps: &Caps) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut GstGLFilterClass;

            (*parent_class)
                .set_caps
                .map(|f| {
                    result_from_gboolean!(
                        f(
                            self.instance()
                                .unsafe_cast_ref::<GLFilter>()
                                .to_glib_none()
                                .0,
                            incaps.to_glib_none().0,
                            outcaps.to_glib_none().0,
                        ),
                        CAT_RUST,
                        "Parent function `set_caps` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_filter(&self, input: &Buffer, output: &Buffer) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut GstGLFilterClass;

            (*parent_class)
                .filter
                .map(|f| {
                    result_from_gboolean!(
                        f(
                            self.instance()
                                .unsafe_cast_ref::<GLFilter>()
                                .to_glib_none()
                                .0,
                            input.to_glib_none().0,
                            output.to_glib_none().0,
                        ),
                        CAT_RUST,
                        "Parent function `filter` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_filter_texture(
        &self,
        input: &GLMemory,
        output: &GLMemory,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut GstGLFilterClass;

            (*parent_class)
                .filter_texture
                .map(|f| {
                    result_from_gboolean!(
                        f(
                            self.instance()
                                .unsafe_cast_ref::<GLFilter>()
                                .to_glib_none()
                                .0,
                            input.to_glib_none().0,
                            output.to_glib_none().0,
                        ),
                        CAT_RUST,
                        "Parent function `filter_texture` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_init_fbo(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut GstGLFilterClass;

            (*parent_class)
                .init_fbo
                .map(|f| {
                    result_from_gboolean!(
                        f(self
                            .instance()
                            .unsafe_cast_ref::<GLFilter>()
                            .to_glib_none()
                            .0),
                        CAT_RUST,
                        "Parent function `init_fbo` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }
    fn parent_transform_internal_caps(
        &self,
        direction: PadDirection,
        caps: &Caps,
        filter_caps: Option<&Caps>,
    ) -> Option<Caps> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut GstGLFilterClass;

            let f = (*parent_class)
                .transform_internal_caps
                .expect("Missing parent function `transform_internal_caps`");

            from_glib_full(f(
                self.instance()
                    .unsafe_cast_ref::<GLFilter>()
                    .to_glib_none()
                    .0,
                direction.into_glib(),
                caps.to_glib_none().0,
                filter_caps.to_glib_none().0,
            ))
        }
    }
}

unsafe impl<T: GLFilterImpl> IsSubclassable<T> for GLFilter {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.set_caps = Some(set_caps::<T>);
        klass.init_fbo = Some(init_fbo::<T>);
        klass.transform_internal_caps = Some(transform_internal_caps::<T>);

        match <T as GLFilterImpl>::MODE {
            GLFilterMode::Buffer => {
                klass.filter = Some(filter::<T>);
                klass.filter_texture = None;
            }
            GLFilterMode::Texture => {
                klass.filter = None;
                klass.filter_texture = Some(filter_texture::<T>);
            }
        }

        if <T as GLFilterImpl>::ADD_RGBA_PAD_TEMPLATES {
            unsafe { ffi::gst_gl_filter_add_rgba_pad_templates(klass) }
        }
    }
}

unsafe extern "C" fn filter<T: GLFilterImpl>(
    ptr: *mut GstGLFilter,
    input: *mut GstBuffer,
    output: *mut GstBuffer,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.filter(&from_glib_borrow(input), &from_glib_borrow(output)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn filter_texture<T: GLFilterImpl>(
    ptr: *mut GstGLFilter,
    input: *mut GstGLMemory,
    output: *mut GstGLMemory,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.filter_texture(&from_glib_borrow(input), &from_glib_borrow(output)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn init_fbo<T: GLFilterImpl>(ptr: *mut GstGLFilter) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.init_fbo() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn set_caps<T: GLFilterImpl>(
    ptr: *mut GstGLFilter,
    incaps: *mut gst::ffi::GstCaps,
    outcaps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match GLFilterImpl::set_caps(imp, &from_glib_borrow(incaps), &from_glib_borrow(outcaps)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn transform_internal_caps<T: GLFilterImpl>(
    ptr: *mut GstGLFilter,
    direction: gst::ffi::GstPadDirection,
    caps: *mut gst::ffi::GstCaps,
    filter_caps: *mut gst::ffi::GstCaps,
) -> *mut gst::ffi::GstCaps {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, None, {
        let filter_caps: Borrowed<Option<Caps>> = from_glib_borrow(filter_caps);

        imp.transform_internal_caps(
            from_glib(direction),
            &from_glib_borrow(caps),
            filter_caps.as_ref().as_ref(),
        )
    })
    .map(|caps| caps.into_glib_ptr())
    .unwrap_or(std::ptr::null_mut())
}
