// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{prelude::*, Pipeline, PipelineFlags};

impl Pipeline {
    // rustdoc-stripper-ignore-next
    /// Creates a new [`Pipeline`] object with a default name.
    ///
    /// Use [`Pipeline::with_name()`] to create a [`Pipeline`] with a specific name.
    /// Use [`Pipeline::builder()`] to get a [`PipelineBuilder`] and then define a specific name.
    #[doc(alias = "gst_pipeline_new")]
    pub fn new() -> Pipeline {
        assert_initialized_main_thread!();
        unsafe {
            crate::Element::from_glib_none(ffi::gst_pipeline_new(std::ptr::null())).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`Pipeline`] object with the specified name.
    ///
    /// Use [`Pipeline::builder()`] for additional configuration.
    #[doc(alias = "gst_pipeline_new")]
    pub fn with_name(name: &str) -> Pipeline {
        assert_initialized_main_thread!();
        unsafe {
            crate::Element::from_glib_none(ffi::gst_pipeline_new(name.to_glib_none().0))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Pipeline`] objects.
    ///
    /// This method returns an instance of [`PipelineBuilder`] which can be used to create [`Pipeline`] objects.
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Pipeline>> Sealed for T {}
}

pub trait GstPipelineExtManual: sealed::Sealed + IsA<Pipeline> + 'static {
    fn set_pipeline_flags(&self, flags: PipelineFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_pipeline_flags(&self, flags: PipelineFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags &= !flags.into_glib();
        }
    }

    #[doc(alias = "get_pipeline_flags")]
    fn pipeline_flags(&self) -> PipelineFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            from_glib((*ptr).flags)
        }
    }
}

impl<O: IsA<Pipeline>> GstPipelineExtManual for O {}

impl Default for Pipeline {
    fn default() -> Self {
        glib::object::Object::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Pipeline`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PipelineBuilder {
    builder: glib::object::ObjectBuilder<'static, Pipeline>,
}

impl PipelineBuilder {
    fn new() -> Self {
        Self {
            builder: glib::Object::builder(),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Pipeline`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Pipeline {
        self.builder.build()
    }

    pub fn auto_flush_bus(self, auto_flush_bus: bool) -> Self {
        Self {
            builder: self.builder.property("auto-flush-bus", auto_flush_bus),
        }
    }

    pub fn delay(self, delay: u64) -> Self {
        Self {
            builder: self.builder.property("delay", delay),
        }
    }

    pub fn latency(self, latency: crate::ClockTime) -> Self {
        Self {
            builder: self.builder.property("latency", latency),
        }
    }

    pub fn async_handling(self, async_handling: bool) -> Self {
        Self {
            builder: self.builder.property("async-handling", async_handling),
        }
    }

    pub fn message_forward(self, message_forward: bool) -> Self {
        Self {
            builder: self.builder.property("message-forward", message_forward),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }
}
