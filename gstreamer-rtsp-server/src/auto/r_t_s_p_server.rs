// This file was generated by gir (https://github.com/gtk-rs/gir @ 47eb915)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPAuth;
use RTSPClient;
use RTSPMountPoints;
use RTSPSessionPool;
use RTSPThreadPool;
use ffi;
use gio;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RTSPServer(Object<ffi::GstRTSPServer, ffi::GstRTSPServerClass>);

    match fn {
        get_type => || ffi::gst_rtsp_server_get_type(),
    }
}

impl RTSPServer {
    pub fn new() -> RTSPServer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_new())
        }
    }

    pub fn io_func(socket: &gio::Socket, condition: glib::IOCondition, server: &RTSPServer) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_rtsp_server_io_func(socket.to_glib_none().0, condition.to_glib(), server.to_glib_none().0))
        }
    }
}

impl Default for RTSPServer {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPServer {}
unsafe impl Sync for RTSPServer {}

pub trait RTSPServerExt {
    //fn client_filter<'a, P: Into<Option<&'a /*Unimplemented*/RTSPServerClientFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q) -> Vec<RTSPClient>;

    //fn create_socket<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P, error: /*Ignored*/Option<Error>) -> Option<gio::Socket>;

    //fn create_source<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P, error: /*Ignored*/Option<Error>) -> Option<glib::Source>;

    fn get_address(&self) -> Option<String>;

    fn get_auth(&self) -> Option<RTSPAuth>;

    fn get_backlog(&self) -> i32;

    fn get_bound_port(&self) -> i32;

    fn get_mount_points(&self) -> Option<RTSPMountPoints>;

    fn get_service(&self) -> Option<String>;

    fn get_session_pool(&self) -> Option<RTSPSessionPool>;

    fn get_thread_pool(&self) -> Option<RTSPThreadPool>;

    fn set_address(&self, address: &str);

    fn set_auth<'a, P: Into<Option<&'a RTSPAuth>>>(&self, auth: P);

    fn set_backlog(&self, backlog: i32);

    fn set_mount_points<'a, P: Into<Option<&'a RTSPMountPoints>>>(&self, mounts: P);

    fn set_service(&self, service: &str);

    fn set_session_pool<'a, P: Into<Option<&'a RTSPSessionPool>>>(&self, pool: P);

    fn set_thread_pool<'a, P: Into<Option<&'a RTSPThreadPool>>>(&self, pool: P);

    fn transfer_connection<'a, P: Into<Option<&'a str>>>(&self, socket: &gio::Socket, ip: &str, port: i32, initial_buffer: P) -> bool;

    fn connect_client_connected<F: Fn(&Self, &RTSPClient) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_address_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_backlog_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bound_port_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mount_points_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_service_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_session_pool_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPServer> + IsA<glib::object::Object>> RTSPServerExt for O {
    //fn client_filter<'a, P: Into<Option<&'a /*Unimplemented*/RTSPServerClientFilterFunc>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: P, user_data: Q) -> Vec<RTSPClient> {
    //    unsafe { TODO: call ffi::gst_rtsp_server_client_filter() }
    //}

    //fn create_socket<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P, error: /*Ignored*/Option<Error>) -> Option<gio::Socket> {
    //    unsafe { TODO: call ffi::gst_rtsp_server_create_socket() }
    //}

    //fn create_source<'a, P: Into<Option<&'a gio::Cancellable>>>(&self, cancellable: P, error: /*Ignored*/Option<Error>) -> Option<glib::Source> {
    //    unsafe { TODO: call ffi::gst_rtsp_server_create_source() }
    //}

    fn get_address(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_address(self.to_glib_none().0))
        }
    }

