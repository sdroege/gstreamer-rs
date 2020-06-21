// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Caps;
use Element;
use Event;
use EventType;
use Object;
use PadDirection;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use PadLinkCheck;
use PadMode;
use PadTemplate;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Stream;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use TaskState;

glib_wrapper! {
    pub struct Pad(Object<gst_sys::GstPad, gst_sys::GstPadClass, PadClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_pad_get_type(),
    }
}

impl Pad {
    pub fn new(name: Option<&str>, direction: PadDirection) -> Pad {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gst_sys::gst_pad_new(
                name.to_glib_none().0,
                direction.to_glib(),
            ))
        }
    }

    pub fn from_template(templ: &PadTemplate, name: Option<&str>) -> Pad {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(gst_sys::gst_pad_new_from_template(
                templ.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for Pad {}
unsafe impl Sync for Pad {}

pub const NONE_PAD: Option<&Pad> = None;

pub trait PadExt: 'static {
    fn activate_mode(&self, mode: PadMode, active: bool) -> Result<(), glib::error::BoolError>;

    fn can_link<P: IsA<Pad>>(&self, sinkpad: &P) -> bool;

    fn check_reconfigure(&self) -> bool;

    fn create_stream_id<P: IsA<Element>>(
        &self,
        parent: &P,
        stream_id: Option<&str>,
    ) -> Option<GString>;

    //fn create_stream_id_printf<P: IsA<Element>>(&self, parent: &P, stream_id: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<GString>;

    //fn create_stream_id_printf_valist<P: IsA<Element>>(&self, parent: &P, stream_id: Option<&str>, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<GString>;

    fn forward<P: FnMut(&Pad) -> bool>(&self, forward: P) -> bool;

    fn get_allowed_caps(&self) -> Option<Caps>;

    fn get_current_caps(&self) -> Option<Caps>;

    fn get_direction(&self) -> PadDirection;

    //fn get_element_private(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    fn get_offset(&self) -> i64;

    fn get_pad_template(&self) -> Option<PadTemplate>;

    fn get_pad_template_caps(&self) -> Option<Caps>;

    fn get_parent_element(&self) -> Option<Element>;

    fn get_peer(&self) -> Option<Pad>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn get_single_internal_link(&self) -> Option<Pad>;

    fn get_sticky_event(&self, event_type: EventType, idx: u32) -> Option<Event>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_stream(&self) -> Option<Stream>;

    fn get_stream_id(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_task_state(&self) -> TaskState;

    fn has_current_caps(&self) -> bool;

    fn is_active(&self) -> bool;

    fn is_blocked(&self) -> bool;

    fn is_blocking(&self) -> bool;

    fn is_linked(&self) -> bool;

    //fn iterate_internal_links(&self) -> /*Ignored*/Option<Iterator>;

    //fn iterate_internal_links_default<P: IsA<Object>>(&self, parent: Option<&P>) -> /*Ignored*/Option<Iterator>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn link_maybe_ghosting<P: IsA<Pad>>(&self, sink: &P) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn link_maybe_ghosting_full<P: IsA<Pad>>(
        &self,
        sink: &P,
        flags: PadLinkCheck,
    ) -> Result<(), glib::error::BoolError>;

    fn mark_reconfigure(&self);

    fn needs_reconfigure(&self) -> bool;

    fn pause_task(&self) -> Result<(), glib::error::BoolError>;

    fn peer_query_accept_caps(&self, caps: &Caps) -> bool;

    fn peer_query_caps(&self, filter: Option<&Caps>) -> Option<Caps>;

    fn query_accept_caps(&self, caps: &Caps) -> bool;

    fn query_caps(&self, filter: Option<&Caps>) -> Option<Caps>;

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError>;

    //fn set_element_private(&self, priv_: /*Unimplemented*/Option<Fundamental: Pointer>);

    fn set_offset(&self, offset: i64);

    fn stop_task(&self) -> Result<(), glib::error::BoolError>;

    fn unlink<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<(), glib::error::BoolError>;

    fn use_fixed_caps(&self);

    fn get_property_caps(&self) -> Option<Caps>;

    fn connect_linked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unlinked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Pad>> PadExt for O {
    fn activate_mode(&self, mode: PadMode, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_pad_activate_mode(
                    self.as_ref().to_glib_none().0,
                    mode.to_glib(),
                    active.to_glib()
                ),
                "Failed to activate mode pad"
            )
        }
    }

    fn can_link<P: IsA<Pad>>(&self, sinkpad: &P) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_can_link(
                self.as_ref().to_glib_none().0,
                sinkpad.as_ref().to_glib_none().0,
            ))
        }
    }

    fn check_reconfigure(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_check_reconfigure(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn create_stream_id<P: IsA<Element>>(
        &self,
        parent: &P,
        stream_id: Option<&str>,
    ) -> Option<GString> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_create_stream_id(
                self.as_ref().to_glib_none().0,
                parent.as_ref().to_glib_none().0,
                stream_id.to_glib_none().0,
            ))
        }
    }

    //fn create_stream_id_printf<P: IsA<Element>>(&self, parent: &P, stream_id: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<GString> {
    //    unsafe { TODO: call gst_sys:gst_pad_create_stream_id_printf() }
    //}

    //fn create_stream_id_printf_valist<P: IsA<Element>>(&self, parent: &P, stream_id: Option<&str>, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<GString> {
    //    unsafe { TODO: call gst_sys:gst_pad_create_stream_id_printf_valist() }
    //}

    fn forward<P: FnMut(&Pad) -> bool>(&self, forward: P) -> bool {
        let forward_data: P = forward;
        unsafe extern "C" fn forward_func<P: FnMut(&Pad) -> bool>(
            pad: *mut gst_sys::GstPad,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let pad = from_glib_borrow(pad);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&pad);
            res.to_glib()
        }
        let forward = Some(forward_func::<P> as _);
        let super_callback0: &P = &forward_data;
        unsafe {
            from_glib(gst_sys::gst_pad_forward(
                self.as_ref().to_glib_none().0,
                forward,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn get_allowed_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_get_allowed_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_current_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_get_current_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_direction(&self) -> PadDirection {
        unsafe {
            from_glib(gst_sys::gst_pad_get_direction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_element_private(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gst_sys:gst_pad_get_element_private() }
    //}

    fn get_offset(&self) -> i64 {
        unsafe { gst_sys::gst_pad_get_offset(self.as_ref().to_glib_none().0) }
    }

    fn get_pad_template(&self) -> Option<PadTemplate> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_get_pad_template(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pad_template_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_get_pad_template_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_parent_element(&self) -> Option<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_get_parent_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_peer(&self) -> Option<Pad> {
        unsafe { from_glib_full(gst_sys::gst_pad_get_peer(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn get_single_internal_link(&self) -> Option<Pad> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_get_single_internal_link(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_sticky_event(&self, event_type: EventType, idx: u32) -> Option<Event> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_get_sticky_event(
                self.as_ref().to_glib_none().0,
                event_type.to_glib(),
                idx,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_stream(&self) -> Option<Stream> {
        unsafe { from_glib_full(gst_sys::gst_pad_get_stream(self.as_ref().to_glib_none().0)) }
    }

    fn get_stream_id(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_get_stream_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_task_state(&self) -> TaskState {
        unsafe {
            from_glib(gst_sys::gst_pad_get_task_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_current_caps(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_has_current_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe { from_glib(gst_sys::gst_pad_is_active(self.as_ref().to_glib_none().0)) }
    }

    fn is_blocked(&self) -> bool {
        unsafe { from_glib(gst_sys::gst_pad_is_blocked(self.as_ref().to_glib_none().0)) }
    }

    fn is_blocking(&self) -> bool {
        unsafe { from_glib(gst_sys::gst_pad_is_blocking(self.as_ref().to_glib_none().0)) }
    }

    fn is_linked(&self) -> bool {
        unsafe { from_glib(gst_sys::gst_pad_is_linked(self.as_ref().to_glib_none().0)) }
    }

    //fn iterate_internal_links(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_pad_iterate_internal_links() }
    //}

    //fn iterate_internal_links_default<P: IsA<Object>>(&self, parent: Option<&P>) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call gst_sys:gst_pad_iterate_internal_links_default() }
    //}

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn link_maybe_ghosting<P: IsA<Pad>>(&self, sink: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_pad_link_maybe_ghosting(
                    self.as_ref().to_glib_none().0,
                    sink.as_ref().to_glib_none().0
                ),
                "Failed to link pads, possibly ghosting"
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn link_maybe_ghosting_full<P: IsA<Pad>>(
        &self,
        sink: &P,
        flags: PadLinkCheck,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_pad_link_maybe_ghosting_full(
                    self.as_ref().to_glib_none().0,
                    sink.as_ref().to_glib_none().0,
                    flags.to_glib()
                ),
                "Failed to link pads, possibly ghosting"
            )
        }
    }

    fn mark_reconfigure(&self) {
        unsafe {
            gst_sys::gst_pad_mark_reconfigure(self.as_ref().to_glib_none().0);
        }
    }

    fn needs_reconfigure(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_needs_reconfigure(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pause_task(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_pad_pause_task(self.as_ref().to_glib_none().0),
                "Failed to pause pad task"
            )
        }
    }

    fn peer_query_accept_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_peer_query_accept_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    fn peer_query_caps(&self, filter: Option<&Caps>) -> Option<Caps> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_peer_query_caps(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn query_accept_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(gst_sys::gst_pad_query_accept_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    fn query_caps(&self, filter: Option<&Caps>) -> Option<Caps> {
        unsafe {
            from_glib_full(gst_sys::gst_pad_query_caps(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_pad_set_active(self.as_ref().to_glib_none().0, active.to_glib()),
                "Failed to activate pad"
            )
        }
    }

    //fn set_element_private(&self, priv_: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gst_sys:gst_pad_set_element_private() }
    //}

    fn set_offset(&self, offset: i64) {
        unsafe {
            gst_sys::gst_pad_set_offset(self.as_ref().to_glib_none().0, offset);
        }
    }

    fn stop_task(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_pad_stop_task(self.as_ref().to_glib_none().0),
                "Failed to stop pad task"
            )
        }
    }

    fn unlink<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_sys::gst_pad_unlink(
                    self.as_ref().to_glib_none().0,
                    sinkpad.as_ref().to_glib_none().0
                ),
                "Failed to unlink pad"
            )
        }
    }

    fn use_fixed_caps(&self) {
        unsafe {
            gst_sys::gst_pad_use_fixed_caps(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_caps(&self) -> Option<Caps> {
        unsafe {
            let mut value = Value::from_type(<Caps as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"caps\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `caps` getter")
        }
    }

    fn connect_linked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn linked_trampoline<P, F: Fn(&P, &Pad) + Send + Sync + 'static>(
            this: *mut gst_sys::GstPad,
            peer: *mut gst_sys::GstPad,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pad>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Pad::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(peer),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"linked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    linked_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_unlinked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn unlinked_trampoline<P, F: Fn(&P, &Pad) + Send + Sync + 'static>(
            this: *mut gst_sys::GstPad,
            peer: *mut gst_sys::GstPad,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pad>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Pad::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(peer),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unlinked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unlinked_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_sys::GstPad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pad>,
        {
            let f: &F = &*(f as *const F);
            f(&Pad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_sys::GstPad,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Pad>,
        {
            let f: &F = &*(f as *const F);
            f(&Pad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
