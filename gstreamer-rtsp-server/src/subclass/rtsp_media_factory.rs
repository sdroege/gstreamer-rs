// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem::transmute;

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::RTSPMediaFactory;

pub trait RTSPMediaFactoryImpl: RTSPMediaFactoryImplExt + ObjectImpl + Send + Sync {
    fn gen_key(&self, url: &gst_rtsp::RTSPUrl) -> Option<glib::GString> {
        self.parent_gen_key(url)
    }

    fn create_element(&self, url: &gst_rtsp::RTSPUrl) -> Option<gst::Element> {
        self.parent_create_element(url)
    }

    fn construct(&self, url: &gst_rtsp::RTSPUrl) -> Option<crate::RTSPMedia> {
        self.parent_construct(url)
    }

    fn create_pipeline(&self, media: &crate::RTSPMedia) -> Option<gst::Pipeline> {
        self.parent_create_pipeline(media)
    }

    fn configure(&self, media: &crate::RTSPMedia) {
        self.parent_configure(media)
    }

    fn media_constructed(&self, media: &crate::RTSPMedia) {
        self.parent_media_constructed(media)
    }

    fn media_configure(&self, media: &crate::RTSPMedia) {
        self.parent_media_configure(media)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::RTSPMediaFactoryImplExt> Sealed for T {}
}

pub trait RTSPMediaFactoryImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_gen_key(&self, url: &gst_rtsp::RTSPUrl) -> Option<glib::GString> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaFactoryClass;
            (*parent_class)
                .gen_key
                .map(|f| {
                    from_glib_full(f(
                        self.obj()
                            .unsafe_cast_ref::<RTSPMediaFactory>()
                            .to_glib_none()
                            .0,
                        url.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_create_element(&self, url: &gst_rtsp::RTSPUrl) -> Option<gst::Element> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaFactoryClass;
            (*parent_class)
                .create_element
                .map(|f| {
                    from_glib_none(f(
                        self.obj()
                            .unsafe_cast_ref::<RTSPMediaFactory>()
                            .to_glib_none()
                            .0,
                        url.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_construct(&self, url: &gst_rtsp::RTSPUrl) -> Option<crate::RTSPMedia> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaFactoryClass;
            (*parent_class)
                .construct
                .map(|f| {
                    from_glib_full(f(
                        self.obj()
                            .unsafe_cast_ref::<RTSPMediaFactory>()
                            .to_glib_none()
                            .0,
                        url.to_glib_none().0,
                    ))
                })
                .unwrap_or(None)
        }
    }

    fn parent_create_pipeline(&self, media: &crate::RTSPMedia) -> Option<gst::Pipeline> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaFactoryClass;
            (*parent_class)
                .create_pipeline
                .map(|f| {
                    let ptr = f(
                        self.obj()
                            .unsafe_cast_ref::<RTSPMediaFactory>()
                            .to_glib_none()
                            .0,
                        media.to_glib_none().0,
                    ) as *mut gst::ffi::GstPipeline;

                    // See https://gitlab.freedesktop.org/gstreamer/gst-rtsp-server/merge_requests/109
                    if glib::gobject_ffi::g_object_is_floating(ptr as *mut _) != glib::ffi::GFALSE {
                        glib::gobject_ffi::g_object_ref_sink(ptr as *mut _);
                    }
                    from_glib_none(ptr)
                })
                .unwrap_or(None)
        }
    }

    fn parent_configure(&self, media: &crate::RTSPMedia) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaFactoryClass;
            if let Some(f) = (*parent_class).configure {
                f(
                    self.obj()
                        .unsafe_cast_ref::<RTSPMediaFactory>()
                        .to_glib_none()
                        .0,
                    media.to_glib_none().0,
                );
            }
        }
    }

    fn parent_media_constructed(&self, media: &crate::RTSPMedia) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaFactoryClass;
            if let Some(f) = (*parent_class).media_constructed {
                f(
                    self.obj()
                        .unsafe_cast_ref::<RTSPMediaFactory>()
                        .to_glib_none()
                        .0,
                    media.to_glib_none().0,
                );
            }
        }
    }

