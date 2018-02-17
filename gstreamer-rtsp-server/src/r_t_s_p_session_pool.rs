use std::cell::RefCell;
use std::mem::transmute;
use RTSPSessionPool;
use ffi;
use glib;
use glib_ffi;
use glib::object::IsA;
use glib::translate::*;
use glib::source::{Continue, Priority};
use glib_ffi::{gboolean, gpointer};

unsafe extern "C" fn trampoline_watch(
    pool: *mut ffi::GstRTSPSessionPool,
    func: gpointer,
) -> gboolean {
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &RefCell<Box<FnMut(&RTSPSessionPool) -> Continue + Send + 'static>> = transmute(func);
    (&mut *func.borrow_mut())(&from_glib_borrow(pool)).to_glib()
}

unsafe extern "C" fn destroy_closure_watch(ptr: gpointer) {
    Box::<RefCell<Box<FnMut(&RTSPSessionPool) -> Continue + Send + 'static>>>::from_raw(
        ptr as *mut _,
    );
}

fn into_raw_watch<F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static>(func: F) -> gpointer {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    let func: Box<RefCell<Box<FnMut(&RTSPSessionPool) -> Continue + Send + 'static>>> =
        Box::new(RefCell::new(Box::new(func)));
    Box::into_raw(func) as gpointer
}

pub trait RTSPSessionPoolExtManual {
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
        F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static
    {
        skip_assert_initialized!();
        unsafe {
            let source = ffi::gst_rtsp_session_pool_create_watch(self.to_glib_none().0);
            let trampoline = trampoline_watch as gpointer;
            glib_ffi::g_source_set_callback(
                source,
                Some(transmute(trampoline)),
                into_raw_watch(func),
                Some(destroy_closure_watch),
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
