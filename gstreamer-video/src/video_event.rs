// Take a look at the license at the top of the repository in the LICENSE file.
use crate::{NavigationCommand, NavigationEventType};
use glib::translate::{from_glib, from_glib_full, IntoGlib};
use glib::ToSendValue;
use gst::EventType;
use std::mem;

#[cfg(any(feature = "v1_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
use crate::NavigationModifierType;

// FIXME: Copy from gstreamer/src/event.rs
macro_rules! event_builder_generic_impl {
    ($new_fn:expr) => {
        pub fn seqnum(self, seqnum: gst::Seqnum) -> Self {
            Self {
                seqnum: Some(seqnum),
                ..self
            }
        }

        pub fn running_time_offset(self, running_time_offset: i64) -> Self {
            Self {
                running_time_offset: Some(running_time_offset),
                ..self
            }
        }

        pub fn other_fields(
            self,
            other_fields: &[(&'a str, &'a (dyn ToSendValue + Sync))],
        ) -> Self {
            Self {
                other_fields: self
                    .other_fields
                    .iter()
                    .cloned()
                    .chain(other_fields.iter().cloned())
                    .collect(),
                ..self
            }
        }

        #[must_use = "Building the event without using it has no effect"]
        pub fn build(mut self) -> gst::Event {
            assert_initialized_main_thread!();
            unsafe {
                let event = $new_fn(&mut self);
                if let Some(seqnum) = self.seqnum {
                    gst::ffi::gst_event_set_seqnum(event, seqnum.into_glib());
                }

                if let Some(running_time_offset) = self.running_time_offset {
                    gst::ffi::gst_event_set_running_time_offset(event, running_time_offset);
                }

                {
                    let s = gst::StructureRef::from_glib_borrow_mut(
                        gst::ffi::gst_event_writable_structure(event),
                    );

                    for (k, v) in self.other_fields {
                        s.set_value(k, v.to_send_value());
                    }
                }

                from_glib_full(event)
            }
        }
    };
}

#[must_use = "The builder must be built to be used"]
pub struct DownstreamForceKeyUnitEventBuilder<'a> {
    seqnum: Option<gst::Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a (dyn ToSendValue + Sync))>,
    timestamp: Option<gst::ClockTime>,
    stream_time: Option<gst::ClockTime>,
    running_time: Option<gst::ClockTime>,
    all_headers: bool,
    count: u32,
}

impl<'a> DownstreamForceKeyUnitEventBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            timestamp: gst::ClockTime::NONE,
            stream_time: gst::ClockTime::NONE,
            running_time: gst::ClockTime::NONE,
            all_headers: true,
            count: 0,
        }
    }

    pub fn timestamp(self, timestamp: impl Into<Option<gst::ClockTime>>) -> Self {
        Self {
            timestamp: timestamp.into(),
            ..self
        }
    }

    pub fn stream_time(self, stream_time: impl Into<Option<gst::ClockTime>>) -> Self {
        Self {
            stream_time: stream_time.into(),
            ..self
        }
    }

    pub fn running_time(self, running_time: impl Into<Option<gst::ClockTime>>) -> Self {
        Self {
            running_time: running_time.into(),
            ..self
        }
    }

    pub fn all_headers(self, all_headers: bool) -> Self {
        Self {
            all_headers,
            ..self
        }
    }

    pub fn count(self, count: u32) -> Self {
        Self { count, ..self }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        ffi::gst_video_event_new_downstream_force_key_unit(
            s.timestamp.into_glib(),
            s.stream_time.into_glib(),
            s.running_time.into_glib(),
            s.all_headers.into_glib(),
            s.count,
        )
    });
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DownstreamForceKeyUnitEvent {
    pub timestamp: Option<gst::ClockTime>,
    pub stream_time: Option<gst::ClockTime>,
    pub running_time: Option<gst::ClockTime>,
    pub all_headers: bool,
    pub count: u32,
}

impl DownstreamForceKeyUnitEvent {
    pub fn builder<'a>() -> DownstreamForceKeyUnitEventBuilder<'a> {
        DownstreamForceKeyUnitEventBuilder::new()
    }

    #[doc(alias = "gst_video_event_parse_downstream_force_key_unit")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut timestamp = mem::MaybeUninit::uninit();
            let mut stream_time = mem::MaybeUninit::uninit();
            let mut running_time = mem::MaybeUninit::uninit();
            let mut all_headers = mem::MaybeUninit::uninit();
            let mut count = mem::MaybeUninit::uninit();

