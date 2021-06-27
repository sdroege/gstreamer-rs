// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use gst_rtsp::ffi::GstRTSPUrl;

use crate::RTSPMountPoints;

pub trait RTSPMountPointsImpl: RTSPMountPointsImplExt + ObjectImpl + Send + Sync {
    fn make_path(
        &self,
        mount_points: &Self::Type,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString> {
        self.parent_make_path(mount_points, url)
    }
}

pub trait RTSPMountPointsImplExt: ObjectSubclass {
    fn parent_make_path(
        &self,
        mount_points: &Self::Type,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString>;
}

impl<T: RTSPMountPointsImpl> RTSPMountPointsImplExt for T {
    fn parent_make_path(
        &self,
        mount_points: &Self::Type,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMountPointsClass;
            let f = (*parent_class)
                .make_path
                .expect("No `make_path` virtual method implementation in parent class");
            from_glib_full(f(
                mount_points
                    .unsafe_cast_ref::<RTSPMountPoints>()
                    .to_glib_none()
                    .0,
                url.to_glib_none().0,
            ))
        }
    }
}

unsafe impl<T: RTSPMountPointsImpl> IsSubclassable<T> for RTSPMountPoints {
    fn class_init(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.make_path = Some(mount_points_make_path::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn mount_points_make_path<T: RTSPMountPointsImpl>(
    ptr: *mut ffi::GstRTSPMountPoints,
    url: *const GstRTSPUrl,
) -> *mut std::os::raw::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<RTSPMountPoints> = from_glib_borrow(ptr);

    imp.make_path(wrap.unsafe_cast_ref(), &from_glib_borrow(url))
        .to_glib_full()
}
