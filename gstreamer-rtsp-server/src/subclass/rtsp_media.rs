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
    fn handle_message(&self, message: &gst::MessageRef) -> bool {
        self.parent_handle_message(message)
    }

    fn prepare(&self, thread: &RTSPThread) -> Result<(), gst::LoggableError> {
        self.parent_prepare(thread)
    }

    fn unprepare(&self) -> Result<(), gst::LoggableError> {
        self.parent_unprepare()
    }

    fn suspend(&self) -> Result<(), gst::LoggableError> {
        self.parent_suspend()
    }

    fn unsuspend(&self) -> Result<(), gst::LoggableError> {
        self.parent_unsuspend()
    }

    // TODO missing: convert_range

    fn query_position(&self) -> Option<gst::ClockTime> {
        self.parent_query_position()
    }

    fn query_stop(&self) -> Option<gst::ClockTime> {
        self.parent_query_stop()
    }

    fn create_rtpbin(&self) -> Option<gst::Element> {
        self.parent_create_rtpbin()
    }

    fn setup_rtpbin(&self, rtpbin: &gst::Element) -> Result<(), gst::LoggableError> {
        self.parent_setup_rtpbin(rtpbin)
    }

    fn setup_sdp(
        &self,
        sdp: &mut gst_sdp::SDPMessageRef,
        info: &SDPInfo,
    ) -> Result<(), gst::LoggableError> {
        self.parent_setup_sdp(sdp, info)
    }

    fn new_stream(&self, stream: &crate::RTSPStream) {
        self.parent_new_stream(stream);
    }

    fn removed_stream(&self, stream: &crate::RTSPStream) {
        self.parent_removed_stream(stream);
    }

    fn prepared(&self) {
        self.parent_prepared();
    }

    fn unprepared(&self) {
        self.parent_unprepared();
    }

    fn target_state(&self, state: gst::State) {
        self.parent_target_state(state);
    }

    fn new_state(&self, state: gst::State) {
        self.parent_new_state(state);
    }

    fn handle_sdp(&self, sdp: &gst_sdp::SDPMessageRef) -> Result<(), gst::LoggableError> {
        self.parent_handle_sdp(sdp)
    }
}

pub trait RTSPMediaImplExt: ObjectSubclass {
    fn parent_handle_message(&self, message: &gst::MessageRef) -> bool;
    fn parent_prepare(&self, thread: &RTSPThread) -> Result<(), gst::LoggableError>;
    fn parent_unprepare(&self) -> Result<(), gst::LoggableError>;
    fn parent_suspend(&self) -> Result<(), gst::LoggableError>;
    fn parent_unsuspend(&self) -> Result<(), gst::LoggableError>;
    // TODO missing: convert_range

    fn parent_query_position(&self) -> Option<gst::ClockTime>;
    fn parent_query_stop(&self) -> Option<gst::ClockTime>;
    fn parent_create_rtpbin(&self) -> Option<gst::Element>;
    fn parent_setup_rtpbin(&self, rtpbin: &gst::Element) -> Result<(), gst::LoggableError>;
    fn parent_setup_sdp(
        &self,
        sdp: &mut gst_sdp::SDPMessageRef,
        info: &SDPInfo,
    ) -> Result<(), gst::LoggableError>;
    fn parent_new_stream(&self, stream: &crate::RTSPStream);
    fn parent_removed_stream(&self, stream: &crate::RTSPStream);
    fn parent_prepared(&self);
    fn parent_unprepared(&self);
    fn parent_target_state(&self, state: gst::State);
    fn parent_new_state(&self, state: gst::State);
    fn parent_handle_sdp(&self, sdp: &gst_sdp::SDPMessageRef) -> Result<(), gst::LoggableError>;
}

