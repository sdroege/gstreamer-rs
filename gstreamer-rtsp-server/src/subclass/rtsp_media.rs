// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use std::ptr;

use crate::RTSPMedia;
use crate::RTSPThread;

#[derive(Debug)]
pub struct SDPInfo(ptr::NonNull<ffi::GstSDPInfo>);

impl SDPInfo {
    pub fn is_ipv6(&self) -> bool {
        unsafe { from_glib(self.0.as_ref().is_ipv6) }
    }

    pub fn server_ip(&self) -> &str {
        unsafe {
            use std::ffi::CStr;
            CStr::from_ptr(self.0.as_ref().server_ip).to_str().unwrap()
        }
    }
}

pub trait RTSPMediaImpl: RTSPMediaImplExt + ObjectImpl + Send + Sync {
    fn handle_message(&self, media: &Self::Type, message: &gst::MessageRef) -> bool {
        self.parent_handle_message(media, message)
    }

    fn prepare(&self, media: &Self::Type, thread: &RTSPThread) -> Result<(), gst::LoggableError> {
        self.parent_prepare(media, thread)
    }

    fn unprepare(&self, media: &Self::Type) -> Result<(), gst::LoggableError> {
        self.parent_unprepare(media)
    }

    fn suspend(&self, media: &Self::Type) -> Result<(), gst::LoggableError> {
        self.parent_suspend(media)
    }

    fn unsuspend(&self, media: &Self::Type) -> Result<(), gst::LoggableError> {
        self.parent_unsuspend(media)
    }

    // TODO missing: convert_range

    fn query_position(&self, media: &Self::Type) -> Option<gst::ClockTime> {
        self.parent_query_position(media)
    }

    fn query_stop(&self, media: &Self::Type) -> Option<gst::ClockTime> {
        self.parent_query_stop(media)
    }

    fn create_rtpbin(&self, media: &Self::Type) -> Option<gst::Element> {
        self.parent_create_rtpbin(media)
    }

    fn setup_rtpbin(
        &self,
        media: &Self::Type,
        rtpbin: &gst::Element,
    ) -> Result<(), gst::LoggableError> {
        self.parent_setup_rtpbin(media, rtpbin)
    }

    fn setup_sdp(
        &self,
        media: &Self::Type,
        sdp: &mut gst_sdp::SDPMessageRef,
        info: &SDPInfo,
    ) -> Result<(), gst::LoggableError> {
        self.parent_setup_sdp(media, sdp, info)
    }

    fn new_stream(&self, media: &Self::Type, stream: &crate::RTSPStream) {
        self.parent_new_stream(media, stream);
    }

    fn removed_stream(&self, media: &Self::Type, stream: &crate::RTSPStream) {
        self.parent_removed_stream(media, stream);
    }

    fn prepared(&self, media: &Self::Type) {
        self.parent_prepared(media);
    }

    fn unprepared(&self, media: &Self::Type) {
        self.parent_unprepared(media);
    }

    fn target_state(&self, media: &Self::Type, state: gst::State) {
        self.parent_target_state(media, state);
    }

    fn new_state(&self, media: &Self::Type, state: gst::State) {
        self.parent_new_state(media, state);
    }

    fn handle_sdp(
        &self,
        media: &Self::Type,
        sdp: &gst_sdp::SDPMessageRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_handle_sdp(media, sdp)
    }
}

pub trait RTSPMediaImplExt: ObjectSubclass {
    fn parent_handle_message(&self, media: &Self::Type, message: &gst::MessageRef) -> bool;
    fn parent_prepare(
        &self,
        media: &Self::Type,
        thread: &RTSPThread,
    ) -> Result<(), gst::LoggableError>;
    fn parent_unprepare(&self, media: &Self::Type) -> Result<(), gst::LoggableError>;
    fn parent_suspend(&self, media: &Self::Type) -> Result<(), gst::LoggableError>;
    fn parent_unsuspend(&self, media: &Self::Type) -> Result<(), gst::LoggableError>;
    // TODO missing: convert_range

