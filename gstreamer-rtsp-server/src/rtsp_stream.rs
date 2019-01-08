use ffi;
use glib::object::IsA;
use glib::translate::*;
use gst;
use RTSPStream;

pub trait RTSPStreamExtManual: 'static {
    fn recv_rtcp(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn recv_rtp(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<O: IsA<RTSPStream>> RTSPStreamExtManual for O {
    fn recv_rtcp(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(ffi::gst_rtsp_stream_recv_rtcp(
                self.to_glib_none().0,
                buffer.to_glib_full(),
            ))
        };
        ret.into_result()
    }

    fn recv_rtp(&self, buffer: &gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(ffi::gst_rtsp_stream_recv_rtp(
                self.to_glib_none().0,
                buffer.to_glib_full(),
            ))
        };
        ret.into_result()
    }
}
