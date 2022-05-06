use crate::auto::Navigation;
use glib::object::IsA;
use glib::translate::{IntoGlibPtr, ToGlibPtr};

pub trait NavigationExtManual: 'static {
    #[doc(alias = "gst_navigation_send_event")]
    fn send_event(&self, structure: gst::Structure);

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_navigation_send_event_simple")]
    fn send_event_simple(&self, event: gst::Event);
}

impl<O: IsA<Navigation>> NavigationExtManual for O {
    fn send_event(&self, structure: gst::Structure) {
        unsafe {
            ffi::gst_navigation_send_event(
                self.as_ref().to_glib_none().0,
                structure.into_glib_ptr(),
            );
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    fn send_event_simple(&self, event: gst::Event) {
        unsafe {
            ffi::gst_navigation_send_event_simple(
                self.as_ref().to_glib_none().0,
                event.into_glib_ptr(),
            );
        }
    }
}
