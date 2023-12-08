// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RTSPAuth, RTSPContext};
use glib::{prelude::*, subclass::prelude::*, translate::*};
use libc::c_char;

pub trait RTSPAuthImpl: RTSPAuthImplExt + ObjectImpl + Send + Sync {
    fn authenticate(&self, ctx: &RTSPContext) -> bool {
        self.parent_authenticate(ctx)
    }

    fn check(&self, ctx: &RTSPContext, check: &glib::GString) -> bool {
        self.parent_check(ctx, check)
    }

    fn generate_authenticate_header(&self, ctx: &RTSPContext) {
        self.parent_generate_authenticate_header(ctx);
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::RTSPAuthImplExt> Sealed for T {}
}

pub trait RTSPAuthImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_authenticate(&self, ctx: &RTSPContext) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPAuthClass;
            (*parent_class)
                .authenticate
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<RTSPAuth>().to_glib_none().0,
                        ctx.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_check(&self, ctx: &RTSPContext, check: &glib::GString) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPAuthClass;
            (*parent_class)
                .check
                .map(|f| {
                    from_glib(f(
                        self.obj().unsafe_cast_ref::<RTSPAuth>().to_glib_none().0,
                        ctx.to_glib_none().0,
                        check.to_glib_none().0,
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_generate_authenticate_header(&self, ctx: &RTSPContext) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPAuthClass;
            if let Some(f) = (*parent_class).generate_authenticate_header {
                f(
                    self.obj().unsafe_cast_ref::<RTSPAuth>().to_glib_none().0,
                    ctx.to_glib_none().0,
                )
            }
        }
    }
}

impl<T: RTSPAuthImpl> RTSPAuthImplExt for T {}

unsafe impl<T: RTSPAuthImpl> IsSubclassable<T> for RTSPAuth {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.authenticate = Some(rtsp_auth_authenticate::<T>);
        klass.check = Some(rtsp_auth_check::<T>);
        klass.generate_authenticate_header = Some(rtsp_auth_generate_authenticate_header::<T>);
    }
}

unsafe extern "C" fn rtsp_auth_authenticate<T: RTSPAuthImpl>(
    ptr: *mut ffi::GstRTSPAuth,
    ctx: *mut ffi::GstRTSPContext,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.authenticate(&from_glib_borrow(ctx)).into_glib()
}

unsafe extern "C" fn rtsp_auth_check<T: RTSPAuthImpl>(
    ptr: *mut ffi::GstRTSPAuth,
    ctx: *mut ffi::GstRTSPContext,
    check: *const c_char,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.check(&from_glib_borrow(ctx), &from_glib_borrow(check))
        .into_glib()
}

unsafe extern "C" fn rtsp_auth_generate_authenticate_header<T: RTSPAuthImpl>(
    ptr: *mut ffi::GstRTSPAuth,
    ctx: *mut ffi::GstRTSPContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.generate_authenticate_header(&from_glib_borrow(ctx));
}
