use once_cell::sync::OnceCell;
use std::sync::{
    atomic::{AtomicPtr, AtomicUsize, Ordering},
    PoisonError,
};
use tracing::{field::FieldSet, Level};
use tracing_core::{identify_callsite, Callsite, Interest, Kind, Metadata};

pub(crate) struct GstCallsite {
    interest: AtomicUsize,
    metadata: AtomicPtr<Metadata<'static>>,
}

impl GstCallsite {
    fn make_static() -> &'static Self {
        #[allow(unused_unsafe)]
        unsafe {
            // SAFETY: for `metadata` to be sound this must initialize with a null pointer.
            Box::leak(Box::new(GstCallsite {
                interest: AtomicUsize::new(0),
                metadata: AtomicPtr::new(std::ptr::null_mut()),
            }))
        }
    }
    fn set_metadata(&'static self, meta: &'static Metadata<'static>) {
        self.metadata
            .compare_exchange(
                std::ptr::null_mut(),
                meta as *const _ as _,
                Ordering::Release,
                Ordering::Relaxed,
            )
            .expect("set_metadata should only be called once");
        tracing_core::callsite::register(self);
    }
    pub(crate) fn interest(&self) -> Interest {
        match self.interest.load(Ordering::Acquire) {
            1 => Interest::never(),
            2 => Interest::sometimes(),
            3 => Interest::always(),
            _ => panic!("attempting to obtain callsite's interest before its been set"),
        }
    }
}

impl Callsite for GstCallsite {
    fn set_interest(&self, interest: Interest) {
        self.interest.store(
            match () {
                _ if interest.is_never() => 1,
                _ if interest.is_always() => 3,
                _ => 2,
            },
            Ordering::Release,
        );
    }

    fn metadata(&self) -> &Metadata<'_> {
        unsafe {
            // SAFETY: this type always contains nullptr (and `as_ref` will return `None`) until it
            // is initialized in the `set_metadata` function. `set_metadata` will always set a
            // valid pointer by definition of storing the address of a `&'static` reference.
            self.metadata
                .load(Ordering::Acquire)
                .as_ref()
                .expect("metadata must have been initialized already!")
        }
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

#[derive(PartialEq, Eq)]
struct Key<'a> {
    level: Level,
    line: Option<u32>,
    module: Option<&'a str>,
    file: Option<&'a str>,
    target: &'a str,
    name: &'static str,
    fields: &'static [&'static str],
    kind: Kind,
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
            .then_with(move || match (&self.kind, &other.kind) {
                (&Kind::EVENT, &Kind::EVENT) | (&Kind::SPAN, &Kind::SPAN) => {
                    std::cmp::Ordering::Equal
                }
                (&Kind::EVENT, &Kind::SPAN) => std::cmp::Ordering::Less,
                (&Kind::SPAN, &Kind::EVENT) => std::cmp::Ordering::Greater,
            })
            .then_with(move || self.target.cmp(other.target))
    }
}

impl<'a> Key<'a> {
    fn leak(&self) -> Key<'static> {
        Key::<'static> {
            level: self.level,
            line: self.line,
            kind: self.kind.clone(),
            fields: self.fields,
            module: self.module.map(leak_str),
            file: self.file.map(leak_str),
            name: self.name,
            target: leak_str(self.target),
        }
    }
}

fn leak_str(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

impl DynamicCallsites {
    pub(crate) fn get() -> &'static Self {
        static MAP: OnceCell<DynamicCallsites> = OnceCell::new();
        MAP.get_or_init(|| DynamicCallsites {
            data: std::sync::Mutex::new(Map::new()),
        })
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
        kind: Kind,
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
        let callsite = GstCallsite::make_static();
        let key = lookup_key.leak();
        let target: &'static str =
            Box::leak(format!("{}::{}", crate::TARGET, key.target).into_boxed_str());
        let metadata = Box::leak(Box::new(Metadata::new(
            key.name,
            target,
            key.level,
            key.file,
            key.line,
            key.module,
            FieldSet::new(key.fields, identify_callsite!(callsite)),
            key.kind.clone(),
        )));
        tracing::debug!(message = "allocated a new callsite",
            current_callsites = guard.len(),
            name = key.name,
            target = target,
            kind = ?key.kind,
            file = ?key.file,
            line = ?key.line,
            module = ?key.module,
            fieldset = ?key.fields,
        );
        guard.insert(key, callsite);
        callsite.set_metadata(metadata);
        callsite
    }
}
