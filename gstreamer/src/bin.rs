use Bin;
use Element;

use glib;
use glib::IsA;
use glib::translate::{ToGlibPtr, from_glib};

use ffi;

pub trait BinExtManual {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;
    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;
}

impl<O: IsA<Bin>> BinExtManual for O {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                let ret: bool = from_glib(ffi::gst_bin_add(self.to_glib_none().0, e.to_glib_none().0));
                if !ret {
                    return Err(glib::BoolError("Failed to add elements"));
                }
            }
        }

        return Ok(());
    }

    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                let ret: bool = from_glib(ffi::gst_bin_remove(self.to_glib_none().0, e.to_glib_none().0));
                if !ret {
                    return Err(glib::BoolError("Failed to add elements"));
                }
            }
        }

        return Ok(());
    }
}
