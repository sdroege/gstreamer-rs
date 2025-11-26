// https://github.com/rust-lang/cargo/issues/5077#issuecomment-1284482987
fn main() {
    #[allow(clippy::needless_return)]
    if std::env::var("DOCS_RS").is_ok() {
        // prevent linking libraries to avoid documentation failure
        return;
    }

    #[cfg(target_os = "macos")]
    match system_deps::Config::new().probe() {
        Ok(deps) => {
            let usr = std::path::Path::new("/usr/lib");
            let usr_local = std::path::Path::new("/usr/local/lib");
            for dep in deps.all_link_paths() {
                if dep != &usr && dep != &usr_local {
                    println!("cargo:rustc-link-arg=-Wl,-rpath,{:?}", dep.as_os_str());
                }
            }
        }
        Err(s) => {
            println!("cargo:warning={s}");
            std::process::exit(1);
        }
    }
}
