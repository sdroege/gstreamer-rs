// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::Formatter;
use glib::{subclass::prelude::*, translate::*};

pub trait FormatterImpl: FormatterImplExt + ObjectImpl + Send + Sync {
    fn can_load_uri(&self, uri: &str) -> Result<(), glib::Error> {
        self.parent_can_load_uri(uri)
    }

    fn load_from_uri(&self, timeline: &crate::Timeline, uri: &str) -> Result<(), glib::Error> {
        self.parent_load_from_uri(timeline, uri)
    }

    fn save_to_uri(
        &self,
        timeline: &crate::Timeline,
        uri: &str,
        overwrite: bool,
    ) -> Result<(), glib::Error> {
        self.parent_save_to_uri(timeline, uri, overwrite)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::FormatterImplExt> Sealed for T {}
}

pub trait FormatterImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_can_load_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GESFormatterClass;

            let f = (*parent_class)
                .can_load_uri
                .expect("Missing parent function `can_load_uri`");

            let mut error = std::ptr::null_mut();
            let res = f(
                self.obj()
                    .unsafe_cast_ref::<crate::Formatter>()
                    .to_glib_none()
                    .0,
                uri.to_glib_none().0,
                &mut error,
            );

            if res == glib::ffi::GFALSE {
                if error.is_null() {
                    Err(glib::Error::new(
                        gst::CoreError::Failed,
                        "Can load uri failed",
                    ))
                } else {
                    Err(from_glib_full(error))
                }
            } else {
                Ok(())
            }
        }
    }

    fn parent_load_from_uri(
        &self,
        timeline: &crate::Timeline,
        uri: &str,
    ) -> Result<(), glib::Error> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GESFormatterClass;

            let f = (*parent_class)
                .load_from_uri
                .expect("Missing parent function `load_from_uri`");

            let mut error = std::ptr::null_mut();
            let res = f(
                self.obj()
                    .unsafe_cast_ref::<crate::Formatter>()
                    .to_glib_none()
                    .0,
                timeline
                    .unsafe_cast_ref::<crate::Timeline>()
                    .to_glib_none()
                    .0,
                uri.to_glib_none().0,
                &mut error,
            );

            if res == glib::ffi::GFALSE {
                if error.is_null() {
                    Err(glib::Error::new(
                        gst::CoreError::Failed,
                        "Load from uri failed",
                    ))
                } else {
                    Err(from_glib_full(error))
                }
            } else {
                Ok(())
            }
        }
    }
    fn parent_save_to_uri(
        &self,
        timeline: &crate::Timeline,
        uri: &str,
        overwrite: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GESFormatterClass;

            let f = (*parent_class)
                .save_to_uri
                .expect("Missing parent function `save_to_uri`");

            let mut error = std::ptr::null_mut();
            let res = f(
                self.obj()
                    .unsafe_cast_ref::<crate::Formatter>()
                    .to_glib_none()
                    .0,
                timeline
                    .unsafe_cast_ref::<crate::Timeline>()
                    .to_glib_none()
                    .0,
                uri.to_glib_none().0,
                overwrite.into_glib(),
                &mut error,
            );

            if res == glib::ffi::GFALSE {
                if error.is_null() {
                    Err(glib::Error::new(
                        gst::CoreError::Failed,
                        "Save to uri failed",
                    ))
                } else {
                    Err(from_glib_full(error))
                }
            } else {
                Ok(())
            }
        }
    }
}

impl<T: FormatterImpl> FormatterImplExt for T {}

unsafe impl<T: FormatterImpl> IsSubclassable<T> for Formatter {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.can_load_uri = Some(formatter_can_load_uri::<T>);
        klass.load_from_uri = Some(formatter_load_from_uri::<T>);
        klass.save_to_uri = Some(formatter_save_to_uri::<T>);
    }
}

unsafe extern "C" fn formatter_can_load_uri<T: FormatterImpl>(
    ptr: *mut ffi::GESFormatter,
    uri: *const libc::c_char,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.can_load_uri(glib::GString::from_glib_borrow(uri).as_str()) {
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }

            glib::ffi::GFALSE
        }
        Ok(_) => glib::ffi::GTRUE,
    }
}

