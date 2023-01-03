// Take a look at the license at the top of the repository in the LICENSE file.

use std::{i32, mem, ptr};

use glib::translate::{from_glib, from_glib_full, IntoGlib, ToGlibPtr};

#[doc(alias = "gst_video_convert_sample")]
pub fn convert_sample(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: gst::ClockTime,
) -> Result<gst::Sample, glib::Error> {
    skip_assert_initialized!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_video_convert_sample(
            sample.to_glib_none().0,
            caps.to_glib_none().0,
            timeout.into_glib(),
            &mut error,
        );

        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn convert_sample_async<F>(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: Option<gst::ClockTime>,
    func: F,
) where
    F: FnOnce(Result<gst::Sample, glib::Error>) + Send + 'static,
{
    skip_assert_initialized!();
    unsafe { convert_sample_async_unsafe(sample, caps, timeout, func) }
}

pub fn convert_sample_async_local<F>(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: Option<gst::ClockTime>,
    func: F,
) where
    F: FnOnce(Result<gst::Sample, glib::Error>) + 'static,
{
    skip_assert_initialized!();
    unsafe {
        let ctx = glib::MainContext::ref_thread_default();
        let _acquire = ctx
            .acquire()
            .expect("thread default main context already acquired by another thread");

        let func = glib::thread_guard::ThreadGuard::new(func);

        convert_sample_async_unsafe(sample, caps, timeout, move |res| (func.into_inner())(res))
    }
}

unsafe fn convert_sample_async_unsafe<F>(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: Option<gst::ClockTime>,
    func: F,
) where
    F: FnOnce(Result<gst::Sample, glib::Error>) + 'static,
{
    unsafe extern "C" fn convert_sample_async_trampoline<F>(
        sample: *mut gst::ffi::GstSample,
        error: *mut glib::ffi::GError,
        user_data: glib::ffi::gpointer,
    ) where
        F: FnOnce(Result<gst::Sample, glib::Error>) + 'static,
    {
        let callback: &mut Option<F> = &mut *(user_data as *mut Option<F>);
        let callback = callback.take().unwrap();

        if error.is_null() {
            callback(Ok(from_glib_full(sample)))
        } else {
            callback(Err(from_glib_full(error)))
        }
    }
    unsafe extern "C" fn convert_sample_async_free<F>(user_data: glib::ffi::gpointer)
    where
        F: FnOnce(Result<gst::Sample, glib::Error>) + 'static,
    {
        let _: Box<Option<F>> = Box::from_raw(user_data as *mut _);
    }

    let user_data: Box<Option<F>> = Box::new(Some(func));

    ffi::gst_video_convert_sample_async(
        sample.to_glib_none().0,
        caps.to_glib_none().0,
        timeout.into_glib(),
        Some(convert_sample_async_trampoline::<F>),
        Box::into_raw(user_data) as glib::ffi::gpointer,
        Some(convert_sample_async_free::<F>),
    );
}

pub fn convert_sample_future(
    sample: &gst::Sample,
    caps: &gst::Caps,
    timeout: Option<gst::ClockTime>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<gst::Sample, glib::Error>> + 'static>>
{
    skip_assert_initialized!();

    use futures_channel::oneshot;

    let (sender, receiver) = oneshot::channel();

    let sample = sample.clone();
    let caps = caps.clone();
    let future = async move {
        assert!(
            glib::MainContext::ref_thread_default().is_owner(),
            "Spawning futures only allowed if the thread is owning the MainContext"
        );

        convert_sample_async(&sample, &caps, timeout, move |res| {
            let _ = sender.send(res);
        });

        receiver
            .await
            .expect("Sender dropped before callback was called")
    };

    Box::pin(future)
}

#[doc(alias = "gst_video_calculate_display_ratio")]
pub fn calculate_display_ratio(
    video_width: u32,
    video_height: u32,
    video_par: gst::Fraction,
    display_par: gst::Fraction,
) -> Option<gst::Fraction> {
    skip_assert_initialized!();

    unsafe {
        let mut dar_n = mem::MaybeUninit::uninit();
        let mut dar_d = mem::MaybeUninit::uninit();

        let res: bool = from_glib(ffi::gst_video_calculate_display_ratio(
            dar_n.as_mut_ptr(),
            dar_d.as_mut_ptr(),
            video_width,
            video_height,
            video_par.numer() as u32,
            video_par.denom() as u32,
            display_par.numer() as u32,
            display_par.denom() as u32,
        ));
        if res {
            Some(gst::Fraction::new(
                dar_n.assume_init() as i32,
                dar_d.assume_init() as i32,
            ))
        } else {
            None
        }
    }
}

#[doc(alias = "gst_video_guess_framerate")]
pub fn guess_framerate(duration: gst::ClockTime) -> Option<gst::Fraction> {
    skip_assert_initialized!();

    unsafe {
        let mut dest_n = mem::MaybeUninit::uninit();
        let mut dest_d = mem::MaybeUninit::uninit();
        let res: bool = from_glib(ffi::gst_video_guess_framerate(
            duration.into_glib(),
            dest_n.as_mut_ptr(),
            dest_d.as_mut_ptr(),
        ));
        if res {
            Some(gst::Fraction::new(
                dest_n.assume_init(),
                dest_d.assume_init(),
            ))
        } else {
            None
        }
    }
}

#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
#[doc(alias = "gst_video_is_common_aspect_ratio")]
pub fn is_common_aspect_ratio(width: u32, height: u32, par: gst::Fraction) -> bool {
    skip_assert_initialized!();

    unsafe {
        from_glib(ffi::gst_video_is_common_aspect_ratio(
            width as i32,
            height as i32,
            par.numer(),
            par.denom(),
        ))
    }
}

pub fn video_make_raw_caps(
    formats: &[crate::VideoFormat],
) -> crate::VideoCapsBuilder<gst::caps::NoFeature> {
    assert_initialized_main_thread!();

    let formats = formats.iter().copied().map(|f| match f {
        crate::VideoFormat::Encoded => panic!("Invalid encoded format"),
        crate::VideoFormat::Unknown => panic!("Invalid unknown format"),
        _ => f,
    });

    crate::VideoCapsBuilder::new().format_list(formats)
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn test_convert_sample_async() {
        gst::init().unwrap();

        let l = glib::MainLoop::new(None, false);

        let mut in_buffer = gst::Buffer::with_size(320 * 240 * 4).unwrap();
        {
            let buffer = in_buffer.get_mut().unwrap();
            let mut data = buffer.map_writable().unwrap();

            for p in data.as_mut_slice().chunks_mut(4) {
                p[0] = 63;
                p[1] = 127;
                p[2] = 191;
                p[3] = 255;
            }
        }
        let in_caps = crate::VideoInfo::builder(crate::VideoFormat::Rgba, 320, 240)
            .build()
            .unwrap()
            .to_caps()
            .unwrap();
        let sample = gst::Sample::builder()
            .buffer(&in_buffer)
            .caps(&in_caps)
            .build();

        let out_caps = crate::VideoInfo::builder(crate::VideoFormat::Abgr, 320, 240)
            .build()
            .unwrap()
            .to_caps()
            .unwrap();

        let l_clone = l.clone();
        let res_store = Arc::new(Mutex::new(None));
        let res_store_clone = res_store.clone();
        convert_sample_async(&sample, &out_caps, gst::ClockTime::NONE, move |res| {
            *res_store_clone.lock().unwrap() = Some(res);
            l_clone.quit();
        });

        l.run();

        let res = res_store.lock().unwrap().take().unwrap();
        let res = res.unwrap();

        let converted_out_caps = res.caps().unwrap();
        assert_eq!(out_caps.as_ref(), converted_out_caps);
        let out_buffer = res.buffer().unwrap();
        {
            let data = out_buffer.map_readable().unwrap();

            for p in data.as_slice().chunks(4) {
                assert_eq!(p, &[255, 191, 127, 63]);
            }
        }
    }

    #[test]
    fn video_caps() {
        gst::init().unwrap();

        let caps =
            video_make_raw_caps(&[crate::VideoFormat::Nv12, crate::VideoFormat::Nv16]).build();
        assert_eq!(caps.to_string(), "video/x-raw, format=(string){ NV12, NV16 }, width=(int)[ 1, 2147483647 ], height=(int)[ 1, 2147483647 ], framerate=(fraction)[ 0/1, 2147483647/1 ]");

        #[cfg(feature = "v1_18")]
        {
            /* video_make_raw_caps() is a re-implementation so ensure it returns the same caps as the C API */
            let c_caps = unsafe {
                let formats: Vec<ffi::GstVideoFormat> =
                    [crate::VideoFormat::Nv12, crate::VideoFormat::Nv16]
                        .iter()
                        .map(|f| f.into_glib())
                        .collect();
                let caps = ffi::gst_video_make_raw_caps(formats.as_ptr(), formats.len() as u32);
                gst::Caps::from_glib_full(caps)
            };
            assert_eq!(caps, c_caps);
        }

        let caps = video_make_raw_caps(&[crate::VideoFormat::Nv12, crate::VideoFormat::Nv16])
            .width(800)
            .height(600)
            .framerate((30, 1).into())
            .build();
        assert_eq!(caps.to_string(), "video/x-raw, format=(string){ NV12, NV16 }, width=(int)800, height=(int)600, framerate=(fraction)30/1");
    }

    #[test]
    #[should_panic(expected = "Invalid encoded format")]
    fn video_caps_encoded() {
        gst::init().unwrap();
        let _caps = video_make_raw_caps(&[crate::VideoFormat::Encoded]);
    }

    #[test]
    #[should_panic(expected = "Invalid unknown format")]
    fn video_caps_unknown() {
        gst::init().unwrap();
        let _caps = video_make_raw_caps(&[crate::VideoFormat::Unknown]);
    }
}
