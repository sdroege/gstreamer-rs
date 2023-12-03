mod imp;

// This here defines the public interface of our element and implements
// the corresponding traits so that it behaves like any other gst::Element
//
// GObject
//     ╰──GstObject
//         ╰──GstElement
//             ╰──GstBaseTransform
//                 ╰──GstAudioFilter
//                     ╰──IirFilter
//                         ╰──Lowpass
glib::wrapper! {
    pub struct Lowpass(ObjectSubclass<imp::Lowpass>) @extends crate::iirfilter::IirFilter, gst_audio::AudioFilter, gst_base::BaseTransform, gst::Element, gst::Object;
}
