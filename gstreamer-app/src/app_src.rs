// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    mem, panic,
    pin::Pin,
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};

use futures_sink::Sink;
use glib::{
    ffi::{gboolean, gpointer},
    prelude::*,
    translate::*,
};

use crate::AppSrc;

#[allow(clippy::type_complexity)]
pub struct AppSrcCallbacks {
    need_data: Option<Box<dyn FnMut(&AppSrc, u32) + Send + 'static>>,
    enough_data: Option<Box<dyn Fn(&AppSrc) + Send + Sync + 'static>>,
    seek_data: Option<Box<dyn Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>>,
    panicked: AtomicBool,
    callbacks: ffi::GstAppSrcCallbacks,
}

unsafe impl Send for AppSrcCallbacks {}
unsafe impl Sync for AppSrcCallbacks {}

impl AppSrcCallbacks {
    pub fn builder() -> AppSrcCallbacksBuilder {
        skip_assert_initialized!();

        AppSrcCallbacksBuilder {
            need_data: None,
            enough_data: None,
            seek_data: None,
        }
    }
}

#[allow(clippy::type_complexity)]
#[must_use = "The builder must be built to be used"]
pub struct AppSrcCallbacksBuilder {
    need_data: Option<Box<dyn FnMut(&AppSrc, u32) + Send + 'static>>,
    enough_data: Option<Box<dyn Fn(&AppSrc) + Send + Sync + 'static>>,
    seek_data: Option<Box<dyn Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>>,
}

impl AppSrcCallbacksBuilder {
    pub fn need_data<F: FnMut(&AppSrc, u32) + Send + 'static>(self, need_data: F) -> Self {
        Self {
            need_data: Some(Box::new(need_data)),
            ..self
        }
    }

    pub fn enough_data<F: Fn(&AppSrc) + Send + Sync + 'static>(self, enough_data: F) -> Self {
        Self {
            enough_data: Some(Box::new(enough_data)),
            ..self
        }
    }

    pub fn seek_data<F: Fn(&AppSrc, u64) -> bool + Send + Sync + 'static>(
        self,
        seek_data: F,
    ) -> Self {
        Self {
            seek_data: Some(Box::new(seek_data)),
            ..self
        }
    }

    #[must_use = "Building the callbacks without using them has no effect"]
    pub fn build(self) -> AppSrcCallbacks {
        let have_need_data = self.need_data.is_some();
        let have_enough_data = self.enough_data.is_some();
        let have_seek_data = self.seek_data.is_some();

        AppSrcCallbacks {
            need_data: self.need_data,
            enough_data: self.enough_data,
            seek_data: self.seek_data,
            panicked: AtomicBool::new(false),
            callbacks: ffi::GstAppSrcCallbacks {
                need_data: if have_need_data {
                    Some(trampoline_need_data)
                } else {
                    None
                },
                enough_data: if have_enough_data {
                    Some(trampoline_enough_data)
                } else {
                    None
                },
                seek_data: if have_seek_data {
                    Some(trampoline_seek_data)
                } else {
                    None
                },
                _gst_reserved: [
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                ],
            },
        }
    }
}

unsafe extern "C" fn trampoline_need_data(
    appsrc: *mut ffi::GstAppSrc,
    length: u32,
    callbacks: gpointer,
) {
    let callbacks = callbacks as *mut AppSrcCallbacks;
    let element: Borrowed<AppSrc> = from_glib_borrow(appsrc);

    if (*callbacks).panicked.load(Ordering::Relaxed) {
        let element: Borrowed<AppSrc> = from_glib_borrow(appsrc);
        gst::subclass::post_panic_error_message(element.upcast_ref(), element.upcast_ref(), None);
        return;
    }

    if let Some(ref mut need_data) = (*callbacks).need_data {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| need_data(&element, length)));
        match result {
            Ok(result) => result,
            Err(err) => {
                (*callbacks).panicked.store(true, Ordering::Relaxed);
                gst::subclass::post_panic_error_message(
                    element.upcast_ref(),
                    element.upcast_ref(),
                    Some(err),
                );
            }
        }
    }
}

unsafe extern "C" fn trampoline_enough_data(appsrc: *mut ffi::GstAppSrc, callbacks: gpointer) {
    let callbacks = callbacks as *const AppSrcCallbacks;
    let element: Borrowed<AppSrc> = from_glib_borrow(appsrc);

    if (*callbacks).panicked.load(Ordering::Relaxed) {
        let element: Borrowed<AppSrc> = from_glib_borrow(appsrc);
        gst::subclass::post_panic_error_message(element.upcast_ref(), element.upcast_ref(), None);
        return;
    }

    if let Some(ref enough_data) = (*callbacks).enough_data {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| enough_data(&element)));
        match result {
            Ok(result) => result,
            Err(err) => {
                (*callbacks).panicked.store(true, Ordering::Relaxed);
                gst::subclass::post_panic_error_message(
                    element.upcast_ref(),
                    element.upcast_ref(),
                    Some(err),
                );
            }
        }
    }
}

