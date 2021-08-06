use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use tracing_core::{Callsite, Interest, Metadata};

pub(super) struct GstCallsite {
    interest: AtomicUsize,
    metadata: AtomicPtr<Metadata<'static>>,
}

impl GstCallsite {
    pub(super) fn make_static() -> &'static Self {
        #[allow(unused_unsafe)]
        unsafe {
            // SAFETY: for `metadata` to be sound this must initialize with a null pointer.
            Box::leak(Box::new(GstCallsite {
                interest: AtomicUsize::new(0),
                metadata: AtomicPtr::new(std::ptr::null_mut()),
            }))
        }
    }
    pub(super) fn set_metadata(&'static self, meta: &'static Metadata<'static>) {
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

    pub(super) fn interest(&self) -> Interest {
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
