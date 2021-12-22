use crate::auto::Navigation;
use glib::object::IsA;
use glib::translate::ToGlibPtr;

pub trait NavigationExtManual: 'static {
    #[doc(alias = "gst_navigation_send_event")]
    fn send_event(&self, structure: gst::Structure);
}

impl<O: IsA<Navigation>> NavigationExtManual for O {
    fn send_event(&self, structure: gst::Structure) {
        unsafe {
            ffi::gst_navigation_send_event(self.as_ref().to_glib_none().0, structure.into_ptr());
        }
    }
}
