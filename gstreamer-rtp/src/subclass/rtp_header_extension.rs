// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

pub trait RTPHeaderExtensionImpl: RTPHeaderExtensionImplExt + ElementImpl {
    const URI: &'static str;

    fn supported_flags(&self, element: &Self::Type) -> crate::RTPHeaderExtensionFlags {
        self.parent_supported_flags(element)
    }

    fn max_size(&self, element: &Self::Type, input: &gst::BufferRef) -> usize {
        self.parent_max_size(element, input)
    }

    fn write(
        &self,
        element: &Self::Type,
        input: &gst::BufferRef,
        write_flags: crate::RTPHeaderExtensionFlags,
        output: &mut gst::BufferRef,
        output_data: &mut [u8],
    ) -> Result<usize, gst::LoggableError> {
        self.parent_write(element, input, write_flags, output, output_data)
    }

    fn read(
        &self,
        element: &Self::Type,
        read_flags: crate::RTPHeaderExtensionFlags,
        input_data: &[u8],
        output: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_read(element, read_flags, input_data, output)
    }

    fn set_non_rtp_sink_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_non_rtp_sink_caps(element, caps)
    }

    fn update_non_rtp_src_caps(
        &self,
        element: &Self::Type,
        caps: &mut gst::CapsRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_update_non_rtp_src_caps(element, caps)
    }

    fn set_attributes(
        &self,
        element: &Self::Type,
        direction: crate::RTPHeaderExtensionDirection,
        attributes: &str,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_attributes(element, direction, attributes)
    }

    fn set_caps_from_attributes(
        &self,
        element: &Self::Type,
        caps: &mut gst::CapsRef,
    ) -> Result<(), gst::LoggableError> {
        self.parent_set_caps_from_attributes(element, caps)
    }
}

pub trait RTPHeaderExtensionImplExt: ObjectSubclass {
    fn parent_supported_flags(&self, element: &Self::Type) -> crate::RTPHeaderExtensionFlags;
    fn parent_max_size(&self, element: &Self::Type, input: &gst::BufferRef) -> usize;
    fn parent_write(
        &self,
        element: &Self::Type,
        input: &gst::BufferRef,
        write_flags: crate::RTPHeaderExtensionFlags,
        output: &mut gst::BufferRef,
        output_data: &mut [u8],
    ) -> Result<usize, gst::LoggableError>;
    fn parent_read(
        &self,
        element: &Self::Type,
        read_flags: crate::RTPHeaderExtensionFlags,
        input_data: &[u8],
        output: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError>;
    fn parent_set_non_rtp_sink_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError>;
    fn parent_update_non_rtp_src_caps(
        &self,
        element: &Self::Type,
        caps: &mut gst::CapsRef,
    ) -> Result<(), gst::LoggableError>;
    fn parent_set_attributes(
        &self,
        element: &Self::Type,
        direction: crate::RTPHeaderExtensionDirection,
        attributes: &str,
    ) -> Result<(), gst::LoggableError>;
    fn parent_set_caps_from_attributes(
        &self,
        element: &Self::Type,
        caps: &mut gst::CapsRef,
    ) -> Result<(), gst::LoggableError>;
}

impl<T: RTPHeaderExtensionImpl> RTPHeaderExtensionImplExt for T {
    fn parent_supported_flags(&self, element: &Self::Type) -> crate::RTPHeaderExtensionFlags {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPHeaderExtensionClass;
            let f = (*parent_class)
                .get_supported_flags
                .expect("no parent \"get_supported_flags\" implementation");
            from_glib(f(element
                .unsafe_cast_ref::<crate::RTPHeaderExtension>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_max_size(&self, element: &Self::Type, input: &gst::BufferRef) -> usize {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPHeaderExtensionClass;
            let f = (*parent_class)
                .get_max_size
                .expect("no parent \"get_max_size\" implementation");
            f(
                element
                    .unsafe_cast_ref::<crate::RTPHeaderExtension>()
                    .to_glib_none()
                    .0,
                input.as_ptr(),
            )
        }
    }

    fn parent_write(
        &self,
        element: &Self::Type,
        input: &gst::BufferRef,
        write_flags: crate::RTPHeaderExtensionFlags,
        output: &mut gst::BufferRef,
        output_data: &mut [u8],
    ) -> Result<usize, gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPHeaderExtensionClass;
            let f = (*parent_class)
                .write
                .expect("no parent \"write\" implementation");

            let res = f(
                element
                    .unsafe_cast_ref::<crate::RTPHeaderExtension>()
                    .to_glib_none()
                    .0,
                input.as_ptr(),
                write_flags.into_glib(),
                output.as_mut_ptr(),
                output_data.as_mut_ptr(),
                output_data.len(),
            );

            if res < 0 {
                Err(gst::loggable_error!(
                    gst::CAT_RUST,
                    "Failed to write extension data"
                ))
            } else {
                Ok(res as usize)
            }
        }
    }

