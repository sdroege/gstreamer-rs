use std::io;
use std::path::Path;
use stripper_lib::{loop_over_files, parse_cmts, regenerate_comments, strip_comments};

#[derive(Clone, Copy, Debug)]
pub enum Library {
    GstWebRTC,
    GstVideo,
    GstSdp,
    GstRtspServer,
    GstRtsp,
    GstRtp,
    GstPlayer,
    GstNet,
    GstGL,
    GES,
    GstCheck,
    GstPbutils,
    GstBase,
    GstAudio,
    GstApp,
    Gst,
}

fn docs(lib: Library) -> &'static str {
    match lib {
        Library::GstWebRTC => include_str!("../gstreamer-webrtc/docs.md"),
        Library::GstVideo => include_str!("../gstreamer-video/docs.md"),
        Library::GstSdp => include_str!("../gstreamer-sdp/docs.md"),
        Library::GstRtspServer => include_str!("../gstreamer-rtsp-server/docs.md"),
        Library::GstRtsp => include_str!("../gstreamer-rtsp/docs.md"),
        Library::GstRtp => include_str!("../gstreamer-rtp/docs.md"),
        Library::GstPlayer => include_str!("../gstreamer-player/docs.md"),
        Library::GstNet => include_str!("../gstreamer-net/docs.md"),
        Library::GstGL => include_str!("../gstreamer-gl/docs.md"),
        Library::GES => include_str!("../gstreamer-editing-services/docs.md"),
        Library::GstCheck => include_str!("../gstreamer-check/docs.md"),
        Library::GstPbutils => include_str!("../gstreamer-pbutils/docs.md"),
        Library::GstBase => include_str!("../gstreamer-base/docs.md"),
        Library::GstAudio => include_str!("../gstreamer-audio/docs.md"),
        Library::GstApp => include_str!("../gstreamer-app/docs.md"),
        Library::Gst => include_str!("../gstreamer/docs.md"),
    }
}

fn vendor_docs(_lib: Library) -> Option<&'static str> {
    None
}

/// Embeds the docs.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn embed<P: AsRef<Path>>(library: Library, path: P, ignores: &[&str]) {
    let docs = docs(library);
    do_embed(docs, path.as_ref(), ignores);

    if let Some(docs) = vendor_docs(library) {
        do_embed(docs, path.as_ref(), ignores);
    }
}

fn do_embed(docs: &str, path: &Path, ignores: &[&str]) {
    let mut infos = parse_cmts(docs.lines(), true);
    loop_over_files(
        path,
        &mut |w, s| regenerate_comments(w, s, &mut infos, true, true),
        &ignores,
        false,
    );
}

/// Remove any doc comments.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn purge<P: AsRef<Path>>(path: P, ignores: &[&str]) {
    loop_over_files(
        path.as_ref(),
        &mut |w, s| strip_comments(w, s, &mut io::sink(), true),
        &ignores,
        false,
    );
}
