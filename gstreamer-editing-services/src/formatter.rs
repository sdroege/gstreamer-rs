// Take a look at the license at the top of the repository in the LICENSE file.
use crate::{prelude::*, Formatter};
use gst::glib::translate::*;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Formatter>> Sealed for T {}
}

pub trait FormatterExtManual: sealed::Sealed + IsA<Formatter> + 'static {
    fn can_load_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let klass = self.class_of::<crate::Formatter>().unwrap();

            let f = klass.as_ref().can_load_uri.ok_or_else(|| {
                glib::Error::new(gst::CoreError::Failed, "No `can_load_uri` method defined")
            })?;

            let mut err = std::ptr::null_mut();
            let res = f(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut err,
            );

            if res == glib::ffi::GTRUE {
                Ok(())
            } else {
                Err(from_glib_full(err))
            }
        }
    }

    #[doc(alias = "ges_formatter_class_register_metas")]
    fn register(
        type_: glib::types::Type,
        name: &str,
        description: Option<&str>,
        extensions: Option<&str>,
        caps: Option<&str>,
        version: f64,
        rank: gst::Rank,
    ) {
        skip_assert_initialized!();

        unsafe {
            let klass = mut_override(
                gst::glib::Class::<crate::Formatter>::from_type(type_)
                    .unwrap()
                    .as_ref(),
            );

            ffi::ges_formatter_class_register_metas(
                klass,
                name.to_glib_none().0,
                description.to_glib_none().0,
                extensions.to_glib_none().0,
                caps.to_glib_none().0,
                version,
                rank.into_glib(),
            )
        }
    }
}

impl<O: IsA<Formatter>> FormatterExtManual for O {}