unsafe extern "C" fn formatter_load_from_uri<T: FormatterImpl>(
    ptr: *mut ffi::GESFormatter,
    timeline: *mut ffi::GESTimeline,
    uri: *const libc::c_char,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let timeline = from_glib_borrow(timeline);

    match imp.load_from_uri(&timeline, glib::GString::from_glib_borrow(uri).as_str()) {
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }

            glib::ffi::GFALSE
        }
        Ok(_) => glib::ffi::GTRUE,
    }
}

unsafe extern "C" fn formatter_save_to_uri<T: FormatterImpl>(
    ptr: *mut ffi::GESFormatter,
    timeline: *mut ffi::GESTimeline,
    uri: *const libc::c_char,
    overwrite: glib::ffi::gboolean,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let timeline = from_glib_borrow(timeline);

    match imp.save_to_uri(
        &timeline,
        glib::GString::from_glib_borrow(uri).as_str(),
        from_glib(overwrite),
    ) {
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }

            glib::ffi::GFALSE
        }
        Ok(_) => glib::ffi::GTRUE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Formatter;

    pub mod imp {
        use super::*;

        #[derive(Default)]
        pub struct SimpleFormatter;

        #[glib::object_subclass]
        impl ObjectSubclass for SimpleFormatter {
            const NAME: &'static str = "SimpleFormatter";
            type Type = super::SimpleFormatter;
            type ParentType = Formatter;
        }
        impl ObjectImpl for SimpleFormatter {}
        impl FormatterImpl for SimpleFormatter {
            fn can_load_uri(&self, uri: &str) -> Result<(), glib::Error> {
                if uri.starts_with("ges:test") {
                    Ok(())
                } else {
                    self.parent_can_load_uri(uri)
                }
            }

            fn load_from_uri(
                &self,
                timeline: &crate::Timeline,
                _uri: &str,
            ) -> Result<(), glib::Error> {
                timeline.append_layer();

                Ok(())
            }

            fn save_to_uri(
                &self,
                timeline: &crate::Timeline,
                uri: &str,
                _overwrite: bool,
            ) -> Result<(), glib::Error> {
                unsafe { timeline.set_data("saved", uri.to_string()) };

                Ok(())
            }
        }
    }

    glib::wrapper! {
        pub struct SimpleFormatter(ObjectSubclass<imp::SimpleFormatter>) @extends Formatter, gst::Object;
    }

    impl SimpleFormatter {
        pub fn new() -> Self {
            glib::Object::builder().build()
        }
    }

    impl Default for SimpleFormatter {
        fn default() -> Self {
            Self::new()
        }
    }

    #[test]
    fn test_formatter_subclass() {
        crate::init().unwrap();

        let formatter = SimpleFormatter::new();
        formatter
            .can_load_uri("ges:test:")
            .expect("We can load anything...");

        assert!(formatter.can_load_uri("nottest").is_err());

        let timeline = crate::Timeline::new();
        assert_eq!(timeline.layers().len(), 0);
        #[allow(deprecated)]
        formatter
            .load_from_uri(&timeline, "test")
            .expect("We can load anything...");
        assert_eq!(timeline.layers().len(), 1);

        unsafe {
            assert_eq!(timeline.data::<Option<String>>("saved"), None);
        }
        #[allow(deprecated)]
        formatter
            .save_to_uri(&timeline, "test", false)
            .expect("We can save anything...");
        unsafe {
            assert_eq!(
                timeline.data::<String>("saved").unwrap().as_ref(),
                &"test".to_string()
            );
        }

        Formatter::register(
            SimpleFormatter::static_type(),
            "SimpleFormatter",
            None,
            None,
            None,
            1.0,
            gst::Rank::Primary,
        );

        let proj = crate::Project::new(Some("ges:test:"));
        let timeline = proj
            .extract()
            .unwrap()
            .downcast::<crate::Timeline>()
            .unwrap();
        assert_eq!(timeline.layers().len(), 1);

        let proj = crate::Project::new(Some("ges:notest:"));
        assert!(proj.extract().is_err());
    }
}
