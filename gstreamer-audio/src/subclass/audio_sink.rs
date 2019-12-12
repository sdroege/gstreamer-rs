use glib_sys;
use gst_audio_sys;

use glib::translate::*;

use glib::subclass::prelude::*;
use gst::subclass::prelude::*;
use gst::LoggableError;

use std::mem;

use AudioRingBufferSpec;
pub use AudioSink;
use AudioSinkClass;

pub trait AudioSinkImpl: AudioSinkImplExt + ElementImpl + Send + Sync + 'static {
    fn close(&self, sink: &mut AudioSink) -> Result<(), LoggableError> {
        self.parent_close(sink)
    }

    fn delay(&self, sink: &AudioSink) -> u32 {
        self.parent_delay(sink)
    }

    fn open(&self, sink: &AudioSink) -> Result<(), LoggableError> {
        self.parent_open(sink)
    }

    fn prepare(
        &self,
        sink: &AudioSink,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError> {
        self.parent_prepare(sink, spec)
    }

    fn unprepare(&self, sink: &AudioSink) -> Result<(), LoggableError> {
        self.parent_unprepare(sink)
    }

    fn write(&self, &AudioSink, &[u8]) -> Result<i32, LoggableError>;
}

pub trait AudioSinkImplExt {
    fn parent_close(&self, sink: &mut AudioSink) -> Result<(), LoggableError>;
    fn parent_delay(&self, sink: &AudioSink) -> u32;
    fn parent_open(&self, sink: &AudioSink) -> Result<(), LoggableError>;
    fn parent_prepare(
        &self,
        sink: &AudioSink,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError>;
    fn parent_unprepare(&self, sink: &AudioSink) -> Result<(), LoggableError>;
    fn parent_write(&self, sink: &AudioSink, &[u8]) -> Result<i32, LoggableError>;
}

impl<T: AudioSinkImpl + ObjectImpl> AudioSinkImplExt for T {
    fn parent_close(&self, sink: &mut AudioSink) -> Result<(), LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            let f = match (*parent_class).close {
                None => return Ok(()),
                Some(f) => f,
            };
            gst_result_from_gboolean!(
                f(sink.to_glib_none().0),
                gst::CAT_RUST,
                "Failed to close element using the parent function"
            )
        }
    }

    fn parent_delay(&self, sink: &AudioSink) -> u32 {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            let f = match (*parent_class).delay {
                Some(f) => f,
                None => return 0,
            };
            f(sink.to_glib_none().0)
        }
    }

    fn parent_open(&self, sink: &AudioSink) -> Result<(), LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            let f = match (*parent_class).open {
                Some(f) => f,
                None => return Ok(()),
            };
            gst_result_from_gboolean!(
                f(sink.to_glib_none().0),
                gst::CAT_RUST,
                "Failed to open element using the parent function"
            )
        }
    }

    fn parent_prepare(
        &self,
        sink: &AudioSink,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            let f = match (*parent_class).prepare {
                Some(f) => f,
                None => return Ok(()),
            };
            gst_result_from_gboolean!(
                f(sink.to_glib_none().0, spec.to_glib_none_mut().0),
                gst::CAT_RUST,
                "Failed to prepare element using the parent function"
            )
        }
    }

    fn parent_unprepare(&self, sink: &AudioSink) -> Result<(), LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            let f = match (*parent_class).unprepare {
                Some(f) => f,
                None => {
                    return Err(gst::gst_loggable_error!(
                        gst::CAT_RUST,
                        "Unprepare is not implemented!"
                    ))
                }
            };
            gst_result_from_gboolean!(
                f(sink.to_glib_none().0),
                gst::CAT_RUST,
                "Failed to unprepare element using the parent function"
            )
        }
    }

    fn parent_write(&self, sink: &AudioSink, buffer: &[u8]) -> Result<i32, LoggableError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            let f = match (*parent_class).write {
                Some(f) => f,
                None => return Ok(-1),
            };
            let buffer_ptr = mem::transmute::<*const u8, *mut std::ffi::c_void>(buffer.as_ptr());
            let ret = f(sink.to_glib_none().0, buffer_ptr, buffer.len() as u32);
            if ret > 0 {
                Ok(ret)
            } else {
                Err(gst::gst_loggable_error!(
                    gst::CAT_RUST,
                    "Failed to write using the parent function"
                ))
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + AudioSinkImpl> IsSubclassable<T> for AudioSinkClass
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(&mut self) {
        <gst::ElementClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_audio_sys::GstAudioSinkClass);
            klass.close = Some(audiosink_close::<T>);
            klass.delay = Some(audiosink_delay::<T>);
            klass.open = Some(audiosink_open::<T>);
            klass.prepare = Some(audiosink_prepare::<T>);
            klass.unprepare = Some(audiosink_unprepare::<T>);
            klass.write = Some(audiosink_write::<T>);
        }
    }
}

unsafe extern "C" fn audiosink_close<T: ObjectSubclass>(
    ptr: *mut gst_audio_sys::GstAudioSink,
) -> glib_sys::gboolean
where
    T: AudioSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let mut wrap: AudioSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.close(&mut wrap).is_ok()
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_delay<T: ObjectSubclass>(
    ptr: *mut gst_audio_sys::GstAudioSink,
) -> u32
where
    T: AudioSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: AudioSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), 0, { imp.delay(&wrap) })
}

unsafe extern "C" fn audiosink_open<T: ObjectSubclass>(
    ptr: *mut gst_audio_sys::GstAudioSink,
) -> glib_sys::gboolean
where
    T: AudioSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: AudioSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.open(&wrap).is_ok()
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_prepare<T: ObjectSubclass>(
    ptr: *mut gst_audio_sys::GstAudioSink,
    spec: *mut gst_audio_sys::GstAudioRingBufferSpec,
) -> glib_sys::gboolean
where
    T: AudioSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: AudioSink = from_glib_borrow(ptr);

    let mut spec_rust = AudioRingBufferSpec::from_glib_none(spec);

    let res = gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.prepare(&wrap, &mut spec_rust).is_ok()
    })
    .to_glib();
    spec_rust.copy_into(spec);
    res
}

unsafe extern "C" fn audiosink_unprepare<T: ObjectSubclass>(
    ptr: *mut gst_audio_sys::GstAudioSink,
) -> glib_sys::gboolean
where
    T: AudioSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: AudioSink = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        imp.unprepare(&wrap).is_ok()
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_write<T: ObjectSubclass>(
    ptr: *mut gst_audio_sys::GstAudioSink,
    data: glib_sys::gpointer,
    length: u32,
) -> i32
where
    T: AudioSinkImpl,
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: AudioSink = from_glib_borrow(ptr);
    let data_slice = std::slice::from_raw_parts(data as *const u8, length as usize);

    gst_panic_to_error!(&wrap, &instance.panicked(), -1, {
        imp.write(&wrap, data_slice).unwrap_or(-1)
    })
}
