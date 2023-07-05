use ffi::GstGLBaseFilter;
use glib::translate::*;
use gst::{ffi::GstCaps, result_from_gboolean, Caps, LoggableError, CAT_RUST};
use gst_base::subclass::prelude::*;

use crate::{prelude::*, GLBaseFilter};

pub trait GLBaseFilterImpl: GLBaseFilterImplExt + BaseTransformImpl {
    fn gl_set_caps(&self, incaps: &Caps, outcaps: &Caps) -> Result<(), LoggableError> {
        self.parent_gl_set_caps(incaps, outcaps)
    }

    fn gl_start(&self) -> Result<(), LoggableError> {
        self.parent_gl_start()
    }

    fn gl_stop(&self) {
        self.parent_gl_stop()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::GLBaseFilterImplExt> Sealed for T {}
}

pub trait GLBaseFilterImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_gl_set_caps(&self, incaps: &Caps, outcaps: &Caps) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseFilterClass;

            (*parent_class)
                .gl_set_caps
                .map(|f| {
                    result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<GLBaseFilter>()
                                .to_glib_none()
                                .0,
                            incaps.to_glib_none().0,
                            outcaps.to_glib_none().0,
                        ),
                        CAT_RUST,
                        "Parent function `gl_set_caps` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_gl_start(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseFilterClass;

            (*parent_class)
                .gl_start
                .map(|f| {
                    result_from_gboolean!(
                        f(self
                            .obj()
                            .unsafe_cast_ref::<GLBaseFilter>()
                            .to_glib_none()
                            .0),
                        CAT_RUST,
                        "Parent function `gl_start` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_gl_stop(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseFilterClass;

            if let Some(f) = (*parent_class).gl_stop {
                f(self
                    .obj()
                    .unsafe_cast_ref::<GLBaseFilter>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

impl<T: GLBaseFilterImpl> GLBaseFilterImplExt for T {}

unsafe impl<T: GLBaseFilterImpl> IsSubclassable<T> for GLBaseFilter {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.gl_set_caps = Some(gl_set_caps::<T>);
        klass.gl_start = Some(gl_start::<T>);
        klass.gl_stop = Some(gl_stop::<T>);
    }
}

unsafe extern "C" fn gl_set_caps<T: GLBaseFilterImpl>(
    ptr: *mut GstGLBaseFilter,
    incaps: *mut GstCaps,
    outcaps: *mut GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.gl_set_caps(&from_glib_borrow(incaps), &from_glib_borrow(outcaps)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn gl_start<T: GLBaseFilterImpl>(
    ptr: *mut GstGLBaseFilter,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.gl_start() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn gl_stop<T: GLBaseFilterImpl>(ptr: *mut GstGLBaseFilter) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, (), { imp.gl_stop() })
}
