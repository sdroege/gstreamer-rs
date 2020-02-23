// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_rtsp_server_sys;

use glib::translate::*;
use gst_rtsp;

use glib::subclass::prelude::*;

use RTSPMediaFactory;
use RTSPMediaFactoryClass;

pub trait RTSPMediaFactoryImpl:
    RTSPMediaFactoryImplExt + ObjectImpl + Send + Sync + 'static
{
    fn gen_key(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString> {
        self.parent_gen_key(factory, url)
    }

    fn create_element(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<gst::Element> {
        self.parent_create_element(factory, url)
    }

    fn construct(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<::RTSPMedia> {
        self.parent_construct(factory, url)
    }

    fn create_pipeline(
        &self,
        factory: &RTSPMediaFactory,
        media: &::RTSPMedia,
    ) -> Option<gst::Pipeline> {
        self.parent_create_pipeline(factory, media)
    }

    fn configure(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia) {
        self.parent_configure(factory, media)
    }

    fn media_constructed(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia) {
        self.parent_media_constructed(factory, media)
    }

    fn media_configure(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia) {
        self.parent_media_configure(factory, media)
    }
}

pub trait RTSPMediaFactoryImplExt {
    fn parent_gen_key(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString>;

    fn parent_create_element(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<gst::Element>;

    fn parent_construct(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<::RTSPMedia>;

    fn parent_create_pipeline(
        &self,
        factory: &RTSPMediaFactory,
        media: &::RTSPMedia,
    ) -> Option<gst::Pipeline>;

    fn parent_configure(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia);

    fn parent_media_constructed(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia);
    fn parent_media_configure(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia);
}

impl<T: RTSPMediaFactoryImpl + ObjectImpl> RTSPMediaFactoryImplExt for T {
    fn parent_gen_key(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<glib::GString> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut gst_rtsp_server_sys::GstRTSPMediaFactoryClass;
            (*parent_class)
                .gen_key
                .map(|f| from_glib_full(f(factory.to_glib_none().0, url.to_glib_none().0)))
                .unwrap_or(None)
        }
    }

    fn parent_create_element(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<gst::Element> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut gst_rtsp_server_sys::GstRTSPMediaFactoryClass;
            (*parent_class)
                .create_element
                .map(|f| from_glib_none(f(factory.to_glib_none().0, url.to_glib_none().0)))
                .unwrap_or(None)
        }
    }

    fn parent_construct(
        &self,
        factory: &RTSPMediaFactory,
        url: &gst_rtsp::RTSPUrl,
    ) -> Option<::RTSPMedia> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut gst_rtsp_server_sys::GstRTSPMediaFactoryClass;
            (*parent_class)
                .construct
                .map(|f| from_glib_full(f(factory.to_glib_none().0, url.to_glib_none().0)))
                .unwrap_or(None)
        }
    }

    fn parent_create_pipeline(
        &self,
        factory: &RTSPMediaFactory,
        media: &::RTSPMedia,
    ) -> Option<gst::Pipeline> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut gst_rtsp_server_sys::GstRTSPMediaFactoryClass;
            (*parent_class)
                .create_pipeline
                .map(|f| {
                    let ptr = f(factory.to_glib_none().0, media.to_glib_none().0)
                        as *mut gst_sys::GstPipeline;

                    // See https://gitlab.freedesktop.org/gstreamer/gst-rtsp-server/merge_requests/109
                    if gobject_sys::g_object_is_floating(ptr as *mut _) != glib_sys::GFALSE {
                        gobject_sys::g_object_ref_sink(ptr as *mut _);
                    }
                    from_glib_none(ptr)
                })
                .unwrap_or(None)
        }
    }

    fn parent_configure(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut gst_rtsp_server_sys::GstRTSPMediaFactoryClass;
            if let Some(f) = (*parent_class).configure {
                f(factory.to_glib_none().0, media.to_glib_none().0);
            }
        }
    }

    fn parent_media_constructed(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut gst_rtsp_server_sys::GstRTSPMediaFactoryClass;
            if let Some(f) = (*parent_class).media_constructed {
                f(factory.to_glib_none().0, media.to_glib_none().0);
            }
        }
    }

    fn parent_media_configure(&self, factory: &RTSPMediaFactory, media: &::RTSPMedia) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut gst_rtsp_server_sys::GstRTSPMediaFactoryClass;
            if let Some(f) = (*parent_class).media_configure {
                f(factory.to_glib_none().0, media.to_glib_none().0);
            }
        }
    }
}
unsafe impl<T: ObjectSubclass + RTSPMediaFactoryImpl> IsSubclassable<T> for RTSPMediaFactoryClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass =
                &mut *(self as *mut Self as *mut gst_rtsp_server_sys::GstRTSPMediaFactoryClass);
            klass.gen_key = Some(factory_gen_key::<T>);
            klass.create_element = Some(factory_create_element::<T>);
            klass.construct = Some(factory_construct::<T>);
            klass.create_pipeline = Some(factory_create_pipeline::<T>);
            klass.configure = Some(factory_configure::<T>);
            klass.media_constructed = Some(factory_media_constructed::<T>);
            klass.media_configure = Some(factory_media_configure::<T>);
        }
    }
}

