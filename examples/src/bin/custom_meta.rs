// This example demonstrates how custom GstMeta can be defined and used on buffers.
//
// It simply attaches a GstMeta with a Rust String to buffers that are passed into
// an appsrc and retrieves them again from an appsink.

#[macro_use]
extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_app as gst_app;

#[path = "../examples-common.rs"]
mod examples_common;

mod custom_meta {
    use gst::gst_sys;
    use gst::prelude::*;
    use std::fmt;
    use std::ptr;

    // Public Rust type for the custom meta.
    #[repr(C)]
    pub struct CustomMeta(imp::CustomMeta);

    // Metas must be Send+Sync.
    unsafe impl Send for CustomMeta {}
    unsafe impl Sync for CustomMeta {}

    impl CustomMeta {
        // Add a new custom meta to the buffer with the given label.
        pub fn add(
            buffer: &mut gst::BufferRef,
            label: String,
        ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
            unsafe {
                // First add it: this will store an empty label via custom_meta_init().
                let meta = gst_sys::gst_buffer_add_meta(
                    buffer.as_mut_ptr(),
                    imp::custom_meta_get_info(),
                    ptr::null_mut(),
                ) as *mut imp::CustomMeta;

                // Then actually set the label.
                {
                    let meta = &mut *meta;
                    meta.label = label;
                }

                Self::from_mut_ptr(buffer, meta)
            }
        }

        // Retrieve the stored label.
        pub fn get_label(&self) -> &str {
            self.0.label.as_str()
        }
    }

    // Trait to allow using the gst::Buffer API with this meta.
    unsafe impl MetaAPI for CustomMeta {
        type GstType = imp::CustomMeta;

        fn get_meta_api() -> glib::Type {
            imp::custom_meta_api_get_type()
        }
    }

    impl fmt::Debug for CustomMeta {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("CustomMeta")
                .field("label", &self.get_label())
                .finish()
        }
    }

    // Actual unsafe implementation of the meta.
    mod imp {
        use glib::glib_sys;
        use glib::translate::*;
        use gst::gst_sys;
        use gst::prelude::*;
        use once_cell::sync::Lazy;
        use std::mem;
        use std::ptr;

        // This is the C type that is actually stored as meta inside the buffers.
        #[repr(C)]
        pub struct CustomMeta {
            parent: gst_sys::GstMeta,
            pub(super) label: String,
        }

        // Function to register the meta API and get a type back.
        pub(super) fn custom_meta_api_get_type() -> glib::Type {
            static TYPE: Lazy<glib::Type> = Lazy::new(|| unsafe {
                let t = from_glib(gst_sys::gst_meta_api_type_register(
                    b"MyCustomMetaAPI\0".as_ptr() as *const _,
                    // We provide no tags here as our meta is just a label and does
                    // not refer to any specific aspect of the buffer
                    [ptr::null::<std::os::raw::c_char>()].as_ptr() as *mut *const _,
                ));

                assert_ne!(t, glib::Type::Invalid);

                t
            });

            *TYPE
        }

        // Initialization function for our meta. This needs to ensure all fields are correctly
        // initialized. They will contain random memory before.
        unsafe extern "C" fn custom_meta_init(
            meta: *mut gst_sys::GstMeta,
            _params: glib_sys::gpointer,
            _buffer: *mut gst_sys::GstBuffer,
        ) -> glib_sys::gboolean {
            let meta = &mut *(meta as *mut CustomMeta);

            // Need to initialize all our fields correctly here
            ptr::write(&mut meta.label, String::new());

            true.to_glib()
        }

        // Free function for our meta. This needs to free/drop all memory we allocated.
        unsafe extern "C" fn custom_meta_free(
            meta: *mut gst_sys::GstMeta,
            _buffer: *mut gst_sys::GstBuffer,
        ) {
            let meta = &mut *(meta as *mut CustomMeta);

            // Need to free/drop all our fields here.
            ptr::drop_in_place(&mut meta.label);
        }

        // Transform function for our meta. This needs to get it from the old buffer to the new one
        // in a way that is compatible with the transformation type. In this case we just always
        // copy it over.
        unsafe extern "C" fn custom_meta_transform(
            dest: *mut gst_sys::GstBuffer,
            meta: *mut gst_sys::GstMeta,
            _buffer: *mut gst_sys::GstBuffer,
            _type_: glib_sys::GQuark,
            _data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let meta = &mut *(meta as *mut CustomMeta);

            // We simply copy over our meta here. Other metas might have to look at the type
            // and do things conditional on that, or even just drop the meta.
            super::CustomMeta::add(gst::BufferRef::from_mut_ptr(dest), meta.label.clone());

            true.to_glib()
        }

        // Register the meta itself with its functions.
        pub(super) fn custom_meta_get_info() -> *const gst_sys::GstMetaInfo {
            struct MetaInfo(ptr::NonNull<gst_sys::GstMetaInfo>);
            unsafe impl Send for MetaInfo {}
            unsafe impl Sync for MetaInfo {}

            static META_INFO: Lazy<MetaInfo> = Lazy::new(|| unsafe {
                MetaInfo(
                    ptr::NonNull::new(gst_sys::gst_meta_register(
                        custom_meta_api_get_type().to_glib(),
                        b"MyCustomMeta\0".as_ptr() as *const _,
                        mem::size_of::<CustomMeta>(),
                        Some(custom_meta_init),
                        Some(custom_meta_free),
                        Some(custom_meta_transform),
                    ) as *mut gst_sys::GstMetaInfo)
                    .expect("Failed to register meta API"),
                )
            });

            META_INFO.0.as_ptr()
        }
    }
}

