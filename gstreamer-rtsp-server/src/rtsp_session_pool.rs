use ffi;
use glib;
use glib::object::IsA;
use glib::source::{Continue, Priority};
use glib::translate::*;
use glib_ffi;
use glib_ffi::{gboolean, gpointer};
use std::cell::RefCell;
use std::mem::transmute;
use RTSPSessionPool;

unsafe extern "C" fn trampoline_watch<F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static>(
    pool: *mut ffi::GstRTSPSessionPool,
    func: gpointer,
) -> gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (&mut *func.borrow_mut())(&from_glib_borrow(pool)).to_glib()
}

unsafe extern "C" fn destroy_closure_watch<
    F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static,
>(
    ptr: gpointer,
) {
    Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

fn into_raw_watch<F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static>(func: F) -> gpointer {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

pub trait RTSPSessionPoolExtManual: 'static {
    fn create_watch<'a, N: Into<Option<&'a str>>, F>(
        &self,
        name: N,
        priority: Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static;
}

impl<O: IsA<RTSPSessionPool>> RTSPSessionPoolExtManual for O {
    fn create_watch<'a, N: Into<Option<&'a str>>, F>(
        &self,
        name: N,
        priority: Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let source = ffi::gst_rtsp_session_pool_create_watch(self.as_ref().to_glib_none().0);
            glib_ffi::g_source_set_callback(
                source,
                Some(transmute(trampoline_watch::<F> as usize)),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );
            glib_ffi::g_source_set_priority(source, priority.to_glib());

            let name = name.into();
            if let Some(name) = name {
                glib_ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }
}
