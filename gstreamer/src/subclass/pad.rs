// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;

use glib;
use glib::translate::*;

use glib::subclass::prelude::*;

use Pad;
use PadClass;

pub trait PadImpl: ObjectImpl + Send + Sync + 'static {
    fn linked(&self, pad: &Pad, peer: &Pad) {
        self.parent_linked(pad, peer)
    }

    fn unlinked(&self, pad: &Pad, peer: &Pad) {
        self.parent_unlinked(pad, peer)
    }

    fn parent_linked(&self, pad: &Pad, peer: &Pad) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstPadClass;

            (*parent_class)
                .linked
                .map(|f| f(pad.to_glib_none().0, peer.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_unlinked(&self, pad: &Pad, peer: &Pad) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GstPadClass;

            (*parent_class)
                .unlinked
                .map(|f| f(pad.to_glib_none().0, peer.to_glib_none().0))
                .unwrap_or(())
        }
    }
}

unsafe impl<T: ObjectSubclass + PadImpl> IsSubclassable<T> for PadClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);

        unsafe {
            let klass = &mut *(self as *const Self as *mut ffi::GstPadClass);
            klass.linked = Some(pad_linked::<T>);
            klass.unlinked = Some(pad_unlinked::<T>);
        }
    }
}

unsafe extern "C" fn pad_linked<T: ObjectSubclass>(ptr: *mut ffi::GstPad, peer: *mut ffi::GstPad)
where
    T: PadImpl,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Pad = from_glib_borrow(ptr);

    imp.linked(&wrap, &from_glib_borrow(peer))
}

unsafe extern "C" fn pad_unlinked<T: ObjectSubclass>(ptr: *mut ffi::GstPad, peer: *mut ffi::GstPad)
where
    T: PadImpl,
{
    glib_floating_reference_guard!(ptr);
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Pad = from_glib_borrow(ptr);

    imp.unlinked(&wrap, &from_glib_borrow(peer))
}
