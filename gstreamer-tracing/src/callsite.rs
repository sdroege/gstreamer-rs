use std::sync::LazyLock;
use std::{
    alloc::GlobalAlloc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        PoisonError,
    },
};
use tracing::{field::FieldSet, Level};
use tracing_core::{identify_callsite, Callsite, Interest, Kind, Metadata};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub(crate) enum GstCallsiteKind {
    Event = 0,
    Span = 1,
}

pub(crate) struct GstCallsite {
    interest: AtomicUsize,
    metadata: Metadata<'static>,
    pub(crate) unprefixed_target: &'static str,
}

const CALLSITE_INTEREST_NEVER: usize = 1;
const CALLSITE_INTEREST_SOMETIMES: usize = 2;
const CALLSITE_INTEREST_ALWAYS: usize = 3;

impl GstCallsite {
    fn make_static(key: &Key) -> &'static Self {
        let module = key.module.unwrap_or("");
        let file = key.file.unwrap_or("");
        let prefixed_target_len = crate::TARGET.len() + 2 + key.target.len();
        unsafe {
            // Super unsafe, nasty, ugly stuff here. All in order to enable just 1 allocation for
            // each callsite only. We draw inspiration from a pattern oft used in C, where a single
            // allocation is split into two parts, one to store a specific structure (`GstCallsite`
            // in our case) and the second one to store a dynamically sized tail of values, inline.
            // In this dynamic area we store various strings that `Metadata` then refers back to.
            let callsite_layout = std::alloc::Layout::new::<GstCallsite>();
            let string_length = module.len() + file.len() + prefixed_target_len;
            let string_layout =
                std::alloc::Layout::array::<u8>(string_length).expect("layout calculation");
            let (callsite_layout, string_offset) = callsite_layout
                .extend(string_layout)
                .expect("layout calculation");
            let alloc = std::alloc::System.alloc(callsite_layout);
            let string = std::slice::from_raw_parts_mut(alloc.add(string_offset), string_length);
            let callsite = alloc as *mut GstCallsite;

            let (module_alloc, rest) = string.split_at_mut(module.len());
            module_alloc.copy_from_slice(module.as_bytes());
            let (file_alloc, rest) = rest.split_at_mut(file.len());
            file_alloc.copy_from_slice(file.as_bytes());
            let (target_prefix_alloc, rest) = rest.split_at_mut(crate::TARGET.len());
            target_prefix_alloc.copy_from_slice(crate::TARGET.as_bytes());
            let (separator_alloc, rest) = rest.split_at_mut(2);
            separator_alloc.copy_from_slice("::".as_bytes());
            let (target_alloc, _) = rest.split_at_mut(key.target.len());
            target_alloc.copy_from_slice(key.target.as_bytes());
            let file_start = module.len();
            let prefixed_target_start = file_start + file.len();
            let unprefixed_target_start = prefixed_target_start + crate::TARGET.len() + 2;
            let string = std::str::from_utf8_unchecked(string);
            let fieldset = FieldSet::new(key.fields, identify_callsite!(&*callsite));
            callsite.write(GstCallsite {
                interest: AtomicUsize::new(0),
                unprefixed_target: &string[unprefixed_target_start..],
                metadata: Metadata::new(
                    key.name,
                    &string[prefixed_target_start..],
                    key.level,
                    key.file.map(|_| &string[file_start..prefixed_target_start]),
                    key.line,
                    key.module.map(|_| &string[..module.len()]),
                    fieldset,
                    match key.kind {
                        GstCallsiteKind::Span => Kind::SPAN,
                        GstCallsiteKind::Event => Kind::EVENT,
                    },
                ),
            });
            &*callsite
        }
    }

    pub(crate) fn interest(&self) -> Interest {
        match self.interest.load(Ordering::Acquire) {
            CALLSITE_INTEREST_NEVER => Interest::never(),
            CALLSITE_INTEREST_SOMETIMES => Interest::sometimes(),
            CALLSITE_INTEREST_ALWAYS => Interest::always(),
            _ => panic!("attempting to obtain callsite's interest before its been set"),
        }
    }
}

impl Callsite for GstCallsite {
    fn set_interest(&self, interest: Interest) {
        self.interest.store(
            match () {
                _ if interest.is_never() => CALLSITE_INTEREST_NEVER,
                _ if interest.is_always() => CALLSITE_INTEREST_ALWAYS,
                _ => CALLSITE_INTEREST_SOMETIMES,
            },
            Ordering::Release,
        );
    }

    fn metadata(&self) -> &Metadata<'_> {
        &self.metadata
    }
}

/// A map of metadata allocations we've made throughout the lifetime of the process.
///
/// [`tracing`] requires the metadata to have a lifetime of `'static` for them to be usable. This
/// is required for a number of reasons, one of which is performance of filtering the messages.
///
/// In order to facilitate this, we maintain a static map which allows us to allocate the necessary
/// data on the heap (and with the required `'static` lifetime we effectively leak)
pub(crate) struct DynamicCallsites {
    data: std::sync::Mutex<Map>,
}

type Map = std::collections::BTreeMap<Key<'static>, &'static GstCallsite>;

#[derive(Eq)]
struct Key<'a> {
    level: Level,
    line: Option<u32>,
    module: Option<&'a str>,
    file: Option<&'a str>,
    target: &'a str,
    name: &'static str,
    fields: &'static [&'static str],
    kind: GstCallsiteKind,
}

impl PartialEq for Key<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Key<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Make sure the fields that are most likely to be different, or those that are cheap to
        // compare, are compared first.
        self.line
            .cmp(&other.line)
            .then_with(move || self.level.cmp(&other.level))
            .then_with(move || self.module.cmp(&other.module))
            .then_with(move || self.file.cmp(&other.file))
            .then_with(move || self.name.cmp(other.name))
            .then_with(move || self.fields.cmp(other.fields))
            .then_with(move || self.kind.cmp(&other.kind))
            .then_with(move || self.target.cmp(other.target))
    }
}

impl DynamicCallsites {
    pub(crate) fn get() -> &'static Self {
        static MAP: LazyLock<DynamicCallsites> = LazyLock::new(|| DynamicCallsites {
            data: std::sync::Mutex::new(Map::new()),
        });

        &MAP
    }

    #[allow(clippy::too_many_arguments)] // This is internal to the crate, clippy.
    pub(crate) fn callsite_for(
        &'static self,
        level: Level,
        name: &'static str,
        target: &str,
        file: Option<&str>,
        module: Option<&str>,
        line: Option<u32>,
        kind: GstCallsiteKind,
        fields: &'static [&'static str],
    ) -> &'static GstCallsite {
        let mut guard = self.data.lock().unwrap_or_else(PoisonError::into_inner);
        let lookup_key = Key {
            level,
            name,
            target,
            file,
            module,
            line,
            kind,
            fields,
        };
        if let Some(callsite) = guard.get(&lookup_key) {
            return callsite;
        }
        let callsite = GstCallsite::make_static(&lookup_key);
        let metadata = callsite.metadata();
        let key = Key::<'static> {
            level,
            name,
            line,
            kind,
            fields,
            file: metadata.file(),
            module: metadata.module_path(),
            target: callsite.unprefixed_target,
        };
        tracing::debug!(message = "allocated a new callsite",
            current_callsites = guard.len(),
            name = lookup_key.name,
            target = target,
            kind = ?lookup_key.kind,
            file = ?lookup_key.file,
            line = ?lookup_key.line,
            module = ?lookup_key.module,
            fieldset = ?lookup_key.fields,
        );
        guard.insert(key, callsite);
        tracing_core::callsite::register(callsite);
        callsite
    }
}