            let res: bool = from_glib(ffi::gst_video_event_parse_downstream_force_key_unit(
                event.as_mut_ptr(),
                timestamp.as_mut_ptr(),
                stream_time.as_mut_ptr(),
                running_time.as_mut_ptr(),
                all_headers.as_mut_ptr(),
                count.as_mut_ptr(),
            ));
            if res {
                Ok(Self {
                    timestamp: from_glib(timestamp.assume_init()),
                    stream_time: from_glib(stream_time.assume_init()),
                    running_time: from_glib(running_time.assume_init()),
                    all_headers: from_glib(all_headers.assume_init()),
                    count: count.assume_init(),
                })
            } else {
                Err(glib::bool_error!("Failed to parse GstEvent"))
            }
        }
    }
}

#[must_use = "The builder must be built to be used"]
pub struct UpstreamForceKeyUnitEventBuilder<'a> {
    seqnum: Option<gst::Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a (dyn ToSendValue + Sync))>,
    running_time: Option<gst::ClockTime>,
    all_headers: bool,
    count: u32,
}

impl<'a> UpstreamForceKeyUnitEventBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            running_time: gst::ClockTime::NONE,
            all_headers: true,
            count: 0,
        }
    }

    pub fn running_time(self, running_time: impl Into<Option<gst::ClockTime>>) -> Self {
        Self {
            running_time: running_time.into(),
            ..self
        }
    }

    pub fn all_headers(self, all_headers: bool) -> Self {
        Self {
            all_headers,
            ..self
        }
    }

    pub fn count(self, count: u32) -> Self {
        Self { count, ..self }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        ffi::gst_video_event_new_upstream_force_key_unit(
            s.running_time.into_glib(),
            s.all_headers.into_glib(),
            s.count,
        )
    });
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpstreamForceKeyUnitEvent {
    pub running_time: Option<gst::ClockTime>,
    pub all_headers: bool,
    pub count: u32,
}

impl UpstreamForceKeyUnitEvent {
    pub fn builder<'a>() -> UpstreamForceKeyUnitEventBuilder<'a> {
        UpstreamForceKeyUnitEventBuilder::new()
    }

    #[doc(alias = "gst_video_event_parse_upstream_force_key_unit")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut running_time = mem::MaybeUninit::uninit();
            let mut all_headers = mem::MaybeUninit::uninit();
            let mut count = mem::MaybeUninit::uninit();

            let res: bool = from_glib(ffi::gst_video_event_parse_upstream_force_key_unit(
                event.as_mut_ptr(),
                running_time.as_mut_ptr(),
                all_headers.as_mut_ptr(),
                count.as_mut_ptr(),
            ));
            if res {
                Ok(Self {
                    running_time: from_glib(running_time.assume_init()),
                    all_headers: from_glib(all_headers.assume_init()),
                    count: count.assume_init(),
                })
            } else {
                Err(glib::bool_error!("Failed to parse GstEvent"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ForceKeyUnitEvent {
    Downstream(DownstreamForceKeyUnitEvent),
    Upstream(UpstreamForceKeyUnitEvent),
}

impl ForceKeyUnitEvent {
    #[doc(alias = "gst_video_event_is_force_key_unit")]
    pub fn is(event: &gst::EventRef) -> bool {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gst_video_event_is_force_key_unit(event.as_mut_ptr())) }
    }

    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        if event.is_upstream() {
            UpstreamForceKeyUnitEvent::parse(event).map(Self::Upstream)
        } else {
            DownstreamForceKeyUnitEvent::parse(event).map(Self::Downstream)
        }
    }
}

#[must_use = "The builder must be built to be used"]
pub struct StillFrameEventBuilder<'a> {
    seqnum: Option<gst::Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a (dyn ToSendValue + Sync))>,
    in_still: bool,
}

impl<'a> StillFrameEventBuilder<'a> {
    fn new(in_still: bool) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            in_still,
        }
    }

    event_builder_generic_impl!(|s: &mut Self| ffi::gst_video_event_new_still_frame(
        s.in_still.into_glib()
    ));
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StillFrameEvent {
    pub in_still: bool,
}

