// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use gst::LoggableError;
use gst_base::subclass::prelude::*;

use super::prelude::*;
use crate::{AudioRingBufferSpec, AudioSink};

pub trait AudioSinkImpl: AudioSinkImplExt + AudioBaseSinkImpl {
    fn close(&self) -> Result<(), LoggableError> {
        self.parent_close()
    }

    fn delay(&self) -> u32 {
        self.parent_delay()
    }

    fn open(&self) -> Result<(), LoggableError> {
        self.parent_open()
    }

    fn prepare(&self, spec: &mut AudioRingBufferSpec) -> Result<(), LoggableError> {
        AudioSinkImplExt::parent_prepare(self, spec)
    }

    fn unprepare(&self) -> Result<(), LoggableError> {
        self.parent_unprepare()
    }

    fn write(&self, audio_data: &[u8]) -> Result<i32, LoggableError> {
        self.parent_write(audio_data)
    }

    fn reset(&self) {
        self.parent_reset()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AudioSinkImplExt> Sealed for T {}
}

pub trait AudioSinkImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_close(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).close {
                None => return Ok(()),
                Some(f) => f,
            };
            gst::result_from_gboolean!(
                f(self.obj().unsafe_cast_ref::<AudioSink>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to close element using the parent function"
            )
        }
    }

    fn parent_delay(&self) -> u32 {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).delay {
                Some(f) => f,
                None => return 0,
            };
            f(self.obj().unsafe_cast_ref::<AudioSink>().to_glib_none().0)
        }
    }

    fn parent_open(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).open {
                Some(f) => f,
                None => return Ok(()),
            };
            gst::result_from_gboolean!(
                f(self.obj().unsafe_cast_ref::<AudioSink>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to open element using the parent function"
            )
        }
    }

    fn parent_prepare(&self, spec: &mut AudioRingBufferSpec) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).prepare {
                Some(f) => f,
                None => return Ok(()),
            };
            gst::result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<AudioSink>().to_glib_none().0,
                    &mut spec.0
                ),
                gst::CAT_RUST,
                "Failed to prepare element using the parent function"
            )
        }
    }

    fn parent_unprepare(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).unprepare {
                Some(f) => f,
                None => {
                    return Err(gst::loggable_error!(
                        gst::CAT_RUST,
                        "Unprepare is not implemented!"
                    ))
                }
            };
            gst::result_from_gboolean!(
                f(self.obj().unsafe_cast_ref::<AudioSink>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to unprepare element using the parent function"
            )
        }
    }

    fn parent_write(&self, buffer: &[u8]) -> Result<i32, LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSinkClass;
            let f = match (*parent_class).write {
                Some(f) => f,
                None => return Ok(-1),
            };
            let buffer_ptr = buffer.as_ptr() as glib::ffi::gpointer;
            let ret = f(
                self.obj().unsafe_cast_ref::<AudioSink>().to_glib_none().0,
                buffer_ptr,
                buffer.len() as u32,
            );
            if ret > 0 {
                Ok(ret)
            } else {
                Err(gst::loggable_error!(
                    gst::CAT_RUST,
                    "Failed to write using the parent function"
                ))
            }
        }
    }

    fn parent_reset(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSinkClass;
            if let Some(f) = (*parent_class).reset {
                f(self.obj().unsafe_cast_ref::<AudioSink>().to_glib_none().0)
            }
        }
    }
}

impl<T: AudioSinkImpl> AudioSinkImplExt for T {}

unsafe impl<T: AudioSinkImpl> IsSubclassable<T> for AudioSink {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.close = Some(audiosink_close::<T>);
        klass.delay = Some(audiosink_delay::<T>);
        klass.open = Some(audiosink_open::<T>);
        klass.prepare = Some(audiosink_prepare::<T>);
        klass.unprepare = Some(audiosink_unprepare::<T>);
        klass.write = Some(audiosink_write::<T>);
        klass.reset = Some(audiosink_reset::<T>);
    }
}

unsafe extern "C" fn audiosink_close<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.close() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audiosink_delay<T: AudioSinkImpl>(ptr: *mut ffi::GstAudioSink) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, 0, { imp.delay() })
}

unsafe extern "C" fn audiosink_open<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.open() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audiosink_prepare<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
    spec: *mut ffi::GstAudioRingBufferSpec,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let spec = &mut *(spec as *mut AudioRingBufferSpec);

    gst::panic_to_error!(imp, false, {
        match AudioSinkImpl::prepare(imp, spec) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audiosink_unprepare<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.unprepare() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audiosink_write<T: AudioSinkImpl>(
    ptr: *mut ffi::GstAudioSink,
    data: glib::ffi::gpointer,
    length: u32,
) -> i32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let data_slice = if length == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(data as *const u8, length as usize)
    };

    gst::panic_to_error!(imp, -1, { imp.write(data_slice).unwrap_or(-1) })
}

unsafe extern "C" fn audiosink_reset<T: AudioSinkImpl>(ptr: *mut ffi::GstAudioSink) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, (), {
        imp.reset();
    });
}