unsafe extern "C" fn trampoline_seek_data(
    appsrc: *mut ffi::GstAppSrc,
    offset: u64,
    callbacks: gpointer,
) -> gboolean {
    let callbacks = callbacks as *const AppSrcCallbacks;
    let element: Borrowed<AppSrc> = from_glib_borrow(appsrc);

    if (*callbacks).panicked.load(Ordering::Relaxed) {
        let element: Borrowed<AppSrc> = from_glib_borrow(appsrc);
        gst::subclass::post_panic_error_message(element.upcast_ref(), element.upcast_ref(), None);
        return false.into_glib();
    }

    let ret = if let Some(ref seek_data) = (*callbacks).seek_data {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| seek_data(&element, offset)));
        match result {
            Ok(result) => result,
            Err(err) => {
                (*callbacks).panicked.store(true, Ordering::Relaxed);
                gst::subclass::post_panic_error_message(
                    element.upcast_ref(),
                    element.upcast_ref(),
                    Some(err),
                );

                false
            }
        }
    } else {
        false
    };

    ret.into_glib()
}

unsafe extern "C" fn destroy_callbacks(ptr: gpointer) {
    let _ = Box::<AppSrcCallbacks>::from_raw(ptr as *mut _);
}

impl AppSrc {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AppSrc`] objects.
    ///
    /// This method returns an instance of [`AppSrcBuilder`](crate::builders::AppSrcBuilder) which can be used to create [`AppSrc`] objects.
    pub fn builder() -> AppSrcBuilder {
        assert_initialized_main_thread!();
        AppSrcBuilder::new()
    }

    #[doc(alias = "gst_app_src_set_callbacks")]
    pub fn set_callbacks(&self, callbacks: AppSrcCallbacks) {
        #[cfg(not(feature = "v1_18"))]
        use glib::once_cell::sync::Lazy;
        #[cfg(not(feature = "v1_18"))]
        static SET_ONCE_QUARK: Lazy<glib::Quark> =
            Lazy::new(|| glib::Quark::from_str("gstreamer-rs-app-src-callbacks"));

        unsafe {
            let src = self.to_glib_none().0;
            #[cfg(not(feature = "v1_18"))]
            {
                // This is not thread-safe before 1.16.3, see
                // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/merge_requests/570
                if gst::version() < (1, 16, 3, 0) {
                    if !glib::gobject_ffi::g_object_get_qdata(
                        src as *mut _,
                        SET_ONCE_QUARK.into_glib(),
                    )
                    .is_null()
                    {
                        panic!("AppSrc callbacks can only be set once");
                    }

                    glib::gobject_ffi::g_object_set_qdata(
                        src as *mut _,
                        SET_ONCE_QUARK.into_glib(),
                        1 as *mut _,
                    );
                }
            }

            ffi::gst_app_src_set_callbacks(
                src,
                mut_override(&callbacks.callbacks),
                Box::into_raw(Box::new(callbacks)) as *mut _,
                Some(destroy_callbacks),
            );
        }
    }

    #[doc(alias = "gst_app_src_set_latency")]
    pub fn set_latency(
        &self,
        min: impl Into<Option<gst::ClockTime>>,
        max: impl Into<Option<gst::ClockTime>>,
    ) {
        unsafe {
            ffi::gst_app_src_set_latency(
                self.to_glib_none().0,
                min.into().into_glib(),
                max.into().into_glib(),
            );
        }
    }

    #[doc(alias = "get_latency")]
    #[doc(alias = "gst_app_src_get_latency")]
    pub fn latency(&self) -> (Option<gst::ClockTime>, Option<gst::ClockTime>) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            ffi::gst_app_src_get_latency(self.to_glib_none().0, min.as_mut_ptr(), max.as_mut_ptr());
            (from_glib(min.assume_init()), from_glib(max.assume_init()))
        }
    }

    #[doc(alias = "do-timestamp")]
    #[doc(alias = "gst_base_src_set_do_timestamp")]
    pub fn set_do_timestamp(&self, timestamp: bool) {
        unsafe {
            gst_base::ffi::gst_base_src_set_do_timestamp(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSrc,
                timestamp.into_glib(),
            );
        }
    }

    #[doc(alias = "do-timestamp")]
    #[doc(alias = "gst_base_src_get_do_timestamp")]
    pub fn do_timestamp(&self) -> bool {
        unsafe {
            from_glib(gst_base::ffi::gst_base_src_get_do_timestamp(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSrc
            ))
        }
    }

    #[doc(alias = "do-timestamp")]
    pub fn connect_do_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_do_timestamp_trampoline<
            F: Fn(&AppSrc) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AppSrc::from_glib_borrow(this))
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::do-timestamp\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_do_timestamp_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    #[doc(alias = "set-automatic-eos")]
    #[doc(alias = "gst_base_src_set_automatic_eos")]
    pub fn set_automatic_eos(&self, automatic_eos: bool) {
        unsafe {
            gst_base::ffi::gst_base_src_set_automatic_eos(
                self.as_ptr() as *mut gst_base::ffi::GstBaseSrc,
                automatic_eos.into_glib(),
            );
        }
    }

    pub fn sink(&self) -> AppSrcSink {
        AppSrcSink::new(self)
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AppSrc`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AppSrcBuilder {
    builder: glib::object::ObjectBuilder<'static, AppSrc>,
    callbacks: Option<AppSrcCallbacks>,
    automatic_eos: Option<bool>,
}

impl AppSrcBuilder {
    fn new() -> Self {
        Self {
            builder: glib::Object::builder(),
            callbacks: None,
            automatic_eos: None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AppSrc`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AppSrc {
        let appsrc = self.builder.build();

        if let Some(callbacks) = self.callbacks {
            appsrc.set_callbacks(callbacks);
        }

        if let Some(automatic_eos) = self.automatic_eos {
            appsrc.set_automatic_eos(automatic_eos);
        }

        appsrc
    }

    pub fn automatic_eos(self, automatic_eos: bool) -> Self {
        Self {
            automatic_eos: Some(automatic_eos),
            ..self
        }
    }

    pub fn block(self, block: bool) -> Self {
        Self {
            builder: self.builder.property("block", block),
            ..self
        }
    }

    pub fn callbacks(self, callbacks: AppSrcCallbacks) -> Self {
        Self {
            callbacks: Some(callbacks),
            ..self
        }
    }

    pub fn caps(self, caps: &gst::Caps) -> Self {
        Self {
            builder: self.builder.property("caps", caps),
            ..self
        }
    }

    pub fn do_timestamp(self, do_timestamp: bool) -> Self {
        Self {
            builder: self.builder.property("do-timestamp", do_timestamp),
            ..self
        }
    }

    pub fn duration(self, duration: u64) -> Self {
        Self {
            builder: self.builder.property("duration", duration),
            ..self
        }
    }

    pub fn format(self, format: gst::Format) -> Self {
        Self {
            builder: self.builder.property("format", format),
            ..self
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    pub fn handle_segment_change(self, handle_segment_change: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("handle-segment-change", handle_segment_change),
            ..self
        }
    }

    pub fn is_live(self, is_live: bool) -> Self {
        Self {
            builder: self.builder.property("is-live", is_live),
            ..self
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub fn leaky_type(self, leaky_type: crate::AppLeakyType) -> Self {
        Self {
            builder: self.builder.property("leaky-type", leaky_type),
            ..self
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub fn max_buffers(self, max_buffers: u64) -> Self {
        Self {
            builder: self.builder.property("max-buffers", max_buffers),
            ..self
        }
    }

    pub fn max_bytes(self, max_bytes: u64) -> Self {
        Self {
            builder: self.builder.property("max-bytes", max_bytes),
            ..self
        }
    }

    pub fn max_latency(self, max_latency: i64) -> Self {
        Self {
            builder: self.builder.property("max-latency", max_latency),
            ..self
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub fn max_time(self, max_time: Option<gst::ClockTime>) -> Self {
        Self {
            builder: self.builder.property("max-time", max_time),
            ..self
        }
    }

    pub fn min_latency(self, min_latency: i64) -> Self {
        Self {
            builder: self.builder.property("min-latency", min_latency),
            ..self
        }
    }

    pub fn min_percent(self, min_percent: u32) -> Self {
        Self {
            builder: self.builder.property("min-percent", min_percent),
            ..self
        }
    }

    pub fn size(self, size: i64) -> Self {
        Self {
            builder: self.builder.property("size", size),
            ..self
        }
    }

    pub fn stream_type(self, stream_type: crate::AppStreamType) -> Self {
        Self {
            builder: self.builder.property("stream-type", stream_type),
            ..self
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
            ..self
        }
    }
}

#[derive(Debug)]
pub struct AppSrcSink {
    app_src: glib::WeakRef<AppSrc>,
    waker_reference: Arc<Mutex<Option<Waker>>>,
}

impl AppSrcSink {
    fn new(app_src: &AppSrc) -> Self {
        skip_assert_initialized!();

        let waker_reference = Arc::new(Mutex::new(None as Option<Waker>));

        app_src.set_callbacks(
            AppSrcCallbacks::builder()
                .need_data({
                    let waker_reference = Arc::clone(&waker_reference);

                    move |_, _| {
                        if let Some(waker) = waker_reference.lock().unwrap().take() {
                            waker.wake();
                        }
                    }
                })
                .build(),
        );

        Self {
            app_src: app_src.downgrade(),
            waker_reference,
        }
    }
}

impl Drop for AppSrcSink {
    fn drop(&mut self) {
        #[cfg(not(feature = "v1_18"))]
        {
            // This is not thread-safe before 1.16.3, see
            // https://gitlab.freedesktop.org/gstreamer/gst-plugins-base/merge_requests/570
            if gst::version() >= (1, 16, 3, 0) {
                if let Some(app_src) = self.app_src.upgrade() {
                    app_src.set_callbacks(AppSrcCallbacks::builder().build());
                }
            }
        }
    }
}

impl Sink<gst::Sample> for AppSrcSink {
    type Error = gst::FlowError;

    fn poll_ready(self: Pin<&mut Self>, context: &mut Context) -> Poll<Result<(), Self::Error>> {
        let mut waker = self.waker_reference.lock().unwrap();

        let app_src = match self.app_src.upgrade() {
            Some(app_src) => app_src,
            None => return Poll::Ready(Err(gst::FlowError::Eos)),
        };

        let current_level_bytes = app_src.current_level_bytes();
        let max_bytes = app_src.max_bytes();

        if current_level_bytes >= max_bytes && max_bytes != 0 {
            waker.replace(context.waker().to_owned());

            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn start_send(self: Pin<&mut Self>, sample: gst::Sample) -> Result<(), Self::Error> {
        let app_src = match self.app_src.upgrade() {
            Some(app_src) => app_src,
            None => return Err(gst::FlowError::Eos),
        };

        app_src.push_sample(&sample)?;

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        let app_src = match self.app_src.upgrade() {
            Some(app_src) => app_src,
            None => return Poll::Ready(Ok(())),
        };

        app_src.end_of_stream()?;

        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use futures_util::{sink::SinkExt, stream::StreamExt};
    use gst::prelude::*;

    use super::*;

    #[test]
    fn test_app_src_sink() {
        gst::init().unwrap();

        let appsrc = gst::ElementFactory::make("appsrc").build().unwrap();
        let fakesink = gst::ElementFactory::make("fakesink")
            .property("signal-handoffs", true)
            .build()
            .unwrap();

        let pipeline = gst::Pipeline::new();
        pipeline.add(&appsrc).unwrap();
        pipeline.add(&fakesink).unwrap();

        appsrc.link(&fakesink).unwrap();

        let mut bus_stream = pipeline.bus().unwrap().stream();
        let mut app_src_sink = appsrc.dynamic_cast::<AppSrc>().unwrap().sink();

        let sample_quantity = 5;

        let samples = (0..sample_quantity)
            .map(|_| gst::Sample::builder().buffer(&gst::Buffer::new()).build())
            .collect::<Vec<gst::Sample>>();

        let mut sample_stream = futures_util::stream::iter(samples).map(Ok);

        let handoff_count_reference = Arc::new(AtomicUsize::new(0));

        fakesink.connect("handoff", false, {
            let handoff_count_reference = Arc::clone(&handoff_count_reference);

            move |_| {
                handoff_count_reference.fetch_add(1, Ordering::AcqRel);

                None
            }
        });

        pipeline.set_state(gst::State::Playing).unwrap();

        futures_executor::block_on(app_src_sink.send_all(&mut sample_stream)).unwrap();
        futures_executor::block_on(app_src_sink.close()).unwrap();

        while let Some(message) = futures_executor::block_on(bus_stream.next()) {
            match message.view() {
                gst::MessageView::Eos(_) => break,
                gst::MessageView::Error(_) => unreachable!(),
                _ => continue,
            }
        }

        pipeline.set_state(gst::State::Null).unwrap();

        assert_eq!(
            handoff_count_reference.load(Ordering::Acquire),
            sample_quantity
        );
    }
}
