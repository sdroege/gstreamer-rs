use crate::*;
use glib::{translate::ToGlibPtr, Cast, ObjectType};
use gstreamer as g;
use std::{collections::VecDeque, sync::Mutex};
use tracing::{
    field::Visit,
    span::{Attributes, Record},
    Id,
};

struct GstEvent {
    message: &'static str,
    gobject: Option<(usize, &'static str)>,
    level: Level,
    target: &'static str,
}

impl Visit for GstEvent {
    fn record_i64(&mut self, field: &tracing_core::Field, _: i64) {
        panic!("unexpected i64 field: {}", field.name());
    }

    fn record_u64(&mut self, field: &tracing_core::Field, value: u64) {
        if field.name() == "gobject_address" {
            assert_eq!(
                value,
                self.gobject.expect("gobject present but not expected").0 as u64
            );
        } else {
            panic!("unexpected u64 field: {}", field.name());
        }
    }

    fn record_bool(&mut self, field: &tracing_core::Field, _: bool) {
        panic!("unexpected boolean field: {}", field.name());
    }

    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        match field.name() {
            "message" => assert_eq!(value, self.message),
            "gobject_type" => {
                assert_eq!(
                    value,
                    self.gobject.expect("gobject present but not expected").1
                );
            }
            _ => panic!("unexpected string field: {}", field.name()),
        }
    }