impl StillFrameEvent {
    pub fn builder<'a>(in_still: bool) -> StillFrameEventBuilder<'a> {
        assert_initialized_main_thread!();
        StillFrameEventBuilder::new(in_still)
    }

    #[doc(alias = "gst_video_event_parse_still_frame")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut in_still = mem::MaybeUninit::uninit();

            let res: bool = from_glib(ffi::gst_video_event_parse_still_frame(
                event.as_mut_ptr(),
                in_still.as_mut_ptr(),
            ));
            if res {
                Ok(Self {
                    in_still: from_glib(in_still.assume_init()),
                })
            } else {
                Err(glib::bool_error!("Invalid still-frame event"))
            }
        }
    }
}

const NAVIGATION_EVENT_NAME: &str = "application/x-gst-navigation";
#[cfg_attr(feature = "ser_de", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ser_de", serde(tag = "event"))]
#[derive(Clone, PartialEq, Debug)]
pub enum NavigationEvent {
    KeyPress {
        key: String,
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        modifier_state: NavigationModifierType,
    },
    KeyRelease {
        key: String,
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        modifier_state: NavigationModifierType,
    },
    MouseMove {
        x: f64,
        y: f64,
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        modifier_state: NavigationModifierType,
    },
    MouseButtonPress {
        button: i32,
        x: f64,
        y: f64,
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        modifier_state: NavigationModifierType,
    },
    MouseButtonRelease {
        button: i32,
        x: f64,
        y: f64,
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        modifier_state: NavigationModifierType,
    },
    Command {
        command: NavigationCommand,
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        modifier_state: NavigationModifierType,
    },
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    MouseScroll {
        x: f64,
        y: f64,
        delta_x: f64,
        delta_y: f64,
        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        modifier_state: NavigationModifierType,
    },
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    TouchDown {
        identifier: u32,
        x: f64,
        y: f64,
        pressure: f64,
        modifier_state: NavigationModifierType,
    },
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    TouchMotion {
        identifier: u32,
        x: f64,
        y: f64,
        pressure: f64,
        modifier_state: NavigationModifierType,
    },
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    TouchUp {
        identifier: u32,
        x: f64,
        y: f64,
        modifier_state: NavigationModifierType,
    },
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    TouchFrame {
        modifier_state: NavigationModifierType,
    },
    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    TouchCancel {
        modifier_state: NavigationModifierType,
    },
}