impl<T: RTSPMediaImpl> RTSPMediaImplExt for T {
    fn parent_handle_message(&self, message: &gst::MessageRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).handle_message {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    message.as_ptr() as *mut _,
                ))
            } else {
                false
            }
        }
    }

    fn parent_prepare(&self, thread: &RTSPThread) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).prepare {
                gst::result_from_gboolean!(
                    f(
                        self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
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

    fn parent_unprepare(&self) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).unprepare {
                gst::result_from_gboolean!(
                    f(self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0),
                    gst::CAT_RUST,
                    "Parent function `unprepare` failed"
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_suspend(&self) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).suspend {
                gst::result_from_gboolean!(
                    f(self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0),
                    gst::CAT_RUST,
                    "Parent function `suspend` failed"
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_unsuspend(&self) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).unsuspend {
                gst::result_from_gboolean!(
                    f(self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0),
                    gst::CAT_RUST,
                    "Parent function `unsuspend` failed"
                )
            } else {
                Ok(())
            }
        }
    }

    // TODO missing: convert_range

    fn parent_query_position(&self) -> Option<gst::ClockTime> {
        unsafe {
            use std::mem;

            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).query_position {
                let mut position = mem::MaybeUninit::uninit();
                if f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    position.as_mut_ptr(),
                ) == glib::ffi::GFALSE
                {
                    None
                } else {
                    from_glib(position.assume_init() as u64)
                }
            } else {
                None
            }
        }
    }

    fn parent_query_stop(&self) -> Option<gst::ClockTime> {
        unsafe {
            use std::mem;

            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).query_stop {
                let mut stop = mem::MaybeUninit::uninit();
                if f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    stop.as_mut_ptr(),
                ) == glib::ffi::GFALSE
                {
                    None
                } else {
                    from_glib(stop.assume_init() as u64)
                }
            } else {
                None
            }
        }
    }

    fn parent_create_rtpbin(&self) -> Option<gst::Element> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            let f = (*parent_class)
                .create_rtpbin
                .expect("No `create_rtpbin` virtual method implementation in parent class");

            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<RTSPMedia>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_setup_rtpbin(&self, rtpbin: &gst::Element) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).setup_rtpbin {
                let ptr = rtpbin.to_glib_none().0;

                // The C code assumes to pass a floating reference around so let's make sure we do
                glib::gobject_ffi::g_object_force_floating(ptr as *mut _);

                let res = gst::result_from_gboolean!(
                    f(
                        self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                        ptr
                    ),
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
        sdp: &mut gst_sdp::SDPMessageRef,
        info: &SDPInfo,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            let f = (*parent_class)
                .setup_sdp
                .expect("No `setup_sdp` virtual method implementation in parent class");

            gst::result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    sdp as *mut _ as *mut gst_sdp::ffi::GstSDPMessage,
                    info.0.as_ptr()
                ),
                gst::CAT_RUST,
                "Parent function `setup_sdp` failed"
            )
        }
    }

    fn parent_new_stream(&self, stream: &crate::RTSPStream) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).new_stream {
                f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    stream.to_glib_none().0,
                );
            }
        }
    }

    fn parent_removed_stream(&self, stream: &crate::RTSPStream) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).removed_stream {
                f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    stream.to_glib_none().0,
                );
            }
        }
    }

    fn parent_prepared(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).prepared {
                f(self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0);
            }
        }
    }

    fn parent_unprepared(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).unprepared {
                f(self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0);
            }
        }
    }

    fn parent_target_state(&self, state: gst::State) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).target_state {
                f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    state.into_glib(),
                );
            }
        }
    }

    fn parent_new_state(&self, state: gst::State) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            if let Some(f) = (*parent_class).new_state {
                f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
                    state.into_glib(),
                );
            }
        }
    }

    fn parent_handle_sdp(&self, sdp: &gst_sdp::SDPMessageRef) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPMediaClass;
            let f = (*parent_class)
                .handle_sdp
                .expect("No `handle_sdp` virtual method implementation in parent class");

            gst::result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<RTSPMedia>().to_glib_none().0,
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
        Self::parent_class_init::<T>(klass);
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
}

unsafe extern "C" fn media_handle_message<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    message: *mut gst::ffi::GstMessage,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.handle_message(gst::MessageRef::from_ptr(message))
        .into_glib()
}

unsafe extern "C" fn media_prepare<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    thread: *mut ffi::GstRTSPThread,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.prepare(&from_glib_borrow(thread)) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_unprepare<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.unprepare() {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_suspend<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.suspend() {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_unsuspend<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.unsuspend() {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_query_position<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    position: *mut i64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.query_position() {
        Some(pos) => {
            *position = pos.into_glib() as i64;
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
    let imp = instance.imp();

    match imp.query_stop() {
        Some(s) => {
            *stop = s.into_glib() as i64;
            glib::ffi::GTRUE
        }
        None => glib::ffi::GFALSE,
    }
}

unsafe extern "C" fn media_create_rtpbin<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
) -> *mut gst::ffi::GstElement {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let res: *mut gst::ffi::GstElement = imp.create_rtpbin().to_glib_full();

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
    let imp = instance.imp();

    // If the rtpbin was floating before make sure it is not anymore for now so
    // we don't accidentally take ownership of it somewhere along the line
    if glib::gobject_ffi::g_object_is_floating(rtpbin as *mut _) != glib::ffi::GFALSE {
        glib::gobject_ffi::g_object_ref_sink(rtpbin as *mut _);
    }

    let res = match imp.setup_rtpbin(&from_glib_borrow(rtpbin)) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
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
    let imp = instance.imp();

    match imp.setup_sdp(
        &mut *(sdp as *mut gst_sdp::SDPMessageRef),
        &SDPInfo(ptr::NonNull::new(info).expect("NULL SDPInfo")),
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn media_new_stream<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    stream: *mut ffi::GstRTSPStream,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.new_stream(&from_glib_borrow(stream));
}

unsafe extern "C" fn media_removed_stream<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    stream: *mut ffi::GstRTSPStream,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.removed_stream(&from_glib_borrow(stream));
}

unsafe extern "C" fn media_prepared<T: RTSPMediaImpl>(ptr: *mut ffi::GstRTSPMedia) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.prepared();
}

unsafe extern "C" fn media_unprepared<T: RTSPMediaImpl>(ptr: *mut ffi::GstRTSPMedia) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.unprepared();
}

unsafe extern "C" fn media_target_state<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    state: gst::ffi::GstState,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.target_state(from_glib(state));
}

unsafe extern "C" fn media_new_state<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    state: gst::ffi::GstState,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.new_state(from_glib(state));
}

unsafe extern "C" fn media_handle_sdp<T: RTSPMediaImpl>(
    ptr: *mut ffi::GstRTSPMedia,
    sdp: *mut gst_sdp::ffi::GstSDPMessage,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.handle_sdp(&*(sdp as *const gst_sdp::SDPMessageRef)) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            err.log_with_imp(imp);
            glib::ffi::GFALSE
        }
    }
}
