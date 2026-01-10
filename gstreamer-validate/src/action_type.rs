// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::c_int, ptr};

use crate::{ffi, Action};
use glib::translate::*;

#[derive(Debug)]
#[repr(transparent)]
#[doc(alias = "GstValidateActionParameter")]
pub struct ActionParameter(pub(crate) ffi::GstValidateActionParameter);
impl Drop for ActionParameter {
    fn drop(&mut self) {
        unsafe {
            if let Some(free_fn) = self.0.free {
                (free_fn)(self as *const _ as glib::ffi::gpointer);
            }
        }
    }
}

fn into_glib_content(mut t: Vec<ActionParameter>) -> *mut ffi::GstValidateActionParameter {
    skip_assert_initialized!();
    if t.is_empty() {
        return ptr::null_mut();
    }

    unsafe {
        let size = std::mem::size_of::<ffi::GstValidateActionParameter>() * (t.len() + 1);
        let v_ptr = glib::ffi::g_malloc0(size) as *mut ffi::GstValidateActionParameter;

        ptr::copy_nonoverlapping(
            t.as_ptr() as *const ffi::GstValidateActionParameter,
            v_ptr,
            t.len(),
        );

        // C side is owning the memory now
        t.set_len(0);

        v_ptr
    }
}

unsafe extern "C" fn action_parameter_free(param: glib::ffi::gpointer) {
    unsafe {
        let param = param as *mut ffi::GstValidateActionParameter;

        glib::ffi::g_free((*param).name as *mut _);
        glib::ffi::g_free((*param).description as *mut _);
        glib::ffi::g_free((*param).def as *mut _);
        glib::ffi::g_free((*param).possible_variables as *mut _);
        glib::ffi::g_free((*param).types as *mut _);
    }
}

pub struct ActionParameterBuilder<'a> {
    name: &'a str,
    description: &'a str,
    possible_variables: Vec<String>,
    mandatory: bool,
    default_value: Option<&'a str>,
    types: Vec<String>,
}

impl<'a> ActionParameterBuilder<'a> {
    pub fn new(name: &'a str, description: &'a str) -> Self {
        skip_assert_initialized!();

        Self {
            name,
            description,
            possible_variables: Default::default(),
            mandatory: false,
            default_value: None,
            types: Default::default(),
        }
    }

    // rustdoc-stripper-ignore-next
    /// The name of the variables that can be used to compute the value of the
    /// parameter. For example for the start value of a seek action, we will
    /// accept to take 'duration' which will be replace by the total duration of
    /// the stream on which the action is executed.
    pub fn add_possible_variable(mut self, possible_variable: &str) -> Self {
        self.possible_variables.push(possible_variable.to_owned());
        self
    }

    pub fn add_possible_variable_if(self, possible_variable: &str, predicate: bool) -> Self {
        if predicate {
            self.add_possible_variable(possible_variable)
        } else {
            self
        }
    }

    pub fn add_possible_variable_if_some(self, possible_variable: Option<&str>) -> Self {
        if let Some(possible_variable) = possible_variable {
            self.add_possible_variable(possible_variable)
        } else {
            self
        }
    }

    pub fn mandatory(mut self) -> Self {
        self.mandatory = true;
        self
    }

    pub fn default_value(mut self, default_value: &'a str) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn default_value_if(self, default_value: &'a str, predicate: bool) -> Self {
        if predicate {
            self.default_value(default_value)
        } else {
            self
        }
    }

    pub fn default_value_if_some(self, default_value: Option<&'a str>) -> Self {
        if let Some(default_value) = default_value {
            self.default_value(default_value)
        } else {
            self
        }
    }

    // rustdoc-stripper-ignore-next
    /// The types the parameter can take described as a string.
    ///
    /// NOTE: The types should end with `(GstClockTime)` if
    /// its final type is a GstClockTime, this way it will be processed when
    /// preparing the actions.
    pub fn add_type(mut self, types: &str) -> Self {
        self.types.push(types.to_owned());
        self
    }

    pub fn add_type_if(self, types: &str, predicate: bool) -> Self {
        if predicate {
            self.add_type(types)
        } else {
            self
        }
    }

    pub fn add_type_if_some(self, types: Option<&str>) -> Self {
        if let Some(types) = types {
            self.add_type(types)
        } else {
            self
        }
    }

    pub fn build(self) -> ActionParameter {
        let types = if self.types.is_empty() {
            ptr::null()
        } else {
            self.types.join("\n").to_glib_full()
        };
        let possible_variables = if self.possible_variables.is_empty() {
            ptr::null()
        } else {
            self.possible_variables.join("\n").to_glib_full()
        };
        ActionParameter(ffi::GstValidateActionParameter {
            name: self.name.to_glib_full(),
            description: self.description.to_glib_full(),
            mandatory: self.mandatory.into_glib(),
            def: self.default_value.to_glib_full(),
            possible_variables,
            types,
            free: Some(action_parameter_free),
            _gst_reserved: [ptr::null_mut(); 3],
        })
    }
}

type ActionFunction = dyn Fn(&crate::Scenario, &mut crate::Action) -> Result<crate::ActionSuccess, crate::ActionError>
    + Sync
    + Send
    + 'static;

unsafe extern "C" fn destroy_notify(ptr: glib::ffi::gpointer) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut Box<ActionFunction>);
    }
}

pub struct ActionTypeBuilder<'a> {
    type_name: &'a str,
    implementer_namespace: Option<&'a str>,
    description: Option<&'a str>,
    parameters: Vec<ActionParameter>,
    flags: crate::ActionTypeFlags,
    function: Box<ActionFunction>,
}

