// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;

use crate::Pipeline;
use crate::PipelineFlags;

impl Pipeline {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Pipeline`] objects.
    ///
    /// This method returns an instance of [`PipelineBuilder`](crate::builders::PipelineBuilder) which can be used to create [`Pipeline`] objects.
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::default()
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
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Pipeline`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PipelineBuilder {
    auto_flush_bus: Option<bool>,
    delay: Option<u64>,
    latency: Option<u64>,
    async_handling: Option<bool>,
    message_forward: Option<bool>,
    name: Option<String>,
}

impl PipelineBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PipelineBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Pipeline`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Pipeline {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref auto_flush_bus) = self.auto_flush_bus {
            properties.push(("auto-flush-bus", auto_flush_bus));
        }
        if let Some(ref delay) = self.delay {
            properties.push(("delay", delay));
        }
        if let Some(ref latency) = self.latency {
            properties.push(("latency", latency));
        }
        if let Some(ref async_handling) = self.async_handling {
            properties.push(("async-handling", async_handling));
        }
        if let Some(ref message_forward) = self.message_forward {
            properties.push(("message-forward", message_forward));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        glib::Object::new::<Pipeline>(&properties)
    }

    pub fn auto_flush_bus(mut self, auto_flush_bus: bool) -> Self {
        self.auto_flush_bus = Some(auto_flush_bus);
        self
    }

    pub fn delay(mut self, delay: u64) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn latency(mut self, latency: u64) -> Self {
        self.latency = Some(latency);
        self
    }

    pub fn async_handling(mut self, async_handling: bool) -> Self {
        self.async_handling = Some(async_handling);
        self
    }

    pub fn message_forward(mut self, message_forward: bool) -> Self {
        self.message_forward = Some(message_forward);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}
