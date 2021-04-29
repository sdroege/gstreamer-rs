// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{from_glib, from_glib_full, IntoGlib};
use glib::ToSendValue;
use std::mem;

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

pub struct DownstreamForceKeyUnitEventBuilder<'a> {
    seqnum: Option<gst::Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a (dyn ToSendValue + Sync))>,
    timestamp: gst::ClockTime,
    stream_time: gst::ClockTime,
    running_time: gst::ClockTime,
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
            timestamp: gst::CLOCK_TIME_NONE,
            stream_time: gst::CLOCK_TIME_NONE,
            running_time: gst::CLOCK_TIME_NONE,
            all_headers: true,
            count: 0,
        }
    }

    pub fn timestamp(self, timestamp: gst::ClockTime) -> Self {
        Self { timestamp, ..self }
    }

    pub fn stream_time(self, stream_time: gst::ClockTime) -> Self {
        Self {
            stream_time,
            ..self
        }
    }

    pub fn running_time(self, running_time: gst::ClockTime) -> Self {
        Self {
            running_time,
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
    pub timestamp: gst::ClockTime,
    pub stream_time: gst::ClockTime,
    pub running_time: gst::ClockTime,
    pub all_headers: bool,
    pub count: u32,
}

impl DownstreamForceKeyUnitEvent {
    pub fn builder<'a>() -> DownstreamForceKeyUnitEventBuilder<'a> {
        DownstreamForceKeyUnitEventBuilder::new()
    }

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

pub struct UpstreamForceKeyUnitEventBuilder<'a> {
    seqnum: Option<gst::Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a (dyn ToSendValue + Sync))>,
    running_time: gst::ClockTime,
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
            running_time: gst::CLOCK_TIME_NONE,
            all_headers: true,
            count: 0,
        }
    }

    pub fn running_time(self, running_time: gst::ClockTime) -> Self {
        Self {
            running_time,
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
    pub running_time: gst::ClockTime,
    pub all_headers: bool,
    pub count: u32,
}

impl UpstreamForceKeyUnitEvent {
    pub fn builder<'a>() -> UpstreamForceKeyUnitEventBuilder<'a> {
        UpstreamForceKeyUnitEventBuilder::new()
    }

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
