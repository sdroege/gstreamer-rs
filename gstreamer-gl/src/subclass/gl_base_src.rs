use ffi::{GstGLBaseSrc, GstGLMemory};

use crate::prelude::*;

use glib::translate::*;

use gst::{result_from_gboolean, LoggableError, CAT_RUST};
use gst_base::subclass::prelude::*;

use crate::{GLBaseSrc, GLMemory, GLAPI};

pub trait GLBaseSrcImpl: GLBaseSrcImplExt + PushSrcImpl {
    const SUPPORTED_GL_API: GLAPI;

    fn gl_start(&self, element: &Self::Type) -> Result<(), LoggableError> {
        self.parent_gl_start(element)
    }

    fn gl_stop(&self, element: &Self::Type) {
        self.parent_gl_stop(element)
    }

    fn fill_gl_memory(&self, element: &Self::Type, memory: &GLMemory) -> Result<(), LoggableError> {
        self.parent_fill_gl_memory(element, memory)
    }
}

pub trait GLBaseSrcImplExt: ObjectSubclass {
    fn parent_gl_start(&self, element: &Self::Type) -> Result<(), LoggableError>;

    fn parent_gl_stop(&self, element: &Self::Type);

    fn parent_fill_gl_memory(
        &self,
        element: &Self::Type,
        memory: &GLMemory,
    ) -> Result<(), LoggableError>;
}

impl<T: GLBaseSrcImpl> GLBaseSrcImplExt for T {
    fn parent_gl_start(&self, element: &Self::Type) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseSrcClass;

            (*parent_class)
                .gl_start
                .map(|f| {
                    result_from_gboolean!(
                        f(element.unsafe_cast_ref::<GLBaseSrc>().to_glib_none().0),
                        CAT_RUST,
                        "Parent function `gl_start` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_gl_stop(&self, element: &Self::Type) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseSrcClass;

            if let Some(f) = (*parent_class).gl_stop {
                f(element.unsafe_cast_ref::<GLBaseSrc>().to_glib_none().0)
            }
        }
    }

    fn parent_fill_gl_memory(
        &self,
        element: &Self::Type,
        memory: &GLMemory,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstGLBaseSrcClass;

            (*parent_class)
                .fill_gl_memory
                .map(|f| {
                    result_from_gboolean!(
                        f(
                            element.unsafe_cast_ref::<GLBaseSrc>().to_glib_none().0,
                            mut_override(memory.to_glib_none().0),
                        ),
                        CAT_RUST,
                        "Parent function `fill_gl_memory` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }
}

unsafe impl<T: GLBaseSrcImpl> IsSubclassable<T> for GLBaseSrc {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.supported_gl_api = T::SUPPORTED_GL_API.into_glib();
        klass.gl_start = Some(gl_start::<T>);
        klass.gl_stop = Some(gl_stop::<T>);
        klass.fill_gl_memory = Some(fill_gl_memory::<T>);
    }
}

unsafe extern "C" fn gl_start<T: GLBaseSrcImpl>(ptr: *mut GstGLBaseSrc) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<GLBaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
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

unsafe extern "C" fn gl_stop<T: GLBaseSrcImpl>(ptr: *mut GstGLBaseSrc) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<GLBaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), (), {
        imp.gl_stop(wrap.unsafe_cast_ref())
    })
}

unsafe extern "C" fn fill_gl_memory<T: GLBaseSrcImpl>(
    ptr: *mut GstGLBaseSrc,
    memory: *mut GstGLMemory,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<GLBaseSrc> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.fill_gl_memory(wrap.unsafe_cast_ref(), &from_glib_borrow(memory)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}
