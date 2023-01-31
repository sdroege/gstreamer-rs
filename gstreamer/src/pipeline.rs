// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{Pipeline, PipelineFlags};

impl Pipeline {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Pipeline`] objects.
    ///
    /// This method returns an instance of [`PipelineBuilder`](crate::builders::PipelineBuilder) which can be used to create [`Pipeline`] objects.
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::new()
    }
}

pub trait GstPipelineExtManual: 'static {
    fn set_pipeline_flags(&self, flags: PipelineFlags);

    fn unset_pipeline_flags(&self, flags: PipelineFlags);

    #[doc(alias = "get_pipeline_flags")]
    fn pipeline_flags(&self) -> PipelineFlags;
}

impl<O: IsA<crate::Pipeline>> GstPipelineExtManual for O {
    fn set_pipeline_flags(&self, flags: PipelineFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_pipeline_flags(&self, flags: PipelineFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.into_glib();
        }
    }

    fn pipeline_flags(&self) -> PipelineFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}

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