    fn get_auth(&self) -> Option<RTSPAuth> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_auth(self.to_glib_none().0))
        }
    }

    fn get_backlog(&self) -> i32 {
        unsafe {
            ffi::gst_rtsp_server_get_backlog(self.to_glib_none().0)
        }
    }

    fn get_bound_port(&self) -> i32 {
        unsafe {
            ffi::gst_rtsp_server_get_bound_port(self.to_glib_none().0)
        }
    }

    fn get_mount_points(&self) -> Option<RTSPMountPoints> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_mount_points(self.to_glib_none().0))
        }
    }

    fn get_service(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_service(self.to_glib_none().0))
        }
    }

    fn get_session_pool(&self) -> Option<RTSPSessionPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_session_pool(self.to_glib_none().0))
        }
    }

    fn get_thread_pool(&self) -> Option<RTSPThreadPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_thread_pool(self.to_glib_none().0))
        }
    }

    fn set_address(&self, address: &str) {
        unsafe {
            ffi::gst_rtsp_server_set_address(self.to_glib_none().0, address.to_glib_none().0);
        }
    }

    fn set_auth<'a, P: Into<Option<&'a RTSPAuth>>>(&self, auth: P) {
        let auth = auth.into();
        let auth = auth.to_glib_none();
        unsafe {
            ffi::gst_rtsp_server_set_auth(self.to_glib_none().0, auth.0);
        }
    }

    fn set_backlog(&self, backlog: i32) {
        unsafe {
            ffi::gst_rtsp_server_set_backlog(self.to_glib_none().0, backlog);
        }
    }

    fn set_mount_points<'a, P: Into<Option<&'a RTSPMountPoints>>>(&self, mounts: P) {
        let mounts = mounts.into();
        let mounts = mounts.to_glib_none();
        unsafe {
            ffi::gst_rtsp_server_set_mount_points(self.to_glib_none().0, mounts.0);
        }
    }

    fn set_service(&self, service: &str) {
        unsafe {
            ffi::gst_rtsp_server_set_service(self.to_glib_none().0, service.to_glib_none().0);
        }
    }

    fn set_session_pool<'a, P: Into<Option<&'a RTSPSessionPool>>>(&self, pool: P) {
        let pool = pool.into();
        let pool = pool.to_glib_none();
        unsafe {
            ffi::gst_rtsp_server_set_session_pool(self.to_glib_none().0, pool.0);
        }
    }

    fn set_thread_pool<'a, P: Into<Option<&'a RTSPThreadPool>>>(&self, pool: P) {
        let pool = pool.into();
        let pool = pool.to_glib_none();
        unsafe {
            ffi::gst_rtsp_server_set_thread_pool(self.to_glib_none().0, pool.0);
        }
    }

    fn transfer_connection<'a, P: Into<Option<&'a str>>>(&self, socket: &gio::Socket, ip: &str, port: i32, initial_buffer: P) -> bool {
        let initial_buffer = initial_buffer.into();
        let initial_buffer = initial_buffer.to_glib_none();
        unsafe {
            from_glib(ffi::gst_rtsp_server_transfer_connection(self.to_glib_none().0, socket.to_glib_full(), ip.to_glib_none().0, port, initial_buffer.0))
        }
    }

    fn connect_client_connected<F: Fn(&Self, &RTSPClient) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &RTSPClient) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "client-connected",
                transmute(client_connected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_address_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::address",
                transmute(notify_address_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_backlog_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::backlog",
                transmute(notify_backlog_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_bound_port_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bound-port",
                transmute(notify_bound_port_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_mount_points_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mount-points",
                transmute(notify_mount_points_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_service_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::service",
                transmute(notify_service_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_session_pool_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::session-pool",
                transmute(notify_session_pool_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn client_connected_trampoline<P>(this: *mut ffi::GstRTSPServer, object: *mut ffi::GstRTSPClient, f: glib_ffi::gpointer)
where P: IsA<RTSPServer> {
    callback_guard!();
    let f: &&(Fn(&P, &RTSPClient) + Send + Sync + 'static) = transmute(f);
    f(&RTSPServer::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(object))
}

unsafe extern "C" fn notify_address_trampoline<P>(this: *mut ffi::GstRTSPServer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPServer> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPServer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_backlog_trampoline<P>(this: *mut ffi::GstRTSPServer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPServer> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPServer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_bound_port_trampoline<P>(this: *mut ffi::GstRTSPServer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPServer> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPServer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mount_points_trampoline<P>(this: *mut ffi::GstRTSPServer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPServer> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPServer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_service_trampoline<P>(this: *mut ffi::GstRTSPServer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPServer> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPServer::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_session_pool_trampoline<P>(this: *mut ffi::GstRTSPServer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPServer> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPServer::from_glib_borrow(this).downcast_unchecked())
}
