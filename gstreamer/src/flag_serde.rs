// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    translate::{from_glib, ToGlibPtr},
    FlagsClass, StaticType, ToValue,
};

bitflags_serde_impl!(crate::BinFlags);
bitflags_serde_impl!(crate::BufferCopyFlags);
bitflags_serde_impl!(crate::BufferFlags);
bitflags_serde_impl!(crate::BufferPoolAcquireFlags);
bitflags_serde_impl!(crate::ClockFlags);

impl serde::Serialize for crate::DebugColorFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "{}+{}{}{}",
            match *self & Self::from_bits(0b111).expect("Failed to create value from fg-color mask")
            {
                Self::FG_BLACK => "fg-black",
                Self::FG_RED => "fg-red",
                Self::FG_GREEN => "fg-green",
                Self::FG_YELLOW => "fg-yellow",
                Self::FG_BLUE => "fg-blue",
                Self::FG_MAGENTA => "fg-magenta",
                Self::FG_CYAN => "fg-cyan",
                Self::FG_WHITE => "fg-white",
                _ => unreachable!(),
            },
            match *self
                & Self::from_bits(0b111_0000).expect("Failed to create value from bg-color mask")
            {
                Self::BG_BLACK => "bg-black",
                Self::BG_RED => "bg-red",
                Self::BG_GREEN => "bg-green",
                Self::BG_YELLOW => "bg-yellow",
                Self::BG_BLUE => "bg-blue",
                Self::BG_MAGENTA => "bg-magenta",
                Self::BG_CYAN => "bg-cyan",
                Self::BG_WHITE => "bg-white",
                _ => unreachable!(),
            },
            if self.contains(Self::BOLD) {
                "+bold"
            } else {
                ""
            },
            if self.contains(Self::UNDERLINE) {
                "+underline"
            } else {
                ""
            }
        ))
    }
}

bitflags_deserialize_impl!(crate::DebugColorFlags);
bitflags_serialize_impl!(crate::DebugGraphDetails, by_ones_decreasing);
bitflags_deserialize_impl!(crate::DebugGraphDetails);
bitflags_serde_impl!(crate::ElementFlags);
bitflags_serde_impl!(crate::EventTypeFlags);
bitflags_serde_impl!(crate::GapFlags, "v1_20");
bitflags_serde_impl!(crate::MemoryFlags);
bitflags_serde_impl!(crate::MetaFlags);
bitflags_serde_impl!(crate::ObjectFlags);
bitflags_serde_impl!(crate::PadFlags);
bitflags_serde_impl!(crate::PadLinkCheck);
bitflags_serde_impl!(crate::PadProbeType);
bitflags_serde_impl!(crate::ParseFlags);
bitflags_serde_impl!(crate::PluginAPIFlags, "v1_18");
bitflags_serde_impl!(crate::PluginDependencyFlags);
bitflags_serde_impl!(crate::PluginFlags);
bitflags_serde_impl!(crate::SchedulingFlags);
bitflags_serde_impl!(crate::SeekFlags);
bitflags_serde_impl!(crate::SegmentFlags);
bitflags_serde_impl!(crate::SerializeFlags, "v1_20");
bitflags_serde_impl!(crate::StackTraceFlags);
bitflags_serde_impl!(crate::StreamFlags);
bitflags_serde_impl!(crate::StreamType);

#[cfg(test)]
mod tests {
    macro_rules! check_serialize {
        ($flags:expr, $expected:expr) => {
            let actual = serde_json::to_string(&$flags).unwrap();
            assert_eq!(actual, $expected);
        };
    }

    macro_rules! check_deserialize {
        ($ty:ty, $expected:expr, $json:expr) => {
            let actual: $ty = serde_json::from_str(&$json).unwrap();
            assert_eq!(actual, $expected);
        };
    }

