use std::{ffi::c_int, ptr};

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
    assert_initialized_main_thread!();
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
    let param = param as *mut ffi::GstValidateActionParameter;

    glib::ffi::g_free((*param).name as *mut _);
    glib::ffi::g_free((*param).description as *mut _);
    glib::ffi::g_free((*param).def as *mut _);
    glib::ffi::g_free((*param).possible_variables as *mut _);
    glib::ffi::g_free((*param).types as *mut _);
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
        assert_initialized_main_thread!();

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
    pub fn add_possible_variable(mut self, possible_variables: &str) -> Self {
        self.possible_variables.push(possible_variables.to_owned());
        self
    }

    pub fn mandatory(mut self) -> Self {
        self.mandatory = true;
        self
    }

    pub fn default_value(mut self, default_value: &'a str) -> Self {
        self.default_value = Some(default_value);
        self
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

type ActionFunction = dyn Fn(&crate::Scenario, &mut crate::ActionRef) -> Result<crate::ActionSuccess, crate::ActionError>
    + Sync
    + Send
    + 'static;

unsafe extern "C" fn destroy_notify(ptr: glib::ffi::gpointer) {
    let _ = Box::from_raw(ptr as *mut Box<ActionFunction>);
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
                &mut crate::ActionRef,
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

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn parameter(mut self, parameter: ActionParameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn flags(mut self, flags: crate::ActionTypeFlags) -> Self {
        self.flags |= flags;
        self
    }

    pub fn build(self) -> crate::ActionType {
        static QUARK_ACTION_TYPE_FUNC: std::sync::OnceLock<glib::Quark> =
            std::sync::OnceLock::new();

        let quark_action_type_func =
            QUARK_ACTION_TYPE_FUNC.get_or_init(|| glib::Quark::from_str("rs-action-type-function"));

        unsafe extern "C" fn execute_func_trampoline(
            scenario: *mut ffi::GstValidateScenario,
            action: *mut ffi::GstValidateAction,
        ) -> c_int {
            let action_type = ffi::gst_validate_get_action_type((*action).type_);
            let scenario = from_glib_borrow(scenario);
            let action = crate::ActionRef::from_mut_ptr(action);

            let func: &ActionFunction = &*(gst::ffi::gst_mini_object_get_qdata(
                action_type as *mut gst::ffi::GstMiniObject,
                QUARK_ACTION_TYPE_FUNC.get().unwrap().into_glib(),
            ) as *const Box<ActionFunction>);

            (*func)(&scenario, action).into_glib()
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

#[cfg(test)]
mod tests {
    use std::{
        io::Write,
        sync::{Arc, Mutex},
    };

    #[test]
    fn test_action_types() {
        gst::init().unwrap();
        crate::init();

        let failling_action_type = crate::ActionTypeBuilder::new("fails", |_, action| {
            action.structure_mut().set("called", true);

            Err(crate::ActionError::Error)
        })
        .build();

        let called = Arc::new(Mutex::new(false));
        let succeeding_action_type = crate::ActionTypeBuilder::new(
            "succeeds",
            glib::clone!(@strong called => move |_, _action| {
                *called.lock().unwrap() = true;

                Ok(crate::ActionSuccess::Ok)
            }),
        )
        .parameter(
            crate::ActionParameterBuilder::new("always", "Does the action always succeeds")
                .add_type("boolean")
                .default_value("true")
                .build(),
        )
        .build();

        // Write scenario to temporary file
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(b"succeeds").unwrap();

        let runner = crate::Runner::new();
        let pipeline = gst::Pipeline::new();
        let scenario =
            crate::Scenario::factory_create(&runner, &pipeline, file.path().to_str().unwrap())
                .unwrap();

        let action = crate::Action::new(
            Some(&scenario),
            &succeeding_action_type,
            gst::Structure::builder("succeeds").build().as_ref(),
            false,
        );

        assert!(!*called.lock().unwrap());
        action.execute().expect("Failed to execute action");
        assert!(*called.lock().unwrap());

        let action = crate::Action::new(
            Some(&scenario),
            &failling_action_type,
            gst::Structure::builder("fails").build().as_ref(),
            false,
        );

        assert!(action.structure().get::<bool>("called").is_err());
        action.execute().expect_err("Action should have failed");
        assert_eq!(action.structure().get::<bool>("called"), Ok(true));

        crate::ActionParameterBuilder::new("unused", "Verify unused param are properly cleaned")
            .default_value("true")
            .add_possible_variable("position")
            .build();
    }
}
