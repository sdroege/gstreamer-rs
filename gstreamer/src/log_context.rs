// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, io::Write, ptr};

use glib::{prelude::*, translate::*};

use crate::log::DebugLogger;
use crate::{ClockTime, DebugCategory, DebugLevel, LogContextFlags, LogContextHashFlags, ffi};

#[derive(Debug)]
#[doc(alias = "GstLogContext")]
#[repr(transparent)]
pub struct LogContext(ptr::NonNull<ffi::GstLogContext>);

impl LogContext {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_log_context_get_category")]
    #[doc(alias = "get_category")]
    #[inline]
    pub fn category(&self) -> DebugCategory {
        unsafe { from_glib_none(ffi::gst_log_context_get_category(self.0.as_ptr())) }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_log_context_reset")]
    #[inline]
    pub fn reset(&self) {
        unsafe {
            ffi::gst_log_context_reset(self.0.as_ptr());
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut ffi::GstLogContext {
        self.0.as_ptr()
    }

    // rustdoc-stripper-ignore-next
    /// Logs without checking the log level.
    #[inline(never)]
    fn log_unfiltered_internal(
        &self,
        obj: Option<&glib::Object>,
        level: DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        let mut w = smallvec::SmallVec::<[u8; 256]>::new();

        // Can't really happen but better safe than sorry
        if Write::write_fmt(&mut w, args).is_err() {
            return;
        }
        w.push(0);

        self.log_literal_unfiltered_internal(obj, level, file, function, line, unsafe {
            glib::GStr::from_utf8_with_nul_unchecked(&w)
        });
    }

    #[inline(never)]
    fn log_literal_unfiltered_internal(
        &self,
        obj: Option<&glib::Object>,
        level: DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        let obj_ptr = match obj {
            Some(obj) => obj.as_ptr(),
            None => ptr::null_mut(),
        };

        function.run_with_gstr(|function| unsafe {
            ffi::gst_debug_log_literal_with_context(
                self.0.as_ptr(),
                level.into_glib(),
                file.as_ptr(),
                function.as_ptr(),
                line as i32,
                obj_ptr,
                msg.as_ptr(),
            );
        });
    }

    #[inline(never)]
    fn log_id_unfiltered_internal(
        &self,
        id: &glib::GStr,
        level: DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        let mut w = smallvec::SmallVec::<[u8; 256]>::new();

        // Can't really happen but better safe than sorry
        if Write::write_fmt(&mut w, args).is_err() {
            return;
        }
        w.push(0);

        self.log_id_literal_unfiltered_internal(id, level, file, function, line, unsafe {
            glib::GStr::from_utf8_with_nul_unchecked(&w)
        });
    }

    #[inline(never)]
    fn log_id_literal_unfiltered_internal(
        &self,
        id: &glib::GStr,
        level: DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        function.run_with_gstr(|function| unsafe {
            ffi::gst_debug_log_id_literal_with_context(
                self.0.as_ptr(),
                level.into_glib(),
                file.as_ptr(),
                function.as_ptr(),
                line as i32,
                id.as_ptr(),
                msg.as_ptr(),
            );
        });
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_debug_log_with_context")]
    pub fn log(
        &self,
        obj: Option<&impl IsA<glib::Object>>,
        level: DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        if !self.above_threshold(level) {
            return;
        }

        self.log_unfiltered_internal(
            obj.map(|obj| obj.as_ref()),
            level,
            file,
            function,
            line,
            args,
        )
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_debug_log_literal_with_context")]
    pub fn log_literal(
        &self,
        obj: Option<&impl IsA<glib::Object>>,
        level: DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        if !self.above_threshold(level) {
            return;
        }

        self.log_literal_unfiltered_internal(
            obj.map(|obj| obj.as_ref()),
            level,
            file,
            function,
            line,
            msg,
        )
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_debug_log_id_with_context")]
    pub fn log_id(
        &self,
        id: impl AsRef<glib::GStr>,
        level: DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        if !self.above_threshold(level) {
            return;
        }

        self.log_id_unfiltered_internal(id.as_ref(), level, file, function, line, args);
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_debug_log_id_literal_with_context")]
    pub fn log_id_literal(
        &self,
        id: impl AsRef<glib::GStr>,
        level: DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        if !self.above_threshold(level) {
            return;
        }

        self.log_id_literal_unfiltered_internal(id.as_ref(), level, file, function, line, msg);
    }
}

impl Drop for LogContext {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_log_context_free(self.0.as_ptr());
        }
    }
}

unsafe impl Send for LogContext {}
unsafe impl Sync for LogContext {}

impl crate::log::DebugLogger for LogContext {
    #[inline]
    fn above_threshold(&self, level: crate::DebugLevel) -> bool {
        self.category().above_threshold(level)
    }

    #[inline]
    fn log_unfiltered(
        &self,
        obj: Option<&impl IsA<glib::Object>>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: std::fmt::Arguments,
    ) {
        self.log_unfiltered_internal(
            obj.map(|obj| obj.as_ref()),
            level,
            file,
            function,
            line,
            args,
        )
    }

    #[inline]
    fn log_literal_unfiltered(
        &self,
        obj: Option<&impl IsA<glib::Object>>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        self.log_literal_unfiltered_internal(
            obj.map(|obj| obj.as_ref()),
            level,
            file,
            function,
            line,
            msg,
        )
    }

    #[inline]
    fn log_id_unfiltered(
        &self,
        id: impl AsRef<glib::GStr>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: std::fmt::Arguments,
    ) {
        self.log_id_unfiltered_internal(id.as_ref(), level, file, function, line, args)
    }

    // rustdoc-stripper-ignore-next
    /// Logs without checking the log level.
    #[inline]
    fn log_id_literal_unfiltered(
        &self,
        id: impl AsRef<glib::GStr>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        self.log_id_literal_unfiltered_internal(id.as_ref(), level, file, function, line, msg)
    }
}

#[derive(Debug)]
#[doc(alias = "GstLogContextBuilder")]
#[repr(transparent)]
pub struct LogContextBuilder(ptr::NonNull<ffi::GstLogContextBuilder>);

impl LogContextBuilder {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_log_context_builder_new")]
    pub fn new(category: DebugCategory, flags: LogContextFlags) -> Self {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_log_context_builder_new(category.as_ptr(), flags.into_glib());
            LogContextBuilder(ptr::NonNull::new_unchecked(ptr))
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_log_context_builder_set_hash_flags")]
    pub fn hash_flags(self, flags: LogContextHashFlags) -> Self {
        unsafe {
            ffi::gst_log_context_builder_set_hash_flags(self.0.as_ptr(), flags.into_glib());
        }
        self
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_log_context_builder_set_category")]
    pub fn category(self, category: DebugCategory) -> Self {
        unsafe {
            ffi::gst_log_context_builder_set_category(self.0.as_ptr(), category.as_ptr());
        }
        self
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_log_context_builder_set_interval")]
    pub fn interval(self, interval: impl Into<Option<ClockTime>>) -> Self {
        unsafe {
            ffi::gst_log_context_builder_set_interval(self.0.as_ptr(), interval.into().into_glib());
        }
        self
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_log_context_builder_build")]
    pub fn build(self) -> LogContext {
        unsafe {
            let ptr = ffi::gst_log_context_builder_build(self.0.as_ptr());
            LogContext(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DebugLevel, LogContextFlags, LogContextHashFlags, log::DebugLogger};

    #[test]
    fn log_context_builder_basic() {
        crate::init().unwrap();

        let cat = crate::DebugCategory::new(
            "test-log-context",
            crate::DebugColorFlags::empty(),
            Some("Test log context category"),
        );

        // Test basic builder pattern
        let context = LogContextBuilder::new(cat, LogContextFlags::empty()).build();

        assert_eq!(context.category(), cat);
    }

    #[test]
    fn log_context_builder_with_flags() {
        crate::init().unwrap();

        let cat = crate::DebugCategory::new(
            "test-log-context-flags",
            crate::DebugColorFlags::empty(),
            Some("Test log context with flags"),
        );

        // Test builder with various configuration options
        let context = LogContextBuilder::new(cat, LogContextFlags::THROTTLE)
            .hash_flags(LogContextHashFlags::USE_LINE_NUMBER)
            .interval(Some(crate::ClockTime::from_seconds(1)))
            .build();

        assert_eq!(context.category(), cat);
    }

    #[test]
    fn log_context_trait_implementation() {
        crate::init().unwrap();

        let cat = crate::DebugCategory::new(
            "test-trait-impl",
            crate::DebugColorFlags::empty(),
            Some("Test trait implementation"),
        );

        let context = LogContextBuilder::new(cat, LogContextFlags::empty()).build();

        // Test that LogContext implements DebugLogger trait
        assert_eq!(
            context.above_threshold(DebugLevel::Error),
            cat.above_threshold(DebugLevel::Error)
        );
        assert_eq!(
            context.above_threshold(DebugLevel::Debug),
            cat.above_threshold(DebugLevel::Debug)
        );
    }

    #[test]
    fn log_context_with_macros() {
        crate::init().unwrap();

        let cat = crate::DebugCategory::new(
            "test-macro-usage",
            crate::DebugColorFlags::empty(),
            Some("Test macro usage"),
        );
        cat.set_threshold(DebugLevel::Trace);

        let context = LogContextBuilder::new(cat, LogContextFlags::empty()).build();

        // Test that LogContext works with all the logging macros
        crate::error!(context, "error message");
        crate::error!(&context, "error message");
        crate::warning!(context, "warning message");
        crate::warning!(&context, "warning message");
        crate::info!(context, "info message");
        crate::info!(&context, "info message");
        crate::debug!(context, "debug message");
        crate::debug!(&context, "debug message");
        crate::trace!(context, "trace message");
        crate::trace!(&context, "trace message");

        // Test with object
        let obj = crate::Bin::with_name("test-bin");
        crate::error!(context, obj = &obj, "error with object");
        crate::warning!(context, obj = &obj, "warning with object");

        // Test with formatting
        let value = 42;
        crate::info!(context, "formatted message: {}", value);
        crate::debug!(context, obj = &obj, "formatted with obj: {}", value);
    }

    #[test]
    fn log_context_interchangeable_with_category() {
        crate::init().unwrap();

        let cat = crate::DebugCategory::new(
            "test-interchangeable",
            crate::DebugColorFlags::empty(),
            Some("Test interchangeable usage"),
        );
        cat.set_threshold(DebugLevel::Info);

        let context = LogContextBuilder::new(cat, LogContextFlags::empty()).build();

        // Test that we can use both category and context in the same way
        let test_message = "test message";

        // These should both work identically
        crate::info!(cat, "{}", test_message);
        crate::info!(&cat, "{}", test_message);
        crate::info!(context, "{}", test_message);
        crate::info!(&context, "{}", test_message);

        // With objects too
        let obj = crate::Bin::with_name("test-bin-2");
        crate::info!(cat, obj = &obj, "{}", test_message);
        crate::info!(context, obj = &obj, "{}", test_message);
    }

    #[test]
    fn static_log_context() {
        crate::init().unwrap();

        // Create a static category first
        static TEST_CATEGORY: std::sync::LazyLock<crate::DebugCategory> =
            std::sync::LazyLock::new(|| {
                crate::DebugCategory::new(
                    "test-static-context",
                    crate::DebugColorFlags::empty(),
                    Some("Test static context"),
                )
            });

        // Create static context directly with LazyLock
        static TEST_CONTEXT: std::sync::LazyLock<LogContext> = std::sync::LazyLock::new(|| {
            LogContextBuilder::new(*TEST_CATEGORY, LogContextFlags::empty()).build()
        });

        // Use the static context
        crate::info!(TEST_CONTEXT, "message from static context");

        assert_eq!(TEST_CONTEXT.category(), *TEST_CATEGORY);
    }

    #[test]
    fn static_log_context_with_advanced_options() {
        crate::init().unwrap();

        // Create a static category first
        static ADVANCED_CATEGORY: std::sync::LazyLock<crate::DebugCategory> =
            std::sync::LazyLock::new(|| {
                crate::DebugCategory::new(
                    "test-static-advanced",
                    crate::DebugColorFlags::empty(),
                    Some("Test static context advanced"),
                )
            });

        // Create static context with advanced options using LazyLock
        static ADVANCED_CONTEXT: std::sync::LazyLock<LogContext> = std::sync::LazyLock::new(|| {
            LogContextBuilder::new(*ADVANCED_CATEGORY, LogContextFlags::THROTTLE)
                .hash_flags(LogContextHashFlags::USE_LINE_NUMBER)
                .interval(Some(crate::ClockTime::from_seconds(2)))
                .build()
        });

        crate::debug!(ADVANCED_CONTEXT, "advanced static context message");
        assert_eq!(ADVANCED_CONTEXT.category(), *ADVANCED_CATEGORY);
    }

    #[test]
    fn log_context_with_id_logging() {
        crate::init().unwrap();

        let cat = crate::DebugCategory::new(
            "test-id-logging",
            crate::DebugColorFlags::empty(),
            Some("Test ID logging"),
        );
        cat.set_threshold(DebugLevel::Trace);

        let context = LogContextBuilder::new(cat, LogContextFlags::empty()).build();

        // Test ID-based logging with LogContext
        crate::trace!(context, id = "test-id-123", "message with ID");
        crate::debug!(
            context,
            id = "test-id-456",
            "another message with ID: {}",
            42
        );
    }

    #[test]
    fn log_context_memory_safety() {
        crate::init().unwrap();

        let cat = crate::DebugCategory::new(
            "test-memory-safety",
            crate::DebugColorFlags::empty(),
            Some("Test memory safety"),
        );

        // Test that LogContext can be safely dropped
        {
            let context = LogContextBuilder::new(cat, LogContextFlags::empty()).build();
            crate::info!(context, "message before drop");
        } // context is dropped here

        // Create another context to ensure no memory issues
        let context2 = LogContextBuilder::new(cat, LogContextFlags::THROTTLE).build();
        crate::info!(context2, "message from second context");
    }

    #[test]
    fn log_context_category_consistency() {
        crate::init().unwrap();

        let cat1 = crate::DebugCategory::new(
            "test-consistency-1",
            crate::DebugColorFlags::empty(),
            Some("Test consistency 1"),
        );

        let cat2 = crate::DebugCategory::new(
            "test-consistency-2",
            crate::DebugColorFlags::empty(),
            Some("Test consistency 2"),
        );

        let context1 = LogContextBuilder::new(cat1, LogContextFlags::empty()).build();
        let context2 = LogContextBuilder::new(cat2, LogContextFlags::empty()).build();

        // Verify that contexts maintain their respective categories
        assert_eq!(context1.category(), cat1);
        assert_eq!(context2.category(), cat2);
        assert_ne!(context1.category(), cat2);
        assert_ne!(context2.category(), cat1);
    }

    #[test]
    fn log_context_threshold_behavior() {
        crate::init().unwrap();

        let cat = crate::DebugCategory::new(
            "test-threshold",
            crate::DebugColorFlags::empty(),
            Some("Test threshold behavior"),
        );

        let context = LogContextBuilder::new(cat, LogContextFlags::empty()).build();

        // Test threshold behavior matches between category and context
        cat.set_threshold(DebugLevel::Warning);

        assert!(context.above_threshold(DebugLevel::Error));
        assert!(context.above_threshold(DebugLevel::Warning));
        assert!(!context.above_threshold(DebugLevel::Info));
        assert!(!context.above_threshold(DebugLevel::Debug));

        // Same as category
        assert_eq!(
            context.above_threshold(DebugLevel::Error),
            cat.above_threshold(DebugLevel::Error)
        );
        assert_eq!(
            context.above_threshold(DebugLevel::Warning),
            cat.above_threshold(DebugLevel::Warning)
        );
        assert_eq!(
            context.above_threshold(DebugLevel::Info),
            cat.above_threshold(DebugLevel::Info)
        );
        assert_eq!(
            context.above_threshold(DebugLevel::Debug),
            cat.above_threshold(DebugLevel::Debug)
        );
    }
}
