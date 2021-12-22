// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{NavigationCommand, NavigationEventType};
use glib::translate::{from_glib, from_glib_none, from_glib_full, IntoGlib};
use glib::ToSendValue;
use std::{mem, ptr};

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

#[derive(Clone, PartialEq, Debug)]
pub struct KeyEvent {
    pub key: String,
}

impl KeyEvent {
    #[doc(alias = "gst_navigation_event_parse_key_event")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut key = ptr::null();
            let ret = from_glib(ffi::gst_navigation_event_parse_key_event(
                event.as_mut_ptr(),
                &mut key,
            ));

            if ret {
                Ok(Self {
                    key: from_glib_none(key),
                })
            } else {
                Err(glib::bool_error!("Invalid key event"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MouseButtonEvent {
    pub button: i32,
    pub x: f64,
    pub y: f64,
}

impl MouseButtonEvent {
    #[doc(alias = "gst_navigation_event_parse_mouse_button_event")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut button = mem::MaybeUninit::uninit();
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_navigation_event_parse_mouse_button_event(
                event.as_mut_ptr(),
                button.as_mut_ptr(),
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            let button = button.assume_init();
            let x = x.assume_init();
            let y = y.assume_init();
            if ret {
                Ok(Self { button, x, y })
            } else {
                Err(glib::bool_error!("Invalid mouse button event"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MouseMoveEvent {
    pub x: f64,
    pub y: f64,
}

impl MouseMoveEvent {
    #[doc(alias = "gst_navigation_event_parse_mouse_move_event")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_navigation_event_parse_mouse_move_event(
                event.as_mut_ptr(),
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            if ret {
                Ok(Self { x, y })
            } else {
                Err(glib::bool_error!("Invalid mouse move event"))
            }
        }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
#[derive(Clone, PartialEq, Debug)]
pub struct MouseScrollEvent {
    pub x: f64,
    pub y: f64,
    pub delta_x: f64,
    pub delta_y: f64,
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl MouseScrollEvent {
    #[doc(alias = "gst_navigation_event_parse_mouse_scroll_event")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_navigation_event_parse_mouse_scroll_event(
                event.as_mut_ptr(),
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                delta_x.as_mut_ptr(),
                delta_y.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            let delta_x = delta_x.assume_init();
            let delta_y = delta_y.assume_init();
            if ret {
                Ok(Self {
                    x,
                    y,
                    delta_x,
                    delta_y,
                })
            } else {
                Err(glib::bool_error!("Invalid mouse button event"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct CommandEvent {
    cmd: NavigationCommand,
}

impl CommandEvent {
    #[doc(alias = "gst_navigation_event_parse_command")]
    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut command = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_navigation_event_parse_command(
                event.as_mut_ptr(),
                command.as_mut_ptr(),
            ));
            let command = command.assume_init();
            if ret {
                Ok(Self {
                    cmd: from_glib(command),
                })
            } else {
                Err(glib::bool_error!("Invalid navigation command event"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NavigationEvent {
    KeyPress(KeyEvent),
    KeyRelease(KeyEvent),
    MouseMove(MouseMoveEvent),
    MouseButtonPress(MouseButtonEvent),
    MouseButtonRelease(MouseButtonEvent),
    Command(CommandEvent),
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    MouseScroll(MouseScrollEvent),
}

impl NavigationEvent {
    #[doc(alias = "gst_navigation_event_get_type")]
    pub fn type_(event: &gst::EventRef) -> NavigationEventType {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_navigation_event_get_type(event.as_mut_ptr())) }
    }

    pub fn parse(event: &gst::EventRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();

        let event_type: NavigationEventType = Self::type_(event);

        match event_type {
            NavigationEventType::MouseMove => MouseMoveEvent::parse(event).map(Self::MouseMove),
            NavigationEventType::KeyPress => KeyEvent::parse(event).map(Self::KeyPress),
            NavigationEventType::KeyRelease => KeyEvent::parse(event).map(Self::KeyRelease),
            #[cfg(any(feature = "v1_18", feature = "dox"))]
            #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
            NavigationEventType::MouseScroll => {
                MouseScrollEvent::parse(event).map(Self::MouseScroll)
            }
            NavigationEventType::MouseButtonPress => {
                MouseButtonEvent::parse(event).map(Self::MouseButtonPress)
            }
            NavigationEventType::MouseButtonRelease => {
                MouseButtonEvent::parse(event).map(Self::MouseButtonRelease)
            }
            NavigationEventType::Command => CommandEvent::parse(event).map(Self::Command),
            NavigationEventType::Invalid | NavigationEventType::__Unknown(_) => {
                return Err(glib::bool_error!("Invalid navigation event"))
            }
        }
    }
}