    fn parent_query_position(&self, media: &Self::Type) -> Option<gst::ClockTime>;
    fn parent_query_stop(&self, media: &Self::Type) -> Option<gst::ClockTime>;
    fn parent_create_rtpbin(&self, media: &Self::Type) -> Option<gst::Element>;
    fn parent_setup_rtpbin(
        &self,
        media: &Self::Type,
        rtpbin: &gst::Element,
    ) -> Result<(), gst::LoggableError>;
    fn parent_setup_sdp(
        &self,
        media: &Self::Type,
        sdp: &mut gst_sdp::SDPMessageRef,
        info: &SDPInfo,
    ) -> Result<(), gst::LoggableError>;
    fn parent_new_stream(&self, media: &Self::Type, stream: &crate::RTSPStream);
    fn parent_removed_stream(&self, media: &Self::Type, stream: &crate::RTSPStream);
    fn parent_prepared(&self, media: &Self::Type);
    fn parent_unprepared(&self, media: &Self::Type);
    fn parent_target_state(&self, media: &Self::Type, state: gst::State);
    fn parent_new_state(&self, media: &Self::Type, state: gst::State);
    fn parent_handle_sdp(
        &self,
        media: &Self::Type,
        sdp: &gst_sdp::SDPMessageRef,
    ) -> Result<(), gst::LoggableError>;
}

