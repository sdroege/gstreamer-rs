// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use gst_base::prelude::*;
use gst_base::subclass::prelude::*;

use crate::AudioFilter;
use crate::AudioInfo;

pub trait AudioFilterImpl: AudioFilterImplExt + BaseTransformImpl {
    fn allowed_caps() -> &'static gst::Caps;

    fn setup(&self, info: &AudioInfo) -> Result<(), gst::LoggableError> {
        self.parent_setup(info)
    }
}

pub trait AudioFilterImplExt: ObjectSubclass {
    fn parent_setup(&self, info: &AudioInfo) -> Result<(), gst::LoggableError>;
}

impl<T: AudioFilterImpl> AudioFilterImplExt for T {
    fn parent_setup(&self, info: &AudioInfo) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioFilterClass;
            (*parent_class)
                .setup
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj().unsafe_cast_ref::<AudioFilter>().to_glib_none().0,
                            info.to_glib_none().0,
                        ),
                        gst::CAT_RUST,
                        "Parent function `setup` failed"
                    )
                })
                .unwrap_or(Ok(()))
        }
    }
}

unsafe impl<T: AudioFilterImpl> IsSubclassable<T> for AudioFilter {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);

        let klass = klass.as_mut();
        klass.setup = Some(audio_filter_setup::<T>);

        unsafe {
            ffi::gst_audio_filter_class_add_pad_templates(
                &mut *klass,
                T::allowed_caps().to_glib_none().0,
            );
        }
    }
}

unsafe extern "C" fn audio_filter_setup<T: AudioFilterImpl>(
    ptr: *mut ffi::GstAudioFilter,
    info: *const ffi::GstAudioInfo,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.setup(&from_glib_none(info)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}