    fn parent_read(
        &self,
        element: &Self::Type,
        read_flags: crate::RTPHeaderExtensionFlags,
        input_data: &[u8],
        output: &mut gst::BufferRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPHeaderExtensionClass;
            let f = (*parent_class)
                .read
                .expect("no parent \"read\" implementation");

            gst::result_from_gboolean!(
                f(
                    element
                        .unsafe_cast_ref::<crate::RTPHeaderExtension>()
                        .to_glib_none()
                        .0,
                    read_flags.into_glib(),
                    input_data.as_ptr(),
                    input_data.len(),
                    output.as_mut_ptr(),
                ),
                gst::CAT_RUST,
                "Failed to read extension data",
            )
        }
    }

    fn parent_set_non_rtp_sink_caps(
        &self,
        element: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPHeaderExtensionClass;
            if let Some(f) = (*parent_class).set_non_rtp_sink_caps {
                gst::result_from_gboolean!(
                    f(
                        element
                            .unsafe_cast_ref::<crate::RTPHeaderExtension>()
                            .to_glib_none()
                            .0,
                        caps.as_mut_ptr(),
                    ),
                    gst::CAT_RUST,
                    "Failed to set non-RTP sink caps",
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_update_non_rtp_src_caps(
        &self,
        element: &Self::Type,
        caps: &mut gst::CapsRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPHeaderExtensionClass;
            if let Some(f) = (*parent_class).update_non_rtp_src_caps {
                gst::result_from_gboolean!(
                    f(
                        element
                            .unsafe_cast_ref::<crate::RTPHeaderExtension>()
                            .to_glib_none()
                            .0,
                        caps.as_mut_ptr(),
                    ),
                    gst::CAT_RUST,
                    "Failed to update non-RTP source caps",
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_set_attributes(
        &self,
        element: &Self::Type,
        direction: crate::RTPHeaderExtensionDirection,
        attributes: &str,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPHeaderExtensionClass;
            if let Some(f) = (*parent_class).set_attributes {
                gst::result_from_gboolean!(
                    f(
                        element
                            .unsafe_cast_ref::<crate::RTPHeaderExtension>()
                            .to_glib_none()
                            .0,
                        direction.into_glib(),
                        attributes.to_glib_none().0,
                    ),
                    gst::CAT_RUST,
                    "Failed to set attributes",
                )
            } else {
                Ok(())
            }
        }
    }

    fn parent_set_caps_from_attributes(
        &self,
        element: &Self::Type,
        caps: &mut gst::CapsRef,
    ) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstRTPHeaderExtensionClass;
            let f = (*parent_class)
                .set_caps_from_attributes
                .expect("no parent \"set_caps_from_attributes\" implementation");

            gst::result_from_gboolean!(
                f(
                    element
                        .unsafe_cast_ref::<crate::RTPHeaderExtension>()
                        .to_glib_none()
                        .0,
                    caps.as_mut_ptr(),
                ),
                gst::CAT_RUST,
                "Failed to set caps from attributes",
            )
        }
    }
}

unsafe impl<T: RTPHeaderExtensionImpl> IsSubclassable<T> for crate::RTPHeaderExtension {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.get_supported_flags = Some(get_supported_flags::<T>);
        klass.get_max_size = Some(get_max_size::<T>);
        klass.write = Some(write::<T>);
        klass.read = Some(read::<T>);
        klass.set_non_rtp_sink_caps = Some(set_non_rtp_sink_caps::<T>);
        klass.update_non_rtp_src_caps = Some(update_non_rtp_src_caps::<T>);
        klass.set_attributes = Some(set_attributes::<T>);
        klass.set_caps_from_attributes = Some(set_caps_from_attributes::<T>);

        unsafe {
            ffi::gst_rtp_header_extension_class_set_uri(&mut *klass, T::URI.to_glib_none().0);
        }
    }
}

unsafe extern "C" fn get_supported_flags<T: RTPHeaderExtensionImpl>(
    ptr: *mut ffi::GstRTPHeaderExtension,
) -> ffi::GstRTPHeaderExtensionFlags {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<crate::RTPHeaderExtension> = from_glib_borrow(ptr);

    gst::panic_to_error!(
        &wrap,
        imp.panicked(),
        crate::RTPHeaderExtensionFlags::empty(),
        { imp.supported_flags(wrap.unsafe_cast_ref()) }
    )
    .into_glib()
}

unsafe extern "C" fn get_max_size<T: RTPHeaderExtensionImpl>(
    ptr: *mut ffi::GstRTPHeaderExtension,
    input: *const gst::ffi::GstBuffer,
) -> usize {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<crate::RTPHeaderExtension> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), 0, {
        imp.max_size(wrap.unsafe_cast_ref(), gst::BufferRef::from_ptr(input))
    })
}