impl<T: RTSPMediaImpl> RTSPMediaImplExt for T {
    fn parent_handle_message(&self, media: &Self::Type, message: &gst::MessageRef) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).handle_message {
                from_glib(f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    message.as_ptr() as *mut _,
                ))
            } else {
                false
            }
        }
    }

    fn parent_prepare(
        &self,
        media: &Self::Type,
        thread: &RTSPThread,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).prepare {
                gst::result_from_gboolean!(
                    f(
                        media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                        thread.to_glib_none().0
                    ),
                    gst::CAT_RUST,
                    "Parent function `prepare` failed"
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_unprepare(&self, media: &Self::Type) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).unprepare {
                gst::result_from_gboolean!(
                    f(media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0),
                    gst::CAT_RUST,
                    "Parent function `unprepare` failed"
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_suspend(&self, media: &Self::Type) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).suspend {
                gst::result_from_gboolean!(
                    f(media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0),
                    gst::CAT_RUST,
                    "Parent function `suspend` failed"
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_unsuspend(&self, media: &Self::Type) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).unsuspend {
                gst::result_from_gboolean!(
                    f(media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0),
                    gst::CAT_RUST,
                    "Parent function `unsuspend` failed"
                )
            } else {
                Ok(())
            }
        }
    }

    // TODO missing: convert_range

    fn parent_query_position(&self, media: &Self::Type) -> Option<gst::ClockTime> {
        unsafe {
            use std::mem;

            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).query_position {
                let mut position = mem::MaybeUninit::uninit();
                if f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    position.as_mut_ptr(),
                ) == glib::ffi::GFALSE
                {
                    None
                } else {
                    Some(from_glib(position.assume_init() as u64))
                }
            } else {
                None
            }
        }
    }

    fn parent_query_stop(&self, media: &Self::Type) -> Option<gst::ClockTime> {
        unsafe {
            use std::mem;

            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).query_stop {
                let mut stop = mem::MaybeUninit::uninit();
                if f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    stop.as_mut_ptr(),
                ) == glib::ffi::GFALSE
                {
                    None
                } else {
                    Some(from_glib(stop.assume_init() as u64))
                }
            } else {
                None
            }
        }
    }

    fn parent_create_rtpbin(&self, media: &Self::Type) -> Option<gst::Element> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            let f = (*parent_class)
                .create_rtpbin
                .expect("No `create_rtpbin` virtual method implementation in parent class");

            from_glib_none(f(media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0))
        }
    }

    fn parent_setup_rtpbin(
        &self,
        media: &Self::Type,
        rtpbin: &gst::Element,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).setup_rtpbin {
                let ptr = rtpbin.to_glib_none().0;

                // The C code assumes to pass a floating reference around so let's make sure we do
                glib::gobject_ffi::g_object_force_floating(ptr as *mut _);

                let res = gst::result_from_gboolean!(
                    f(media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0, ptr),
                    gst::CAT_RUST,
                    "Parent function `setup_sdp` failed"
                );

                // If the code didn't accidentally sink it then we have to do that
                // here now so that we don't have any floating reference on our side
                // anymore
                if glib::gobject_ffi::g_object_is_floating(ptr as *mut _) != glib::ffi::GFALSE {
                    glib::gobject_ffi::g_object_ref_sink(ptr as *mut _);
                }

                res
            } else {
                Ok(())
            }
        }
    }

    fn parent_setup_sdp(
        &self,
        media: &Self::Type,
        sdp: &mut gst_sdp::SDPMessageRef,
        info: &SDPInfo,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            let f = (*parent_class)
                .setup_sdp
                .expect("No `setup_sdp` virtual method implementation in parent class");

            gst::result_from_gboolean!(
                f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    sdp as *mut _ as *mut gst_sdp::ffi::GstSDPMessage,
                    info.0.as_ptr()
                ),
                gst::CAT_RUST,
                "Parent function `setup_sdp` failed"
            )
        }
    }

    fn parent_new_stream(&self, media: &Self::Type, stream: &crate::RTSPStream) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).new_stream {
                f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    stream.to_glib_none().0,
                );
            }
        }
    }

    fn parent_removed_stream(&self, media: &Self::Type, stream: &crate::RTSPStream) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).removed_stream {
                f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    stream.to_glib_none().0,
                );
            }
        }
    }

    fn parent_prepared(&self, media: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).prepared {
                f(media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0);
            }
        }
    }

    fn parent_unprepared(&self, media: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).unprepared {
                f(media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0);
            }
        }
    }

    fn parent_target_state(&self, media: &Self::Type, state: gst::State) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).target_state {
                f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    state.to_glib(),
                );
            }
        }
    }

    fn parent_new_state(&self, media: &Self::Type, state: gst::State) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).new_state {
                f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    state.to_glib(),
                );
            }
        }
    }

    fn parent_handle_sdp(
        &self,
        media: &Self::Type,
        sdp: &gst_sdp::SDPMessageRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstRTSPMediaClass;
            let f = (*parent_class)
                .handle_sdp
                .expect("No `handle_sdp` virtual method implementation in parent class");

            gst::result_from_gboolean!(
                f(
                    media.unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    sdp as *const _ as *mut gst_sdp::ffi::GstSDPMessage
                ),
                gst::CAT_RUST,
                "Parent function `handle_sdp` failed"
            )
        }
    }
}
unsafe impl<T: RTSPMediaImpl> IsSubclassable<T> for RTSPMedia {
    fn class_init(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.handle_message = Some(media_handle_message::<T>);
        klass.prepare = Some(media_prepare::<T>);
        klass.unprepare = Some(media_unprepare::<T>);
        klass.suspend = Some(media_suspend::<T>);
        klass.unsuspend = Some(media_unsuspend::<T>);
        klass.query_position = Some(media_query_position::<T>);
        klass.query_stop = Some(media_query_stop::<T>);
        klass.create_rtpbin = Some(media_create_rtpbin::<T>);
        klass.setup_rtpbin = Some(media_setup_rtpbin::<T>);
        klass.setup_sdp = Some(media_setup_sdp::<T>);
        klass.new_stream = Some(media_new_stream::<T>);
        klass.removed_stream = Some(media_removed_stream::<T>);
        klass.prepared = Some(media_prepared::<T>);
        klass.unprepared = Some(media_unprepared::<T>);
        klass.target_state = Some(media_target_state::<T>);
        klass.new_state = Some(media_new_state::<T>);
        klass.handle_sdp = Some(media_handle_sdp::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn media_handle_message<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    message: *mut gst::ffi::GstMessage,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    imp.handle_message(wrap.unsafe_cast_ref(), gst::MessageRef::from_ptr(message))
        .to_glib()
}

unsafe extern "C" fn media_prepare<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    thread: *mut ffi::GstRTSPThread,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    match imp.prepare(wrap.unsafe_cast_ref(), &from_glib_borrow(thread)) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_unprepare<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    match imp.unprepare(wrap.unsafe_cast_ref()) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_suspend<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    match imp.suspend(wrap.unsafe_cast_ref()) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_unsuspend<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    match imp.unsuspend(wrap.unsafe_cast_ref()) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_query_position<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    position: *mut i64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    match imp.query_position(wrap.unsafe_cast_ref()) {
        Some(pos) => {
            *position = pos.to_glib() as i64;
            glib::ffi::GTRUE
        }
        None => glib::ffi::GFALSE,
    }
}

unsafe extern "C" fn media_query_stop<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    stop: *mut i64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    match imp.query_stop(wrap.unsafe_cast_ref()) {
        Some(s) => {
            *stop = s.to_glib() as i64;
            glib::ffi::GTRUE
        }
        None => glib::ffi::GFALSE,
    }
}

