/// macOS has a specific requirement that there must be a run loop running
/// on the main thread in order to open windows and use OpenGL.

#[cfg(target_os = "macos")]
#[link(name = "foundation", kind = "framework")]
extern "C" {
    fn CFRunLoopRun();
}

/// On macOS this launches the callback function on a thread.
/// On other platforms it's just executed immediately.
#[cfg(not(target_os = "macos"))]
pub fn run<F: FnOnce() + Send + 'static>(main: F) {
    main();
}

#[cfg(target_os = "macos")]
pub fn run<F: FnOnce() + Send + 'static>(main: F) {
    ::std::thread::spawn(main);
    unsafe {
        CFRunLoopRun();
    }
}
