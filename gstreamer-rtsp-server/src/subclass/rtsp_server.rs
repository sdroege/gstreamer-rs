// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_rtsp_server_sys;

use glib::subclass::prelude::*;
use glib::translate::*;

use RTSPServer;
use RTSPServerClass;

pub trait RTSPServerImpl: RTSPServerImplExt + ObjectImpl + Send + Sync + 'static {
    fn create_client(&self, server: &RTSPServer) -> Option<::RTSPClient> {
        self.parent_create_client(server)
    }

    fn client_connected(&self, server: &RTSPServer, client: &::RTSPClient) {
        self.parent_client_connected(server, client);
    }
}

pub trait RTSPServerImplExt {
    fn parent_create_client(&self, server: &RTSPServer) -> Option<::RTSPClient>;

    fn parent_client_connected(&self, server: &RTSPServer, client: &::RTSPClient);
}

impl<T: RTSPServerImpl + ObjectImpl> RTSPServerImplExt for T {
    fn parent_create_client(&self, server: &RTSPServer) -> Option<::RTSPClient> {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_rtsp_server_sys::GstRTSPServerClass;
            let f = (*parent_class)
                .create_client
                .expect("No `create_client` virtual method implementation in parent class");
            from_glib_full(f(server.to_glib_none().0))
        }
    }

    fn parent_client_connected(&self, server: &RTSPServer, client: &::RTSPClient) {
        unsafe {
            let data = self.get_type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gst_rtsp_server_sys::GstRTSPServerClass;
            if let Some(f) = (*parent_class).client_connected {
                f(server.to_glib_none().0, client.to_glib_none().0)
            }
        }
    }
}
unsafe impl<T: ObjectSubclass + RTSPServerImpl> IsSubclassable<T> for RTSPServerClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_rtsp_server_sys::GstRTSPServerClass);
            klass.create_client = Some(server_create_client::<T>);
            klass.client_connected = Some(server_client_connected::<T>);
        }
    }
}

unsafe extern "C" fn server_create_client<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPServer,
) -> *mut gst_rtsp_server_sys::GstRTSPClient
where
    T: RTSPServerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPServer> = from_glib_borrow(ptr);

    imp.create_client(&wrap).to_glib_full()
}

unsafe extern "C" fn server_client_connected<T: ObjectSubclass>(
    ptr: *mut gst_rtsp_server_sys::GstRTSPServer,
    client: *mut gst_rtsp_server_sys::GstRTSPClient,
) where
    T: RTSPServerImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<RTSPServer> = from_glib_borrow(ptr);

    imp.client_connected(&wrap, &from_glib_borrow(client));
}
