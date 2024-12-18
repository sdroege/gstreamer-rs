// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{ffi, prelude::*, Pipeline, PipelineFlags};

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
    pub fn builder<'a>() -> PipelineBuilder<'a> {
        assert_initialized_main_thread!();
        PipelineBuilder {
            builder: crate::Object::builder(),
        }
    }
}

pub trait GstPipelineExtManual: IsA<Pipeline> + 'static {
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
pub struct PipelineBuilder<'a> {
    builder: crate::gobject::GObjectBuilder<'a, Pipeline>,
}

impl<'a> PipelineBuilder<'a> {
    // rustdoc-stripper-ignore-next
    /// Build the [`Pipeline`].
    ///
    /// # Panics
    ///
    /// This panics if the [`Pipeline`] doesn't have all the given properties or
    /// property values of the wrong type are provided.
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Pipeline {
        self.builder.build().unwrap()
    }

    pub fn auto_flush_bus(self, auto_flush_bus: bool) -> Self {
        Self {
            builder: self.builder.property("auto-flush-bus", auto_flush_bus),
        }
    }

    pub fn auto_flush_bus_if_some(self, auto_flush_bus: Option<bool>) -> Self {
        if let Some(auto_flush_bus) = auto_flush_bus {
            self.auto_flush_bus(auto_flush_bus)
        } else {
            self
        }
    }

    pub fn delay(self, delay: u64) -> Self {
        Self {
            builder: self.builder.property("delay", delay),
        }
    }

    pub fn delay_if(self, delay: u64, predicate: bool) -> Self {
        if predicate {
            self.delay(delay)
        } else {
            self
        }
    }

    pub fn delay_if_some(self, delay: Option<u64>) -> Self {
        if let Some(delay) = delay {
            self.delay(delay)
        } else {
            self
        }
    }

    pub fn latency(self, latency: impl Into<Option<crate::ClockTime>>) -> Self {
        if let Some(latency) = latency.into() {
            Self {
                builder: self.builder.property("latency", latency),
            }
        } else {
            self
        }
    }

    pub fn latency_if(self, latency: impl Into<Option<crate::ClockTime>>, predicate: bool) -> Self {
        if predicate {
            self.latency(latency)
        } else {
            self
        }
    }

    pub fn latency_if_some(self, latency: Option<crate::ClockTime>) -> Self {
        if let Some(latency) = latency {
            self.latency(latency)
        } else {
            self
        }
    }

    pub fn async_handling(self, async_handling: bool) -> Self {
        Self {
            builder: self.builder.property("async-handling", async_handling),
        }
    }

    pub fn async_handling_if_some(self, async_handling: Option<bool>) -> Self {
        if let Some(async_handling) = async_handling {
            self.async_handling(async_handling)
        } else {
            self
        }
    }

    pub fn message_forward(self, message_forward: bool) -> Self {
        Self {
            builder: self.builder.property("message-forward", message_forward),
        }
    }

    pub fn message_forward_if_some(self, message_forward: Option<bool>) -> Self {
        if let Some(message_forward) = message_forward {
            self.message_forward(message_forward)
        } else {
            self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets property `name` to the given value `value`.
    ///
    /// Overrides any default or previously defined value for `name`.
    #[inline]
    pub fn property(self, name: &'a str, value: impl Into<glib::Value> + 'a) -> Self {
        Self {
            builder: self.builder.property(name, value),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets property `name` to the given string value `value`.
    #[inline]
    pub fn property_from_str(self, name: &'a str, value: &'a str) -> Self {
        Self {
            builder: self.builder.property_from_str(name, value),
        }
    }

    impl_builder_gvalue_extra_setters!(property_and_name);
}
