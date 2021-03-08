// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use crate::Pad;

pub trait PadImpl: PadImplExt + ObjectImpl + Send + Sync {
    fn linked(&self, pad: &Self::Type, peer: &Pad) {
        self.parent_linked(pad, peer)
    }

    fn unlinked(&self, pad: &Self::Type, peer: &Pad) {
        self.parent_unlinked(pad, peer)
    }
}

pub trait PadImplExt: ObjectSubclass {
    fn parent_linked(&self, pad: &Self::Type, peer: &Pad);

    fn parent_unlinked(&self, pad: &Self::Type, peer: &Pad);
}

impl<T: PadImpl> PadImplExt for T {
    fn parent_linked(&self, pad: &Self::Type, peer: &Pad) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstPadClass;

            (*parent_class)
                .linked
                .map(|f| {
                    f(
                        pad.unsafe_cast_ref::<Pad>().to_glib_none().0,
                        peer.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }

    fn parent_unlinked(&self, pad: &Self::Type, peer: &Pad) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstPadClass;

            (*parent_class)
                .unlinked
                .map(|f| {
                    f(
                        pad.unsafe_cast_ref::<Pad>().to_glib_none().0,
                        peer.to_glib_none().0,
                    )
                })
                .unwrap_or(())
        }
    }
}

unsafe impl<T: PadImpl> IsSubclassable<T> for Pad {
    fn class_init(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(klass);
        let klass = klass.as_mut();
        klass.linked = Some(pad_linked::<T>);
        klass.unlinked = Some(pad_unlinked::<T>);
    }
}

unsafe extern "C" fn pad_linked<T: PadImpl>(ptr: *mut ffi::GstPad, peer: *mut ffi::GstPad) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Pad> = from_glib_borrow(ptr);

    imp.linked(wrap.unsafe_cast_ref(), &from_glib_borrow(peer))
}

unsafe extern "C" fn pad_unlinked<T: PadImpl>(ptr: *mut ffi::GstPad, peer: *mut ffi::GstPad) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Pad> = from_glib_borrow(ptr);

    imp.unlinked(wrap.unsafe_cast_ref(), &from_glib_borrow(peer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::sync::atomic;

    use crate::PadDirection;

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

        impl PadImpl for TestPad {
            fn linked(&self, pad: &Self::Type, peer: &Pad) {
                self.linked.store(true, atomic::Ordering::SeqCst);
                self.parent_linked(pad, peer)
            }

            fn unlinked(&self, pad: &Self::Type, peer: &Pad) {
                self.unlinked.store(true, atomic::Ordering::SeqCst);
                self.parent_unlinked(pad, peer)
            }
        }
    }

    glib::wrapper! {
        pub struct TestPad(ObjectSubclass<imp::TestPad>) @extends Pad, crate::Object;
    }

    unsafe impl Send for TestPad {}
    unsafe impl Sync for TestPad {}

    impl TestPad {
        pub fn new(name: &str, direction: PadDirection) -> Self {
            glib::Object::new(&[("name", &name), ("direction", &direction)]).unwrap()
        }
    }

    #[test]
    fn test_pad_subclass() {
        crate::init().unwrap();

        let pad = TestPad::new("test", PadDirection::Src);

        assert_eq!(pad.get_name(), "test");

        let otherpad = Pad::new(Some("other-test"), PadDirection::Sink);
        pad.link(&otherpad).unwrap();
        pad.unlink(&otherpad).unwrap();

        let imp = imp::TestPad::from_instance(&pad);
        assert!(imp.linked.load(atomic::Ordering::SeqCst));
        assert!(imp.unlinked.load(atomic::Ordering::SeqCst));
    }
}
