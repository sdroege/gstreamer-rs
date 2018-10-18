#[cfg(feature = "gl")]
extern crate gl_generator;

#[cfg(feature = "gl")]
fn generate_gl_bindings() {
    let dest = std::path::PathBuf::from(&std::env::var("OUT_DIR").unwrap());
    let mut file = std::fs::File::create(&dest.join("test_gl_bindings.rs")).unwrap();
    gl_generator::Registry::new(
        gl_generator::Api::Gles2,
        (3, 0),
        gl_generator::Profile::Core,
        gl_generator::Fallbacks::All,
        [],
    )
    .write_bindings(gl_generator::StructGenerator, &mut file)
    .unwrap();
}

#[cfg(not(feature = "gl"))]
fn generate_gl_bindings() {}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    generate_gl_bindings();
}
