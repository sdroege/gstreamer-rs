use std::sync::atomic::{AtomicPtr, Ordering};
use tracing_core::{Callsite, Interest, Metadata};

pub(super) struct GstCallsite(AtomicPtr<Metadata<'static>>);

impl GstCallsite {
    pub(super) fn make_static() -> &'static Self {
        #[allow(unused_unsafe)]
        unsafe {
            // SAFETY: for `metadata` to be sound this must initialize with a null pointer.
            Box::leak(Box::new(GstCallsite(AtomicPtr::new(std::ptr::null_mut()))))
        }
    }
    pub(super) fn set_metadata(&self, meta: &'static Metadata<'static>) {
        self.0.store(meta as *const _ as _, Ordering::Release);
    }
}

impl Callsite for GstCallsite {
    fn set_interest(&self, _: Interest) {}
    fn metadata(&self) -> &Metadata<'_> {
        unsafe {
            // SAFETY: this type always contains nullptr (and `as_ref` will return `None`) until it
            // is initialized in the `set_metadata` function. `set_metadata` will always set a
            // valid pointer by definition of storing the address of a `&'static` reference.
            self.0
                .load(Ordering::Acquire)
                .as_ref()
                .expect("metadata must have been initialized already!")
        }
    }
}
