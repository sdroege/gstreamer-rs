use gstreamer_validate as gst_validate;
use gstreamer_validate::prelude::*;
use std::{
    env,
    io::Write,
    sync::{Arc, Condvar, Mutex},
};

fn init() {
    std::sync::Once::new().call_once(|| {
        // Validate should not exit process on criticals
        env::set_var("GST_VALIDATE", "");

        gst::init().unwrap();
        gst_validate::init();
    })
}

#[test]
fn test_action_types() {
    init();

    let fails_called = Arc::new(Mutex::new(false));
    let failling_action_type = gst_validate::ActionTypeBuilder::new(
        "fails",
        glib::clone!(
            #[strong]
            fails_called,
            move |_, _action| {
                *fails_called.lock().unwrap() = true;

                Err(gst_validate::ActionError::Error(
                    "the `fails` action seems to fail".into(),
                ))
            }
        ),
    )
    .build();

    let succeeds_called = Arc::new(Mutex::new(false));
    let succeeding_action_type = gst_validate::ActionTypeBuilder::new(
        "succeeds",
        glib::clone!(
            #[strong]
            succeeds_called,
            move |_, _action| {
                *succeeds_called.lock().unwrap() = true;

                Ok(gst_validate::ActionSuccess::Ok)
            }
        ),
    )
    .parameter(
        gst_validate::ActionParameterBuilder::new("always", "Does the action always succeeds")
            .add_type("boolean")
            .default_value("true")
            .build(),
    )
    .build();

    // Write scenario to temporary file
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(b"stop, on-message=eos").unwrap();

    let runner = gst_validate::Runner::new();
    let pipeline = gst::Pipeline::new();
    let scenario =
        gst_validate::Scenario::factory_create(&runner, &pipeline, file.path().to_str().unwrap())
            .unwrap();

    let action = gst_validate::Action::new(
        Some(&scenario),
        &succeeding_action_type,
        gst::Structure::builder("succeeds").build().as_ref(),
        false,
    );

    assert!(!*succeeds_called.lock().unwrap());
    action.execute().expect("Failed to execute action");
    assert!(*succeeds_called.lock().unwrap());

    let action = gst_validate::Action::new(
        Some(&scenario),
        &failling_action_type,
        gst::Structure::builder("fails").build().as_ref(),
        false,
    );

    assert!(!*fails_called.lock().unwrap());
    action.execute().expect_err("Action should have failed");
    assert!(*fails_called.lock().unwrap());

    gst_validate::ActionParameterBuilder::new("async", "Verify unused param are properly cleaned")
        .default_value("true")
        .add_possible_variable("position")
        .build();

    let async_called = Arc::new((Mutex::new(false), Condvar::new()));
    gst_validate::ActionTypeBuilder::new(
        "async",
        glib::clone!(
            #[strong]
            async_called,
            move |_, action| {
                let action_mut = action.get_mut().unwrap();
                action_mut
                    .structure_mut()
                    .expect("We should have a structure set in that action")
                    .set("running-async", true);

                std::thread::spawn(glib::clone!(
                    #[strong]
                    async_called,
                    #[strong]
                    action,
                    move || {
                        *async_called.0.lock().unwrap() = true;

                        assert!(action
                            .structure()
                            .unwrap()
                            .get::<bool>("running-async")
                            .unwrap());
                        action.set_done();
                    }
                ));

                Ok(gst_validate::ActionSuccess::Async)
            }
        ),
    )
    .build();

    let async_type = gst_validate::ActionType::find("async").expect("Failed to find action type");
    let action = gst_validate::Action::new(
        Some(&scenario),
        &async_type,
        gst::Structure::builder("async").build().as_ref(),
        false,
    );

    scenario.connect_action_done(glib::clone!(
        #[strong]
        async_called,
        move |_, _| {
            async_called.1.notify_one();
        }
    ));

    {
        let called = async_called.0.lock().unwrap();
        match action.execute() {
            Ok(gst_validate::ActionSuccess::Async) => (),
            _ => panic!("Action should have returned Async"),
        }
        assert!(!*called);
    }

    let mut called = async_called.0.lock().unwrap();
    while !*called {
        called = async_called.1.wait(called).unwrap();
    }
}
