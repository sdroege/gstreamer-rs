use glib_sys;
use gst_audio_sys;

use std::mem;

use glib::translate::*;

use glib::subclass::prelude::*;
use gst::subclass::prelude::*;
use gst::LoggableError;
use gst_base::subclass::prelude::*;

use AudioRingBufferSpec;
use AudioSrc;

pub trait AudioSrcImpl: AudioSrcImplExt + BaseSrcImpl {
    fn close(&self, src: &AudioSrc) -> Result<(), LoggableError> {
        self.parent_close(src)
    }

    fn delay(&self, src: &AudioSrc) -> u32 {
        self.parent_delay(src)
    }

    fn open(&self, src: &AudioSrc) -> Result<(), LoggableError> {
        self.parent_open(src)
    }

    fn prepare(&self, src: &AudioSrc, spec: &mut AudioRingBufferSpec) -> Result<(), LoggableError> {
        AudioSrcImplExt::parent_prepare(self, src, spec)
    }

    fn unprepare(&self, src: &AudioSrc) -> Result<(), LoggableError> {
        self.parent_unprepare(src)
    }

    fn read(
        &self,
        src: &AudioSrc,
        audio_data: &mut [u8],
    ) -> Result<(u32, gst::ClockTime), LoggableError> {
        self.parent_read(src, audio_data)
    }

    fn reset(&self, src: &AudioSrc) {
        self.parent_reset(src)
    }
}

pub trait AudioSrcImplExt {
    fn parent_close(&self, src: &AudioSrc) -> Result<(), LoggableError>;
    fn parent_delay(&self, src: &AudioSrc) -> u32;
    fn parent_open(&self, src: &AudioSrc) -> Result<(), LoggableError>;
    fn parent_prepare(
        &self,
        src: &AudioSrc,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError>;
    fn parent_unprepare(&self, src: &AudioSrc) -> Result<(), LoggableError>;
    fn parent_read(
        &self,
        src: &AudioSrc,
        audio_data: &mut [u8],
    ) -> Result<(u32, gst::ClockTime), LoggableError>;
    fn parent_reset(&self, src: &AudioSrc);
}

impl<T: AudioSrcImpl> AudioSrcImplExt for T {
    fn parent_close(&self, src: &AudioSrc) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSrcClass;
            let f = match (*parent_class).close {
                None => return Ok(()),
                Some(f) => f,
            };
            gst_result_from_gboolean!(
                f(src.to_glib_none().0),
                gst::CAT_RUST,
                "Failed to close element using the parent function"
            )
        }
    }

    fn parent_delay(&self, src: &AudioSrc) -> u32 {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSrcClass;
            let f = match (*parent_class).delay {
                Some(f) => f,
                None => return 0,
            };
            f(src.to_glib_none().0)
        }
    }

    fn parent_open(&self, src: &AudioSrc) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSrcClass;
            let f = match (*parent_class).open {
                Some(f) => f,
                None => return Ok(()),
            };
            gst_result_from_gboolean!(
                f(src.to_glib_none().0),
                gst::CAT_RUST,
                "Failed to open element using the parent function"
            )
        }
    }

    fn parent_prepare(
        &self,
        src: &AudioSrc,
        spec: &mut AudioRingBufferSpec,
    ) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSrcClass;
            let f = match (*parent_class).prepare {
                Some(f) => f,
                None => return Ok(()),
            };
            gst_result_from_gboolean!(
                f(src.to_glib_none().0, &mut spec.0),
                gst::CAT_RUST,
                "Failed to prepare element using the parent function"
            )
        }
    }

    fn parent_unprepare(&self, src: &AudioSrc) -> Result<(), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSrcClass;
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
                f(src.to_glib_none().0),
                gst::CAT_RUST,
                "Failed to unprepare element using the parent function"
            )
        }
    }

    fn parent_read(
        &self,
        src: &AudioSrc,
        buffer: &mut [u8],
    ) -> Result<(u32, gst::ClockTime), LoggableError> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSrcClass;
            let f = match (*parent_class).read {
                Some(f) => f,
                None => return Ok((0, gst::CLOCK_TIME_NONE)),
            };
            let buffer_ptr = buffer.as_mut_ptr() as *mut _;
            let mut timestamp = mem::MaybeUninit::uninit();
            let ret = f(
                src.to_glib_none().0,
                buffer_ptr,
                buffer.len() as u32,
                timestamp.as_mut_ptr(),
            );
            if ret > 0 {
                Ok((ret, from_glib(timestamp.assume_init())))
            } else {
                Err(gst::gst_loggable_error!(
                    gst::CAT_RUST,
                    "Failed to read using the parent function"
                ))
            }
        }
    }

    fn parent_reset(&self, src: &AudioSrc) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_audio_sys::GstAudioSrcClass;
            if let Some(f) = (*parent_class).reset {
                f(src.to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: AudioSrcImpl> IsSubclassable<T> for AudioSrc
where
    <T as ObjectSubclass>::Instance: PanicPoison,
{
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <gst_base::BaseSrc as IsSubclassable<T>>::override_vfuncs(klass);
        unsafe {
            let klass = &mut *(klass.as_mut() as *mut gst_audio_sys::GstAudioSrcClass);
            klass.close = Some(audiosrc_close::<T>);
            klass.delay = Some(audiosrc_delay::<T>);
            klass.open = Some(audiosrc_open::<T>);
            klass.prepare = Some(audiosrc_prepare::<T>);
            klass.unprepare = Some(audiosrc_unprepare::<T>);
            klass.read = Some(audiosrc_read::<T>);
            klass.reset = Some(audiosrc_reset::<T>);
        }
    }
}

unsafe extern "C" fn audiosrc_close<T: AudioSrcImpl>(
    ptr: *mut gst_audio_sys::GstAudioSrc,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

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

unsafe extern "C" fn audiosrc_delay<T: AudioSrcImpl>(ptr: *mut gst_audio_sys::GstAudioSrc) -> u32
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), 0, { imp.delay(&wrap) })
}

