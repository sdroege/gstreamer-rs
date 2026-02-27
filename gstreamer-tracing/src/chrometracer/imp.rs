use gstreamer::{glib, prelude::*, subclass::prelude::*};
use std::str::FromStr;
use std::sync::Mutex;
use tracing::error;
use tracing_subscriber::prelude::*;

use crate::tracer::{TracingTracer, TracingTracerImpl};

#[derive(Default)]
struct State {
    chrome_guard: Option<tracing_chrome::FlushGuard>,
}

#[derive(Default)]
pub struct ChromeTracer {
    state: Mutex<State>,
}

#[glib::object_subclass]
impl ObjectSubclass for ChromeTracer {
    const NAME: &'static str = "ChromeTracer";
    type Type = super::ChromeTracer;
    type ParentType = TracingTracer;
    type Interfaces = ();
}

impl ObjectImpl for ChromeTracer {
    fn constructed(&self) {
        let mut include_args = true;

        if let Some(params) = self.obj().property::<Option<String>>("params") {
            let tmp = format!("params,{}", params);
            include_args = gstreamer::Structure::from_str(&tmp)
                .unwrap_or_else(|e| {
                    eprintln!("Invalid params string: {:?}: {e:?}", tmp);
                    gstreamer::Structure::new_empty("params")
                })
                .get::<bool>("include-args")
                .unwrap_or(true)
        }

        let (chrome_layer, guard) = tracing_chrome::ChromeLayerBuilder::new()
            .include_args(include_args)
            .build();

        self.state.lock().unwrap().chrome_guard = Some(guard);
        if let Err(e) = tracing_subscriber::registry().with(chrome_layer).try_init() {
            error!("Failed to initialize tracing subscriber: {e:?}");
        }

        self.parent_constructed();
    }
}

impl GstObjectImpl for ChromeTracer {}
impl TracerImpl for ChromeTracer {}
impl TracingTracerImpl for ChromeTracer {}
