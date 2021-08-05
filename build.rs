fn main() {
    #[cfg(not(feature = "tracing-gstreamer-docs"))]
    if let Err(e) = pkg_config::Config::new().probe("gstreamer-1.0") {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