impl NavigationEvent {
    #[doc(alias = "gst_navigation_event_new_key_press")]
    pub fn new_key_press(key: &str) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::KeyPress {
            key: key.to_string(),
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[doc(alias = "gst_navigation_event_new_key_release")]
    pub fn new_key_release(key: &str) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::KeyRelease {
            key: key.to_string(),
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[doc(alias = "gst_navigation_event_new_mouse_move")]
    pub fn new_mouse_move(x: f64, y: f64) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::MouseMove {
            x,
            y,
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[doc(alias = "gst_navigation_event_new_mouse_button_press")]
    pub fn new_mouse_button_press(button: i32, x: f64, y: f64) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::MouseButtonPress {
            button,
            x,
            y,
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[doc(alias = "gst_navigation_event_new_mouse_button_release")]
    pub fn new_mouse_button_release(button: i32, x: f64, y: f64) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::MouseButtonRelease {
            button,
            x,
            y,
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_navigation_event_new_mouse_scroll")]
    pub fn new_mouse_scroll(x: f64, y: f64, delta_x: f64, delta_y: f64) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::MouseScroll {
            x,
            y,
            delta_x,
            delta_y,
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[doc(alias = "gst_navigation_event_new_command")]
    pub fn new_command(command: NavigationCommand) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::Command {
            command,
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_navigation_event_new_touch_down")]
    pub fn new_touch_down(identifier: u32, x: f64, y: f64, pressure: f64) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::TouchDown {
            identifier,
            x,
            y,
            pressure,
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_navigation_event_new_touch_motion")]
    pub fn new_touch_motion(identifier: u32, x: f64, y: f64, pressure: f64) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::TouchMotion {
            identifier,
            x,
            y,
            pressure,
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_navigation_event_new_touch_up")]
    pub fn new_touch_up(identifier: u32, x: f64, y: f64) -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::TouchUp {
            identifier,
            x,
            y,
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_navigation_event_new_touch_frame")]
    pub fn new_touch_frame() -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::TouchFrame {
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_navigation_event_new_touch_cancel")]
    pub fn new_touch_cancel() -> NavigationEvent {
        assert_initialized_main_thread!();
        Self::TouchCancel {
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            modifier_state: NavigationModifierType::empty(),
        }
    }

    #[doc(alias = "gst_navigation_event_get_type")]
    pub fn type_(event: &gst::EventRef) -> NavigationEventType {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_navigation_event_get_type(event.as_mut_ptr())) }
    }

    #[doc(alias = "gst_navigation_event_parse_key_event")]
    #[doc(alias = "gst_navigation_event_parse_mouse_button_event")]
    #[doc(alias = "gst_navigation_event_parse_mouse_scroll_event")]
    #[doc(alias = "gst_navigation_event_parse_mouse_move_event")]
    #[doc(alias = "gst_navigation_event_parse_touch_event")]
    #[doc(alias = "gst_navigation_event_parse_touch_up_event")]
    #[doc(alias = "gst_navigation_event_parse_command")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        if event.type_() != EventType::Navigation {
            return Err(glib::bool_error!("Invalid navigation event"));
        }

        let structure = event
            .structure()
            .ok_or_else(|| glib::bool_error!("Invalid navigation event"))?;
        if structure.name() != NAVIGATION_EVENT_NAME {
            return Err(glib::bool_error!("Invalid navigation event"));
        }

        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        let modifier_state = structure
            .get("state")
            .unwrap_or(NavigationModifierType::empty());
        let event = match Self::type_(event) {
            NavigationEventType::MouseMove => NavigationEvent::MouseMove {
                x: structure
                    .get("pointer_x")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                y: structure
                    .get("pointer_y")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                #[cfg(any(feature = "v1_22", feature = "dox"))]
                #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
                modifier_state,
            },
            NavigationEventType::MouseButtonPress => NavigationEvent::MouseButtonPress {
                button: structure
                    .get("button")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                x: structure
                    .get("pointer_x")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                y: structure
                    .get("pointer_y")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                #[cfg(any(feature = "v1_22", feature = "dox"))]
                #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
                modifier_state,
            },
            NavigationEventType::MouseButtonRelease => NavigationEvent::MouseButtonRelease {
                button: structure
                    .get("button")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                x: structure
                    .get("pointer_x")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                y: structure
                    .get("pointer_y")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                #[cfg(any(feature = "v1_22", feature = "dox"))]
                #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
                modifier_state,
            },
            #[cfg(any(feature = "v1_18", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
            NavigationEventType::MouseScroll => NavigationEvent::MouseScroll {
                x: structure
                    .get("pointer_x")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                y: structure
                    .get("pointer_y")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                delta_x: structure
                    .get("delta_pointer_x")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                delta_y: structure
                    .get("delta_pointer_y")
                    .map_err(|_| glib::bool_error!("Invalid mouse event"))?,
                #[cfg(any(feature = "v1_22", feature = "dox"))]
                #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
                modifier_state,
            },
            NavigationEventType::KeyPress => NavigationEvent::KeyPress {
                key: structure
                    .get("key")
                    .map_err(|_| glib::bool_error!("Invalid key press event"))?,
                #[cfg(any(feature = "v1_22", feature = "dox"))]
                #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
                modifier_state,
            },
            NavigationEventType::KeyRelease => NavigationEvent::KeyRelease {
                key: structure
                    .get("key")
                    .map_err(|_| glib::bool_error!("Invalid key press event"))?,
                #[cfg(any(feature = "v1_22", feature = "dox"))]
                #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
                modifier_state,
            },
            NavigationEventType::Command => NavigationEvent::Command {
                command: structure
                    .get("command-code")
                    .map_err(|_| glib::bool_error!("Invalid key press event"))?,
                #[cfg(any(feature = "v1_22", feature = "dox"))]
                #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
                modifier_state,
            },
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            NavigationEventType::TouchDown => NavigationEvent::TouchDown {
                identifier: structure
                    .get("identifier")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                x: structure
                    .get("pointer_x")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                y: structure
                    .get("pointer_y")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                pressure: structure
                    .get("pressure")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                modifier_state,
            },
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            NavigationEventType::TouchMotion => NavigationEvent::TouchMotion {
                identifier: structure
                    .get("identifier")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                x: structure
                    .get("pointer_x")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                y: structure
                    .get("pointer_y")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                pressure: structure
                    .get("pressure")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                modifier_state,
            },
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            NavigationEventType::TouchUp => NavigationEvent::TouchUp {
                identifier: structure
                    .get("identifier")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                x: structure
                    .get("pointer_x")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                y: structure
                    .get("pointer_y")
                    .map_err(|_| glib::bool_error!("Invalid touch event"))?,
                modifier_state,
            },
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            NavigationEventType::TouchFrame => NavigationEvent::TouchFrame { modifier_state },
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            NavigationEventType::TouchCancel => NavigationEvent::TouchCancel { modifier_state },
            NavigationEventType::Invalid | NavigationEventType::__Unknown(_) => {
                return Err(glib::bool_error!("Invalid navigation event"))
            }
        };
        Ok(event)
    }

    pub fn structure(&self) -> gst::Structure {
        skip_assert_initialized!();
        #[allow(unused_mut)]
        let mut structure = match self {
            Self::MouseMove { x, y, .. } => gst::Structure::builder(NAVIGATION_EVENT_NAME)
                .field("event", "mouse-move")
                .field("pointer_x", x)
                .field("pointer_y", y),
            Self::MouseButtonPress { button, x, y, .. } => {
                gst::Structure::builder(NAVIGATION_EVENT_NAME)
                    .field("event", "mouse-button-press")
                    .field("button", button)
                    .field("pointer_x", x)
                    .field("pointer_y", y)
            }
            Self::MouseButtonRelease { button, x, y, .. } => {
                gst::Structure::builder(NAVIGATION_EVENT_NAME)
                    .field("event", "mouse-button-release")
                    .field("button", button)
                    .field("pointer_x", x)
                    .field("pointer_y", y)
            }
            #[cfg(any(feature = "v1_18", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
            Self::MouseScroll {
                x,
                y,
                delta_x,
                delta_y,
                ..
            } => gst::Structure::builder(NAVIGATION_EVENT_NAME)
                .field("event", "mouse-scroll")
                .field("pointer_x", x)
                .field("pointer_y", y)
                .field("delta_pointer_x", delta_x)
                .field("delta_pointer_y", delta_y),
            Self::KeyPress { key, .. } => gst::Structure::builder(NAVIGATION_EVENT_NAME)
                .field("event", "key-press")
                .field("key", key),
            Self::KeyRelease { key, .. } => gst::Structure::builder(NAVIGATION_EVENT_NAME)
                .field("event", "key-release")
                .field("key", key),
            Self::Command { command, .. } => gst::Structure::builder(NAVIGATION_EVENT_NAME)
                .field("event", "command")
                .field("command-code", command),
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            Self::TouchDown {
                identifier,
                x,
                y,
                pressure,
                ..
            } => gst::Structure::builder(NAVIGATION_EVENT_NAME)
                .field("event", "touch-down")
                .field("identifier", identifier)
                .field("pointer_x", x)
                .field("pointer_y", y)
                .field("pressure", pressure),
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            Self::TouchMotion {
                identifier,
                x,
                y,
                pressure,
                ..
            } => gst::Structure::builder(NAVIGATION_EVENT_NAME)
                .field("event", "touch-motion")
                .field("identifier", identifier)
                .field("pointer_x", x)
                .field("pointer_y", y)
                .field("pressure", pressure),
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            Self::TouchUp {
                identifier, x, y, ..
            } => gst::Structure::builder(NAVIGATION_EVENT_NAME)
                .field("event", "touch-up")
                .field("identifier", identifier)
                .field("pointer_x", x)
                .field("pointer_y", y),
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            Self::TouchFrame { .. } => {
                gst::Structure::builder(NAVIGATION_EVENT_NAME).field("event", "touch-frame")
            }
            #[cfg(any(feature = "v1_22", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
            Self::TouchCancel { .. } => {
                gst::Structure::builder(NAVIGATION_EVENT_NAME).field("event", "touch-cancel")
            }
        };

        #[cfg(any(feature = "v1_22", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
        if true {
            structure = match self {
                Self::MouseMove { modifier_state, .. } => structure.field("state", modifier_state),
                Self::MouseButtonPress { modifier_state, .. } => {
                    structure.field("state", modifier_state)
                }
                Self::MouseButtonRelease { modifier_state, .. } => {
                    structure.field("state", modifier_state)
                }
                Self::MouseScroll { modifier_state, .. } => {
                    structure.field("state", modifier_state)
                }
                Self::KeyPress { modifier_state, .. } => structure.field("state", modifier_state),
                Self::KeyRelease { modifier_state, .. } => structure.field("state", modifier_state),
                Self::Command { modifier_state, .. } => structure.field("state", modifier_state),
                Self::TouchDown { modifier_state, .. } => structure.field("state", modifier_state),
                Self::TouchMotion { modifier_state, .. } => {
                    structure.field("state", modifier_state)
                }
                Self::TouchUp { modifier_state, .. } => structure.field("state", modifier_state),
                Self::TouchFrame { modifier_state, .. } => structure.field("state", modifier_state),
                Self::TouchCancel { modifier_state, .. } => {
                    structure.field("state", modifier_state)
                }
            };
        }

        structure.build()
    }

    pub fn build(&self) -> gst::Event {
        skip_assert_initialized!();

        gst::event::Navigation::new(self.structure())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "ser_de")]
    fn serialize_navigation_events() {
        use crate::NavigationEvent;

        gst::init().unwrap();

        let ev = NavigationEvent::new_mouse_scroll(1.0, 2.0, 3.0, 4.0).build();
        let navigation_event = NavigationEvent::parse(&ev).unwrap();
        match &navigation_event {
            NavigationEvent::MouseScroll {
                x,
                y,
                delta_x,
                delta_y,
            } => {
                assert!(*x == 1.0 && *y == 2.0 && *delta_x == 3.0 && *delta_y == 4.0);
            }
            _ => unreachable!(),
        }

        let json_event = serde_json::to_string(&navigation_event).unwrap();
        assert_eq!(
            json_event,
            r#"{"event":"MouseScroll","x":1.0,"y":2.0,"delta_x":3.0,"delta_y":4.0}"#
        );
        let navigation_event: NavigationEvent = serde_json::from_str(&json_event).unwrap();
        match &navigation_event {
            NavigationEvent::MouseScroll {
                x,
                y,
                delta_x,
                delta_y,
            } => {
                assert!(*x == 1.0 && *y == 2.0 && *delta_x == 3.0 && *delta_y == 4.0);
            }
            _ => unreachable!(),
        }

        let ev = NavigationEvent::new_mouse_button_press(1, 1.0, 2.0).build();
        let navigation_event = NavigationEvent::parse(&ev).unwrap();
        match &navigation_event {
            NavigationEvent::MouseButtonPress { button, x, y } => {
                assert!(*button == 1 && *x == 1.0 && *y == 2.0);
            }
            _ => unreachable!(),
        }

        let json_event = serde_json::to_string(&navigation_event).unwrap();
        assert_eq!(
            json_event,
            r#"{"event":"MouseButtonPress","button":1,"x":1.0,"y":2.0}"#
        );

        let ev = NavigationEvent::new_key_release("a").build();
        let navigation_event = NavigationEvent::parse(&ev).unwrap();
        match &navigation_event {
            NavigationEvent::KeyRelease { key } => {
                assert_eq!(key, "a");
            }
            _ => unreachable!(),
        }

        let json_event = serde_json::to_string(&navigation_event).unwrap();
        assert_eq!(json_event, r#"{"event":"KeyRelease","key":"a"}"#);

        let ev = NavigationEvent::new_touch_motion(0, 1.0, 2.0, 0.5).build();
        let navigation_event = NavigationEvent::parse(&ev).unwrap();
        match &navigation_event {
            NavigationEvent::TouchMotion { id, x, y, pressure } => {
                assert!(*id == 0 && *x = 1.0 && *y == 2.0 && *pressure == 0.5);
            }
            _ => unreachable!(),
        }

        let json_event = serde_json::to_string(&navigation_event).unwrap();
        assert_eq!(
            json_event,
            r#"{"event":"TouchMotion","identifier":0,"x":1.0,"y":2.0,"pressure":0.5}"#
        );

        let ev = NavigationEvent::new_touch_cancel().build();
        let navigation_event = NavigationEvent::parse(&ev).unwrap();
        assert!(matches!(&navigation_event, NavigationEvent::TouchCancel {}));

        let json_event = serde_json::to_string(&navigation_event).unwrap();
        assert_eq!(json_event, r#"{"event":"TouchCancel"}"#);
    }
}
