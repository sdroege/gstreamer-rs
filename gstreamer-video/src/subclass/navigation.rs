// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;

use glib::subclass::prelude::*;

use crate::Navigation;

pub trait NavigationImpl: ObjectImpl {
    fn send_event(&self, nav: &Self::Type, structure: gst::Structure);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn send_event_simple(&self, nav: &Self::Type, event: gst::Event) {
        if let Some(structure) = event.structure() {
            self.send_event(nav, structure.to_owned());
        }
    }
}

pub trait NavigationImplExt: ObjectSubclass {
    fn parent_send_event(&self, nav: &Self::Type, structure: gst::Structure);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn parent_send_event_simple(&self, nav: &Self::Type, event: gst::Event) {
        if let Some(structure) = event.structure() {
            self.parent_send_event(nav, structure.to_owned());
        }
    }
}

impl<T: NavigationImpl> NavigationImplExt for T {
    fn parent_send_event(&self, nav: &Self::Type, structure: gst::Structure) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Navigation>()
                as *const ffi::GstNavigationInterface;

            let func = match (*parent_iface).send_event {
                Some(func) => func,
                None => return,
            };

            func(
                nav.unsafe_cast_ref::<Navigation>().to_glib_none().0,
                structure.into_ptr(),
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn parent_send_event_simple(&self, nav: &Self::Type, event: gst::Event) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Navigation>()
                as *const ffi::GstNavigationInterface;

            let func = match (*parent_iface).send_event_simple {
                Some(func) => func,
                None => return,
            };

            func(
                nav.unsafe_cast_ref::<Navigation>().to_glib_none().0,
                event.into_ptr(),
            );
        }
    }
}

unsafe impl<T: NavigationImpl> IsImplementable<T> for Navigation {
    #[cfg(not(feature = "v1_22"))]
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.send_event = Some(navigation_send_event::<T>);
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.send_event = Some(navigation_send_event::<T>);
        iface.send_event_simple = Some(navigation_send_event_simple::<T>);
    }
}

unsafe extern "C" fn navigation_send_event<T: NavigationImpl>(
    nav: *mut ffi::GstNavigation,
    structure: *mut gst::ffi::GstStructure,
) {
    let instance = &*(nav as *mut T::Instance);
    let imp = instance.imp();

    imp.send_event(
        from_glib_borrow::<_, Navigation>(nav).unsafe_cast_ref(),
        from_glib_full(structure),
    );
}

#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
unsafe extern "C" fn navigation_send_event_simple<T: NavigationImpl>(
    nav: *mut ffi::GstNavigation,
    event: *mut gst::ffi::GstEvent,
) {
    let instance = &*(nav as *mut T::Instance);
    let imp = instance.imp();

    imp.send_event_simple(
        from_glib_borrow::<_, Navigation>(nav).unsafe_cast_ref(),
        from_glib_full(event),
    );
}