    fn parent_media_configure(&self, media: &crate::RTSPMedia) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaFactoryClass;
            if let Some(f) = (*parent_class).media_configure {
                f(
                    self.obj()
                        .unsafe_cast_ref::<RTSPMediaFactory>()
                        .to_glib_none()
                        .0,
                    media.to_glib_none().0,
                );
            }
        }
    }
}

impl<T: RTSPMediaFactoryImpl> RTSPMediaFactoryImplExt for T {}
unsafe impl<T: RTSPMediaFactoryImpl> IsSubclassable<T> for RTSPMediaFactory {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.gen_key = Some(factory_gen_key::<T>);
        klass.create_element = Some(factory_create_element::<T>);
        klass.construct = Some(factory_construct::<T>);
        klass.create_pipeline = Some(factory_create_pipeline::<T>);
        klass.configure = Some(factory_configure::<T>);
        klass.media_constructed = Some(factory_media_constructed::<T>);
        klass.media_configure = Some(factory_media_configure::<T>);
    }
}

unsafe extern "C" fn factory_gen_key<T: RTSPMediaFactoryImpl>(
    ptr: *mut ffi::GstRTSPMediaFactory,
    url: *const gst_rtsp::ffi::GstRTSPUrl,
) -> *mut std::os::raw::c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.gen_key(&from_glib_borrow(url)).into_glib_ptr()
}

unsafe extern "C" fn factory_create_element<T: RTSPMediaFactoryImpl>(
    ptr: *mut ffi::GstRTSPMediaFactory,
    url: *const gst_rtsp::ffi::GstRTSPUrl,
) -> *mut gst::ffi::GstElement {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let element = imp.create_element(&from_glib_borrow(url)).into_glib_ptr();
    glib::gobject_ffi::g_object_force_floating(element as *mut _);
    element
}

unsafe extern "C" fn factory_construct<T: RTSPMediaFactoryImpl>(
    ptr: *mut ffi::GstRTSPMediaFactory,
    url: *const gst_rtsp::ffi::GstRTSPUrl,
) -> *mut ffi::GstRTSPMedia {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.construct(&from_glib_borrow(url)).into_glib_ptr()
}

unsafe extern "C" fn factory_create_pipeline<T: RTSPMediaFactoryImpl>(
    ptr: *mut ffi::GstRTSPMediaFactory,
    media: *mut ffi::GstRTSPMedia,
) -> *mut gst::ffi::GstElement {
    use glib::once_cell::sync::Lazy;

    static PIPELINE_QUARK: Lazy<glib::Quark> =
        Lazy::new(|| glib::Quark::from_str("gstreamer-rs-rtsp-media-pipeline"));

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let pipeline: *mut gst::ffi::GstPipeline = imp
        .create_pipeline(&from_glib_borrow(media))
        .into_glib_ptr();

    // FIXME We somehow need to ensure the pipeline actually stays alive...
    glib::gobject_ffi::g_object_set_qdata_full(
        media as *mut _,
        PIPELINE_QUARK.into_glib(),
        pipeline as *mut _,
        Some(transmute::<_, unsafe extern "C" fn(glib::ffi::gpointer)>(
            glib::gobject_ffi::g_object_unref as *const (),
        )),
    );

    pipeline as *mut _
}

unsafe extern "C" fn factory_configure<T: RTSPMediaFactoryImpl>(
    ptr: *mut ffi::GstRTSPMediaFactory,
    media: *mut ffi::GstRTSPMedia,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.configure(&from_glib_borrow(media));
}

unsafe extern "C" fn factory_media_constructed<T: RTSPMediaFactoryImpl>(
    ptr: *mut ffi::GstRTSPMediaFactory,
    media: *mut ffi::GstRTSPMedia,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.media_constructed(&from_glib_borrow(media));
}

unsafe extern "C" fn factory_media_configure<T: RTSPMediaFactoryImpl>(
    ptr: *mut ffi::GstRTSPMediaFactory,
    media: *mut ffi::GstRTSPMedia,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.media_configure(&from_glib_borrow(media));
}