unsafe extern "C" fn media_create_rtpbin<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
) -> *mut gst::ffi::GstElement {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    let res: *mut gst::ffi::GstElement = imp.create_rtpbin(wrap.unsafe_cast_ref()).to_glib_full();

    if !res.is_null() {
        glib::gobject_ffi::g_object_force_floating(res as *mut _);
    }

    res
}

unsafe extern "C" fn media_setup_rtpbin<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    rtpbin: *mut gst::ffi::GstElement,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    // If the rtpbin was floating before make sure it is not anymore for now so
    // we don't accidentally take ownership of it somewhere along the line
    if glib::gobject_ffi::g_object_is_floating(rtpbin as *mut _) != glib::ffi::GFALSE {
        glib::gobject_ffi::g_object_ref_sink(rtpbin as *mut _);
    }

    let res = match imp.setup_rtpbin(wrap.unsafe_cast_ref(), &from_glib_borrow(rtpbin)) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    };

    // Ensure that the rtpbin is still floating afterwards here
    glib::gobject_ffi::g_object_force_floating(rtpbin as *mut _);

    res
}

unsafe extern "C" fn media_setup_sdp<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    sdp: *mut gst_sdp::ffi::GstSDPMessage,
    info: *mut ffi::GstSDPInfo,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    match imp.setup_sdp(
        wrap.unsafe_cast_ref(),
        &mut *(sdp as *mut gst_sdp::SDPMessageRef),
        &SDPInfo(ptr::NonNull::new(info).expect("NULL SDPInfo")),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_new_stream<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    stream: *mut ffi::GstRTSPStream,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    imp.new_stream(wrap.unsafe_cast_ref(), &from_glib_borrow(stream));
}

unsafe extern "C" fn media_removed_stream<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    stream: *mut ffi::GstRTSPStream,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    imp.removed_stream(wrap.unsafe_cast_ref(), &from_glib_borrow(stream));
}

unsafe extern "C" fn media_prepared<T: RTSPMediaImpl>(ptr: *mut ffi::GstRTSPMedia) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    imp.prepared(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn media_unprepared<T: RTSPMediaImpl>(ptr: *mut ffi::GstRTSPMedia) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    imp.unprepared(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn media_target_state<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    state: gst::ffi::GstState,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    imp.target_state(wrap.unsafe_cast_ref(), from_glib(state));
}

unsafe extern "C" fn media_new_state<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    state: gst::ffi::GstState,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    imp.new_state(wrap.unsafe_cast_ref(), from_glib(state));
}

unsafe extern "C" fn media_handle_sdp<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    sdp: *mut gst_sdp::ffi::GstSDPMessage,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPMedia> = from_glib_borrow(ptr);

    match imp.handle_sdp(
        wrap.unsafe_cast_ref(),
        &*(sdp as *const gst_sdp::SDPMessageRef),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_object(&*wrap);
            glib::ffi::GFALSE
        }
    }
}