unsafe extern "C" fn write<T: RTPHeaderExtensionImpl>(
    ptr: *mut ffi::GstRTPHeaderExtension,
    input: *const gst::ffi::GstBuffer,
    write_flags: ffi::GstRTPHeaderExtensionFlags,
    output: *mut gst::ffi::GstBuffer,
    output_data: *mut u8,
    output_data_len: usize,
) -> isize {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<crate::RTPHeaderExtension> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), -1, {
        match imp.write(
            wrap.unsafe_cast_ref(),
            gst::BufferRef::from_ptr(input),
            from_glib(write_flags),
            gst::BufferRef::from_mut_ptr(output),
            if output_data_len == 0 {
                &mut []
            } else {
                std::slice::from_raw_parts_mut(output_data, output_data_len)
            },
        ) {
            Ok(len) => len as isize,
            Err(err) => {
                err.log_with_object(&*wrap);
                -1
            }
        }
    })
}

unsafe extern "C" fn read<T: RTPHeaderExtensionImpl>(
    ptr: *mut ffi::GstRTPHeaderExtension,
    read_flags: ffi::GstRTPHeaderExtensionFlags,
    input_data: *const u8,
    input_data_len: usize,
    output: *mut gst::ffi::GstBuffer,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<crate::RTPHeaderExtension> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.read(
            wrap.unsafe_cast_ref(),
            from_glib(read_flags),
            if input_data_len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(input_data, input_data_len)
            },
            gst::BufferRef::from_mut_ptr(output),
        ) {
            Ok(_) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn set_non_rtp_sink_caps<T: RTPHeaderExtensionImpl>(
    ptr: *mut ffi::GstRTPHeaderExtension,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<crate::RTPHeaderExtension> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.set_non_rtp_sink_caps(wrap.unsafe_cast_ref(), &from_glib_borrow(caps)) {
            Ok(_) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn update_non_rtp_src_caps<T: RTPHeaderExtensionImpl>(
    ptr: *mut ffi::GstRTPHeaderExtension,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<crate::RTPHeaderExtension> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.update_non_rtp_src_caps(wrap.unsafe_cast_ref(), gst::CapsRef::from_mut_ptr(caps))
        {
            Ok(_) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn set_attributes<T: RTPHeaderExtensionImpl>(
    ptr: *mut ffi::GstRTPHeaderExtension,
    direction: ffi::GstRTPHeaderExtensionDirection,
    attributes: *const libc::c_char,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<crate::RTPHeaderExtension> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.set_attributes(
            wrap.unsafe_cast_ref(),
            from_glib(direction),
            &glib::GString::from_glib_borrow(attributes),
        ) {
            Ok(_) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn set_caps_from_attributes<T: RTPHeaderExtensionImpl>(
    ptr: *mut ffi::GstRTPHeaderExtension,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<crate::RTPHeaderExtension> = from_glib_borrow(ptr);

    gst::panic_to_error!(&wrap, imp.panicked(), false, {
        match imp.set_caps_from_attributes(wrap.unsafe_cast_ref(), gst::CapsRef::from_mut_ptr(caps))
        {
            Ok(_) => true,
            Err(err) => {
                err.log_with_object(&*wrap);
                false
            }
        }
    })
    .into_glib()
}
