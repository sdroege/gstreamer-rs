// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use super::prelude::*;
use crate::RTSPOnvifMediaFactory;

pub trait RTSPOnvifMediaFactoryImpl:
    RTSPMediaFactoryImplExt + RTSPMediaFactoryImpl + Send + Sync
{
    fn has_backchannel_support(&self, factory: &Self::Type) -> bool {
        self.parent_has_backchannel_support(factory)
    }
}

pub trait RTSPOnvifMediaFactoryImplExt: ObjectSubclass {
    fn parent_has_backchannel_support(&self, factory: &Self::Type) -> bool;
}

impl<T: RTSPOnvifMediaFactoryImpl> RTSPOnvifMediaFactoryImplExt for T {
    fn parent_has_backchannel_support(&self, factory: &Self::Type) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::GstRTSPOnvifMediaFactoryClass;
            (*parent_class)
                .has_backchannel_support
                .map(|f| {
                    from_glib(f(factory
                        .unsafe_cast_ref::<RTSPOnvifMediaFactory>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(false)
        }
    }
}

unsafe impl<T: RTSPOnvifMediaFactoryImpl> IsSubclassable<T> for RTSPOnvifMediaFactory {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.has_backchannel_support = Some(factory_has_backchannel_support::<T>);
    }
}

unsafe extern "C" fn factory_has_backchannel_support<T: RTSPOnvifMediaFactoryImpl>(
    ptr: *mut ffi::GstRTSPOnvifMediaFactory,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<RTSPOnvifMediaFactory> = from_glib_borrow(ptr);

    imp.has_backchannel_support(wrap.unsafe_cast_ref())
        .into_glib()
}