unsafe extern "C" fn audiosrc_open<T: AudioSrcImpl>(
    ptr: *mut gst_audio_sys::GstAudioSrc,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

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

unsafe extern "C" fn audiosrc_prepare<T: AudioSrcImpl>(
    ptr: *mut gst_audio_sys::GstAudioSrc,
    spec: *mut gst_audio_sys::GstAudioRingBufferSpec,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    let spec = &mut *(spec as *mut AudioRingBufferSpec);

    gst_panic_to_error!(&wrap, &instance.panicked(), false, {
        match AudioSrcImpl::prepare(imp, &wrap, spec) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .to_glib()
}

unsafe extern "C" fn audiosrc_unprepare<T: AudioSrcImpl>(
    ptr: *mut gst_audio_sys::GstAudioSrc,
) -> glib_sys::gboolean
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

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

unsafe extern "C" fn audiosrc_read<T: AudioSrcImpl>(
    ptr: *mut gst_audio_sys::GstAudioSrc,
    data: glib_sys::gpointer,
    length: u32,
    timestamp: *mut gst_sys::GstClockTime,
) -> u32
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);
    let data_slice = std::slice::from_raw_parts_mut(data as *mut u8, length as usize);

    gst_panic_to_error!(&wrap, &instance.panicked(), 0, {
        let (res, timestamp_res) = imp
            .read(&wrap, data_slice)
            .unwrap_or((0, gst::CLOCK_TIME_NONE));
        *timestamp = timestamp_res.to_glib();

        res
    })
}

unsafe extern "C" fn audiosrc_reset<T: AudioSrcImpl>(ptr: *mut gst_audio_sys::GstAudioSrc)
where
    T::Instance: PanicPoison,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<AudioSrc> = from_glib_borrow(ptr);

    gst_panic_to_error!(&wrap, &instance.panicked(), (), {
        imp.reset(&wrap);
    });
}