    macro_rules! check_roundtrip {
        ($ty:ty, $flags:expr) => {
            let json = serde_json::to_string(&$flags).unwrap();
            let deserialized: $ty = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, $flags);
        };
    }

    #[test]
    fn test_serialize() {
        crate::init().unwrap();

        check_serialize!(crate::BinFlags::empty(), "\"\"");
        check_serialize!(crate::BinFlags::all(), "\"no-resync+streams-aware\"");
        check_serialize!(
            crate::BufferCopyFlags::all(),
            "\"flags+timestamps+meta+memory+merge+deep\""
        );
        check_serialize!(
            crate::BufferFlags::all(),
            concat!(
                "\"live+decode-only+discont+resync+corrupted+marker+header+gap",
                "+droppable+delta-unit+tag-memory+sync-after+non-droppable\""
            )
        );
        check_serialize!(
            crate::BufferPoolAcquireFlags::all(),
            "\"key-unit+dontwait+discont\""
        );
        check_serialize!(
            crate::ClockFlags::all(),
            concat!(
                "\"can-do-single-sync+can-do-single-async",
                "+can-do-periodic-sync+can-do-periodic-async",
                "+can-set-resolution+can-set-master+needs-startup-sync\""
            )
        );

        check_serialize!(
            crate::DebugColorFlags::all(),
            "\"fg-white+bg-white+bold+underline\""
        );
        check_serialize!(
            crate::DebugColorFlags::FG_MAGENTA | crate::DebugColorFlags::BOLD,
            "\"fg-magenta+bg-black+bold\""
        );
        check_serialize!(
            crate::DebugColorFlags::FG_RED
                | crate::DebugColorFlags::FG_BLUE
                | crate::DebugColorFlags::BG_BLACK,
            "\"fg-magenta+bg-black\""
        );

        check_serialize!(crate::DebugGraphDetails::all(), "\"verbose\"");
        check_serialize!(
            crate::DebugGraphDetails::MEDIA_TYPE
                | crate::DebugGraphDetails::CAPS_DETAILS
                | crate::DebugGraphDetails::NON_DEFAULT_PARAMS
                | crate::DebugGraphDetails::STATES
                | crate::DebugGraphDetails::FULL_PARAMS
                | crate::DebugGraphDetails::ALL,
            "\"all+full-params\""
        );
        check_serialize!(
            crate::DebugGraphDetails::MEDIA_TYPE
                | crate::DebugGraphDetails::CAPS_DETAILS
                | crate::DebugGraphDetails::NON_DEFAULT_PARAMS
                | crate::DebugGraphDetails::STATES
                | crate::DebugGraphDetails::FULL_PARAMS,
            "\"all+full-params\""
        );

        check_serialize!(
            crate::ElementFlags::all(),
            "\"locked-state+sink+source+provide-clock+require-clock+indexable\""
        );
        check_serialize!(
            crate::EventTypeFlags::all(),
            "\"upstream+downstream+serialized+sticky+sticky-multi\""
        );
        #[cfg(feature = "v1_20")]
        check_serialize!(crate::GapFlags::all(), "\"data\"");
        check_serialize!(
            crate::MemoryFlags::all(),
            concat!(
                "\"readonly+no-share+zero-prefixed+zero-padded",
                "+physically-contiguous+not-mappable\""
            )
        );
        check_serialize!(crate::MetaFlags::all(), "\"readonly+pooled+locked\"");
        check_serialize!(crate::ObjectFlags::all(), "\"may-be-leaked\"");
        check_serialize!(
            crate::PadFlags::all(),
            concat!(
                "\"blocked+flushing+eos+blocking+need-parent+need-reconfigure",
                "+pending-events+fixed-caps+proxy-caps+proxy-allocation",
                "+proxy-scheduling+accept-intersect+accept-template\""
            )
        );
        check_serialize!(
            crate::PadLinkCheck::all(),
            "\"hierarchy+template-caps+caps+no-reconfigure\""
        );
        check_serialize!(
            crate::PadProbeType::all(),
            concat!(
                "\"idle+block+buffer+buffer-list+event-downstream",
                "+event-upstream+event-flush+query-downstream+query-upstream",
                "+push+pull\""
            )
        );
        check_serialize!(
            crate::ParseFlags::all(),
            "\"fatal-errors+no-single-element-bins+place-in-bin\""
        );
        #[cfg(feature = "v1_18")]
        check_serialize!(crate::PluginAPIFlags::all(), "\"members\"");
        check_serialize!(
            crate::PluginDependencyFlags::all(),
            concat!(
                "\"recurse+paths-are-default-only+file-name-is-suffix",
                "+file-name-is-prefix+paths-are-relative-to-exe\""
            )
        );
        check_serialize!(crate::PluginFlags::all(), "\"cached+blacklisted\"");
        check_serialize!(
            crate::SchedulingFlags::all(),
            "\"seekable+sequential+bandwidth-limited\""
        );
        #[cfg(feature = "v1_18")]
        check_serialize!(
            crate::SeekFlags::all(),
            concat!(
                "\"flush+accurate+key-unit+segment+trickmode+snap-before",
                "+snap-after+trickmode-key-units+trickmode-no-audio",
                "+trickmode-forward-predicted+instant-rate-change\""
            )
        );
        #[cfg(feature = "v1_18")]
        check_serialize!(
            crate::SegmentFlags::all(),
            concat!(
                "\"reset+trickmode+segment+trickmode-key-units",
                "+trickmode-forward-predicted+trickmode-no-audio\""
            )
        );
        #[cfg(feature = "v1_20")]
        check_serialize!(crate::SerializeFlags::all(), "\"backward-compat\"");
        check_serialize!(crate::StackTraceFlags::all(), "\"full\"");
        check_serialize!(crate::StreamFlags::all(), "\"sparse+select+unselect\"");
        check_serialize!(
            crate::StreamType::all(),
            "\"unknown+audio+video+container+text\""
        );
    }

    #[test]
    fn test_deserialize() {
        crate::init().unwrap();

        check_deserialize!(crate::BinFlags, crate::BinFlags::empty(), "\"\"");
        check_deserialize!(
            crate::BinFlags,
            crate::BinFlags::all(),
            "\"no-resync+streams-aware\""
        );
        check_deserialize!(
            crate::BufferCopyFlags,
            crate::BufferCopyFlags::all(),
            "\"flags+timestamps+meta+memory+merge+deep\""
        );
        check_deserialize!(
            crate::BufferFlags,
            crate::BufferFlags::all(),
            concat!(
                "\"live+decode-only+discont+resync+corrupted+marker+header+gap",
                "+droppable+delta-unit+tag-memory+sync-after+non-droppable\""
            )
        );
        check_deserialize!(
            crate::BufferPoolAcquireFlags,
            crate::BufferPoolAcquireFlags::all(),
            "\"key-unit+dontwait+discont\""
        );
        check_deserialize!(
            crate::ClockFlags,
            crate::ClockFlags::all(),
            concat!(
                "\"can-do-single-sync+can-do-single-async",
                "+can-do-periodic-sync+can-do-periodic-async",
                "+can-set-resolution+can-set-master+needs-startup-sync\""
            )
        );

        check_deserialize!(
            crate::DebugColorFlags,
            crate::DebugColorFlags::all(),
            "\"fg-white+bg-white+bold+underline\""
        );
        check_deserialize!(
            crate::DebugColorFlags,
            crate::DebugColorFlags::FG_MAGENTA | crate::DebugColorFlags::BOLD,
            "\"fg-magenta+bg-black+bold\""
        );
        check_deserialize!(
            crate::DebugColorFlags,
            crate::DebugColorFlags::FG_RED
                | crate::DebugColorFlags::FG_BLUE
                | crate::DebugColorFlags::BG_BLACK,
            "\"fg-magenta+bg-black\""
        );

        check_deserialize!(
            crate::DebugGraphDetails,
            crate::DebugGraphDetails::all(),
            "\"verbose\""
        );
        check_deserialize!(
            crate::DebugGraphDetails,
            crate::DebugGraphDetails::MEDIA_TYPE
                | crate::DebugGraphDetails::CAPS_DETAILS
                | crate::DebugGraphDetails::NON_DEFAULT_PARAMS
                | crate::DebugGraphDetails::STATES
                | crate::DebugGraphDetails::FULL_PARAMS
                | crate::DebugGraphDetails::ALL,
            "\"all+full-params\""
        );
        check_deserialize!(
            crate::DebugGraphDetails,
            crate::DebugGraphDetails::MEDIA_TYPE
                | crate::DebugGraphDetails::CAPS_DETAILS
                | crate::DebugGraphDetails::NON_DEFAULT_PARAMS
                | crate::DebugGraphDetails::STATES
                | crate::DebugGraphDetails::FULL_PARAMS,
            "\"all+full-params\""
        );

        check_deserialize!(
            crate::ElementFlags,
            crate::ElementFlags::all(),
            "\"locked-state+sink+source+provide-clock+require-clock+indexable\""
        );
        check_deserialize!(
            crate::EventTypeFlags,
            crate::EventTypeFlags::all(),
            "\"upstream+downstream+serialized+sticky+sticky-multi\""
        );
        #[cfg(feature = "v1_20")]
        check_deserialize!(crate::GapFlags, crate::GapFlags::all(), "\"data\"");
        check_deserialize!(
            crate::MemoryFlags,
            crate::MemoryFlags::all(),
            concat!(
                "\"readonly+no-share+zero-prefixed+zero-padded",
                "+physically-contiguous+not-mappable\""
            )
        );
        check_deserialize!(
            crate::MetaFlags,
            crate::MetaFlags::all(),
            "\"readonly+pooled+locked\""
        );
        check_deserialize!(
            crate::ObjectFlags,
            crate::ObjectFlags::all(),
            "\"may-be-leaked\""
        );
        check_deserialize!(
            crate::PadFlags,
            crate::PadFlags::all(),
            concat!(
                "\"blocked+flushing+eos+blocking+need-parent+need-reconfigure",
                "+pending-events+fixed-caps+proxy-caps+proxy-allocation",
                "+proxy-scheduling+accept-intersect+accept-template\""
            )
        );
        check_deserialize!(
            crate::PadLinkCheck,
            crate::PadLinkCheck::all(),
            "\"hierarchy+template-caps+caps+no-reconfigure\""
        );
        check_deserialize!(
            crate::PadProbeType,
            crate::PadProbeType::all(),
            concat!(
                "\"idle+block+buffer+buffer-list+event-downstream",
                "+event-upstream+event-flush+query-downstream+query-upstream",
                "+push+pull\""
            )
        );
        check_deserialize!(
            crate::ParseFlags,
            crate::ParseFlags::all(),
            "\"fatal-errors+no-single-element-bins+place-in-bin\""
        );
        #[cfg(feature = "v1_18")]
        check_deserialize!(
            crate::PluginAPIFlags,
            crate::PluginAPIFlags::all(),
            "\"members\""
        );
        check_deserialize!(
            crate::PluginDependencyFlags,
            crate::PluginDependencyFlags::all(),
            concat!(
                "\"recurse+paths-are-default-only+file-name-is-suffix",
                "+file-name-is-prefix+paths-are-relative-to-exe\""
            )
        );
        check_deserialize!(
            crate::PluginFlags,
            crate::PluginFlags::all(),
            "\"cached+blacklisted\""
        );
        check_deserialize!(
            crate::SchedulingFlags,
            crate::SchedulingFlags::all(),
            "\"seekable+sequential+bandwidth-limited\""
        );
        #[cfg(feature = "v1_18")]
        check_deserialize!(
            crate::SeekFlags,
            crate::SeekFlags::all(),
            concat!(
                "\"flush+accurate+key-unit+segment+trickmode+snap-before",
                "+snap-after+trickmode-key-units+trickmode-no-audio",
                "+trickmode-forward-predicted+instant-rate-change\""
            )
        );
        #[cfg(feature = "v1_18")]
        check_deserialize!(
            crate::SegmentFlags,
            crate::SegmentFlags::all(),
            concat!(
                "\"reset+trickmode+segment+trickmode-key-units",
                "+trickmode-forward-predicted+trickmode-no-audio\""
            )
        );
        #[cfg(feature = "v1_20")]
        check_deserialize!(
            crate::SerializeFlags,
            crate::SerializeFlags::all(),
            "\"backward-compat\""
        );
        check_deserialize!(
            crate::StackTraceFlags,
            crate::StackTraceFlags::all(),
            "\"full\""
        );
        check_deserialize!(
            crate::StreamFlags,
            crate::StreamFlags::all(),
            "\"sparse+select+unselect\""
        );
        check_deserialize!(
            crate::StreamType,
            crate::StreamType::all(),
            "\"unknown+audio+video+container+text\""
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        crate::init().unwrap();

        check_roundtrip!(crate::BinFlags, crate::BinFlags::empty());
        check_roundtrip!(crate::BinFlags, crate::BinFlags::all());
        check_roundtrip!(crate::BufferCopyFlags, crate::BufferCopyFlags::all());
        check_roundtrip!(crate::BufferFlags, crate::BufferFlags::all());
        check_roundtrip!(
            crate::BufferPoolAcquireFlags,
            crate::BufferPoolAcquireFlags::all()
        );
        check_roundtrip!(crate::ClockFlags, crate::ClockFlags::all());

        check_roundtrip!(crate::DebugColorFlags, crate::DebugColorFlags::all());
        check_roundtrip!(
            crate::DebugColorFlags,
            crate::DebugColorFlags::FG_MAGENTA | crate::DebugColorFlags::BOLD
        );
        check_roundtrip!(
            crate::DebugColorFlags,
            crate::DebugColorFlags::FG_RED
                | crate::DebugColorFlags::FG_BLUE
                | crate::DebugColorFlags::BG_BLACK
        );

        check_roundtrip!(crate::DebugGraphDetails, crate::DebugGraphDetails::all());
        check_roundtrip!(
            crate::DebugGraphDetails,
            crate::DebugGraphDetails::MEDIA_TYPE
                | crate::DebugGraphDetails::CAPS_DETAILS
                | crate::DebugGraphDetails::NON_DEFAULT_PARAMS
                | crate::DebugGraphDetails::STATES
                | crate::DebugGraphDetails::FULL_PARAMS
                | crate::DebugGraphDetails::ALL
        );
        check_roundtrip!(
            crate::DebugGraphDetails,
            crate::DebugGraphDetails::MEDIA_TYPE
                | crate::DebugGraphDetails::CAPS_DETAILS
                | crate::DebugGraphDetails::NON_DEFAULT_PARAMS
                | crate::DebugGraphDetails::STATES
                | crate::DebugGraphDetails::FULL_PARAMS
        );

        check_roundtrip!(crate::ElementFlags, crate::ElementFlags::all());
        check_roundtrip!(crate::EventTypeFlags, crate::EventTypeFlags::all());
        #[cfg(feature = "v1_20")]
        check_roundtrip!(crate::GapFlags, crate::GapFlags::all());
        check_roundtrip!(crate::MemoryFlags, crate::MemoryFlags::all());
        check_roundtrip!(crate::MetaFlags, crate::MetaFlags::all());
        check_roundtrip!(crate::ObjectFlags, crate::ObjectFlags::all());
        check_roundtrip!(crate::PadFlags, crate::PadFlags::all());
        check_roundtrip!(crate::PadLinkCheck, crate::PadLinkCheck::all());
        check_roundtrip!(crate::PadProbeType, crate::PadProbeType::all());
        check_roundtrip!(crate::ParseFlags, crate::ParseFlags::all());
        #[cfg(feature = "v1_18")]
        check_roundtrip!(crate::PluginAPIFlags, crate::PluginAPIFlags::all());
        check_roundtrip!(
            crate::PluginDependencyFlags,
            crate::PluginDependencyFlags::all()
        );
        check_roundtrip!(crate::PluginFlags, crate::PluginFlags::all());
        check_roundtrip!(crate::SchedulingFlags, crate::SchedulingFlags::all());
        #[cfg(feature = "v1_18")]
        check_roundtrip!(crate::SeekFlags, crate::SeekFlags::all());
        #[cfg(feature = "v1_18")]
        check_roundtrip!(crate::SegmentFlags, crate::SegmentFlags::all());
        #[cfg(feature = "v1_20")]
        check_roundtrip!(crate::SerializeFlags, crate::SerializeFlags::all());
        check_roundtrip!(crate::StackTraceFlags, crate::StackTraceFlags::all());
        check_roundtrip!(crate::StreamFlags, crate::StreamFlags::all());
        check_roundtrip!(crate::StreamType, crate::StreamType::all());
    }
}