impl<'a> ActionTypeBuilder<'a> {
    pub fn new<
        F: Fn(
                &crate::Scenario,
                &mut crate::Action,
            ) -> Result<crate::ActionSuccess, crate::ActionError>
            + Send
            + Sync
            + 'static,
    >(
        type_name: &'a str,
        func: F,
    ) -> Self {
        Self {
            type_name,
            implementer_namespace: None,
            description: None,
            parameters: Vec::new(),
            flags: crate::ActionTypeFlags::empty(),
            function: Box::new(func),
        }
    }

    pub fn implementer_namespace(mut self, implementer_namespace: &'a str) -> Self {
        self.implementer_namespace = Some(implementer_namespace);
        self
    }

    pub fn implementer_namespace_if(
        mut self,
        implementer_namespace: &'a str,
        predicate: bool,
    ) -> Self {
        if predicate {
            self.implementer_namespace = Some(implementer_namespace);
            self
        } else {
            self
        }
    }

    pub fn implementer_namespace_if_some(self, implementer_namespace: Option<&'a str>) -> Self {
        if let Some(implementer_namespace) = implementer_namespace {
            self.implementer_namespace(implementer_namespace)
        } else {
            self
        }
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn description_if(mut self, description: &'a str, predicate: bool) -> Self {
        if predicate {
            self.description = Some(description);
            self
        } else {
            self
        }
    }

    pub fn description_if_some(self, description: Option<&'a str>) -> Self {
        if let Some(description) = description {
            self.description(description)
        } else {
            self
        }
    }

    pub fn parameter(mut self, parameter: ActionParameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn parameter_if(mut self, parameter: ActionParameter, predicate: bool) -> Self {
        if predicate {
            self.parameters.push(parameter);
            self
        } else {
            self
        }
    }

    pub fn parameter_if_some(self, parameter: Option<ActionParameter>) -> Self {
        if let Some(parameter) = parameter {
            self.parameter(parameter)
        } else {
            self
        }
    }

    pub fn flags(mut self, flags: crate::ActionTypeFlags) -> Self {
        self.flags |= flags;
        self
    }

    pub fn flags_if(mut self, flags: crate::ActionTypeFlags, predicate: bool) -> Self {
        if predicate {
            self.flags |= flags;
            self
        } else {
            self
        }
    }

    pub fn flags_if_some(self, flags: Option<crate::ActionTypeFlags>) -> Self {
        if let Some(flags) = flags {
            self.flags(flags)
        } else {
            self
        }
    }

    pub fn build(self) -> crate::ActionType {
        static QUARK_ACTION_TYPE_FUNC: std::sync::OnceLock<glib::Quark> =
            std::sync::OnceLock::new();

        let quark_action_type_func =
            QUARK_ACTION_TYPE_FUNC.get_or_init(|| glib::Quark::from_str("rs-action-type-function"));

        unsafe extern "C" fn execute_func_trampoline(
            scenario: *mut ffi::GstValidateScenario,
            mut action_ptr: *mut ffi::GstValidateAction,
        ) -> c_int {
            unsafe {
                let action_type = ffi::gst_validate_get_action_type((*action_ptr).type_);
                let scenario = from_glib_borrow(scenario);

                let func: &ActionFunction = &*(gst::ffi::gst_mini_object_get_qdata(
                    action_type as *mut gst::ffi::GstMiniObject,
                    QUARK_ACTION_TYPE_FUNC.get().unwrap().into_glib(),
                ) as *const Box<ActionFunction>);

                // SAFETY: `execute_func_trampoline` is called with the unic reference of `action_ptr`
                // so we can safely borrow it mutably
                let original_ptr = action_ptr;
                let action = Action::from_glib_ptr_borrow_mut(&mut action_ptr);
                let res = (*func)(&scenario, action);

                debug_assert_eq!(action.as_ptr(), original_ptr);
                match res {
                    Err(crate::ActionError::Error(ref err)) => {
                        action.report_error(err);
                        ffi::GST_VALIDATE_EXECUTE_ACTION_ERROR_REPORTED
                    }
                    Ok(v) => v.into_glib(),
                }
            }
        }

        unsafe {
            let params = into_glib_content(self.parameters);
            let action_type = ffi::gst_validate_register_action_type(
                self.type_name.to_glib_none().0,
                self.implementer_namespace
                    .unwrap_or("validaters")
                    .to_glib_none()
                    .0,
                Some(execute_func_trampoline),
                params,
                self.description.to_glib_none().0,
                self.flags.into_glib(),
            );

            // gst_validate_register_action_type() takes ownership of the content
            // of the params array but not of the container itself so we need to
            // free it manually.
            glib::ffi::g_free(params as *mut _);

            let f = self.function;

            gst::ffi::gst_mini_object_set_qdata(
                action_type as *mut gst::ffi::GstMiniObject,
                quark_action_type_func.into_glib(),
                Box::into_raw(Box::new(f)) as *mut _,
                Some(destroy_notify),
            );

            from_glib_none(action_type)
        }
    }
}

pub trait ActionTypeExtManual: 'static {
    #[doc(alias = "gst_validate_get_action_type")]
    fn find(name: &str) -> Option<crate::ActionType>;
}

impl ActionTypeExtManual for crate::ActionType {
    fn find(name: &str) -> Option<crate::ActionType> {
        assert_initialized_main_thread!();
        unsafe {
            let action_type = ffi::gst_validate_get_action_type(name.to_glib_none().0);

            if action_type.is_null() {
                None
            } else {
                Some(from_glib_full(action_type))
            }
        }
    }
}
