// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use gst::subclass::prelude::*;

use crate::prelude::*;
use crate::RTPBaseDepayload;
use std::ptr;

pub trait RTPBaseDepayloadImpl: RTPBaseDepayloadImplExt + ElementImpl {
    fn set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_caps(caps)
    }

    fn handle_event(&self, event: gst::Event) -> bool {
        self.parent_handle_event(event)
    }

    fn packet_lost(&self, event: &gst::EventRef) -> bool {
        self.parent_packet_lost(event)
    }

    fn process_rtp_packet(
        &self,
        rtp_buffer: &crate::RTPBuffer<crate::rtp_buffer::Readable>,
    ) -> Option<gst::Buffer> {
        self.parent_process_rtp_packet(rtp_buffer)
    }
}

pub trait RTPBaseDepayloadImplExt: ObjectSubclass {
    fn parent_set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError>;

    fn parent_handle_event(&self, event: gst::Event) -> bool;

    fn parent_packet_lost(&self, event: &gst::EventRef) -> bool;

    fn parent_process_rtp_packet(
        &self,
        rtp_buffer: &crate::RTPBuffer<crate::rtp_buffer::Readable>,
    ) -> Option<gst::Buffer>;
}

impl<T: RTPBaseDepayloadImpl> RTPBaseDepayloadImplExt for T {
    fn parent_set_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBaseDepayloadClass;
            (*parent_class)
                .set_caps
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj()
                                .unsafe_cast_ref::<RTPBaseDepayload>()
                                .to_glib_none()
                                .0,
                            caps.to_glib_none().0
                        ),
                        gst::CAT_RUST,
                        "Parent function `set_caps` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_handle_event(&self, event: gst::Event) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBaseDepayloadClass;
            (*parent_class)
                .handle_event
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<RTPBaseDepayload>()
                            .to_glib_none()
                            .0,
                        event.into_glib_ptr(),
                    ))
                })
                .unwrap_or(false)
        }
    }

    fn parent_packet_lost(&self, event: &gst::EventRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBaseDepayloadClass;
            (*parent_class)
                .packet_lost
                .map(|f| {
                    from_glib(f(
                        self.obj()
                            .unsafe_cast_ref::<RTPBaseDepayload>()
                            .to_glib_none()
                            .0,
                        event.as_mut_ptr(),
                    ))
                })
                .unwrap_or(true)
        }
    }

    fn parent_process_rtp_packet(
        &self,
        rtp_buffer: &crate::RTPBuffer<crate::rtp_buffer::Readable>,
    ) -> Option<gst::Buffer> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPBaseDepayloadClass;

            let f = (*parent_class)
                .process_rtp_packet
                .expect("no parent \"process\" implementation");

            from_glib_full(f(
                self.obj()
                    .unsafe_cast_ref::<crate::RTPBaseDepayload>()
                    .to_glib_none()
                    .0,
                mut_override(rtp_buffer.as_ptr()),
            ))
        }
    }
}

unsafe impl<T: RTPBaseDepayloadImpl> IsSubclassable<T> for RTPBaseDepayload {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();

        klass.process = None;
        klass.process_rtp_packet = Some(rtp_base_depayload_process_rtp_packet::<T>);
        klass.set_caps = Some(rtp_base_depayload_set_caps::<T>);
        klass.handle_event = Some(rtp_base_depayload_handle_event::<T>);
        klass.packet_lost = Some(rtp_base_depayload_packet_lost::<T>);
    }
}

unsafe extern "C" fn rtp_base_depayload_set_caps<T: RTPBaseDepayloadImpl>(
    ptr: *mut ffi::GstRTPBaseDepayload,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let caps = from_glib_borrow(caps);

    gst::panic_to_error!(imp, false, {
        match imp.set_caps(&caps) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn rtp_base_depayload_handle_event<T: RTPBaseDepayloadImpl>(
    ptr: *mut ffi::GstRTPBaseDepayload,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, { imp.handle_event(from_glib_full(event)) }).into_glib()
}

unsafe extern "C" fn rtp_base_depayload_packet_lost<T: RTPBaseDepayloadImpl>(
    ptr: *mut ffi::GstRTPBaseDepayload,
    event: *mut gst::ffi::GstEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        imp.packet_lost(gst::EventRef::from_ptr(event))
    })
    .into_glib()
}

unsafe extern "C" fn rtp_base_depayload_process_rtp_packet<T: RTPBaseDepayloadImpl>(
    ptr: *mut ffi::GstRTPBaseDepayload,
    rtp_packet: *mut ffi::GstRTPBuffer,
) -> *mut gst::ffi::GstBuffer {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, ptr::null_mut(), {
        let bufwrap = crate::RTPBuffer::<crate::rtp_buffer::Readable>::from_glib_borrow(rtp_packet);

        imp.process_rtp_packet(&bufwrap)
            .map(|buffer| buffer.into_glib_ptr())
            .unwrap_or(ptr::null_mut())
    })
}
