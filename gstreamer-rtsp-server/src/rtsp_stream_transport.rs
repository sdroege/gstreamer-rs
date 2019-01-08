use ffi;
use glib::object::IsA;
use glib::translate::*;
use gst;
use RTSPStreamTransport;

pub trait RTSPStreamTransportExtManual: 'static {
    fn recv_data(
        &self,
        channel: u32,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<O: IsA<RTSPStreamTransport>> RTSPStreamTransportExtManual for O {
    fn recv_data(
        &self,
        channel: u32,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(ffi::gst_rtsp_stream_transport_recv_data(
                self.to_glib_none().0,
                channel,
                buffer.to_glib_full(),
            ))
        };
        ret.into_result()
    }
}