    fn record_error(&mut self, field: &tracing_core::Field, _: &(dyn std::error::Error + 'static)) {
        panic!("unexpected error field: {}", field.name());
    }

    fn record_debug(&mut self, field: &tracing_core::Field, _: &dyn std::fmt::Debug) {
        panic!("unexpected debug field: {}", field.name());
    }
}

enum Expect {
    GstEvent(GstEvent),
}

struct MockSubscriber {
    expected: Mutex<VecDeque<Expect>>,
    filter: fn(&Metadata<'_>) -> bool,
    name: &'static str,
}

impl MockSubscriber {
    fn with_expected<F>(
        filter: fn(&Metadata<'_>) -> bool,
        name: &'static str,
        cb: F,
        expected: Vec<Expect>,
    ) where
        F: FnOnce(),
    {
        let subscriber = MockSubscriber {
            expected: Mutex::new(expected.into()),
            name,
            filter,
        };
        let dispatch = tracing::Dispatch::new(subscriber);
        tracing::dispatcher::with_default(&dispatch, cb);
    }
}

impl tracing::Subscriber for MockSubscriber {
    fn enabled(&self, meta: &tracing_core::Metadata<'_>) -> bool {
        meta.name() == crate::NAME && (self.filter)(meta)
    }
    fn event(&self, e: &tracing_core::Event<'_>) {
        if !(self.filter)(e.metadata()) {
            return;
        }
        println!("[{}] event: {:?}", self.name, e);

        match self.expected.lock().expect("mutex lock").pop_front() {
            None => {
                panic!("[{}] unexpected extra event received", self.name)
            }
            Some(Expect::GstEvent(mut expected @ GstEvent { .. })) => {
                let meta = e.metadata();
                if meta.target() != expected.target {
                    panic!(
                        "[{}] event with target {} received, but expected {}",
                        self.name,
                        meta.target(),
                        expected.target,
                    );
                }
                if *meta.level() != expected.level {
                    panic!(
                        "[{}] event with level {} received, but expected {}",
                        self.name,
                        meta.level(),
                        expected.level,
                    );
                }
                e.record(&mut expected);
            }
        }
    }
    fn exit(&self, _: &Id) {
        todo!()
    }
    fn enter(&self, _: &Id) {
        todo!()
    }
    fn record_follows_from(&self, _: &Id, _: &Id) {
        todo!()
    }
    fn record(&self, _: &Id, _: &Record<'_>) {
        todo!()
    }
    fn new_span(&self, _: &Attributes<'_>) -> Id {
        todo!()
    }
}

fn test_simple_error() {
    MockSubscriber::with_expected(
        |_| true,
        "test_simple_error",
        || {
            let cat = g::DebugCategory::new("test_error_cat", g::DebugColorFlags::empty(), None);
            g::gst_error!(cat, "simple error");
        },
        vec![Expect::GstEvent(GstEvent {
            message: "simple error",
            gobject: None,
            level: Level::ERROR,
            target: "test_error_cat",
        })],
    );
}

fn test_simple_warning() {
    MockSubscriber::with_expected(
        |_| true,
        "test_simple_warning",
        || {
            let cat = g::DebugCategory::new("test_simple_cat", g::DebugColorFlags::empty(), None);
            g::gst_warning!(cat, "simple warning");
        },
        vec![Expect::GstEvent(GstEvent {
            message: "simple warning",
            gobject: None,
            level: Level::WARN,
            target: "test_simple_cat",
        })],
    );
}

fn test_simple_events() {
    MockSubscriber::with_expected(
        |_| true,
        "test_simple_events",
        || {
            let cat = g::DebugCategory::new("test_simple_cat", g::DebugColorFlags::empty(), None);
            g::gst_fixme!(cat, "simple fixme");
            g::gst_info!(cat, "simple info");
            g::gst_memdump!(cat, "simple memdump");
            g::gst_trace!(cat, "simple trace");
        },
        vec![
            Expect::GstEvent(GstEvent {
                message: "simple fixme",
                gobject: None,
                level: Level::WARN,
                target: "test_simple_cat",
            }),
            Expect::GstEvent(GstEvent {
                message: "simple info",
                gobject: None,
                level: Level::INFO,
                target: "test_simple_cat",
            }),
            Expect::GstEvent(GstEvent {
                message: "simple memdump",
                gobject: None,
                level: Level::TRACE,
                target: "test_simple_cat",
            }),
            Expect::GstEvent(GstEvent {
                message: "simple trace",
                gobject: None,
                level: Level::TRACE,
                target: "test_simple_cat",
            }),
        ],
    );
}

fn test_with_object() {
    let p = g::Pipeline::new(None);
    let p_addr = p.as_object_ref().to_glib_none().0 as usize;
    MockSubscriber::with_expected(
        |m| m.target() == "test_object_cat",
        "test_with_object",
        move || {
            let cat = g::DebugCategory::new("test_object_cat", g::DebugColorFlags::empty(), None);
            g::gst_error!(cat, obj: &p, "with object");
        },
        vec![Expect::GstEvent(GstEvent {
            message: "with object",
            gobject: Some((p_addr, "GstPipeline")),
            level: Level::ERROR,
            target: "test_object_cat",
        })],
    );
}

fn test_with_upcast_object() {
    let obj: glib::Object = g::Bin::new(None).upcast();
    let obj_addr = obj.as_object_ref().to_glib_none().0 as usize;
    MockSubscriber::with_expected(
        |m| m.target() == "test_object_cat",
        "test_with_upcast_object",
        move || {
            let cat = g::DebugCategory::new("test_object_cat", g::DebugColorFlags::empty(), None);
            g::gst_error!(cat, obj: &obj, "with upcast object");
        },
        vec![Expect::GstEvent(GstEvent {
            message: "with upcast object",
            gobject: Some((obj_addr, "GstBin")),
            level: Level::ERROR,
            target: "test_object_cat",
        })],
    );
}

fn test_disintegration() {
    MockSubscriber::with_expected(
        |m| m.target() == "disintegration",
        "test_disintegration",
        move || {
            let cat = g::DebugCategory::new("disintegration", g::DebugColorFlags::empty(), None);
            g::gst_error!(cat, "apple");
            disintegrate();
            g::gst_error!(cat, "banana");
            integrate();
            g::gst_error!(cat, "chaenomeles");
        },
        vec![
            Expect::GstEvent(GstEvent {
                message: "apple",
                gobject: None,
                level: Level::ERROR,
                target: "disintegration",
            }),
            Expect::GstEvent(GstEvent {
                message: "chaenomeles",
                gobject: None,
                level: Level::ERROR,
                target: "disintegration",
            }),
        ],
    );
}

fn test_formatting() {
    MockSubscriber::with_expected(
        |_| true,
        "test_formatting",
        || {
            let cat = g::DebugCategory::new("ANSWERS", g::DebugColorFlags::empty(), None);
            g::gst_warning!(cat, "the answer is believed to be {}.", 42);
        },
        vec![Expect::GstEvent(GstEvent {
            message: "the answer is believed to be 42.",
            gobject: None,
            level: Level::WARN,
            target: "ANSWERS",
        })],
    );
}

// NB: we aren't using the test harness here to allow us for the necessary gstreamer setup more
// straightforwardly.
pub(crate) fn run() {
    g::debug_remove_default_log_function();
    g::init().expect("gst init");
    g::debug_set_default_threshold(g::DebugLevel::Count);
    integrate();
    test_simple_error();
    test_simple_warning();
    test_simple_events();
    test_with_object();
    test_with_upcast_object();
    test_disintegration();
    test_formatting();
    disintegrate();
}
