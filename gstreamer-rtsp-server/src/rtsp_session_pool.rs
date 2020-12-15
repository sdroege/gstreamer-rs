// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RTSPSessionPool;
use glib::ffi::{gboolean, gpointer};
use glib::object::IsA;
use glib::source::{Continue, Priority};
use glib::translate::*;
use std::cell::RefCell;
use std::mem::transmute;

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
    #[allow(clippy::type_complexity)]
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

pub trait RTSPSessionPoolExtManual: 'static {
    fn create_watch<F>(&self, name: Option<&str>, priority: Priority, func: F) -> glib::Source
    where
        F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static;
}

impl<O: IsA<RTSPSessionPool>> RTSPSessionPoolExtManual for O {
    fn create_watch<F>(&self, name: Option<&str>, priority: Priority, func: F) -> glib::Source
    where
        F: FnMut(&RTSPSessionPool) -> Continue + Send + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let source = ffi::gst_rtsp_session_pool_create_watch(self.as_ref().to_glib_none().0);
            glib::ffi::g_source_set_callback(
                source,
                Some(transmute::<
                    _,
                    unsafe extern "C" fn(glib::ffi::gpointer) -> i32,
                >(trampoline_watch::<F> as *const ())),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );
            glib::ffi::g_source_set_priority(source, priority.to_glib());

            if let Some(name) = name {
                glib::ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }
}
