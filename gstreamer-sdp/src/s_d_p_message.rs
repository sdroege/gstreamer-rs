
impl SDPMessage {
    pub fn new() -> SDPMessage {
        assert_initialized_main_thread!();
        unsafe {
            let msg = glib_ffi::g_malloc(mem::size_of::<ffi::GstSDPMessage>());
            let _ = ffi::gst_sdp_message_new(msg);
            from_glib_full(msg)
        }
    }
}
