// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::c_char;

use glib::{prelude::*, translate::*};
use gst::{LoggableError, loggable_error};
use gst_base::subclass::prelude::*;

use crate::{AudioCdSrc, ffi};

pub trait AudioCdSrcImpl: BaseSrcImpl + ObjectSubclass<Type: IsA<AudioCdSrc>> {
    fn open(&self, device: &glib::GStr) -> Result<(), LoggableError> {
        self.parent_open(device)
    }

    fn read_sector(&self, sector: i32) -> Result<gst::Buffer, LoggableError> {
        self.parent_read_sector(sector)
    }

    fn close(&self) {
        self.parent_close()
    }
}

pub trait AudioCdSrcImplExt: AudioCdSrcImpl {
    fn parent_open(&self, device: &glib::GStr) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioCdSrcClass;
            let f = (*parent_class)
                .open
                .expect("parent function `open` should be defined");
            gst::result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<AudioCdSrc>().to_glib_none().0,
                    device.to_glib_none().0
                ),
                gst::CAT_RUST,
                "Failed to open element using the parent function"
            )
        }
    }

    fn parent_read_sector(&self, sector: i32) -> Result<gst::Buffer, LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioCdSrcClass;
            let f = (*parent_class)
                .read_sector
                .expect("parent function `read_sector` should be defined");
            let ptr = f(
                self.obj().unsafe_cast_ref::<AudioCdSrc>().to_glib_none().0,
                sector,
            );
            Option::<_>::from_glib_full(ptr).ok_or_else(|| {
                loggable_error!(
                    gst::CAT_RUST,
                    "Failed to read sector using the parent function"
                )
            })
        }
    }

    fn parent_close(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioCdSrcClass;
            let f = (*parent_class)
                .close
                .expect("parent function `close` should be defined");
            f(self.obj().unsafe_cast_ref::<AudioCdSrc>().to_glib_none().0)
        }
    }
}

impl<T: AudioCdSrcImpl> AudioCdSrcImplExt for T {}

unsafe impl<T: AudioCdSrcImpl> IsSubclassable<T> for AudioCdSrc {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.open = Some(audiocdsrc_open::<T>);
        klass.close = Some(audiocdsrc_close::<T>);
        klass.read_sector = Some(audiocdsrc_read_sector::<T>);
    }
}

unsafe extern "C" fn audiocdsrc_open<T: AudioCdSrcImpl>(
    ptr: *mut ffi::GstAudioCdSrc,
    device: *const c_char,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, false, {
            match imp.open(from_glib_none(device)) {
                Ok(()) => true,
                Err(err) => {
                    err.log_with_imp(imp);
                    false
                }
            }
        })
        .into_glib()
    }
}

unsafe extern "C" fn audiocdsrc_close<T: AudioCdSrcImpl>(ptr: *mut ffi::GstAudioCdSrc) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, (), { imp.close() });
    }
}

unsafe extern "C" fn audiocdsrc_read_sector<T: AudioCdSrcImpl>(
    ptr: *mut ffi::GstAudioCdSrc,
    sector: i32,
) -> *mut gst::ffi::GstBuffer {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, None, {
            match imp.read_sector(sector) {
                Ok(buffer) => Some(buffer),
                Err(err) => {
                    err.log_with_imp(imp);
                    None
                }
            }
        })
        .into_glib_ptr()
    }
}
