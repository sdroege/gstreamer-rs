use ffi::GstGLBaseFilter;
use gst::ffi::GstCaps;

use crate::prelude::*;

use glib::translate::*;

use gst::{result_from_gboolean, Caps, LoggableError, CAT_RUST};
use gst_base::subclass::prelude::*;

use crate::GLBaseFilter;

pub trait GLBaseFilterImpl: GLBaseFilterImplExt + BaseTransformImpl {
    fn gl_set_caps(
        &self,
        filter: &Self::Type,
        incaps: &Caps,
        outcaps: &Caps,
    ) -> Result<(), LoggableError> {
        self.parent_gl_set_caps(filter, incaps, outcaps)
    }

    fn gl_start(&self, filter: &Self::Type) -> Result<(), LoggableError> {
        self.parent_gl_start(filter)
    }

    fn gl_stop(&self, filter: &Self::Type) {
        self.parent_gl_stop(filter)
    }
}

pub trait GLBaseFilterImplExt: ObjectSubclass {
    fn parent_gl_set_caps(
        &self,
        filter: &Self::Type,
        incaps: &Caps,
        outcaps: &Caps,
    ) -> Result<(), LoggableError>;

    fn parent_gl_start(&self, filter: &Self::Type) -> Result<(), LoggableError>;

    fn parent_gl_stop(&self, filter: &Self::Type);
}

impl<T: GLBaseFilterImpl> GLBaseFilterImplExt for T {
    fn parent_gl_set_caps(
        &self,
        filter: &Self::Type,
        incaps: &Caps,
        outcaps: &Caps,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseFilterClass;

            (*parent_class)
                .gl_set_caps
                .map(|f| {
                    result_from_gboolean!(
                        f(
                            filter.unsafe_cast_ref::<GLBaseFilter>().to_glib_none().0,
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

    fn parent_gl_start(&self, filter: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseFilterClass;

            (*parent_class)
                .gl_start
                .map(|f| {
                    result_from_gboolean!(
                        f(filter.unsafe_cast_ref::<GLBaseFilter>().to_glib_none().0),
                        CAT_RUST,
                        "Parent function `gl_start` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_gl_stop(&self, filter: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseFilterClass;

            if let Some(f) = (*parent_class).gl_stop {
                f(filter.unsafe_cast_ref::<GLBaseFilter>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: GLBaseFilterImpl> IsSubclassable<T> for GLBaseFilter {
    fn class_init(klass: &mut glib::Class<Self>) {
        <gst_base::BaseTransform as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.gl_set_caps = Some(gl_set_caps::<T>);
        klass.gl_start = Some(gl_start::<T>);
        klass.gl_stop = Some(gl_stop::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gst_base::BaseTransform as IsSubclassable<T>>::instance_init(instance)
    }
}

unsafe extern "C" fn gl_set_caps<T: GLBaseFilterImpl>(
    ptr: *mut GstGLBaseFilter,
    incaps: *mut GstCaps,
    outcaps: *mut GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<GLBaseFilter> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.gl_set_caps(
            wrap.unsafe_cast_ref(),
            &from_glib_borrow(incaps),
            &from_glib_borrow(outcaps),
        ) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
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
    let imp = instance.impl_();
    let wrap: Borrowed<GLBaseFilter> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), false, {
        match imp.gl_start(wrap.unsafe_cast_ref()) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn gl_stop<T: GLBaseFilterImpl>(ptr: *mut GstGLBaseFilter) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<GLBaseFilter> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, &imp.panicked(), (), {
        imp.gl_stop(wrap.unsafe_cast_ref())
    })
}
