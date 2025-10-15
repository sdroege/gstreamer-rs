/// macOS has a specific requirement that there must be a run loop running on the main thread in
/// order to open windows and use OpenGL, and that the global NSApplication instance must be
/// initialized.
/// On macOS this launches the callback function on a thread.
/// On other platforms it's just executed immediately.
#[cfg(not(target_os = "macos"))]
pub fn run<T, F: FnOnce() -> T + Send + 'static>(main: F) -> T
where
    T: Send + 'static,
{
    main()
}

#[cfg(target_os = "macos")]
pub fn run<T, F: FnOnce() -> T + Send + 'static>(main: F) -> T
where
    T: Send + 'static,
{
    use std::{
        cell::RefCell,
        sync::mpsc::{channel, Sender},
        thread,
    };

    use dispatch::Queue;
    use objc2::rc::Retained;
    use objc2::runtime::ProtocolObject;
    use objc2::{define_class, msg_send, DefinedClass, MainThreadOnly};
    use objc2_app_kit::{
        NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSEvent,
        NSEventModifierFlags, NSEventSubtype, NSEventType,
    };
    use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint};

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "AppDelegate"]
        #[ivars = RefCell<Option<Sender<()>>>]
        struct AppDelegate;

        unsafe impl NSObjectProtocol for AppDelegate {}

        unsafe impl NSApplicationDelegate for AppDelegate {
            #[unsafe(method(applicationDidFinishLaunching:))]
            unsafe fn application_did_finish_launching(&self, _notification: &NSNotification) {
                if let Some(sender) = self.ivars().borrow_mut().take() {
                    let _ = sender.send(());
                }
            }
        }
    );

    impl AppDelegate {
        fn new(sender: Sender<()>, mtm: MainThreadMarker) -> Retained<Self> {
            let this = mtm.alloc();
            let this = this.set_ivars(RefCell::new(Some(sender)));
            unsafe { msg_send![super(this), init] }
        }
    }

    let mtm = MainThreadMarker::new().expect("Must be called on main thread");
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

    let (send, recv) = channel::<()>();
    let delegate = AppDelegate::new(send, mtm);
    let delegate = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(delegate));

    let t = thread::spawn(move || {
        // Wait for the NSApp to launch to avoid possibly calling stop_() too early
        recv.recv().unwrap();

        let res = main();

        // Dispatch the stop call to the main queue to be thread-safe
        Queue::main().exec_async(|| {
            // This block runs on the main thread, so MainThreadMarker::new() will succeed
            let mtm = MainThreadMarker::new().expect("Block should run on main thread");
            let app = NSApplication::sharedApplication(mtm);
            app.stop(None);

            // Stopping the event loop requires an actual event
            let location = NSPoint::new(0.0, 0.0);
            let event = NSEvent::otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2(
                NSEventType::ApplicationDefined,
                location,
                NSEventModifierFlags::empty(),
                0.0,
                0,
                None,
                NSEventSubtype::ApplicationActivated.0,
                0,
                0,
            ).unwrap();
            app.postEvent_atStart(&event, true);
        });

        res
    });

    app.run();

    t.join().unwrap()
}
