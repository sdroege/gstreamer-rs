// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use super::prelude::*;
use crate::Pad;

pub trait PadImpl: PadImplExt + GstObjectImpl + Send + Sync {
    fn linked(&self, peer: &Pad) {
        self.parent_linked(peer)
    }

    fn unlinked(&self, peer: &Pad) {
        self.parent_unlinked(peer)
    }
}

pub trait PadImplExt: ObjectSubclass {
    fn parent_linked(&self, peer: &Pad);

    fn parent_unlinked(&self, peer: &Pad);
}

impl<T: PadImpl> PadImplExt for T {
    fn parent_linked(&self, peer: &Pad) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPadClass;

            (*parent_class)
                .linked
                .map(|f| {
                    f(
                        self.obj().unsafe_cast_ref::<Pad>().to_glib_none().0,
                        peer.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_unlinked(&self, peer: &Pad) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstPadClass;

            (*parent_class)
                .unlinked
                .map(|f| {
                    f(
                        self.obj().unsafe_cast_ref::<Pad>().to_glib_none().0,
                        peer.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }
}

unsafe impl<T: PadImpl> IsSubclassable<T> for Pad {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.linked = Some(pad_linked::<T>);
        klass.unlinked = Some(pad_unlinked::<T>);
    }
}

unsafe extern "C" fn pad_linked<T: PadImpl>(ptr: *mut ffi::GstPad, peer: *mut ffi::GstPad) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.linked(&from_glib_borrow(peer))
}

unsafe extern "C" fn pad_unlinked<T: PadImpl>(ptr: *mut ffi::GstPad, peer: *mut ffi::GstPad) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.unlinked(&from_glib_borrow(peer))
}

#[cfg(test)]
mod tests {
    use std::sync::atomic;

    use super::*;
    use crate::{prelude::*, PadDirection};

    pub mod imp {
        use super::*;

        #[derive(Default)]
        pub struct TestPad {
            pub(super) linked: atomic::AtomicBool,
            pub(super) unlinked: atomic::AtomicBool,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestPad {
            const NAME: &'static str = "TestPad";
            type Type = super::TestPad;
            type ParentType = Pad;
        }

        impl ObjectImpl for TestPad {}

        impl GstObjectImpl for TestPad {}

        impl PadImpl for TestPad {
            fn linked(&self, peer: &Pad) {
                self.linked.store(true, atomic::Ordering::SeqCst);
                self.parent_linked(peer)
            }

            fn unlinked(&self, peer: &Pad) {
                self.unlinked.store(true, atomic::Ordering::SeqCst);
                self.parent_unlinked(peer)
            }
        }
    }

    glib::wrapper! {
        pub struct TestPad(ObjectSubclass<imp::TestPad>) @extends Pad, crate::Object;
    }

    impl TestPad {
        pub fn new(name: &str, direction: PadDirection) -> Self {
            glib::Object::new(&[("name", &name), ("direction", &direction)])
        }
    }

    #[test]
    fn test_pad_subclass() {
        crate::init().unwrap();

        let pad = TestPad::new("test", PadDirection::Src);

        assert_eq!(pad.name(), "test");

        let otherpad = Pad::new(Some("other-test"), PadDirection::Sink);
        pad.link(&otherpad).unwrap();
        pad.unlink(&otherpad).unwrap();

        let imp = pad.imp();
        assert!(imp.linked.load(atomic::Ordering::SeqCst));
        assert!(imp.unlinked.load(atomic::Ordering::SeqCst));
    }
}