fn example_main() {
    gst::init().unwrap();

    // This creates a pipeline with appsrc and appsink.
    let pipeline = gst::Pipeline::new(None);
    let appsrc = gst::ElementFactory::make("appsrc", None)
        .unwrap()
        .downcast::<gst_app::AppSrc>()
        .unwrap();
    let appsink = gst::ElementFactory::make("appsink", None)
        .unwrap()
        .downcast::<gst_app::AppSink>()
        .unwrap();

    pipeline.add(&appsrc).unwrap();
    pipeline.add(&appsink).unwrap();
    appsrc.link(&appsink).unwrap();

    // Our buffer counter, that is stored in the mutable environment
    // of the closure of the need-data callback.
    let mut i = 0;
    appsrc.set_callbacks(
        gst_app::AppSrcCallbacks::new()
            .need_data(move |appsrc, _| {
                // We only produce 5 buffers.
                if i == 5 {
                    let _ = appsrc.end_of_stream();
                    return;
                }

                println!("Producing buffer {}", i);

                // Add a custom meta with a label to this buffer.
                let mut buffer = gst::Buffer::new();
                {
                    let buffer = buffer.get_mut().unwrap();
                    custom_meta::CustomMeta::add(buffer, format!("This is buffer {}", i));
                }

                i += 1;

                // appsrc already handles the error here for us.
                let _ = appsrc.push_buffer(buffer);
            })
            .build(),
    );

    // Getting data out of the appsink is done by setting callbacks on it.
    // The appsink will then call those handlers, as soon as data is available.
    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::new()
            // Add a handler to the "new-sample" signal.
            .new_sample(|appsink| {
                // Pull the sample in question out of the appsink's buffer.
                let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.get_buffer().ok_or_else(|| {
                    gst_element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gst::FlowError::Error
                })?;

                // Retrieve the custom meta from the buffer and print it.
                let meta = buffer
                    .get_meta::<custom_meta::CustomMeta>()
                    .expect("No custom meta found");
                println!("Got buffer with label: {}", meta.get_label());

                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );

    // Actually start the pipeline.
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    // And run until EOS or an error happened.
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Finally shut down everything.
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // tutorials_common::run is only required to set up the application environent on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically).
    examples_common::run(example_main);
}