unsafe extern "C" fn factory_gen_key<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
    url: *const gst_rtsp_sys::GstRTSPUrl,
) -> *mut std::os::raw::c_char
where
    T: RTSPMediaFactoryImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: RTSPMediaFactory = from_glib_borrow(ptr);

    imp.gen_key(&wrap, &from_glib_borrow(url)).to_glib_full()
}

unsafe extern "C" fn factory_create_element<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
    url: *const gst_rtsp_sys::GstRTSPUrl,
) -> *mut gst_sys::GstElement
where
    T: RTSPMediaFactoryImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: RTSPMediaFactory = from_glib_borrow(ptr);

    let element = imp
        .create_element(&wrap, &from_glib_borrow(url))
        .to_glib_full();
    gobject_sys::g_object_force_floating(element as *mut _);
    element
}

unsafe extern "C" fn factory_construct<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
    url: *const gst_rtsp_sys::GstRTSPUrl,
) -> *mut gst_rtsp_server_sys::GstRTSPMedia
where
    T: RTSPMediaFactoryImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: RTSPMediaFactory = from_glib_borrow(ptr);

    imp.construct(&wrap, &from_glib_borrow(url)).to_glib_full()
}

lazy_static! {
    static ref PIPELINE_QUARK: glib::Quark =
        glib::Quark::from_string("gstreamer-rs-rtsp-media-pipeline");
}

unsafe extern "C" fn factory_create_pipeline<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
    media: *mut gst_rtsp_server_sys::GstRTSPMedia,
) -> *mut gst_sys::GstElement
where
    T: RTSPMediaFactoryImpl,
{
    use std::mem;

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: RTSPMediaFactory = from_glib_borrow(ptr);

    let pipeline: *mut gst_sys::GstPipeline = imp
        .create_pipeline(&wrap, &from_glib_borrow(media))
        .to_glib_none()
        .0;

    // FIXME We somehow need to ensure the pipeline actually stays alive...
    gobject_sys::g_object_set_qdata_full(
        media as *mut _,
        PIPELINE_QUARK.to_glib(),
        pipeline as *mut _,
        Some(mem::transmute(gobject_sys::g_object_unref as usize)),
    );

    pipeline as *mut _
}

unsafe extern "C" fn factory_configure<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
    media: *mut gst_rtsp_server_sys::GstRTSPMedia,
) where
    T: RTSPMediaFactoryImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: RTSPMediaFactory = from_glib_borrow(ptr);

    imp.configure(&wrap, &from_glib_borrow(media));
}

unsafe extern "C" fn factory_media_constructed<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
    media: *mut gst_rtsp_server_sys::GstRTSPMedia,
) where
    T: RTSPMediaFactoryImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: RTSPMediaFactory = from_glib_borrow(ptr);

    imp.media_constructed(&wrap, &from_glib_borrow(media));
}

unsafe extern "C" fn factory_media_configure<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPMediaFactory,
    media: *mut gst_rtsp_server_sys::GstRTSPMedia,
) where
    T: RTSPMediaFactoryImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: RTSPMediaFactory = from_glib_borrow(ptr);

    imp.media_configure(&wrap, &from_glib_borrow(media));
}
