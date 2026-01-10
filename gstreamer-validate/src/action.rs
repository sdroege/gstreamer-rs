// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use glib::prelude::*;
use glib::translate::*;

use crate::{ActionType, Scenario, ffi};

gst::mini_object_wrapper!(Action, ActionRef, ffi::GstValidateAction, || {
    ffi::gst_validate_action_get_type()
});

impl ActionRef {
    pub fn structure(&self) -> Option<&gst::StructureRef> {
        unsafe {
            let action = &self.0 as *const ffi::GstValidateAction;

            if (*action).structure.is_null() {
                None
            } else {
                Some(gst::StructureRef::from_glib_borrow((*action).structure))
            }
        }
    }

    pub fn structure_mut(&mut self) -> Option<&mut gst::StructureRef> {
        unsafe {
            let action = &mut self.0 as *mut ffi::GstValidateAction;

            if (*action).structure.is_null() {
                None
            } else {
                Some(gst::StructureRef::from_glib_borrow_mut((*action).structure))
            }
        }
    }

    #[doc(alias = "gst_validate_action_get_scenario")]
    pub fn scenario(&self) -> Option<Scenario> {
        unsafe {
            let scenario = ffi::gst_validate_action_get_scenario(self.as_mut_ptr());

            from_glib_full(scenario)
        }
    }
}

impl Action {
    pub(crate) unsafe fn from_glib_ptr_borrow_mut(
        ptr: &mut *mut ffi::GstValidateAction,
    ) -> &mut Self {
        assert_initialized_main_thread!();

        unsafe {
            debug_assert_eq!((*(*ptr)).mini_object.refcount, 1);

            debug_assert_eq!(
                std::mem::size_of::<Action>(),
                std::mem::size_of::<gst::glib::ffi::gpointer>()
            );
            debug_assert!(!ptr.is_null());

            &mut *(ptr as *mut *mut ffi::GstValidateAction as *mut Action)
        }
    }

    #[doc(alias = "gst_validate_action_new")]
    pub fn new(
        scenario: Option<&impl IsA<Scenario>>,
        action_type: &ActionType,
        structure: &gst::StructureRef,
        add_to_lists: bool,
    ) -> Action {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_validate_action_new(
                scenario.map(|p| p.as_ref()).to_glib_none().0,
                action_type.to_glib_none().0,
                structure.as_mut_ptr(),
                add_to_lists.into_glib(),
            ))
        }
    }

    pub fn name(&self) -> &str {
        unsafe {
            let action: *mut ffi::GstValidateAction = self.to_glib_none().0;
            CStr::from_ptr((*action).name).to_str().unwrap()
        }
    }

    pub fn report_error(&self, error_message: &str) {
        if let Some(scenario) = self.scenario() {
            scenario.upcast_ref::<crate::Reporter>().report_action(
                self,
                glib::Quark::from_str("scenario::execution-error"),
                error_message,
            )
        }
    }

    #[doc(alias = "gst_validate_execute_action")]
    pub fn execute(self) -> Result<crate::ActionSuccess, crate::ActionError> {
        unsafe {
            let action: *mut ffi::GstValidateAction = self.into_glib_ptr();
            let action_type = ffi::gst_validate_get_action_type((*action).type_);

            let res = ffi::gst_validate_execute_action(action_type, action);

            if let Some(v) = crate::ActionSuccess::from_value(res) {
                Ok(v)
            } else {
                Err(crate::ActionError::from_value(res))
            }
        }
    }

    #[doc(alias = "gst_validate_action_set_done")]
    pub fn set_done(self) {
        unsafe {
            ffi::gst_validate_action_set_done(self.into_glib_ptr());
        }
    }
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Action")
            .field("structure", &self.structure())
            .field("name", &self.name())
            .finish()
    }
}
