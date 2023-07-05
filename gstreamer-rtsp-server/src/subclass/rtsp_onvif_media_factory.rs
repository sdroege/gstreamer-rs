// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use super::prelude::*;
use crate::RTSPOnvifMediaFactory;

pub trait RTSPOnvifMediaFactoryImpl:
    RTSPMediaFactoryImplExt + RTSPMediaFactoryImpl + Send + Sync
{
    fn has_backchannel_support(&self) -> bool {
        self.parent_has_backchannel_support()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::RTSPOnvifMediaFactoryImplExt> Sealed for T {}
}

pub trait RTSPOnvifMediaFactoryImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_has_backchannel_support(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::GstRTSPOnvifMediaFactoryClass;
            (*parent_class)
                .has_backchannel_support
                .map(|f| {
                    from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<RTSPOnvifMediaFactory>()
                        .to_glib_none()
                        .0))
                })
                .unwrap_or(false)
        }
    }
}

impl<T: RTSPOnvifMediaFactoryImpl> RTSPOnvifMediaFactoryImplExt for T {}

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

    imp.has_backchannel_support().into_glib()
}
