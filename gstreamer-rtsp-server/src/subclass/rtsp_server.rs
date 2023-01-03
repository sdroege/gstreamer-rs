// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::RTSPServer;

pub trait RTSPServerImpl: RTSPServerImplExt + ObjectImpl + Send + Sync {
    fn create_client(&self) -> Option<crate::RTSPClient> {
        self.parent_create_client()
    }

    fn client_connected(&self, client: &crate::RTSPClient) {
        self.parent_client_connected(client);
    }
}

pub trait RTSPServerImplExt: ObjectSubclass {
    fn parent_create_client(&self) -> Option<crate::RTSPClient>;

    fn parent_client_connected(&self, client: &crate::RTSPClient);
}

impl<T: RTSPServerImpl> RTSPServerImplExt for T {
    fn parent_create_client(&self) -> Option<crate::RTSPClient> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPServerClass;
            let f = (*parent_class)
                .create_client
                .expect("No `create_client` virtual method implementation in parent class");
            from_glib_full(f(self
                .obj()
                .unsafe_cast_ref::<RTSPServer>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_client_connected(&self, client: &crate::RTSPClient) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTSPServerClass;
            if let Some(f) = (*parent_class).client_connected {
                f(
                    self.obj().unsafe_cast_ref::<RTSPServer>().to_glib_none().0,
                    client.to_glib_none().0,
                )
            }
        }
    }
}
unsafe impl<T: RTSPServerImpl> IsSubclassable<T> for RTSPServer {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.create_client = Some(server_create_client::<T>);
        klass.client_connected = Some(server_client_connected::<T>);
    }
}

unsafe extern "C" fn server_create_client<T: RTSPServerImpl>(
    ptr: *mut ffi::GstRTSPServer,
) -> *mut ffi::GstRTSPClient {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.create_client().to_glib_full()
}

unsafe extern "C" fn server_client_connected<T: RTSPServerImpl>(
    ptr: *mut ffi::GstRTSPServer,
    client: *mut ffi::GstRTSPClient,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.client_connected(&from_glib_borrow(client));
}
