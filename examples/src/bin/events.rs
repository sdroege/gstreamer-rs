extern crate gstreamer as gst;
use gst::*;

extern crate glib;
use glib::*;

fn main() {
    gst::init().unwrap();

    let main_loop = MainLoop::new(None, false);

    let pipeline = gst::parse_launch("audiotestsrc ! fakesink").unwrap();
    let bus = pipeline.get_bus().unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let main_loop_clone = main_loop.clone();

    let pipeline_clone = pipeline.clone();
    glib::timeout_add_seconds(5, move || {
        let pipeline = &pipeline_clone;

        println!("sending eos");

        let ev = Event::new_eos().build();
        pipeline.send_event(ev);

        glib::Continue(false)
    });

    //bus.add_signal_watch();
    //bus.connect_message(move |_, msg| {
    bus.add_watch(move |_, msg| {
        let main_loop = &main_loop_clone;
        match msg.view() {
            MessageView::Eos(..) => {
                println!("received eos");
                main_loop.quit()
            }
            MessageView::Error(err) => {
                println!(
                    "Error from {}: {} ({:?})",
                    msg.get_src().get_path_string(),
                    err.get_error(),
                    err.get_debug()
                );
                main_loop.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    });

    main_loop.run();

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}
