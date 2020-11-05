use glib_sys;
use gst_audio_sys;

use glib::translate::*;

use glib::subclass::prelude::*;
use gst::subclass::prelude::*;
use gst::LoggableError;
use gst_base::subclass::prelude::*;

use AudioRingBufferSpec;
use AudioSink;

pub trait AudioSinkImpl: AudioSinkImplExt + BaseSinkImpl {
    fn close(&self, sink: &AudioSink) -> Result<(), LoggableError> {
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
        AudioSinkImplExt::parent_prepare(self, sink, spec)
    }

    fn unprepare(&self, sink: &AudioSink) -> Result<(), LoggableError> {
        self.parent_unprepare(sink)
    }

    fn write(&self, sink: &AudioSink, audio_data: &[u8]) -> Result<i32, LoggableError> {
        self.parent_write(sink, audio_data)
    }

    fn reset(&self, sink: &AudioSink) {
        self.parent_reset(sink)
    }
}

pub trait AudioSinkImplExt {
    fn parent_close(&self, sink: &AudioSink) -> Result<(), LoggableError>;
    fn parent_delay(&self, sink: &AudioSink) -> u32;
    fn parent_open(&self, sink: &AudioSink) -> Result<(), LoggableError>;
    fn parent_prepare(
        &self,
        sink: &AudioSink,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError>;
    fn parent_unprepare(&self, sink: &AudioSink) -> Result<(), LoggableError>;
    fn parent_write(&self, sink: &AudioSink, audio_data: &[u8]) -> Result<i32, LoggableError>;
    fn parent_reset(&self, sink: &AudioSink);
}

impl<T: AudioSinkImpl> AudioSinkImplExt for T {
    fn parent_close(&self, sink: &AudioSink) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
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
            let data = T::type_data();
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
            let data = T::type_data();
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
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            let f = match (*parent_class).prepare {
                Some(f) => f,
                None => return Ok(()),
            };
            gst_result_from_gboolean!(
                f(sink.to_glib_none().0, &mut spec.0),
                gst::CAT_RUST,
                "Failed to prepare element using the parent function"
            )
        }
    }

    fn parent_unprepare(&self, sink: &AudioSink) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
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
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            let f = match (*parent_class).write {
                Some(f) => f,
                None => return Ok(-1),
            };
            let buffer_ptr = buffer.as_ptr() as *const _ as *mut _;
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

    fn parent_reset(&self, sink: &AudioSink) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSinkClass;
            if let Some(f) = (*parent_class).reset {
                f(sink.to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: AudioSinkImpl> IsSubclassable<T> for AudioSink
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <gst_base::BaseSink as IsSubclassable<T>>::override_vfuncs(klass);
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
    ptr: *mut gst_audio_sys::GstAudioSink,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.close(&wrap) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_delay<T: AudioSinkImpl>(ptr: *mut gst_audio_sys::GstAudioSink) -> u32
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), 0, { imp.delay(&wrap) })
}

unsafe extern "C" fn audiosink_open<T: AudioSinkImpl>(
    ptr: *mut gst_audio_sys::GstAudioSink,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.open(&wrap) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_prepare<T: AudioSinkImpl>(
    ptr: *mut gst_audio_sys::GstAudioSink,
    spec: *mut gst_audio_sys::GstAudioRingBufferSpec,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    let spec = &mut *(spec as *mut AudioRingBufferSpec);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match AudioSinkImpl::prepare(imp, &wrap, spec) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_unprepare<T: AudioSinkImpl>(
    ptr: *mut gst_audio_sys::GstAudioSink,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match imp.unprepare(&wrap) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosink_write<T: AudioSinkImpl>(
    ptr: *mut gst_audio_sys::GstAudioSink,
    data: glib_sys::gpointer,
    length: u32,
) -> i32
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);
    let data_slice = std::slice::from_raw_parts(data as *const u8, length as usize);

    gst_panic_to_error!(&wrap, &instance.panicked(), -1, {
        imp.write(&wrap, data_slice).unwrap_or(-1)
    })
}

unsafe extern "C" fn audiosink_reset<T: AudioSinkImpl>(ptr: *mut gst_audio_sys::GstAudioSink)
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSink> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.reset(&wrap);
    });
}
